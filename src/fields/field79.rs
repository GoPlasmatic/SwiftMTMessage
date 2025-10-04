use super::swift_utils::parse_swift_chars;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

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
/// ## See Also
/// - Swift FIN User Handbook: Narrative Field Specifications
/// - Free Format Message Standards: MT 199 and MT 299 Guidelines
/// - Content Guidelines: Narrative Content Best Practices
/// - Regulatory Standards: Narrative Documentation Requirements

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field79 {
    /// Extended narrative information
    ///
    /// Format: 35*50x - Up to 35 lines of 50 characters each (1,750 total characters)
    /// Contains comprehensive narrative information, explanations, and documentation
    /// Used for detailed transaction descriptions, reasons, and extended communication
    pub information: Vec<String>,
}

impl SwiftField for Field79 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut lines = Vec::new();

        // Parse up to 35 lines of 50 characters each
        for line in input.lines().take(35) {
            // Validate line length (max 50 characters)
            if line.len() > 50 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 79 line exceeds 50 characters: {}", line.len()),
                });
            }

            // Validate SWIFT character set
            parse_swift_chars(line, "Field 79 line")?;

            lines.push(line.to_string());
        }

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 79 must contain at least one line".to_string(),
            });
        }

        Ok(Field79 { information: lines })
    }

    fn to_swift_string(&self) -> String {
        let content = self.information.join("\n");
        format!(":79:{}", content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field79_parse_single_line() {
        let field = Field79::parse("PAYMENT FOR INVOICE 12345 DATED 2023-12-01").unwrap();
        assert_eq!(field.information.len(), 1);
        assert_eq!(
            field.information[0],
            "PAYMENT FOR INVOICE 12345 DATED 2023-12-01"
        );
    }

    #[test]
    fn test_field79_parse_multiple_lines() {
        let input = "PAYMENT DETAILS:\nINVOICE NUMBER: 12345\nSERVICES PROVIDED: CONSULTING\nPERIOD: DECEMBER 2023";
        let field = Field79::parse(input).unwrap();
        assert_eq!(field.information.len(), 4);
        assert_eq!(field.information[0], "PAYMENT DETAILS:");
        assert_eq!(field.information[1], "INVOICE NUMBER: 12345");
        assert_eq!(field.information[2], "SERVICES PROVIDED: CONSULTING");
        assert_eq!(field.information[3], "PERIOD: DECEMBER 2023");
    }

    #[test]
    fn test_field79_line_too_long() {
        let long_line = "THIS LINE IS MUCH TOO LONG TO BE ACCEPTED IN FIELD 79 AS IT EXCEEDS THE 50 CHARACTER LIMIT";
        assert!(Field79::parse(long_line).is_err());
    }

    #[test]
    fn test_field79_max_line_length() {
        // Exactly 50 characters should work
        let line_50_chars = "12345678901234567890123456789012345678901234567890";
        let field = Field79::parse(line_50_chars).unwrap();
        assert_eq!(field.information[0], line_50_chars);
    }

    #[test]
    fn test_field79_empty_input() {
        assert!(Field79::parse("").is_err());
    }

    #[test]
    fn test_field79_to_swift_string() {
        let field = Field79 {
            information: vec![
                "TRANSACTION DESCRIPTION:".to_string(),
                "PAYMENT FOR SERVICES".to_string(),
                "INVOICE: 2023-12345".to_string(),
            ],
        };
        let expected = ":79:TRANSACTION DESCRIPTION:\nPAYMENT FOR SERVICES\nINVOICE: 2023-12345";
        assert_eq!(field.to_swift_string(), expected);
    }

    #[test]
    fn test_field79_single_line_to_swift_string() {
        let field = Field79 {
            information: vec!["SINGLE LINE NARRATIVE".to_string()],
        };
        assert_eq!(field.to_swift_string(), ":79:SINGLE LINE NARRATIVE");
    }

    #[test]
    fn test_field79_max_lines() {
        let mut lines = Vec::new();
        for i in 1..=35 {
            lines.push(format!("LINE {}", i));
        }
        let input = lines.join("\n");
        let field = Field79::parse(&input).unwrap();
        assert_eq!(field.information.len(), 35);
        assert_eq!(field.information[0], "LINE 1");
        assert_eq!(field.information[34], "LINE 35");
    }
}
