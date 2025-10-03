use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    fn from_fields(fields: HashMap<String, Vec<(String, usize)>>) -> crate::SwiftResult<Self> {
        // Reconstruct block4 from fields
        let mut all_fields: Vec<(String, String, usize)> = Vec::new();
        for (tag, values) in fields {
            for (value, position) in values {
                all_fields.push((tag.clone(), value, position));
            }
        }

        // Sort by position
        all_fields.sort_by_key(|f| f.2);

        // Build block4
        let mut block4 = String::new();
        for (tag, value, _) in all_fields {
            block4.push_str(&format!(":{}:{}\n", tag, value));
        }

        Self::parse_from_block4(&block4)
    }

    fn from_fields_with_config(
        fields: HashMap<String, Vec<(String, usize)>>,
        _config: &ParserConfig,
    ) -> Result<ParseResult<Self>, ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(ParseResult::Success(msg)),
            Err(e) => Err(e),
        }
    }

    fn to_fields(&self) -> HashMap<String, Vec<String>> {
        let mut fields = HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.reference.clone()]);
        fields.insert(
            "32A".to_string(),
            vec![format!(
                "{}{}{}",
                self.field_32a.value_date.format("%y%m%d"),
                self.field_32a.currency,
                self.field_32a.amount.to_string().replace('.', ",")
            )],
        );

        if let Some(ref field_53b) = self.field_53b {
            fields.insert("53B".to_string(), vec![field_53b.to_swift_value()]);
        }

        if let Some(ref field_56) = self.field_56 {
            match field_56 {
                Field56IntermediaryAD::A(f) => {
                    fields.insert("56A".to_string(), vec![f.to_swift_value()]);
                }
                Field56IntermediaryAD::D(f) => {
                    fields.insert("56D".to_string(), vec![f.to_swift_value()]);
                }
            }
        }

        match &self.field_57 {
            Field57DebtInstitution::A(f) => {
                let mut value = String::new();
                if let Some(ref party) = f.party_identifier {
                    value.push_str(&format!("/{}\n", party));
                }
                value.push_str(&f.bic);
                fields.insert("57A".to_string(), vec![value]);
            }
            Field57DebtInstitution::B(f) => {
                let mut value = String::new();
                if let Some(ref party) = f.party_identifier {
                    value.push_str(&format!("/{}\n", party));
                }
                if let Some(ref loc) = f.location {
                    value.push_str(loc);
                }
                fields.insert("57B".to_string(), vec![value]);
            }
            Field57DebtInstitution::D(f) => {
                let mut lines = Vec::new();
                if let Some(ref party) = f.party_identifier {
                    lines.push(format!("/{}", party));
                }
                lines.extend(f.name_and_address.clone());
                fields.insert("57D".to_string(), vec![lines.join("\n")]);
            }
        }

        if let Some(ref field_72) = self.field_72 {
            fields.insert("72".to_string(), vec![field_72.information.join("\n")]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "32A", "57"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["53B", "56", "72"]
    }
}
