use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

///   **Field 76: Answers**
///
/// ## Purpose
/// Specifies answer information in Swift MT message query/response workflows. This field
/// contains structured responses to queries submitted through Field 75, providing
/// clarifications, status updates, and additional information regarding original transactions.
/// Essential component completing the Swift query/answer ecosystem for systematic
/// information exchange between financial institutions.
///
/// ## Format Specification
/// - **Swift Format**: `6*35x`
/// - **Structure**: Up to 6 lines of 35 characters each
/// - **Content**: Structured answer information with codes and descriptive responses
/// - **Character Set**: Standard SWIFT character set with narrative formatting
///
/// ## Business Context Applications
/// - **Answer Messages**: Core component of MT n96 series answer messages
/// - **Query Responses**: Providing responses to Field 75 queries
/// - **Status Updates**: Delivering status information on requested transactions
/// - **Clarification Delivery**: Providing requested clarifications and details
///
/// ## Message Type Integration
/// ### Answer Message Types (MT n96 Series)
/// - **MT 196**: Customer payment answers (Category 1)
/// - **MT 296**: Treasury answers (Category 2)
/// - **MT 396**: Foreign exchange answers (Category 3)
/// - **MT 496**: Securities answers (Category 4)
/// - **MT 596**: Securities lending answers (Category 5)
/// - **MT 696**: Commodity answers (Category 6)
/// - **MT 796**: Documentary credits answers (Category 7)
/// - **MT 896**: Traveler's checks answers (Category 8)
/// - **MT 996**: Cash management answers (Category 9)
///
/// ## Network Validation Requirements
/// - **Line Length**: Maximum 6 lines of 35 characters each
/// - **Character Set**: Must use valid SWIFT character set
/// - **Answer Structure**: Should follow structured answer format
/// - **Reference Consistency**: Must correspond to original query references
/// - **Response Completeness**: Must address all points raised in original query
///
/// ## Answer Types and Response Codes
/// ### Common Answer Categories
/// - **Status Responses**: Current transaction processing status
/// - **Clarification Responses**: Detailed explanations of transaction elements
/// - **Amendment Confirmations**: Confirmations of transaction modifications
/// - **Settlement Information**: Settlement status and timing details
/// - **Documentation Responses**: Provision of requested documentation
///
/// ### Common Response Codes (from MT 292 implementation)
/// - **AGNT**: Agent/Intermediary related response
/// - **AM09**: Wrong amount clarification
/// - **COVR**: Cover payment information
/// - **CURR**: Currency-related response
/// - **CUST**: Customer-related information
/// - **CUTA**: Cut-off time information
/// - **DUPL**: Duplicate transaction response
/// - **FRAD**: Fraudulent transaction information
/// - **TECH**: Technical problem resolution
/// - **UPAY**: Unpaid transaction status
///
/// ## Query/Answer Relationship
/// ### Direct Correspondence
/// - **Field 75 Query**: Original query information and codes
/// - **Field 76 Answer**: Direct response to specific query points
/// - **Reference Linkage**: Common transaction references maintain connection
/// - **Complete Resolution**: Answers should address all query elements
///
/// ### Response Structure
/// ```logic
/// :76:ANSWER TYPE: [Response]
/// Detailed answer information
/// Status or clarification details
/// Additional relevant information
/// ```
///
/// ## Answer Processing Workflow
/// ### Answer Preparation
/// - **Query Analysis**: Thorough analysis of Field 75 query
/// - **Investigation**: Detailed investigation of queried transaction
/// - **Response Formulation**: Comprehensive answer preparation
/// - **Quality Review**: Answer completeness and accuracy verification
///
/// ### Answer Content Guidelines
/// - **Complete Response**: Address all points raised in original query
/// - **Clear Format**: Organized and structured answer information
/// - **Accurate Information**: Verified and current information
/// - **Reference Consistency**: Maintain reference accuracy throughout
///
/// ## Regional Considerations
/// - **Global Standards**: Consistent answer format across all SWIFT regions
/// - **Local Requirements**: Regional regulatory response requirements
/// - **Time Zone Coordination**: Response timing considerations
/// - **Language Standards**: English language requirement for international answers
///
/// ## Error Prevention Guidelines
/// - **Query Review**: Thorough review of original query requirements
/// - **Answer Completeness**: Ensure all query points are addressed
/// - **Reference Accuracy**: Verify all transaction references
/// - **Format Compliance**: Follow established answer format standards
///
/// ## Related Fields Integration
/// - **Field 75**: Queries (corresponding query field)
/// - **Field 20**: Transaction Reference (answer context)
/// - **Field 21**: Related Reference (query linkage)
/// - **Field 79**: Narrative (extended answer information)
///
/// ## Compliance Framework
/// - **Audit Documentation**: Complete answer documentation for audit trails
/// - **Regulatory Compliance**: Meeting regulatory response requirements
/// - **Customer Service**: Providing effective customer query resolution
/// - **Service Level Agreements**: Meeting response time commitments
///
/// ## Answer Quality Standards
/// - **Timeliness**: Prompt response within established timeframes
/// - **Completeness**: Comprehensive answers addressing all query aspects
/// - **Accuracy**: Verified and current information provision
/// - **Clarity**: Clear and understandable answer format
///
/// ## Best Practices
/// - **Response Time**: Timely processing and response delivery
/// - **Complete Resolution**: Comprehensive answers avoiding follow-up queries
/// - **Documentation**: Proper documentation of answer rationale
/// - **Follow-up**: Proactive follow-up on complex query resolutions
///
/// ## See Also
/// - Swift FIN User Handbook: Answer Field Specifications
/// - MT n96 Message Standards: Answer Message Types
/// - Query Processing Guidelines: Answer Quality Standards
/// - Field 75 Documentation: Query Field Specifications

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field76 {
    /// Answer information
    ///
    /// Format: 6*35x - Up to 6 lines of 35 characters each
    /// Contains structured responses to queries, codes, and descriptive information
    /// Used to provide clarifications, status updates, and detailed transaction information
    #[component("6*35x")]
    pub information: Vec<String>,
}
