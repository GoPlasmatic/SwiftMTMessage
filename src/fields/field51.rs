use super::field_utils::parse_party_identifier;
use super::swift_utils::parse_bic;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 51A: Sending Institution**
///
/// Identifies the message sender in FileAct messages and specialized contexts.
///
/// **Format:** `[/1!a][/34x]` + BIC (8 or 11 chars)
///
/// **Example:**
/// ```text
/// :51A:DEUTDEFFXXX
/// :51A:/D/12345678
/// CHASUS33XXX
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field51A {
    /// Optional party identifier (max 34 chars, clearing/account ref)
    pub party_identifier: Option<String>,

    /// BIC code (8 or 11 chars)
    pub bic: String,
}

impl SwiftField for Field51A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut remaining = input;
        let mut party_identifier = None;

        // Check for optional party identifier on first line
        if let Some(newline_pos) = input.find('\n') {
            let first_line = &input[..newline_pos];
            if let Some(id) = parse_party_identifier(first_line)? {
                party_identifier = Some(format!("/{}", id));
                remaining = &input[newline_pos + 1..];
            }
        }

        // If no party identifier found, check if entire input starts with '/'
        if party_identifier.is_none() && input.starts_with('/') {
            // This might be a party identifier without BIC
            if input.len() <= 36 && !input.contains('\n') {
                return Err(ParseError::InvalidFormat {
                    message: "Field 51A requires BIC code after party identifier".to_string(),
                });
            }
        }

        // Parse BIC code
        let bic = parse_bic(remaining)?;

        Ok(Field51A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            result.push('\n');
        }
        result.push_str(&self.bic);
        format!(":51A:{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field51a_valid() {
        // Without party identifier
        let field = Field51A::parse("DEUTDEFFXXX").unwrap();
        assert_eq!(field.bic, "DEUTDEFFXXX");
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.to_swift_string(), ":51A:DEUTDEFFXXX");

        // With party identifier
        let field = Field51A::parse("/D/12345678\nCHASUS33XXX").unwrap();
        assert_eq!(field.bic, "CHASUS33XXX");
        assert_eq!(field.party_identifier, Some("/D/12345678".to_string()));
        assert_eq!(field.to_swift_string(), ":51A:/D/12345678\nCHASUS33XXX");

        // 8-character BIC
        let field = Field51A::parse("MIDLGB22").unwrap();
        assert_eq!(field.bic, "MIDLGB22");
        assert_eq!(field.party_identifier, None);
    }

    #[test]
    fn test_field51a_invalid() {
        // Invalid BIC
        assert!(Field51A::parse("INVALID").is_err());

        // Party identifier without BIC
        assert!(Field51A::parse("/D/12345678").is_err());

        // Invalid characters in BIC
        assert!(Field51A::parse("DEUT@EFF").is_err());
    }
}
