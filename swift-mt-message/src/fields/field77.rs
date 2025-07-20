use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 77: Narrative and Envelope Contents**
///
/// ## Purpose
/// Provides extended narrative information and envelope contents for various financial
/// messages. This field family supports detailed documentation, regulatory information,
/// and structured content that requires more extensive text than standard narrative fields.
/// Essential for compliance, documentation, and detailed communication requirements.
///
/// ## Field Options Overview
/// - **Field 77T**: Envelope Contents - structured envelope information
/// - **Field 77A**: Narrative - extended narrative text (20 lines)
/// - **Field 77B**: Narrative - shorter narrative text (3 lines)
///
/// ## Business Context Applications
/// - **Regulatory Documentation**: Detailed regulatory and compliance information
/// - **Trade Finance**: Extended trade documentation and terms
/// - **Complex Instructions**: Detailed processing instructions
/// - **Legal Documentation**: Legal terms and conditions
///
/// ## Network Validation Requirements
/// - **Format Compliance**: Each variant has specific format requirements
/// - **Character Set**: Must use valid SWIFT character set
/// - **Length Restrictions**: Varying length limits for different options
/// - **Content Validation**: Content must be relevant and appropriate
///
/// ## See Also
/// - Swift FIN User Handbook: Narrative Field Specifications
/// - Regulatory Documentation: Compliance Information Requirements
/// - Trade Finance: Documentary Requirements
/// - Message Documentation: Extended Information Standards
/// **Field 77T: Envelope Contents**
///
/// Contains structured envelope information with specific format requirements.
/// Used for regulatory and compliance documentation with extensive content capacity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field77T {
    /// Envelope content
    ///
    /// Format: 9000z - Up to 9000 characters with specific structure
    /// Contains structured regulatory and compliance information
    #[component("9000z")]
    pub envelope_content: String,
}

/// **Field 77A: Extended Narrative**
///
/// Provides extended narrative information with up to 20 lines of text.
/// Used for detailed documentation and extensive information requirements.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field77A {
    /// Extended narrative content
    ///
    /// Format: 20*35x - Up to 20 lines of 35 characters each
    /// Contains detailed documentation and extended information
    #[component("20*35x")]
    pub narrative: Vec<String>,
}

/// **Field 77B: Short Narrative**
///
/// Provides shorter narrative information with up to 3 lines of text.
/// Used for concise documentation and brief additional information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field77B {
    /// Short narrative content
    ///
    /// Format: 3*35x - Up to 3 lines of 35 characters each
    /// Contains brief additional information and documentation
    #[component("3*35x")]
    pub narrative: Vec<String>,
}
