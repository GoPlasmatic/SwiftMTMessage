use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 86: Information to Account Owner**
///
/// ## Purpose
/// Provides additional information to the account owner regarding specific transactions
/// or account activities. This field supplements transaction details in statement messages,
/// offering extended explanations, references, and context that help account holders
/// understand and reconcile their account activities.
///
/// ## Format Specification
/// - **Swift Format**: `6*65x`
/// - **Structure**: Up to 6 lines of 65 characters each
/// - **Content**: Free-form narrative with transaction details
/// - **Character Set**: SWIFT character set with extended line length
///
/// ## Business Context Applications
/// - **Customer Statements**: Additional details in MT 940 Customer Statement
/// - **Transaction Explanation**: Extended transaction descriptions
/// - **Reference Information**: Additional references and codes
/// - **Account Communication**: Important account-related communications
///
/// ## Network Validation Requirements
/// - **Line Length**: Maximum 6 lines of 65 characters each
/// - **Character Set**: Must use valid SWIFT character set
/// - **Content Relevance**: Information should be relevant to account owner
/// - **Format Compliance**: Proper line structure and formatting
///
/// ## Information Categories
/// ### Transaction Details
/// - **Extended Descriptions**: Detailed transaction explanations
/// - **Reference Numbers**: Additional reference numbers and codes
/// - **Counterparty Information**: Details about transaction counterparties
/// - **Purpose Codes**: Transaction purpose and classification codes
///
/// ### Account Information
/// - **Balance Explanations**: Explanations of balance changes
/// - **Fee Descriptions**: Detailed fee and charge descriptions
/// - **Service Information**: Account service notifications
/// - **Regulatory Information**: Compliance-related information
///
/// ## Regional Considerations
/// - **European Banking**: SEPA statement information requirements
/// - **US Banking**: Federal and state banking information standards
/// - **Asian Markets**: Local banking communication requirements
/// - **Cross-Border**: International account information standards
///
/// ## Error Prevention Guidelines
/// - **Relevance Check**: Ensure information is relevant to account owner
/// - **Length Validation**: Confirm content fits within length limits
/// - **Character Validation**: Verify all characters are SWIFT-valid
/// - **Clarity Verification**: Ensure information is clear and understandable
///
/// ## Related Fields Integration
/// - **Field 61**: Statement Line (related transaction details)
/// - **Field 60/62**: Opening/Closing Balance (balance context)
/// - **Field 20**: Transaction Reference (reference coordination)
/// - **Field 25**: Account Identification (account context)
///
/// ## Compliance Framework
/// - **Customer Communication**: Clear and transparent account communication
/// - **Regulatory Requirements**: Meeting account information disclosure requirements
/// - **Consumer Protection**: Adequate information for account holder protection
/// - **Audit Documentation**: Complete account information documentation
///
/// ## See Also
/// - Swift FIN User Handbook: Information to Account Owner Specifications
/// - MT 940 Standards: Customer Statement Information
/// - Banking Communication: Account Holder Information Standards
/// - Customer Protection: Account Information Requirements

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field86 {
    /// Information narrative for account owner
    ///
    /// Format: 6*65x - Up to 6 lines of 65 characters each
    /// Contains additional transaction and account information for account holder
    /// Provides extended details beyond basic transaction information
    #[component("6*65x")]
    pub narrative: Vec<String>,
}
