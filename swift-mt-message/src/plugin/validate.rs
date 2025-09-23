use crate::{ParseError, SwiftParser, ValidationError, ValidationResult};
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

        let mt_message_field = input
            .get("mt_message")
            .and_then(Value::as_str)
            .ok_or_else(|| DataflowError::Validation("'mt_message' parameter is required".to_string()))?;

        let validation_result_field = input
            .get("validation_result")
            .and_then(Value::as_str)
            .ok_or_else(|| DataflowError::Validation("'validation_result' parameter is required".to_string()))?;

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
            validation_result_field = %validation_result_field,
            "Validating MT message"
        );

        // Perform validation
        let validation_result = self.validate_mt_message(&mt_content)?;

        // Store validation result
        message
            .data_mut()
            .as_object_mut()
            .unwrap()
            .insert(validation_result_field.to_string(), validation_result.clone());

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
                // Use the built-in validation from ParsedSwiftMessage
                let validation_result: ValidationResult = parsed_message.validate();

                if !validation_result.is_valid {
                    // Convert ValidationError enum variants to error strings
                    for validation_error in validation_result.errors {
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
                            errors.push(self.format_validation_error(validation_error));
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

    /// Format a ValidationError into a human-readable string
    fn format_validation_error(&self, error: &ValidationError) -> String {
        match error {
            ValidationError::FormatValidation { field_tag, message } => {
                format!(
                    "Field {}: Format validation failed - {}",
                    field_tag, message
                )
            }
            ValidationError::LengthValidation {
                field_tag,
                expected,
                actual,
            } => {
                format!(
                    "Field {}: Length validation failed - expected {}, got {}",
                    field_tag, expected, actual
                )
            }
            ValidationError::PatternValidation { field_tag, message } => {
                format!(
                    "Field {}: Pattern validation failed - {}",
                    field_tag, message
                )
            }
            ValidationError::ValueValidation { field_tag, message } => {
                format!("Field {}: Value validation failed - {}", field_tag, message)
            }
            ValidationError::BusinessRuleValidation { rule_name, message } => {
                format!("Business Rule '{}': {}", rule_name, message)
            }
        }
    }
}
