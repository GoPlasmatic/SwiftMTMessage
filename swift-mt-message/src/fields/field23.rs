use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 23: Further Identification**
///
/// ## Purpose
/// Provides additional identification information for financial transactions, particularly
/// in money market and deposit transactions. This field enables precise categorization
/// of transaction types, timing specifications, and reference information essential
/// for proper transaction processing and regulatory compliance.
///
/// ## Format Specification
/// - **Swift Format**: `3!a[2!n]11x`
/// - **Structure**: Function code + optional days + reference information
/// - **Total Length**: Maximum 16 characters
/// - **Components**: Mandatory function code, conditional days, mandatory reference
///
/// ## Business Context Applications
/// - **Money Market Transactions**: Deposit and call money operations
/// - **Treasury Operations**: Interest rate and liquidity management
/// - **Corporate Banking**: Commercial account management
/// - **Settlement Processing**: Transaction categorization and routing
///
/// ## Function Code Categories
/// ### Deposit Operations
/// - **DEPOSIT**: Standard deposit transactions
/// - **NOTICE**: Notice deposit with specific day requirements
/// - **CALL**: Call money transactions (immediate settlement)
/// - **CURRENT**: Current account operations
///
/// ### Commercial Operations
/// - **COMMERCIAL**: Commercial transaction identification
/// - **BASE**: Base rate reference transactions
/// - **PRIME**: Prime rate related operations
///
/// ## Network Validation Requirements
/// - **Function Code**: Must be valid 3-character alphabetic code
/// - **Days Field**: Only required/allowed for NOTICE function code
/// - **Days Range**: 1-99 days when specified for NOTICE transactions
/// - **Reference Format**: Must comply with 11x character set restrictions
/// - **Character Set**: Standard SWIFT character set compliance
///
/// ## Message Type Integration
/// - **MT 200**: Financial institution transfer (function classification)
/// - **MT 202**: General financial institution transfer (operation type)
/// - **MT 210**: Notice to receive (notice period specification)
/// - **Treasury Messages**: Various treasury operations requiring identification
///
/// ## Regional Considerations
/// - **Money Market Standards**: Regional money market convention compliance
/// - **Central Bank Requirements**: Regulatory classification requirements
/// - **Settlement Systems**: Local settlement system integration
/// - **Regulatory Reporting**: Transaction classification for reporting purposes
///
/// ## Validation Logic
/// ### Function Code Rules
/// - **NOTICE**: Requires days field (2!n format, 1-99)
/// - **Other Codes**: Days field must not be present
/// - **Reference**: Always required, 11x format compliance
/// - **Character Validation**: Uppercase alphabetic characters only
///
/// ### Processing Impact
/// - **Settlement Timing**: Function code affects settlement procedures
/// - **Interest Calculation**: Impacts interest computation methods
/// - **Regulatory Classification**: Determines reporting categories
/// - **Risk Assessment**: Influences risk management procedures
///
/// ## Error Prevention Guidelines
/// - **Function Code Verification**: Confirm valid function code selection
/// - **Days Field Logic**: Only include days for NOTICE transactions
/// - **Reference Accuracy**: Ensure reference information is correct
/// - **Format Compliance**: Verify character set and length requirements
///
/// ## Related Fields Integration
/// - **Field 20**: Transaction Reference (transaction context)
/// - **Field 30**: Value Date (timing coordination)
/// - **Field 32A**: Currency/Amount (transaction details)
/// - **Field 52A/D**: Ordering Institution (institutional context)
///
/// ## Compliance Framework
/// - **Money Market Regulations**: Compliance with money market standards
/// - **Central Bank Reporting**: Regulatory classification requirements
/// - **Audit Documentation**: Complete transaction categorization
/// - **Risk Management**: Transaction type classification for risk assessment
///
/// ## Best Practices
/// - **Accurate Classification**: Select appropriate function code
/// - **Notice Period Management**: Proper days specification for NOTICE
/// - **Reference Standards**: Consistent reference information format
/// - **Documentation**: Complete transaction categorization documentation
///
/// ## See Also
/// - Swift FIN User Handbook: Further Identification Field Specifications
/// - Money Market Standards: Function Code Classifications
/// - Treasury Operations Guide: Transaction Identification Best Practices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field23 {
    /// Function code (3!a format: BASE, CALL, COMMERCIAL, CURRENT, DEPOSIT, NOTICE, PRIME)
    #[component("3!a")]
    pub function_code: String,
    /// Number of days (2!n format, optional, only for NOTICE function)
    #[component("2!n")]
    pub days: Option<u8>,
    /// Reference information (11x format)
    #[component("11x")]
    pub reference: String,
}

///   **Field 23B: Bank Operation Code**
///
/// ## Purpose
/// Identifies the type of operation and associated service level for the payment instruction.
/// This field determines processing rules, STP compliance requirements, and available service features.
///
/// ## Format
/// - **Swift Format**: `4!c`
/// - **Description**: Exactly 4 alphabetic characters (uppercase)
/// - **Character Set**: A-Z only, no numbers or special characters
///
/// ## Presence
/// - **Status**: Mandatory in MT103, MT102, and related payment messages
/// - **Swift Error Codes**: T12 (invalid code), T15 (field not allowed)
/// - **Referenced in Rules**: C3, C4, C5, C6, C8, C10, C11, C12 (MT103)
///
/// ## Valid Codes
/// - **CRED**: Normal credit transfer (no specific SWIFT Service Level)
/// - **CRTS**: Test message (should not be processed on FIN network)
/// - **SPAY**: SWIFTPay Service Level (premium service)
/// - **SPRI**: Priority Service Level (highest priority)
/// - **SSTD**: Standard Service Level (standard processing)
///
/// ## Network Validation Rules
/// - **C3**: If SPRI → field 23E restricted to SDVA, TELB, PHOB, INTC only
/// - **C3**: If SSTD or SPAY → field 23E must not be present
/// - **C4**: Service levels affect correspondent institution field requirements
/// - **C6**: SPRI requires specific BIC validation for certain regions
/// - **C10**: If SPRI → field 56a (Intermediary) must not be present
/// - **C12**: Service levels mandate beneficiary account requirements
///
/// ## Usage Rules
/// - **Service Level Selection**: Choose based on urgency and STP requirements
/// - **Cost Implications**: SPRI and SPAY typically incur higher fees
/// - **Processing Time**: SPRI processes faster than SSTD, CRED has standard timing
/// - **STP Compliance**: SPRI, SPAY, SSTD enable straight-through processing
///
/// ## STP Compliance
/// - **STP-Enabled Codes**: SPRI, SPAY, SSTD
/// - **Non-STP Code**: CRED (legacy processing)
/// - **Additional Restrictions**: STP codes have stricter field format requirements
/// - **Correspondent Constraints**: Service levels limit correspondent field options
///
/// ## Regional Considerations
/// - **EU/EEA**: Service level affects SEPA compliance and processing routes
/// - **Correspondents**: Some institutions only support specific service levels
/// - **Settlement**: Service level determines settlement timing and priority
///
/// ## Examples
/// ```logic
/// :23B:CRED    // Normal credit transfer
/// :23B:SPRI    // Priority service level
/// :23B:SSTD    // Standard service level  
/// :23B:SPAY    // SWIFTPay service
/// ```
///
/// ## Related Fields
/// - **Field 23E**: Instruction Code (availability depends on 23B value)
/// - **Field 13C**: Time Indication (may be required for certain service levels)
/// - **Field 72**: Sender to Receiver Information (service level details)
///
/// ## Error Handling
/// - **Invalid Code**: Results in T12 error and message rejection
/// - **Rule Violations**: Service level constraints trigger specific C-rule errors
/// - **STP Failures**: Non-compliant combinations cause processing delays
///
/// ## See Also
/// - Swift FIN User Handbook: Service Level Definitions
/// - MT103 Usage Rules: Bank Operation Code Guidelines
/// - STP Implementation Guide: Service Level Requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field23B {
    /// Bank operation code indicating service level and processing type
    ///
    /// Format: 4!c - Exactly 4 alphabetic characters
    /// Valid codes: CRED, CRTS, SPAY, SPRI, SSTD
    #[component("4!c")]
    pub instruction_code: String,
}

///   **Field 23E: Instruction Code**
///
/// ## Purpose
/// Specifies detailed payment instructions and processing directives that complement
/// the bank operation code in Field 23B. This field provides granular control over
/// payment processing, delivery methods, and special handling requirements essential
/// for precise transaction execution and compliance with specific service levels.
///
/// ## Format Specification
/// - **Swift Format**: `4!c[/30x]`
/// - **Structure**: Mandatory instruction code + optional additional information
/// - **Instruction Code**: Exactly 4 alphabetic characters (uppercase)
/// - **Additional Info**: Up to 30 characters for supplementary details
///
/// ## Business Context Applications
/// - **Payment Instructions**: Specific delivery and processing directives
/// - **Service Level Compliance**: Instructions aligned with Field 23B service levels
/// - **Special Handling**: Non-standard processing requirements
/// - **Delivery Methods**: Communication and notification preferences
///
/// ## Instruction Code Categories
/// ### Standard Delivery Instructions
/// - **SDVA**: Same day value (immediate processing)
/// - **TELB**: Telephone beneficiary before crediting
/// - **PHOB**: Phone beneficiary organization before payment
/// - **INTC**: Intermediary contact required
///
/// ### Communication Instructions
/// - **TELE**: Telephone advice required
/// - **TELEAUTH**: Telephone authorization needed
/// - **HOLD**: Hold payment for specific conditions
/// - **RETN**: Return if undeliverable
///
/// ## Network Validation Requirements
/// - **Service Level Constraints**: Availability depends on Field 23B value
/// - **SPRI Service**: Only SDVA, TELB, PHOB, INTC allowed with Priority Service
/// - **SSTD/SPAY Services**: Field 23E must not be present
/// - **Character Set**: Standard SWIFT character set for additional information
/// - **Length Validation**: Additional info maximum 30 characters
///
/// ## Field 23B Integration Rules
/// ### Priority Service Level (SPRI)
/// - **Allowed Codes**: SDVA, TELB, PHOB, INTC only
/// - **Processing Impact**: Immediate execution requirements
/// - **Validation Rule**: C3 constraint enforcement
/// - **STP Compliance**: Enhanced straight-through processing
///
/// ### Standard/SWIFTPay Service (SSTD/SPAY)
/// - **Field Restriction**: Field 23E must not be present
/// - **Automated Processing**: No manual intervention instructions
/// - **STP Requirements**: Full automation compliance
/// - **Error Prevention**: C3 rule violation avoidance
///
/// ## Regional Considerations
/// - **Local Practices**: Regional instruction code interpretation
/// - **Central Bank Rules**: Regulatory instruction requirements
/// - **Correspondent Capabilities**: Institution-specific instruction support
/// - **Settlement Systems**: Local system instruction integration
///
/// ## Processing Impact
/// ### Same Day Value (SDVA)
/// - **Timing**: Immediate value date processing
/// - **Priority**: High priority execution
/// - **Settlement**: Same day settlement requirements
/// - **Cost**: Potential premium charges
///
/// ### Communication Requirements (TELB/PHOB)
/// - **Manual Intervention**: Human contact required
/// - **Processing Delay**: Additional processing time
/// - **Verification**: Beneficiary confirmation needed
/// - **Documentation**: Contact record maintenance
///
/// ## Error Prevention Guidelines
/// - **Service Level Validation**: Verify compatibility with Field 23B
/// - **Instruction Appropriateness**: Confirm instruction necessity
/// - **Additional Info Format**: Ensure character set compliance
/// - **Processing Capability**: Verify recipient institution capability
///
/// ## Related Fields Integration
/// - **Field 23B**: Bank Operation Code (service level dependency)
/// - **Field 13C**: Time Indication (timing coordination)
/// - **Field 72**: Sender to Receiver Information (instruction context)
/// - **Field 57A**: Account with Institution (delivery coordination)
///
/// ## Compliance Framework
/// - **Service Level Agreements**: Instruction compliance with SLAs
/// - **Regulatory Requirements**: Instruction regulatory compliance
/// - **Operational Procedures**: Standard operating procedure alignment
/// - **Audit Documentation**: Instruction rationale documentation
///
/// ## Usage Examples
/// ```logic
/// :23E:SDVA             // Same day value instruction
/// :23E:TELB/URGENT      // Telephone beneficiary with urgency note
/// :23E:PHOB/CONFIRM     // Phone organization with confirmation required
/// :23E:INTC/AUTH REQ    // Intermediary contact with authorization request
/// ```
///
/// ## Best Practices
/// - **Selective Usage**: Only include when necessary for processing
/// - **Service Level Alignment**: Ensure compatibility with Field 23B
/// - **Clear Instructions**: Provide clear additional information
/// - **Cost Consideration**: Understand fee implications of special instructions
///
/// ## See Also
/// - Swift FIN User Handbook: Instruction Code Specifications
/// - MT103 Usage Rules: Field 23E Implementation Guidelines
/// - Service Level Guide: Instruction Code Compatibility Matrix
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field23E {
    /// Instruction code
    #[component("4!c")]
    pub instruction_code: String,
    /// Additional information (optional)
    #[component("[/30x]")]
    pub additional_info: Option<String>,
}
