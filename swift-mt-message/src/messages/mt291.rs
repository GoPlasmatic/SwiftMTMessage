use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use serde::{Deserialize, Serialize};

/// MT291 - Request for Payment of Charges, Interest and Other Expenses
///
/// Used by financial institutions to request payment of charges, interest and other expenses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT291 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Mandatory)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Field 32B - Currency Code, Amount (Mandatory)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    /// Field 52 - Ordering Institution (Optional)
    /// Can be 52A or 52D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Field 57 - Account With Institution (Optional)
    /// Can be 57A, 57B, or 57D per MT n91 specification
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57AccountWithABD>,

    /// Field 71B - Details of Charges (Mandatory)
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT291 {
    /// Parse MT291 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "291");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_32b = parser.parse_field::<Field32B>("32B")?;

        // Parse optional Field 52 - Ordering Institution
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse optional Field 57 - Account With Institution (A, B, D only per spec)
        let field_57 = parser.parse_optional_variant_field::<Field57AccountWithABD>("57")?;

        // Parse mandatory Field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional Field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT291 {
            field_20,
            field_21,
            field_32b,
            field_52,
            field_57,
            field_71b,
            field_72,
        })
    }

    /// Static validation rules for MT291
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT291 {
    fn message_type() -> &'static str {
        "291"
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
                Field57AccountWithABD::A(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithABD::B(f) => result.push_str(&f.to_swift_string()),
                Field57AccountWithABD::D(f) => result.push_str(&f.to_swift_string()),
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
