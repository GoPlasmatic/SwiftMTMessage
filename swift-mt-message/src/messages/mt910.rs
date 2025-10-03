use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT910: Confirmation of Credit
// Used to confirm that a credit entry has been posted to an account.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT910 {
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
    pub field_50: Option<Field50OrderingCustomerAFK>,

    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    #[serde(flatten)]
    pub field_56: Option<Field56Intermediary>,

    #[serde(rename = "72")]
    pub field_72: Option<Field72>,
}

impl MT910 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "910");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25AccountIdentification>("25")?;
        let field_32a = parser.parse_field::<Field32A>("32A")?;

        // Parse optional fields
        let field_13d = parser.parse_optional_field::<Field13D>("13D")?;
        let field_50 = parser.parse_optional_variant_field::<Field50OrderingCustomerAFK>("50")?;
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
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
            field_50,
            field_52,
            field_56,
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
        #[cfg(test)]
        println!("MT910 block4: {}", block4);
        let message = Self::parse_from_block4(&block4)?;

        // Apply validation rule C1
        message.validate_c1()?;

        Ok(message)
    }

    /// Validation rule C1: Either field 50 or field 52 must be present
    fn validate_c1(&self) -> Result<(), crate::errors::ParseError> {
        #[cfg(test)]
        {
            println!(
                "MT910 validation - field_50: {:?}, field_52: {:?}",
                self.field_50.is_some(),
                self.field_52.is_some()
            );
        }
        if self.field_50.is_none() && self.field_52.is_none() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT910: Either field 50 (Ordering Customer) or field 52 (Ordering Institution) must be present".to_string()
            });
        }
        Ok(())
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

        if let Some(ref field_50) = self.field_50 {
            result.push_str(&field_50.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_52) = self.field_52 {
            result.push_str(&field_52.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_56) = self.field_56 {
            result.push_str(&field_56.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_72) = self.field_72 {
            result.push_str(&field_72.to_swift_string());
            result.push_str("\r\n");
        }

        result.push('-');
        result
    }

    /// Get validation rules in JSON format
    pub fn validation_rules() -> &'static str {
        r#"{
  "rules": [
    {
      "id": "C1",
      "description": "Either field 50a or field 52a must be present",
      "condition": {
        "or": [
          {"exists": ["fields", "50"]},
          {"exists": ["fields", "52"]}
        ]
      }
    }
  ]
}"#
    }
}

impl crate::traits::SwiftMessageBody for MT910 {
    fn message_type() -> &'static str {
        "910"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT910::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT910::to_mt_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt910_parse() {
        let mt910_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
:50K:JOHN DOE
123 MAIN STREET
NEW YORK
-"#;
        let result = MT910::parse_from_block4(mt910_text);
        assert!(result.is_ok());
        let mt910 = result.unwrap();
        assert_eq!(mt910.field_20.reference, "20240719001");
        assert_eq!(mt910.field_21.reference, "REF20240719001");
    }

    #[test]
    fn test_mt910_validation_c1() {
        // Test without field 50 and 52 - should fail
        let mt910_text = r#":20:20240719001
:21:REF20240719001
:25:12345678901234567890
:32A:240719USD1000,00
-"#;
        let result = MT910::parse(mt910_text);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Either field 50"));
    }
}
