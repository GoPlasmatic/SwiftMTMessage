use super::field_utils::validate_multiline_text;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 77: Narrative and Envelope Contents**
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
///   **Field 77T: Envelope Contents**
///
/// Contains structured envelope information with specific format requirements.
/// Used for regulatory and compliance documentation with extensive content capacity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field77T {
    /// Envelope content
    ///
    /// Format: 9000z - Up to 9000 characters with specific structure
    /// Contains structured regulatory and compliance information
    pub envelope_content: String,
}

impl SwiftField for Field77T {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 77T cannot be empty".to_string(),
            });
        }

        if input.len() > 9000 {
            return Err(ParseError::InvalidFormat {
                message: format!("Field 77T exceeds 9000 characters, found {}", input.len()),
            });
        }

        // Note: 'z' format allows any character including spaces and newlines
        // No character set validation needed for this format

        Ok(Field77T {
            envelope_content: input.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":77T:{}", self.envelope_content)
    }
}

///   **Field 77A: Extended Narrative**
///
/// Provides extended narrative information with up to 20 lines of text.
/// Used for detailed documentation and extensive information requirements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field77A {
    /// Extended narrative content
    ///
    /// Format: 20*35x - Up to 20 lines of 35 characters each
    /// Contains detailed documentation and extended information
    pub narrative: Vec<String>,
}

impl SwiftField for Field77A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();
        let narrative = validate_multiline_text(&lines, 20, 35, "Field 77A")?;
        Ok(Field77A { narrative })
    }

    fn to_swift_string(&self) -> String {
        format!(":77A:{}", self.narrative.join("\n"))
    }
}

///   **Field 77B: Short Narrative**
///
/// Provides shorter narrative information with up to 3 lines of text.
/// Used for concise documentation and brief additional information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field77B {
    /// Short narrative content
    ///
    /// Format: 3*35x - Up to 3 lines of 35 characters each
    /// Contains brief additional information and documentation
    pub narrative: Vec<String>,
}

impl SwiftField for Field77B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();
        let narrative = validate_multiline_text(&lines, 3, 35, "Field 77B")?;
        Ok(Field77B { narrative })
    }

    fn to_swift_string(&self) -> String {
        format!(":77B:{}", self.narrative.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field77t_valid() {
        let content = "This is envelope content with multiple lines\nAnd special characters\nRegulatory information";
        let field = Field77T::parse(content).unwrap();
        assert_eq!(field.envelope_content, content);
        assert_eq!(field.to_swift_string(), format!(":77T:{}", content));

        // Large content
        let large_content = "x".repeat(9000);
        let field = Field77T::parse(&large_content).unwrap();
        assert_eq!(field.envelope_content.len(), 9000);
    }

    #[test]
    fn test_field77t_invalid() {
        // Empty
        assert!(Field77T::parse("").is_err());

        // Too large
        let content = "x".repeat(9001);
        assert!(Field77T::parse(&content).is_err());
    }

    #[test]
    fn test_field77a_valid() {
        let field = Field77A::parse("LINE 1\nLINE 2\nLINE 3").unwrap();
        assert_eq!(field.narrative.len(), 3);
        assert_eq!(field.narrative[0], "LINE 1");
        assert_eq!(field.narrative[1], "LINE 2");
        assert_eq!(field.narrative[2], "LINE 3");

        // Single line
        let field = Field77A::parse("SINGLE LINE").unwrap();
        assert_eq!(field.narrative.len(), 1);

        // Max lines
        let mut lines = Vec::new();
        for i in 1..=20 {
            lines.push(format!("LINE {}", i));
        }
        let field = Field77A::parse(&lines.join("\n")).unwrap();
        assert_eq!(field.narrative.len(), 20);
    }

    #[test]
    fn test_field77a_invalid() {
        // Empty
        assert!(Field77A::parse("").is_err());

        // Too many lines
        let mut lines = Vec::new();
        for i in 1..=21 {
            lines.push(format!("LINE {}", i));
        }
        assert!(Field77A::parse(&lines.join("\n")).is_err());

        // Line too long
        assert!(
            Field77A::parse("THIS LINE IS TOO LONG AND EXCEEDS THE 35 CHARACTER LIMIT").is_err()
        );
    }

    #[test]
    fn test_field77b_valid() {
        let field = Field77B::parse("LINE 1\nLINE 2\nLINE 3").unwrap();
        assert_eq!(field.narrative.len(), 3);
        assert_eq!(field.narrative[0], "LINE 1");
        assert_eq!(field.narrative[1], "LINE 2");
        assert_eq!(field.narrative[2], "LINE 3");

        // Single line
        let field = Field77B::parse("SINGLE LINE").unwrap();
        assert_eq!(field.narrative.len(), 1);
    }

    #[test]
    fn test_field77b_invalid() {
        // Empty
        assert!(Field77B::parse("").is_err());

        // Too many lines
        assert!(Field77B::parse("L1\nL2\nL3\nL4").is_err());

        // Line too long
        assert!(
            Field77B::parse("THIS LINE IS TOO LONG AND EXCEEDS THE 35 CHARACTER LIMIT").is_err()
        );
    }
}
