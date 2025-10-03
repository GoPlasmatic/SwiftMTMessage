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

        if let Some(ref field_53) = self.field_53a {
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

        if let Some(ref field_54) = self.field_54a {
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
        for cheque in &self.cheques {
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

            // Amount field (32A, 32B, 32C, or 32D)
            match &cheque.field_32 {
                Field32AB::A(f) => {
                    fields
                        .entry("32A".to_string())
                        .or_insert_with(Vec::new)
                        .push(f.to_swift_value());
                }
                Field32AB::B(f) => {
                    fields
                        .entry("32B".to_string())
                        .or_insert_with(Vec::new)
                        .push(f.to_swift_value());
                }
            }

            // Optional payer field
            if let Some(ref field_50) = cheque.field_50 {
                match field_50 {
                    Field50OrderingCustomerAFK::A(f) => {
                        fields
                            .entry("50A".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                    Field50OrderingCustomerAFK::F(f) => {
                        fields
                            .entry("50F".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                    Field50OrderingCustomerAFK::K(f) => {
                        fields
                            .entry("50K".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                }
            }

            // Optional drawer bank (supports A, B, D variants)
            if let Some(ref field_52) = cheque.field_52 {
                match field_52 {
                    Field52DrawerBank::A(f) => {
                        fields
                            .entry("52A".to_string())
                            .or_insert_with(Vec::new)
                            .push(f.to_swift_value());
                    }
                    Field52DrawerBank::B(f) => {
                        fields
                            .entry("52B".to_string())
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

            // Payee (mandatory, can be various variants)
            match &cheque.field_59 {
                Field59::NoOption(f) => {
                    fields
                        .entry("59".to_string())
                        .or_insert_with(Vec::new)
                        .push(f.to_swift_value());
                }
                Field59::A(f) => {
                    fields
                        .entry("59A".to_string())
                        .or_insert_with(Vec::new)
                        .push(f.to_swift_value());
                }
                Field59::F(f) => {
                    fields
                        .entry("59F".to_string())
                        .or_insert_with(Vec::new)
                        .push(f.to_swift_value());
                }
            }
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "30", "32", "59F"] // Note: 32 can be 32A or 32B
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["53", "54", "72", "50", "52"]
    }

    fn to_ordered_fields(&self) -> Vec<(String, String)> {
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

        ordered_fields
    }
}
