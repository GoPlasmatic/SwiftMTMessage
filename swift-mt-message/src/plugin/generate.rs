use async_trait::async_trait;
use datafake_rs::DataGenerator;
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

pub struct Generate;

#[async_trait]
impl AsyncFunctionHandler for Generate {
    #[instrument(skip(self, message, config, _datalogic))]
    async fn execute(
        &self,
        message: &mut Message,
        config: &FunctionConfig,
        _datalogic: Arc<DataLogic>,
    ) -> Result<(usize, Vec<Change>)> {
        debug!("Starting datafake generation");

        // Extract configuration
        let input = match config {
            FunctionConfig::Custom { input, name: _ } => input,
            _ => {
                return Err(DataflowError::Validation(
                    "Invalid configuration type".to_string(),
                ));
            }
        };

        // Get schema and generated field names
        let schema_field = input.get("schema").and_then(Value::as_str).ok_or_else(|| {
            DataflowError::Validation("'schema' parameter is required".to_string())
        })?;

        let generated_field = input
            .get("generated")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                DataflowError::Validation("'generated' parameter is required".to_string())
            })?;

        // Get the datafake schema from the schema field
        let schema = message.data().get(schema_field).cloned().ok_or_else(|| {
            DataflowError::Validation(format!(
                "Schema field '{}' not found in message data",
                schema_field
            ))
        })?;

        debug!(
            schema_field = %schema_field,
            generated_field = %generated_field,
            "Generating data using datafake"
        );

        // Generate data using datafake
        let generated_data = match DataGenerator::from_value(schema.clone()) {
            Ok(generator) => generator.generate().map_err(|e| {
                error!(error = ?e, "Datafake generation failed");
                DataflowError::Validation(format!("Datafake generation failed: {}", e))
            })?,
            Err(e) => {
                error!(error = ?e, "Failed to create datafake generator from schema");
                return Err(DataflowError::Validation(format!(
                    "Invalid datafake schema: {}",
                    e
                )));
            }
        };

        // Store the generated data in the generated field
        let old_value = message
            .data()
            .get(generated_field)
            .cloned()
            .unwrap_or(Value::Null);

        message
            .data_mut()
            .as_object_mut()
            .ok_or_else(|| DataflowError::Validation("Message data must be an object".to_string()))?
            .insert(generated_field.to_string(), generated_data.clone());

        // Invalidate cache after modification
        message.invalidate_context_cache();

        debug!("Successfully generated data");

        Ok((
            200,
            vec![Change {
                path: Arc::from(format!("data.{}", generated_field)),
                old_value: Arc::new(old_value),
                new_value: Arc::new(generated_data),
            }],
        ))
    }
}
