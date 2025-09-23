use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 58: Beneficiary Institution**
///
/// ## Purpose
/// Specifies the ultimate recipient institution of the funds being transferred in specialized
/// payment scenarios. This field identifies the final institutional beneficiary when the
/// payment is destined for a financial institution rather than a customer account.
/// Used in institutional transfers, central bank operations, and specialized financial
/// market transactions where the beneficiary is itself a financial institution.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured institutional beneficiary
/// - **Option D**: Party identifier with name/address - detailed institutional beneficiary
///
/// ## Business Context Applications
/// - **Institutional Transfers**: Payments between financial institutions
/// - **Central Bank Operations**: Transfers to/from central banks and monetary authorities
/// - **Market Infrastructure**: Payments to clearing houses, settlement systems
/// - **Correspondent Banking**: Institutional correspondent relationship transfers
///
/// ## Usage Rules and Conditions
/// - **Mandatory Context**: Required when beneficiary is a financial institution
/// - **Option A Preference**: Option A must be used whenever possible
/// - **Option D Exception**: Only in exceptional circumstances or regulatory requirements
/// - **MT 200/201 Consistency**: Must match Field 52A content if from MT 200/201 transfer
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Institutional Status**: Beneficiary must be recognized financial institution
/// - **Service Capability**: Institution must be capable of receiving institutional transfers
/// - **Regulatory Compliance**: Must meet regulatory requirements for institutional recipients
///
/// ## Extended Clearing Codes (Option D)
/// ### Specialized Institution Codes
/// - **CH**: CHIPS Universal Identifier - 6!n format for CHIPS participants
/// - **CP**: CHIPS Participant - Direct CHIPS participation identifier
/// - **FW**: Fedwire Routing - 9!n format for Federal Reserve routing
/// - **RU**: Russian Central Bank - Russian Federation central bank identifier
/// - **SW**: Swiss Clearing - Swiss national clearing system identifier
///
/// ### Code Applications
/// - **Payment System Integration**: Direct integration with specialized payment systems
/// - **Central Bank Coordination**: Coordination with central bank and monetary authority systems
/// - **Market Infrastructure**: Integration with financial market infrastructure
/// - **Cross-Border Settlement**: International institutional settlement arrangements
///
/// ## Institutional Transfer Types
/// ### Central Bank Operations
/// - **Monetary Policy**: Central bank monetary policy implementation transfers
/// - **Reserve Management**: Bank reserve requirement transfers
/// - **Foreign Exchange**: Central bank FX intervention operations
/// - **Government Operations**: Government banking and treasury operations
///
/// ### Financial Market Infrastructure
/// - **Clearing Houses**: Transfers to/from clearing and settlement organizations
/// - **Securities Settlement**: Settlement of securities transactions
/// - **Derivatives Clearing**: Clearing of derivative instruments
/// - **Payment Systems**: Transfers within payment system infrastructure
///
/// ### Correspondent Banking
/// - **Nostro/Vostro**: Correspondent account management transfers
/// - **Liquidity Management**: Inter-bank liquidity management
/// - **Settlement Services**: Correspondent settlement service payments
/// - **Relationship Management**: Correspondent banking relationship transactions
///
/// ## Regional Considerations
/// ### North American Systems
/// - **Federal Reserve**: US central bank and Federal Reserve Bank transfers
/// - **CHIPS Integration**: Clearing House Interbank Payments System
/// - **Canadian Systems**: Bank of Canada and Canadian institutional transfers
///
/// ### European Systems
/// - **ECB Operations**: European Central Bank institutional transfers
/// - **National Central Banks**: Individual country central bank operations
/// - **TARGET2**: European RTGS system institutional transfers
///
/// ### Asia-Pacific Systems
/// - **Central Bank Networks**: Regional central bank cooperation
/// - **Market Infrastructure**: Regional financial market infrastructure
/// - **Cross-Border Initiatives**: Regional payment and settlement initiatives
///
/// ## Compliance and Risk Management
/// ### Regulatory Framework
/// - **Institutional Due Diligence**: Enhanced due diligence for institutional beneficiaries
/// - **Regulatory Reporting**: Institutional transfer reporting requirements
/// - **Central Bank Oversight**: Central bank supervision and oversight compliance
/// - **Market Conduct**: Financial market conduct and integrity requirements
///
/// ### Risk Considerations
/// - **Counterparty Risk**: Institutional counterparty risk assessment
/// - **Settlement Risk**: Institutional settlement risk management
/// - **Operational Risk**: Institutional operational risk considerations
/// - **Systemic Risk**: Systemic risk implications of institutional transfers
///
/// ## STP Processing Benefits
/// - **Institutional Automation**: Automated processing of institutional transfers
/// - **System Integration**: Direct integration with institutional systems
/// - **Exception Handling**: Specialized handling of institutional transfer exceptions
/// - **Risk Monitoring**: Enhanced risk monitoring for institutional transfers
///
/// ## Error Prevention Guidelines
/// - **Institution Verification**: Confirm beneficiary institution status and capability
/// - **System Compatibility**: Verify institutional system compatibility
/// - **Regulatory Checking**: Ensure regulatory compliance for institutional transfers
/// - **Code Validation**: Validate specialized clearing codes and identifiers
///
/// ## Related Fields Integration
/// - **Field 52A**: Ordering Institution (institutional transfer context)
/// - **Field 57A**: Account With Institution (institutional account relationships)
/// - **Field 32A**: Value Date, Currency, Amount (institutional transfer details)
/// - **Field 72**: Sender to Receiver Information (institutional transfer purpose)
///
/// ## Performance Optimization
/// - **Processing Speed**: Optimized processing for institutional transfers
/// - **Cost Management**: Efficient institutional transfer cost management
/// - **Liquidity Management**: Coordination with institutional liquidity management
/// - **Settlement Timing**: Optimal settlement timing for institutional operations
///
/// ## See Also
/// - Swift FIN User Handbook: Beneficiary Institution Specifications
/// - Central Bank Guidelines: Institutional Transfer Requirements
/// - Financial Market Infrastructure: Institutional Settlement Standards
/// - Regulatory Framework: Institutional Transfer Compliance
///
///   **Field 58A: Beneficiary Institution (BIC with Party Identifier)**
///
/// Structured institutional beneficiary identification using BIC code with optional party identifier.
/// Preferred option for institutional transfers and financial institution beneficiaries.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field58A {
    /// Optional party identifier for institutional account or system reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for institutional account identification and system-specific routing
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the beneficiary institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution capable of receiving institutional transfers
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

///   **Field 58D: Beneficiary Institution (Party Identifier with Name and Address)**
///
/// Detailed institutional beneficiary identification with full name and address information.
/// Used only in exceptional circumstances when structured BIC identification is not available.
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field58D {
    /// Optional party identifier for institutional system reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// May contain extended clearing codes: CH (CHIPS), CP (CHIPS Participant),
    /// FW (Fedwire), RU (Russian Central Bank), SW (Swiss Clearing)
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address of the beneficiary institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field58 {
    A(Field58A),
    D(Field58D),
}
