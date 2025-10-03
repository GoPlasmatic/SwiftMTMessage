use crate::fields::{field34::Field34F, *};
use serde::{Deserialize, Serialize};

// MT920: Request Message
// Used to request specific account information or statement messages from another financial institution.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT920 {
    #[serde(rename = "20")]
    pub field_20: Field20,

    #[serde(rename = "#")]
    pub sequence: Vec<MT920Sequence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT920Sequence {
    #[serde(rename = "12")]
    pub field_12: Field12,

    #[serde(rename = "25A")]
    pub field_25: Field25A,

    #[serde(rename = "34F_1")]
    pub floor_limit_debit: Option<Field34F>,

    #[serde(rename = "34F_2")]
    pub floor_limit_credit: Option<Field34F>,
}

impl MT920 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "920");

        // Parse header field
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse repetitive sequences
        let mut sequence = Vec::new();

        // Enable duplicate field handling for repeated sequences
        parser = parser.with_duplicates(true);

        // Detect and parse each sequence (field 12 marks the start of a new sequence)
        while parser.detect_field("12") {
            let field_12 = parser.parse_field::<Field12>("12")?;
            let field_25 = parser.parse_field::<Field25A>("25A")?;
            let floor_limit_debit = parser.parse_optional_field::<Field34F>("34F_1")?;
            let floor_limit_credit = parser.parse_optional_field::<Field34F>("34F_2")?;

            // Apply max repetitions validation
            if sequence.len() >= 100 {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: "Maximum 100 repetitions allowed".to_string(),
                });
            }

            sequence.push(MT920Sequence {
                field_12,
                field_25,
                floor_limit_debit,
                floor_limit_credit,
            });
        }

        // Verify all content is consumed
        if !parser.is_complete() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "Unparsed content remaining in message: {}",
                    parser.remaining()
                ),
            });
        }

        Ok(Self { field_20, sequence })
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
        let message = Self::parse_from_block4(&block4)?;

        // Apply validation rules
        message.validate_network_rules()?;

        Ok(message)
    }

    /// Validate network rules specific to MT920
    pub fn validate_network_rules(&self) -> Result<(), crate::errors::ParseError> {
        // C1: At least one sequence must be present
        if self.sequence.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT920: At least one sequence must be present".to_string(),
            });
        }

        // C2: Maximum 100 sequences allowed
        if self.sequence.len() > 100 {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "MT920: Maximum 100 sequences allowed, found {}",
                    self.sequence.len()
                ),
            });
        }

        Ok(())
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        // Add header field
        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        // Add sequences
        for seq in &self.sequence {
            result.push_str(&seq.field_12.to_swift_string());
            result.push_str("\r\n");
            result.push_str(&seq.field_25.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref floor_limit) = seq.floor_limit_debit {
                result.push_str(&floor_limit.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref floor_limit) = seq.floor_limit_credit {
                result.push_str(&floor_limit.to_swift_string());
                result.push_str("\r\n");
            }
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
      "description": "At least one sequence must be present",
      "condition": {
        "min_sequences": 1
      }
    },
    {
      "id": "C2",
      "description": "Maximum 100 sequences allowed",
      "condition": {
        "max_sequences": 100
      }
    }
  ]
}"#
    }
}

impl crate::traits::SwiftMessageBody for MT920 {
    fn message_type() -> &'static str {
        "920"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        // Call the existing public method implementation
        MT920::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT920::to_mt_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt920_parse() {
        let mt920_text = r#":20:REQ123456
:12:100
:25:/GB12ABCD12345678901234
:12:200
:25:/US98EFGH98765432109876
-"#;
        let result = MT920::parse_from_block4(mt920_text);
        if let Err(ref e) = result {
            eprintln!("MT920 parse error: {:?}", e);
        }
        assert!(result.is_ok());
        let mt920 = result.unwrap();
        assert_eq!(mt920.field_20.reference, "REQ123456");
        assert_eq!(mt920.sequence.len(), 2);
        assert_eq!(mt920.sequence[0].field_12.type_code, "100");
        assert_eq!(mt920.sequence[1].field_12.type_code, "200");
    }

    #[test]
    fn test_mt920_validation() {
        // Test empty sequence - should fail
        let mt920_text = r#":20:REQ123456
-"#;
        let result = MT920::parse(mt920_text);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("At least one sequence")
        );
    }

    #[test]
    fn test_mt920_json_deserialization() {
        // Test JSON deserialization for MT920
        let json = r##"{
            "20": {
                "reference": "REQ123456"
            },
            "#": [
                {
                    "12": {
                        "type_code": "940"
                    },
                    "25": {
                        "account": "1234567890"
                    }
                }
            ]
        }"##;

        let result = serde_json::from_str::<MT920>(json);
        if let Err(ref e) = result {
            eprintln!("MT920 JSON deserialization error: {}", e);
        }
        assert!(result.is_ok(), "Failed to deserialize MT920 from JSON");
        let mt920 = result.unwrap();
        assert_eq!(mt920.field_20.reference, "REQ123456");
        assert_eq!(mt920.sequence.len(), 1);
        assert_eq!(mt920.sequence[0].field_12.type_code, "940");
        assert_eq!(mt920.sequence[0].field_25.account, "1234567890");
    }

    #[test]
    fn test_mt920_swift_message_json() {
        use crate::swift_message::SwiftMessage;

        // Test complete SwiftMessage<MT920> JSON deserialization
        let json = r##"{
            "basic_header": {
                "application_id": "F",
                "service_id": "01",
                "sender_bic": "DEUTDEFF",
                "logical_terminal": "DEUTDEFFXXXX",
                "session_number": "0001",
                "sequence_number": "000123"
            },
            "application_header": {
                "direction": "I",
                "message_type": "920",
                "receiver_bic": "DEUTDEFF",
                "destination_address": "DEUTDEFFXXXX",
                "priority": "N"
            },
            "message_type": "920",
            "fields": {
                "20": {
                    "reference": "REQ123456"
                },
                "#": [
                    {
                        "12": {
                            "type_code": "940"
                        },
                        "25": {
                            "account": "1234567890"
                        }
                    }
                ]
            }
        }"##;

        let result = serde_json::from_str::<SwiftMessage<MT920>>(json);
        if let Err(ref e) = result {
            eprintln!("SwiftMessage<MT920> JSON deserialization error: {}", e);
        }
        assert!(
            result.is_ok(),
            "Failed to deserialize SwiftMessage<MT920> from JSON"
        );
        let swift_msg = result.unwrap();
        assert_eq!(swift_msg.message_type, "920");
        assert_eq!(swift_msg.fields.field_20.reference, "REQ123456");
        assert_eq!(swift_msg.fields.sequence.len(), 1);
    }
}
