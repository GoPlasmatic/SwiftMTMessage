//! MT103: Single Customer Credit Transfer
//!
//! This module contains the MT103 message structure built using the generic field system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::{ParseError, Result};
use crate::field_parser::{SwiftFieldContainer, SwiftMessage};
use crate::json::ToJson;
use crate::mt_models::fields::institutions::{
    Field52, Field53, Field54, Field55, Field56, Field57,
};
use crate::mt_models::fields::{
    Field13C, Field20, Field23B, Field23E, Field26T, Field32A, Field33B, Field36, Field50,
    Field51A, Field59, Field70, Field71A, Field71F, Field71G, Field72, Field77B,
};
use crate::validation::{ValidationReport, validate_mt_message_with_rules};

/// MT103: Single Customer Credit Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT103 {
    // Required fields
    pub field_20: Field20,   // Transaction Reference
    pub field_23b: Field23B, // Bank Operation Code
    pub field_32a: Field32A, // Value Date, Currency, Amount
    pub field_50: Field50,   // Ordering Customer (A/F/K options)
    pub field_59: Field59,   // Beneficiary Customer
    pub field_71a: Field71A, // Details of Charges

    // Optional fields (now complete for 100% MT103 compliance)
    pub field_13c: Option<Field13C>, // Time Indication
    pub field_23e: Option<Field23E>, // Instruction Code
    pub field_26t: Option<Field26T>, // Transaction Type Code
    pub field_33b: Option<Field33B>, // Currency/Instructed Amount
    pub field_36: Option<Field36>,   // Exchange Rate
    pub field_51a: Option<Field51A>, // Sending Institution
    pub field_52a: Option<Field52>,  // Ordering Institution
    pub field_53a: Option<Field53>,  // Sender's Correspondent
    pub field_54a: Option<Field54>,  // Receiver's Correspondent
    pub field_55a: Option<Field55>,  // Third Reimbursement Institution
    pub field_56a: Option<Field56>,  // Intermediary Institution
    pub field_57a: Option<Field57>,  // Account With Institution
    pub field_70: Option<Field70>,   // Remittance Information
    pub field_71f: Option<Field71F>, // Sender's Charges
    pub field_71g: Option<Field71G>, // Receiver's Charges
    pub field_72: Option<Field72>,   // Sender to Receiver Information
    pub field_77b: Option<Field77B>, // Regulatory Reporting
}

impl MT103 {
    /// Create MT103 from generic SwiftMessage
    pub fn from_swift_message(message: SwiftMessage) -> Result<Self> {
        Self::from_swift_message_preserving_headers(message).map(|(mt103, _headers)| mt103)
    }

    /// Create MT103 from generic SwiftMessage, preserving headers for later use
    pub fn from_swift_message_preserving_headers(message: SwiftMessage) -> Result<(Self, (Option<crate::tokenizer::BasicHeader>, Option<crate::tokenizer::ApplicationHeader>, Option<crate::tokenizer::UserHeader>, Option<crate::tokenizer::Trailer>))> {
        if message.message_type != "103" {
            return Err(ParseError::WrongMessageType {
                expected: "103".to_string(),
                actual: message.message_type,
            });
        }

        // Extract required fields
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
        let field_51a = Self::extract_optional_field_51a(&message);
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

        let mt103 = MT103 {
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
            field_51a,
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

        Ok((mt103, headers))
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

        // Add fields in standard order
        fields.insert(
            "20".to_string(),
            SwiftFieldContainer::Field20(self.field_20.clone()),
        );
        field_order.push("20".to_string());

        fields.insert(
            "23B".to_string(),
            SwiftFieldContainer::Field23B(self.field_23b.clone()),
        );
        field_order.push("23B".to_string());

        fields.insert(
            "32A".to_string(),
            SwiftFieldContainer::Field32A(self.field_32a.clone()),
        );
        field_order.push("32A".to_string());

        fields.insert(
            "50".to_string(),
            SwiftFieldContainer::Field50(self.field_50.clone()),
        );
        field_order.push("50".to_string());

        fields.insert(
            "59".to_string(),
            SwiftFieldContainer::Field59(self.field_59.clone()),
        );
        field_order.push("59".to_string());

        fields.insert(
            "71A".to_string(),
            SwiftFieldContainer::Field71A(self.field_71a.clone()),
        );
        field_order.push("71A".to_string());

        SwiftMessage {
            message_type: "103".to_string(),
            basic_header,
            application_header,
            user_header,
            trailer_block,
            blocks: crate::tokenizer::SwiftMessageBlocks::default(), // TODO: populate blocks
            fields,
            field_order,
        }
    }

    // Helper methods for field extraction
    fn extract_field_20(message: &SwiftMessage) -> Result<Field20> {
        match message.get_field("20") {
            Some(SwiftFieldContainer::Field20(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type("20", "103")),
        }
    }

    fn extract_field_23b(message: &SwiftMessage) -> Result<Field23B> {
        match message.get_field("23B") {
            Some(SwiftFieldContainer::Field23B(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type("23B", "103")),
        }
    }

    fn extract_field_32a(message: &SwiftMessage) -> Result<Field32A> {
        match message.get_field("32A") {
            Some(SwiftFieldContainer::Field32A(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type("32A", "103")),
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

        Err(ParseError::missing_required_field_for_type("50", "103"))
    }

    fn extract_field_59(message: &SwiftMessage) -> Result<Field59> {
        // Try different Field59 variants
        if let Some(SwiftFieldContainer::Field59(field)) = message.get_field("59A") {
            return Ok(field.clone());
        }
        if let Some(SwiftFieldContainer::Field59(field)) = message.get_field("59") {
            return Ok(field.clone());
        }

        Err(ParseError::missing_required_field_for_type("59", "103"))
    }

    fn extract_field_71a(message: &SwiftMessage) -> Result<Field71A> {
        match message.get_field("71A") {
            Some(SwiftFieldContainer::Field71A(field)) => Ok(field.clone()),
            _ => Err(ParseError::missing_required_field_for_type("71A", "103")),
        }
    }

    // Helper methods for optional field extraction
    fn extract_optional_field_33b(message: &SwiftMessage) -> Option<Field33B> {
        match message.get_field("33B") {
            Some(SwiftFieldContainer::Field33B(field)) => Some(field.clone()),
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

    // Additional optional field extraction methods
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

    fn extract_optional_field_36(message: &SwiftMessage) -> Option<Field36> {
        match message.get_field("36") {
            Some(SwiftFieldContainer::Field36(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_51a(message: &SwiftMessage) -> Option<Field51A> {
        match message.get_field("51A") {
            Some(SwiftFieldContainer::Field51A(field)) => Some(field.clone()),
            _ => None,
        }
    }

    fn extract_optional_field_52a(message: &SwiftMessage) -> Option<Field52> {
        // Try different Field52 variants
        if let Some(SwiftFieldContainer::Field52(field)) = message.get_field("52A") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field52(field)) = message.get_field("52D") {
            return Some(field.clone());
        }
        None
    }

    fn extract_optional_field_53a(message: &SwiftMessage) -> Option<Field53> {
        // Try different Field53 variants
        if let Some(SwiftFieldContainer::Field53(field)) = message.get_field("53A") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field53(field)) = message.get_field("53B") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field53(field)) = message.get_field("53D") {
            return Some(field.clone());
        }
        None
    }

    fn extract_optional_field_54a(message: &SwiftMessage) -> Option<Field54> {
        // Try different Field54 variants
        if let Some(SwiftFieldContainer::Field54(field)) = message.get_field("54A") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field54(field)) = message.get_field("54B") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field54(field)) = message.get_field("54D") {
            return Some(field.clone());
        }
        None
    }

    fn extract_optional_field_55a(message: &SwiftMessage) -> Option<Field55> {
        // Try different Field55 variants
        if let Some(SwiftFieldContainer::Field55(field)) = message.get_field("55A") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field55(field)) = message.get_field("55B") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field55(field)) = message.get_field("55D") {
            return Some(field.clone());
        }
        None
    }

    fn extract_optional_field_56a(message: &SwiftMessage) -> Option<Field56> {
        // Try different Field56 variants
        if let Some(SwiftFieldContainer::Field56(field)) = message.get_field("56A") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field56(field)) = message.get_field("56C") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field56(field)) = message.get_field("56D") {
            return Some(field.clone());
        }
        None
    }

    fn extract_optional_field_57a(message: &SwiftMessage) -> Option<Field57> {
        // Try different Field57 variants
        if let Some(SwiftFieldContainer::Field57(field)) = message.get_field("57A") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field57(field)) = message.get_field("57B") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field57(field)) = message.get_field("57C") {
            return Some(field.clone());
        }
        if let Some(SwiftFieldContainer::Field57(field)) = message.get_field("57D") {
            return Some(field.clone());
        }
        None
    }

    fn extract_optional_field_71g(message: &SwiftMessage) -> Option<Field71G> {
        match message.get_field("71G") {
            Some(SwiftFieldContainer::Field71G(field)) => Some(field.clone()),
            _ => None,
        }
    }

    /// Validate MT103 specific business rules using JSONLogic rules
    ///
    /// # Arguments
    /// * `rules_file_path` - Path to the JSON file containing MT103 validation rules
    ///
    /// # Returns
    /// * `ValidationReport` containing detailed validation results
    pub fn validate_business_rules(&self) -> Result<ValidationReport> {
        // Convert MT103 to JSON for validation
        let message_json = self.to_json().map_err(|e| ParseError::ValidationError {
            message: format!("Failed to convert MT103 to JSON: {}", e),
        })?;

        // Use the validation utility to apply JSONLogic rules
        validate_mt_message_with_rules(message_json, "config/mt103_validation_rules.json")
    }
}
