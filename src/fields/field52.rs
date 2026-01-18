use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 52A: Ordering Institution (BIC with Party Identifier)**
///
/// Identifies the ordering customer's bank when different from message sender.
/// Format: [/1!a][/34x] + BIC (8 or 11 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field52A {
    /// Optional party identifier (max 34 chars, clearing/account ref)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_identifier: Option<String>,

    /// BIC code (8 or 11 chars)
    pub bic: String,
}

impl SwiftField for Field52A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 52A cannot be empty".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut bic_line_idx = 0;

        // Check for optional party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(party_id);
            bic_line_idx = 1;
        }

        // Parse BIC
        if bic_line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 52A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field52A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut lines = Vec::new();

        if let Some(ref id) = self.party_identifier {
            lines.push(format!("/{}", id));
        }

        lines.push(self.bic.clone());
        format!(":52A:{}", lines.join("\n"))
    }
}

/// **Field 52B: Ordering Institution (Party Identifier with Location)**
///
/// Domestic routing using party identifier and location.
/// Format: [/1!a][/34x] + [35x]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field52B {
    /// Optional party identifier (max 34 chars)
    pub party_identifier: Option<String>,

    /// Location (max 35 chars)
    pub location: Option<String>,
}

impl SwiftField for Field52B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Ok(Field52B {
                party_identifier: None,
                location: None,
            });
        }

        let lines: Vec<&str> = input.lines().collect();
        let mut party_identifier = None;
        let mut location = None;
        let mut current_idx = 0;

        // Check for party identifier
        if !lines.is_empty() && lines[0].starts_with('/') {
            let line = &lines[0][1..]; // Remove leading /

            // Check if it's /1!a/34x format
            if let Some(slash_pos) = line.find('/') {
                let code = &line[..slash_pos];
                let id = &line[slash_pos + 1..];

                if code.len() == 1
                    && code.chars().all(|c| c.is_ascii_alphabetic())
                    && id.len() <= 34
                {
                    parse_swift_chars(id, "Field 52B party identifier")?;
                    party_identifier = Some(format!("{}/{}", code, id));
                    current_idx = 1;
                }
            } else if line.len() <= 34 {
                // Just /34x format
                parse_swift_chars(line, "Field 52B party identifier")?;
                party_identifier = Some(line.to_string());
                current_idx = 1;
            }
        }

        // Check for location
        if current_idx < lines.len() {
            let loc = lines[current_idx];
            if !loc.is_empty() && loc.len() <= 35 {
                parse_swift_chars(loc, "Field 52B location")?;
                location = Some(loc.to_string());
            }
        }

        Ok(Field52B {
            party_identifier,
            location,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(ref id) = self.party_identifier {
            result.push(format!("/{}", id));
        }

        if let Some(ref loc) = self.location {
            result.push(loc.clone());
        }

        format!(":52B:{}", result.join("\n"))
    }
}

/// **Field 52C: Ordering Institution (Party Identifier Only)**
///
/// Simplified institutional reference with party identifier only.
/// Format: /34x (mandatory slash prefix)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field52C {
    /// Party identifier (1-34 chars, domestic/clearing codes)
    pub party_identifier: String,
}

impl SwiftField for Field52C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if !input.starts_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 52C must start with '/'".to_string(),
            });
        }

        let identifier = &input[1..];

        if identifier.is_empty() || identifier.len() > 34 {
            return Err(ParseError::InvalidFormat {
                message: "Field 52C party identifier must be 1-34 characters".to_string(),
            });
        }

        parse_swift_chars(identifier, "Field 52C party identifier")?;

        Ok(Field52C {
            party_identifier: identifier.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":52C:/{}", self.party_identifier)
    }
}

/// **Field 52D: Ordering Institution (Party Identifier with Name and Address)**
///
/// Detailed institutional identification with full name and address.
/// Format: [/1!a][/34x] + 4*35x
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field52D {
    /// Optional party identifier (max 34 chars)
    pub party_identifier: Option<String>,

    /// Name and address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field52D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 52D must have at least one line".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut start_idx = 0;

        // Check for party identifier
        if lines[0].starts_with('/') {
            let line = &lines[0][1..]; // Remove leading /

            // Check if it's /1!a/34x format
            if let Some(slash_pos) = line.find('/') {
                let code = &line[..slash_pos];
                let id = &line[slash_pos + 1..];

                if code.len() == 1
                    && code.chars().all(|c| c.is_ascii_alphabetic())
                    && id.len() <= 34
                {
                    parse_swift_chars(id, "Field 52D party identifier")?;
                    party_identifier = Some(format!("{}/{}", code, id));
                    start_idx = 1;
                }
            } else if line.len() <= 34 {
                // Just /34x format
                parse_swift_chars(line, "Field 52D party identifier")?;
                party_identifier = Some(line.to_string());
                start_idx = 1;
            }
        }

        // Parse name and address lines
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field 52D")?;

        Ok(Field52D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut lines = Vec::new();

        if let Some(ref id) = self.party_identifier {
            lines.push(format!("/{}", id));
        }

        for line in &self.name_and_address {
            lines.push(line.clone());
        }

        format!(":52D:{}", lines.join("\n"))
    }
}

/// Enum for Field52 Account Servicing Institution variants (A, C)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field52AccountServicingInstitution {
    #[serde(rename = "52A")]
    A(Field52A),
    #[serde(rename = "52C")]
    C(Field52C),
}

impl SwiftField for Field52AccountServicingInstitution {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based)
        if let Ok(field) = Field52A::parse(input) {
            return Ok(Field52AccountServicingInstitution::A(field));
        }

        // Try Option C (party identifier only)
        if let Ok(field) = Field52C::parse(input) {
            return Ok(Field52AccountServicingInstitution::C(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 52 Account Servicing Institution could not be parsed as option A or C"
                .to_string(),
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
                let field = Field52A::parse(value)?;
                Ok(Field52AccountServicingInstitution::A(field))
            }
            Some("C") => {
                let field = Field52C::parse(value)?;
                Ok(Field52AccountServicingInstitution::C(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field52AccountServicingInstitution::A(field) => field.to_swift_string(),
            Field52AccountServicingInstitution::C(field) => field.to_swift_string(),
        }
    }
}

/// Enum for Field52 Ordering Institution variants (A, D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field52OrderingInstitution {
    #[serde(rename = "52A")]
    A(Field52A),
    #[serde(rename = "52D")]
    D(Field52D),
}

impl SwiftField for Field52OrderingInstitution {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field52A::parse(input) {
            return Ok(Field52OrderingInstitution::A(field));
        }

        // Try Option D (party identifier with name/address)
        if let Ok(field) = Field52D::parse(input) {
            return Ok(Field52OrderingInstitution::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 52 Ordering Institution could not be parsed as option A or D"
                .to_string(),
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
                let field = Field52A::parse(value)?;
                Ok(Field52OrderingInstitution::A(field))
            }
            Some("D") => {
                let field = Field52D::parse(value)?;
                Ok(Field52OrderingInstitution::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field52OrderingInstitution::A(field) => field.to_swift_string(),
            Field52OrderingInstitution::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field52OrderingInstitution::A(_) => Some("A"),
            Field52OrderingInstitution::D(_) => Some("D"),
        }
    }
}

/// Enum for Field52 Creditor Bank variants (A, C, D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field52CreditorBank {
    #[serde(rename = "52A")]
    A(Field52A),
    #[serde(rename = "52C")]
    C(Field52C),
    #[serde(rename = "52D")]
    D(Field52D),
}

impl SwiftField for Field52CreditorBank {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field52A::parse(input) {
            return Ok(Field52CreditorBank::A(field));
        }

        // Try Option C (party identifier only)
        if input.starts_with('/')
            && !input.contains('\n')
            && let Ok(field) = Field52C::parse(input)
        {
            return Ok(Field52CreditorBank::C(field));
        }

        // Try Option D (party identifier with name/address)
        if let Ok(field) = Field52D::parse(input) {
            return Ok(Field52CreditorBank::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 52 Creditor Bank could not be parsed as option A, C or D".to_string(),
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
                let field = Field52A::parse(value)?;
                Ok(Field52CreditorBank::A(field))
            }
            Some("C") => {
                let field = Field52C::parse(value)?;
                Ok(Field52CreditorBank::C(field))
            }
            Some("D") => {
                let field = Field52D::parse(value)?;
                Ok(Field52CreditorBank::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field52CreditorBank::A(field) => field.to_swift_string(),
            Field52CreditorBank::C(field) => field.to_swift_string(),
            Field52CreditorBank::D(field) => field.to_swift_string(),
        }
    }
}

/// Enum for Field52 Drawer Bank variants (A, B, D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field52DrawerBank {
    #[serde(rename = "52A")]
    A(Field52A),
    #[serde(rename = "52B")]
    B(Field52B),
    #[serde(rename = "52D")]
    D(Field52D),
}

impl SwiftField for Field52DrawerBank {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field52A::parse(input) {
            return Ok(Field52DrawerBank::A(field));
        }

        // Try Option B (party identifier with location)
        if let Ok(field) = Field52B::parse(input) {
            return Ok(Field52DrawerBank::B(field));
        }

        // Try Option D (party identifier with name/address)
        if let Ok(field) = Field52D::parse(input) {
            return Ok(Field52DrawerBank::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 52 Drawer Bank could not be parsed as option A, B or D".to_string(),
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
                let field = Field52A::parse(value)?;
                Ok(Field52DrawerBank::A(field))
            }
            Some("B") => {
                let field = Field52B::parse(value)?;
                Ok(Field52DrawerBank::B(field))
            }
            Some("D") => {
                let field = Field52D::parse(value)?;
                Ok(Field52DrawerBank::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field52DrawerBank::A(field) => field.to_swift_string(),
            Field52DrawerBank::B(field) => field.to_swift_string(),
            Field52DrawerBank::D(field) => field.to_swift_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field52a() {
        // With party identifier
        let field = Field52A::parse("/C/US123456\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("C/US123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");

        // Without party identifier
        let field = Field52A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.bic, "CHASUS33XXX");
    }

    #[test]
    fn test_field52b() {
        // With party identifier and location
        let field = Field52B::parse("/A/12345\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("A/12345".to_string()));
        assert_eq!(field.location, Some("NEW YORK".to_string()));

        // Empty
        let field = Field52B::parse("").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.location, None);
    }

    #[test]
    fn test_field52c() {
        let field = Field52C::parse("/UKCLEARING123").unwrap();
        assert_eq!(field.party_identifier, "UKCLEARING123");
        assert_eq!(field.to_swift_string(), ":52C:/UKCLEARING123");
    }

    #[test]
    fn test_field52d() {
        // With party identifier
        let field = Field52D::parse("/D/DE123456\nDEUTSCHE BANK\nFRANKFURT").unwrap();
        assert_eq!(field.party_identifier, Some("D/DE123456".to_string()));
        assert_eq!(field.name_and_address.len(), 2);
        assert_eq!(field.name_and_address[0], "DEUTSCHE BANK");

        // Without party identifier
        let field = Field52D::parse("ACME BANK\nLONDON").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field52_invalid() {
        // Invalid BIC
        assert!(Field52A::parse("INVALID").is_err());

        // Missing slash in 52C
        assert!(Field52C::parse("NOSLASH").is_err());

        // Too many lines in 52D
        assert!(Field52D::parse("LINE1\nLINE2\nLINE3\nLINE4\nLINE5").is_err());
    }
}
