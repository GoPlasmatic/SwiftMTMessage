use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

/// MT290 - Advice of Charges, Interest and Other Adjustments
///
/// Used by financial institutions to advise charges, interest and other adjustments
/// that have been debited/credited to an account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT290 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 21 - Related Reference (Mandatory)
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    /// Field 25 - Account Identification (Mandatory)
    #[serde(rename = "25")]
    pub field_25: Field25NoOption,

    /// Field 32 - Value Date, Currency Code, Amount (Mandatory)
    /// Can be 32C (credit) or 32D (debit)
    #[serde(flatten)]
    pub field_32: Field32AmountCD,

    /// Field 52 - Ordering Institution (Optional)
    /// Can be 52A or 52D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    /// Field 71B - Details of Charges (Mandatory)
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT290 {
    /// Parse MT290 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "290");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;

        // Parse Field 25 - Account Identification
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;

        // Parse Field 32 - variant field (32C or 32D only per spec)
        let field_32 = parser.parse_variant_field::<Field32AmountCD>("32")?;

        // Parse optional Field 52 - Ordering Institution
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse mandatory Field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional Field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT290 {
            field_20,
            field_21,
            field_25,
            field_32,
            field_52,
            field_71b,
            field_72,
        })
    }

    /// Static validation rules for MT290
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT290 {
    fn message_type() -> &'static str {
        "290"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_21);
        append_field(&mut result, &self.field_25);
        append_field(&mut result, &self.field_32);
        append_optional_field(&mut result, &self.field_52);
        append_field(&mut result, &self.field_71b);
        append_optional_field(&mut result, &self.field_72);

        finalize_mt_string(result, false)
    }
}
