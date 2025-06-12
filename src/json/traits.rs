//! Core traits for JSON conversion

use crate::errors::{ParseError, Result};
use serde_json::Value;

/// Convert SWIFT message to JSON
pub trait ToJson {
    /// Convert to JSON Value
    fn to_json(&self) -> Result<Value>;

    /// Convert to JSON string
    fn to_json_string(&self) -> Result<String> {
        let json_value = self.to_json()?;
        serde_json::to_string_pretty(&json_value).map_err(|e| ParseError::JsonError {
            message: format!("Failed to serialize to JSON: {}", e),
        })
    }

    /// Convert to compact JSON string
    fn to_json_compact(&self) -> Result<String> {
        let json_value = self.to_json()?;
        serde_json::to_string(&json_value).map_err(|e| ParseError::JsonError {
            message: format!("Failed to serialize to JSON: {}", e),
        })
    }
}

/// Convert from JSON to SWIFT message
pub trait FromJson<T> {
    /// Parse from JSON Value
    fn from_json(json: &Value) -> Result<T>;

    /// Parse from JSON string
    fn from_json_string(json_str: &str) -> Result<T> {
        let json_value: Value =
            serde_json::from_str(json_str).map_err(|e| ParseError::JsonError {
                message: format!("Invalid JSON: {}", e),
            })?;
        Self::from_json(&json_value)
    }
}
