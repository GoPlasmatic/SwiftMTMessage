use crate::fields::Field52DrawerBank;
use crate::fields::*;
use chrono::Datelike;
use serde::{Deserialize, Serialize};

// MT110: Advice of Cheque(s)
// Sent by a drawer bank (or its agent) to the drawee bank to advise
// or confirm the issuance of one or multiple cheques.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110Cheque {
    // Cheque Number
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Date of Issue
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Amount field - can be variants A or B per SWIFT spec
    #[serde(flatten)]
    pub field_32: Field32AB,

    // Payer (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50: Option<Field50OrderingCustomerAFK>,

    // Drawer Bank (optional) - supports A, B, D variants per MT110 spec
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52DrawerBank>,

    // Payee
    #[serde(flatten)]
    pub field_59: Field59,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT110 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Sender's Correspondent (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53a: Option<Field53SenderCorrespondent>,

    // Receiver's Correspondent (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_54a: Option<Field54ReceiverCorrespondent>,

    // Sender to Receiver Information (optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    // Cheque Details (repeating sequence, max 10)
    #[serde(rename = "#", default)]
    pub cheques: Vec<MT110Cheque>,
}

impl MT110 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "110");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;

        // Parse optional fields before cheque details
        let field_53a = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        let field_54a =
            parser.parse_optional_variant_field::<Field54ReceiverCorrespondent>("54")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Parse cheque details - enable duplicates for repeating fields
        let mut cheques = Vec::new();
        parser = parser.with_duplicates(true);

        // Parse each cheque detail - they start with field 21
        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let field_30 = parser.parse_field::<Field30>("30")?;

            // Parse field 32 (amount) - only A or B per spec
            let field_32 = parser.parse_variant_field::<Field32AB>("32")?;

            // Parse optional fields
            let field_50 =
                parser.parse_optional_variant_field::<Field50OrderingCustomerAFK>("50")?;
            let field_52 = parser.parse_optional_variant_field::<Field52DrawerBank>("52")?;

            // Parse field 59 (payee)
            let field_59 = parser.parse_variant_field::<Field59>("59")?;

            cheques.push(MT110Cheque {
                field_21,
                field_30,
                field_32,
                field_50,
                field_52,
                field_59,
            });

            // Check max 10 repetitions (NVR C1)
            if cheques.len() > 10 {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: "MT110: Maximum 10 cheque details allowed (NVR C1)".to_string(),
                });
            }
        }

        // Validate we have at least one cheque detail
        if cheques.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT110: At least one cheque detail is required".to_string(),
            });
        }

        // NVR C2: Validate currency consistency across all cheque amounts
        let mut currency: Option<String> = None;
        for (idx, cheque) in cheques.iter().enumerate() {
            let cheque_currency = match &cheque.field_32 {
                Field32AB::A(amt) => Some(amt.currency.clone()),
                Field32AB::B(amt) => Some(amt.currency.clone()),
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
            field_53a,
            field_54a,
            field_72,
            cheques,
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
        if self.cheques.len() > 10 {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT110: Maximum 10 cheque details allowed (NVR C1)".to_string(),
            });
        }

        // At least one cheque detail is required
        if self.cheques.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT110: At least one cheque detail is required".to_string(),
            });
        }

        // NVR C2: Currency consistency
        let mut currency: Option<String> = None;
        for (idx, cheque) in self.cheques.iter().enumerate() {
            let cheque_currency = match &cheque.field_32 {
                Field32AB::A(amt) => Some(amt.currency.clone()),
                Field32AB::B(amt) => Some(amt.currency.clone()),
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

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut ordered_fields = Vec::new();

        // Add header fields
        ordered_fields.push(("20".to_string(), self.field_20.to_swift_value()));

        if let Some(ref field_53) = self.field_53a {
            match field_53 {
                Field53SenderCorrespondent::A(f) => {
                    ordered_fields.push(("53A".to_string(), f.to_swift_value()));
                }
                Field53SenderCorrespondent::B(f) => {
                    ordered_fields.push(("53B".to_string(), f.to_swift_value()));
                }
                Field53SenderCorrespondent::D(f) => {
                    ordered_fields.push(("53D".to_string(), f.to_swift_value()));
                }
            }
        }

        if let Some(ref field_54) = self.field_54a {
            match field_54 {
                Field54ReceiverCorrespondent::A(f) => {
                    ordered_fields.push(("54A".to_string(), f.to_swift_value()));
                }
                Field54ReceiverCorrespondent::B(f) => {
                    ordered_fields.push(("54B".to_string(), f.to_swift_value()));
                }
                Field54ReceiverCorrespondent::D(f) => {
                    ordered_fields.push(("54D".to_string(), f.to_swift_value()));
                }
            }
        }

        if let Some(ref field_72) = self.field_72 {
            ordered_fields.push(("72".to_string(), field_72.information.join("\n")));
        }

        // Add cheque details in sequence (important for proper parsing)
        for cheque in &self.cheques {
            // Each cheque's fields must be in sequence
            ordered_fields.push(("21".to_string(), cheque.field_21.to_swift_value()));

            ordered_fields.push((
                "30".to_string(),
                format!(
                    "{:02}{:02}{:02}",
                    cheque.field_30.execution_date.year() % 100,
                    cheque.field_30.execution_date.month(),
                    cheque.field_30.execution_date.day()
                ),
            ));

            // Amount field
            match &cheque.field_32 {
                Field32AB::A(f) => ordered_fields.push(("32A".to_string(), f.to_swift_value())),
                Field32AB::B(f) => ordered_fields.push(("32B".to_string(), f.to_swift_value())),
            }

            // Optional payer
            if let Some(ref field_50) = cheque.field_50 {
                match field_50 {
                    Field50OrderingCustomerAFK::A(f) => {
                        ordered_fields.push(("50A".to_string(), f.to_swift_value()));
                    }
                    Field50OrderingCustomerAFK::F(f) => {
                        ordered_fields.push(("50F".to_string(), f.to_swift_value()));
                    }
                    Field50OrderingCustomerAFK::K(f) => {
                        ordered_fields.push(("50K".to_string(), f.to_swift_value()));
                    }
                }
            }

            // Optional drawer bank (supports A, B, D variants)
            if let Some(ref field_52) = cheque.field_52 {
                match field_52 {
                    Field52DrawerBank::A(f) => {
                        ordered_fields.push(("52A".to_string(), f.to_swift_value()));
                    }
                    Field52DrawerBank::B(f) => {
                        ordered_fields.push(("52B".to_string(), f.to_swift_value()));
                    }
                    Field52DrawerBank::D(f) => {
                        ordered_fields.push(("52D".to_string(), f.to_swift_value()));
                    }
                }
            }

            // Payee
            match &cheque.field_59 {
                Field59::NoOption(f) => {
                    ordered_fields.push(("59".to_string(), f.to_swift_value()));
                }
                Field59::A(f) => {
                    ordered_fields.push(("59A".to_string(), f.to_swift_value()));
                }
                Field59::F(f) => {
                    ordered_fields.push(("59F".to_string(), f.to_swift_value()));
                }
            }
        }

        // Convert ordered_fields to MT string format
        let mut result = String::new();
        for (tag, value) in ordered_fields {
            result.push_str(&format!(":{tag}:{value}\r\n"));
        }

        // Remove trailing \r\n if present
        if result.ends_with("\r\n") {
            result.truncate(result.len() - 2);
        }

        result
    }
}
