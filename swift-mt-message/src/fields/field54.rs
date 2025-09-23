use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 54: Receiver's Correspondent**
///
/// ## Purpose
/// Specifies the branch of the Receiver or financial institution where funds will be made
/// available in correspondent banking arrangements. This field defines the receiving end
/// of the correspondent relationship, indicating where funds become available to the Receiver
/// or through which institution final settlement occurs. Essential for completion of
/// correspondent banking settlement chains.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured receiver correspondent identification
/// - **Option B**: Party identifier with location - domestic receiver correspondent routing
/// - **Option D**: Party identifier with name/address - detailed receiver correspondent information
///
/// ## Business Context Applications
/// - **Fund Availability**: Defines where funds become available to the Receiver
/// - **Settlement Completion**: Final link in correspondent banking settlement chain
/// - **Branch Specification**: Identifies specific Receiver branch for fund availability
/// - **Intermediary Institution**: Non-Receiver institution providing fund availability
///
/// ## Usage Rules and Conditions
/// - **Conditional Presence**: Required based on Rule C4 correspondent banking logic
/// - **Receiver Branch**: Can specify Receiver's branch for fund availability
/// - **Intermediary Usage**: May reference institution other than Receiver
/// - **Reimbursement Claims**: Defines reimbursement path when used with Receiver branch
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Institution Validity**: Referenced institutions must be operational and reachable
/// - **Service Capability**: Institutions must provide correspondent services
/// - **Currency Support**: Must support transaction currency and settlement
///
/// ## Correspondent Banking Integration
/// - **Field 53A Coordination**: Works with Sender's Correspondent for complete settlement path
/// - **Field 55A Usage**: Triggers Field 55A when funds available through different institution
/// - **Direct Relationships**: Enables direct settlement when Receiver branch specified
/// - **Cover Message Avoidance**: Proper usage can eliminate need for cover messages
///
/// ## Regional Considerations
/// - **European Networks**: TARGET2 correspondent arrangements and Euro settlement
/// - **US Systems**: Fedwire correspondent relationships and USD clearing
/// - **Asian Markets**: Regional correspondent networks and local currency settlement
/// - **Cross-Border**: Multi-currency correspondent arrangements and final settlement
///
/// ## Error Prevention Guidelines
/// - **Relationship Verification**: Confirm correspondent relationships are active
/// - **Institution Validation**: Verify referenced institutions can provide services
/// - **Currency Checking**: Ensure institution supports transaction currency
/// - **Chain Validation**: Verify complete correspondent chain is operational
///
/// ## Related Fields Integration
/// - **Field 53A**: Sender's Correspondent (settlement chain coordination)
/// - **Field 55A**: Third Reimbursement Institution (complex routing scenarios)
/// - **Field 57A**: Account With Institution (final beneficiary bank)
/// - **Field 32A**: Value Date, Currency, Amount (settlement details)
///
/// ## STP Processing Benefits
/// - **Automated Settlement**: System-driven correspondent settlement routing
/// - **Chain Optimization**: Efficient correspondent banking chain processing
/// - **Exception Reduction**: Proper correspondent identification reduces delays
/// - **Straight-Through Processing**: Enhanced STP through structured correspondent data
///
/// ## See Also
/// - Swift FIN User Handbook: Receiver's Correspondent Specifications
/// - Correspondent Banking Guidelines: Settlement Chain Management
/// - Cross-Border Payments: Correspondent Banking Settlement
/// - Risk Management: Correspondent Banking Risk Assessment
///
///   **Field 54A: Receiver's Correspondent (BIC with Party Identifier)**
///
/// Structured receiver correspondent identification using BIC code with optional party identifier.
/// Preferred option for automated correspondent banking processing on the receiving end.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field54A {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier  
    /// Used for vostro account identification and receiver correspondent routing
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the receiver's correspondent
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

///   **Field 54B: Receiver's Correspondent (Party Identifier with Location)**
///
/// Domestic receiver correspondent routing using party identifier and location details.
/// Used for location-based routing in domestic correspondent arrangements.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field54B {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for vostro account identification in domestic systems
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Location information for receiver correspondent routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based routing within domestic correspondent networks
    #[component("[35x]")]
    pub location: Option<String>,
}

///   **Field 54D: Receiver's Correspondent (Party Identifier with Name and Address)**
///
/// Detailed receiver correspondent identification with full name and address information.
/// Used when structured BIC identification is not available for receiver correspondent.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field54D {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for vostro account identification and receiver correspondent routing
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address of the receiver's correspondent
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains correspondent name, address, city, country details
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field54ReceiverCorrespondent {
    A(Field54A),
    B(Field54B),
    D(Field54D),
}
