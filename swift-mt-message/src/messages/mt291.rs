use crate::errors::{ParseError, ParseResult, ParserConfig};
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

        fields.insert("20".to_string(), vec![self.field_20.to_swift_string()]);
        fields.insert("21".to_string(), vec![self.field_21.to_swift_string()]);
        fields.insert("32B".to_string(), vec![self.field_32b.to_swift_string()]);

        if let Some(ref ord_inst) = self.field_52 {
            match ord_inst {
                Field52OrderingInstitution::A(f) => {
                    fields.insert("52A".to_string(), vec![f.to_swift_string()]);
                }
                Field52OrderingInstitution::D(f) => {
                    fields.insert("52D".to_string(), vec![f.to_swift_string()]);
                }
            }
        }

        if let Some(ref acc_with) = self.field_57 {
            match acc_with {
                Field57AccountWithABD::A(f) => {
                    fields.insert("57A".to_string(), vec![f.to_swift_string()]);
                }
                Field57AccountWithABD::B(f) => {
                    fields.insert("57B".to_string(), vec![f.to_swift_string()]);
                }
                Field57AccountWithABD::D(f) => {
                    fields.insert("57D".to_string(), vec![f.to_swift_string()]);
                }
            }
        }

        fields.insert("71B".to_string(), vec![self.field_71b.to_swift_string()]);

        if let Some(ref sender_info) = self.field_72 {
            fields.insert("72".to_string(), vec![sender_info.to_swift_string()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "32B", "71B"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["52", "57", "72"]
    }
}
