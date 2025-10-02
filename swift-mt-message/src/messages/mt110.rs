use crate::fields::*;
use chrono::Datelike;
use serde::{Deserialize, Serialize};

// MT110: Advice of Cheque(s)
// Sent by a drawer bank (or its agent) to the drawee bank to advise
// or confirm the issuance of one or multiple cheques.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110ChequeDetails {
    // Cheque Number
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Date of Issue
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Amount
    #[serde(rename = "32A")]
    pub field_32a: Option<Field32A>,

    #[serde(rename = "32B")]
    pub field_32b: Option<Field32B>,

    // Payer (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50: Option<Field50PayerAFK>,

    // Drawer Bank (optional)  - MT110 spec says A, B, D but our Field52 only supports A, D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52DrawerBank>,

    // Payee (mandatory - only F option allowed for MT110)
    #[serde(rename = "59F")]
    pub field_59f: Field59F,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Sender's Correspondent (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53: Option<Field53SenderCorrespondent>,

    // Receiver's Correspondent (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    // Sender to Receiver Information (optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    // Cheque Details (repeating sequence, max 10)
    #[serde(rename = "#")]
    pub cheque_details: Vec<MT110ChequeDetails>,
}

impl MT110 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "110");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional fields before cheque details
        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        let field_54 = parser.parse_optional_variant_field::<Field54ReceiverCorrespondent>("54")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Parse cheque details - enable duplicates for repeating fields
        let mut cheque_details = Vec::new();
        parser = parser.with_duplicates(true);

        // Parse each cheque detail - they start with field 21
        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let field_30 = parser.parse_field::<Field30>("30")?;

            // Amount can be 32A or 32B
            let field_32a = parser.parse_optional_field::<Field32A>("32A")?;
            let field_32b = if field_32a.is_none() {
                Some(parser.parse_field::<Field32B>("32B")?)
            } else {
                None
            };

            // Validate that we have at least one amount field
            if field_32a.is_none() && field_32b.is_none() {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: "MT110: Either field 32A or 32B is required for amount".to_string(),
                });
            }

            // Parse optional fields
            let field_50 = parser.parse_optional_variant_field::<Field50PayerAFK>("50")?;
            let field_52 = parser.parse_optional_variant_field::<Field52DrawerBank>("52")?;

            // Field 59F is mandatory for payee
            let field_59f = parser.parse_field::<Field59F>("59F")?;

            cheque_details.push(MT110ChequeDetails {
                field_21,
                field_30,
                field_32a,
                field_32b,
                field_50,
                field_52,
                field_59f,
            });

            // Check max 10 repetitions (NVR C1)
            if cheque_details.len() > 10 {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: "MT110: Maximum 10 cheque details allowed (NVR C1)".to_string(),
                });
            }
        }

        // Validate we have at least one cheque detail
        if cheque_details.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT110: At least one cheque detail is required".to_string(),
            });
        }

        // NVR C2: Validate currency consistency across all cheque amounts
        let mut currency: Option<String> = None;
        for (idx, cheque) in cheque_details.iter().enumerate() {
            let cheque_currency = if let Some(ref amt) = cheque.field_32a {
                Some(amt.currency.clone())
            } else {
                cheque.field_32b.as_ref().map(|amt| amt.currency.clone())
            };

            if let Some(cheque_curr) = cheque_currency {
                if let Some(ref expected_currency) = currency {
                    if cheque_curr != *expected_currency {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: format!(
                                "MT110: Currency mismatch in cheque {}: expected {}, found {} (NVR C2)",
                                idx + 1,
                                expected_currency,
                                cheque_curr
                            ),
                        });
                    }
                } else {
                    currency = Some(cheque_curr);
                }
            }
        }

        Ok(MT110 {
            field_20,
            field_53,
            field_54,
            field_72,
            cheque_details,
        })
    }

    /// Static validation rules for MT110
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "C1", "description": "Maximum 10 cheque details allowed", "condition": true},
            {"id": "C2", "description": "Currency must be consistent across all cheques", "condition": true}
        ]}"#
    }

    /// Validate the message instance according to MT110 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // NVR C1: Max 10 repetitions
        if self.cheque_details.len() > 10 {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT110: Maximum 10 cheque details allowed (NVR C1)".to_string(),
            });
        }

        // At least one cheque detail is required
        if self.cheque_details.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT110: At least one cheque detail is required".to_string(),
            });
        }

        // NVR C2: Currency consistency
        let mut currency: Option<String> = None;
        for (idx, cheque) in self.cheque_details.iter().enumerate() {
            let cheque_currency = if let Some(ref amt) = cheque.field_32a {
                Some(amt.currency.clone())
            } else if let Some(ref amt) = cheque.field_32b {
                Some(amt.currency.clone())
            } else {
                continue; // Should not happen as we validate presence during parsing
            };

            if let Some(cheque_curr) = cheque_currency {
                if let Some(ref expected_currency) = currency {
                    if cheque_curr != *expected_currency {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: format!(
                                "MT110: Currency mismatch in cheque {}: expected {}, found {} (NVR C2)",
                                idx + 1,
                                expected_currency,
                                cheque_curr
                            ),
                        });
                    }
                } else {
                    currency = Some(cheque_curr);
                }
            }
        }

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT110
impl crate::traits::SwiftMessageBody for MT110 {
    fn message_type() -> &'static str {
        "110"
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
            block4.push_str(&format!(":{}:{}\n", tag, value));
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

        // Add header fields
        fields.insert("20".to_string(), vec![self.field_20.to_swift_value()]);

        if let Some(ref field_53) = self.field_53 {
            match field_53 {
                Field53SenderCorrespondent::A(f) => {
                    fields.insert("53A".to_string(), vec![f.to_swift_value()]);
                }
                Field53SenderCorrespondent::B(f) => {
                    fields.insert("53B".to_string(), vec![f.to_swift_value()]);
                }
                Field53SenderCorrespondent::D(f) => {
                    fields.insert("53D".to_string(), vec![f.to_swift_value()]);
                }
            }
        }

        if let Some(ref field_54) = self.field_54 {
            match field_54 {
                Field54ReceiverCorrespondent::A(f) => {
                    fields.insert("54A".to_string(), vec![f.to_swift_value()]);
                }
                Field54ReceiverCorrespondent::B(f) => {
                    fields.insert("54B".to_string(), vec![f.to_swift_value()]);
                }
                Field54ReceiverCorrespondent::D(f) => {
                    fields.insert("54D".to_string(), vec![f.to_swift_value()]);
                }
            }
        }

        if let Some(ref field_72) = self.field_72 {
            fields.insert("72".to_string(), vec![field_72.information.join("\n")]);
        }

        // Add cheque details
        for cheque in &self.cheque_details {
            // Field 21 (can repeat)
            fields
                .entry("21".to_string())
                .or_insert_with(Vec::new)
                .push(cheque.field_21.to_swift_value());

            // Field 30 (can repeat)
            fields
                .entry("30".to_string())
                .or_insert_with(Vec::new)
                .push(format!(
                    "{:02}{:02}{:02}",
                    cheque.field_30.execution_date.year() % 100,
                    cheque.field_30.execution_date.month(),
                    cheque.field_30.execution_date.day()
                ));

            // Amount field (32A or 32B)
            if let Some(ref field_32a) = cheque.field_32a {
                fields
                    .entry("32A".to_string())
                    .or_insert_with(Vec::new)
                    .push(field_32a.to_swift_value());
            }

            if let Some(ref field_32b) = cheque.field_32b {
                fields
                    .entry("32B".to_string())
                    .or_insert_with(Vec::new)
                    .push(field_32b.to_swift_value());
            }

            // Optional payer field
            if let Some(ref field_50) = cheque.field_50 {
                match field_50 {
                    Field50PayerAFK::A(f) => {
                        fields
                            .entry("50A".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                    Field50PayerAFK::F(f) => {
                        fields
                            .entry("50F".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                    Field50PayerAFK::K(f) => {
                        fields
                            .entry("50K".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                }
            }

            // Optional drawer bank (only A and D variants are supported in our implementation)
            if let Some(ref field_52) = cheque.field_52 {
                match field_52 {
                    Field52DrawerBank::A(f) => {
                        fields
                            .entry("52A".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                    Field52DrawerBank::D(f) => {
                        fields
                            .entry("52D".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                }
            }

            // Payee (mandatory, always 59F)
            fields
                .entry("59F".to_string())
                .or_insert_with(Vec::new)
                .push(cheque.field_59f.to_swift_value());
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "30", "32", "59F"] // Note: 32 can be 32A or 32B
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["53", "54", "72", "50", "52"]
    }
}

// Type aliases for field variants used in MT110
pub type Field50PayerAFK = Field50OrderingCustomerAFK; // Can be A, F, or K
pub type Field52DrawerBank = Field52OrderingInstitution; // Can be A, B, or D
