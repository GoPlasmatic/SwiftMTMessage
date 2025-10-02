use super::field_utils::parse_multiline_text;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 70: Remittance Information**
///
/// ## Purpose
/// Specifies details of individual transactions or references to other messages containing
/// details to be transmitted to the beneficiary customer. This field carries payment-related
/// information that helps the beneficiary identify the purpose of the payment, reconcile
/// accounts, and process the transaction appropriately. Critical for STP processing and
/// beneficiary transaction identification.
///
/// ## Format Specification
/// - **Swift Format**: `4*35x`
/// - **Structure**: Up to 4 lines of 35 characters each
/// - **Content**: Narrative format with structured codes and references
/// - **Character Set**: SWIFT character set (A-Z, 0-9, space, and limited special characters)
///
/// ## Business Context Applications
/// - **Payment Instructions**: MT 103 STP and customer credit transfers
/// - **Beneficiary Information**: Details for payment recipient identification
/// - **Invoice References**: Commercial payment references and invoice details
/// - **Trade Finance**: Documentary credit and trade transaction references
///
/// ## Network Validation Requirements
/// - **Length Restrictions**: Maximum 4 lines of 35 characters each
/// - **Character Set**: Must use valid SWIFT character set
/// - **Format Compliance**: Structured codes must follow specified formats
/// - **Reference Validation**: Invoice and payment references must be properly formatted
/// - **STP Requirements**: ISO 11649 Creditor Reference must appear alone on first line for STP
///
/// ## Structured Content Codes
/// ### Payment References
/// - **INV**: Invoice reference followed by date, reference, and details
/// - **IPI**: International Payment Instruction (up to 20 characters)
/// - **RFB**: Reference for Beneficiary (up to 16 characters)
/// - **ROC**: Reference of Customer
/// - **TSU**: Trade Services Utility transaction reference
///
/// ### Format Examples
/// ```logic
/// :70:/INV/20231215/INV-12345/Payment for goods
/// :70:/RFB/PAY-REF-789456
/// :70:/ROC/Customer order 123456
/// :70:PAYMENT FOR SERVICES RENDERED
/// ```
///
/// ## STP Processing Requirements
/// ### ISO 11649 Creditor Reference
/// - Must appear alone on first line
/// - Format: Creditor Reference (up to 25 characters)
/// - When present, enables full STP processing
/// - Must not be combined with other information on same line
///
/// ### Structured References
/// - Invoice references enable automated reconciliation
/// - Payment references support automated matching
/// - Customer references facilitate transaction identification
///
/// ## Regional Considerations
/// - **European Payments**: ISO 11649 reference standards for SEPA
/// - **US Payments**: ACH addenda record mappings and requirements
/// - **Asian Markets**: Local language considerations for beneficiary details
/// - **Cross-Border**: Multi-language and character set requirements
///
/// ## Error Prevention Guidelines
/// - **Character Validation**: Verify all characters are SWIFT-compliant
/// - **Line Length**: Ensure no line exceeds 35 characters
/// - **Reference Formats**: Validate structured reference formats
/// - **STP Compliance**: Verify ISO 11649 reference format when used
///
/// ## Related Fields Integration
/// - **Field 20**: Sender's Reference (transaction identification)
/// - **Field 21**: Related Reference (linked transaction reference)
/// - **Field 59**: Beneficiary Customer (recipient details)
/// - **Field 72**: Sender to Receiver Information (bank-to-bank info)
///
/// ## Compliance Framework
/// - **Regulatory Reporting**: Transaction purpose for compliance monitoring
/// - **Anti-Money Laundering**: Payment purpose verification requirements
/// - **Tax Reporting**: Invoice and payment reference documentation
/// - **Audit Requirements**: Complete transaction documentation standards
///
/// ## Best Practices
/// - **Clear References**: Use unambiguous invoice/payment references
/// - **Structured Codes**: Prefer structured codes for STP processing
/// - **Concise Information**: Keep descriptions clear and brief
/// - **Character Compliance**: Use only SWIFT-approved characters
///
/// ## Common Patterns
/// - Invoice payments: /INV/ followed by invoice details
/// - Salary payments: Clear employee/period references
/// - Trade transactions: Documentary credit references
/// - Service payments: Contract or service agreement references
///
/// ## See Also
/// - Swift FIN User Handbook: Remittance Information Field Specifications
/// - ISO 11649: Structured Creditor Reference to Remittance Information
/// - SEPA Implementation Guidelines: Remittance Information Standards
/// - Payment Processing Standards: Beneficiary Information Requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field70 {
    /// Remittance information narrative
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains payment details, invoice references, or other remittance information
    pub narrative: Vec<String>,
}

impl SwiftField for Field70 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Parse as multiline text (up to 4 lines, 35 chars each)
        let narrative = parse_multiline_text(input, 4, 35)?;

        if narrative.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 70 must have at least one line of narrative".to_string(),
            });
        }

        Ok(Field70 { narrative })
    }

    fn to_swift_string(&self) -> String {
        format!(":70:{}", self.narrative.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field70_single_line() {
        let field = Field70::parse("PAYMENT FOR INVOICE 12345").unwrap();
        assert_eq!(field.narrative.len(), 1);
        assert_eq!(field.narrative[0], "PAYMENT FOR INVOICE 12345");
    }

    #[test]
    fn test_field70_multiline() {
        let input = "/INV/123456\nPAYMENT FOR GOODS\nDELIVERED ON 2024-07-19\nREF: CONTRACT-001";
        let field = Field70::parse(input).unwrap();
        assert_eq!(field.narrative.len(), 4);
        assert_eq!(field.narrative[0], "/INV/123456");
        assert_eq!(field.narrative[1], "PAYMENT FOR GOODS");
        assert_eq!(field.narrative[2], "DELIVERED ON 2024-07-19");
        assert_eq!(field.narrative[3], "REF: CONTRACT-001");
    }

    #[test]
    fn test_field70_to_swift_string() {
        let field = Field70 {
            narrative: vec!["LINE ONE".to_string(), "LINE TWO".to_string()],
        };
        assert_eq!(field.to_swift_string(), ":70:LINE ONE\nLINE TWO");
    }

    #[test]
    fn test_field70_max_lines() {
        let input = "LINE1\nLINE2\nLINE3\nLINE4";
        let field = Field70::parse(input).unwrap();
        assert_eq!(field.narrative.len(), 4);

        // Test that 5 lines would fail
        let too_many = "LINE1\nLINE2\nLINE3\nLINE4\nLINE5";
        assert!(Field70::parse(too_many).is_err());
    }

    #[test]
    fn test_field70_line_length() {
        // Test max length line (35 chars)
        let max_line = "A".repeat(35);
        let field = Field70::parse(&max_line).unwrap();
        assert_eq!(field.narrative[0].len(), 35);

        // Test too long line (36 chars)
        let too_long = "A".repeat(36);
        assert!(Field70::parse(&too_long).is_err());
    }
}
