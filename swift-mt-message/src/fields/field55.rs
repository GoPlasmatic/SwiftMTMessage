use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 55: Third Reimbursement Institution**
///
/// ## Purpose
/// Specifies the Receiver's branch when funds are made available through a different
/// institution than specified in Field 53A (Sender's Correspondent). This field enables
/// complex reimbursement chains involving multiple institutions, typically used when
/// the fund availability institution differs from the primary correspondent relationship.
/// Essential for sophisticated correspondent banking arrangements.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured third institution identification
/// - **Option B**: Party identifier with location - domestic third institution routing
/// - **Option D**: Party identifier with name/address - detailed third institution information
///
/// ## Business Context Applications
/// - **Complex Reimbursement**: Multi-institution settlement chains requiring third party
/// - **Branch Specification**: Receiver's branch when funds available through intermediary
/// - **Settlement Optimization**: Efficient routing through specialized institutions
/// - **Regional Networks**: Local institution integration in cross-border payments
///
/// ## Usage Rules and Conditions
/// - **Conditional Presence**: Optional field referenced in Rule C4 correspondent logic
/// - **Field 54A Dependency**: Typically used when Field 54A contains non-Receiver institution
/// - **Receiver Branch**: Usually contains Receiver's branch in complex chains
/// - **Settlement Finalization**: Represents final settlement point for funds availability
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Institution Capability**: Must provide reimbursement and settlement services
/// - **Operational Status**: Institutions must be operational and reachable
/// - **Currency Support**: Must support transaction currency and settlement requirements
///
/// ## Complex Settlement Scenarios
/// ### Multi-Institution Chains
/// - **Field 53A**: Primary correspondent relationship (Sender side)
/// - **Field 54A**: Intermediary institution for fund availability
/// - **Field 55A**: Final settlement institution (typically Receiver's branch)
/// - **Settlement Flow**: Funds flow through multiple institutions to reach final destination
///
/// ### Regional Integration
/// - **Local Presence**: Integration with local banking networks
/// - **Regulatory Compliance**: Meeting local settlement requirements
/// - **Currency Optimization**: Efficient local currency settlement
/// - **Service Specialization**: Leveraging specialized institution capabilities
///
/// ## Risk Management Applications
/// - **Settlement Risk**: Distribution of settlement risk across multiple institutions
/// - **Operational Risk**: Redundancy and backup settlement paths
/// - **Counterparty Risk**: Diversification of counterparty exposure
/// - **Liquidity Management**: Optimization of liquidity across correspondent network
///
/// ## Regional Considerations
/// - **European Networks**: TARGET2 integration and Euro settlement optimization
/// - **US Systems**: Federal Reserve and commercial bank integration
/// - **Asian Markets**: Regional banking network integration and local settlement
/// - **Emerging Markets**: Local institution integration for regulatory compliance
///
/// ## STP Processing Benefits
/// - **Chain Automation**: Automated processing of complex settlement chains
/// - **Exception Handling**: Structured handling of multi-institution scenarios
/// - **Settlement Optimization**: Efficient routing through multiple institutions
/// - **Risk Distribution**: Automated risk assessment across institution chain
///
/// ## Error Prevention Guidelines
/// - **Chain Validation**: Verify complete settlement chain is operational
/// - **Institution Verification**: Confirm all institutions can provide required services
/// - **Relationship Checking**: Validate relationships between all chain participants
/// - **Currency Support**: Ensure all institutions support transaction currency
///
/// ## Related Fields Integration
/// - **Field 53A**: Sender's Correspondent (settlement chain initiation)
/// - **Field 54A**: Receiver's Correspondent (intermediate settlement)
/// - **Field 57A**: Account With Institution (final beneficiary bank)
/// - **Field 32A**: Value Date, Currency, Amount (settlement details)
///
/// ## Compliance Framework
/// - **Multi-Institution Due Diligence**: Enhanced due diligence across institution chain
/// - **Regulatory Coordination**: Compliance across multiple regulatory jurisdictions
/// - **Audit Trail**: Complete documentation of multi-institution settlement path
/// - **Risk Assessment**: Comprehensive risk evaluation across institution chain
///
/// ## Settlement Coordination
/// - **Value Date Alignment**: Coordination of value dates across multiple institutions
/// - **Cut-off Management**: Alignment with multiple institution processing cut-offs
/// - **Holiday Coordination**: Management of multiple market holiday calendars
/// - **Liquidity Planning**: Coordination of liquidity across correspondent network
///
/// ## See Also
/// - Swift FIN User Handbook: Third Reimbursement Institution Specifications
/// - Correspondent Banking Guidelines: Complex Settlement Chain Management
/// - Multi-Institution Settlement: Risk and Operational Considerations
/// - Cross-Border Payments: Advanced Correspondent Banking Arrangements

/// **Field 55A: Third Reimbursement Institution (BIC with Party Identifier)**
///
/// Structured third institution identification using BIC code with optional party identifier.
/// Used for complex correspondent banking chains requiring additional institutional routing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field55A {
    /// Optional party identifier for third institution account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for specialized account identification in complex settlement chains
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the third reimbursement institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

/// **Field 55B: Third Reimbursement Institution (Party Identifier with Location)**
///
/// Domestic third institution routing using party identifier and location details.
/// Used for location-based routing in complex domestic settlement arrangements.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field55B {
    /// Optional party identifier for third institution account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for specialized routing in complex settlement chains
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Location information for third institution routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based routing in complex correspondent networks
    #[component("[35x]")]
    pub location: Option<String>,
}

/// **Field 55D: Third Reimbursement Institution (Party Identifier with Name and Address)**
///
/// Detailed third institution identification with full name and address information.
/// Used when structured BIC identification is not available for third institution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field55D {
    /// Optional party identifier for third institution account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for specialized routing in complex settlement chains
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address of the third reimbursement institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field55ThirdReimbursementInstitution {
    A(Field55A),
    B(Field55B),
    D(Field55D),
}
