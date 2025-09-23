use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 57: Account With Institution**
///
/// ## Purpose
/// Specifies the financial institution that services the account for the beneficiary customer
/// (Field 59A). This field identifies the beneficiary's bank where the account is maintained
/// and where funds will ultimately be credited. Essential for final delivery of funds and
/// beneficiary account identification. Critical component of payment settlement chain.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured beneficiary bank identification
/// - **Option B**: Party identifier with location - domestic beneficiary bank routing
/// - **Option C**: Party identifier only - simplified beneficiary bank reference
/// - **Option D**: Party identifier with name/address - detailed beneficiary bank information
///
/// ## Business Context Applications
/// - **Beneficiary Bank**: Institution maintaining beneficiary's account
/// - **Final Settlement**: Ultimate destination for payment funds
/// - **Account Services**: Institution providing account services to beneficiary
/// - **Regulatory Reporting**: Required for beneficiary institution identification
///
/// ## Usage Rules and Conditions
/// - **Conditional Presence**: Required based on Rules C5 and C10
/// - **Receiver Default**: When absent, Receiver is also the account with institution
/// - **IBAN Compatibility**: Applicable even when Field 59A contains IBAN
/// - **Direct Settlement**: Enables direct settlement when Receiver is account institution
///
/// ## Special Payment Method Codes
/// ### Critical Settlement Instructions
/// - **//FW**: Fedwire routing - Required by US banks for Fedwire settlement
/// - **//RT**: Real-Time Gross Settlement - Binding instruction for RTGS systems
/// - **//AU**: Australian payment system settlement
/// - **//IN**: Indian payment system settlement
///
/// ### Code Usage Rules
/// - **Single Usage**: Codes //FW, //AU, //IN, //RT should appear only once in Field 56A or 57A
/// - **Binding Nature**: //RT code is binding and cannot be followed by other information
/// - **Final Settlement**: Ensures proper final settlement through appropriate systems
/// - **System Integration**: Enables automated settlement in national payment systems
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Account Services**: Institution must provide account services to beneficiaries
/// - **Settlement Capability**: Must support final settlement in transaction currency
/// - **Regulatory Compliance**: Must meet beneficiary institution requirements
///
/// ## Settlement Logic and Processing
/// ### Direct Settlement
/// - **Receiver as Account Institution**: Simplest settlement scenario
/// - **Direct Relationship**: When Sender has direct relationship with beneficiary bank
/// - **Bilateral Agreements**: Pre-established settlement arrangements
/// - **Currency Considerations**: Direct settlement in transaction currency
///
/// ### Intermediated Settlement
/// - **Through Intermediary**: Settlement via Field 56A intermediary
/// - **Correspondent Network**: Utilizing correspondent banking relationships
/// - **Multi-Hop Settlement**: Complex settlement chains with multiple institutions
/// - **Optimization**: Most efficient settlement path selection
///
/// ## Regional Payment System Integration
/// ### North American Systems
/// - **Fedwire (//FW)**: US Federal Reserve final settlement system
/// - **ACH Networks**: Automated clearing house final settlement
/// - **Canadian Systems**: Canadian payment system final settlement
///
/// ### European Systems
/// - **TARGET2**: European Central Bank RTGS final settlement
/// - **SEPA**: Single Euro Payments Area account crediting
/// - **National Systems**: Country-specific final settlement systems
///
/// ### Asia-Pacific Systems
/// - **Australian (//AU)**: Australian payment system final settlement
/// - **Indian (//IN)**: Indian payment system final settlement
/// - **Regional Networks**: ASEAN and other regional final settlement
///
/// ## Beneficiary Protection and Compliance
/// - **Account Verification**: Ensuring beneficiary account exists and is operational
/// - **Name Matching**: Coordination with beneficiary customer name in Field 59A
/// - **Regulatory Requirements**: Meeting beneficiary institution reporting requirements
/// - **Sanctions Screening**: Beneficiary institution sanctions compliance
///
/// ## STP Processing Benefits
/// - **Automated Settlement**: System-driven final settlement based on clear identification
/// - **Account Integration**: Direct integration with beneficiary account systems
/// - **Exception Reduction**: Proper institution identification reduces settlement failures
/// - **Straight-Through Processing**: Enhanced STP through structured settlement data
///
/// ## Error Prevention Guidelines
/// - **Institution Verification**: Confirm institution provides account services
/// - **Account Relationship**: Verify institution-beneficiary account relationship
/// - **System Compatibility**: Ensure institution supports required settlement systems
/// - **Currency Support**: Confirm institution handles transaction currency
///
/// ## Related Fields Integration
/// - **Field 59A**: Beneficiary Customer (account holder identification)
/// - **Field 56A**: Intermediary (settlement routing)
/// - **Field 32A**: Value Date, Currency, Amount (settlement details)
/// - **Field 70**: Remittance Information (payment purpose)
///
/// ## Compliance Framework
/// - **Beneficiary Institution Due Diligence**: Enhanced due diligence requirements
/// - **Account Verification**: Regulatory account verification requirements
/// - **Settlement Documentation**: Complete settlement chain documentation
/// - **Audit Trail**: Comprehensive beneficiary institution audit trail
///
/// ## Performance and Risk Management
/// - **Settlement Speed**: Optimized settlement speed through proper institution identification
/// - **Settlement Risk**: Risk mitigation through established institution relationships
/// - **Operational Risk**: Backup settlement arrangements for business continuity
/// - **Cost Optimization**: Efficient settlement cost management
///
/// ## See Also
/// - Swift FIN User Handbook: Account With Institution Specifications
/// - Payment Settlement Guidelines: Beneficiary Institution Requirements
/// - Cross-Border Payments: Final Settlement Standards
/// - Regulatory Compliance: Beneficiary Institution Due Diligence
///
///   **Field 57A: Account With Institution (BIC with Party Identifier)**
///
/// Structured beneficiary bank identification using BIC code with optional party identifier.
/// Preferred option for automated settlement and final fund delivery.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field57A {
    /// Optional party identifier for settlement and payment method codes
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// May contain special codes: //FW (Fedwire), //RT (RTGS), //AU (Australian), //IN (Indian)
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the account with institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution providing account services
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

///   **Field 57B: Account With Institution (Party Identifier with Location)**
///
/// Domestic beneficiary bank routing using party identifier and location details.
/// Used for location-based settlement in domestic payment systems.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field57B {
    /// Optional party identifier for settlement routing
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for domestic clearing codes and beneficiary bank identification
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Location information for beneficiary bank routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based settlement within domestic systems
    #[component("[35x]")]
    pub location: Option<String>,
}

///   **Field 57C: Account With Institution (Party Identifier Only)**
///
/// Simplified beneficiary bank reference using party identifier only.
/// Used when BIC is not required or available for settlement purposes.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field57C {
    /// Party identifier for beneficiary bank settlement
    ///
    /// Format: /34x - Mandatory slash prefix + up to 34 character identifier
    /// Used for domestic settlement codes and clearing system identifiers
    #[component("/34x")]
    pub party_identifier: String,
}

///   **Field 57D: Account With Institution (Party Identifier with Name and Address)**
///
/// Detailed beneficiary bank identification with full name and address information.
/// Used when structured BIC identification is not available for beneficiary bank.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field57D {
    /// Optional party identifier for settlement routing
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for settlement codes and beneficiary bank identification
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address of the account with institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field57AccountWithInstitution {
    A(Field57A),
    B(Field57B),
    C(Field57C),
    D(Field57D),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field57DebtorBank {
    A(Field57A),
    C(Field57C),
    D(Field57D),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field57DebtInstitution {
    A(Field57A),
    B(Field57B),
    D(Field57D),
}
