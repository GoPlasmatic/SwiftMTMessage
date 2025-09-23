use async_trait::async_trait;
use dataflow_rs::engine::error::DataflowError;
use dataflow_rs::engine::{
    AsyncFunctionHandler, FunctionConfig,
    error::Result,
    message::{Change, Message},
};
use datalogic_rs::DataLogic;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, instrument};
use crate::{SwiftMessage, messages::*};

/// Fix numbered enum fields in MT message by adding variant letters
fn fix_numbered_enum_fields_in_mt(mt_message: &str, json_value: &Value) -> String {
    let mut fixed_message = mt_message.to_string();

    #[cfg(debug_assertions)]
    eprintln!("DEBUG fix_numbered_enum_fields_in_mt called");

    // Look for numbered fields in JSON that are enum types
    if let Some(fields) = json_value.get("fields") {
        #[cfg(debug_assertions)]
        eprintln!("DEBUG fields is_object: {}, is_array: {}", fields.is_object(), fields.is_array());

        // Check for numbered fields at the top level (for MT101, MT104, MT107 sequence A)
        check_and_fix_field_50(&mut fixed_message, fields, "50#1", 0);
        check_and_fix_field_50(&mut fixed_message, fields, "50#2", 1);

        // Fix Field 60 variants (F or M)
        if let Some(field_60) = fields.get("60") {
            if let Some((variant, _)) = field_60.as_object().and_then(|o| o.iter().next()) {
                // Replace :60: with :60<variant>:
                fixed_message = fixed_message.replace(":60:", &format!(":60{}:", variant));
            }
        }

        // Fix Field 62 variants (F or M)
        if let Some(field_62) = fields.get("62") {
            if let Some((variant, _)) = field_62.as_object().and_then(|o| o.iter().next()) {
                // Replace :62: with :62<variant>:
                fixed_message = fixed_message.replace(":62:", &format!(":62{}:", variant));
            }
        }

        // For messages with transactions (MT101, MT104, MT107), check the "#" field which is the array
        if let Some(transactions) = fields.get("#").and_then(|t| t.as_array()) {
            #[cfg(debug_assertions)]
            eprintln!("DEBUG found {} transactions", transactions.len());

            for transaction in transactions {
                // Check for 50#1 and 50#2 fields in each transaction
                if let Some(field_50_1) = transaction.get("50#1") {
                    if let Some((variant, _)) = field_50_1.as_object().and_then(|o| o.iter().next()) {
                        // Always replace the first remaining :50: with :50<variant>:
                        replace_nth_field_50(&mut fixed_message, 0, variant);
                    }
                }
                if let Some(field_50_2) = transaction.get("50#2") {
                    if let Some((variant, _)) = field_50_2.as_object().and_then(|o| o.iter().next()) {
                        // Always replace the first remaining :50: with :50<variant>:
                        replace_nth_field_50(&mut fixed_message, 0, variant);
                    }
                }

                // Also check for Field 60 and 62 in transactions if they exist
                if let Some(field_60) = transaction.get("60") {
                    if let Some((variant, _)) = field_60.as_object().and_then(|o| o.iter().next()) {
                        // Replace :60: with :60<variant>:
                        fixed_message = fixed_message.replace(":60:", &format!(":60{}:", variant));
                    }
                }
                if let Some(field_62) = transaction.get("62") {
                    if let Some((variant, _)) = field_62.as_object().and_then(|o| o.iter().next()) {
                        // Replace :62: with :62<variant>:
                        fixed_message = fixed_message.replace(":62:", &format!(":62{}:", variant));
                    }
                }
            }
        }
    }

    fixed_message
}

/// Check for a numbered field and fix it if found
fn check_and_fix_field_50(fixed_message: &mut String, fields: &Value, field_name: &str, occurrence: usize) {
    if let Some(field) = fields.get(field_name) {
        if let Some((variant, _)) = field.as_object().and_then(|o| o.iter().next()) {
            replace_nth_field_50(fixed_message, occurrence, variant);
        }
    }
}

/// Replace the nth occurrence of :50: with :50<variant>:
fn replace_nth_field_50(message: &mut String, n: usize, variant: &str) -> bool {
    let target = ":50:";
    let mut count = 0;
    let mut pos = 0;

    while let Some(found_pos) = message[pos..].find(target) {
        let actual_pos = pos + found_pos;
        if count == n {
            message.replace_range(actual_pos..actual_pos + target.len(), &format!(":50{}:", variant));
            return true;
        }
        count += 1;
        pos = actual_pos + target.len();
    }

    false
}

/// Helper function to clean null values from fields before serialization
fn clean_null_fields(data: &Value) -> Value {
    let mut cleaned = data.clone();
    if let Some(fields) = cleaned.get_mut("fields").and_then(|f| f.as_object_mut()) {
        fields.retain(|_key, value| {
            if let Some(obj) = value.as_object() {
                // Remove fields where any nested values are null
                !obj.values().any(|v| v.is_null())
            } else {
                true // Keep non-object values
            }
        });
    }
    cleaned
}

/// Parse JSON into a SwiftMessage and convert to MT format
fn json_to_mt(message_type: &str, json_value: &Value) -> std::result::Result<String, DataflowError> {
    macro_rules! convert_json {
        ($mt_type:ty) => {{
            let msg: SwiftMessage<$mt_type> =
                serde_json::from_value(json_value.clone()).map_err(|e| {
                    DataflowError::Validation(format!("Failed to parse JSON as {}: {}", stringify!($mt_type), e))
                })?;
            let mut mt_message = msg.to_mt_message();

            // Fix numbered enum fields that are missing variant letters
            // This is a workaround for the issue with numbered enum fields in nested messages
            mt_message = fix_numbered_enum_fields_in_mt(&mt_message, json_value);

            Ok(mt_message)
        }};
    }

    match message_type {
        "101" | "MT101" => convert_json!(MT101),
        "103" | "MT103" => convert_json!(MT103),
        "104" | "MT104" => convert_json!(MT104),
        "107" | "MT107" => convert_json!(MT107),
        "110" | "MT110" => convert_json!(MT110),
        "111" | "MT111" => convert_json!(MT111),
        "112" | "MT112" => convert_json!(MT112),
        "190" | "MT190" => convert_json!(MT190),
        "191" | "MT191" => convert_json!(MT191),
        "192" | "MT192" => convert_json!(MT192),
        "196" | "MT196" => convert_json!(MT196),
        "199" | "MT199" => convert_json!(MT199),
        "200" | "MT200" => convert_json!(MT200),
        "202" | "MT202" => convert_json!(MT202),
        "204" | "MT204" => convert_json!(MT204),
        "205" | "MT205" => convert_json!(MT205),
        "210" | "MT210" => convert_json!(MT210),
        "290" | "MT290" => convert_json!(MT290),
        "291" | "MT291" => convert_json!(MT291),
        "292" | "MT292" => convert_json!(MT292),
        "296" | "MT296" => convert_json!(MT296),
        "299" | "MT299" => convert_json!(MT299),
        "900" | "MT900" => convert_json!(MT900),
        "910" | "MT910" => convert_json!(MT910),
        "920" | "MT920" => convert_json!(MT920),
        "935" | "MT935" => convert_json!(MT935),
        "940" | "MT940" => convert_json!(MT940),
        "941" | "MT941" => convert_json!(MT941),
        "942" | "MT942" => convert_json!(MT942),
        "950" | "MT950" => convert_json!(MT950),
        _ => Err(DataflowError::Validation(format!(
            "Unsupported message type: {}",
            message_type
        ))),
    }
}

pub struct Publish;

#[async_trait]
impl AsyncFunctionHandler for Publish {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Starting JSON to MT message publishing");

        // Extract custom configuration
        let input = match config {
            FunctionConfig::Custom { input, .. } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        // Get json_data and mt_message field names
        let json_data_field = input
            .get("json_data")
            .and_then(Value::as_str)
            .ok_or_else(|| DataflowError::Validation("'json_data' parameter is required".to_string()))?;

        let mt_message_field = input
            .get("mt_message")
            .and_then(Value::as_str)
            .ok_or_else(|| DataflowError::Validation("'mt_message' parameter is required".to_string()))?;

        // Extract JSON data from the message
        let json_data = message.data().get(json_data_field).cloned().ok_or_else(|| {
            error!(
                json_data_field = %json_data_field,
                available_fields = ?message.data().as_object().map(|obj| obj.keys().collect::<Vec<_>>()),
                "JSON data field not found in message data"
            );
            DataflowError::Validation(format!(
                "Field '{}' not found in message data",
                json_data_field
            ))
        })?;

        debug!(
            json_data_field = %json_data_field,
            mt_message_field = %mt_message_field,
            "Processing JSON to MT conversion"
        );

        // Extract the actual JSON data if it's wrapped in a generate result
        let json_to_convert = if let Some(inner_json) = json_data.get("json_data") {
            // This is output from the generate function
            inner_json.clone()
        } else {
            // Direct JSON data
            json_data.clone()
        };

        // Clean null values from fields before serialization (required for swift-mt-message library)
        let cleaned_data = clean_null_fields(&json_to_convert);

        // Extract message type from the JSON data
        let message_type = json_data.get("message_type")
            .and_then(Value::as_str)
            .map(|mt| mt.trim_start_matches("MT").to_string())
            .ok_or_else(|| {
                DataflowError::Validation(
                    "Missing 'message_type' field in JSON data. The message_type field is required at the root level.".to_string()
                )
            })?;

        debug!(message_type = %message_type, "Converting JSON to MT{}", message_type);

        // Convert JSON to MT message
        let mt_message = json_to_mt(&message_type, &cleaned_data)
            .map_err(|e| DataflowError::Validation(format!("Failed to convert to MT{}: {}", message_type, e)))?;

        debug!(
            message_length = mt_message.len(),
            "MT message published successfully"
        );

        // Store the MT message in the output field
        let old_value = message.data().get(mt_message_field).cloned().unwrap_or(Value::Null);

        message.data_mut()[mt_message_field] = Value::String(mt_message.clone());

        // Invalidate cache after modifications
        message.invalidate_context_cache();

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", mt_message_field)),
                old_value: Arc::new(old_value),
                new_value: Arc::new(Value::String(mt_message)),
            }],
        ))
    }
}