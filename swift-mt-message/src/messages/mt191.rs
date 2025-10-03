use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT191: Request for Payment of Charges, Interest and Other Expenses
// Used to request payment of charges, interest and other expenses from
// another financial institution.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT191 {
    // Transaction Reference Number
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Currency Code, Amount
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Ordering Institution (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Account With Institution (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57AccountWithInstitution>,

    // Details of Charges
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    // Sender to Receiver Information (optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT191 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "191");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_32b = parser.parse_field::<Field32B>("32B")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_57 =
            parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;

        // Parse mandatory field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT191 {
            field_20,
            field_21,
            field_32b,
            field_52,
            field_57,
            field_71b,
            field_72,
        })
    }

    /// Static validation rules for MT191
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F32B", "description": "Field 32B must contain valid currency and positive amount"},
            {"id": "F71B", "description": "Field 71B must contain at least one line of charge details"}
        ]}"#
    }

    /// Validate the message instance according to MT191 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT191: Field 20 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let related_ref = &self.field_21.reference;
        if related_ref.starts_with('/') || related_ref.ends_with('/') || related_ref.contains("//")
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT191: Field 21 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 71B has content
        if self.field_71b.details.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT191: Field 71B must contain at least one line of charge details"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT191
impl crate::traits::SwiftMessageBody for MT191 {
    fn message_type() -> &'static str {
        "191"
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

        result.push_str(&self.field_32b.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field) = self.field_52 {
            match field {
                Field52OrderingInstitution::A(f) => result.push_str(&f.to_swift_string()),
                Field52OrderingInstitution::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.field_57 {
            match field {
                Field57AccountWithInstitution::A(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::B(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::C(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithInstitution::D(f) => result.push_str(&f.to_swift_string()),
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
