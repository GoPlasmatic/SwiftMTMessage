use crate::fields::MultiLineField;
use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 72: Sender to Receiver Information
///
/// ## Overview
/// Field 72 contains sender to receiver information in SWIFT payment messages, providing
/// additional instructions, codes, or information that the sending institution wants to
/// communicate to the receiving institution. This field serves as a communication channel
/// for operational instructions, regulatory codes, special handling requirements, and other
/// relevant information that supports proper payment processing and compliance.
///
/// ## Format Specification
/// **Format**: `6*35x`
/// - **6*35x**: Up to 6 lines of 35 characters each
/// - **Line structure**: Free-form text for instructions and information
/// - **Character set**: SWIFT character set (A-Z, 0-9, and limited special characters)
/// - **Line separation**: Each line on separate row
///
/// ## Structure
/// ```text
/// /ACC/BENEFICIARY ACCOUNT DETAILS
/// /BNF/ADDITIONAL BENEFICIARY INFO
/// /INS/SPECIAL HANDLING REQUIRED
/// /REC/REGULATORY REPORTING CODE
/// /FEE/CHARGE INSTRUCTIONS
/// /REM/ADDITIONAL REMITTANCE INFO
/// │                              │
/// └──────────────────────────────┘
///        Up to 35 characters per line
///        Maximum 6 lines
/// ```
///
/// ## Field Components
/// - **Instruction Codes**: Structured codes for specific instructions
/// - **Regulatory Information**: Compliance and reporting codes
/// - **Processing Instructions**: Special handling requirements
/// - **Additional Details**: Supplementary payment information
/// - **Communication Messages**: Bank-to-bank communications
///
/// ## Usage Context
/// Field 72 is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Operational instructions**: Special processing requirements
/// - **Regulatory compliance**: Required reporting codes and information
/// - **Payment routing**: Additional routing or handling instructions
/// - **Fee instructions**: Charge allocation and billing details
/// - **Beneficiary information**: Additional beneficiary details
/// - **Investigation support**: Information for payment inquiries
///
/// ## Common Instruction Codes
/// ### /ACC/ - Account Information
/// - Additional account details or instructions
/// - Alternative account numbers or references
/// - Account-specific processing requirements
///
/// ### /BNF/ - Beneficiary Information
/// - Additional beneficiary identification
/// - Alternative beneficiary details
/// - Beneficiary-specific instructions
///
/// ### /INS/ - Special Instructions
/// - Processing instructions for receiving bank
/// - Handling requirements or restrictions
/// - Operational guidance
///
/// ### /REC/ - Regulatory Information
/// - Regulatory reporting codes
/// - Compliance-related information
/// - Authority-required data
///
/// ### /FEE/ - Fee Instructions
/// - Charge allocation instructions
/// - Fee payment arrangements
/// - Billing-related information
///
/// ### /REM/ - Additional Remittance
/// - Supplementary remittance information
/// - Extended payment descriptions
/// - Reference details
///
/// ## Examples
/// ```text
/// :72:/ACC/CREDIT TO ACCOUNT 123456789
/// └─── Account crediting instruction
///
/// :72:/BNF/JOHN DOE TRADING COMPANY
/// /INS/URGENT PROCESSING REQUIRED
/// /REC/REGULATORY CODE ABC123
/// └─── Multi-line instructions with codes
///
/// :72:/FEE/CHARGES TO BE SHARED
/// /REM/INVOICE REF INV-2024-001
/// └─── Fee and remittance instructions
///
/// :72:CORRESPONDENT BANK CHARGES APPLY
/// BENEFICIARY BANK TO DEDUCT FEES
/// PAYMENT FOR TRADE SETTLEMENT
/// └─── Free-form instructions without codes
/// ```
///
/// ## Information Categories
/// - **Processing instructions**: How to handle the payment
/// - **Regulatory codes**: Required compliance information
/// - **Routing details**: Additional routing or correspondent instructions
/// - **Charge information**: Fee allocation and payment instructions
/// - **Reference data**: Additional reference numbers or codes
/// - **Special requirements**: Urgent processing, holds, or restrictions
///
/// ## Validation Rules
/// 1. **Line count**: Maximum 6 lines
/// 2. **Line length**: Maximum 35 characters per line
/// 3. **Character set**: SWIFT character set only
/// 4. **Content**: Should contain meaningful instructions or information
/// 5. **Empty lines**: Generally avoided for clarity
/// 6. **Control characters**: Not allowed
/// 7. **Special characters**: Limited to SWIFT-approved set
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Maximum 6 lines allowed (Error: T26)
/// - Each line maximum 35 characters (Error: T50)
/// - Must use SWIFT character set only (Error: T61)
/// - Content should be meaningful (Error: T40)
/// - No control characters permitted (Error: T35)
/// - Field is optional but widely used (Warning: W72)
/// - Structured codes should follow conventions (Warning: W73)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field72 {
    /// Information lines (up to 6 lines of 35 characters each)
    pub information: Vec<String>,
}

impl MultiLineField for Field72 {
    const MAX_LINES: usize = 6;
    const FIELD_TAG: &'static str = "72";

    fn lines(&self) -> &[String] {
        &self.information
    }

    fn lines_mut(&mut self) -> &mut Vec<String> {
        &mut self.information
    }

    fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
        Ok(Field72 { information: lines })
    }
}

impl Field72 {
    /// Create a new Field72 with validation
    pub fn new(information: Vec<String>) -> Result<Self, ParseError> {
        <Self as MultiLineField>::new(information)
    }

    /// Create from a single string, splitting on newlines
    pub fn from_string(content: impl Into<String>) -> Result<Self, ParseError> {
        let content = content.into();
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self::new(lines)
    }

    /// Get the information lines
    pub fn information(&self) -> &[String] {
        &self.information
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.information.len()
    }

    /// Get a specific line by index
    pub fn line(&self, index: usize) -> Option<&str> {
        self.information.get(index).map(|s| s.as_str())
    }

    /// Add a line of information
    pub fn add_line(&mut self, line: String) -> Result<(), ParseError> {
        <Self as MultiLineField>::add_line(self, line)
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Sender to Receiver Information ({} lines)",
            self.line_count()
        )
    }
}

impl SwiftField for Field72 {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Self::parse_content(value)
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_format()
    }

    fn validate(&self) -> ValidationResult {
        self.validate_multiline()
    }

    fn format_spec() -> &'static str {
        "6*35x"
    }
}

impl std::fmt::Display for Field72 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.information.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field72_creation() {
        let lines = vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()];
        let field = Field72::new(lines.clone()).unwrap();
        assert_eq!(field.information(), &lines);
        assert_eq!(field.line_count(), 2);
    }

    #[test]
    fn test_field72_from_string() {
        let content = "/INS/CHQS\n/BENEFRES/BE\n/ORDERRES/DE";
        let field = Field72::from_string(content).unwrap();
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.line(0), Some("/INS/CHQS"));
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
        assert_eq!(field.line(2), Some("/ORDERRES/DE"));
    }

    #[test]
    fn test_field72_parse() {
        let field = Field72::parse("/INS/CHQS\n/BENEFRES/BE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("/INS/CHQS"));
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
    }

    #[test]
    fn test_field72_parse_with_prefix() {
        let field = Field72::parse(":72:/INS/CHQS\n/BENEFRES/BE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("/INS/CHQS"));
    }

    #[test]
    fn test_field72_to_swift_string() {
        let lines = vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()];
        let field = Field72::new(lines).unwrap();
        assert_eq!(field.to_swift_string(), ":72:/INS/CHQS\n/BENEFRES/BE");
    }

    #[test]
    fn test_field72_add_line() {
        let mut field = Field72::new(vec!["/INS/CHQS".to_string()]).unwrap();
        field.add_line("/BENEFRES/BE".to_string()).unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
    }

    #[test]
    fn test_field72_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(),
            "Line 6".to_string(),
            "Line 7".to_string(), // Too many
        ];
        let result = Field72::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field72_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field72::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field72_empty() {
        let result = Field72::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_field72_validation() {
        let field = Field72::new(vec!["/INS/CHQS".to_string()]).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field72_display() {
        let field =
            Field72::new(vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()]).unwrap();
        assert_eq!(format!("{}", field), "/INS/CHQS\n/BENEFRES/BE");
    }

    #[test]
    fn test_field72_description() {
        let field =
            Field72::new(vec!["/INS/CHQS".to_string(), "/BENEFRES/BE".to_string()]).unwrap();
        assert_eq!(
            field.description(),
            "Sender to Receiver Information (2 lines)"
        );
    }

    #[test]
    fn test_field72_line_access() {
        let field = Field72::new(vec!["/INS/CHQS".to_string()]).unwrap();
        assert_eq!(field.line(0), Some("/INS/CHQS"));
        assert_eq!(field.line(1), None);
    }

    #[test]
    fn test_field72_add_line_max_reached() {
        let mut field = Field72::new(vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(),
            "Line 6".to_string(),
        ])
        .unwrap();

        let result = field.add_line("Line 7".to_string());
        assert!(result.is_err());
    }
}
