use super::swift_utils::parse_swift_chars;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 86: Information to Account Owner**
///
/// Additional information to account owner regarding transactions or account activities
/// in statement messages.
///
/// **Format:** `6*65x` (max 6 lines, 65 chars each)
/// **Used in:** MT 940 (customer statement), MT 942 (interim transaction report)
///
/// **Example:**
/// ```text
/// :86:WIRE TRANSFER RECEIVED
/// FROM: INTERNATIONAL BANK
/// PURPOSE: TRADE SETTLEMENT
/// ```

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field86 {
    /// Information narrative for account owner (max 6 lines, 65 chars each)
    pub narrative: Vec<String>,
}

impl SwiftField for Field86 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut lines = Vec::new();

        // Parse up to 6 lines of 65 characters each
        for line in input.lines().take(6) {
            // Validate line length (max 65 characters)
            if line.len() > 65 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 86 line exceeds 65 characters: {}", line.len()),
                });
            }

            // Validate SWIFT character set
            parse_swift_chars(line, "Field 86 line")?;

            lines.push(line.to_string());
        }

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 86 must contain at least one line".to_string(),
            });
        }

        Ok(Field86 { narrative: lines })
    }

    fn to_swift_string(&self) -> String {
        let content = self.narrative.join("\n");
        format!(":86:{}", content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field86_parse_single_line() {
        let field = Field86::parse("PAYMENT RECEIVED FROM ABC COMPANY FOR INVOICE 12345").unwrap();
        assert_eq!(field.narrative.len(), 1);
        assert_eq!(
            field.narrative[0],
            "PAYMENT RECEIVED FROM ABC COMPANY FOR INVOICE 12345"
        );
    }

    #[test]
    fn test_field86_parse_multiple_lines() {
        let input = "TRANSACTION DETAILS:\nCOUNTERPARTY: ABC CORP\nREFERENCE: PAY-2023-12345\nDESCRIPTION: MONTHLY SERVICE FEE";
        let field = Field86::parse(input).unwrap();
        assert_eq!(field.narrative.len(), 4);
        assert_eq!(field.narrative[0], "TRANSACTION DETAILS:");
        assert_eq!(field.narrative[1], "COUNTERPARTY: ABC CORP");
        assert_eq!(field.narrative[2], "REFERENCE: PAY-2023-12345");
        assert_eq!(field.narrative[3], "DESCRIPTION: MONTHLY SERVICE FEE");
    }

    #[test]
    fn test_field86_line_too_long() {
        let long_line = "THIS LINE IS MUCH TOO LONG TO BE ACCEPTED IN FIELD 86 AS IT EXCEEDS THE 65 CHARACTER LIMIT SIGNIFICANTLY";
        assert!(Field86::parse(long_line).is_err());
    }

    #[test]
    fn test_field86_max_line_length() {
        // Exactly 65 characters should work
        let line_65_chars = "12345678901234567890123456789012345678901234567890123456789012345";
        let field = Field86::parse(line_65_chars).unwrap();
        assert_eq!(field.narrative[0], line_65_chars);
    }

    #[test]
    fn test_field86_max_lines() {
        let input = "LINE 1\nLINE 2\nLINE 3\nLINE 4\nLINE 5\nLINE 6";
        let field = Field86::parse(input).unwrap();
        assert_eq!(field.narrative.len(), 6);
        assert_eq!(field.narrative[0], "LINE 1");
        assert_eq!(field.narrative[5], "LINE 6");
    }

    #[test]
    fn test_field86_empty_input() {
        assert!(Field86::parse("").is_err());
    }

    #[test]
    fn test_field86_to_swift_string() {
        let field = Field86 {
            narrative: vec![
                "WIRE TRANSFER RECEIVED".to_string(),
                "FROM: INTERNATIONAL BANK".to_string(),
                "PURPOSE: TRADE SETTLEMENT".to_string(),
            ],
        };
        let expected =
            ":86:WIRE TRANSFER RECEIVED\nFROM: INTERNATIONAL BANK\nPURPOSE: TRADE SETTLEMENT";
        assert_eq!(field.to_swift_string(), expected);
    }

    #[test]
    fn test_field86_single_line_to_swift_string() {
        let field = Field86 {
            narrative: vec!["DIRECT DEBIT AUTHORIZATION PAYMENT".to_string()],
        };
        assert_eq!(
            field.to_swift_string(),
            ":86:DIRECT DEBIT AUTHORIZATION PAYMENT"
        );
    }

    #[test]
    fn test_field86_account_holder_information() {
        let field = Field86 {
            narrative: vec![
                "ACCOUNT BALANCE UPDATE".to_string(),
                "INTEREST CREDITED: USD 125.50".to_string(),
                "ANNUAL RATE: 2.5%".to_string(),
                "CALCULATION PERIOD: Q4 2023".to_string(),
            ],
        };

        let expected = ":86:ACCOUNT BALANCE UPDATE\nINTEREST CREDITED: USD 125.50\nANNUAL RATE: 2.5%\nCALCULATION PERIOD: Q4 2023";
        assert_eq!(field.to_swift_string(), expected);
        assert_eq!(field.narrative.len(), 4);
    }
}
