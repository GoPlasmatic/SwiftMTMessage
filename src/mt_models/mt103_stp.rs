//! MT103-STP: Single Customer Credit Transfer (Straight Through Processing)
//!
//! This module contains the MT103-STP message structure with enhanced validation
//! rules for straight through processing compliance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::{ParseError, Result};
use crate::field_parser::{SwiftFieldContainer, SwiftMessage};
use crate::json::ToJson;
use crate::mt_models::fields::institutions::{
    Field52, Field53, Field54, Field55, Field56, Field57,
};
use crate::mt_models::fields::{
    Field13C, Field20, Field23B, Field23E, Field26T, Field32A, Field33B, Field36, Field50, Field59,
    Field70, Field71A, Field71F, Field71G, Field72, Field77B,
};
use crate::validation::{ValidationReport, validate_mt_message_with_rules};

/// MT103-STP: Single Customer Credit Transfer (Straight Through Processing)
///
/// MT103-STP has stricter validation rules than regular MT103, designed for
/// automatic processing without manual intervention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT103STP {
    // Mandatory fields
    pub field_20: Field20,   // Sender's Reference
    pub field_23b: Field23B, // Bank Operation Code
    pub field_32a: Field32A, // Value Date/Currency/Amount
    pub field_50: Field50,   // Ordering Customer (A/F/K options)
    pub field_59: Field59,   // Beneficiary Customer (A/F/no letter options)
    pub field_71a: Field71A, // Details of Charges

    // Optional fields
    pub field_13c: Option<Field13C>, // Time Indication
    pub field_23e: Option<Field23E>, // Instruction Code (conditional C3)
    pub field_26t: Option<Field26T>, // Transaction Type Code
    pub field_33b: Option<Field33B>, // Currency/Instructed Amount (conditional C1, C2, C8)
    pub field_36: Option<Field36>,   // Exchange Rate (conditional C1)
    pub field_52a: Option<Field52>,  // Ordering Institution
    pub field_53a: Option<Field53>,  // Sender's Correspondent (conditional C4)
    pub field_54a: Option<Field54>,  // Receiver's Correspondent (conditional C4)
    pub field_55a: Option<Field55>,  // Third Reimbursement Institution
    pub field_56a: Option<Field56>,  // Intermediary Institution (conditional C5, C6)
    pub field_57a: Option<Field57>,  // Account With Institution (conditional C5, C10)
    pub field_70: Option<Field70>,   // Remittance Information
    pub field_71f: Option<Field71F>, // Sender's Charges (conditional C7, C8)
    pub field_71g: Option<Field71G>, // Receiver's Charges (conditional C7, C8)
    pub field_72: Option<Field72>,   // Sender to Receiver Information
    pub field_77b: Option<Field77B>, // Regulatory Reporting
}

/// STP Conditional Rule Violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STPRuleViolation {
    pub rule: String,
    pub description: String,
    pub affected_fields: Vec<String>,
}

/// STP Validation Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STPValidationReport {
    pub is_stp_compliant: bool,
    pub rule_violations: Vec<STPRuleViolation>,
    pub warnings: Vec<String>,
}

impl MT103STP {
    /// Create MT103-STP from generic SwiftMessage
    pub fn from_swift_message(message: SwiftMessage) -> Result<Self> {
        Self::from_swift_message_preserving_headers(message).map(|(mt103_stp, _headers)| mt103_stp)
    }

    /// Create MT103-STP from generic SwiftMessage, preserving headers for later use
    pub fn from_swift_message_preserving_headers(
        message: SwiftMessage,
    ) -> Result<(
        Self,
        (
            Option<crate::tokenizer::BasicHeader>,
            Option<crate::tokenizer::ApplicationHeader>,
            Option<crate::tokenizer::UserHeader>,
            Option<crate::tokenizer::Trailer>,
        ),
    )> {
        if message.message_type != "103" {
            return Err(ParseError::WrongMessageType {
                expected: "103".to_string(),
                actual: message.message_type,
            });
        }

        // Extract mandatory fields
        let field_20 = Self::extract_field_20(&message)?;
        let field_23b = Self::extract_field_23b(&message)?;
        let field_32a = Self::extract_field_32a(&message)?;
        let field_50 = Self::extract_field_50(&message)?;
        let field_59 = Self::extract_field_59(&message)?;
        let field_71a = Self::extract_field_71a(&message)?;

        // Extract optional fields
        let field_13c = Self::extract_optional_field_13c(&message);
        let field_23e = Self::extract_optional_field_23e(&message);
        let field_26t = Self::extract_optional_field_26t(&message);
        let field_33b = Self::extract_optional_field_33b(&message);
        let field_36 = Self::extract_optional_field_36(&message);
        let field_52a = Self::extract_optional_field_52a(&message);
        let field_53a = Self::extract_optional_field_53a(&message);
        let field_54a = Self::extract_optional_field_54a(&message);
        let field_55a = Self::extract_optional_field_55a(&message);
        let field_56a = Self::extract_optional_field_56a(&message);
        let field_57a = Self::extract_optional_field_57a(&message);
        let field_70 = Self::extract_optional_field_70(&message);
        let field_71f = Self::extract_optional_field_71f(&message);
        let field_71g = Self::extract_optional_field_71g(&message);
        let field_72 = Self::extract_optional_field_72(&message);
        let field_77b = Self::extract_optional_field_77b(&message);

        // Preserve headers
        let headers = (
            message.basic_header.clone(),
            message.application_header.clone(),
            message.user_header.clone(),
            message.trailer_block.clone(),
        );

        let mt103_stp = MT103STP {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c,
            field_23e,
            field_26t,
            field_33b,
            field_36,
            field_52a,
            field_53a,
            field_54a,
            field_55a,
            field_56a,
            field_57a,
            field_70,
            field_71f,
            field_71g,
            field_72,
            field_77b,
        };

        // Validate STP rules after construction
        let validation_report = mt103_stp.validate_stp_rules()?;
        if !validation_report.is_stp_compliant {
            return Err(ParseError::ValidationError {
                message: format!(
                    "MT103-STP validation failed: {} rule violations",
                    validation_report.rule_violations.len()
                ),
            });
        }

        Ok((mt103_stp, headers))
    }

    /// Convert back to generic SwiftMessage
    pub fn to_swift_message(&self) -> SwiftMessage {
        self.to_swift_message_with_headers(None, None, None, None)
    }

    /// Convert back to generic SwiftMessage with headers
    pub fn to_swift_message_with_headers(
        &self,
        basic_header: Option<crate::tokenizer::BasicHeader>,
        application_header: Option<crate::tokenizer::ApplicationHeader>,
        user_header: Option<crate::tokenizer::UserHeader>,
        trailer_block: Option<crate::tokenizer::Trailer>,
    ) -> SwiftMessage {
        let mut fields = HashMap::new();
        let mut field_order = Vec::new();

        // Add mandatory fields in standard order
        self.add_field(
            &mut fields,
            &mut field_order,
            "20",
            SwiftFieldContainer::Field20(self.field_20.clone()),
        );
        self.add_field(
            &mut fields,
            &mut field_order,
            "23B",
            SwiftFieldContainer::Field23B(self.field_23b.clone()),
        );
        self.add_field(
            &mut fields,
            &mut field_order,
            "32A",
            SwiftFieldContainer::Field32A(self.field_32a.clone()),
        );
        self.add_field(
            &mut fields,
            &mut field_order,
            "50",
            SwiftFieldContainer::Field50(self.field_50.clone()),
        );
        self.add_field(
            &mut fields,
            &mut field_order,
            "59",
            SwiftFieldContainer::Field59(self.field_59.clone()),
        );
        self.add_field(
            &mut fields,
            &mut field_order,
            "71A",
            SwiftFieldContainer::Field71A(self.field_71a.clone()),
        );

        // Add optional fields if present
        if let Some(field) = &self.field_13c {
            self.add_field(
                &mut fields,
                &mut field_order,
                "13C",
                SwiftFieldContainer::Field13C(field.clone()),
            );
        }
        if let Some(field) = &self.field_23e {
            self.add_field(
                &mut fields,
                &mut field_order,
                "23E",
                SwiftFieldContainer::Field23E(field.clone()),
            );
        }
        if let Some(field) = &self.field_26t {
            self.add_field(
                &mut fields,
                &mut field_order,
                "26T",
                SwiftFieldContainer::Field26T(field.clone()),
            );
        }
        if let Some(field) = &self.field_33b {
            self.add_field(
                &mut fields,
                &mut field_order,
                "33B",
                SwiftFieldContainer::Field33B(field.clone()),
            );
        }
        if let Some(field) = &self.field_36 {
            self.add_field(
                &mut fields,
                &mut field_order,
                "36",
                SwiftFieldContainer::Field36(field.clone()),
            );
        }
        if let Some(field) = &self.field_52a {
            self.add_field(
                &mut fields,
                &mut field_order,
                "52A",
                SwiftFieldContainer::Field52(field.clone()),
            );
        }
        if let Some(field) = &self.field_53a {
            self.add_field(
                &mut fields,
                &mut field_order,
                "53A",
                SwiftFieldContainer::Field53(field.clone()),
            );
        }
        if let Some(field) = &self.field_54a {
            self.add_field(
                &mut fields,
                &mut field_order,
                "54A",
                SwiftFieldContainer::Field54(field.clone()),
            );
        }
        if let Some(field) = &self.field_55a {
            self.add_field(
                &mut fields,
                &mut field_order,
                "55A",
                SwiftFieldContainer::Field55(field.clone()),
            );
        }
        if let Some(field) = &self.field_56a {
            self.add_field(
                &mut fields,
                &mut field_order,
                "56A",
                SwiftFieldContainer::Field56(field.clone()),
            );
        }
        if let Some(field) = &self.field_57a {
            self.add_field(
                &mut fields,
                &mut field_order,
                "57A",
                SwiftFieldContainer::Field57(field.clone()),
            );
        }
        if let Some(field) = &self.field_70 {
            self.add_field(
                &mut fields,
                &mut field_order,
                "70",
                SwiftFieldContainer::Field70(field.clone()),
            );
        }
        if let Some(field) = &self.field_71f {
            self.add_field(
                &mut fields,
                &mut field_order,
                "71F",
                SwiftFieldContainer::Field71F(field.clone()),
            );
        }
        if let Some(field) = &self.field_71g {
            self.add_field(
                &mut fields,
                &mut field_order,
                "71G",
                SwiftFieldContainer::Field71G(field.clone()),
            );
        }
        if let Some(field) = &self.field_72 {
            self.add_field(
                &mut fields,
                &mut field_order,
                "72",
                SwiftFieldContainer::Field72(field.clone()),
            );
        }
        if let Some(field) = &self.field_77b {
            self.add_field(
                &mut fields,
                &mut field_order,
                "77B",
                SwiftFieldContainer::Field77B(field.clone()),
            );
        }

        SwiftMessage {
            message_type: "103".to_string(),
            basic_header,
            application_header,
            user_header,
            trailer_block,
            blocks: crate::tokenizer::SwiftMessageBlocks::default(),
            fields,
            field_order,
        }
    }

    /// Helper method to add fields in order
    fn add_field(
        &self,
        fields: &mut HashMap<String, SwiftFieldContainer>,
        field_order: &mut Vec<String>,
        tag: &str,
        container: SwiftFieldContainer,
    ) {
        fields.insert(tag.to_string(), container);
        field_order.push(tag.to_string());
    }

    /// Validate MT103-STP specific conditional rules
    pub fn validate_stp_rules(&self) -> Result<STPValidationReport> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // C1: Currency and Exchange Rate Validation
        if let Some(field_33b) = &self.field_33b {
            if field_33b.currency != self.field_32a.currency {
                if self.field_36.is_none() {
                    violations.push(STPRuleViolation {
                        rule: "C1".to_string(),
                        description: "If 33B currency differs from 32A, field 36 must be present"
                            .to_string(),
                        affected_fields: vec![
                            "33B".to_string(),
                            "32A".to_string(),
                            "36".to_string(),
                        ],
                    });
                }
            } else if self.field_36.is_some() {
                violations.push(STPRuleViolation {
                    rule: "C1".to_string(),
                    description: "If 33B currency matches 32A, field 36 must not be present"
                        .to_string(),
                    affected_fields: vec!["33B".to_string(), "32A".to_string(), "36".to_string()],
                });
            }
        }

        // C3: Bank Operation Code and Instruction Code Validation
        match self.field_23b.bank_operation_code.as_str() {
            "SPRI" => {
                if let Some(field_23e) = &self.field_23e {
                    if !matches!(field_23e.instruction_code.as_str(), "SDVA" | "INTC") {
                        violations.push(STPRuleViolation {
                            rule: "C3".to_string(),
                            description: "If 23B = SPRI, 23E can only contain SDVA or INTC"
                                .to_string(),
                            affected_fields: vec!["23B".to_string(), "23E".to_string()],
                        });
                    }
                }
            }
            "SSTD" | "SPAY" => {
                if self.field_23e.is_some() {
                    violations.push(STPRuleViolation {
                        rule: "C3".to_string(),
                        description: "If 23B = SSTD or SPAY, 23E must not be present".to_string(),
                        affected_fields: vec!["23B".to_string(), "23E".to_string()],
                    });
                }
            }
            _ => {}
        }

        // C4: Correspondent Bank Chain Validation
        if self.field_55a.is_some() && (self.field_53a.is_none() || self.field_54a.is_none()) {
            violations.push(STPRuleViolation {
                rule: "C4".to_string(),
                description: "If 55A is present, both 53A and 54A become mandatory".to_string(),
                affected_fields: vec!["55A".to_string(), "53A".to_string(), "54A".to_string()],
            });
        }

        // C5: Intermediary and Account With Institution Validation
        if self.field_56a.is_some() && self.field_57a.is_none() {
            violations.push(STPRuleViolation {
                rule: "C5".to_string(),
                description: "If 56A is present, 57A becomes mandatory".to_string(),
                affected_fields: vec!["56A".to_string(), "57A".to_string()],
            });
        }

        // C6: Bank Operation Code and Intermediary Institution Validation
        if self.field_23b.bank_operation_code == "SPRI" && self.field_56a.is_some() {
            violations.push(STPRuleViolation {
                rule: "C6".to_string(),
                description: "If 23B = SPRI, 56A is not allowed".to_string(),
                affected_fields: vec!["23B".to_string(), "56A".to_string()],
            });
        }

        // C7: Charges Validation based on 71A
        match self.field_71a.details_of_charges.as_str() {
            "OUR" => {
                if self.field_71f.is_some() {
                    violations.push(STPRuleViolation {
                        rule: "C7".to_string(),
                        description: "If 71A = OUR, 71F is not allowed".to_string(),
                        affected_fields: vec!["71A".to_string(), "71F".to_string()],
                    });
                }
                // 71G is optional for OUR
            }
            "SHA" => {
                if self.field_71g.is_some() {
                    violations.push(STPRuleViolation {
                        rule: "C7".to_string(),
                        description: "If 71A = SHA, 71G is not allowed".to_string(),
                        affected_fields: vec!["71A".to_string(), "71G".to_string()],
                    });
                }
                // 71F is optional for SHA
            }
            "BEN" => {
                if self.field_71f.is_none() {
                    violations.push(STPRuleViolation {
                        rule: "C7".to_string(),
                        description: "If 71A = BEN, at least one 71F is mandatory".to_string(),
                        affected_fields: vec!["71A".to_string(), "71F".to_string()],
                    });
                }
                if self.field_71g.is_some() {
                    violations.push(STPRuleViolation {
                        rule: "C7".to_string(),
                        description: "If 71A = BEN, 71G is not allowed".to_string(),
                        affected_fields: vec!["71A".to_string(), "71G".to_string()],
                    });
                }
            }
            _ => {}
        }

        // C8: Charges and Currency Amount Validation
        if (self.field_71f.is_some() || self.field_71g.is_some()) && self.field_33b.is_none() {
            violations.push(STPRuleViolation {
                rule: "C8".to_string(),
                description: "If 71F or 71G present, 33B becomes mandatory".to_string(),
                affected_fields: vec!["71F".to_string(), "71G".to_string(), "33B".to_string()],
            });
        }

        // C9: Receiver's Charges Currency Validation
        if let Some(field_71g) = &self.field_71g {
            if field_71g.currency != self.field_32a.currency {
                violations.push(STPRuleViolation {
                    rule: "C9".to_string(),
                    description: "Currency code in 71G must match 32A".to_string(),
                    affected_fields: vec!["71G".to_string(), "32A".to_string()],
                });
            }
        }

        // C10: IBAN Validation (simplified check)
        // Note: Full IBAN validation would require country-specific logic
        if self.field_57a.is_none() {
            // For EU/EEA countries, IBAN would be mandatory in 59a
            warnings.push(
                "For EU/EEA countries, IBAN may be required in 59a when 57A is absent".to_string(),
            );
        }

        Ok(STPValidationReport {
            is_stp_compliant: violations.is_empty(),
            rule_violations: violations,
            warnings,
        })
    }

    /// Validate enhanced STP business rules using external rules file
    pub fn validate_business_rules(&self) -> Result<ValidationReport> {
        // Convert MT103-STP to JSON for validation
        let message_json = self.to_json().map_err(|e| ParseError::ValidationError {
            message: format!("Failed to convert MT103-STP to JSON: {}", e),
        })?;

        // Use the validation utility to apply JSONLogic rules
        validate_mt_message_with_rules(message_json, "config/mt103_stp_validation_rules.json")
    }

    /// Check if message is valid for straight-through processing
    pub fn is_stp_compliant(&self) -> bool {
        if let Ok(report) = self.validate_stp_rules() {
            report.is_stp_compliant
        } else {
            false
        }
    }

    /// Get list of all STP rule violations
    pub fn get_stp_violations(&self) -> Vec<STPRuleViolation> {
        if let Ok(report) = self.validate_stp_rules() {
            report.rule_violations
        } else {
            vec![]
        }
    }

    // Field extraction methods (similar to MT103)
    fn extract_field_20(message: &SwiftMessage) -> Result<Field20> {
        match message.get_field("20") {
            Some(SwiftFieldContainer::Field20(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type("20", "103-STP")),
        }
    }

    fn extract_field_23b(message: &SwiftMessage) -> Result<Field23B> {
        match message.get_field("23B") {
            Some(SwiftFieldContainer::Field23B(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type(
                "23B", "103-STP",
            )),
        }
    }

    fn extract_field_32a(message: &SwiftMessage) -> Result<Field32A> {
        match message.get_field("32A") {
            Some(SwiftFieldContainer::Field32A(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type(
                "32A", "103-STP",
            )),
        }
    }

    fn extract_field_50(message: &SwiftMessage) -> Result<Field50> {
        // Try different Field50 variants
        if let Some(SwiftFieldContainer::Field50(field)) = message.get_field("50K") {
            return Ok(field.clone());
        }
        if let Some(SwiftFieldContainer::Field50(field)) = message.get_field("50A") {
            return Ok(field.clone());
        }
        if let Some(SwiftFieldContainer::Field50(field)) = message.get_field("50F") {
            return Ok(field.clone());
        }

        Err(ParseError::missing_required_field_for_type("50", "103-STP"))
    }

    fn extract_field_59(message: &SwiftMessage) -> Result<Field59> {
        // Try different Field59 variants
        if let Some(SwiftFieldContainer::Field59(field)) = message.get_field("59A") {
            return Ok(field.clone());
        }
        if let Some(SwiftFieldContainer::Field59(field)) = message.get_field("59F") {
            return Ok(field.clone());
        }
        if let Some(SwiftFieldContainer::Field59(field)) = message.get_field("59") {
            return Ok(field.clone());
        }

        Err(ParseError::missing_required_field_for_type("59", "103-STP"))
    }

    fn extract_field_71a(message: &SwiftMessage) -> Result<Field71A> {
        match message.get_field("71A") {
            Some(SwiftFieldContainer::Field71A(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type(
                "71A", "103-STP",
            )),
        }
    }

    // Optional field extraction methods
    fn extract_optional_field_13c(message: &SwiftMessage) -> Option<Field13C> {
        match message.get_field("13C") {
            Some(SwiftFieldContainer::Field13C(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_23e(message: &SwiftMessage) -> Option<Field23E> {
        match message.get_field("23E") {
            Some(SwiftFieldContainer::Field23E(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_26t(message: &SwiftMessage) -> Option<Field26T> {
        match message.get_field("26T") {
            Some(SwiftFieldContainer::Field26T(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_33b(message: &SwiftMessage) -> Option<Field33B> {
        match message.get_field("33B") {
            Some(SwiftFieldContainer::Field33B(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_36(message: &SwiftMessage) -> Option<Field36> {
        match message.get_field("36") {
            Some(SwiftFieldContainer::Field36(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_52a(message: &SwiftMessage) -> Option<Field52> {
        match message.get_field("52A") {
            Some(SwiftFieldContainer::Field52(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_53a(message: &SwiftMessage) -> Option<Field53> {
        // Try different variants
        if let Some(SwiftFieldContainer::Field53(field)) = message.get_field("53A") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field53(field)) = message.get_field("53B") {
            return Some(field.clone());
        }
        None
    }

    fn extract_optional_field_54a(message: &SwiftMessage) -> Option<Field54> {
        match message.get_field("54A") {
            Some(SwiftFieldContainer::Field54(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_55a(message: &SwiftMessage) -> Option<Field55> {
        match message.get_field("55A") {
            Some(SwiftFieldContainer::Field55(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_56a(message: &SwiftMessage) -> Option<Field56> {
        match message.get_field("56A") {
            Some(SwiftFieldContainer::Field56(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_57a(message: &SwiftMessage) -> Option<Field57> {
        match message.get_field("57A") {
            Some(SwiftFieldContainer::Field57(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_70(message: &SwiftMessage) -> Option<Field70> {
        match message.get_field("70") {
            Some(SwiftFieldContainer::Field70(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_71f(message: &SwiftMessage) -> Option<Field71F> {
        match message.get_field("71F") {
            Some(SwiftFieldContainer::Field71F(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_71g(message: &SwiftMessage) -> Option<Field71G> {
        match message.get_field("71G") {
            Some(SwiftFieldContainer::Field71G(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_72(message: &SwiftMessage) -> Option<Field72> {
        match message.get_field("72") {
            Some(SwiftFieldContainer::Field72(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_77b(message: &SwiftMessage) -> Option<Field77B> {
        match message.get_field("77B") {
            Some(SwiftFieldContainer::Field77B(field)) => Some(field.clone()),
            _ => None,
        }
    }
}

impl ToJson for MT103STP {
    fn to_json(&self) -> Result<serde_json::Value> {
        // Convert to SwiftMessage first, then to JSON
        let swift_message = self.to_swift_message();
        swift_message.to_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::SwiftMessage;

    #[test]
    fn test_mt103_stp_creation() {
        let message_text = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:STP123456789
:23B:CRED
:32A:241231USD1000000,00
:50K:/1234567890
ORDERING CUSTOMER
:59A:DEUTDEFF
:71A:OUR
-}"#;

        let swift_message = SwiftMessage::parse(message_text).expect("Should parse");
        let mt103_stp = MT103STP::from_swift_message(swift_message);

        assert!(
            mt103_stp.is_ok(),
            "MT103-STP should be created successfully"
        );

        let stp = mt103_stp.unwrap();
        assert_eq!(stp.field_20.transaction_reference, "STP123456789");
        assert_eq!(stp.field_23b.bank_operation_code, "CRED");
        assert!(stp.is_stp_compliant());
    }

    #[test]
    fn test_stp_rule_c1_violation() {
        let message_text = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:STP123456789
:23B:CRED
:32A:241231USD1000000,00
:33B:EUR950000,00
:50K:ORDERING CUSTOMER
:59A:DEUTDEFF
:71A:OUR
-}"#;

        let swift_message = SwiftMessage::parse(message_text).expect("Should parse");
        let mt103_stp = MT103STP::from_swift_message(swift_message);

        // Should fail due to C1 rule violation (33B currency differs from 32A but no 36)
        assert!(
            mt103_stp.is_err(),
            "Should fail STP validation due to C1 rule"
        );
    }

    #[test]
    fn test_stp_rule_c7_ben_charges() {
        let message_text = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:STP123456789
:23B:CRED
:32A:241231USD1000000,00
:33B:USD1000000,00
:50K:ORDERING CUSTOMER
:59A:DEUTDEFF
:71A:BEN
-}"#;

        let swift_message = SwiftMessage::parse(message_text).expect("Should parse");
        let mt103_stp = MT103STP::from_swift_message(swift_message);

        // Should fail due to C7 rule violation (71A = BEN requires 71F)
        assert!(
            mt103_stp.is_err(),
            "Should fail STP validation due to C7 rule"
        );
    }

    #[test]
    fn test_valid_stp_with_correspondent_chain() {
        let message_text = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:STP123456789
:23B:CRED
:32A:241231EUR1000000,00
:50K:ORDERING CUSTOMER
:53A:CHASUS33
:54A:DEUTDEFF
:55A:BNPAFRPP
:59A:DEUTDEFF
:71A:OUR
-}"#;

        let swift_message = SwiftMessage::parse(message_text).expect("Should parse");
        let mt103_stp = MT103STP::from_swift_message(swift_message);

        assert!(
            mt103_stp.is_ok(),
            "Should pass STP validation with valid correspondent chain"
        );

        let stp = mt103_stp.unwrap();
        assert!(stp.is_stp_compliant());
        assert!(stp.field_53a.is_some());
        assert!(stp.field_54a.is_some());
        assert!(stp.field_55a.is_some());
    }
}
