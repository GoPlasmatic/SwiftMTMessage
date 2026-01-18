//! # SwiftMessage
//!
//! Complete SWIFT message with headers (Blocks 1-3, 5) and typed message body (Block 4).

use crate::{
    ValidationError, ValidationResult,
    headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader},
    traits::SwiftMessageBody,
};
use serde::{Deserialize, Serialize};
use std::any::Any;

/// Complete SWIFT message (headers + typed body)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "jsonschema",
    derive(schemars::JsonSchema),
    schemars(bound = "T: schemars::JsonSchema")
)]
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
    /// Check if message contains reject codes (REJT in field 20, block 3 MUR, or field 72)
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

    /// Check if message contains return codes (RETN in field 20, block 3 MUR, or field 72)
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

    /// Validate message using SWIFT SR2025 network validation rules
    pub fn validate(&self) -> ValidationResult {
        // Use the new validate_network_rules method
        let validation_errors = self.fields.validate_network_rules(false);

        // Convert SwiftValidationError to ValidationError for backward compatibility
        let errors: Vec<ValidationError> = validation_errors
            .into_iter()
            .map(|swift_error| {
                let message = format!("{}", swift_error);
                ValidationError::BusinessRuleValidation {
                    rule_name: swift_error.error_code().to_string(),
                    message,
                }
            })
            .collect();

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    pub fn to_mt_message(&self) -> String {
        // Pre-allocate capacity based on typical message size
        // Headers ~200 chars + typical message body ~2000 chars
        let mut swift_message = String::with_capacity(2200);

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
        // Use the message type's to_mt_string() implementation
        let mut block4_content = self.fields.to_mt_string();

        // Convert \r\n to \n for consistency with existing format
        if block4_content.contains("\r\n") {
            block4_content = block4_content.replace("\r\n", "\n");
        }

        // Add leading newline if content doesn't already have one
        let block4 = if block4_content.starts_with('\n') {
            block4_content
        } else {
            format!("\n{}", block4_content)
        };

        swift_message.push_str(&format!("{{4:{block4}\n-}}\n"));

        // Block 5: Trailer (if present)
        if let Some(ref trailer) = self.trailer {
            let block5 = &trailer.to_string();
            swift_message.push_str(&format!("{{5:{block5}}}\n"));
        }

        swift_message
    }
}
