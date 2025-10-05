use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_max_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 54A: Receiver's Correspondent (BIC with Party Identifier)**
///
/// Specifies the receiver's correspondent bank for fund availability.
/// Format: [/1!a][/34x] + BIC (8 or 11 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field54A {
    /// Optional party identifier (max 34 chars, vostro account ref)
    pub party_identifier: Option<String>,

    /// BIC code (8 or 11 chars)
    pub bic: String,
}

impl SwiftField for Field54A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.split('\n').collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 54A requires input".to_string(),
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
                message: "Field 54A requires BIC code after party identifier".to_string(),
            });
        }

        // Parse BIC code
        let bic = parse_bic(lines[line_idx])?;

        Ok(Field54A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":54A:");
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            result.push('\n');
        }
        result.push_str(&self.bic);
        result
    }
}

/// **Field 54B: Receiver's Correspondent (Party Identifier with Location)**
///
/// Domestic receiver correspondent routing with party identifier and location.
/// Format: [/1!a][/34x] + [35x]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field54B {
    /// Optional party identifier (max 34 chars, vostro account)
    pub party_identifier: Option<String>,

    /// Location (max 35 chars)
    pub location: Option<String>,
}

impl SwiftField for Field54B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Ok(Field54B {
                party_identifier: None,
                location: None,
            });
        }

        let lines: Vec<&str> = input.split('\n').collect();
        let mut party_identifier = None;
        let mut location = None;
        let mut line_idx = 0;

        // Check for party identifier on first line
        if !lines.is_empty() && lines[0].starts_with('/') {
            party_identifier = Some(lines[0].to_string());
            line_idx = 1;
        }

        // Remaining line is location
        if line_idx < lines.len() && !lines[line_idx].is_empty() {
            location = Some(parse_max_length(lines[line_idx], 35, "Field54B location")?);
        }

        Ok(Field54B {
            party_identifier,
            location,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":54B:");
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            if self.location.is_some() {
                result.push('\n');
            }
        }
        if let Some(ref loc) = self.location {
            result.push_str(loc);
        }
        result
    }
}

/// **Field 54D: Receiver's Correspondent (Party Identifier with Name and Address)**
///
/// Detailed receiver correspondent identification with name and address.
/// Format: [/1!a][/34x] + 4*35x
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field54D {
    /// Optional party identifier (max 34 chars, vostro account)
    pub party_identifier: Option<String>,

    /// Name and address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field54D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.split('\n').collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 54D requires at least one line".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut start_idx = 0;

        // Check for party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(format!("/{}", party_id));
            start_idx = 1;
        }

        // Parse remaining lines as name and address
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field54D")?;

        Ok(Field54D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":54D:");
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
        result
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field54ReceiverCorrespondent {
    #[serde(rename = "54A")]
    A(Field54A),
    #[serde(rename = "54B")]
    B(Field54B),
    #[serde(rename = "54D")]
    D(Field54D),
}

impl SwiftField for Field54ReceiverCorrespondent {
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
            // Try parsing as Field54A
            if let Ok(field) = Field54A::parse(input) {
                return Ok(Field54ReceiverCorrespondent::A(field));
            }
        }

        // Check for multiple lines suggesting D format
        if lines.len() > 2 || (lines.len() == 2 && !lines[0].starts_with('/')) {
            // Try parsing as Field54D (multiple lines of name/address)
            if let Ok(field) = Field54D::parse(input) {
                return Ok(Field54ReceiverCorrespondent::D(field));
            }
        }

        // Try parsing as Field54B (simpler format)
        if let Ok(field) = Field54B::parse(input) {
            return Ok(Field54ReceiverCorrespondent::B(field));
        }

        // If all fail, try in order
        if let Ok(field) = Field54A::parse(input) {
            return Ok(Field54ReceiverCorrespondent::A(field));
        }
        if let Ok(field) = Field54D::parse(input) {
            return Ok(Field54ReceiverCorrespondent::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 54 could not be parsed as any valid option (A, B, or D)".to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field54ReceiverCorrespondent::A(field) => field.to_swift_string(),
            Field54ReceiverCorrespondent::B(field) => field.to_swift_string(),
            Field54ReceiverCorrespondent::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field54ReceiverCorrespondent::A(_) => Some("A"),
            Field54ReceiverCorrespondent::B(_) => Some("B"),
            Field54ReceiverCorrespondent::D(_) => Some("D"),
        }
    }
}

// Type alias for backward compatibility and simplicity
pub type Field54 = Field54ReceiverCorrespondent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field54a_valid() {
        // Without party identifier
        let field = Field54A::parse("DEUTDEFFXXX").unwrap();
        assert_eq!(field.bic, "DEUTDEFFXXX");
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.to_swift_string(), ":54A:DEUTDEFFXXX");

        // With party identifier
        let field = Field54A::parse("/A/987654321\nDEUTDEFF").unwrap();
        assert_eq!(field.bic, "DEUTDEFF");
        assert_eq!(field.party_identifier, Some("/A/987654321".to_string()));
        assert_eq!(field.to_swift_string(), ":54A:/A/987654321\nDEUTDEFF");
    }

    #[test]
    fn test_field54b_valid() {
        // Only location
        let field = Field54B::parse("FRANKFURT BRANCH").unwrap();
        assert_eq!(field.location, Some("FRANKFURT BRANCH".to_string()));
        assert_eq!(field.party_identifier, None);

        // With party identifier
        let field = Field54B::parse("/B/11223344\nFRANKFURT").unwrap();
        assert_eq!(field.party_identifier, Some("/B/11223344".to_string()));
        assert_eq!(field.location, Some("FRANKFURT".to_string()));

        // Empty
        let field = Field54B::parse("").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.location, None);
    }

    #[test]
    fn test_field54d_valid() {
        // With party identifier and name/address
        let field = Field54D::parse("/A/987654321\nRECEIVER BANK\n456 BANK ST\nFRANKFURT\nGERMANY")
            .unwrap();
        assert_eq!(field.party_identifier, Some("/A/987654321".to_string()));
        assert_eq!(field.name_and_address.len(), 4);
        assert_eq!(field.name_and_address[0], "RECEIVER BANK");
        assert_eq!(field.name_and_address[3], "GERMANY");

        // Without party identifier
        let field = Field54D::parse("RECEIVER BANK\nFRANKFURT").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field54_enum() {
        // Parse as A
        let field = Field54ReceiverCorrespondent::parse("DEUTDEFFXXX").unwrap();
        assert!(matches!(field, Field54ReceiverCorrespondent::A(_)));

        // Parse as B
        let field = Field54ReceiverCorrespondent::parse("FRANKFURT BRANCH").unwrap();
        assert!(matches!(field, Field54ReceiverCorrespondent::B(_)));

        // Parse as D
        let field = Field54ReceiverCorrespondent::parse("BANK NAME\nADDRESS LINE 1\nCITY").unwrap();
        assert!(matches!(field, Field54ReceiverCorrespondent::D(_)));
    }
}
