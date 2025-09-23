use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 56: Intermediary**
///
/// ## Purpose
/// Specifies the financial institution through which the transaction must pass to reach
/// the account with institution (Field 57A). This field defines the routing path for
/// payment instructions, enabling proper routing through intermediary banks when direct
/// relationships do not exist. Critical for straight-through processing and automated
/// payment routing in complex banking networks.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured intermediary identification
/// - **Option C**: Party identifier only - simplified intermediary reference
/// - **Option D**: Party identifier with name/address - detailed intermediary information
///
/// ## Business Context Applications
/// - **Payment Routing**: Defines intermediary path to reach beneficiary's bank
/// - **Correspondent Networks**: Utilizes existing correspondent banking relationships
/// - **Cross-Border Payments**: Essential for international payment routing
/// - **Domestic Clearing**: Integration with national payment systems
///
/// ## Special Payment Method Codes
/// ### Critical Routing Instructions
/// - **//FW**: Fedwire routing - Required by US banks for Fedwire processing
/// - **//RT**: Real-Time Gross Settlement - Binding instruction for RTGS systems
/// - **//AU**: Australian payment system routing
/// - **//IN**: Indian payment system routing
///
/// ### Code Usage Rules
/// - **Single Usage**: Codes //FW, //AU, //IN, //RT should appear only once in Field 56A or 57A
/// - **Binding Nature**: //RT code is binding and cannot be followed by other information
/// - **System Integration**: Enables automated processing in national clearing systems
/// - **Precedence**: Special codes take precedence over standard routing information
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Routing Capability**: Intermediaries must provide routing services to destination
/// - **System Support**: Must support specified payment method codes
/// - **Currency Capability**: Must handle transaction currency and settlement
///
/// ## Routing Logic and Rules
/// ### Direct vs. Intermediary Routing
/// - **Direct Access**: When Sender has direct relationship with Account With Institution
/// - **Intermediary Required**: When direct relationship does not exist or is inefficient
/// - **Multi-Hop Routing**: Complex routing through multiple intermediaries
/// - **Optimization**: Selection of most efficient routing path
///
/// ### Payment System Integration
/// - **RTGS Systems**: Real-time gross settlement system routing
/// - **ACH Networks**: Automated clearing house routing
/// - **Wire Networks**: Wire transfer system routing
/// - **Clearing Systems**: National and regional clearing system integration
///
/// ## Regional Payment System Support
/// ### North American Systems
/// - **Fedwire (//FW)**: US Federal Reserve wire transfer system
/// - **ACH**: Automated Clearing House networks
/// - **Canadian Systems**: Integration with Canadian payment networks
///
/// ### Asia-Pacific Systems
/// - **Australian (//AU)**: Australian payment system integration
/// - **Indian (//IN)**: Indian payment system routing
/// - **Regional Networks**: ASEAN and other regional payment systems
///
/// ### European Systems
/// - **TARGET2**: European Central Bank RTGS system
/// - **SEPA**: Single Euro Payments Area routing
/// - **National RTGS**: Country-specific RTGS systems
///
/// ## STP Processing Benefits
/// - **Automated Routing**: System-driven routing based on intermediary identification
/// - **Exception Reduction**: Proper routing reduces payment exceptions and delays
/// - **Straight-Through Processing**: Enhanced STP through structured routing data
/// - **System Integration**: Seamless integration with payment system networks
///
/// ## Error Prevention Guidelines
/// - **Routing Validation**: Verify intermediary can route to destination
/// - **System Compatibility**: Confirm intermediary supports required payment systems
/// - **Code Verification**: Validate special payment method codes are appropriate
/// - **Relationship Checking**: Verify Sender has relationship with intermediary
///
/// ## Related Fields Integration
/// - **Field 57A**: Account With Institution (routing destination)
/// - **Field 53A**: Sender's Correspondent (routing coordination)
/// - **Field 32A**: Value Date, Currency, Amount (routing context)
/// - **Field 72**: Sender to Receiver Information (routing instructions)
///
/// ## Compliance Framework
/// - **Routing Documentation**: Complete routing path documentation
/// - **Intermediary Due Diligence**: Enhanced due diligence on routing intermediaries
/// - **Regulatory Compliance**: Meeting routing and settlement regulations
/// - **Audit Trail**: Comprehensive routing audit trail maintenance
///
/// ## Performance Optimization
/// - **Routing Efficiency**: Selection of optimal routing paths
/// - **Cost Management**: Consideration of routing costs and fees
/// - **Speed Optimization**: Fastest routing path selection
/// - **Reliability**: Most reliable routing path selection
///
/// ## See Also
/// - Swift FIN User Handbook: Intermediary Institution Specifications
/// - Payment System Guides: National Payment System Routing
/// - Correspondent Banking: Intermediary Routing Arrangements
/// - Cross-Border Payments: International Routing Standards
///
///   **Field 56A: Intermediary (BIC with Party Identifier)**
///
/// Structured intermediary identification using BIC code with optional party identifier.
/// Preferred option for automated payment routing and system integration.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field56A {
    /// Optional party identifier for routing and payment method codes
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// May contain special codes: //FW (Fedwire), //RT (RTGS), //AU (Australian), //IN (Indian)
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the intermediary institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution with routing capability
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

///   **Field 56C: Intermediary (Party Identifier Only)**
///
/// Simplified intermediary reference using party identifier only.
/// Used when BIC is not required or available for routing purposes.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field56C {
    /// Party identifier for intermediary routing
    ///
    /// Format: /34x - Mandatory slash prefix + up to 34 character identifier
    /// Used for domestic routing codes and clearing system identifiers
    #[component("/34x")]
    pub party_identifier: String,
}

///   **Field 56D: Intermediary (Party Identifier with Name and Address)**
///
/// Detailed intermediary identification with full name and address information.
/// Used when structured BIC identification is not available for intermediary.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field56D {
    /// Optional party identifier for routing and payment method codes
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// May contain special routing codes and clearing system identifiers
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address of the intermediary institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field56Intermediary {
    A(Field56A),
    C(Field56C),
    D(Field56D),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field56IntermediaryAD {
    A(Field56A),
    D(Field56D),
}
