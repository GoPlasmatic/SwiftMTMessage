use super::swift_utils::{parse_alphanumeric, parse_exact_length};
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 26T: Transaction Type Code**
///
/// ## Purpose
/// Specifies the type or nature of a financial transaction or instruction using standardized
/// codes. This field enables automatic processing, routing, and categorization of transactions
/// based on their business purpose and regulatory requirements.
///
/// ## Format
/// - **Swift Format**: `3!c`
/// - **Description**: Exactly 3 alphanumeric characters
/// - **Character Set**: Letters and digits following Swift standards
/// - **Code Type**: Standardized transaction type codes
///
/// ## Presence
/// - **Status**: Conditional/Optional depending on message type and business requirements
/// - **Swift Error Codes**: T50 (invalid code), T12 (format violation)
/// - **Usage Context**: Transaction categorization and processing logic
///
/// ## Usage Rules
/// - **Code Validation**: Must be valid standardized transaction type code
/// - **Business Logic**: Determines processing rules and regulatory treatment
/// - **Routing Decisions**: Influences message routing and handling procedures
/// - **Compliance**: Required for certain regulatory reporting and monitoring
///
/// ## Network Validation Rules
/// - **Format Validation**: Must be exactly 3 characters
/// - **Code Registry**: Must be recognized transaction type code
/// - **Character Set**: Only alphanumeric characters permitted
/// - **Business Context**: Code must be appropriate for message type and context
///
/// ## Common Transaction Type Codes
///
/// ### Payment Instructions
/// - **PAY**: Standard payment instruction
/// - **SAL**: Salary payment
/// - **PEN**: Pension payment
/// - **DIV**: Dividend payment
/// - **INT**: Interest payment
/// - **TAX**: Tax payment
/// - **FEE**: Fee payment
///
/// ### Treasury Operations
/// - **FXD**: Foreign exchange deal
/// - **MMD**: Money market deal
/// - **DER**: Derivative transaction
/// - **SEC**: Securities transaction
/// - **COL**: Collateral transaction
/// - **REP**: Repurchase agreement
///
/// ### Trade Finance
/// - **TRD**: Trade transaction
/// - **DOC**: Documentary transaction
/// - **LCR**: Letter of credit
/// - **GUA**: Guarantee
/// - **COL**: Collection
/// - **FIN**: Trade financing
///
/// ### Corporate Actions
/// - **CAP**: Capital payment
/// - **RIG**: Rights issue
/// - **BON**: Bonus issue
/// - **SPL**: Stock split
/// - **MER**: Merger transaction
/// - **ACQ**: Acquisition
///
/// ## Business Context
/// - **Transaction Classification**: Systematic categorization of financial transactions
/// - **Regulatory Compliance**: Meeting reporting requirements for different transaction types
/// - **Processing Automation**: Enabling automated routing and handling based on type
/// - **Risk Management**: Transaction type-based risk assessment and controls
///
/// ## Examples
/// ```logic
/// :26T:PAY    // Standard payment instruction
/// :26T:SAL    // Salary payment
/// :26T:FXD    // Foreign exchange deal
/// :26T:DIV    // Dividend payment
/// :26T:TRD    // Trade transaction
/// :26T:INT    // Interest payment
/// ```
///
/// ## Regional Considerations
/// - **European Markets**: SEPA transaction type requirements
/// - **US Markets**: ACH and Fedwire transaction classifications
/// - **Asian Markets**: Local payment type categorizations
/// - **Cross-Border**: International transaction type harmonization
///
/// ## Regulatory Impact
/// - **Reporting Requirements**: Different codes trigger specific reporting obligations
/// - **Monitoring Systems**: Enables automated transaction monitoring and analysis
/// - **Compliance Checks**: Type-specific compliance validation rules
/// - **Audit Trails**: Enhanced transaction tracking by business purpose
///
/// ## Error Prevention
/// - **Code Validation**: Verify transaction type code is valid and recognized
/// - **Context Checking**: Ensure code is appropriate for message type and business context
/// - **Format Verification**: Confirm exactly 3 character format requirement
/// - **Business Logic**: Validate code aligns with transaction purpose
///
/// ## Related Fields
/// - **Field 23**: Instruction Code (additional transaction instructions)
/// - **Field 70**: Remittance Information (transaction description)
/// - **Field 72**: Sender to Receiver Information (additional details)
/// - **Block Headers**: Message type in application header
///
/// ## Processing Impact
/// - **Routing Logic**: Determines appropriate processing channels
/// - **Validation Rules**: Triggers specific validation requirements
/// - **STP Processing**: Enables automated straight-through processing
/// - **Exception Handling**: Type-specific exception processing procedures
///
/// ## Compliance Framework
/// - **AML Monitoring**: Enhanced monitoring for high-risk transaction types
/// - **Sanctions Screening**: Type-specific sanctions checking requirements
/// - **Regulatory Reporting**: Automated reporting based on transaction type
/// - **Documentation**: Type-specific documentation and record-keeping requirements
///
/// ## STP Compliance
/// - **Code Standardization**: Consistent transaction type coding for automation
/// - **Processing Rules**: Type-based automated processing decisions
/// - **Quality Control**: Enhanced validation for specific transaction types
/// - **Exception Management**: Automated handling of type-specific exceptions
///
/// ## See Also
/// - Swift Standards: Transaction Type Code Registry
/// - FIN User Handbook: Transaction Classification Guidelines
/// - Regulatory Guidelines: Transaction Type Reporting Requirements
/// - Processing Manuals: Type-Based Transaction Handling
///
///   **Field 26T: Transaction Type Code Structure**
///
/// Contains the 3-character transaction type code for categorizing
/// and processing financial transactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field26T {
    /// Transaction type code
    ///
    /// Format: 3!c - Exactly 3 alphanumeric characters
    /// Must be valid standardized transaction type code (PAY, SAL, FXD, etc.)
    /// Determines transaction processing rules and regulatory treatment
    pub type_code: String,
}

impl SwiftField for Field26T {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Must be exactly 3 characters
        let type_code = parse_exact_length(input, 3, "Field 26T type code")?;

        // Must be alphanumeric
        parse_alphanumeric(&type_code, "Field 26T type code")?;

        Ok(Field26T { type_code })
    }

    fn to_swift_string(&self) -> String {
        format!(":26T:{}", self.type_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field26t_valid() {
        let field = Field26T::parse("PAY").unwrap();
        assert_eq!(field.type_code, "PAY");
        assert_eq!(field.to_swift_string(), ":26T:PAY");

        let field = Field26T::parse("FXD").unwrap();
        assert_eq!(field.type_code, "FXD");

        let field = Field26T::parse("123").unwrap();
        assert_eq!(field.type_code, "123");

        let field = Field26T::parse("A1B").unwrap();
        assert_eq!(field.type_code, "A1B");
    }

    #[test]
    fn test_field26t_invalid() {
        // Too short
        assert!(Field26T::parse("PA").is_err());

        // Too long
        assert!(Field26T::parse("PAYM").is_err());

        // Non-alphanumeric characters
        assert!(Field26T::parse("PA-").is_err());
        assert!(Field26T::parse("P@Y").is_err());
    }
}
