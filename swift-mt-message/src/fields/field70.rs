use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

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
/// - **Format**: RF followed by 2 check digits and up to 21 alphanumeric characters
/// - **Position**: Must appear alone on first line for STP processing
/// - **Validation**: Check digit validation required
/// - **Usage**: Automated reconciliation and payment processing
///
/// ### Multiple References
/// - **Separation**: Multiple references separated by double slash '//'
/// - **Line Management**: Each line maximum 35 characters
/// - **Continuation**: Long references can span multiple lines
/// - **Priority**: Most important reference should appear first
///
/// ## Regional Considerations
/// - **European Payments**: SEPA remittance information requirements
/// - **US Payments**: ACH and wire transfer reference standards
/// - **Asian Markets**: Local payment reference requirements
/// - **Cross-Border**: International payment reference coordination
///
/// ## Clearing System Requirements
/// - **Length Checking**: Sender must verify length restrictions with receiver
/// - **Format Validation**: Structured codes must be properly formatted
/// - **Character Validation**: All characters must be SWIFT-valid
/// - **Reference Uniqueness**: References should be unique for reconciliation
///
/// ## Error Prevention Guidelines
/// - **Length Validation**: Confirm total length does not exceed limits
/// - **Character Checking**: Verify all characters are SWIFT-valid
/// - **Reference Format**: Ensure structured references follow correct format
/// - **Beneficiary Clarity**: Ensure information is clear for beneficiary
///
/// ## Related Fields Integration
/// - **Field 59**: Beneficiary Customer (recipient of remittance information)
/// - **Field 72**: Sender to Receiver Information (additional instructions)
/// - **Field 77A**: Narrative (extended narrative information)
/// - **Field 50**: Ordering Customer (originator context)
///
/// ## Compliance Framework
/// - **Regulatory Requirements**: Payment reference reporting requirements
/// - **AML Compliance**: Transaction purpose identification for AML screening
/// - **Customer Protection**: Clear payment purpose communication
/// - **Audit Trail**: Complete payment reference documentation
///
/// ## Trade Finance Applications
/// - **Documentary Credits**: Letter of credit references
/// - **Trade Settlements**: Commercial invoice and shipping references
/// - **Supply Chain**: Purchase order and delivery references
/// - **International Trade**: Import/export documentation references
///
/// ## See Also
/// - Swift FIN User Handbook: Remittance Information Specifications
/// - ISO 11649: Creditor Reference Standard
/// - Payment Reference Standards: International Payment References
/// - STP Guidelines: Straight Through Processing Requirements

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field70 {
    /// Remittance information narrative
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains payment details, references, and instructions for beneficiary
    /// May include structured codes (INV, IPI, RFB, ROC, TSU) and ISO 11649 references
    #[component("4*35x")]
    pub narrative: Vec<String>,
}
