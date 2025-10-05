//! # Field 50: Ordering Customer
//!
//! ## Purpose
//! Identifies the ordering customer (originator) of the payment instruction. The ordering customer
//! is the party that instructs the sender of the MT103 to execute the payment. Different options
//! provide various levels of detail and identification methods for optimal processing efficiency.
//!
//! ## Options Overview
//! - **Option A**: Account + BIC identification (optimal for STP)
//! - **Option F**: Party identifier + BIC (structured identification)
//! - **Option K**: Account + Name/Address details (flexible format)
//! - **Option C**: BIC identification only (institution-based)
//! - **Option G**: Account + BIC (alternative format)
//! - **Option H**: Account + Name/Address (alternative format)
//! - **Option L**: Party identifier only (simplified identification)
//! - **No Option**: Name/Address only (basic identification)
//!
//! ## Usage by Message Type
//! - **MT103**: Options A, F, K supported (Field50OrderingCustomerAFK)
//! - **MT101**: Options A, F, K supported for batch payments
//! - **MT102**: Options available depending on batch type
//! - **Creditor Payments**: Options A, K supported (Field50Creditor)
//! - **Instructing Party**: Options C, L supported (Field50InstructingParty)
//!
//! ## STP Compliance Guidelines
//! ### STP Preferred (Optimal Automation)
//! - **Option A**: Account + BIC - maximum STP efficiency
//! - **Option F**: Party identifier + BIC - structured processing
//! - **Option C**: BIC only - institution-based routing
//!
//! ### STP Compatible (Good Automation)
//! - **Option K**: Account + Name/Address with complete information
//! - **Option G**: Account + BIC alternative format
//! - **Option H**: Account + Name/Address alternative format
//!
//! ### Manual Processing Risk
//! - **No Option**: Name/Address only - may require manual intervention
//! - **Option L**: Party identifier only - limited automation
//!
//! ## Format Selection Guidelines
//! ### When to Use Each Option
//! - **Option A**: Standard customer payments with account and BIC
//! - **Option F**: Enhanced identification requirements
//! - **Option K**: Flexible customer identification scenarios
//! - **Option C**: Institution-to-institution transactions
//! - **Option G/H**: Alternative formats for specific message types
//! - **Option L**: Simplified party identification
//! - **No Option**: Basic customer identification
//!
//! ## Business Context Applications
//! - **Payment Origination**: Customer-initiated payment instructions
//! - **Corporate Payments**: Business-to-business transaction origination
//! - **Retail Payments**: Individual customer payment instructions
//! - **Batch Processing**: Multiple payment origination identification
//!
//! ## Network Validation Requirements
//! - **BIC Validation**: Must be active and reachable in SWIFT network
//! - **Account Validation**: Must conform to local account standards
//! - **Character Set**: Standard SWIFT character set compliance
//! - **Address Standards**: Adequate detail for customer identification
//!
//! ## Compliance Framework
//! - **KYC Standards**: Customer identification and verification
//! - **AML Requirements**: Anti-money laundering originator screening
//! - **Regulatory Documentation**: Complete originator record keeping
//! - **Audit Trail**: Comprehensive origination audit information
//!
//! ## Related Fields Integration
//! - **Field 52A**: Ordering Institution (originator's bank)
//! - **Field 53A**: Sender's Correspondent (routing)
//! - **Field 59**: Beneficiary Customer (payment destination)
//! - **Field 70**: Remittance Information (payment purpose)
//!
//! ## Error Prevention Guidelines
//! - **Complete Information**: Provide full originator identification details
//! - **Accurate Codes**: Verify BIC codes and account numbers
//! - **Format Consistency**: Follow established format conventions
//! - **Compliance Verification**: Screen against sanctions and watch lists
//!
//! ## See Also
//! - Swift FIN User Handbook: Ordering Customer Specifications
//! - KYC Guidelines: Customer Identification Requirements
//! - AML/CFT Compliance: Originator Screening Best Practices
//! - STP Implementation Guide: Ordering Customer Optimization

use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 50 (No Option): Ordering Customer**
///
/// Basic variant of [Field 50 module](index.html). Provides ordering customer identification
/// using name and address information only.
///
/// **Components:**
/// - Name and address lines (4*35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50NoOption {
    /// Name and address lines (up to 4 lines of 35 characters each)
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

/// **Field 50A: Ordering Customer (Option A)**
///
/// Account + BIC variant of [Field 50 module](index.html). Provides structured identification
/// using optional account identifier and numbered name/address lines.
///
/// **Components:**
/// - Party identifier (optional, \[/34x\])
/// - Name and address lines (4*(1!n/33x))
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50A {
    /// Optional account identifier (IBAN, account number, etc.)
    /// Format: [/34x] - Up to 34 characters with leading slash
    pub party_identifier: Option<String>,

    /// Name and address lines with mandatory line numbering
    /// Format: 4*(1!n/33x) - Line number + slash + 33 character text
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field50A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 50A must have at least one line".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut name_and_address = Vec::new();
        let mut start_index = 0;

        // Check if first line is party identifier
        if lines[0].starts_with('/') {
            let identifier = &lines[0][1..];
            if identifier.len() > 34 {
                return Err(ParseError::InvalidFormat {
                    message: "Field 50A party identifier exceeds 34 characters".to_string(),
                });
            }
            parse_swift_chars(identifier, "Field 50A party identifier")?;
            party_identifier = Some(identifier.to_string());
            start_index = 1;
        }

        // Parse numbered name/address lines
        for (i, line) in lines.iter().enumerate().skip(start_index) {
            // Expected format: digit/text (e.g., "1/ACME CORP")
            if line.len() < 2 || !line.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 50A line {} must start with line number",
                        i - start_index + 1
                    ),
                });
            }

            if line.chars().nth(1) != Some('/') {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 50A line {} must have '/' after line number",
                        i - start_index + 1
                    ),
                });
            }

            let text = &line[2..];
            if text.len() > 33 {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 50A line {} text exceeds 33 characters",
                        i - start_index + 1
                    ),
                });
            }

            parse_swift_chars(text, &format!("Field 50A line {}", i - start_index + 1))?;
            name_and_address.push(text.to_string());
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 50A must have at least one name/address line".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 50A cannot have more than 4 name/address lines, found {}",
                    name_and_address.len()
                ),
            });
        }

        Ok(Field50A {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(ref id) = self.party_identifier {
            result.push(format!("/{}", id));
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            result.push(format!("{}/{}", i + 1, line));
        }

        format!(":50A:{}", result.join("\n"))
    }
}

/// **Field 50F: Ordering Customer (Option F)**
///
/// Party identifier + Name/Address + BIC variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Account/Party Identifier (/34x)
/// - Optional Party Identifier (/34x)
/// - Name and Address (1-4 lines, 35x each)
/// - BIC (4!a2!a2!c\[3!c\])
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50F {
    /// Account or party identifier
    pub account: String,
    /// Optional party identifier (e.g., "SEC/1234567890")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_identifier: Option<String>,
    /// Name and address information (1-4 lines)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_and_address: Option<Vec<String>>,
    /// BIC code
    pub bic: String,
}

impl SwiftField for Field50F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.len() < 2 {
            return Err(ParseError::InvalidFormat {
                message: format!("Field 50F must have at least 2 lines (account + BIC), found {}", lines.len()),
            });
        }

        // Parse account (first line)
        let account = lines[0];
        if account.is_empty() || account.len() > 35 {
            return Err(ParseError::InvalidFormat {
                message: "Field 50F account must be 1-35 characters".to_string(),
            });
        }
        parse_swift_chars(account, "Field 50F account")?;

        // Find BIC line (last line)
        let bic = parse_bic(lines[lines.len() - 1])?;

        // Check if there's a party identifier (line starting with /) after account
        let mut party_identifier = None;
        let mut name_start = 1;

        if lines.len() > 2 && lines[1].starts_with('/') {
            let party_id = &lines[1][1..]; // Remove leading slash
            if party_id.len() > 34 {
                return Err(ParseError::InvalidFormat {
                    message: "Field 50F party identifier exceeds 34 characters".to_string(),
                });
            }
            parse_swift_chars(party_id, "Field 50F party identifier")?;
            party_identifier = Some(party_id.to_string());
            name_start = 2;
        }

        // Parse name and address lines (between party_id/account and BIC)
        let mut name_and_address = Vec::new();
        for line in &lines[name_start..lines.len() - 1] {
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: "Field 50F name/address line exceeds 35 characters".to_string(),
                });
            }
            parse_swift_chars(line, "Field 50F name/address")?;
            name_and_address.push(line.to_string());
        }

        Ok(Field50F {
            account: account.to_string(),
            party_identifier,
            name_and_address: if name_and_address.is_empty() { None } else { Some(name_and_address) },
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut lines = vec![self.account.clone()];

        if let Some(ref party_id) = self.party_identifier {
            lines.push(format!("/{}", party_id));
        }

        if let Some(ref addr) = self.name_and_address {
            lines.extend(addr.clone());
        }

        lines.push(self.bic.clone());

        format!(":50F:{}", lines.join("\n"))
    }
}

/// **Field 50K: Ordering Customer (Option K)**
///
/// Flexible variant of [Field 50 module](index.html). Provides ordering customer identification
/// using optional account information and free-format name/address details.
///
/// **Components:**
/// - Account (optional, \[/34x\])
/// - Name and address lines (4*35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50K {
    /// Optional account identifier (free format)
    /// Format: \[/34x\] - Up to 34 characters with leading slash
    pub account: Option<String>,

    /// Name and address information in free format
    /// Format: 4*35x - Up to 4 lines of 35 characters each
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

/// **Field 50C: Ordering Customer (Option C)**
///
/// BIC-only variant of [Field 50 module](index.html).
///
/// **Components:**
/// - BIC (4!a2!a2!c\[3!c\])
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50C {
    /// BIC code
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

/// **Field 50L: Ordering Customer (Option L)**
///
/// Party identifier variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Party identifier (35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50L {
    /// Party identifier
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

/// **Field 50G: Ordering Customer (Option G)**
///
/// Account + BIC variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Account (/34x)
/// - BIC (4!a2!a2!c\[3!c\])
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50G {
    /// Account (with leading slash)
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

/// **Field 50H: Ordering Customer (Option H)**
///
/// Account + Name/Address variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Account (/34x)
/// - Name and address lines (4*35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field50H {
    /// Account (with leading slash)
    pub account: String,
    /// Name and address lines
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
            // Check if second line is a BIC
            if (8..=11).contains(&lines[1].len()) {
                // Could be F or G
                if lines[0].starts_with('/') {
                    // Option G: /account + BIC
                    if let Ok(field) = Field50G::parse(input) {
                        return Ok(Field50OrderingCustomerFGH::G(field));
                    }
                } else {
                    // Option F: account + BIC
                    if let Ok(field) = Field50F::parse(input) {
                        return Ok(Field50OrderingCustomerFGH::F(field));
                    }
                }
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
        // Try Option A first (numbered lines)
        let lines: Vec<&str> = input.lines().collect();

        // Check for numbered lines (characteristic of Option A)
        let mut has_numbered_lines = false;
        for line in &lines {
            let mut chars = line.chars();
            if line.len() >= 2
                && chars.next().is_some_and(|c| c.is_ascii_digit())
                && chars.next() == Some('/')
            {
                has_numbered_lines = true;
                break;
            }
        }

        if has_numbered_lines && let Ok(field) = Field50A::parse(input) {
            return Ok(Field50OrderingCustomerAFK::A(field));
        }

        // Try Option F (account + BIC)
        if lines.len() == 2
            && (8..=11).contains(&lines[1].len())
            && let Ok(field) = Field50F::parse(input)
        {
            return Ok(Field50OrderingCustomerAFK::F(field));
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

        // Try Option F (account + BIC)
        if lines.len() == 2
            && (8..=11).contains(&lines[1].len())
            && let Ok(field) = Field50F::parse(input)
        {
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
        // Check for numbered lines (characteristic of Option A)
        let lines: Vec<&str> = input.lines().collect();

        for line in &lines {
            let mut chars = line.chars();
            if line.len() >= 2
                && chars.next().is_some_and(|c| c.is_ascii_digit())
                && chars.next() == Some('/')
            {
                // Has numbered lines, try Option A
                if let Ok(field) = Field50A::parse(input) {
                    return Ok(Field50Creditor::A(field));
                }
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
        let field =
            Field50A::parse("/US123456789\n1/ACME CORP\n2/123 MAIN ST\n3/NEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("US123456789".to_string()));
        assert_eq!(field.name_and_address.len(), 3);
        assert_eq!(field.name_and_address[0], "ACME CORP");

        let swift_str = field.to_swift_string();
        assert!(swift_str.starts_with(":50A:"));
        assert!(swift_str.contains("/US123456789"));
        assert!(swift_str.contains("1/ACME CORP"));
    }

    #[test]
    fn test_field50f() {
        let field = Field50F::parse("ACCOUNT123\nDEUTDEFFXXX").unwrap();
        assert_eq!(field.account, "ACCOUNT123");
        assert_eq!(field.bic, "DEUTDEFFXXX");
        assert_eq!(field.to_swift_string(), ":50F:ACCOUNT123\nDEUTDEFFXXX");
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
        // Test Option A
        let field = Field50OrderingCustomerAFK::parse("1/ACME CORP\n2/NEW YORK").unwrap();
        assert!(matches!(field, Field50OrderingCustomerAFK::A(_)));

        // Test Option K
        let field = Field50OrderingCustomerAFK::parse("/ACC123\nJOHN DOE").unwrap();
        assert!(matches!(field, Field50OrderingCustomerAFK::K(_)));

        // Test Option F
        let field = Field50OrderingCustomerAFK::parse("ACCOUNT\nDEUTDEFF").unwrap();
        assert!(matches!(field, Field50OrderingCustomerAFK::F(_)));
    }
}
