use super::field_utils::parse_multiline_text;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 72: Sender to Receiver Information**
///
/// Additional information for receiver or other parties in financial messages,
/// enabling institutional coordination and processing instructions.
///
/// **Format:** `6*35x` (max 6 lines, 35 chars each)
/// **Common codes:** `/ACC/` (account), `/BNF/` (beneficiary), `/INST/` (instruction), `/INS/` (instructing institution)
///
/// **Example:**
/// ```text
/// :72:/BNF/BENEFICIARY DETAILS
/// /INST/CREDIT ACCOUNT IMMEDIATELY
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field72 {
    /// Sender to receiver information (max 6 lines, 35 chars each)
    pub information: Vec<String>,
}

impl SwiftField for Field72 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Parse as multiline text (up to 6 lines, 35 chars each)
        let information = parse_multiline_text(input, 6, 35)?;

        if information.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 72 must have at least one line of information".to_string(),
            });
        }

        Ok(Field72 { information })
    }

    fn to_swift_string(&self) -> String {
        format!(":72:{}", self.information.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field72_single_line() {
        let field = Field72::parse("/BNF/BENEFICIARY DETAILS").unwrap();
        assert_eq!(field.information.len(), 1);
        assert_eq!(field.information[0], "/BNF/BENEFICIARY DETAILS");
    }

    #[test]
    fn test_field72_multiline() {
        let input = "/BNF/BENEFICIARY DETAILS\n/ACC/ACCOUNT INFO\n/REC/RECEIVER INFO";
        let field = Field72::parse(input).unwrap();
        assert_eq!(field.information.len(), 3);
        assert_eq!(field.information[0], "/BNF/BENEFICIARY DETAILS");
        assert_eq!(field.information[1], "/ACC/ACCOUNT INFO");
        assert_eq!(field.information[2], "/REC/RECEIVER INFO");
    }

    #[test]
    fn test_field72_continuation() {
        let input = "/INST/LONG INSTRUCTION THAT\n//CONTINUES ON NEXT LINE\n//AND ANOTHER LINE";
        let field = Field72::parse(input).unwrap();
        assert_eq!(field.information.len(), 3);
        assert!(field.information[1].starts_with("//"));
        assert!(field.information[2].starts_with("//"));
    }

    #[test]
    fn test_field72_to_swift_string() {
        let field = Field72 {
            information: vec!["/CODE1/INFO1".to_string(), "/CODE2/INFO2".to_string()],
        };
        assert_eq!(field.to_swift_string(), ":72:/CODE1/INFO1\n/CODE2/INFO2");
    }

    #[test]
    fn test_field72_max_lines() {
        let input = "LINE1\nLINE2\nLINE3\nLINE4\nLINE5\nLINE6";
        let field = Field72::parse(input).unwrap();
        assert_eq!(field.information.len(), 6);

        // Test that 7 lines would fail
        let too_many = "LINE1\nLINE2\nLINE3\nLINE4\nLINE5\nLINE6\nLINE7";
        assert!(Field72::parse(too_many).is_err());
    }

    #[test]
    fn test_field72_line_length() {
        // Test max length line (35 chars)
        let max_line = format!("/ACC/{}", "A".repeat(30));
        assert_eq!(max_line.len(), 35);
        let field = Field72::parse(&max_line).unwrap();
        assert_eq!(field.information[0].len(), 35);

        // Test too long line (36 chars)
        let too_long = "A".repeat(36);
        assert!(Field72::parse(&too_long).is_err());
    }
}
