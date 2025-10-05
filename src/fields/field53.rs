use super::field_utils::parse_party_identifier;
use super::swift_utils::{parse_bic, parse_max_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 53A: Sender's Correspondent (BIC with Party Identifier)**
///
/// Specifies the sender's correspondent bank for reimbursement.
/// Format: [/1!a][/34x] + BIC (8 or 11 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field53A {
    /// Optional party identifier (max 34 chars, nostro account ref)
    pub party_identifier: Option<String>,

    /// BIC code (8 or 11 chars)
    pub bic: String,
}

impl SwiftField for Field53A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.split('\n').collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 53A requires input".to_string(),
            });
        }

        let mut line_idx = 0;
        let mut party_identifier = None;

        // Check for optional party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(format!("/{}", party_id));
            line_idx = 1;
        }

        // Ensure we have a BIC line
        if line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 53A requires BIC code after party identifier".to_string(),
            });
        }

        // Parse BIC code
        let bic = parse_bic(lines[line_idx])?;

        Ok(Field53A {
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
        format!(":53A:{}", result)
    }
}

/// **Field 53B: Sender's Correspondent (Party Identifier with Location)**
///
/// Domestic correspondent routing with party identifier and location.
/// Format: [/1!a][/34x] + [35x]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field53B {
    /// Optional party identifier (max 34 chars, nostro account)
    pub party_identifier: Option<String>,

    /// Location (max 35 chars)
    pub location: Option<String>,
}

impl SwiftField for Field53B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Ok(Field53B {
                party_identifier: None,
                location: None,
            });
        }

        let lines: Vec<&str> = input.split('\n').collect();
        let mut party_identifier = None;
        let mut location = None;

        // Field 53B format:
        // If 2 lines: Line 1 = party_identifier, Line 2 = location
        // If 1 line: Could be party_identifier OR location - use heuristics:
        //   - Starts with '/' -> party_identifier
        //   - Looks like BIC (8-11 uppercase alphanumeric) -> party_identifier
        //   - Otherwise -> location
        if lines.len() >= 2 {
            // Two lines: first is party_identifier, second is location
            if !lines[0].is_empty() {
                party_identifier =
                    Some(parse_max_length(lines[0], 34, "Field53B party_identifier")?);
            }
            if !lines[1].is_empty() {
                location = Some(parse_max_length(lines[1], 35, "Field53B location")?);
            }
        } else if lines.len() == 1 && !lines[0].is_empty() {
            let line = lines[0];

            // Determine if single line is party_identifier or location
            let is_party_identifier = line.starts_with('/')
                || ((8..=11).contains(&line.len())
                    && line
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()));

            if is_party_identifier {
                party_identifier = Some(parse_max_length(line, 34, "Field53B party_identifier")?);
            } else {
                location = Some(parse_max_length(line, 35, "Field53B location")?);
            }
        }

        Ok(Field53B {
            party_identifier,
            location,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            if self.location.is_some() {
                result.push('\n');
            }
        }
        if let Some(ref loc) = self.location {
            result.push_str(loc);
        }
        format!(":53B:{}", result)
    }
}

/// **Field 53D: Sender's Correspondent (Party Identifier with Name and Address)**
///
/// Detailed correspondent identification with name and address.
/// Format: [/1!a][/34x] + 4*35x
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field53D {
    /// Optional party identifier (max 34 chars, nostro account)
    pub party_identifier: Option<String>,

    /// Name and address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field53D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut lines = input.split('\n').collect::<Vec<_>>();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 53D requires at least one line".to_string(),
            });
        }

        let mut party_identifier = None;

        // Check if first line is a party identifier
        // Party identifier can be on its own line (with or without leading /)
        // If first line starts with '/' and is short, it's a party identifier
        if let Some(first_line) = lines.first() {
            // If it starts with '/' or looks like an account identifier (short alphanumeric)
            let looks_like_party_id = first_line.starts_with('/')
                || (first_line.len() <= 34
                    && !first_line.contains(' ')
                    && first_line.chars().any(|c| c.is_ascii_digit()));

            if looks_like_party_id && !first_line.is_empty() && lines.len() > 1 {
                // Entire first line is party identifier
                party_identifier = Some(first_line.to_string());
                lines.remove(0);
            }
        }

        // Parse remaining lines as name and address (max 4 lines, max 35 chars each)
        let mut name_and_address = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i >= 4 {
                break;
            }
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 53D line {} exceeds 35 characters", i + 1),
                });
            }
            name_and_address.push(line.to_string());
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 53D must contain name and address information".to_string(),
            });
        }

        Ok(Field53D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            result.push('\n');
        }
        for (i, line) in self.name_and_address.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }
            result.push_str(line);
        }
        format!(":53D:{}", result)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field53SenderCorrespondent {
    #[serde(rename = "53A")]
    A(Field53A),
    #[serde(rename = "53B")]
    B(Field53B),
    #[serde(rename = "53D")]
    D(Field53D),
}

impl SwiftField for Field53SenderCorrespondent {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try parsing as each variant
        // A: Has BIC code (8 or 11 uppercase letters/digits)
        // B: Has optional party identifier and/or location
        // D: Has party identifier and/or multiple lines of name/address

        let lines: Vec<&str> = input.split('\n').collect();
        let last_line = lines.last().unwrap_or(&"");

        // Check if last line looks like a BIC code
        if (8..=11).contains(&last_line.len())
            && last_line
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            // Try parsing as Field53A
            if let Ok(field) = Field53A::parse(input) {
                return Ok(Field53SenderCorrespondent::A(field));
            }
        }

        // Check for multiple lines suggesting D format
        if lines.len() > 2 || (lines.len() == 2 && !lines[0].starts_with('/')) {
            // Try parsing as Field53D (multiple lines of name/address)
            if let Ok(field) = Field53D::parse(input) {
                return Ok(Field53SenderCorrespondent::D(field));
            }
        }

        // Try parsing as Field53B (simpler format)
        if let Ok(field) = Field53B::parse(input) {
            return Ok(Field53SenderCorrespondent::B(field));
        }

        // If all fail, try in order
        if let Ok(field) = Field53A::parse(input) {
            return Ok(Field53SenderCorrespondent::A(field));
        }
        if let Ok(field) = Field53D::parse(input) {
            return Ok(Field53SenderCorrespondent::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 53 could not be parsed as any valid option (A, B, or D)".to_string(),
        })
    }

    fn parse_with_variant(
        value: &str,
        variant: Option<&str>,
        _field_tag: Option<&str>,
    ) -> crate::Result<Self>
    where
        Self: Sized,
    {
        match variant {
            Some("A") => {
                let field = Field53A::parse(value)?;
                Ok(Field53SenderCorrespondent::A(field))
            }
            Some("B") => {
                let field = Field53B::parse(value)?;
                Ok(Field53SenderCorrespondent::B(field))
            }
            Some("D") => {
                let field = Field53D::parse(value)?;
                Ok(Field53SenderCorrespondent::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field53SenderCorrespondent::A(field) => field.to_swift_string(),
            Field53SenderCorrespondent::B(field) => field.to_swift_string(),
            Field53SenderCorrespondent::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field53SenderCorrespondent::A(_) => Some("A"),
            Field53SenderCorrespondent::B(_) => Some("B"),
            Field53SenderCorrespondent::D(_) => Some("D"),
        }
    }
}

// Type alias for backward compatibility and simplicity
pub type Field53 = Field53SenderCorrespondent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field53a_valid() {
        // Without party identifier
        let field = Field53A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.bic, "CHASUS33XXX");
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.to_swift_string(), ":53A:CHASUS33XXX");

        // With party identifier
        let field = Field53A::parse("/C/12345678\nCHASUS33").unwrap();
        assert_eq!(field.bic, "CHASUS33");
        assert_eq!(field.party_identifier, Some("/C/12345678".to_string()));
        assert_eq!(field.to_swift_string(), ":53A:/C/12345678\nCHASUS33");
    }

    #[test]
    fn test_field53b_valid() {
        // Only location
        let field = Field53B::parse("NEW YORK BRANCH").unwrap();
        assert_eq!(field.location, Some("NEW YORK BRANCH".to_string()));
        assert_eq!(field.party_identifier, None);

        // With party identifier
        let field = Field53B::parse("/D/98765432\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("/D/98765432".to_string()));
        assert_eq!(field.location, Some("NEW YORK".to_string()));

        // Empty
        let field = Field53B::parse("").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.location, None);
    }

    #[test]
    fn test_field53d_valid() {
        // With party identifier and name/address
        let field =
            Field53D::parse("/C/12345678\nCORRESPONDENT BANK\n123 MAIN ST\nNEW YORK\nUSA").unwrap();
        assert_eq!(field.party_identifier, Some("/C/12345678".to_string()));
        assert_eq!(field.name_and_address.len(), 4);
        assert_eq!(field.name_and_address[0], "CORRESPONDENT BANK");
        assert_eq!(field.name_and_address[3], "USA");

        // Without party identifier
        let field = Field53D::parse("CORRESPONDENT BANK\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field53_enum() {
        // Parse as A
        let field = Field53SenderCorrespondent::parse("CHASUS33XXX").unwrap();
        assert!(matches!(field, Field53SenderCorrespondent::A(_)));

        // Parse as B
        let field = Field53SenderCorrespondent::parse("NEW YORK BRANCH").unwrap();
        assert!(matches!(field, Field53SenderCorrespondent::B(_)));

        // Parse as D
        let field = Field53SenderCorrespondent::parse("BANK NAME\nADDRESS LINE 1\nCITY").unwrap();
        assert!(matches!(field, Field53SenderCorrespondent::D(_)));
    }
}
