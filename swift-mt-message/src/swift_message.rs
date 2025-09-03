//! Complete SWIFT message with headers and body

use crate::{
    Result, ValidationError, ValidationResult,
    errors::ParseError,
    headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader},
    messages,
    parser::extract_base_tag,
    traits::SwiftMessageBody,
};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Complete SWIFT message with headers and body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwiftMessage<T: SwiftMessageBody> {
    /// Basic Header (Block 1)
    pub basic_header: BasicHeader,

    /// Application Header (Block 2)
    pub application_header: ApplicationHeader,

    /// User Header (Block 3) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_header: Option<UserHeader>,

    /// Trailer (Block 5) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailer: Option<Trailer>,

    /// Message type identifier
    pub message_type: String,

    /// Parsed message body with typed fields
    pub fields: T,
}

impl<T: SwiftMessageBody> SwiftMessage<T> {
    /// Check if this message contains reject codes (MT103 specific)
    ///
    /// Reject messages are identified by checking:
    /// 1. Field 20 (Sender's Reference) for "REJT" prefix
    /// 2. Block 3 field 108 (MUR - Message User Reference) for "REJT"
    /// 3. Field 72 (Sender to Receiver Information) containing `/REJT/` code
    pub fn has_reject_codes(&self) -> bool {
        // Check Block 3 field 108 (MUR - Message User Reference)
        if let Some(ref user_header) = self.user_header
            && let Some(ref mur) = user_header.message_user_reference
            && mur.to_uppercase().contains("REJT")
        {
            return true;
        }

        if let Some(mt103_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT103>()
        {
            return mt103_fields.has_reject_codes();
        } else if let Some(mt202_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT202>()
        {
            return mt202_fields.has_reject_codes();
        } else if let Some(mt205_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT205>()
        {
            return mt205_fields.has_reject_codes();
        }

        false
    }

    /// Check if this message contains return codes (MT103 specific)
    ///
    /// Return messages are identified by checking:
    /// 1. Field 20 (Sender's Reference) for "RETN" prefix
    /// 2. Block 3 field 108 (MUR - Message User Reference) for "RETN"
    /// 3. Field 72 (Sender to Receiver Information) containing `/RETN/` code
    pub fn has_return_codes(&self) -> bool {
        // Check Block 3 field 108 (MUR - Message User Reference)
        if let Some(ref user_header) = self.user_header
            && let Some(ref mur) = user_header.message_user_reference
            && mur.to_uppercase().contains("RETN")
        {
            return true;
        }

        if let Some(mt103_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT103>()
        {
            return mt103_fields.has_return_codes();
        } else if let Some(mt202_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT202>()
        {
            return mt202_fields.has_return_codes();
        } else if let Some(mt205_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT205>()
        {
            return mt205_fields.has_return_codes();
        }

        false
    }

    pub fn is_cover_message(&self) -> bool {
        if let Some(mt202_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT202>()
        {
            return mt202_fields.is_cover_message();
        }
        if let Some(mt205_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT205>()
        {
            return mt205_fields.is_cover_message();
        }

        false
    }

    pub fn is_stp_message(&self) -> bool {
        if let Some(mt103_fields) =
            (&self.fields as &dyn Any).downcast_ref::<crate::messages::MT103>()
        {
            return mt103_fields.is_stp_compliant();
        }

        false
    }

    /// Validate message against business rules using JSONLogic
    /// This validation method has access to both headers and message fields,
    /// allowing for comprehensive validation of MT103 and other message types.
    pub fn validate(&self) -> ValidationResult {
        // Check if the message type has validation rules
        let validation_rules = match T::message_type() {
            "101" => messages::MT101::validate(),
            "103" => messages::MT103::validate(),
            "104" => messages::MT104::validate(),
            "107" => messages::MT107::validate(),
            "110" => messages::MT110::validate(),
            "111" => messages::MT111::validate(),
            "112" => messages::MT112::validate(),
            "190" => messages::MT190::validate(),
            "191" => messages::MT191::validate(),
            "200" => messages::MT200::validate(),
            "202" => messages::MT202::validate(),
            "204" => messages::MT204::validate(),
            "205" => messages::MT205::validate(),
            "210" => messages::MT210::validate(),
            "290" => messages::MT290::validate(),
            "291" => messages::MT291::validate(),
            "900" => messages::MT900::validate(),
            "910" => messages::MT910::validate(),
            "920" => messages::MT920::validate(),
            "935" => messages::MT935::validate(),
            "940" => messages::MT940::validate(),
            "941" => messages::MT941::validate(),
            "942" => messages::MT942::validate(),
            "950" => messages::MT950::validate(),
            "192" => messages::MT192::validate(),
            "196" => messages::MT196::validate(),
            "292" => messages::MT292::validate(),
            "296" => messages::MT296::validate(),
            "199" => messages::MT199::validate(),
            "299" => messages::MT299::validate(),
            _ => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "UNSUPPORTED_MESSAGE_TYPE".to_string(),
                    message: format!(
                        "No validation rules defined for message type {}",
                        T::message_type()
                    ),
                });
            }
        };

        // Parse the validation rules JSON
        let rules_json: serde_json::Value = match serde_json::from_str(validation_rules) {
            Ok(json) => json,
            Err(e) => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "JSON_PARSE".to_string(),
                    message: format!("Failed to parse validation rules JSON: {e}"),
                });
            }
        };

        // Extract rules array from the JSON
        let rules = match rules_json.get("rules").and_then(|r| r.as_array()) {
            Some(rules) => rules,
            None => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "RULES_FORMAT".to_string(),
                    message: "Validation rules must contain a 'rules' array".to_string(),
                });
            }
        };

        // Get constants if they exist
        let constants = rules_json
            .get("constants")
            .and_then(|c| c.as_object())
            .cloned()
            .unwrap_or_default();

        // Create comprehensive data context with headers and fields
        let context_value = match self.create_validation_context(&constants) {
            Ok(context) => {
                // Debug: Always show validation context in debug mode
                if std::env::var("TEST_DEBUG").is_ok()
                    && let Ok(context_str) = serde_json::to_string_pretty(&context)
                {
                    eprintln!("\n=== VALIDATION CONTEXT for {} ===", T::message_type());
                    eprintln!("{}", context_str);
                    eprintln!("=== END VALIDATION CONTEXT ===\n");
                }
                context
            }
            Err(e) => {
                return ValidationResult::with_error(ValidationError::BusinessRuleValidation {
                    rule_name: "CONTEXT_CREATION".to_string(),
                    message: format!("Failed to create validation context: {e}"),
                });
            }
        };

        // Validate each rule using datalogic-rs
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for (rule_index, rule) in rules.iter().enumerate() {
            let rule_id = rule
                .get("id")
                .and_then(|id| id.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("RULE_{rule_index}"));

            let rule_description = rule
                .get("description")
                .and_then(|desc| desc.as_str())
                .unwrap_or("No description");

            if let Some(condition) = rule.get("condition") {
                // Create DataLogic instance for evaluation
                let dl = datalogic_rs::DataLogic::new();
                match dl.evaluate_json(condition, &context_value) {
                    Ok(result) => {
                        match result.as_bool() {
                            Some(true) => {
                                // Rule passed
                                continue;
                            }
                            Some(false) => {
                                // Rule failed
                                errors.push(ValidationError::BusinessRuleValidation {
                                    rule_name: rule_id.clone(),
                                    message: format!(
                                        "Business rule validation failed: {rule_id} - {rule_description}"
                                    ),
                                });
                            }
                            None => {
                                // Rule returned non-boolean value
                                warnings.push(format!(
                                    "Rule {rule_id} returned non-boolean value: {result:?}"
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        // JSONLogic evaluation error
                        errors.push(ValidationError::BusinessRuleValidation {
                            rule_name: rule_id.clone(),
                            message: format!("JSONLogic evaluation error for rule {rule_id}: {e}"),
                        });
                    }
                }
            } else {
                warnings.push(format!("Rule {rule_id} has no condition"));
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Create a comprehensive validation context that includes headers, fields, and constants
    fn create_validation_context(
        &self,
        constants: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        // Serialize the entire message (including headers) to JSON for data context
        let full_message_data = match serde_json::to_value(self) {
            Ok(data) => data,
            Err(e) => {
                return Err(ParseError::SerializationError {
                    message: format!("Failed to serialize complete message: {e}"),
                });
            }
        };

        // Create a comprehensive data context
        let mut data_context = serde_json::Map::new();

        // Add the complete message data
        if let serde_json::Value::Object(msg_obj) = full_message_data {
            for (key, value) in msg_obj {
                data_context.insert(key, value);
            }
        }

        // Add constants to data context
        for (key, value) in constants {
            data_context.insert(key.clone(), value.clone());
        }

        // Extract sender and receiver BIC from headers for enhanced validation context
        let (sender_country, receiver_country) = self.extract_country_codes_from_bics();

        // Add enhanced message context including BIC-derived information
        data_context.insert("message_context".to_string(), serde_json::json!({
            "message_type": self.message_type,
            "sender_country": sender_country,
            "receiver_country": receiver_country,
            "sender_bic": self.basic_header.logical_terminal,
            "receiver_bic": &self.application_header.destination_address,
            "message_priority": &self.application_header.priority,
            "delivery_monitoring": self.application_header.delivery_monitoring.as_ref().unwrap_or(&"3".to_string()),
        }));

        Ok(serde_json::Value::Object(data_context))
    }

    /// Extract country codes from BIC codes in the headers
    fn extract_country_codes_from_bics(&self) -> (String, String) {
        // Extract sender country from basic header BIC (positions 4-5)
        let sender_country = if self.basic_header.logical_terminal.len() >= 6 {
            self.basic_header.logical_terminal[4..6].to_string()
        } else {
            "XX".to_string() // Unknown country
        };

        // Extract receiver country from application header destination BIC
        let receiver_country = if self.application_header.destination_address.len() >= 6 {
            self.application_header.destination_address[4..6].to_string()
        } else {
            "XX".to_string()
        };

        (sender_country, receiver_country)
    }

    pub fn to_mt_message(&self) -> String {
        // Pre-allocate capacity based on typical message size
        // Headers ~200 chars + fields vary but typically 20-100 chars each
        let estimated_size = 200 + self.fields.to_fields().len() * 50;
        let mut swift_message = String::with_capacity(estimated_size);

        // Block 1: Basic Header
        let block1 = &self.basic_header.to_string();
        swift_message.push_str(&format!("{{1:{block1}}}\n"));

        // Block 2: Application Header
        let block2 = &self.application_header.to_string();
        swift_message.push_str(&format!("{{2:{block2}}}\n"));

        // Block 3: User Header (if present)
        if let Some(ref user_header) = self.user_header {
            let block3 = &user_header.to_string();
            swift_message.push_str(&format!("{{3:{block3}}}\n"));
        }

        // Block 4: Text Block with fields
        let mut block4 = String::new();

        // Get optional field tags for this message type to determine which fields can be skipped
        let optional_fields: std::collections::HashSet<String> = T::optional_fields()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Use to_ordered_fields for proper sequence ordering
        let ordered_fields = self.fields.to_ordered_fields();

        // Output fields in the correct order
        for (field_tag, field_value) in ordered_fields {
            // Skip empty optional fields
            if optional_fields.contains(&field_tag) && field_value.trim().is_empty() {
                continue;
            }

            // field_value already includes the field tag prefix from to_swift_string()
            // but we need to check if it starts with ':' to avoid double prefixing
            if field_value.starts_with(':') {
                // Value already has field tag prefix, use as-is
                block4.push_str(&format!("\n{field_value}"));
            } else {
                // Value doesn't have field tag prefix, add it
                block4.push_str(&format!(
                    "\n:{}:{field_value}",
                    extract_base_tag(&field_tag)
                ));
            }
        }

        swift_message.push_str(&format!("{{4:{block4}\n-}}\n"));

        // Block 5: Trailer (if present)
        if let Some(ref trailer) = self.trailer {
            let block5 = &trailer.to_string();
            swift_message.push_str(&format!("{{5:{block5}}}\n"));
        }

        swift_message
    }
}
