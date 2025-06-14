use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 77T: Envelope Contents
///
/// ## Overview
/// Field 77T contains envelope contents information in SWIFT MT103.REMIT messages, providing
/// structured data about the remittance information envelope. This field is mandatory for
/// MT103.REMIT messages and contains codes that identify the type and format of remittance
/// information being transmitted. It supports the Extended Remittance Registration (ERR)
/// framework for structured remittance data exchange.
///
/// ## Format Specification
/// **Format**: `1!a1!a/35x`
/// - **1!a**: Envelope type code (1 alphabetic character)
/// - **1!a**: Envelope format code (1 alphabetic character)
/// - **/**: Separator
/// - **35x**: Envelope identifier (up to 35 characters)
///
/// ## Structure
/// ```text
/// RD/REMITTANCE-2024-001234567890
/// ││ │
/// ││ └─ Envelope identifier (up to 35 chars)
/// │└─── Format code (D = Detailed)
/// └──── Type code (R = Remittance)
/// ```
///
/// ## Field Components
/// - **Envelope Type**: Single character indicating envelope content type
/// - **Envelope Format**: Single character indicating data format
/// - **Envelope Identifier**: Unique identifier for the envelope contents
///
/// ## Usage Context
/// Field 77T is used in:
/// - **MT103.REMIT**: Single Customer Credit Transfer with Remittance (mandatory)
///
/// ### Business Applications
/// - **Structured remittance**: Supporting ISO 20022 remittance data
/// - **Extended remittance**: Enabling detailed payment information
/// - **Regulatory compliance**: Meeting remittance reporting requirements
/// - **Automated processing**: Supporting straight-through remittance processing
/// - **Invoice matching**: Facilitating automated accounts receivable processing
///
/// ## Envelope Type Codes
/// ### R - Remittance Information
/// - **Description**: Contains structured remittance data
/// - **Usage**: Standard remittance information envelope
/// - **Content**: Invoice details, payment references, structured data
///
/// ### S - Supplementary Information
/// - **Description**: Additional supporting information
/// - **Usage**: Supplementary data beyond basic remittance
/// - **Content**: Extended commercial details, regulatory data
///
/// ### T - Trade Information
/// - **Description**: Trade finance related information
/// - **Usage**: Trade settlement and documentary credit data
/// - **Content**: LC references, trade documents, commercial terms
///
/// ## Envelope Format Codes
/// ### D - Detailed Format
/// - **Description**: Comprehensive structured format
/// - **Usage**: Full remittance data with all available fields
/// - **Content**: Complete invoice and payment details
///
/// ### S - Summary Format
/// - **Description**: Condensed format with key information
/// - **Usage**: Essential remittance data only
/// - **Content**: Basic payment references and amounts
///
/// ### C - Custom Format
/// - **Description**: Institution-specific format
/// - **Usage**: Proprietary or specialized data structures
/// - **Content**: Custom remittance information layout
///
/// ## Examples
/// ```text
/// :77T:RD/REMITTANCE-2024-001234567890
/// └─── Detailed remittance envelope
///
/// :77T:SS/SUPP-INFO-2024-03-15-001
/// └─── Summary supplementary information
///
/// :77T:TC/TRADE-LC-2024-567890123
/// └─── Custom trade information envelope
///
/// :77T:RD/INV-2024-001234-PAYMENT-REF
/// └─── Invoice-based remittance envelope
/// ```
///
/// ## Envelope Identifier Guidelines
/// - **Uniqueness**: Should be unique within sender's system
/// - **Traceability**: Enable tracking and reconciliation
/// - **Format**: Alphanumeric with limited special characters
/// - **Length**: Maximum 35 characters
/// - **Content**: Meaningful identifier for envelope contents
///
/// ## Common Identifier Patterns
/// - **REMITTANCE-YYYY-NNNNNNNNNN**: Standard remittance pattern
/// - **INV-YYYY-NNNNNN-REF**: Invoice-based identifier
/// - **TRADE-LC-YYYY-NNNNNN**: Trade finance identifier
/// - **SUPP-INFO-YYYY-MM-DD-NNN**: Supplementary information pattern
///
/// ## Validation Rules
/// 1. **Envelope type**: Must be single alphabetic character
/// 2. **Envelope format**: Must be single alphabetic character
/// 3. **Separator**: Must be forward slash (/)
/// 4. **Identifier length**: Maximum 35 characters
/// 5. **Character set**: SWIFT character set only
/// 6. **Content validation**: Identifier must be meaningful
/// 7. **Uniqueness**: Should be unique within context
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Envelope type must be alphabetic (Error: T15)
/// - Envelope format must be alphabetic (Error: T15)
/// - Separator must be forward slash (Error: T77)
/// - Identifier cannot exceed 35 characters (Error: T50)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Field 77T mandatory in MT103.REMIT (Error: M77)
/// - Field 77T not used in MT103 Core/STP (Error: C77)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field77T {
    /// Envelope type code (1 alphabetic character)
    pub envelope_type: String,

    /// Envelope format code (1 alphabetic character)
    pub envelope_format: String,

    /// Envelope identifier (up to 35 characters)
    pub envelope_identifier: String,
}

impl SwiftField for Field77T {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":77T:") {
            stripped
        } else if let Some(stripped) = value.strip_prefix("77T:") {
            stripped
        } else {
            value
        };

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = content.trim();

        // Expected format: XY/identifier
        if content.len() < 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Content too short (minimum format: XY/id)".to_string(),
            });
        }

        let envelope_type = content.chars().nth(0).unwrap().to_string();
        let envelope_format = content.chars().nth(1).unwrap().to_string();

        if content.chars().nth(2) != Some('/') {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Missing separator '/' after envelope codes".to_string(),
            });
        }

        let envelope_identifier = content[3..].to_string();

        Self::new(envelope_type, envelope_format, envelope_identifier)
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":77T:{}{}/{}",
            self.envelope_type, self.envelope_format, self.envelope_identifier
        )
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate envelope type (1 alphabetic character)
        if self.envelope_type.len() != 1 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "77T".to_string(),
                expected: "1 character".to_string(),
                actual: self.envelope_type.len(),
            });
        } else if !self.envelope_type.chars().all(|c| c.is_ascii_alphabetic()) {
            errors.push(ValidationError::FormatValidation {
                field_tag: "77T".to_string(),
                message: "Envelope type must be alphabetic".to_string(),
            });
        }

        // Validate envelope format (1 alphabetic character)
        if self.envelope_format.len() != 1 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "77T".to_string(),
                expected: "1 character".to_string(),
                actual: self.envelope_format.len(),
            });
        } else if !self
            .envelope_format
            .chars()
            .all(|c| c.is_ascii_alphabetic())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "77T".to_string(),
                message: "Envelope format must be alphabetic".to_string(),
            });
        }

        // Validate envelope identifier
        if self.envelope_identifier.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "77T".to_string(),
                message: "Envelope identifier cannot be empty".to_string(),
            });
        }

        if self.envelope_identifier.len() > 35 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "77T".to_string(),
                expected: "max 35 characters".to_string(),
                actual: self.envelope_identifier.len(),
            });
        }

        // Validate character set (SWIFT character set)
        if !self
            .envelope_identifier
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "77T".to_string(),
                message: "Envelope identifier contains invalid characters".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "1!a1!a/35x"
    }
}

impl Field77T {
    /// Create a new Field77T with validation
    ///
    /// # Arguments
    /// * `envelope_type` - Envelope type code (1 alphabetic character)
    /// * `envelope_format` - Envelope format code (1 alphabetic character)
    /// * `envelope_identifier` - Envelope identifier (up to 35 characters)
    ///
    /// # Examples
    /// ```rust
    /// use swift_mt_message::fields::Field77T;
    ///
    /// // Detailed remittance envelope
    /// let field = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
    ///
    /// // Summary supplementary information
    /// let field = Field77T::new("S", "S", "SUPP-INFO-2024-03-15-001").unwrap();
    ///
    /// // Custom trade information
    /// let field = Field77T::new("T", "C", "TRADE-LC-2024-567890123").unwrap();
    /// ```
    pub fn new(
        envelope_type: impl Into<String>,
        envelope_format: impl Into<String>,
        envelope_identifier: impl Into<String>,
    ) -> crate::Result<Self> {
        let envelope_type = envelope_type.into().trim().to_uppercase();
        let envelope_format = envelope_format.into().trim().to_uppercase();
        let envelope_identifier = envelope_identifier.into().trim().to_string();

        // Validate envelope type
        if envelope_type.len() != 1 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Envelope type must be exactly 1 character".to_string(),
            });
        }

        if !envelope_type.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Envelope type must be alphabetic".to_string(),
            });
        }

        // Validate envelope format
        if envelope_format.len() != 1 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Envelope format must be exactly 1 character".to_string(),
            });
        }

        if !envelope_format.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Envelope format must be alphabetic".to_string(),
            });
        }

        // Validate envelope identifier
        if envelope_identifier.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Envelope identifier cannot be empty".to_string(),
            });
        }

        if envelope_identifier.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Envelope identifier cannot exceed 35 characters".to_string(),
            });
        }

        if !envelope_identifier
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77T".to_string(),
                message: "Envelope identifier contains invalid characters".to_string(),
            });
        }

        Ok(Field77T {
            envelope_type,
            envelope_format,
            envelope_identifier,
        })
    }

    /// Get the envelope type code
    pub fn envelope_type(&self) -> &str {
        &self.envelope_type
    }

    /// Get the envelope format code
    pub fn envelope_format(&self) -> &str {
        &self.envelope_format
    }

    /// Get the envelope identifier
    pub fn envelope_identifier(&self) -> &str {
        &self.envelope_identifier
    }

    /// Check if this is a remittance information envelope
    pub fn is_remittance_envelope(&self) -> bool {
        self.envelope_type == "R"
    }

    /// Check if this is a supplementary information envelope
    pub fn is_supplementary_envelope(&self) -> bool {
        self.envelope_type == "S"
    }

    /// Check if this is a trade information envelope
    pub fn is_trade_envelope(&self) -> bool {
        self.envelope_type == "T"
    }

    /// Check if this uses detailed format
    pub fn is_detailed_format(&self) -> bool {
        self.envelope_format == "D"
    }

    /// Check if this uses summary format
    pub fn is_summary_format(&self) -> bool {
        self.envelope_format == "S"
    }

    /// Check if this uses custom format
    pub fn is_custom_format(&self) -> bool {
        self.envelope_format == "C"
    }

    /// Get a description of the envelope type and format
    pub fn description(&self) -> String {
        let type_desc = match self.envelope_type.as_str() {
            "R" => "Remittance Information",
            "S" => "Supplementary Information",
            "T" => "Trade Information",
            _ => "Unknown Type",
        };

        let format_desc = match self.envelope_format.as_str() {
            "D" => "Detailed Format",
            "S" => "Summary Format",
            "C" => "Custom Format",
            _ => "Unknown Format",
        };

        format!(
            "{} - {} ({})",
            type_desc, format_desc, self.envelope_identifier
        )
    }

    /// Check if this field is required for MT103.REMIT
    pub fn is_required_for_remit(&self) -> bool {
        true // Field 77T is mandatory for MT103.REMIT
    }

    /// Check if this field is allowed in MT103 Core/STP
    pub fn is_allowed_in_core_stp(&self) -> bool {
        false // Field 77T is not used in MT103 Core/STP
    }
}

impl std::fmt::Display for Field77T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}/{}",
            self.envelope_type, self.envelope_format, self.envelope_identifier
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field77t_creation() {
        let field = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
        assert_eq!(field.envelope_type(), "R");
        assert_eq!(field.envelope_format(), "D");
        assert_eq!(field.envelope_identifier(), "REMITTANCE-2024-001234567890");
    }

    #[test]
    fn test_field77t_parse() {
        let field = Field77T::parse("RD/REMITTANCE-2024-001234567890").unwrap();
        assert_eq!(field.envelope_type(), "R");
        assert_eq!(field.envelope_format(), "D");
        assert_eq!(field.envelope_identifier(), "REMITTANCE-2024-001234567890");
    }

    #[test]
    fn test_field77t_parse_with_prefix() {
        let field = Field77T::parse(":77T:SS/SUPP-INFO-2024-03-15-001").unwrap();
        assert_eq!(field.envelope_type(), "S");
        assert_eq!(field.envelope_format(), "S");
        assert_eq!(field.envelope_identifier(), "SUPP-INFO-2024-03-15-001");

        let field = Field77T::parse("77T:TC/TRADE-LC-2024-567890123").unwrap();
        assert_eq!(field.envelope_type(), "T");
        assert_eq!(field.envelope_format(), "C");
        assert_eq!(field.envelope_identifier(), "TRADE-LC-2024-567890123");
    }

    #[test]
    fn test_field77t_case_normalization() {
        let field = Field77T::new("r", "d", "remittance-2024-001").unwrap();
        assert_eq!(field.envelope_type(), "R");
        assert_eq!(field.envelope_format(), "D");
        assert_eq!(field.envelope_identifier(), "remittance-2024-001");
    }

    #[test]
    fn test_field77t_invalid_envelope_type() {
        let result = Field77T::new("", "D", "REMITTANCE-2024-001");
        assert!(result.is_err());

        let result = Field77T::new("AB", "D", "REMITTANCE-2024-001");
        assert!(result.is_err());

        let result = Field77T::new("1", "D", "REMITTANCE-2024-001");
        assert!(result.is_err());
    }

    #[test]
    fn test_field77t_invalid_envelope_format() {
        let result = Field77T::new("R", "", "REMITTANCE-2024-001");
        assert!(result.is_err());

        let result = Field77T::new("R", "AB", "REMITTANCE-2024-001");
        assert!(result.is_err());

        let result = Field77T::new("R", "1", "REMITTANCE-2024-001");
        assert!(result.is_err());
    }

    #[test]
    fn test_field77t_invalid_identifier() {
        let result = Field77T::new("R", "D", "");
        assert!(result.is_err());

        let result = Field77T::new("R", "D", "a".repeat(36));
        assert!(result.is_err());
    }

    #[test]
    fn test_field77t_invalid_format() {
        let result = Field77T::parse("RD");
        assert!(result.is_err());

        let result = Field77T::parse("R/IDENTIFIER");
        assert!(result.is_err());

        let result = Field77T::parse("RDIDENTIFIER");
        assert!(result.is_err());
    }

    #[test]
    fn test_field77t_to_swift_string() {
        let field = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":77T:RD/REMITTANCE-2024-001234567890"
        );

        let field = Field77T::new("S", "S", "SUPP-INFO-2024-03-15-001").unwrap();
        assert_eq!(field.to_swift_string(), ":77T:SS/SUPP-INFO-2024-03-15-001");
    }

    #[test]
    fn test_field77t_validation() {
        let field = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let invalid_field = Field77T {
            envelope_type: "".to_string(),
            envelope_format: "D".to_string(),
            envelope_identifier: "REMITTANCE-2024-001".to_string(),
        };
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field77t_type_checks() {
        let remittance_field = Field77T::new("R", "D", "REMITTANCE-2024-001").unwrap();
        assert!(remittance_field.is_remittance_envelope());
        assert!(!remittance_field.is_supplementary_envelope());
        assert!(!remittance_field.is_trade_envelope());

        let supp_field = Field77T::new("S", "S", "SUPP-INFO-2024-001").unwrap();
        assert!(!supp_field.is_remittance_envelope());
        assert!(supp_field.is_supplementary_envelope());
        assert!(!supp_field.is_trade_envelope());

        let trade_field = Field77T::new("T", "C", "TRADE-LC-2024-001").unwrap();
        assert!(!trade_field.is_remittance_envelope());
        assert!(!trade_field.is_supplementary_envelope());
        assert!(trade_field.is_trade_envelope());
    }

    #[test]
    fn test_field77t_format_checks() {
        let detailed_field = Field77T::new("R", "D", "REMITTANCE-2024-001").unwrap();
        assert!(detailed_field.is_detailed_format());
        assert!(!detailed_field.is_summary_format());
        assert!(!detailed_field.is_custom_format());

        let summary_field = Field77T::new("S", "S", "SUPP-INFO-2024-001").unwrap();
        assert!(!summary_field.is_detailed_format());
        assert!(summary_field.is_summary_format());
        assert!(!summary_field.is_custom_format());

        let custom_field = Field77T::new("T", "C", "TRADE-LC-2024-001").unwrap();
        assert!(!custom_field.is_detailed_format());
        assert!(!custom_field.is_summary_format());
        assert!(custom_field.is_custom_format());
    }

    #[test]
    fn test_field77t_compliance_checks() {
        let field = Field77T::new("R", "D", "REMITTANCE-2024-001").unwrap();
        assert!(field.is_required_for_remit());
        assert!(!field.is_allowed_in_core_stp());
    }

    #[test]
    fn test_field77t_description() {
        let field = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
        let description = field.description();
        assert!(description.contains("Remittance Information"));
        assert!(description.contains("Detailed Format"));
        assert!(description.contains("REMITTANCE-2024-001234567890"));
    }

    #[test]
    fn test_field77t_display() {
        let field = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
        assert_eq!(format!("{}", field), "RD/REMITTANCE-2024-001234567890");

        let field = Field77T::new("S", "S", "SUPP-INFO-2024-03-15-001").unwrap();
        assert_eq!(format!("{}", field), "SS/SUPP-INFO-2024-03-15-001");
    }

    #[test]
    fn test_field77t_format_spec() {
        assert_eq!(Field77T::format_spec(), "1!a1!a/35x");
    }

    #[test]
    fn test_field77t_real_world_examples() {
        // Standard remittance envelope
        let remittance = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
        assert_eq!(
            remittance.to_swift_string(),
            ":77T:RD/REMITTANCE-2024-001234567890"
        );
        assert!(remittance.is_remittance_envelope());
        assert!(remittance.is_detailed_format());

        // Invoice-based identifier
        let invoice = Field77T::new("R", "D", "INV-2024-001234-PAYMENT-REF").unwrap();
        assert_eq!(
            invoice.to_swift_string(),
            ":77T:RD/INV-2024-001234-PAYMENT-REF"
        );

        // Trade finance envelope
        let trade = Field77T::new("T", "C", "TRADE-LC-2024-567890123").unwrap();
        assert_eq!(trade.to_swift_string(), ":77T:TC/TRADE-LC-2024-567890123");
        assert!(trade.is_trade_envelope());
        assert!(trade.is_custom_format());

        // Supplementary information
        let supp = Field77T::new("S", "S", "SUPP-INFO-2024-03-15-001").unwrap();
        assert_eq!(supp.to_swift_string(), ":77T:SS/SUPP-INFO-2024-03-15-001");
        assert!(supp.is_supplementary_envelope());
        assert!(supp.is_summary_format());
    }
}
