//! # Field 50: Ordering Customer
//!
//! Identifies the payment originator. Different variants provide structured
//! (BIC/account-based) or flexible (name/address-based) identification.
//!
//! **Variants:**
//! - **A:** Optional party identifier + BIC (like Field 52A)
//! - **F:** Party identifier + numbered name/address lines (structured)
//! - **K:** Account + name/address (flexible format, most common)
//! - **C:** BIC only (institution-based)
//! - **G:** Account + BIC
//! - **H:** Account + name/address
//! - **L:** Party identifier only
//! - **No Option:** Name/address only
//!
//! **Example:**
//! ```text
//! :50K:/DE89370400440532013000
//! JOHN DOE
//! 123 MAIN STREET
//! ```

use super::field_utils::parse_party_identifier;
use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 50 (No Option): Name/Address Only**
///
/// Free-format name and address (4*35x).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50NoOption {
    /// Name/address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field50NoOption {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 50 (No Option) must have at least one line".to_string(),
            });
        }

        if lines.len() > 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 50 (No Option) cannot have more than 4 lines, found {}",
                    lines.len()
                ),
            });
        }

        // Validate each line
        for (i, line) in lines.iter().enumerate() {
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 50 (No Option) line {} exceeds 35 characters", i + 1),
                });
            }
            parse_swift_chars(line, &format!("Field 50 (No Option) line {}", i + 1))?;
        }

        Ok(Field50NoOption {
            name_and_address: lines,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":50:{}", self.name_and_address.join("\n"))
    }
}

/// **Field 50A: Account/Party Identifier + BIC**
///
/// Structured format with optional party identifier and BIC code.
/// Format: [/34x] + 4!a2!a2!c[3!c] (like Field 52A)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50A {
    /// Optional party identifier (max 34 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_identifier: Option<String>,
    /// BIC code (8 or 11 chars)
    pub bic: String,
}

impl SwiftField for Field50A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 50A cannot be empty".to_string(),
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
                message: "Field 50A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field50A {
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
        format!(":50A:{}", lines.join("\n"))
    }
}

/// **Field 50F: Party Identifier + Numbered Name/Address Lines**
///
/// Structured format with party identifier and numbered detail lines.
/// Format: 35x (party identifier) + 4*(1!n/33x) (numbered lines)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50F {
    /// Party identifier (max 35 chars, first line)
    pub party_identifier: String,
    /// Numbered name/address lines (stored WITHOUT number prefix, e.g., "ACME CORP")
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field50F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.len() < 2 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 50F must have at least 2 lines (party identifier + numbered line), found {}",
                    lines.len()
                ),
            });
        }

        // Parse party identifier (first line, max 35 chars)
        let party_identifier = lines[0];
        if party_identifier.is_empty() || party_identifier.len() > 35 {
            return Err(ParseError::InvalidFormat {
                message: "Field 50F party identifier must be 1-35 characters".to_string(),
            });
        }
        parse_swift_chars(party_identifier, "Field 50F party identifier")?;

        // Parse numbered name/address lines (remaining lines)
        let mut name_and_address = Vec::new();
        for (i, line) in lines.iter().enumerate().skip(1) {
            // Expected format: digit/text (e.g., "1/ACME CORP")
            if line.len() < 2 || !line.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 50F line {} must start with line number", i),
                });
            }

            if line.chars().nth(1) != Some('/') {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 50F line {} must have '/' after line number", i),
                });
            }

            let text = &line[2..];
            if text.len() > 33 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 50F line {} text exceeds 33 characters", i),
                });
            }

            parse_swift_chars(text, &format!("Field 50F line {}", i))?;
            name_and_address.push(text.to_string());
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 50F must have at least one numbered name/address line".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 50F cannot have more than 4 name/address lines, found {}",
                    name_and_address.len()
                ),
            });
        }

        Ok(Field50F {
            party_identifier: party_identifier.to_string(),
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = vec![self.party_identifier.clone()];

        for (i, line) in self.name_and_address.iter().enumerate() {
            result.push(format!("{}/{}", i + 1, line));
        }

        format!(":50F:{}", result.join("\n"))
    }
}

/// **Field 50K: Account + Free-Format Name/Address**
///
/// Most common variant. Free-format name and address.
/// Format: [/34x]4*35x
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50K {
    /// Optional account (max 34 chars)
    pub account: Option<String>,
    /// Name/address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field50K {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 50K must have at least one line".to_string(),
            });
        }

        let mut account = None;
        let mut name_and_address = Vec::new();
        let mut start_index = 0;

        // Check if first line is account (with leading slash in MT format)
        if lines[0].starts_with('/') {
            let acc = &lines[0][1..];
            if acc.len() > 34 {
                return Err(ParseError::InvalidFormat {
                    message: "Field 50K account exceeds 34 characters".to_string(),
                });
            }
            parse_swift_chars(acc, "Field 50K account")?;
            // Store account without the slash
            account = Some(acc.to_string());
            start_index = 1;
        }

        // Parse name/address lines
        for (i, line) in lines.iter().enumerate().skip(start_index) {
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 50K line {} exceeds 35 characters",
                        i - start_index + 1
                    ),
                });
            }
            parse_swift_chars(line, &format!("Field 50K line {}", i - start_index + 1))?;
            name_and_address.push(line.to_string());
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 50K must have at least one name/address line".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 50K cannot have more than 4 name/address lines, found {}",
                    name_and_address.len()
                ),
            });
        }

        Ok(Field50K {
            account,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        // Add slash prefix when converting to MT format
        if let Some(ref acc) = self.account {
            result.push(format!("/{}", acc));
        }

        for line in &self.name_and_address {
            result.push(line.clone());
        }

        format!(":50K:{}", result.join("\n"))
    }
}

/// **Field 50C: BIC Only**
///
/// Institution-based identification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50C {
    /// BIC code (8 or 11 chars)
    pub bic: String,
}

impl SwiftField for Field50C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let bic = parse_bic(input)?;
        Ok(Field50C { bic })
    }

    fn to_swift_string(&self) -> String {
        format!(":50C:{}", self.bic)
    }
}

/// **Field 50L: Party Identifier Only**
///
/// Single-line party identifier (35x).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50L {
    /// Party ID (max 35 chars)
    pub party_identifier: String,
}

impl SwiftField for Field50L {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Field 50L should be a single-line party identifier
        // Reject if contains newlines (which would indicate it's a different variant)
        if input.contains('\n') {
            return Err(ParseError::InvalidFormat {
                message: "Field 50L party identifier must be single line".to_string(),
            });
        }

        if input.is_empty() || input.len() > 35 {
            return Err(ParseError::InvalidFormat {
                message: "Field 50L party identifier must be 1-35 characters".to_string(),
            });
        }

        parse_swift_chars(input, "Field 50L party identifier")?;

        Ok(Field50L {
            party_identifier: input.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":50L:{}", self.party_identifier)
    }
}

/// **Field 50G: Account + BIC**
///
/// Simple account and BIC identification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50G {
    /// Account (max 34 chars)
    pub account: String,
    /// BIC code
    pub bic: String,
}

impl SwiftField for Field50G {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.len() != 2 {
            return Err(ParseError::InvalidFormat {
                message: format!("Field 50G must have exactly 2 lines, found {}", lines.len()),
            });
        }

        // Parse account (first line, must start with /)
        if !lines[0].starts_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 50G account must start with '/'".to_string(),
            });
        }

        let account = &lines[0][1..];
        if account.is_empty() || account.len() > 34 {
            return Err(ParseError::InvalidFormat {
                message: "Field 50G account must be 1-34 characters".to_string(),
            });
        }
        parse_swift_chars(account, "Field 50G account")?;

        // Parse BIC (second line)
        let bic = parse_bic(lines[1])?;

        Ok(Field50G {
            account: account.to_string(),
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":50G:/{}\n{}", self.account, self.bic)
    }
}

/// **Field 50H: Account + Name/Address**
///
/// Account with free-format name/address.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field50H {
    /// Account (max 34 chars)
    pub account: String,
    /// Name/address (max 4 lines, 35 chars each)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field50H {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.len() < 2 {
            return Err(ParseError::InvalidFormat {
                message: "Field 50H must have at least 2 lines".to_string(),
            });
        }

        // Parse account (first line, must start with /)
        if !lines[0].starts_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 50H account must start with '/'".to_string(),
            });
        }

        let account = &lines[0][1..];
        if account.is_empty() || account.len() > 34 {
            return Err(ParseError::InvalidFormat {
                message: "Field 50H account must be 1-34 characters".to_string(),
            });
        }
        parse_swift_chars(account, "Field 50H account")?;

        // Parse name/address lines
        let mut name_and_address = Vec::new();
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 50H line {} exceeds 35 characters", i),
                });
            }
            parse_swift_chars(line, &format!("Field 50H line {}", i))?;
            name_and_address.push(line.to_string());
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 50H cannot have more than 4 name/address lines, found {}",
                    name_and_address.len()
                ),
            });
        }

        Ok(Field50H {
            account: account.to_string(),
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = vec![format!("/{}", self.account)];
        result.extend(self.name_and_address.clone());
        format!(":50H:{}", result.join("\n"))
    }
}

/// Enum for Field50 Instructing Party variants (C, L)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field50InstructingParty {
    #[serde(rename = "50C")]
    C(Field50C),
    #[serde(rename = "50L")]
    L(Field50L),
}

impl SwiftField for Field50InstructingParty {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try to detect variant by format
        // Option C is a BIC (8 or 11 characters)
        // Option L is a party identifier (up to 35 characters)

        let trimmed = input.trim();

        // Try parsing as BIC first (more restrictive)
        if let Ok(field) = Field50C::parse(trimmed) {
            return Ok(Field50InstructingParty::C(field));
        }

        // Try parsing as party identifier
        if let Ok(field) = Field50L::parse(trimmed) {
            return Ok(Field50InstructingParty::L(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 50 Instructing Party could not be parsed as option C or L".to_string(),
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
            Some("C") => {
                let field = Field50C::parse(value)?;
                Ok(Field50InstructingParty::C(field))
            }
            Some("L") => {
                let field = Field50L::parse(value)?;
                Ok(Field50InstructingParty::L(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50InstructingParty::C(field) => field.to_swift_string(),
            Field50InstructingParty::L(field) => field.to_swift_string(),
        }
    }
}

/// Enum for Field50 Ordering Customer variants (F, G, H)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field50OrderingCustomerFGH {
    #[serde(rename = "50F")]
    F(Field50F),
    #[serde(rename = "50G")]
    G(Field50G),
    #[serde(rename = "50H")]
    H(Field50H),
}

impl SwiftField for Field50OrderingCustomerFGH {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.len() >= 2 {
            // Check for numbered lines (characteristic of Option F)
            let has_numbered_lines = lines.iter().skip(1).any(|line| {
                line.len() >= 2
                    && line.chars().next().is_some_and(|c| c.is_ascii_digit())
                    && line.chars().nth(1) == Some('/')
            });

            if has_numbered_lines && let Ok(field) = Field50F::parse(input) {
                return Ok(Field50OrderingCustomerFGH::F(field));
            }

            // Try Option G: /account + BIC
            if lines[0].starts_with('/')
                && lines.len() == 2
                && let Ok(field) = Field50G::parse(input)
            {
                return Ok(Field50OrderingCustomerFGH::G(field));
            }

            // Try Option H: /account + name/address
            if lines[0].starts_with('/')
                && let Ok(field) = Field50H::parse(input)
            {
                return Ok(Field50OrderingCustomerFGH::H(field));
            }
        }

        Err(ParseError::InvalidFormat {
            message: "Field 50 Ordering Customer could not be parsed as option F, G or H"
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
            Some("F") => {
                let field = Field50F::parse(value)?;
                Ok(Field50OrderingCustomerFGH::F(field))
            }
            Some("G") => {
                let field = Field50G::parse(value)?;
                Ok(Field50OrderingCustomerFGH::G(field))
            }
            Some("H") => {
                let field = Field50H::parse(value)?;
                Ok(Field50OrderingCustomerFGH::H(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50OrderingCustomerFGH::F(field) => field.to_swift_string(),
            Field50OrderingCustomerFGH::G(field) => field.to_swift_string(),
            Field50OrderingCustomerFGH::H(field) => field.to_swift_string(),
        }
    }
}

/// Enum for Field50 Ordering Customer variants (A, F, K)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field50OrderingCustomerAFK {
    #[serde(rename = "50A")]
    A(Field50A),
    #[serde(rename = "50F")]
    F(Field50F),
    #[serde(rename = "50K")]
    K(Field50K),
}

impl SwiftField for Field50OrderingCustomerAFK {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        // Check for numbered lines (characteristic of Option F)
        let has_numbered_lines = lines.iter().any(|line| {
            line.len() >= 2
                && line.chars().next().is_some_and(|c| c.is_ascii_digit())
                && line.chars().nth(1) == Some('/')
        });

        if has_numbered_lines && let Ok(field) = Field50F::parse(input) {
            return Ok(Field50OrderingCustomerAFK::F(field));
        }

        // Check if last line is a valid BIC (characteristic of Option A)
        if let Some(last_line) = lines.last() {
            let trimmed = last_line.trim();
            if (trimmed.len() == 8 || trimmed.len() == 11)
                && trimmed.chars().all(|c| c.is_ascii_alphanumeric())
                && let Ok(field) = Field50A::parse(input)
            {
                return Ok(Field50OrderingCustomerAFK::A(field));
            }
        }

        // Try Option K (flexible format)
        if let Ok(field) = Field50K::parse(input) {
            return Ok(Field50OrderingCustomerAFK::K(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 50 Ordering Customer could not be parsed as option A, F or K"
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
                let field = Field50A::parse(value)?;
                Ok(Field50OrderingCustomerAFK::A(field))
            }
            Some("F") => {
                let field = Field50F::parse(value)?;
                Ok(Field50OrderingCustomerAFK::F(field))
            }
            Some("K") => {
                let field = Field50K::parse(value)?;
                Ok(Field50OrderingCustomerAFK::K(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50OrderingCustomerAFK::A(field) => field.to_swift_string(),
            Field50OrderingCustomerAFK::F(field) => field.to_swift_string(),
            Field50OrderingCustomerAFK::K(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field50OrderingCustomerAFK::A(_) => Some("A"),
            Field50OrderingCustomerAFK::F(_) => Some("F"),
            Field50OrderingCustomerAFK::K(_) => Some("K"),
        }
    }
}

/// Enum for Field50 Ordering Customer variants (No Option, C, F)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field50OrderingCustomerNCF {
    #[serde(rename = "50")]
    NoOption(Field50NoOption),
    #[serde(rename = "50C")]
    C(Field50C),
    #[serde(rename = "50F")]
    F(Field50F),
}

impl SwiftField for Field50OrderingCustomerNCF {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        // Try Option C (single line BIC)
        if lines.len() == 1
            && (8..=11).contains(&lines[0].len())
            && let Ok(field) = Field50C::parse(input)
        {
            return Ok(Field50OrderingCustomerNCF::C(field));
        }

        // Check for numbered lines (characteristic of Option F)
        let has_numbered_lines = lines.iter().any(|line| {
            line.len() >= 2
                && line.chars().next().is_some_and(|c| c.is_ascii_digit())
                && line.chars().nth(1) == Some('/')
        });

        if has_numbered_lines && let Ok(field) = Field50F::parse(input) {
            return Ok(Field50OrderingCustomerNCF::F(field));
        }

        // Try No Option (name/address only)
        if let Ok(field) = Field50NoOption::parse(input) {
            return Ok(Field50OrderingCustomerNCF::NoOption(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 50 Ordering Customer could not be parsed as No Option, C or F"
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
            None => {
                let field = Field50NoOption::parse(value)?;
                Ok(Field50OrderingCustomerNCF::NoOption(field))
            }
            Some("C") => {
                let field = Field50C::parse(value)?;
                Ok(Field50OrderingCustomerNCF::C(field))
            }
            Some("F") => {
                let field = Field50F::parse(value)?;
                Ok(Field50OrderingCustomerNCF::F(field))
            }
            _ => {
                // Unknown variant, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50OrderingCustomerNCF::NoOption(field) => field.to_swift_string(),
            Field50OrderingCustomerNCF::C(field) => field.to_swift_string(),
            Field50OrderingCustomerNCF::F(field) => field.to_swift_string(),
        }
    }
}

/// Enum for Field50 Creditor variants (A, K)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub enum Field50Creditor {
    #[serde(rename = "50A")]
    A(Field50A),
    #[serde(rename = "50K")]
    K(Field50K),
}

impl SwiftField for Field50Creditor {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        // Check if last line is a valid BIC (characteristic of Option A)
        if let Some(last_line) = lines.last() {
            let trimmed = last_line.trim();
            if (trimmed.len() == 8 || trimmed.len() == 11)
                && trimmed.chars().all(|c| c.is_ascii_alphanumeric())
                && let Ok(field) = Field50A::parse(input)
            {
                return Ok(Field50Creditor::A(field));
            }
        }

        // Try Option K
        if let Ok(field) = Field50K::parse(input) {
            return Ok(Field50Creditor::K(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 50 Creditor could not be parsed as option A or K".to_string(),
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
                let field = Field50A::parse(value)?;
                Ok(Field50Creditor::A(field))
            }
            Some("K") => {
                let field = Field50K::parse(value)?;
                Ok(Field50Creditor::K(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50Creditor::A(field) => field.to_swift_string(),
            Field50Creditor::K(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field50Creditor::A(_) => Some("A"),
            Field50Creditor::K(_) => Some("K"),
        }
    }
}

// Type alias for backward compatibility - most common use case is ordering customer
pub type Field50 = Field50OrderingCustomerNCF;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field50_no_option() {
        let field = Field50NoOption::parse("JOHN DOE\n123 MAIN ST\nNEW YORK").unwrap();
        assert_eq!(field.name_and_address.len(), 3);
        assert_eq!(field.name_and_address[0], "JOHN DOE");
        assert_eq!(
            field.to_swift_string(),
            ":50:JOHN DOE\n123 MAIN ST\nNEW YORK"
        );
    }

    #[test]
    fn test_field50a() {
        // With party identifier
        let field = Field50A::parse("/ACC123\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("ACC123".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");

        let swift_str = field.to_swift_string();
        assert_eq!(swift_str, ":50A:/ACC123\nDEUTDEFF");

        // Without party identifier
        let field = Field50A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.bic, "CHASUS33XXX");
        assert_eq!(field.to_swift_string(), ":50A:CHASUS33XXX");
    }

    #[test]
    fn test_field50f() {
        let field =
            Field50F::parse("/US123456789\n1/ACME CORP\n2/123 MAIN ST\n3/US/NEW YORK").unwrap();
        assert_eq!(field.party_identifier, "/US123456789");
        assert_eq!(field.name_and_address.len(), 3);
        assert_eq!(field.name_and_address[0], "ACME CORP");
        assert_eq!(field.name_and_address[1], "123 MAIN ST");
        assert_eq!(field.name_and_address[2], "US/NEW YORK");

        let swift_str = field.to_swift_string();
        assert!(swift_str.starts_with(":50F:"));
        assert!(swift_str.contains("/US123456789"));
        assert!(swift_str.contains("1/ACME CORP"));
    }

    #[test]
    fn test_field50k() {
        let field = Field50K::parse(
            "/DE89370400440532013000\nJOHN DOE\n123 MAIN STREET\nNEW YORK NY 10001",
        )
        .unwrap();
        assert_eq!(field.account, Some("DE89370400440532013000".to_string()));
        assert_eq!(field.name_and_address[0], "JOHN DOE");
        assert_eq!(field.name_and_address.len(), 3);
    }

    #[test]
    fn test_field50c() {
        let field = Field50C::parse("DEUTDEFF").unwrap();
        assert_eq!(field.bic, "DEUTDEFF");
        assert_eq!(field.to_swift_string(), ":50C:DEUTDEFF");
    }

    #[test]
    fn test_field50l() {
        let field = Field50L::parse("PARTY123").unwrap();
        assert_eq!(field.party_identifier, "PARTY123");
        assert_eq!(field.to_swift_string(), ":50L:PARTY123");
    }

    #[test]
    fn test_field50g() {
        let field = Field50G::parse("/ACCOUNT456\nCHASUS33XXX").unwrap();
        assert_eq!(field.account, "ACCOUNT456");
        assert_eq!(field.bic, "CHASUS33XXX");
        assert_eq!(field.to_swift_string(), ":50G:/ACCOUNT456\nCHASUS33XXX");
    }

    #[test]
    fn test_field50h() {
        let field = Field50H::parse("/ACCOUNT789\nJANE SMITH\n456 ELM ST").unwrap();
        assert_eq!(field.account, "ACCOUNT789");
        assert_eq!(field.name_and_address.len(), 2);
        assert_eq!(field.name_and_address[0], "JANE SMITH");
    }

    #[test]
    fn test_field50_ordering_customer_afk() {
        // Test Option A (BIC-based)
        let field = Field50OrderingCustomerAFK::parse("/ACC123\nDEUTDEFF").unwrap();
        assert!(matches!(field, Field50OrderingCustomerAFK::A(_)));

        // Test Option A without party identifier
        let field = Field50OrderingCustomerAFK::parse("CHASUS33XXX").unwrap();
        assert!(matches!(field, Field50OrderingCustomerAFK::A(_)));

        // Test Option F (numbered lines)
        let field =
            Field50OrderingCustomerAFK::parse("/US123456789\n1/ACME CORP\n2/NEW YORK").unwrap();
        assert!(matches!(field, Field50OrderingCustomerAFK::F(_)));

        // Test Option K (flexible format)
        let field = Field50OrderingCustomerAFK::parse("/ACC123\nJOHN DOE").unwrap();
        assert!(matches!(field, Field50OrderingCustomerAFK::K(_)));
    }
}
