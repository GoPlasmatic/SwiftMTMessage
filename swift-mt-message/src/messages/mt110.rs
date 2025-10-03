use crate::fields::Field52DrawerBank;
use crate::fields::*;
use crate::parsing_utils::*;
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
        let mut result = String::new();

        // Add header fields
        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_53a);
        append_optional_field(&mut result, &self.field_54a);
        append_optional_field(&mut result, &self.field_72);

        // Add cheque details in sequence
        for cheque in &self.cheques {
            append_field(&mut result, &cheque.field_21);
            append_field(&mut result, &cheque.field_30);
            append_field(&mut result, &cheque.field_32);
            append_optional_field(&mut result, &cheque.field_50);
            append_optional_field(&mut result, &cheque.field_52);
            append_field(&mut result, &cheque.field_59);
        }

        finalize_mt_string(result, false)
    }
}
