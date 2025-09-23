use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 52: Ordering Institution / Account Servicing Institution**
///
/// ## Purpose
/// Specifies the financial institution of the ordering customer when different from
/// the Sender, or identifies the account servicing institution in various transaction
/// contexts. This field enables proper institutional identification and routing in
/// complex multi-party transactions where the ordering customer's bank differs from
/// the message originator. Critical for correspondent banking and institutional relationships.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured institutional identification
/// - **Option B**: Party identifier with location - domestic routing information
/// - **Option C**: Party identifier only - simplified institutional reference
/// - **Option D**: Party identifier with name/address - detailed institutional information
///
/// ## Business Context Applications
/// - **Ordering Institution**: When ordering customer's bank differs from message sender
/// - **Account Servicing**: Institution maintaining the ordering customer's account
/// - **Correspondent Banking**: Institutional relationships in cross-border payments
/// - **Multi-Party Transactions**: Complex routing scenarios requiring institutional clarity
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Clearing Codes**: National clearing codes must be valid for respective countries
/// - **Format Compliance**: Exact adherence to option-specific format requirements
/// - **Institutional Validity**: Referenced institutions must be reachable and operational
///
/// ## National Clearing System Support
/// ### European Systems
/// - **AT (Austria)**: Bankleitzahl - 5!n format
/// - **BL (Germany)**: Bankleitzahl - 8!n format  
/// - **ES (Spain)**: Spanish Domestic - 8..9n format
/// - **GR (Greece)**: HEBIC - 7!n format
/// - **IE (Ireland)**: NSC - 6!n format
/// - **IT (Italy)**: Italian Domestic - 10!n format
/// - **PL (Poland)**: KNR - 8!n format
/// - **PT (Portugal)**: Portuguese - 8!n format
/// - **SC (UK)**: Sort Code - 6!n format
///
/// ### North American Systems
/// - **CC (Canada)**: Canadian Routing - 9!n format
/// - **FW (US)**: Fedwire - without 9 digit code
///
/// ### Asia-Pacific Systems
/// - **AU (Australia)**: BSB - 6!n format
/// - **CN (China)**: CNAPS - 12..14n format
/// - **HK (Hong Kong)**: Hong Kong - 3!n format
/// - **IN (India)**: IFSC - 11!c format
///
/// ## Regional Considerations
/// - **European Payments**: SEPA routing and TARGET2 integration
/// - **US Payments**: Fedwire and ACH routing requirements
/// - **Asian Markets**: Local clearing system compliance
/// - **Cross-Border**: International correspondent banking arrangements
///
/// ## STP Processing Benefits
/// - **Automated Routing**: System-driven institutional routing based on clear identification
/// - **Exception Reduction**: Proper institutional identification reduces manual intervention
/// - **Straight-Through Processing**: Enhanced STP rates through structured data
/// - **Risk Mitigation**: Clear institutional accountability and routing paths
///
/// ## Error Prevention Guidelines
/// - **BIC Validation**: Verify all BIC codes are registered and reachable
/// - **Code Verification**: Confirm national clearing codes are current and valid
/// - **Format Checking**: Ensure exact compliance with option format requirements
/// - **Institutional Verification**: Confirm referenced institutions can process transactions
///
/// ## Related Fields Integration
/// - **Field 50A/K**: Ordering Customer (institutional customer relationship)
/// - **Field 53A**: Sender's Correspondent (reimbursement routing)
/// - **Field 57A**: Account With Institution (beneficiary institutional relationship)
/// - **Field 72**: Sender to Receiver Information (additional institutional details)
///
/// ## Compliance Framework
/// - **Regulatory Identification**: Clear institutional identification for compliance
/// - **Audit Documentation**: Complete institutional routing trail
/// - **Risk Management**: Proper institutional identification for risk assessment
/// - **Investigation Support**: Clear institutional details for compliance reviews
///
/// ## See Also
/// - Swift FIN User Handbook: Ordering Institution Specifications
/// - National Clearing Code Directory: Country-Specific Routing Codes
/// - Correspondent Banking Guidelines: Institutional Relationship Standards
/// - BIC Directory: Registered Financial Institution Codes
///
///   **Field 52A: Ordering Institution (BIC with Party Identifier)**
///
/// Structured institutional identification using BIC code with optional party identifier.
/// Preferred option for automated processing and correspondent banking.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field52A {
    /// Optional party identifier for clearing system or account reference
    ///
    /// Format: \[/1!a/34x\] - Single character code + up to 34 character identifier
    /// Used for national clearing codes and institutional account references
    #[component("[/1!a/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the ordering institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

///   **Field 52B: Ordering Institution (Party Identifier with Location)**
///
/// Domestic routing information using party identifier and location details.
/// Used for national clearing systems requiring location-based routing.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field52B {
    /// Optional party identifier for clearing system or account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for domestic clearing systems and institutional references
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Location information for domestic routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based routing within domestic systems
    #[component("[35x]")]
    pub location: Option<String>,
}

///   **Field 52C: Ordering Institution (Party Identifier Only)**
///
/// Simplified institutional reference using party identifier only.
/// Used when BIC is not required or available for institutional identification.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field52C {
    /// Party identifier for institutional reference
    ///
    /// Format: /34x - Mandatory slash prefix + up to 34 character identifier
    /// Used for domestic institutional references and clearing codes
    #[component("/34x")]
    pub party_identifier: String,
}

///   **Field 52D: Ordering Institution (Party Identifier with Name and Address)**
///
/// Detailed institutional identification with full name and address information.
/// Used when structured BIC identification is not available or sufficient.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field52D {
    /// Optional party identifier for clearing system or account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for domestic clearing systems and institutional references
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address of the ordering institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field52AccountServicingInstitution {
    A(Field52A),
    C(Field52C),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field52OrderingInstitution {
    A(Field52A),
    D(Field52D),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field52CreditorBank {
    A(Field52A),
    C(Field52C),
    D(Field52D),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field52DrawerBank {
    A(Field52A),
    B(Field52B),
    D(Field52D),
}
