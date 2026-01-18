use super::field_utils::parse_multiline_text;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 70: Remittance Information**
///
/// Payment details and references transmitted to beneficiary for
/// transaction identification and reconciliation.
///
/// **Format:** `4*35x` (max 4 lines, 35 chars each)
/// **Common codes:** `/INV/` (invoice), `/RFB/` (reference for beneficiary), `/ROC/` (reference of customer)
///
/// **Example:**
/// ```text
/// :70:/INV/20231215/INV-12345
/// PAYMENT FOR SERVICES
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field70 {
    /// Remittance narrative (max 4 lines, 35 chars each)
    pub narrative: Vec<String>,
}

impl SwiftField for Field70 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Parse as multiline text (up to 4 lines, 35 chars each)
        let narrative = parse_multiline_text(input, 4, 35)?;

        if narrative.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 70 must have at least one line of narrative".to_string(),
            });
        }

        Ok(Field70 { narrative })
    }

    fn to_swift_string(&self) -> String {
        format!(":70:{}", self.narrative.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field70_single_line() {
        let field = Field70::parse("PAYMENT FOR INVOICE 12345").unwrap();
        assert_eq!(field.narrative.len(), 1);
        assert_eq!(field.narrative[0], "PAYMENT FOR INVOICE 12345");
    }

    #[test]
    fn test_field70_multiline() {
        let input = "/INV/123456\nPAYMENT FOR GOODS\nDELIVERED ON 2024-07-19\nREF: CONTRACT-001";
        let field = Field70::parse(input).unwrap();
        assert_eq!(field.narrative.len(), 4);
        assert_eq!(field.narrative[0], "/INV/123456");
        assert_eq!(field.narrative[1], "PAYMENT FOR GOODS");
        assert_eq!(field.narrative[2], "DELIVERED ON 2024-07-19");
        assert_eq!(field.narrative[3], "REF: CONTRACT-001");
    }

    #[test]
    fn test_field70_to_swift_string() {
        let field = Field70 {
            narrative: vec!["LINE ONE".to_string(), "LINE TWO".to_string()],
        };
        assert_eq!(field.to_swift_string(), ":70:LINE ONE\nLINE TWO");
    }

    #[test]
    fn test_field70_max_lines() {
        let input = "LINE1\nLINE2\nLINE3\nLINE4";
        let field = Field70::parse(input).unwrap();
        assert_eq!(field.narrative.len(), 4);

        // Test that 5 lines would fail
        let too_many = "LINE1\nLINE2\nLINE3\nLINE4\nLINE5";
        assert!(Field70::parse(too_many).is_err());
    }

    #[test]
    fn test_field70_line_length() {
        // Test max length line (35 chars)
        let max_line = "A".repeat(35);
        let field = Field70::parse(&max_line).unwrap();
        assert_eq!(field.narrative[0].len(), 35);

        // Test too long line (36 chars)
        let too_long = "A".repeat(36);
        assert!(Field70::parse(&too_long).is_err());
    }
}
