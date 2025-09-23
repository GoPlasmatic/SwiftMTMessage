use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 59F: Beneficiary Customer (Option F)**
///
/// ## Purpose
/// Provides detailed beneficiary customer identification using party identifier combined
/// with structured name and address information. This option enables comprehensive
/// beneficiary documentation while maintaining STP compatibility through structured
/// format requirements and enhanced customer identification capabilities.
///
/// ## Format Specification
/// - **Swift Format**: `[/34x]4*(1!n/33x)`
/// - **Party Identifier**: Optional 34-character identifier (account, reference, or ID)
/// - **Name/Address Lines**: Up to 4 lines with line number + 33 characters each
/// - **Line Structure**: Each line starts with line number (1-4) followed by content
///
/// ## Business Context Applications
/// - **Enhanced KYC**: Detailed customer identification for compliance
/// - **Corporate Beneficiaries**: Complex corporate structure identification
/// - **Multi-jurisdictional**: Cross-border regulatory compliance support
/// - **High-value Transactions**: Enhanced due diligence requirements
///
/// ## Party Identifier Usage
/// ### Identification Types
/// - **Account Numbers**: IBAN, domestic account numbers, or special identifiers
/// - **Customer References**: Internal bank customer reference numbers
/// - **Government IDs**: Tax identification numbers or business registration numbers
/// - **Special Codes**: Regulatory or industry-specific identification codes
///
/// ## Structured Name and Address Format
/// ### Line Number Requirements
/// - **Line 1**: Primary beneficiary name (mandatory)
/// - **Line 2**: Secondary name or business unit (optional)
/// - **Line 3**: Street address or PO Box (recommended)
/// - **Line 4**: City, postal code, country (recommended)
///
/// ### Content Guidelines
/// - **Character Limit**: 33 characters per line (excluding line number)
/// - **Character Set**: Standard SWIFT character set compliance
/// - **Address Completeness**: Sufficient detail for payment delivery
/// - **Name Accuracy**: Legal name matching official documentation
///
/// ## STP Processing Advantages
/// - **Structured Format**: Consistent field parsing and validation
/// - **Line Numbering**: Automated address field mapping
/// - **Regulatory Compliance**: Enhanced compliance documentation
/// - **Data Quality**: Improved accuracy through structured input
///
/// ## Network Validation Requirements
/// - **Line Number Validation**: Must use consecutive numbers 1-4
/// - **Character Set Compliance**: Standard SWIFT character restrictions
/// - **Address Sufficiency**: Adequate address detail for delivery
/// - **Name Consistency**: Consistent beneficiary name across lines
///
/// ## Regional Considerations
/// - **Address Standards**: Local address format compliance
/// - **Regulatory Requirements**: Enhanced beneficiary documentation
/// - **Language Requirements**: English language for international payments
/// - **Cultural Sensitivity**: Appropriate name and address formatting
///
/// ## Enhanced Due Diligence Support
/// ### Compliance Benefits
/// - **Detailed Records**: Comprehensive beneficiary documentation
/// - **Audit Trail**: Complete identification information
/// - **Risk Assessment**: Enhanced risk profiling capabilities
/// - **Regulatory Reporting**: Structured data for compliance reporting
///
/// ## Error Prevention Guidelines
/// - **Complete Information**: Provide all available identification details
/// - **Accurate Line Numbering**: Use correct sequential line numbers
/// - **Character Compliance**: Verify SWIFT character set usage
/// - **Address Verification**: Confirm address accuracy and completeness
///
/// ## Usage Examples
/// ```logic
/// :59F:/GB82WEST12345698765432
/// 1/ACME CORPORATION LIMITED
/// 2/INTERNATIONAL TRADE DIVISION
/// 3/123 BUSINESS PARK AVENUE
/// 4/LONDON EC1A 1BB UNITED KINGDOM
/// ```
///
/// ## Related Fields Integration
/// - **Field 57A**: Account with Institution (beneficiary bank)
/// - **Field 70**: Remittance Information (payment purpose)
/// - **Field 77T**: Structured Remittance Information (enhanced details)
/// - **Field 72**: Sender to Receiver Information (additional context)
///
/// ## Compliance Framework
/// - **KYC Enhancement**: Detailed customer identification support
/// - **AML Compliance**: Enhanced anti-money laundering documentation
/// - **Regulatory Documentation**: Complete beneficiary record keeping
/// - **Audit Support**: Comprehensive identification audit trail
///
/// ## Best Practices
/// - **Complete Documentation**: Provide all available beneficiary details
/// - **Structured Approach**: Use consistent line numbering and formatting
/// - **Accuracy Verification**: Verify all identification information
/// - **Compliance Awareness**: Understand regulatory documentation requirements
///
/// ## See Also
/// - Swift FIN User Handbook: Option F Beneficiary Specifications
/// - KYC Guidelines: Enhanced Customer Identification Requirements
/// - Regulatory Compliance: Beneficiary Documentation Standards
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field59F {
    /// Party identifier
    #[component("[/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address lines
    #[component("4*(1!n/33x)")]
    pub name_and_address: Vec<String>,
}

///   **Field 59A: Beneficiary Customer (Option A)**
///
/// ## Purpose
/// Provides structured beneficiary customer identification using BIC-based format
/// for optimal STP (Straight-Through Processing) compliance. This option combines
/// optional account information with mandatory BIC identification, enabling
/// automated processing and efficient routing through the correspondent banking network.
///
/// ## Format Specification
/// - **Swift Format**: `[/34x]4!a2!a2!c[3!c]`
/// - **Account**: Optional 34-character account identifier
/// - **BIC Structure**: 8 or 11 character Bank Identifier Code
/// - **BIC Format**: Bank Code (4) + Country (2) + Location (2) + Optional Branch (3)
///
/// ## Business Context Applications
/// - **STP Processing**: Optimal format for automated payment processing
/// - **Correspondent Banking**: Direct routing through BIC network
/// - **High-volume Payments**: Efficient processing for payment batches
/// - **Standard Transfers**: Most common format for routine payments
///
/// ## BIC Code Structure and Validation
/// ### 8-Character BIC (Head Office)
/// - **Bank Code**: 4 alphabetic characters (institution identifier)
/// - **Country Code**: 2 alphabetic characters (ISO 3166-1 alpha-2)
/// - **Location Code**: 2 alphanumeric characters (city/region identifier)
/// - **Usage**: Default routing to institution's head office
///
/// ### 11-Character BIC (Branch Office)
/// - **Branch Code**: Additional 3 alphanumeric characters
/// - **Branch Identification**: Specific branch or department routing
/// - **Usage**: Direct routing to specific branch location
/// - **Validation**: Branch code must be valid for the institution
///
/// ## Account Information Guidelines
/// ### Account Format Options
/// - **IBAN**: International Bank Account Number (preferred for EUR and some currencies)
/// - **Domestic Account**: Country-specific account number format
/// - **Special Identifiers**: Institution-specific customer identifiers
/// - **No Account**: BIC-only identification for certain transaction types
///
/// ### Account Validation Requirements
/// - **IBAN Validation**: Checksum and format validation for IBAN accounts
/// - **Domestic Standards**: Compliance with local account number formats
/// - **Character Set**: Standard SWIFT character set restrictions
/// - **Length Limits**: Maximum 34 characters for account information
///
/// ## STP Processing Advantages
/// - **Automated Routing**: Direct BIC-based routing without manual intervention
/// - **Validation Efficiency**: Automated BIC verification and reachability checks
/// - **Processing Speed**: Fastest processing option for payment instructions
/// - **Cost Effectiveness**: Lower processing costs due to automation
///
/// ## Network Validation Requirements
/// - **BIC Reachability**: BIC must be active and reachable in SWIFT network
/// - **BIC Format**: Must conform to ISO 9362 standard
/// - **Account Compatibility**: Account format must be compatible with receiving institution
/// - **Currency Support**: Institution must support the payment currency
///
/// ## Regional Considerations
/// ### SEPA (Single Euro Payments Area)
/// - **IBAN Requirement**: IBAN mandatory for EUR payments within SEPA
/// - **BIC Usage**: BIC required for non-SEPA or high-value SEPA payments
/// - **Processing Rules**: SEPA-specific validation and routing rules
///
/// ### US Dollar Payments
/// - **Fedwire**: BIC required for USD payments via Fedwire
/// - **CHIPS**: BIC-based routing for CHIPS network payments
/// - **Correspondent Banking**: BIC enables correspondent banking relationships
///
/// ## Error Prevention Guidelines
/// - **BIC Verification**: Confirm BIC is active and supports required services
/// - **Account Validation**: Verify account format matches institution standards
/// - **Currency Check**: Ensure institution accepts the payment currency
/// - **Reachability Test**: Confirm institution is reachable for the message type
///
/// ## Usage Examples
/// ```logic
/// // With IBAN account
/// :59A:/GB82WEST12345698765432
/// MIDLGB22XXX
///
/// // With domestic account number
/// :59A:/12345678
/// CHASUS33XXX
///
/// // BIC-only identification
/// :59A:DEUTDEFFXXX
/// ```
///
/// ## Related Fields Integration
/// - **Field 57A**: Account with Institution (beneficiary bank BIC)
/// - **Field 56A**: Intermediary Institution (routing BIC)
/// - **Field 70**: Remittance Information (payment purpose)
/// - **Field 33B**: Currency/Amount (currency compatibility check)
///
/// ## Compliance Framework
/// - **STP Standards**: Full compliance with STP processing requirements
/// - **BIC Directory**: Compliance with SWIFT BIC directory standards
/// - **Regulatory Requirements**: Meeting regulatory identification standards
/// - **Audit Trail**: Complete electronic audit trail for automated processing
///
/// ## Processing Impact
/// - **Routing Efficiency**: Direct routing through correspondent banking network
/// - **Processing Time**: Fastest processing due to automated handling
/// - **Cost Optimization**: Lower fees due to STP processing
/// - **Error Reduction**: Reduced manual intervention and error rates
///
/// ## Best Practices
/// - **BIC Accuracy**: Always verify BIC codes before transmission
/// - **Account Standards**: Follow local account number conventions
/// - **STP Optimization**: Use this option for maximum processing efficiency
/// - **Regular Updates**: Keep BIC information current and validated
///
/// ## See Also
/// - Swift FIN User Handbook: Option A Beneficiary Specifications
/// - ISO 9362 Standard: BIC Code Structure and Validation
/// - STP Implementation Guide: Beneficiary Identification Best Practices
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field59A {
    /// Account number (optional)
    #[component("[/34x]")]
    pub account: Option<String>,
    /// BIC code
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

/// **Field 59 (No Option): Beneficiary Customer**
///
/// Flexible variant of [Field 59 module](index.html). Provides beneficiary customer
/// identification combining optional account information with free-format name and address.
///
/// **Components:**
/// - Account identifier (optional, [/34x])
/// - Name and address lines (4*35x)
///
/// For complete documentation, see the [Field 59 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field59NoOption {
    /// Account number (optional)
    ///
    /// Format: [/34x] - Optional account identifier up to 34 characters
    /// Used for IBAN, domestic account numbers, or special identifiers
    #[component("[/34x]")]
    pub account: Option<String>,

    /// Name and address lines
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains beneficiary name and address information in flexible format
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

///   **Field 59: Beneficiary Customer**
///
/// ## Purpose
/// Identifies the ultimate beneficiary of the payment instruction. The beneficiary customer
/// is the final recipient of the funds being transferred. Different options provide various
/// levels of detail and identification methods to accommodate different business scenarios.
///
/// ## Options Overview
/// - **Option A**: Account + BIC identification (structured, STP-preferred)
/// - **Option F**: Party identifier + name/address (detailed identification)
/// - **No Option**: Account + name/address (flexible format)
///
/// ## STP Compliance Requirements
/// - **STP Preferred**: Option A with valid BIC and account information
/// - **STP Compatible**: No option with complete account and address details
/// - **Manual Processing**: Incomplete or incorrectly formatted beneficiary information
///
/// ## Network Validation Rules
/// - **Account Validation**: Account numbers must conform to domestic standards when provided
/// - **BIC Validation**: BIC codes must be valid and reachable when specified
/// - **Name Validation**: Beneficiary name must not match sanctions screening lists
/// - **Address Completeness**: Sufficient address detail for regulatory compliance
///
/// ## Regional Considerations
/// - **SEPA**: IBAN mandatory for EUR payments within SEPA zone
/// - **US**: Fedwire and ACH routing information may be required
/// - **UK**: Sort code and account number validation for GBP payments
/// - **Emerging Markets**: Enhanced beneficiary documentation may be required
///
/// ## Anti-Money Laundering (AML) Requirements
/// - **Customer Due Diligence**: Beneficiary information must support KYC requirements
/// - **Sanctions Screening**: Real-time screening against global watchlists
/// - **Regulatory Reporting**: Some jurisdictions require detailed beneficiary reporting
/// - **Record Keeping**: Beneficiary details retained for compliance periods
///
/// ## Examples by Option
/// ```logic
/// // Option A: BIC-based (STP preferred)
/// :59A:/GB82WEST12345698765432
/// MIDLGB22XXX
///
/// // Option F: Party identifier with details
/// :59F:/GB82WEST12345698765432
/// 1/ACME CORPORATION LIMITED
/// 2/123 BUSINESS STREET
/// 3/LONDON EC1A 1BB
/// 4/UNITED KINGDOM
///
/// // No option: Flexible format
/// :59:/GB82WEST12345698765432
/// JOHN SMITH
/// 456 RESIDENTIAL AVENUE
/// MANCHESTER M1 1AA
/// UNITED KINGDOM
/// ```
///
/// ## Related Fields
/// - **Field 57a**: Account With Institution (beneficiary's bank)
/// - **Field 70**: Remittance Information (payment purpose/reference)
/// - **Field 77T**: Structured Remittance Information (REMIT messages)
/// - **Field 72**: Sender to Receiver Information (additional beneficiary details)
///
/// ## Error Prevention Guidelines
/// - **Complete Information**: Provide full name, address, and account details
/// - **Accurate BICs**: Verify BIC codes before transmission
/// - **Consistent Formatting**: Follow domestic account number standards
/// - **Sanctions Compliance**: Screen against current sanctions lists
///
/// ## See Also
/// - Swift FIN User Handbook: Beneficiary Customer Specifications
/// - FATF Guidelines: Customer Due Diligence Requirements
/// - Regional Payment Guides: Country-specific beneficiary requirements
/// - AML/CFT Compliance: Beneficiary Screening Best Practices
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field59 {
    /// Option A: BIC-based identification with optional account
    /// Preferred for STP processing with structured bank identification
    A(Field59A),

    /// Option F: Party identifier with detailed name and address
    /// Used when enhanced beneficiary identification is required
    F(Field59F),

    /// No option: Account and name/address in flexible format
    /// Most common option providing balance of structure and flexibility
    NoOption(Field59NoOption),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field59Debtor {
    A(Field59A),
    NoOption(Field59NoOption),
}
