use crate::SwiftParser;
use async_trait::async_trait;
use dataflow_rs::engine::error::DataflowError;
use dataflow_rs::engine::{
    AsyncFunctionHandler, FunctionConfig,
    error::Result,
    message::{Change, Message},
};
use datalogic_rs::DataLogic;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::{debug, error, instrument};

pub struct Parse;

#[async_trait]
impl AsyncFunctionHandler for Parse {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Starting MT message parsing for forward transformation");

        // Extract custom configuration
        let input = match config {
            FunctionConfig::Custom { input, .. } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        let mt_message_field = input
            .get("mt_message")
            .and_then(Value::as_str)
            .ok_or_else(|| DataflowError::Validation("'mt_message' parameter is required".to_string()))?;

        let parsed_field = input
            .get("parsed")
            .and_then(Value::as_str)
            .ok_or_else(|| DataflowError::Validation("'parsed' parameter is required".to_string()))?;

        let payload = if mt_message_field == "payload" {
            message.payload.to_string().replace("\\n", "\n")
        } else {
            // Check if the field contains an object with mt_message (from generate_mt output)
            let field_value = message
                .data()
                .get(mt_message_field)
                .ok_or_else(|| DataflowError::Validation(format!("MT message field '{}' not found in message data", mt_message_field)))?;

            // If it's an object with mt_message field, extract that
            if let Some(mt_msg) = field_value.get("mt_message").and_then(Value::as_str) {
                mt_msg.to_string()
            } else if let Some(s) = field_value.as_str() {
                // If it's a direct string, use it
                s.to_string()
            } else {
                return Err(DataflowError::Validation(
                    format!("Field '{}' does not contain a valid MT message", mt_message_field)
                ));
            }
        };

        debug!(
            mt_message_field = %mt_message_field,
            parsed_field = %parsed_field,
            payload_length = payload.len(),
            "Extracted MT payload for parsing"
        );

        self.parse_swift_mt(message, &payload, parsed_field)
    }
}

impl Parse {
    fn parse_swift_mt(
        &self,
        message: &mut Message,
        payload: &str,
        parsed_field: &str,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Parsing SwiftMT message for forward transformation");

        let payload = Parse::manual_unescape(payload);
        debug!("Parsing MT message with payload length: {}", payload.len());
        let parsed_message = SwiftParser::parse_auto(&payload).map_err(|e| {
            error!(error = ?e, "SwiftMT parsing failed");
            DataflowError::Validation(format!("SwiftMT parser error: {e:?}"))
        })?;

        let message_type = parsed_message.message_type().to_string();
        debug!(message_type = %message_type, "Successfully parsed SwiftMT message");

        let method: String;

        let parsed_data = match message_type.as_str() {
            "101" => {
                let Some(mt101_message) = parsed_message.into_mt101() else {
                    error!("Failed to convert SwiftMessage to MT101");
                    return Err(DataflowError::Validation(
                        "MT101 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();

                serde_json::to_value(&mt101_message).map_err(|e| {
                    error!(error = ?e, "MT101 JSON conversion failed");
                    DataflowError::Validation(format!("MT101 JSON conversion failed: {e}"))
                })?
            }
            "103" => {
                let Some(mt103_message) = parsed_message.into_mt103() else {
                    error!("Failed to convert SwiftMessage to MT103");
                    return Err(DataflowError::Validation(
                        "MT103 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = if mt103_message.has_reject_codes() {
                    "reject".to_string()
                } else if mt103_message.has_return_codes() {
                    "return".to_string()
                } else if mt103_message.is_stp_message() {
                    "stp".to_string()
                } else {
                    "normal".to_string()
                };

                debug!(method = %method, "Determined MT103 processing method");

                serde_json::to_value(&mt103_message).map_err(|e| {
                    error!(error = ?e, "MT103 JSON conversion failed");
                    DataflowError::Validation(format!("MT103 JSON conversion failed: {e}"))
                })?
            }
            "200" => {
                let Some(mt200_message) = parsed_message.into_mt200() else {
                    error!("Failed to convert SwiftMessage to MT200");
                    return Err(DataflowError::Validation(
                        "MT200 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();
                debug!("Processing MT200 with normal method");

                serde_json::to_value(&mt200_message).map_err(|e| {
                    error!(error = ?e, "MT200 JSON conversion failed");
                    DataflowError::Validation(format!("MT200 JSON conversion failed: {e}"))
                })?
            }
            "202" => {
                let Some(mt202_message) = parsed_message.into_mt202() else {
                    error!("Failed to convert SwiftMessage to MT202");
                    return Err(DataflowError::Validation(
                        "MT202 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = if mt202_message.has_reject_codes()
                    || mt202_message
                        .user_header
                        .as_ref()
                        .and_then(|h| h.validation_flag.as_ref())
                        .map(|flag| flag.as_str() == "REJT")
                        .unwrap_or(false)
                {
                    "reject".to_string()
                } else if mt202_message.has_return_codes()
                    || mt202_message
                        .user_header
                        .as_ref()
                        .and_then(|h| h.validation_flag.as_ref())
                        .map(|flag| flag.as_str() == "RETN")
                        .unwrap_or(false)
                {
                    "return".to_string()
                } else if mt202_message.is_cover_message()
                    || mt202_message
                        .user_header
                        .as_ref()
                        .and_then(|h| h.validation_flag.as_ref())
                        .map(|flag| flag.as_str() == "COV")
                        .unwrap_or(false)
                {
                    "cover".to_string()
                } else {
                    "normal".to_string()
                };

                debug!(method = %method, "Determined MT202 processing method");

                serde_json::to_value(&mt202_message).map_err(|e| {
                    error!(error = ?e, "MT202 JSON conversion failed");
                    DataflowError::Validation(format!("MT202 JSON conversion failed: {e}"))
                })?
            }
            "205" => {
                let Some(mt205_message) = parsed_message.into_mt205() else {
                    error!("Failed to convert SwiftMessage to MT205");
                    return Err(DataflowError::Validation(
                        "MT205 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = if mt205_message.has_reject_codes()
                    || mt205_message
                        .user_header
                        .as_ref()
                        .and_then(|h| h.validation_flag.as_ref())
                        .map(|flag| flag.as_str() == "REJT")
                        .unwrap_or(false)
                {
                    "reject".to_string()
                } else if mt205_message.has_return_codes()
                    || mt205_message
                        .user_header
                        .as_ref()
                        .and_then(|h| h.validation_flag.as_ref())
                        .map(|flag| flag.as_str() == "RETN")
                        .unwrap_or(false)
                {
                    "return".to_string()
                } else if mt205_message.is_cover_message()
                    || mt205_message
                        .user_header
                        .as_ref()
                        .and_then(|h| h.validation_flag.as_ref())
                        .map(|flag| flag.as_str() == "COV")
                        .unwrap_or(false)
                {
                    "cover".to_string()
                } else {
                    "normal".to_string()
                };

                debug!(method = %method, "Determined MT205 processing method");

                serde_json::to_value(&mt205_message).map_err(|e| {
                    error!(error = ?e, "MT205 JSON conversion failed");
                    DataflowError::Validation(format!("MT205 JSON conversion failed: {e}"))
                })?
            }
            "900" => {
                let Some(mt900_message) = parsed_message.into_mt900() else {
                    error!("Failed to convert SwiftMessage to MT900");
                    return Err(DataflowError::Validation(
                        "MT900 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();
                debug!("Processing MT900 with normal method");

                serde_json::to_value(&mt900_message).map_err(|e| {
                    error!(error = ?e, "MT900 JSON conversion failed");
                    DataflowError::Validation(format!("MT900 JSON conversion failed: {e}"))
                })?
            }
            "910" => {
                let Some(mt910_message) = parsed_message.into_mt910() else {
                    error!("Failed to convert SwiftMessage to MT910");
                    return Err(DataflowError::Validation(
                        "MT910 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();
                debug!("Processing MT910 with normal method");

                serde_json::to_value(&mt910_message).map_err(|e| {
                    error!(error = ?e, "MT910 JSON conversion failed");
                    DataflowError::Validation(format!("MT910 JSON conversion failed: {e}"))
                })?
            }
            "192" => {
                let Some(mt192_message) = parsed_message.into_mt192() else {
                    error!("Failed to convert SwiftMessage to MT192");
                    return Err(DataflowError::Validation(
                        "MT192 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();
                debug!("Processing MT192 with normal method");

                serde_json::to_value(&mt192_message).map_err(|e| {
                    error!(error = ?e, "MT192 JSON conversion failed");
                    DataflowError::Validation(format!("MT192 JSON conversion failed: {e}"))
                })?
            }
            "292" => {
                let Some(mt292_message) = parsed_message.into_mt292() else {
                    error!("Failed to convert SwiftMessage to MT292");
                    return Err(DataflowError::Validation(
                        "MT292 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();
                debug!("Processing MT292 with normal method");

                serde_json::to_value(&mt292_message).map_err(|e| {
                    error!(error = ?e, "MT292 JSON conversion failed");
                    DataflowError::Validation(format!("MT292 JSON conversion failed: {e}"))
                })?
            }
            "196" => {
                let Some(mt196_message) = parsed_message.into_mt196() else {
                    error!("Failed to convert SwiftMessage to MT196");
                    return Err(DataflowError::Validation(
                        "MT196 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();
                debug!("Processing MT196 with normal method");

                serde_json::to_value(&mt196_message).map_err(|e| {
                    error!(error = ?e, "MT196 JSON conversion failed");
                    DataflowError::Validation(format!("MT196 JSON conversion failed: {e}"))
                })?
            }
            "296" => {
                let Some(mt296_message) = parsed_message.into_mt296() else {
                    error!("Failed to convert SwiftMessage to MT296");
                    return Err(DataflowError::Validation(
                        "MT296 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();
                debug!("Processing MT296 with normal method");

                serde_json::to_value(&mt296_message).map_err(|e| {
                    error!(error = ?e, "MT296 JSON conversion failed");
                    DataflowError::Validation(format!("MT296 JSON conversion failed: {e}"))
                })?
            }
            "104" => {
                let Some(mt104_message) = parsed_message.into_mt104() else {
                    error!("Failed to convert SwiftMessage to MT104");
                    return Err(DataflowError::Validation(
                        "MT104 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();

                serde_json::to_value(&mt104_message).map_err(|e| {
                    error!(error = ?e, "MT104 JSON conversion failed");
                    DataflowError::Validation(format!("MT104 JSON conversion failed: {e}"))
                })?
            }
            "920" => {
                let Some(mt920_message) = parsed_message.into_mt920() else {
                    error!("Failed to convert SwiftMessage to MT920");
                    return Err(DataflowError::Validation(
                        "MT920 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();

                serde_json::to_value(&mt920_message).map_err(|e| {
                    error!(error = ?e, "MT920 JSON conversion failed");
                    DataflowError::Validation(format!("MT920 JSON conversion failed: {e}"))
                })?
            }
            "940" => {
                let Some(mt940_message) = parsed_message.into_mt940() else {
                    error!("Failed to convert SwiftMessage to MT940");
                    return Err(DataflowError::Validation(
                        "MT940 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();

                serde_json::to_value(&mt940_message).map_err(|e| {
                    error!(error = ?e, "MT940 JSON conversion failed");
                    DataflowError::Validation(format!("MT940 JSON conversion failed: {e}"))
                })?
            }
            "950" => {
                let Some(mt950_message) = parsed_message.into_mt950() else {
                    error!("Failed to convert SwiftMessage to MT950");
                    return Err(DataflowError::Validation(
                        "MT950 message not found in SwiftMT message".to_string(),
                    ));
                };

                method = "normal".to_string();

                serde_json::to_value(&mt950_message).map_err(|e| {
                    error!(error = ?e, "MT950 JSON conversion failed");
                    DataflowError::Validation(format!("MT950 JSON conversion failed: {e}"))
                })?
            }
            "107" => {
                let Some(mt107_message) = parsed_message.into_mt107() else {
                    error!("Failed to convert SwiftMessage to MT107");
                    return Err(DataflowError::Validation(
                        "MT107 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt107_message).map_err(|e| {
                    error!(error = ?e, "MT107 JSON conversion failed");
                    DataflowError::Validation(format!("MT107 JSON conversion failed: {e}"))
                })?
            }
            "110" => {
                let Some(mt110_message) = parsed_message.into_mt110() else {
                    error!("Failed to convert SwiftMessage to MT110");
                    return Err(DataflowError::Validation(
                        "MT110 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt110_message).map_err(|e| {
                    error!(error = ?e, "MT110 JSON conversion failed");
                    DataflowError::Validation(format!("MT110 JSON conversion failed: {e}"))
                })?
            }
            "111" => {
                let Some(mt111_message) = parsed_message.into_mt111() else {
                    error!("Failed to convert SwiftMessage to MT111");
                    return Err(DataflowError::Validation(
                        "MT111 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt111_message).map_err(|e| {
                    error!(error = ?e, "MT111 JSON conversion failed");
                    DataflowError::Validation(format!("MT111 JSON conversion failed: {e}"))
                })?
            }
            "112" => {
                let Some(mt112_message) = parsed_message.into_mt112() else {
                    error!("Failed to convert SwiftMessage to MT112");
                    return Err(DataflowError::Validation(
                        "MT112 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt112_message).map_err(|e| {
                    error!(error = ?e, "MT112 JSON conversion failed");
                    DataflowError::Validation(format!("MT112 JSON conversion failed: {e}"))
                })?
            }
            "190" => {
                let Some(mt190_message) = parsed_message.into_mt190() else {
                    error!("Failed to convert SwiftMessage to MT190");
                    return Err(DataflowError::Validation(
                        "MT190 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt190_message).map_err(|e| {
                    error!(error = ?e, "MT190 JSON conversion failed");
                    DataflowError::Validation(format!("MT190 JSON conversion failed: {e}"))
                })?
            }
            "191" => {
                let Some(mt191_message) = parsed_message.into_mt191() else {
                    error!("Failed to convert SwiftMessage to MT191");
                    return Err(DataflowError::Validation(
                        "MT191 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt191_message).map_err(|e| {
                    error!(error = ?e, "MT191 JSON conversion failed");
                    DataflowError::Validation(format!("MT191 JSON conversion failed: {e}"))
                })?
            }
            "199" => {
                let Some(mt199_message) = parsed_message.into_mt199() else {
                    error!("Failed to convert SwiftMessage to MT199");
                    return Err(DataflowError::Validation(
                        "MT199 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt199_message).map_err(|e| {
                    error!(error = ?e, "MT199 JSON conversion failed");
                    DataflowError::Validation(format!("MT199 JSON conversion failed: {e}"))
                })?
            }
            "204" => {
                let Some(mt204_message) = parsed_message.into_mt204() else {
                    error!("Failed to convert SwiftMessage to MT204");
                    return Err(DataflowError::Validation(
                        "MT204 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt204_message).map_err(|e| {
                    error!(error = ?e, "MT204 JSON conversion failed");
                    DataflowError::Validation(format!("MT204 JSON conversion failed: {e}"))
                })?
            }
            "210" => {
                let Some(mt210_message) = parsed_message.into_mt210() else {
                    error!("Failed to convert SwiftMessage to MT210");
                    return Err(DataflowError::Validation(
                        "MT210 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt210_message).map_err(|e| {
                    error!(error = ?e, "MT210 JSON conversion failed");
                    DataflowError::Validation(format!("MT210 JSON conversion failed: {e}"))
                })?
            }
            "290" => {
                let Some(mt290_message) = parsed_message.into_mt290() else {
                    error!("Failed to convert SwiftMessage to MT290");
                    return Err(DataflowError::Validation(
                        "MT290 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt290_message).map_err(|e| {
                    error!(error = ?e, "MT290 JSON conversion failed");
                    DataflowError::Validation(format!("MT290 JSON conversion failed: {e}"))
                })?
            }
            "291" => {
                let Some(mt291_message) = parsed_message.into_mt291() else {
                    error!("Failed to convert SwiftMessage to MT291");
                    return Err(DataflowError::Validation(
                        "MT291 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt291_message).map_err(|e| {
                    error!(error = ?e, "MT291 JSON conversion failed");
                    DataflowError::Validation(format!("MT291 JSON conversion failed: {e}"))
                })?
            }
            "299" => {
                let Some(mt299_message) = parsed_message.into_mt299() else {
                    error!("Failed to convert SwiftMessage to MT299");
                    return Err(DataflowError::Validation(
                        "MT299 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt299_message).map_err(|e| {
                    error!(error = ?e, "MT299 JSON conversion failed");
                    DataflowError::Validation(format!("MT299 JSON conversion failed: {e}"))
                })?
            }
            "935" => {
                let Some(mt935_message) = parsed_message.into_mt935() else {
                    error!("Failed to convert SwiftMessage to MT935");
                    return Err(DataflowError::Validation(
                        "MT935 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt935_message).map_err(|e| {
                    error!(error = ?e, "MT935 JSON conversion failed");
                    DataflowError::Validation(format!("MT935 JSON conversion failed: {e}"))
                })?
            }
            "941" => {
                let Some(mt941_message) = parsed_message.into_mt941() else {
                    error!("Failed to convert SwiftMessage to MT941");
                    return Err(DataflowError::Validation(
                        "MT941 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt941_message).map_err(|e| {
                    error!(error = ?e, "MT941 JSON conversion failed");
                    DataflowError::Validation(format!("MT941 JSON conversion failed: {e}"))
                })?
            }
            "942" => {
                let Some(mt942_message) = parsed_message.into_mt942() else {
                    error!("Failed to convert SwiftMessage to MT942");
                    return Err(DataflowError::Validation(
                        "MT942 message not found in SwiftMT message".to_string(),
                    ));
                };
                method = "normal".to_string();
                serde_json::to_value(&mt942_message).map_err(|e| {
                    error!(error = ?e, "MT942 JSON conversion failed");
                    DataflowError::Validation(format!("MT942 JSON conversion failed: {e}"))
                })?
            }
            _ => {
                error!(message_type = %message_type, "Unsupported message type encountered");
                return Err(DataflowError::Validation(format!(
                    "Unsupported message type: {message_type}"
                )));
            }
        };

        // Store the parsed result in message data
        message
            .data_mut()
            .as_object_mut()
            .unwrap()
            .insert(parsed_field.to_string(), parsed_data.clone());

        message.metadata_mut().as_object_mut().unwrap().insert(
            parsed_field.to_string(),
            json!({
                "message_type": message_type,
                "method": method,
            }),
        );

        debug!(
            message_type = %message_type,
            method = %method,
            parsed_field = %parsed_field,
            "MT message parsing completed successfully for forward transformation"
        );

        // Important: invalidate cache after modifications
        message.invalidate_context_cache();

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", parsed_field)),
                old_value: Arc::new(Value::Null),
                new_value: Arc::new(parsed_data),
            }],
        ))
    }

    /// Manual string unescaping for common escape sequences
    fn manual_unescape(input: &str) -> String {
        let mut result = input.trim();

        // Remove surrounding double quotes if present
        if result.starts_with('"') && result.ends_with('"') && result.len() > 1 {
            result = &result[1..result.len() - 1];
        }

        // Now unescape the inner content
        result
            .replace("\\r\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\"", "\"")
            .replace("\\'", "'")
            .replace("\\\\", "\\")
            .replace("\\u0020", " ")
            .replace("\\u0022", "\"")
            .replace("\\u003C", "<")
            .replace("\\u003E", ">")
            .replace("\\u003D", "=")
            .replace("\\u002F", "/")
    }
}
