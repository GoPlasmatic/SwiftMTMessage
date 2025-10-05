use super::swift_utils::parse_swift_chars;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 76: Answers**
///
/// Answer information in MT n96 series messages providing responses to Field 75 queries,
/// including status updates and clarifications.
///
/// **Format:** `6*35x` (max 6 lines, 35 chars each)
/// **Used in:** MT 196, 296, 396, 496, 596, 696, 796, 896, 996 (answer messages)
///
/// **Example:**
/// ```text
/// :76:STATUS: COMPLETED
/// SETTLEMENT DATE: 2023-12-25
/// AMOUNT: USD 1000.00
/// ```

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field76 {
    /// Answer information (max 6 lines, 35 chars each)
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
