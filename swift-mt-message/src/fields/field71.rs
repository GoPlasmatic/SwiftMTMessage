use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 71: Charges and Fee Information**
///
/// ## Purpose
/// Specifies charge allocation and fee details for payment transactions. This field family
/// determines which party bears transaction costs and provides detailed charge amounts
/// for various fees associated with payment processing. Essential for transparent
/// cost allocation and compliance with payment regulations.
///
/// ## Field Options Overview
/// - **Field 71A**: Details of Charges (charge allocation code)
/// - **Field 71F**: Sender's Charges (specific charge amounts)
/// - **Field 71G**: Receiver's Charges (additional charge amounts)
///
/// ## Business Context Applications
/// - **Payment Processing**: Charge allocation in MT 103 and other payment messages
/// - **Cost Transparency**: Clear identification of transaction costs
/// - **Regulatory Compliance**: Meeting charge disclosure requirements
/// - **Customer Communication**: Transparent fee structure communication
///
/// ## Charge Allocation Principles
/// ### Allocation Options (Field 71A)
/// - **BEN**: Beneficiary bears all charges
/// - **OUR**: Ordering customer bears all charges
/// - **SHA**: Shared charges (sender pays own bank, beneficiary pays others)
///
/// ### Charge Types
/// - **Correspondent Charges**: Fees charged by intermediary banks
/// - **Beneficiary Bank Charges**: Fees charged by receiving bank
/// - **Service Charges**: Additional service fees
/// - **Conversion Charges**: Currency conversion fees
///
/// ## Regional Considerations
/// - **European Payments**: SEPA charge regulations and transparency requirements
/// - **US Payments**: Federal Reserve and commercial bank fee structures
/// - **Asian Markets**: Local charge allocation practices
/// - **Cross-Border**: International payment fee coordination
///
/// ## Error Prevention Guidelines
/// - **Code Validation**: Verify charge allocation codes are valid
/// - **Amount Verification**: Confirm charge amounts are reasonable
/// - **Currency Consistency**: Ensure charge currency matches context
/// - **Disclosure Compliance**: Meet regulatory charge disclosure requirements
///
/// ## Related Fields Integration
/// - **Field 32A**: Value Date, Currency, Amount (transaction amount context)
/// - **Field 33B**: Currency/Instructed Amount (original amount before charges)
/// - **Field 72**: Sender to Receiver Information (charge instructions)
/// - **Field 64**: Closing Available Balance (net amount after charges)
///
/// ## Compliance Framework
/// - **Regulatory Requirements**: Charge transparency and disclosure regulations
/// - **Consumer Protection**: Clear charge communication requirements
/// - **Fee Regulation**: Compliance with local fee regulation standards
/// - **Audit Documentation**: Complete charge allocation documentation
///
/// ## See Also
/// - Swift FIN User Handbook: Charge Field Specifications
/// - Payment Regulations: Charge Transparency Requirements
/// - Banking Fee Standards: International Charge Allocation
/// - Customer Protection: Charge Disclosure Guidelines
///
///   **Field 71A: Details of Charges**
///
/// Specifies which party will bear the charges for the transaction.
/// Mandatory field in payment messages for charge allocation transparency.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field71A {
    /// Charge allocation code
    ///
    /// Format: 3!a - Three alphabetic characters
    /// Values: BEN (Beneficiary), OUR (Ordering customer), SHA (Shared)
    /// Error T08 if invalid code used
    #[component("3!a")]
    pub code: String,
}

///   **Field 71F: Sender's Charges**
///
/// Specifies the currency and amount of charges to be borne by the sender.
/// Used to detail specific charge amounts in sender's currency.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field71F {
    /// Currency of sender's charges
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must be valid currency for charge specification
    #[component("3!a")]
    pub currency: String,

    /// Amount of sender's charges
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Precision must match currency requirements
    #[component("15d")]
    pub amount: f64,
}

///   **Field 71G: Receiver's Charges**
///
/// Specifies the currency and amount of charges to be borne by the receiver.
/// Used to detail specific charge amounts in receiver's currency.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field71G {
    /// Currency of receiver's charges
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must be valid currency for charge specification
    #[component("3!a")]
    pub currency: String,

    /// Amount of receiver's charges
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Represents additional charges for receiver
    #[component("15d")]
    pub amount: f64,
}

///   **Field 71B: Details of Charges**
///
/// Specifies detailed information about charges, interest and other adjustments.
/// Used in MT n90 messages (MT190, MT290, etc.) to provide comprehensive charge details.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field71B {
    /// Details of charges
    ///
    /// Format: 6*35x - Up to 6 lines of 35 characters each
    /// Contains detailed breakdown of charges, interest and other adjustments
    #[component("6*35x")]
    pub details: Vec<String>,
}
