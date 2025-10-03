use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT190: Advice of Charges, Interest and Other Adjustments
// Used to advise charges, interest and other adjustments that have been
// debited or credited to an account.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT190 {
    // Transaction Reference Number
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Account Identification
    #[serde(rename = "25")]
    pub field_25: Field25NoOption,

    // Value Date, Currency Code, Amount (can be 32C or 32D for MT190)
    #[serde(flatten)]
    pub field_32: Field32AmountCD,

    // Ordering Institution (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Details of Charges
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    // Sender to Receiver Information (optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT190 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "190");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;

        // Parse amount - can be 32C or 32D for credit/debit adjustments
        let field_32 = parser.parse_variant_field::<Field32AmountCD>("32")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse mandatory field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT190 {
            field_20,
            field_21,
            field_25,
            field_32,
            field_52,
            field_71b,
            field_72,
        })
    }

    /// Static validation rules for MT190
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F25", "description": "Field 25 must contain valid account identification"},
            {"id": "F32", "description": "Field 32C or 32D must contain valid currency and amount"},
            {"id": "F71B", "description": "Field 71B must contain at least one line of charge details"}
        ]}"#
    }

    /// Validate the message instance according to MT190 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT190: Field 20 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let related_ref = &self.field_21.reference;
        if related_ref.starts_with('/') || related_ref.ends_with('/') || related_ref.contains("//")
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT190: Field 21 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 71B has content
        if self.field_71b.details.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT190: Field 71B must contain at least one line of charge details"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT190
impl crate::traits::SwiftMessageBody for MT190 {
    fn message_type() -> &'static str {
        "190"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.field_21.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.field_25.to_swift_string());
        result.push_str("\r\n");

        match &self.field_32 {
            Field32AmountCD::C(f) => result.push_str(&f.to_swift_string()),
            Field32AmountCD::D(f) => result.push_str(&f.to_swift_string()),
        }
        result.push_str("\r\n");

        if let Some(ref field) = self.field_52 {
            match field {
                Field52OrderingInstitution::A(f) => result.push_str(&f.to_swift_string()),
                Field52OrderingInstitution::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        result.push_str(&self.field_71b.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field) = self.field_72 {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        // Remove trailing \r\n
        if result.ends_with("\r\n") {
            result.truncate(result.len() - 2);
        }

        result
    }
}
