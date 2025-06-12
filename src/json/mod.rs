//! JSON conversion utilities for SWIFT MT messages
//!
//! This module provides bidirectional conversion between SWIFT MT messages
//! and JSON format, supporting both message-level and field-level serialization.

pub mod field_converter;
pub mod message_converter;
pub mod traits;

pub use message_converter::{JsonBlocks, JsonMessage, MessageMetadata};
pub use traits::{FromJson, ToJson};

/// High-level utility functions for common use cases
pub mod utils {
    use super::traits::{FromJson, ToJson};
    use crate::errors::{ParseError, Result};
    use crate::field_parser::SwiftMessage;
    use crate::mt_models::mt103::MT103;
    use crate::mt_models::mt103_stp::MT103STP;
    use serde_json::Value;

    /// Configuration for JSON conversion
    #[derive(Debug, Clone)]
    pub struct JsonConversionOptions {
        /// Whether to include pretty formatting
        pub pretty_format: bool,
        /// Whether to include metadata
        pub include_metadata: bool,
        /// Whether to preserve original field order
        pub preserve_field_order: bool,
    }

    impl Default for JsonConversionOptions {
        fn default() -> Self {
            Self {
                pretty_format: true,
                include_metadata: false,
                preserve_field_order: true,
            }
        }
    }

    /// Parse MT message from SWIFT format and convert to JSON
    pub fn swift_to_json(swift_message: &str) -> Result<String> {
        swift_to_json_with_options(swift_message, JsonConversionOptions::default())
    }

    /// Parse MT message from SWIFT format and convert to JSON with options
    pub fn swift_to_json_with_options(
        swift_message: &str,
        options: JsonConversionOptions,
    ) -> Result<String> {
        let message = SwiftMessage::parse(swift_message)?;
        if options.pretty_format {
            message.to_json_string()
        } else {
            message.to_json_compact()
        }
    }

    /// Parse JSON and convert to SWIFT format
    pub fn json_to_swift(json_str: &str) -> Result<String> {
        validate_json_input(json_str)?;
        let message = SwiftMessage::from_json_string(json_str)?;
        super::message_converter::swift_message_to_swift_format(&message)
    }

    /// Parse specific MT103 from JSON
    pub fn json_to_mt103(json_str: &str) -> Result<MT103> {
        validate_json_input(json_str)?;
        MT103::from_json_string(json_str)
    }

    /// Convert MT103 to JSON
    pub fn mt103_to_json(mt103: &MT103) -> Result<String> {
        mt103.to_json_string()
    }

    /// Parse specific MT103-STP from JSON
    pub fn json_to_mt103_stp(json_str: &str) -> Result<MT103STP> {
        validate_json_input(json_str)?;
        let swift_message = SwiftMessage::from_json_string(json_str)?;
        MT103STP::from_swift_message(swift_message)
    }

    /// Convert MT103-STP to JSON
    pub fn mt103_stp_to_json(mt103_stp: &MT103STP) -> Result<String> {
        let swift_message = mt103_stp.to_swift_message();
        swift_message.to_json_string()
    }

    /// Validate and pretty-print JSON
    pub fn prettify_json(json_str: &str) -> Result<String> {
        let value: Value = serde_json::from_str(json_str).map_err(|e| ParseError::JsonError {
            message: format!("Invalid JSON input: {}", e),
        })?;

        serde_json::to_string_pretty(&value).map_err(|e| ParseError::JsonError {
            message: format!("Failed to prettify JSON: {}", e),
        })
    }

    /// Compact JSON (remove formatting)
    pub fn compact_json(json_str: &str) -> Result<String> {
        let value: Value = serde_json::from_str(json_str).map_err(|e| ParseError::JsonError {
            message: format!("Invalid JSON input: {}", e),
        })?;

        serde_json::to_string(&value).map_err(|e| ParseError::JsonError {
            message: format!("Failed to compact JSON: {}", e),
        })
    }

    /// Validate JSON structure for SWIFT messages
    pub fn validate_swift_json(json_str: &str) -> Result<()> {
        let value: Value = serde_json::from_str(json_str).map_err(|e| ParseError::JsonError {
            message: format!("Invalid JSON format: {}", e),
        })?;

        // Basic validation for SWIFT JSON structure
        if let Some(obj) = value.as_object() {
            if !obj.contains_key("message_type") {
                return Err(ParseError::JsonError {
                    message: "Missing required 'message_type' field".to_string(),
                });
            }
            if !obj.contains_key("fields") {
                return Err(ParseError::JsonError {
                    message: "Missing required 'fields' field".to_string(),
                });
            }
        } else {
            return Err(ParseError::JsonError {
                message: "JSON must be an object".to_string(),
            });
        }

        Ok(())
    }

    /// Convert between different MT message types via JSON
    pub fn convert_mt_message<F, T>(source: &F) -> Result<T>
    where
        F: ToJson,
        T: for<'a> FromJson<T>,
    {
        let json_value = source.to_json()?;
        T::from_json(&json_value)
    }

    /// Helper function to validate JSON input
    fn validate_json_input(json_str: &str) -> Result<()> {
        if json_str.trim().is_empty() {
            return Err(ParseError::JsonError {
                message: "Empty JSON input".to_string(),
            });
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_json_conversion_options() {
            let options = JsonConversionOptions::default();
            assert!(options.pretty_format);
            assert!(!options.include_metadata);
            assert!(options.preserve_field_order);
        }

        #[test]
        fn test_validate_json_input() {
            assert!(validate_json_input("{}").is_ok());
            assert!(validate_json_input("").is_err());
            assert!(validate_json_input("   ").is_err());
        }

        #[test]
        fn test_prettify_compact_json() {
            let compact = r#"{"test":"value"}"#;

            let pretty_result = prettify_json(compact);
            assert!(pretty_result.is_ok());

            let pretty_json = pretty_result.unwrap();
            assert!(pretty_json.contains('\n'));

            let compact_result = compact_json(&pretty_json);
            assert!(compact_result.is_ok());
            assert!(!compact_result.unwrap().contains('\n'));
        }

        #[test]
        fn test_validate_swift_json() {
            let valid_json = r#"{"message_type":"103","fields":{}}"#;
            assert!(validate_swift_json(valid_json).is_ok());

            let invalid_json = r#"{"fields":{}}"#;
            assert!(validate_swift_json(invalid_json).is_err());
        }

        #[test]
        fn test_json_structure_improvement() {
            let swift_message = r#"{1:F01BNPAFRPPXXX0000000000}{2:O1031234240101DEUTDEFFXXXX12345678952401011234N}{4:
:20:FT21001234567890
:23B:CRED
:32A:240101USD1000,00
:50K:/1234567890
ACME CORPORATION
123 MAIN STREET
NEW YORK NY 10001
:57A:DEUTDEFFXXX
:59:/DE89370400440532013000
MUELLER GMBH
HAUPTSTRASSE 1
10115 BERLIN
:71A:OUR
-}"#;

            let json_result = swift_to_json(swift_message);
            assert!(json_result.is_ok(), "JSON conversion should succeed");
            
            let json_value: serde_json::Value = serde_json::from_str(&json_result.unwrap()).unwrap();
            let fields = json_value.get("fields").unwrap().as_object().unwrap();
            
            // Verify that fields no longer have extra nesting layers
            // Field20 should have transaction_reference directly
            let field20 = fields.get("20").unwrap();
            assert!(field20.get("transaction_reference").is_some());
            assert!(field20.get("Field20").is_none(), "Field20 should not have extra nesting");
            
            // Field23B should have bank_operation_code directly  
            let field23b = fields.get("23B").unwrap();
            assert!(field23b.get("bank_operation_code").is_some());
            assert!(field23b.get("Field23B").is_none(), "Field23B should not have extra nesting");
            
            // Field32A should have currency, amount etc. directly
            let field32a = fields.get("32A").unwrap();
            assert!(field32a.get("currency").is_some());
            assert!(field32a.get("amount").is_some());
            assert!(field32a.get("Field32A").is_none(), "Field32A should not have extra nesting");
            
            // Field71A should have details_of_charges directly
            let field71a = fields.get("71A").unwrap();
            assert!(field71a.get("details_of_charges").is_some());
            assert!(field71a.get("Field71A").is_none(), "Field71A should not have extra nesting");
            
            // Enum-based fields should still work correctly (these have legitimate structure)
            let field50k = fields.get("50K").unwrap();
            assert!(field50k.get("name_and_address").is_some());
            
            let field57a = fields.get("57A").unwrap(); 
            assert!(field57a.get("bic").is_some());
            
            let field59 = fields.get("59").unwrap();
            assert!(field59.get("beneficiary_customer").is_some());
            
            println!("Improved JSON structure (no extra nesting):");
            println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        }
    }
}
