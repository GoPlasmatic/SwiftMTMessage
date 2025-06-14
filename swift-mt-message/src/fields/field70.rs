use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// # Field 70: Remittance Information
///
/// ## Overview
/// Field 70 contains remittance information in SWIFT payment messages, providing details about
/// the purpose and context of the payment. This field allows the ordering customer to include
/// relevant information that helps the beneficiary identify and process the payment, such as
/// invoice numbers, contract references, or payment descriptions. The remittance information
/// is crucial for payment reconciliation and business process automation.
///
/// ## Format Specification
/// **Format**: `4*35x`
/// - **4*35x**: Up to 4 lines of 35 characters each
/// - **Line structure**: Free-form text for remittance details
/// - **Character set**: SWIFT character set (A-Z, 0-9, and limited special characters)
/// - **Line separation**: Each line on separate row
///
/// ## Structure
/// ```text
/// PAYMENT FOR INVOICE INV-2024-001234
/// CONTRACT REF: SUPPLY-AGREEMENT-2024
/// DELIVERY DATE: 15-MAR-2024
/// NET 30 DAYS PAYMENT TERMS
/// │                              │
/// └──────────────────────────────┘
///        Up to 35 characters per line
///        Maximum 4 lines
/// ```
///
/// ## Field Components
/// - **Invoice References**: Invoice numbers and billing details
/// - **Contract Information**: Contract numbers and references
/// - **Payment Purpose**: Description of goods or services
/// - **Additional Details**: Delivery dates, terms, or special instructions
///
/// ## Usage Context
/// Field 70 is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Payment reconciliation**: Matching payments to invoices
/// - **Accounts receivable**: Automated payment processing
/// - **Compliance reporting**: Supporting audit trails
/// - **Business process automation**: Enabling straight-through processing
/// - **Customer communication**: Providing payment context
/// - **Dispute resolution**: Supporting payment inquiries
///
/// ## Examples
/// ```text
/// :70:PAYMENT FOR INVOICE 12345
/// └─── Simple invoice payment reference
///
/// :70:INVOICE INV-2024-001234
/// CONTRACT SUPPLY-AGREEMENT-2024
/// DELIVERY 15-MAR-2024
/// NET 30 PAYMENT TERMS
/// └─── Detailed commercial payment information
///
/// :70:SALARY PAYMENT MARCH 2024
/// EMPLOYEE ID: EMP-789012
/// PAYROLL REF: PR-2024-03
/// └─── Payroll payment details
///
/// :70:TRADE FINANCE SETTLEMENT
/// LC NUMBER: LC-2024-567890
/// DOCUMENTS COMPLIANT
/// PAYMENT AS PER LC TERMS
/// └─── Trade finance payment reference
/// ```
///
/// ## Remittance Information Types
/// - **Commercial payments**: Invoice and purchase order references
/// - **Trade finance**: Letter of credit and documentary collection details
/// - **Payroll**: Salary and benefit payment information
/// - **Tax payments**: Tax reference numbers and periods
/// - **Loan payments**: Loan account and installment details
/// - **Utility payments**: Account numbers and billing periods
/// - **Insurance**: Policy numbers and coverage details
///
/// ## Validation Rules
/// 1. **Line count**: Maximum 4 lines
/// 2. **Line length**: Maximum 35 characters per line
/// 3. **Character set**: SWIFT character set only
/// 4. **Content**: Should contain meaningful remittance information
/// 5. **Empty lines**: Generally avoided for clarity
/// 6. **Control characters**: Not allowed
/// 7. **Special characters**: Limited to SWIFT-approved set
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Maximum 4 lines allowed (Error: T26)
/// - Each line maximum 35 characters (Error: T50)
/// - Must use SWIFT character set only (Error: T61)
/// - Content should be meaningful (Error: T40)
/// - No control characters permitted (Error: T35)
/// - Field is optional but recommended (Warning: W70)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("4*35x")]
pub struct Field70 {
    /// Remittance information lines (up to 4 lines of 35 characters each)
    #[format("4*35x")]
    pub information: Vec<String>,
}

impl Field70 {
    pub fn new(information: Vec<String>) -> Self {
        Self { information }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field70_creation() {
        let info = vec!["PAYMENT FOR INVOICE 12345".to_string()];
        let field70 = Field70::new(info.clone());
        assert_eq!(field70.information, info);
    }
}
