use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 59F: Party ID + Numbered Name/Address**
///
/// Structured beneficiary identification with numbered lines.
/// Format: [/34x]4*(1!n/33x)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field59F {
    /// Optional party ID (max 34 chars)
    pub party_identifier: Option<String>,
    /// Numbered name/address lines (e.g., "1/ACME CORP")
    pub name_and_address: Vec<String>,
}

/// **Field 59A: Account + BIC**
///
/// BIC-based beneficiary identification (STP-preferred).
/// Format: [/34x] + BIC (8 or 11 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field59A {
    /// Optional account (max 34 chars, IBAN or domestic)
    pub account: Option<String>,
    /// BIC code (8 or 11 chars)
    pub bic: String,
}

/// **Field 59 (No Option): Account + Free-Format Name/Address**
///
/// Most common variant. Flexible beneficiary identification.
/// Format: [/34x]4*35x
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field59NoOption {
    /// Optional account (max 34 chars)
    pub account: Option<String>,
    /// Name/address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

/// **Field 59: Beneficiary Customer**
///
/// Final recipient of payment funds.
///
/// **Variants:**
/// - **A:** Account + BIC (STP-preferred)
/// - **F:** Party ID + numbered name/address
/// - **No Option:** Account + free-format name/address (most common)
///
/// **Example:**
/// ```text
/// :59:/GB82WEST12345698765432
/// JOHN SMITH
/// 456 RESIDENTIAL AVENUE
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field59 {
    #[serde(rename = "59A")]
    A(Field59A),
    #[serde(rename = "59F")]
    F(Field59F),
    #[serde(rename = "59")]
    NoOption(Field59NoOption),
}

impl SwiftField for Field59F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 59F cannot be empty".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut start_idx = 0;

        // Check for party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(party_id);
            start_idx = 1;
        }

        // Parse name and address lines with line number format: 1!n/33x
        let mut name_and_address = Vec::new();
        for (i, line) in lines.iter().enumerate().skip(start_idx) {
            // Check for line number format (1!n/33x)
            let mut chars = line.chars();
            let first_char = chars.next();
            let second_char = chars.next();
            if line.len() < 2 || !first_char.unwrap().is_ascii_digit() || second_char != Some('/') {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 59F line {} must start with line number and slash (e.g., '1/')",
                        i - start_idx + 1
                    ),
                });
            }

            let line_num = first_char.unwrap().to_digit(10).unwrap() as usize;
            let expected_line_num = i - start_idx + 1;

            if line_num != expected_line_num {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 59F line number {} doesn't match expected {}",
                        line_num, expected_line_num
                    ),
                });
            }

            let content = &line[2..];
            if content.len() > 33 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 59F line {} content exceeds 33 characters", line_num),
                });
            }

            parse_swift_chars(content, &format!("Field 59F line {}", line_num))?;
            name_and_address.push(line.to_string());
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 59F must have at least one name/address line".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 59F cannot have more than 4 name/address lines, found {}",
                    name_and_address.len()
                ),
            });
        }

        Ok(Field59F {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":59F:");

        if let Some(ref id) = self.party_identifier {
            result.push_str(&format!("/{}\n", id));
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            if i > 0 || self.party_identifier.is_some() {
                result.push('\n');
            }
            result.push_str(line);
        }

        result
    }
}

impl SwiftField for Field59A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 59A cannot be empty".to_string(),
            });
        }

        let mut account = None;
        let bic_line_idx;

        // Check if first line is account (/...)
        if lines[0].starts_with('/') {
            let identifier = &lines[0][1..];
            if identifier.len() <= 34 {
                parse_swift_chars(identifier, "Field 59A account")?;
                account = Some(identifier.to_string());
                bic_line_idx = 1;
            } else {
                bic_line_idx = 0;
            }
        } else {
            bic_line_idx = 0;
        }

        // Parse BIC
        if bic_line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 59A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field59A { account, bic })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":59A:");

        if let Some(ref acc) = self.account {
            result.push_str(&format!("/{}\n", acc));
        }

        result.push_str(&self.bic);
        result
    }
}

impl SwiftField for Field59NoOption {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 59 (No Option) cannot be empty".to_string(),
            });
        }

        let mut account = None;
        let mut start_idx = 0;

        // Check for account
        if lines[0].starts_with('/') {
            let identifier = &lines[0][1..];
            if identifier.len() <= 34 {
                parse_swift_chars(identifier, "Field 59 account")?;
                account = Some(identifier.to_string());
                start_idx = 1;
            }
        }

        // Parse remaining lines as name and address
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field59NoOption")?;

        Ok(Field59NoOption {
            account,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":59:");

        if let Some(ref acc) = self.account {
            result.push_str(&format!("/{}", acc));
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            if i > 0 || (i == 0 && self.account.is_some()) {
                result.push('\n');
            }
            result.push_str(line);
        }

        result
    }
}

impl SwiftField for Field59 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field59A::parse(input) {
            return Ok(Field59::A(field));
        }

        // Try Option F (structured name/address with line numbers)
        // This is identifiable by the line number format (1/content, 2/content, etc.)
        let lines: Vec<&str> = input.lines().collect();
        if !lines.is_empty() {
            // Check if any line (after optional account) has line number format
            let check_start = if lines[0].starts_with('/') { 1 } else { 0 };
            if check_start < lines.len() {
                let test_line = lines[check_start];
                let mut chars = test_line.chars();
                if test_line.len() >= 2
                    && chars.next().unwrap().is_ascii_digit()
                    && chars.next() == Some('/')
                    && let Ok(field) = Field59F::parse(input)
                {
                    return Ok(Field59::F(field));
                }
            }
        }

        // Try No Option (account + name/address)
        if let Ok(field) = Field59NoOption::parse(input) {
            return Ok(Field59::NoOption(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 59 could not be parsed as option A, F, or No Option".to_string(),
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
            None => {
                let field = Field59NoOption::parse(value)?;
                Ok(Field59::NoOption(field))
            }
            Some("A") => {
                let field = Field59A::parse(value)?;
                Ok(Field59::A(field))
            }
            Some("F") => {
                let field = Field59F::parse(value)?;
                Ok(Field59::F(field))
            }
            _ => {
                // Unknown variant, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field59::A(field) => field.to_swift_string(),
            Field59::F(field) => field.to_swift_string(),
            Field59::NoOption(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field59::A(_) => Some("A"),
            Field59::F(_) => Some("F"),
            Field59::NoOption(_) => None, // No option doesn't have a variant letter
        }
    }
}

impl SwiftField for Field59Debtor {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field59A::parse(input) {
            return Ok(Field59Debtor::A(field));
        }

        // Try No Option (account + name/address)
        if let Ok(field) = Field59NoOption::parse(input) {
            return Ok(Field59Debtor::NoOption(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 59 Debtor could not be parsed as option A or No Option".to_string(),
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
            None => {
                let field = Field59NoOption::parse(value)?;
                Ok(Field59Debtor::NoOption(field))
            }
            Some("A") => {
                let field = Field59A::parse(value)?;
                Ok(Field59Debtor::A(field))
            }
            _ => {
                // Unknown variant, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field59Debtor::A(field) => field.to_swift_string(),
            Field59Debtor::NoOption(field) => field.to_swift_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field59Debtor {
    #[serde(rename = "59A")]
    A(Field59A),
    #[serde(rename = "59")]
    NoOption(Field59NoOption),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field59f() {
        // With party identifier
        let field = Field59F::parse("/GB82WEST12345698765432\n1/ACME CORPORATION LIMITED\n2/INTERNATIONAL TRADE DIVISION\n3/123 BUSINESS PARK AVENUE\n4/LONDON EC1A 1BB UNITED KINGDOM").unwrap();
        assert_eq!(
            field.party_identifier,
            Some("GB82WEST12345698765432".to_string())
        );
        assert_eq!(field.name_and_address.len(), 4);
        assert_eq!(field.name_and_address[0], "1/ACME CORPORATION LIMITED");

        // Without party identifier
        let field =
            Field59F::parse("1/JOHN SMITH\n2/123 MAIN STREET\n3/LONDON\n4/UNITED KINGDOM").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 4);
    }

    #[test]
    fn test_field59a() {
        // With account
        let field = Field59A::parse("/GB82WEST12345698765432\nMIDLGB22XXX").unwrap();
        assert_eq!(field.account, Some("GB82WEST12345698765432".to_string()));
        assert_eq!(field.bic, "MIDLGB22XXX");

        // Without account
        let field = Field59A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.account, None);
        assert_eq!(field.bic, "CHASUS33XXX");
    }

    #[test]
    fn test_field59_no_option() {
        // With account
        let field = Field59NoOption::parse("/GB82WEST12345698765432\nJOHN SMITH\n456 RESIDENTIAL AVENUE\nMANCHESTER M1 1AA\nUNITED KINGDOM").unwrap();
        assert_eq!(field.account, Some("GB82WEST12345698765432".to_string()));
        assert_eq!(field.name_and_address.len(), 4);
        assert_eq!(field.name_and_address[0], "JOHN SMITH");

        // Without account
        let field = Field59NoOption::parse("JANE DOE\n789 MAIN STREET\nLONDON").unwrap();
        assert_eq!(field.account, None);
        assert_eq!(field.name_and_address.len(), 3);
    }

    #[test]
    fn test_field59_invalid() {
        // Invalid BIC
        assert!(Field59A::parse("INVALID").is_err());

        // Invalid line number in 59F
        assert!(Field59F::parse("2/WRONG LINE NUMBER").is_err());

        // Too many lines
        assert!(Field59NoOption::parse("LINE1\nLINE2\nLINE3\nLINE4\nLINE5").is_err());
    }
}
