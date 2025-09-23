use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 79: Narrative**
///
/// ## Purpose
/// Provides extended narrative information for various Swift MT messages, offering
/// comprehensive text capacity for detailed transaction descriptions, explanations,
/// and supplementary information. This field supports extensive documentation
/// requirements across multiple message types, enabling complete transaction
/// context and detailed communication between financial institutions.
///
/// ## Format Specification
/// - **Swift Format**: `35*50x`
/// - **Structure**: Up to 35 lines of 50 characters each
/// - **Total Capacity**: Maximum 1,750 characters
/// - **Character Set**: Standard SWIFT character set with extended line capacity
///
/// ## Business Context Applications
/// - **Extended Documentation**: Comprehensive transaction documentation
/// - **Free Format Messages**: Core narrative field for MT 199 and MT 299 messages
/// - **Query/Answer Support**: Extended information for query and answer messages
/// - **Cancellation Reasons**: Detailed explanations in cancellation messages
/// - **Amendment Details**: Complete amendment descriptions and justifications
///
/// ## Message Type Integration
/// ### Primary Applications
/// - **MT 199**: Free format customer messages
/// - **MT 196**: Customer payment answers (optional extended narrative)
/// - **MT 292**: Treasury cancellation (reason details)
/// - **MT 296**: Treasury answers
/// - **MT 299**: Free format treasury messages
/// - **MT 705**: Documentary credits (as Field 79Z)
/// - **Various n96**: Answer messages requiring extended explanations
/// - **Various n99**: Free format messages across categories
///
/// ## Network Validation Requirements
/// - **Line Capacity**: Maximum 35 lines of 50 characters each
/// - **Character Set**: Must use valid SWIFT character set
/// - **Format Restrictions**: Prohibited content and special character rules
/// - **Code Validation**: Special validation for specific content codes
/// - **Reference Patterns**: Restricted slash patterns for security
///
/// ## Content Categories and Applications
/// ### Transaction Documentation
/// - **Detailed Descriptions**: Comprehensive transaction explanations
/// - **Business Context**: Complete business rationale and background
/// - **Regulatory Information**: Compliance and regulatory details
/// - **Amendment Justifications**: Detailed reasons for transaction changes
///
/// ### Reason Codes and Explanations
/// #### Common Reason Categories (from MT 292 integration)
/// - **AGNT**: Agent/Intermediary related issues
/// - **AM09**: Wrong amount scenarios
/// - **COVR**: Cover payment problems
/// - **CURR**: Currency-related issues
/// - **CUST**: Customer-related matters
/// - **CUTA**: Cut-off time violations
/// - **DUPL**: Duplicate transaction handling
/// - **FRAD**: Fraudulent transaction detection
/// - **TECH**: Technical processing problems
/// - **UPAY**: Unpaid transaction situations
///
/// ### Free Format Communication
/// - **Institutional Messages**: Communication between financial institutions
/// - **Customer Communications**: Detailed customer information
/// - **Operational Instructions**: Complex operational procedures
/// - **Exception Explanations**: Detailed exception handling explanations
///
/// ## Special Content Validation
/// ### Prohibited Content Patterns
/// - **Security Restrictions**: Certain slash patterns prohibited for security
/// - **Code Validation**: `/REJT/` and `/RETN/` codes have specific rules
/// - **Reference Restrictions**: Invalid reference patterns must be avoided
/// - **Format Compliance**: Structured content must follow established patterns
///
/// ### Content Structure Guidelines
/// ```logic
/// :79:CATEGORY: [Description]
/// Detailed explanation across multiple lines
/// Additional context and information
/// Supporting details and references
/// Conclusion or summary information
/// ```
///
/// ## Regional Considerations
/// - **Global Standards**: Consistent narrative format across all SWIFT regions
/// - **Local Requirements**: Regional regulatory narrative requirements
/// - **Language Standards**: English language requirement for international messages
/// - **Cultural Considerations**: Appropriate cultural sensitivity in narrative content
///
/// ## Content Quality Standards
/// ### Clarity and Completeness
/// - **Clear Communication**: Unambiguous and clear narrative content
/// - **Complete Information**: Comprehensive coverage of relevant details
/// - **Logical Structure**: Well-organized information presentation
/// - **Professional Tone**: Appropriate professional communication style
///
/// ### Technical Requirements
/// - **Character Limits**: Adherence to line and total character limits
/// - **Format Compliance**: Proper formatting and structure
/// - **Content Relevance**: Relevant and appropriate information only
/// - **Reference Accuracy**: Accurate references and cross-references
///
/// ## Error Prevention Guidelines
/// - **Content Review**: Thorough review of narrative content before transmission
/// - **Format Verification**: Confirmation of proper format compliance
/// - **Character Validation**: Verification of valid character set usage
/// - **Length Checking**: Confirmation of line and total length compliance
///
/// ## Related Fields Integration
/// - **Field 75**: Queries (extended query information)
/// - **Field 76**: Answers (extended answer information)
/// - **Field 77A/B**: Other narrative fields (complementary information)
/// - **Field 20/21**: References (narrative context)
/// - **Field 72**: Sender to Receiver Information (institutional context)
///
/// ## Compliance Framework
/// - **Regulatory Documentation**: Meeting regulatory narrative requirements
/// - **Audit Trail**: Complete narrative documentation for audit purposes
/// - **Customer Communication**: Effective customer information provision
/// - **Legal Documentation**: Proper legal and contractual documentation
///
/// ## Best Practices
/// ### Content Development
/// - **Structured Approach**: Organized and logical information presentation
/// - **Completeness**: Comprehensive coverage of all relevant aspects
/// - **Clarity**: Clear and unambiguous communication
/// - **Professional Standards**: Appropriate professional communication style
///
/// ### Quality Assurance
/// - **Content Review**: Multi-level review of narrative content
/// - **Technical Validation**: Format and character set validation
/// - **Relevance Check**: Confirmation of content relevance and appropriateness
/// - **Compliance Verification**: Regulatory and standard compliance checking
///
/// ## See Also
/// - Swift FIN User Handbook: Narrative Field Specifications
/// - Free Format Message Standards: MT 199 and MT 299 Guidelines
/// - Content Guidelines: Narrative Content Best Practices
/// - Regulatory Standards: Narrative Documentation Requirements

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field79 {
    /// Extended narrative information
    ///
    /// Format: 35*50x - Up to 35 lines of 50 characters each (1,750 total characters)
    /// Contains comprehensive narrative information, explanations, and documentation
    /// Used for detailed transaction descriptions, reasons, and extended communication
    #[component("35*50x")]
    pub information: Vec<String>,
}
