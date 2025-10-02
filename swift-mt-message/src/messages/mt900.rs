use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT900: Confirmation of Debit
// Used to confirm that a debit entry has been posted to an account.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT900 {
    #[serde(rename = "20")]
    pub field_20: Field20,

    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    #[serde(rename = "25")]
    pub field_25: Field25AccountIdentification,

    #[serde(rename = "13D")]
    pub field_13d: Option<Field13D>,

    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    #[serde(rename = "72")]
    pub field_72: Option<Field72>,
}

impl MT900 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "900");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25AccountIdentification>("25")?;
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional fields
        let field_13d = parser.parse_optional_field::<Field13D>("13D")?;
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Verify all content is consumed
        if !parser.is_complete() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "Unparsed content remaining in message: {}",
                    parser.remaining()
                ),
            });
        }

        Ok(Self {
            field_20,
            field_21,
            field_25,
            field_13d,
            field_32a,
            field_52,
            field_72,
        })
    }

    /// Validation rules for the message
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#
    }

    /// Parse from SWIFT MT text format
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        // If input starts with block headers, extract Block 4
        let block4 = if input.starts_with("{") {
            crate::parser::SwiftParser::extract_block(input, 4)?.ok_or_else(|| {
                crate::errors::ParseError::InvalidFormat {
                    message: "Block 4 not found".to_string(),
                }
            })?
        } else {
            // Assume input is already block 4 content
            input.to_string()
        };
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        // Add fields in order
        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");
        result.push_str(&self.field_21.to_swift_string());
        result.push_str("\r\n");
        result.push_str(&self.field_25.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field_13d) = self.field_13d {
            result.push_str(&field_13d.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_32a.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field_52) = self.field_52 {
            result.push_str(&field_52.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_72) = self.field_72 {
            result.push_str(&field_72.to_swift_string());
            result.push_str("\r\n");
        }

        result.push('-');
        result
    }
}

impl crate::traits::SwiftMessageBody for MT900 {
    fn message_type() -> &'static str {
        "900"
    }

    fn from_fields(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
    ) -> crate::SwiftResult<Self> {
        // Collect all fields with their positions
        let mut all_fields: Vec<(String, String, usize)> = Vec::new();
        for (tag, values) in fields {
            for (value, position) in values {
                all_fields.push((tag.clone(), value, position));
            }
        }

        // Sort by position to preserve field order
        all_fields.sort_by_key(|(_, _, pos)| *pos);

        // Reconstruct block4 in the correct order
        let mut block4 = String::new();
        for (tag, value, _) in all_fields {
            block4.push_str(&format!(
                ":{}:{}
",
                tag, value
            ));
        }
        Self::parse_from_block4(&block4)
    }
    fn from_fields_with_config(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
        _config: &crate::errors::ParserConfig,
    ) -> std::result::Result<crate::errors::ParseResult<Self>, crate::errors::ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(crate::errors::ParseResult::Success(msg)),
            Err(e) => Err(e),
        }
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        use crate::traits::SwiftField;
        let mut fields = std::collections::HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.to_swift_value()]);
        fields.insert("21".to_string(), vec![self.field_21.to_swift_value()]);
        fields.insert("25".to_string(), vec![self.field_25.to_swift_value()]);
        fields.insert("32A".to_string(), vec![self.field_32a.to_swift_value()]);

        if let Some(ref field) = self.field_13d {
            fields.insert("13D".to_string(), vec![field.to_swift_value()]);
        }

        if let Some(ref field) = self.field_52 {
            if let Some(variant_tag) = field.get_variant_tag() {
                fields.insert(format!("52{}", variant_tag), vec![field.to_swift_value()]);
            } else {
                fields.insert("52".to_string(), vec![field.to_swift_value()]);
            }
        }

        if let Some(ref field) = self.field_72 {
            fields.insert("72".to_string(), vec![field.to_swift_value()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "25", "32A"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["13D", "52", "72"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt900_parse() {
        let mt900_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
-"#;
        let result = MT900::parse_from_block4(mt900_text);
        assert!(result.is_ok());
        let mt900 = result.unwrap();
        assert_eq!(mt900.field_20.reference, "20240719001");
        assert_eq!(mt900.field_21.reference, "REF20240719001");
    }
}
