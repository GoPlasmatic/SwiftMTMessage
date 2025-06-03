//! Message validation logic for SWIFT MT messages

use crate::error::{MTError, Result};
use crate::messages::MTMessage;

/// Validation levels for MT messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    /// Basic structure validation only
    Basic,
    /// Standard validation including field formats
    Standard,
    /// Strict validation including business rules
    Strict,
}

/// Validation result containing errors and warnings
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    pub fn warnings(&self) -> &[ValidationWarning] {
        &self.warnings
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: String,
    pub error_code: String,
}

impl ValidationError {
    pub fn new(field: Option<String>, message: String, error_code: String) -> Self {
        Self {
            field,
            message,
            error_code,
        }
    }
}

/// Validation warning
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub field: Option<String>,
    pub message: String,
    pub warning_code: String,
}

impl ValidationWarning {
    pub fn new(field: Option<String>, message: String, warning_code: String) -> Self {
        Self {
            field,
            message,
            warning_code,
        }
    }
}

/// Main validator for MT messages
pub struct MTValidator;

impl MTValidator {
    /// Validate an MT message with the specified validation level
    pub fn validate(message: &MTMessage, level: ValidationLevel) -> ValidationResult {
        let mut result = ValidationResult::new();

        match message {
            MTMessage::MT103(mt103) => {
                Self::validate_mt103(mt103, level, &mut result);
            }
            MTMessage::MT102(mt102) => {
                Self::validate_mt102(mt102, level, &mut result);
            }
            MTMessage::MT202(mt202) => {
                Self::validate_mt202(mt202, level, &mut result);
            }
            MTMessage::MT202COV(mt202cov) => {
                Self::validate_mt202cov(mt202cov, level, &mut result);
            }
            MTMessage::MT210(mt210) => {
                Self::validate_mt210(mt210, level, &mut result);
            }
            MTMessage::MT940(mt940) => {
                Self::validate_mt940(mt940, level, &mut result);
            }
            MTMessage::MT941(mt941) => {
                Self::validate_mt941(mt941, level, &mut result);
            }
            MTMessage::MT942(mt942) => {
                Self::validate_mt942(mt942, level, &mut result);
            }
            MTMessage::MT192(mt192) => {
                Self::validate_mt192(mt192, level, &mut result);
            }
            MTMessage::MT195(mt195) => {
                Self::validate_mt195(mt195, level, &mut result);
            }
            MTMessage::MT196(mt196) => {
                Self::validate_mt196(mt196, level, &mut result);
            }
            MTMessage::MT197(mt197) => {
                Self::validate_mt197(mt197, level, &mut result);
            }
            MTMessage::MT199(mt199) => {
                Self::validate_mt199(mt199, level, &mut result);
            }
        }

        result
    }

    fn validate_mt103(
        _mt103: &crate::messages::mt103::MT103,
        level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        match level {
            ValidationLevel::Basic => {
                // Basic validation - check required fields exist
                // This will be implemented when we have the MT103 struct
            }
            ValidationLevel::Standard => {
                // Standard validation - check field formats
            }
            ValidationLevel::Strict => {
                // Strict validation - check business rules
            }
        }
    }

    fn validate_mt102(
        _mt102: &crate::messages::mt102::MT102,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT102 validation
    }

    fn validate_mt202(
        _mt202: &crate::messages::mt202::MT202,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT202 validation
    }

    fn validate_mt202cov(
        _mt202cov: &crate::messages::mt202cov::MT202COV,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT202COV validation
    }

    fn validate_mt210(
        _mt210: &crate::messages::mt210::MT210,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT210 validation
    }

    fn validate_mt940(
        _mt940: &crate::messages::mt940::MT940,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT940 validation
    }

    fn validate_mt941(
        _mt941: &crate::messages::mt941::MT941,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT941 validation
    }

    fn validate_mt942(
        _mt942: &crate::messages::mt942::MT942,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT942 validation
    }

    fn validate_mt192(
        _mt192: &crate::messages::mt192::MT192,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT192 validation
    }

    fn validate_mt195(
        _mt195: &crate::messages::mt195::MT195,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT195 validation
    }

    fn validate_mt196(
        _mt196: &crate::messages::mt196::MT196,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT196 validation
    }

    fn validate_mt197(
        _mt197: &crate::messages::mt197::MT197,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT197 validation
    }

    fn validate_mt199(
        _mt199: &crate::messages::mt199::MT199,
        _level: ValidationLevel,
        _result: &mut ValidationResult,
    ) {
        // TODO: Implement MT199 validation
    }
}

/// Validate a SWIFT MT message
pub fn validate_message(message: &MTMessage, level: ValidationLevel) -> ValidationResult {
    MTValidator::validate(message, level)
}

/// Common field validation functions
pub mod field_validators {
    use super::*;
    use regex::Regex;

    /// Validate BIC (Bank Identifier Code)
    pub fn validate_bic(bic: &str) -> Result<()> {
        let bic_regex = Regex::new(r"^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$").unwrap();
        if bic_regex.is_match(bic) {
            Ok(())
        } else {
            Err(MTError::ValidationError {
                field: "BIC".to_string(),
                message: format!("Invalid BIC format: {}", bic),
            })
        }
    }

    /// Validate IBAN (International Bank Account Number)
    pub fn validate_iban(iban: &str) -> Result<()> {
        // Basic IBAN format validation
        let iban_regex =
            Regex::new(r"^[A-Z]{2}[0-9]{2}[A-Z0-9]{4}[0-9]{7}([A-Z0-9]?){0,16}$").unwrap();
        if iban_regex.is_match(iban) {
            Ok(())
        } else {
            Err(MTError::ValidationError {
                field: "IBAN".to_string(),
                message: format!("Invalid IBAN format: {}", iban),
            })
        }
    }

    /// Validate amount format
    pub fn validate_amount(amount: &str) -> Result<()> {
        let amount_regex = Regex::new(r"^[A-Z]{3}[0-9]+([,.][0-9]{1,2})?$").unwrap();
        if amount_regex.is_match(amount) {
            Ok(())
        } else {
            Err(MTError::ValidationError {
                field: "Amount".to_string(),
                message: format!("Invalid amount format: {}", amount),
            })
        }
    }

    /// Validate date format (YYMMDD)
    pub fn validate_date_yymmdd(date: &str) -> Result<()> {
        use crate::common::SwiftDate;

        let date_regex = Regex::new(r"^[0-9]{6}$").unwrap();
        if date_regex.is_match(date) {
            // Use SwiftDate to validate the actual date
            SwiftDate::parse_yymmdd(date).map_err(|_| MTError::ValidationError {
                field: "Date".to_string(),
                message: format!("Invalid date: {}", date),
            })?;
            Ok(())
        } else {
            Err(MTError::ValidationError {
                field: "Date".to_string(),
                message: format!("Invalid date format, expected YYMMDD: {}", date),
            })
        }
    }

    /// Validate reference number format
    pub fn validate_reference(reference: &str) -> Result<()> {
        if reference.len() > 16 {
            return Err(MTError::ValidationError {
                field: "Reference".to_string(),
                message: format!("Reference too long (max 16 chars): {}", reference),
            });
        }

        let ref_regex = Regex::new(r"^[A-Z0-9/\-?:().,'+\s]*$").unwrap();
        if ref_regex.is_match(reference) {
            Ok(())
        } else {
            Err(MTError::ValidationError {
                field: "Reference".to_string(),
                message: format!("Invalid reference format: {}", reference),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::field_validators::*;
    use super::*;

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();
        assert!(result.is_valid());
        assert!(!result.has_warnings());

        result.add_error(ValidationError::new(
            Some("20".to_string()),
            "Missing field".to_string(),
            "E001".to_string(),
        ));
        assert!(!result.is_valid());

        result.add_warning(ValidationWarning::new(
            Some("23B".to_string()),
            "Deprecated field".to_string(),
            "W001".to_string(),
        ));
        assert!(result.has_warnings());
    }

    #[test]
    fn test_bic_validation() {
        assert!(validate_bic("BANKDEFFXXX").is_ok());
        assert!(validate_bic("BANKDEFF").is_ok());
        assert!(validate_bic("BANKDE").is_err());
        assert!(validate_bic("bankdeffxxx").is_err());
    }

    #[test]
    fn test_amount_validation() {
        assert!(validate_amount("EUR1234567,89").is_ok());
        assert!(validate_amount("USD1000.50").is_ok());
        assert!(validate_amount("EUR1000").is_ok());
        assert!(validate_amount("1000.50").is_err());
        assert!(validate_amount("EUR").is_err());
    }

    #[test]
    fn test_date_validation() {
        assert!(validate_date_yymmdd("210315").is_ok());
        assert!(validate_date_yymmdd("991231").is_ok());
        assert!(validate_date_yymmdd("210230").is_err()); // Invalid date (Feb 30)
        assert!(validate_date_yymmdd("211301").is_err()); // Invalid month
        assert!(validate_date_yymmdd("21031").is_err()); // Too short
    }

    #[test]
    fn test_reference_validation() {
        assert!(validate_reference("FT21234567890").is_ok());
        assert!(validate_reference("REF-123/456").is_ok());
        assert!(validate_reference("A".repeat(17).as_str()).is_err()); // Too long
    }
}
