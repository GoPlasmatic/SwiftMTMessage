use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 58A: Beneficiary Institution (BIC with Party Identifier)**
///
/// Specifies beneficiary institution for institutional transfers.
///
/// **Format:** [/1!a][/34x] + BIC (8 or 11 chars)
/// **Usage:** Central bank operations, institutional transfers, market infrastructure
///
/// **Example:**
/// ```text
/// :58A:DEUTDEFF
/// :58A:/CHGS123456
/// DEUTDEFF
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field58A {
    /// Optional party identifier (max 34 chars, institutional account)
    pub party_identifier: Option<String>,

    /// BIC code (8 or 11 chars)
    pub bic: String,
}

/// **Field 58D: Beneficiary Institution (Party Identifier with Name and Address)**
///
/// Detailed institutional beneficiary identification with name and address.
///
/// **Format:** [/1!a][/34x] + 4*35x
/// **Extended Clearing Codes:** CH (CHIPS), CP (CHIPS Participant), FW (Fedwire), RU (Russian), SW (Swiss)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field58D {
    /// Optional party identifier (max 34 chars, may include CH, CP, FW, RU, SW codes)
    pub party_identifier: Option<String>,

    /// Name and address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field58 {
    #[serde(rename = "58A")]
    A(Field58A),
    #[serde(rename = "58D")]
    D(Field58D),
}

impl SwiftField for Field58A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        let mut party_identifier = None;
        let mut bic_line_idx = 0;

        // Check for optional party identifier on first line
        if !lines.is_empty() && lines[0].starts_with('/') {
            party_identifier = Some(lines[0][1..].to_string()); // Strip the leading / (format prefix)
            bic_line_idx = 1;
        }

        // Ensure we have a BIC line
        if bic_line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 58A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field58A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = ":58A:".to_string();

        if let Some(ref party_id) = self.party_identifier {
            result.push('/'); // Add format prefix
            result.push_str(party_id);
            result.push('\n');
        }

        result.push_str(&self.bic);
        result
    }
}

impl SwiftField for Field58D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut lines = input.lines().collect::<Vec<_>>();
        let mut party_identifier = None;

        // Check if first line is a party identifier
        // Party identifier can be on its own line (starting with /)
        // If first line is short and there are more lines, it's likely a party identifier
        if let Some(first_line) = lines.first() {
            // Party identifier should start with / and be short (≤35 chars to account for the /)
            if first_line.starts_with('/') && first_line.len() <= 35 && lines.len() > 1 {
                // Entire first line is party identifier (strip the leading / format prefix)
                party_identifier = Some(first_line[1..].to_string());
                lines.remove(0);
            }
        }

        // Parse name and address lines (max 4 lines, max 35 chars each)
        let mut name_and_address = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i >= 4 {
                break;
            }
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 58D line {} exceeds 35 characters", i + 1),
                });
            }
            parse_swift_chars(line, &format!("Field 58D line {}", i + 1))?;
            name_and_address.push(line.to_string());
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 58D must contain name and address information".to_string(),
            });
        }

        Ok(Field58D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = ":58D:".to_string();

        if let Some(ref party_id) = self.party_identifier {
            result.push('/'); // Add format prefix
            result.push_str(party_id);
            result.push('\n');
        }

        result.push_str(&self.name_and_address.join("\n"));
        result
    }
}

impl SwiftField for Field58 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try parsing as Field58A first (BIC-based)
        if let Ok(field) = Field58A::parse(input) {
            return Ok(Field58::A(field));
        }

        // Try parsing as Field58D (name and address)
        if let Ok(field) = Field58D::parse(input) {
            return Ok(Field58::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 58 could not be parsed as either option A or D".to_string(),
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
                let field = Field58A::parse(value)?;
                Ok(Field58::A(field))
            }
            Some("D") => {
                let field = Field58D::parse(value)?;
                Ok(Field58::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field58::A(field) => field.to_swift_string(),
            Field58::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field58::A(_) => Some("A"),
            Field58::D(_) => Some("D"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field58a_parse_with_bic_only() {
        let field = Field58A::parse("DEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.bic, "DEUTDEFF");
    }

    #[test]
    fn test_field58a_parse_with_party_identifier() {
        let field = Field58A::parse("/CHGS123456\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("CHGS123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");
    }

    #[test]
    fn test_field58a_to_swift_string() {
        let field = Field58A {
            party_identifier: Some("CHGS123456".to_string()),
            bic: "DEUTDEFF".to_string(),
        };
        assert_eq!(field.to_swift_string(), ":58A:/CHGS123456\nDEUTDEFF");
    }

    #[test]
    fn test_field58d_parse_with_name_only() {
        let input = "DEUTSCHE BANK AG\nFRANKFURT AM MAIN\nGERMANY";
        let field = Field58D::parse(input).unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 3);
        assert_eq!(field.name_and_address[0], "DEUTSCHE BANK AG");
        assert_eq!(field.name_and_address[1], "FRANKFURT AM MAIN");
        assert_eq!(field.name_and_address[2], "GERMANY");
    }

    #[test]
    fn test_field58d_parse_with_party_identifier() {
        let input = "/CH123456\nDEUTSCHE BANK AG\nFRANKFURT AM MAIN";
        let field = Field58D::parse(input).unwrap();
        assert_eq!(field.party_identifier, Some("CH123456".to_string()));
        assert_eq!(field.name_and_address.len(), 2);
        assert_eq!(field.name_and_address[0], "DEUTSCHE BANK AG");
        assert_eq!(field.name_and_address[1], "FRANKFURT AM MAIN");
    }

    #[test]
    fn test_field58d_line_too_long() {
        let input = "THIS BANK NAME IS MUCH TOO LONG TO BE ACCEPTED IN FIELD 58D";
        assert!(Field58D::parse(input).is_err());
    }

    #[test]
    fn test_field58_enum_to_swift_string() {
        let field_a = Field58::A(Field58A {
            party_identifier: None,
            bic: "DEUTDEFF".to_string(),
        });
        assert_eq!(field_a.to_swift_string(), ":58A:DEUTDEFF");

        let field_d = Field58::D(Field58D {
            party_identifier: None,
            name_and_address: vec!["DEUTSCHE BANK AG".to_string()],
        });
        assert_eq!(field_d.to_swift_string(), ":58D:DEUTSCHE BANK AG");
    }
}
