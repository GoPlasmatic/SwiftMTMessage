use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 57A: Account With Institution (BIC with Party Identifier)**
///
/// Specifies beneficiary's bank where account is maintained.
///
/// **Format:** [/1!a][/34x] + BIC (8 or 11 chars)
/// **Payment Method Codes:** //FW (Fedwire), //RT (RTGS), //AU, //IN
///
/// **Example:**
/// ```text
/// :57A:CHASUS33XXX
/// :57A://FW
/// DEUTDEFF
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field57A {
    /// Optional party identifier (max 34 chars, may include //FW, //RT, //AU, //IN codes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_identifier: Option<String>,

    /// BIC code (8 or 11 chars)
    pub bic: String,
}

impl SwiftField for Field57A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 57A cannot be empty".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut bic_line_idx = 0;

        // Check for optional party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(party_id);
            bic_line_idx = 1;
        }

        // Ensure we have a BIC line
        if bic_line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 57A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field57A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(ref id) = self.party_identifier {
            result.push(format!("/{}", id));
        }

        result.push(self.bic.clone());
        format!(":57A:{}", result.join("\n"))
    }
}

/// **Field 57B: Account With Institution (Party Identifier with Location)**
///
/// Domestic routing with party identifier and location.
/// Format: [/1!a][/34x] + [35x]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field57B {
    /// Optional party identifier (max 34 chars)
    pub party_identifier: Option<String>,

    /// Location (max 35 chars)
    pub location: Option<String>,
}

impl SwiftField for Field57B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Ok(Field57B {
                party_identifier: None,
                location: None,
            });
        }

        let lines: Vec<&str> = input.lines().collect();
        let mut party_identifier = None;
        let mut location = None;
        let mut current_idx = 0;

        // Check for party identifier
        if !lines.is_empty()
            && let Some(party_id) = parse_party_identifier(lines[0])?
        {
            party_identifier = Some(party_id);
            current_idx = 1;
        }

        // Check for location
        if current_idx < lines.len() {
            let loc = lines[current_idx];
            if !loc.is_empty() && loc.len() <= 35 {
                parse_swift_chars(loc, "Field 57B location")?;
                location = Some(loc.to_string());
            }
        }

        Ok(Field57B {
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

        format!(":57B:{}", result.join("\n"))
    }
}

/// **Field 57C: Account With Institution (Party Identifier Only)**
///
/// Simplified institutional reference with party identifier only.
/// Format: /34x (mandatory slash prefix)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field57C {
    /// Party identifier (1-34 chars, domestic/clearing codes)
    pub party_identifier: String,
}

impl SwiftField for Field57C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if !input.starts_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 57C must start with '/'".to_string(),
            });
        }

        let identifier = &input[1..];

        if identifier.is_empty() || identifier.len() > 34 {
            return Err(ParseError::InvalidFormat {
                message: "Field 57C party identifier must be 1-34 characters".to_string(),
            });
        }

        parse_swift_chars(identifier, "Field 57C party identifier")?;

        Ok(Field57C {
            party_identifier: identifier.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":57C:/{}", self.party_identifier)
    }
}

/// **Field 57D: Account With Institution (Party Identifier with Name and Address)**
///
/// Detailed institutional identification with name and address.
/// Format: [/1!a][/34x] + 4*35x
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field57D {
    /// Optional party identifier (max 34 chars, routing codes)
    pub party_identifier: Option<String>,

    /// Name and address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field57D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 57D must have at least one line".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut start_idx = 0;

        // Check for party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(party_id);
            start_idx = 1;
        }

        // Parse remaining lines as name and address
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field57D")?;

        Ok(Field57D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(ref id) = self.party_identifier {
            result.push(format!("/{}", id));
        }

        for line in &self.name_and_address {
            result.push(line.clone());
        }

        format!(":57D:{}", result.join("\n"))
    }
}

/// Enum for Field57 Account With Institution variants (A, B, C, D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field57 {
    #[serde(rename = "57A")]
    A(Field57A),
    #[serde(rename = "57B")]
    B(Field57B),
    #[serde(rename = "57C")]
    C(Field57C),
    #[serde(rename = "57D")]
    D(Field57D),
}

impl SwiftField for Field57 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field57A::parse(input) {
            return Ok(Field57::A(field));
        }

        // Try Option C (party identifier only)
        if input.starts_with('/')
            && !input.contains('\n')
            && let Ok(field) = Field57C::parse(input)
        {
            return Ok(Field57::C(field));
        }

        // Try Option B (party identifier with location)
        if let Ok(field) = Field57B::parse(input) {
            return Ok(Field57::B(field));
        }

        // Try Option D (party identifier with name/address)
        if let Ok(field) = Field57D::parse(input) {
            return Ok(Field57::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 57 could not be parsed as option A, B, C or D".to_string(),
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
                let field = Field57A::parse(value)?;
                Ok(Field57::A(field))
            }
            Some("B") => {
                let field = Field57B::parse(value)?;
                Ok(Field57::B(field))
            }
            Some("C") => {
                let field = Field57C::parse(value)?;
                Ok(Field57::C(field))
            }
            Some("D") => {
                let field = Field57D::parse(value)?;
                Ok(Field57::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field57::A(field) => field.to_swift_string(),
            Field57::B(field) => field.to_swift_string(),
            Field57::C(field) => field.to_swift_string(),
            Field57::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field57::A(_) => Some("A"),
            Field57::B(_) => Some("B"),
            Field57::C(_) => Some("C"),
            Field57::D(_) => Some("D"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field57a() {
        // With payment method code
        let field = Field57A::parse("//FW123456\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("/FW123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");

        // With standard party identifier
        let field = Field57A::parse("/C/US123456\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("C/US123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");

        // Without party identifier
        let field = Field57A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.bic, "CHASUS33XXX");
    }

    #[test]
    fn test_field57b() {
        // With party identifier and location
        let field = Field57B::parse("/A/12345\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("A/12345".to_string()));
        assert_eq!(field.location, Some("NEW YORK".to_string()));

        // Empty
        let field = Field57B::parse("").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.location, None);
    }

    #[test]
    fn test_field57c() {
        let field = Field57C::parse("/UKCLEARING123").unwrap();
        assert_eq!(field.party_identifier, "UKCLEARING123");
        assert_eq!(field.to_swift_string(), ":57C:/UKCLEARING123");
    }

    #[test]
    fn test_field57d() {
        // With payment method code
        let field = Field57D::parse("//FW\nCHASE BANK\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("/FW".to_string()));
        assert_eq!(field.name_and_address.len(), 2);
        assert_eq!(field.name_and_address[0], "CHASE BANK");

        // Without party identifier
        let field = Field57D::parse("BENEFICIARY BANK\nLONDON").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field57_payment_method_codes() {
        // Fedwire code
        let field = Field57A::parse("//FW\nCHASUS33").unwrap();
        assert_eq!(field.party_identifier, Some("/FW".to_string()));

        // RTGS code
        let field = Field57A::parse("//RT\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("/RT".to_string()));

        // Australian code
        let field = Field57A::parse("//AU\nANZBAU3M").unwrap();
        assert_eq!(field.party_identifier, Some("/AU".to_string()));

        // Indian code
        let field = Field57A::parse("//IN\nHDFCINBB").unwrap();
        assert_eq!(field.party_identifier, Some("/IN".to_string()));
    }

    #[test]
    fn test_field57_invalid() {
        // Invalid BIC
        assert!(Field57A::parse("INVALID").is_err());

        // Missing slash in 57C
        assert!(Field57C::parse("NOSLASH").is_err());

        // Too many lines in 57D
        assert!(Field57D::parse("LINE1\nLINE2\nLINE3\nLINE4\nLINE5").is_err());
    }
}

// Type aliases for backward compatibility
pub type Field57AccountWithInstitution = Field57;
pub type Field57DebtorBank = Field57;

/// Field57DebtInstitution: Account With Institution for MT200 and similar messages
///
/// Restricted enum supporting only variants A, B, and D per SWIFT specification.
/// Used in MT200 where Field 57 is mandatory and limited to these options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field57DebtInstitution {
    #[serde(rename = "57A")]
    A(Field57A),
    #[serde(rename = "57B")]
    B(Field57B),
    #[serde(rename = "57D")]
    D(Field57D),
}

impl SwiftField for Field57DebtInstitution {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try parsing as 57A (party identifier + BIC)
        if let Ok(field) = Field57A::parse(input) {
            return Ok(Field57DebtInstitution::A(field));
        }

        // Try parsing as 57B (party identifier only)
        if let Ok(field) = Field57B::parse(input) {
            return Ok(Field57DebtInstitution::B(field));
        }

        // Try parsing as 57D (name and address)
        if let Ok(field) = Field57D::parse(input) {
            return Ok(Field57DebtInstitution::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 57 must be one of formats: 57A (party + BIC), 57B (party + location), or 57D (name + address)".to_string(),
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
                let field = Field57A::parse(value)?;
                Ok(Field57DebtInstitution::A(field))
            }
            Some("B") => {
                let field = Field57B::parse(value)?;
                Ok(Field57DebtInstitution::B(field))
            }
            Some("D") => {
                let field = Field57D::parse(value)?;
                Ok(Field57DebtInstitution::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field57DebtInstitution::A(field) => field.to_swift_string(),
            Field57DebtInstitution::B(field) => field.to_swift_string(),
            Field57DebtInstitution::D(field) => field.to_swift_string(),
        }
    }
}

/// Field57AccountWithABD: Account With Institution for MT291 and similar messages
///
/// Restricted enum supporting only variants A, B, and D per SWIFT specification.
/// Used in MT291 where Field 57a is optional and limited to these options.
/// This is an alias for Field57DebtInstitution with a more descriptive name.
pub type Field57AccountWithABD = Field57DebtInstitution;
