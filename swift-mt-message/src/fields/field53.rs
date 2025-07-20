use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 53: Sender's Correspondent**
///
/// ## Purpose
/// Specifies the account or branch of the Sender through which reimbursement will occur
/// in correspondent banking arrangements. This field defines the reimbursement path between
/// the Sender and Receiver, enabling proper settlement coordination in cross-border payments.
/// Critical for establishing clear correspondent banking relationships and settlement flows.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured correspondent identification
/// - **Option B**: Party identifier with location - domestic correspondent routing
/// - **Option D**: Party identifier with name/address - detailed correspondent information
///
/// ## Business Context Applications
/// - **Reimbursement Routing**: Defines settlement path between correspondent banks
/// - **Nostro Account Management**: Specifies accounts used for correspondent settlements
/// - **Currency Settlement**: Enables currency-specific correspondent arrangements
/// - **Cross-Border Payments**: Essential for international payment routing and settlement
///
/// ## Usage Rules and Conditions
/// - **Conditional Presence**: Required when no direct account relationship exists (Rule C4)
/// - **Direct Relationships**: Omitted when unique bilateral account relationship exists
/// - **Multiple Accounts**: Option B with party identifier when multiple accounts exist
/// - **Cover Messages**: May trigger MT 202 COV requirement for certain configurations
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Account Validity**: Party identifiers must reference valid correspondent accounts
/// - **Reachability**: Correspondent institutions must be operational and reachable
/// - **Currency Support**: Correspondents must support transaction currency
///
/// ## Correspondent Banking Logic
/// ### Direct Account Relationships
/// - **Unique Accounts**: When only one account exists, field may be omitted
/// - **Bilateral Agreements**: Pre-agreed account arrangements remove field requirement
/// - **Standard Currencies**: Common currency pairs with established relationships
///
/// ### Multiple Account Scenarios
/// - **Currency-Specific**: Different correspondents for different currencies
/// - **Service-Specific**: Specialized correspondents for different services
/// - **Geographic**: Regional correspondents for specific market coverage
/// - **Risk Management**: Diversified correspondent relationships for risk mitigation
///
/// ## Cover Message Requirements
/// - **Non-Receiver Branch**: Field 53A containing non-Receiver branch triggers cover message
/// - **MT 202 COV**: Cover payment message required for certain correspondent configurations
/// - **Settlement Coordination**: Ensures proper settlement through correspondent network
/// - **Regulatory Compliance**: Meets regulatory requirements for payment transparency
///
/// ## Regional Considerations
/// - **European Networks**: TARGET2 correspondent arrangements and SEPA integration
/// - **US Systems**: Fedwire correspondent relationships and dollar clearing
/// - **Asian Markets**: Regional correspondent networks and local currency clearing
/// - **Cross-Border**: Multi-currency correspondent arrangements and settlement
///
/// ## Risk Management Applications
/// - **Counterparty Risk**: Correspondent bank risk assessment and management
/// - **Settlement Risk**: Mitigation through established correspondent relationships
/// - **Operational Risk**: Backup correspondent arrangements for business continuity
/// - **Regulatory Risk**: Compliance with correspondent banking regulations
///
/// ## STP Processing Benefits
/// - **Automated Routing**: System-driven correspondent routing based on clear identification
/// - **Settlement Efficiency**: Streamlined settlement through established relationships
/// - **Exception Reduction**: Proper correspondent identification reduces processing delays
/// - **Straight-Through Processing**: Enhanced STP rates through structured correspondent data
///
/// ## Error Prevention Guidelines
/// - **Relationship Verification**: Confirm correspondent relationships are active
/// - **Account Validation**: Verify correspondent accounts are operational
/// - **Currency Checking**: Ensure correspondent supports transaction currency
/// - **Format Compliance**: Exact adherence to option format requirements
///
/// ## Related Fields Integration
/// - **Field 52A**: Ordering Institution (institutional hierarchy)
/// - **Field 54A**: Receiver's Correspondent (settlement coordination)
/// - **Field 57A**: Account With Institution (final delivery arrangement)
/// - **Field 32A**: Value Date, Currency, Amount (settlement details)
///
/// ## Compliance Framework
/// - **Correspondent Due Diligence**: Enhanced due diligence on correspondent relationships
/// - **Regulatory Reporting**: Correspondent banking relationship reporting requirements
/// - **AML Compliance**: Anti-money laundering considerations in correspondent banking
/// - **Sanctions Screening**: Correspondent bank sanctions screening requirements
///
/// ## Settlement Coordination
/// - **Nostro Management**: Coordination with nostro account balances and limits
/// - **Value Dating**: Alignment with correspondent value dating practices
/// - **Cut-off Times**: Coordination with correspondent processing cut-offs
/// - **Holiday Calendars**: Consideration of correspondent market holidays
///
/// ## See Also
/// - Swift FIN User Handbook: Sender's Correspondent Specifications
/// - Correspondent Banking Guidelines: Relationship Management Standards
/// - Settlement Systems: Cross-Border Settlement Mechanisms
/// - Risk Management: Correspondent Banking Risk Assessment
///
///   **Field 53A: Sender's Correspondent (BIC with Party Identifier)**
///
/// Structured correspondent identification using BIC code with optional party identifier.
/// Preferred option for automated correspondent banking processing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field53A {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for nostro account identification and correspondent routing
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the sender's correspondent
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

///   **Field 53B: Sender's Correspondent (Party Identifier with Location)**
///
/// Domestic correspondent routing using party identifier and location details.
/// Used when multiple correspondent accounts exist and location-based routing is required.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field53B {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for nostro account identification when multiple accounts exist
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Location information for correspondent routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based correspondent routing within domestic systems
    #[component("[35x]")]
    pub location: Option<String>,
}

///   **Field 53D: Sender's Correspondent (Party Identifier with Name and Address)**
///
/// Detailed correspondent identification with full name and address information.
/// Used when structured BIC identification is not available for correspondent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field53D {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for nostro account identification and correspondent routing
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address of the sender's correspondent
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains correspondent name, address, city, country details
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field53SenderCorrespondent {
    A(Field53A),
    B(Field53B),
    D(Field53D),
}
