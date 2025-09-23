use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

///   **Field 75: Queries**
///
/// ## Purpose
/// Specifies query information in Swift MT message query/response workflows. This field
/// contains structured query requests used to obtain clarification, additional information,
/// or status updates regarding original transactions. Essential component of the Swift
/// query/answer ecosystem enabling systematic information exchange between financial institutions.
///
/// ## Format Specification
/// - **Swift Format**: `6*35x`
/// - **Structure**: Up to 6 lines of 35 characters each
/// - **Content**: Structured query information with codes and descriptive text
/// - **Character Set**: Standard SWIFT character set with narrative formatting
///
/// ## Business Context Applications
/// - **Query Messages**: Core component of MT n95 series query messages
/// - **Transaction Clarification**: Requesting clarification on previous transactions
/// - **Status Requests**: Obtaining status updates on pending transactions
/// - **Information Requests**: Requesting additional transaction details
///
/// ## Message Type Integration
/// ### Query Message Types (MT n95 Series)
/// - **MT 195**: Customer payment queries (Category 1)
/// - **MT 295**: Treasury queries (Category 2)
/// - **MT 395**: Foreign exchange queries (Category 3)
/// - **MT 495**: Securities queries (Category 4)
/// - **MT 595**: Securities lending queries (Category 5)
/// - **MT 695**: Commodity queries (Category 6)
/// - **MT 795**: Documentary credits queries (Category 7)
/// - **MT 895**: Traveler's checks queries (Category 8)
/// - **MT 995**: Cash management queries (Category 9)
///
/// ## Network Validation Requirements
/// - **Line Length**: Maximum 6 lines of 35 characters each
/// - **Character Set**: Must use valid SWIFT character set
/// - **Query Structure**: Should follow structured query format
/// - **Reference Consistency**: Must align with transaction references
/// - **Code Validation**: Query codes must be properly formatted
///
/// ## Query Types and Codes
/// ### Common Query Categories
/// - **Status Inquiries**: Transaction processing status requests
/// - **Clarification Requests**: Details about transaction elements
/// - **Amendment Queries**: Questions about transaction modifications
/// - **Settlement Queries**: Settlement status and timing inquiries
/// - **Documentation Requests**: Additional documentation requirements
///
/// ### Structured Query Format
/// ```logic
/// :75:QUERY TYPE: [Description]
/// Additional details on subsequent lines
/// Reference information
/// Specific questions or requirements
/// ```
///
/// ## Query Processing Workflow
/// ### Query Initiation
/// - **Field 75**: Contains the specific query information
/// - **Field 20**: Transaction reference being queried
/// - **Field 21**: Related reference for context
/// - **Message Context**: MT n95 message type for specific category
///
/// ### Query Content Guidelines
/// - **Clear Identification**: Specific transaction or issue identification
/// - **Structured Format**: Organized query information
/// - **Complete Details**: Sufficient detail for proper response
/// - **Reference Accuracy**: Correct references to original transactions
///
/// ## Regional Considerations
/// - **Global Standards**: Consistent query format across all SWIFT regions
/// - **Local Practices**: Regional variations in query handling procedures
/// - **Time Zone Coordination**: Query timing considerations for global operations
/// - **Language Standards**: English language requirement for international queries
///
/// ## Error Prevention Guidelines
/// - **Reference Verification**: Confirm transaction references are accurate
/// - **Query Clarity**: Ensure query is clear and specific
/// - **Format Compliance**: Follow established query format standards
/// - **Complete Information**: Provide sufficient detail for effective response
///
/// ## Related Fields Integration
/// - **Field 76**: Answers (corresponding response field)
/// - **Field 20**: Transaction Reference (query context)
/// - **Field 21**: Related Reference (additional context)
/// - **Field 79**: Narrative (extended query information)
///
/// ## Compliance Framework
/// - **Audit Documentation**: Complete query documentation for audit trails
/// - **Regulatory Requirements**: Meeting regulatory query and response requirements
/// - **Customer Service**: Providing effective customer query resolution
/// - **Operational Efficiency**: Streamlined query processing workflows
///
/// ## Query Resolution Process
/// - **Query Receipt**: Acknowledgment and initial processing
/// - **Investigation**: Detailed investigation of query subject
/// - **Response Preparation**: Formulation of comprehensive answer
/// - **Answer Delivery**: MT n96 message with Field 76 response
///
/// ## Best Practices
/// - **Timely Processing**: Prompt query acknowledgment and response
/// - **Complete Responses**: Comprehensive answers addressing all query points
/// - **Reference Management**: Accurate reference tracking throughout process
/// - **Documentation**: Proper documentation of query/answer cycles
///
/// ## See Also
/// - Swift FIN User Handbook: Query Field Specifications
/// - MT n95 Message Standards: Query Message Types
/// - Query Processing Guidelines: Best Practices for Query Handling
/// - Field 76 Documentation: Answer Field Specifications

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field75 {
    /// Query information
    ///
    /// Format: 6*35x - Up to 6 lines of 35 characters each
    /// Contains structured query requests, codes, and descriptive information
    /// Used to request clarification, status updates, or additional transaction details
    #[component("6*35x")]
    pub information: Vec<String>,
}
