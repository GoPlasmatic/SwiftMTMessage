use crate::errors::ParseError;
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::parsing_utils::*;
use serde::{Deserialize, Serialize};

/// MT200 - Financial Institution Transfer for Own Account
///
/// Used by financial institutions to transfer funds for their own account,
/// typically for nostro account funding, liquidity management, or internal transfers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT200 {
    /// Field 20 - Transaction Reference Number (Mandatory)
    #[serde(rename = "20")]
    pub field_20: Field20,

    /// Field 32A - Value Date, Currency Code, Amount (Mandatory)
    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    /// Field 53B - Sender's Correspondent (Optional)
    #[serde(rename = "53B", skip_serializing_if = "Option::is_none")]
    pub field_53b: Option<Field53B>,

    /// Field 56 - Intermediary Institution (Optional)
    /// Can be 56A or 56D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_56: Option<Field56IntermediaryAD>,

    /// Field 57 - Account With Institution (Mandatory)
    #[serde(flatten)]
    pub field_57: Field57DebtInstitution,

    /// Field 72 - Sender to Receiver Information (Optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT200 {
    /// Parse MT200 from a raw SWIFT message string
    pub fn parse_from_block4(block4: &str) -> Result<Self, ParseError> {
        let mut parser = MessageParser::new(block4, "200");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional Field 53B - Sender's Correspondent
        let field_53b = parser.parse_optional_field::<Field53B>("53B")?;

        // Parse optional Field 56 - Intermediary Institution
        let field_56 = parser.parse_optional_variant_field::<Field56IntermediaryAD>("56")?;

        // Parse mandatory Field 57 - Account With Institution
        let field_57 = parser.parse_variant_field::<Field57DebtInstitution>("57")?;

        // Parse optional Field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT200 {
            field_20,
            field_32a,
            field_53b,
            field_56,
            field_57,
            field_72,
        })
    }

    /// Static validation rules for MT200
    pub fn validate() -> &'static str {
        r#"{"rules": []}"#
    }
}

impl crate::traits::SwiftMessageBody for MT200 {
    fn message_type() -> &'static str {
        "200"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        append_field(&mut result, &self.field_20);
        append_field(&mut result, &self.field_32a);
        append_optional_field(&mut result, &self.field_53b);
        append_optional_field(&mut result, &self.field_56);
        append_field(&mut result, &self.field_57);
        append_optional_field(&mut result, &self.field_72);

        finalize_mt_string(result, false)
    }
}
