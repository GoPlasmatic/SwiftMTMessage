use super::swift_utils::parse_swift_chars;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

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
/// ## See Also
/// - Swift FIN User Handbook: Answer Field Specifications
/// - MT n96 Message Standards: Answer Message Types
/// - Query Processing Guidelines: Answer Quality Standards
/// - Field 75 Documentation: Query Field Specifications

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field76 {
    /// Answer information
    ///
    /// Format: 6*35x - Up to 6 lines of 35 characters each
    /// Contains structured responses to queries, codes, and descriptive information
    /// Used to provide clarifications, status updates, and detailed transaction information
    pub information: Vec<String>,
}

impl SwiftField for Field76 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut lines = Vec::new();

        // Parse up to 6 lines of 35 characters each
        for line in input.lines().take(6) {
            // Validate line length (max 35 characters)
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 76 line exceeds 35 characters: {}", line.len()),
                });
            }

            // Validate SWIFT character set
            parse_swift_chars(line, "Field 76 line")?;

            lines.push(line.to_string());
        }

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 76 must contain at least one line".to_string(),
            });
        }

        Ok(Field76 { information: lines })
    }

    fn to_swift_string(&self) -> String {
        let content = self.information.join("\n");
        format!(":76:{}", content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field76_parse_single_line() {
        let field = Field76::parse("STATUS: PROCESSED SUCCESSFULLY").unwrap();
        assert_eq!(field.information.len(), 1);
        assert_eq!(field.information[0], "STATUS: PROCESSED SUCCESSFULLY");
    }

    #[test]
    fn test_field76_parse_multiple_lines() {
        let input =
            "TRANSACTION STATUS: COMPLETED\nSETTLEMENT DATE: 2023-12-25\nAMOUNT: USD 1000.00";
        let field = Field76::parse(input).unwrap();
        assert_eq!(field.information.len(), 3);
        assert_eq!(field.information[0], "TRANSACTION STATUS: COMPLETED");
        assert_eq!(field.information[1], "SETTLEMENT DATE: 2023-12-25");
        assert_eq!(field.information[2], "AMOUNT: USD 1000.00");
    }

    #[test]
    fn test_field76_line_too_long() {
        let long_line = "THIS LINE IS MUCH TOO LONG TO BE ACCEPTED IN FIELD 76";
        assert!(Field76::parse(long_line).is_err());
    }

    #[test]
    fn test_field76_max_lines() {
        let input = "LINE 1\nLINE 2\nLINE 3\nLINE 4\nLINE 5\nLINE 6";
        let field = Field76::parse(input).unwrap();
        assert_eq!(field.information.len(), 6);
    }

    #[test]
    fn test_field76_empty_input() {
        assert!(Field76::parse("").is_err());
    }

    #[test]
    fn test_field76_to_swift_string() {
        let field = Field76 {
            information: vec![
                "STATUS: COMPLETED".to_string(),
                "DATE: 2023-12-25".to_string(),
            ],
        };
        assert_eq!(
            field.to_swift_string(),
            ":76:STATUS: COMPLETED\nDATE: 2023-12-25"
        );
    }

    #[test]
    fn test_field76_single_line_to_swift_string() {
        let field = Field76 {
            information: vec!["ANSWER: CONFIRMED".to_string()],
        };
        assert_eq!(field.to_swift_string(), ":76:ANSWER: CONFIRMED");
    }
}
