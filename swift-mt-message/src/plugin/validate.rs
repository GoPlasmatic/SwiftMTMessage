use crate::{ParseError, SwiftParser, SwiftValidationError};
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
use tracing::{debug, instrument};

pub struct Validate;

#[async_trait]
impl AsyncFunctionHandler for Validate {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Starting MT message validation");

        // Extract custom configuration
        let input = match config {
            FunctionConfig::Custom { input, .. } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        let mt_message_field =
            input
                .get("mt_message")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    DataflowError::Validation("'mt_message' parameter is required".to_string())
                })?;

        let validation_result_field = input
            .get("validation_result")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                DataflowError::Validation("'validation_result' parameter is required".to_string())
            })?;

        // Get the MT message to validate
        let mt_content = if mt_message_field == "payload" {
            // Extract string value from the payload JSON
            if let Some(s) = message.payload.as_str() {
                s.to_string()
            } else {
                // If it's not a string directly, try to convert
                message.payload.to_string().trim_matches('"').to_string()
            }
        } else {
            // Check if the field contains an object with mt_message (from generate_mt output)
            let field_value = message.data().get(mt_message_field).ok_or_else(|| {
                DataflowError::Validation(format!(
                    "MT message field '{}' not found in message data",
                    mt_message_field
                ))
            })?;

            // If it's an object with mt_message field, extract that
            if let Some(mt_msg) = field_value.get("mt_message").and_then(Value::as_str) {
                mt_msg.to_string()
            } else if let Some(s) = field_value.as_str() {
                // If it's a direct string, use it
                s.to_string()
            } else {
                return Err(DataflowError::Validation(format!(
                    "Field '{}' does not contain a valid MT message",
                    mt_message_field
                )));
            }
        };

        debug!(
            mt_message_field = %mt_message_field,
            validation_result_field = %validation_result_field,
            "Validating MT message"
        );

        // Perform validation
        let validation_result = self.validate_mt_message(&mt_content)?;

        // Store validation result
        message.data_mut().as_object_mut().unwrap().insert(
            validation_result_field.to_string(),
            validation_result.clone(),
        );

        // Update metadata with validation summary
        message.metadata_mut().as_object_mut().unwrap().insert(
            "validation".to_string(),
            json!({
                "validated": true,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }),
        );

        message.invalidate_context_cache();

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", validation_result_field)),
                old_value: Arc::new(Value::Null),
                new_value: Arc::new(validation_result),
            }],
        ))
    }
}

impl Validate {
    fn validate_mt_message(&self, mt_content: &str) -> Result<Value> {
        let mut errors = Vec::new();

        // Try to parse the message
        match SwiftParser::parse_auto(mt_content) {
            Ok(parsed_message) => {
                // Use the new network validation rules from SwiftMessageBody trait
                let validation_errors = self.validate_network_rules(&parsed_message);

                if !validation_errors.is_empty() {
                    // Convert SwiftValidationError instances to formatted error strings
                    for validation_error in validation_errors {
                        errors.push(self.format_validation_error(&validation_error));
                    }
                }
            }
            Err(parse_error) => {
                // Accumulate parse errors - the Display trait already provides good error messages
                errors.push(format!("Parse error: {}", parse_error));

                // Add more specific error details for certain error types
                match &parse_error {
                    ParseError::InvalidFieldFormat(e) => {
                        errors.push(format!(
                            "Field {} - {}: Invalid value '{}', expected format: {}",
                            e.field_tag, e.component_name, e.value, e.format_spec
                        ));
                    }
                    ParseError::MissingRequiredField {
                        field_tag,
                        field_name,
                        ..
                    } => {
                        errors.push(format!(
                            "Missing required field: {} ({})",
                            field_tag, field_name
                        ));
                    }
                    ParseError::InvalidFormat { message } => {
                        errors.push(format!("Invalid message format: {}", message));
                    }
                    ParseError::ValidationFailed {
                        errors: validation_errors,
                    } => {
                        for validation_error in validation_errors {
                            errors.push(validation_error.to_string());
                        }
                    }
                    _ => {
                        // Other error types are already covered by the Display trait
                    }
                }
            }
        }

        let is_valid = errors.is_empty();

        Ok(json!({
            "valid": is_valid,
            "errors": errors,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }))
    }

    /// Validate network rules on parsed message using the SwiftMessageBody trait
    fn validate_network_rules(
        &self,
        parsed_message: &crate::parsed_message::ParsedSwiftMessage,
    ) -> Vec<SwiftValidationError> {
        use crate::parsed_message::ParsedSwiftMessage;

        // Call validate_network_rules on the message body (stop_on_first_error = false to get all errors)
        match parsed_message {
            ParsedSwiftMessage::MT101(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT103(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT104(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT107(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT110(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT111(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT112(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT190(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT191(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT192(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT196(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT199(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT200(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT202(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT204(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT205(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT210(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT290(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT291(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT292(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT296(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT299(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT900(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT910(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT920(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT935(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT940(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT941(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT942(msg) => msg.fields.validate_network_rules(false),
            ParsedSwiftMessage::MT950(msg) => msg.fields.validate_network_rules(false),
        }
    }

    /// Format a SwiftValidationError into a human-readable string
    fn format_validation_error(&self, error: &SwiftValidationError) -> String {
        use crate::errors::SwiftValidationError;

        match error {
            SwiftValidationError::Format(err) => {
                format!(
                    "[{}] Field {}: {} - Invalid value '{}' (expected: {})",
                    err.code, err.field, err.message, err.value, err.expected
                )
            }
            SwiftValidationError::Content(err) => {
                format!(
                    "[{}] Field {}: {} - Content '{}' violates requirement: {}",
                    err.code, err.field, err.message, err.content, err.requirements
                )
            }
            SwiftValidationError::Relation(err) => {
                let related = if !err.related_fields.is_empty() {
                    format!(" (related: {})", err.related_fields.join(", "))
                } else {
                    String::new()
                };
                let instruction = err
                    .instruction_context
                    .as_ref()
                    .map(|ctx| format!(" [{}]", ctx))
                    .unwrap_or_default();
                format!(
                    "[{}] Field {}: {}{}{}",
                    err.code, err.field, err.message, related, instruction
                )
            }
            SwiftValidationError::Business(err) => {
                let related = if !err.related_fields.is_empty() {
                    format!(" (related: {})", err.related_fields.join(", "))
                } else {
                    String::new()
                };
                format!(
                    "[{}] Field {}: {} - Rule: {}{}",
                    err.code, err.field, err.message, err.rule_description, related
                )
            }
            SwiftValidationError::General(err) => {
                let category = err
                    .category
                    .as_ref()
                    .map(|c| format!(" [{}]", c))
                    .unwrap_or_default();
                format!(
                    "[{}] Field {}: {} - Value '{}'{}",
                    err.code, err.field, err.message, err.value, category
                )
            }
        }
    }
}
