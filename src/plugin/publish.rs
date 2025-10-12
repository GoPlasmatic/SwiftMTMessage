use crate::{SwiftMessage, messages::*};
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
fn json_to_mt(
    message_type: &str,
    json_value: &Value,
) -> std::result::Result<String, DataflowError> {
    macro_rules! convert_json {
        ($mt_type:ty) => {{
            let msg: SwiftMessage<$mt_type> =
                serde_json::from_value(json_value.clone()).map_err(|e| {
                    DataflowError::Validation(format!(
                        "Failed to parse JSON as {}: {}",
                        stringify!($mt_type),
                        e
                    ))
                })?;
            Ok(msg.to_mt_message())
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

        // Get source and target field names
        let source_field = input.get("source").and_then(Value::as_str).ok_or_else(|| {
            DataflowError::Validation("'source' parameter is required".to_string())
        })?;

        let target_field = input.get("target").and_then(Value::as_str).ok_or_else(|| {
            DataflowError::Validation("'target' parameter is required".to_string())
        })?;

        // Extract JSON data from the message
        let json_data = message.data().get(source_field).cloned().ok_or_else(|| {
            error!(
                source_field = %source_field,
                available_fields = ?message.data().as_object().map(|obj| obj.keys().collect::<Vec<_>>()),
                "JSON data field not found in message data"
            );
            DataflowError::Validation(format!(
                "Field '{}' not found in message data",
                source_field
            ))
        })?;

        debug!(
            source_field = %source_field,
            target_field = %target_field,
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
        let mt_message = json_to_mt(&message_type, &cleaned_data).map_err(|e| {
            DataflowError::Validation(format!("Failed to convert to MT{}: {}", message_type, e))
        })?;

        debug!(
            message_length = mt_message.len(),
            "MT message published successfully"
        );

        // Store the MT message in the output field
        let old_value = message
            .data()
            .get(target_field)
            .cloned()
            .unwrap_or(Value::Null);

        message.data_mut()[target_field] = Value::String(mt_message.clone());

        // Invalidate cache after modifications
        message.invalidate_context_cache();

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", target_field)),
                old_value: Arc::new(old_value),
                new_value: Arc::new(Value::String(mt_message)),
            }],
        ))
    }
}
