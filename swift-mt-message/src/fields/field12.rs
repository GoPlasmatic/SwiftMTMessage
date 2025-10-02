//! # Field 12: Sub Message Type
//!
//! ## Purpose
//! Specifies a sub-message type or additional message categorization code that supplements
//! the main MT message type. This field provides further classification within a message
//! category, enabling more precise message routing and processing.
//!
//! ## Format Specification
//! - **Swift Format**: `3!n`
//! - **Description**: Exactly 3 numeric digits
//! - **Character Set**: 0-9 only, no alphabetic characters
//! - **Range**: Valid codes depend on parent message type and business context
//!
//! ## Presence and Usage
//! - **Status**: Optional in most contexts, mandatory when sub-classification is required
//! - **Swift Error Codes**: T12 (invalid format), T50 (invalid sub-type code)
//! - **Usage Context**: Varies by message type and processing requirements
//!
//! ## Business Applications
//! ### Message Classification
//! - **Sub-Classification**: Provides additional categorization within main message type
//! - **Processing Logic**: Used by routing and processing systems for message handling
//! - **Validation**: Must be valid sub-type code for the specific message context
//! - **System Integration**: Enables automated processing based on sub-type classification
//!
//! ### Network Processing
//! - **Message Routing**: Determines specific processing paths within message categories
//! - **System Processing**: Enables automated handling based on sub-type requirements
//! - **Compliance**: May be required for certain regulatory or business requirements
//! - **Integration**: Facilitates system-to-system communication with precise message typing
//!
//! ## Network Validation Rules
//! - **Format Validation**: Must be exactly 3 numeric digits
//! - **Code Validation**: Sub-type code must be valid for the message context
//! - **Processing Rules**: Used by SWIFT network for routing and validation decisions
//! - **Context Verification**: Sub-type must be appropriate for business scenario
//!
//! ## Common Sub-Type Codes
//! - **103**: Customer credit transfer variant
//! - **102**: Multiple customer credit transfer
//! - **950**: Statement message variant
//! - **001**: Standard processing
//!
//! ## Regional Considerations
//! - **Local Variations**: Some regions may use specific sub-type codes
//! - **Processing Standards**: Different markets may have preferred sub-type classifications
//! - **Regulatory Requirements**: Certain jurisdictions may mandate specific sub-types
//! - **System Integration**: Local clearing systems may require specific sub-type codes
//!
//! ## Processing Impact
//! - **Routing Decisions**: Influences how messages are routed through SWIFT network
//! - **Validation Rules**: May trigger specific validation requirements
//! - **STP Processing**: Can affect straight-through processing capabilities
//! - **Exception Handling**: Determines appropriate exception handling procedures
//!
//! ## Error Prevention Guidelines
//! - **Code Validation**: Verify sub-type code is valid for message context
//! - **Format Checking**: Ensure exactly 3 numeric digits
//! - **Context Validation**: Confirm sub-type is appropriate for business scenario
//! - **System Compatibility**: Check receiving system supports the sub-type code
//!
//! ## Related Fields Integration
//! - **Block 2**: Application Header (contains main message type)
//! - **Field 11**: MT Reference (may reference messages with specific sub-types)
//! - **Message Body**: Other fields may have dependencies on sub-type classification
//!
//! ## See Also
//! - Swift FIN User Handbook: Message Type Classification
//! - Network Rules: Sub-Message Type Standards
//! - Processing Guidelines: Sub-Type Routing Rules
//! - System Integration Guide: Sub-Type Code Usage

use super::swift_utils::{parse_exact_length, parse_numeric};
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 12: Sub Message Type**
///
/// Sub-message type classification variant of [Field 12 module](index.html). Provides additional
/// categorization within main MT message types for precise routing and processing.
///
/// **Components:**
/// - Type code (3!n, exactly 3 numeric digits)
///
/// For complete documentation, see the [Field 12 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field12 {
    /// Sub-message type or categorization code
    ///
    /// Format: 3!n - Exactly 3 numeric digits (0-9)
    /// Used for additional message classification within main MT type
    pub type_code: String,
}

impl SwiftField for Field12 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Must be exactly 3 numeric digits
        let type_code = parse_exact_length(input, 3, "Field 12 type code")?;
        parse_numeric(&type_code, "Field 12 type code")?;

        Ok(Field12 { type_code })
    }

    fn to_swift_string(&self) -> String {
        format!(":12:{}", self.type_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field12_valid() {
        let field = Field12::parse("103").unwrap();
        assert_eq!(field.type_code, "103");
        assert_eq!(field.to_swift_string(), ":12:103");

        let field = Field12::parse("001").unwrap();
        assert_eq!(field.type_code, "001");
        assert_eq!(field.to_swift_string(), ":12:001");

        let field = Field12::parse("950").unwrap();
        assert_eq!(field.type_code, "950");
        assert_eq!(field.to_swift_string(), ":12:950");
    }

    #[test]
    fn test_field12_invalid() {
        // Too short
        assert!(Field12::parse("12").is_err());

        // Too long
        assert!(Field12::parse("1234").is_err());

        // Non-numeric
        assert!(Field12::parse("ABC").is_err());
        assert!(Field12::parse("12A").is_err());
    }
}
