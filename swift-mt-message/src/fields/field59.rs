use crate::MultiLineField;
use crate::{GenericBicField, SwiftField, ValidationResult, errors::ParseError};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// # Field 59: Beneficiary Customer
///
/// ## Overview
/// Field 59 identifies the beneficiary customer in SWIFT payment messages, representing the
/// ultimate recipient of the payment funds. This field supports multiple identification
/// options (59A, 59F, and 59 without letter) to accommodate different beneficiary types
/// and identification requirements. The beneficiary customer is the final destination for
/// payment funds and must be clearly identified for regulatory compliance and payment delivery.
///
/// ## Format Specification
/// ### Field 59A (BIC Option)
/// **Format**: `[/account]BIC`
/// - **account**: Optional account number (up to 34 characters)
/// - **BIC**: Bank Identifier Code (8 or 11 characters)
///
/// ### Field 59F (Party Identifier Option)
/// **Format**: `party_identifier + 4*(1!n/33x)`
/// - **party_identifier**: Up to 35 characters
/// - **name_and_address**: Up to 4 structured lines (format: n/text)
///
/// ### Field 59 (Basic Option)
/// **Format**: `4*35x`
/// - **beneficiary_customer**: Up to 4 lines of 35 characters each
///
/// ## Structure
/// ```text
/// Option A (BIC):
/// /DE89370400440532013000
/// DEUTDEFFXXX
///
/// Option F (Party ID):
/// PARTYID123456789
/// 1/JOHN DOE
/// 2/123 MAIN STREET
/// 3/NEW YORK NY 10001
/// 4/UNITED STATES
///
/// Basic Option:
/// MUELLER GMBH
/// HAUPTSTRASSE 1
/// 60311 FRANKFURT
/// GERMANY
/// ```
///
/// ## Field Components
/// - **Account Number**: Beneficiary's account identifier (Option A)
/// - **BIC Code**: Bank Identifier Code (Option A)
/// - **Party Identifier**: Structured party identification (Option F)
/// - **Name and Address**: Beneficiary details (Options F and Basic)
/// - **Line Numbers**: Structured addressing (Option F: 1-4)
///
/// ## Usage Context
/// Field 59 variants are used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Payment delivery**: Identifying final payment recipient
/// - **Regulatory compliance**: Meeting beneficiary identification requirements
/// - **AML/KYC compliance**: Supporting anti-money laundering checks
/// - **Payment transparency**: Providing clear beneficiary details
/// - **Cross-border payments**: International beneficiary identification
/// - **Sanctions screening**: Enabling compliance checks
///
/// ## Examples
/// ```text
/// :59A:/DE89370400440532013000
/// DEUTDEFFXXX
/// └─── German bank customer with IBAN and BIC
///
/// :59F:PARTYID123456789
/// 1/JOHN DOE
/// 2/123 MAIN STREET
/// 3/NEW YORK NY 10001
/// 4/UNITED STATES
/// └─── Structured party identification with address
///
/// :59:MUELLER GMBH
/// HAUPTSTRASSE 1
/// 60311 FRANKFURT
/// GERMANY
/// └─── Basic beneficiary identification
/// ```
///
/// ## Validation Rules
/// ### Option A (BIC)
/// 1. **BIC format**: Must be valid 8 or 11 character BIC
/// 2. **Account number**: Maximum 34 characters (optional)
/// 3. **Character validation**: SWIFT character set compliance
///
/// ### Option F (Party Identifier)
/// 1. **Party identifier**: Maximum 35 characters, required
/// 2. **Name/address lines**: Maximum 4 lines, format n/text
/// 3. **Line content**: Maximum 33 characters per line
/// 4. **Line numbers**: Must be 1-4
///
/// ### Basic Option
/// 1. **Line count**: Maximum 4 lines
/// 2. **Line length**: Maximum 35 characters per line
/// 3. **Content**: Must contain meaningful beneficiary information
///
/// ## Network Validated Rules (SWIFT Standards)
/// - BIC must be valid if used (Error: T10)
/// - Account number cannot exceed 34 characters (Error: T14)
/// - Party identifier cannot exceed 35 characters (Error: T50)
/// - Name/address lines cannot exceed specified limits (Error: T26)
/// - Must use SWIFT character set only (Error: T61)
/// - Beneficiary must be identifiable (Error: T51)
/// - Only one 59 option per message (Error: C59)
///
/// Field 59F: Beneficiary Customer (Option F)
///
/// Format: 35x (party identifier) + 4*(1!n/33x) (name and address)
///
/// This field specifies the beneficiary customer using a party identifier and structured name/address.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field59F {
    /// Party identifier (up to 35 characters)
    pub party_identifier: String,
    /// Name and address lines (up to 4 lines in format 1/Name, 2/Address, etc.)
    pub name_and_address: Vec<String>,
}

impl Field59F {
    /// Create a new Field59F with validation
    pub fn new(
        party_identifier: impl Into<String>,
        name_and_address: Vec<String>,
    ) -> Result<Self, ParseError> {
        let party_identifier = party_identifier.into().trim().to_string();

        if party_identifier.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59F".to_string(),
                message: "Party identifier cannot be empty".to_string(),
            });
        }

        if party_identifier.len() > 35 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59F".to_string(),
                message: "Party identifier cannot exceed 35 characters".to_string(),
            });
        }

        if !party_identifier
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59F".to_string(),
                message: "Party identifier contains invalid characters".to_string(),
            });
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59F".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59F".to_string(),
                message: "Cannot exceed 4 name and address lines".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!("Name and address line {} cannot be empty", i + 1),
                });
            }

            // Validate format: should be n/text where n is 1-4
            if !trimmed.starts_with(char::is_numeric) || !trimmed.contains('/') {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!("Name and address line {} must be in format 'n/text'", i + 1),
                });
            }

            let parts: Vec<&str> = trimmed.splitn(2, '/').collect();
            if parts.len() != 2 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!(
                        "Name and address line {} must contain exactly one '/'",
                        i + 1
                    ),
                });
            }

            let line_number: u8 = parts[0]
                .parse()
                .map_err(|_| ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!(
                        "Name and address line {} must start with a number 1-4",
                        i + 1
                    ),
                })?;

            if !(1..=4).contains(&line_number) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!(
                        "Name and address line {} number must be between 1 and 4",
                        i + 1
                    ),
                });
            }

            if parts[1].is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!("Name and address line {} content cannot be empty", i + 1),
                });
            }

            if parts[1].len() > 33 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!(
                        "Name and address line {} content cannot exceed 33 characters",
                        i + 1
                    ),
                });
            }

            if !parts[1].chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59F".to_string(),
                    message: format!(
                        "Name and address line {} contains invalid characters",
                        i + 1
                    ),
                });
            }
        }

        let trimmed_lines: Vec<String> = name_and_address
            .iter()
            .map(|line| line.trim().to_string())
            .collect();

        Ok(Field59F {
            party_identifier,
            name_and_address: trimmed_lines,
        })
    }

    /// Get the party identifier
    pub fn party_identifier(&self) -> &str {
        &self.party_identifier
    }

    /// Get the name and address lines
    pub fn name_and_address(&self) -> &[String] {
        &self.name_and_address
    }

    /// Get a specific name/address line by line number (1-4)
    pub fn name_address_line(&self, line_number: u8) -> Option<&str> {
        self.name_and_address
            .iter()
            .find(|line| line.starts_with(&format!("{}/", line_number)))
            .map(|line| line.split_once('/').map(|x| x.1).unwrap_or(""))
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Beneficiary Customer (Party ID: {}, Name/Address: {})",
            self.party_identifier,
            self.name_and_address.join(", ")
        )
    }
}

impl std::fmt::Display for Field59F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Party: {}, Name/Address: {}",
            self.party_identifier,
            self.name_and_address.join(" / ")
        )
    }
}

impl SwiftField for Field59F {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let content = if let Some(stripped) = content.strip_prefix(":59F:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("59F:") {
            stripped
        } else {
            content
        };

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59F".to_string(),
                message: "No content provided".to_string(),
            });
        }

        let party_identifier = lines[0].to_string();
        let name_and_address = lines[1..].iter().map(|s| s.to_string()).collect();

        Field59F::new(party_identifier, name_and_address)
    }

    fn to_swift_string(&self) -> String {
        let mut content = self.party_identifier.clone();
        for line in &self.name_and_address {
            content.push('\n');
            content.push_str(line);
        }
        format!(":59F:{}", content)
    }

    fn validate(&self) -> ValidationResult {
        // Validation is done in constructor
        ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "party_identifier_and_name_address"
    }
}

/// Field 59: Beneficiary Customer (Basic)
///
/// Format: 4*35x (up to 4 lines of 35 characters each)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field59Basic {
    /// Beneficiary customer information (up to 4 lines)
    pub beneficiary_customer: Vec<String>,
}

impl MultiLineField for Field59Basic {
    const MAX_LINES: usize = 4;
    const FIELD_TAG: &'static str = "59";

    fn lines(&self) -> &[String] {
        &self.beneficiary_customer
    }

    fn lines_mut(&mut self) -> &mut Vec<String> {
        &mut self.beneficiary_customer
    }

    fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
        Ok(Field59Basic {
            beneficiary_customer: lines,
        })
    }
}

impl Field59Basic {
    /// Create a new Field59Basic with validation
    pub fn new(beneficiary_customer: Vec<String>) -> Result<Self, ParseError> {
        <Self as MultiLineField>::new(beneficiary_customer)
    }

    /// Get the beneficiary customer lines
    pub fn beneficiary_customer(&self) -> &[String] {
        &self.beneficiary_customer
    }
}

/// Field 59: Beneficiary Customer (with options A, F, and no letter option)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field59 {
    A(GenericBicField),
    F(Field59F),
    NoOption(Field59Basic),
}

impl Field59 {
    /// Parse Field59 with a specific tag (59A, 59F, or 59)
    pub fn parse_with_tag(tag: &str, content: &str) -> Result<Self, ParseError> {
        match tag {
            "59A" => {
                let content = if let Some(stripped) = content.strip_prefix(":59A:") {
                    stripped
                } else if let Some(stripped) = content.strip_prefix("59A:") {
                    stripped
                } else {
                    content
                };

                let mut lines = content.lines();
                let first_line = lines.next().unwrap_or_default();

                let (account, bic_line) = if let Some(stripped) = first_line.strip_prefix('/') {
                    (Some(stripped.to_string()), lines.next().unwrap_or_default())
                } else {
                    (None, first_line)
                };

                Ok(Field59::A(
                    GenericBicField::new(None, account, bic_line.to_string()).map_err(|e| {
                        ParseError::InvalidFieldFormat {
                            field_tag: "59A".to_string(),
                            message: e,
                        }
                    })?,
                ))
            }
            "59F" => Ok(Field59::F(Field59F::parse(content)?)),
            "59" => Ok(Field59::NoOption(Field59Basic::parse(content)?)),
            _ => Err(ParseError::InvalidFieldFormat {
                field_tag: "59".to_string(),
                message: format!("Unknown Field59 option: {}", tag),
            }),
        }
    }

    /// Get the tag for this field variant
    pub fn tag(&self) -> &'static str {
        match self {
            Field59::A(_) => "59A",
            Field59::F(_) => "59F",
            Field59::NoOption(_) => "59",
        }
    }
}

impl std::fmt::Display for Field59 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field59::A(field) => match &field.account_number {
                Some(account) => write!(f, "59A: /{} {}", account, field.bic),
                None => write!(f, "59A: {}", field.bic),
            },
            Field59::F(field) => write!(f, "59F: {}", field),
            Field59::NoOption(field) => {
                write!(f, "59: {}", field.beneficiary_customer.join(", "))
            }
        }
    }
}

// Custom serialization for Field59 to flatten the JSON structure
impl Serialize for Field59 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Field59::A(field) => field.serialize(serializer),
            Field59::F(field) => field.serialize(serializer),
            Field59::NoOption(field) => field.serialize(serializer),
        }
    }
}

// Custom deserialization for Field59
impl<'de> Deserialize<'de> for Field59 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Field59Visitor;

        impl<'de> Visitor<'de> for Field59Visitor {
            type Value = Field59;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Field59 variant")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Field59, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut fields = std::collections::HashMap::new();

                while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                    fields.insert(key, value);
                }

                // Try to determine the variant based on the fields present
                if fields.contains_key("bic") {
                    // Field59A variant
                    let account = fields
                        .get("account_number")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let bic = fields
                        .get("bic")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| de::Error::missing_field("bic"))?
                        .to_string();

                    GenericBicField::new(None, account, bic)
                        .map(Field59::A)
                        .map_err(|e| de::Error::custom(format!("Field59A validation error: {}", e)))
                } else if fields.contains_key("party_identifier")
                    && fields.contains_key("name_and_address")
                {
                    // Field59F variant
                    let party_identifier = fields
                        .get("party_identifier")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| de::Error::missing_field("party_identifier"))?
                        .to_string();
                    let name_and_address = fields
                        .get("name_and_address")
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| de::Error::missing_field("name_and_address"))?
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect();

                    Field59F::new(party_identifier, name_and_address)
                        .map(Field59::F)
                        .map_err(|e| de::Error::custom(format!("Field59F validation error: {}", e)))
                } else if fields.contains_key("beneficiary_customer") {
                    // Field59Basic variant
                    let beneficiary_customer = fields
                        .get("beneficiary_customer")
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| de::Error::missing_field("beneficiary_customer"))?
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect();

                    Field59Basic::new(beneficiary_customer)
                        .map(Field59::NoOption)
                        .map_err(|e| {
                            de::Error::custom(format!("Field59Basic validation error: {}", e))
                        })
                } else {
                    Err(de::Error::custom("Unable to determine Field59 variant"))
                }
            }
        }

        deserializer.deserialize_map(Field59Visitor)
    }
}

impl SwiftField for Field59Basic {
    fn parse(content: &str) -> Result<Self, ParseError> {
        Self::parse_content(content)
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_format()
    }

    fn validate(&self) -> ValidationResult {
        self.validate_multiline()
    }

    fn format_spec() -> &'static str {
        "4*35x"
    }
}

impl SwiftField for Field59 {
    fn parse(input: &str) -> Result<Self, ParseError> {
        // Try to determine the variant from the input
        if input.starts_with(":59A:") || input.starts_with("59A:") {
            let content = if let Some(stripped) = input.strip_prefix(":59A:") {
                stripped
            } else if let Some(stripped) = input.strip_prefix("59A:") {
                stripped
            } else {
                input
            };

            let mut lines = content.lines();
            let first_line = lines.next().unwrap_or_default();

            let (account, bic_line) = if let Some(stripped) = first_line.strip_prefix('/') {
                (Some(stripped.to_string()), lines.next().unwrap_or_default())
            } else {
                (None, first_line)
            };

            Ok(Field59::A(
                GenericBicField::new(None, account, bic_line.to_string()).map_err(|e| {
                    ParseError::InvalidFieldFormat {
                        field_tag: "59A".to_string(),
                        message: e,
                    }
                })?,
            ))
        } else if input.starts_with(":59F:") || input.starts_with("59F:") {
            Ok(Field59::F(Field59F::parse(input)?))
        } else if input.starts_with(":59:") || input.starts_with("59:") {
            Ok(Field59::NoOption(Field59Basic::parse(input)?))
        } else {
            // Default to NoOption if no clear indicator
            Ok(Field59::NoOption(Field59Basic::parse(input)?))
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field59::A(field) => match &field.account_number {
                Some(account) => format!(":59A:/{}\n{}", account, field.bic),
                None => format!(":59A:{}", field.bic),
            },
            Field59::F(field) => field.to_swift_string(),
            Field59::NoOption(field) => field.to_swift_string(),
        }
    }

    fn validate(&self) -> ValidationResult {
        match self {
            Field59::A(field) => field.validate(),
            Field59::F(field) => field.validate(),
            Field59::NoOption(field) => field.validate(),
        }
    }

    fn format_spec() -> &'static str {
        "beneficiary_customer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field59a_creation() {
        let field =
            GenericBicField::new(None, Some("123456789".to_string()), "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account_number(), Some("123456789"));
        assert_eq!(field.bic(), "DEUTDEFFXXX");
        let field59 = Field59::A(field);
        assert_eq!(field59.to_swift_string(), ":59A:/123456789\nDEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_without_account() {
        let field = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account_number(), None);
        assert_eq!(field.bic(), "DEUTDEFFXXX");
        let field59 = Field59::A(field);
        assert_eq!(field59.to_swift_string(), ":59A:DEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_parse() {
        let field = Field59::parse(":59A:/123456789\nDEUTDEFFXXX").unwrap();
        if let Field59::A(bic_field) = field {
            assert_eq!(bic_field.account_number(), Some("123456789"));
            assert_eq!(bic_field.bic(), "DEUTDEFFXXX");
        } else {
            panic!("Expected Field59::A variant");
        }

        let field = Field59::parse(":59A:DEUTDEFFXXX").unwrap();
        if let Field59::A(bic_field) = field {
            assert_eq!(bic_field.account_number(), None);
            assert_eq!(bic_field.bic(), "DEUTDEFFXXX");
        } else {
            panic!("Expected Field59::A variant");
        }
    }

    #[test]
    fn test_field59a_invalid_bic() {
        let result = GenericBicField::new(None, None, "INVALID"); // Too short
        assert!(result.is_err());

        let result = GenericBicField::new(None, None, "TOOLONGBICCODE"); // Too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field59_basic_creation() {
        let field = Field59Basic::new(vec![
            "MUELLER GMBH".to_string(),
            "HAUPTSTRASSE 1".to_string(),
        ])
        .unwrap();
        assert_eq!(
            field.beneficiary_customer(),
            &["MUELLER GMBH", "HAUPTSTRASSE 1"]
        );
    }

    #[test]
    fn test_field59_basic_parse() {
        let field = Field59Basic::parse("MUELLER GMBH\nHAUPTSTRASSE 1").unwrap();
        assert_eq!(
            field.beneficiary_customer(),
            &["MUELLER GMBH", "HAUPTSTRASSE 1"]
        );
    }

    #[test]
    fn test_field59_basic_too_many_lines() {
        let result = Field59Basic::new(vec![
            "LINE1".to_string(),
            "LINE2".to_string(),
            "LINE3".to_string(),
            "LINE4".to_string(),
            "LINE5".to_string(), // Too many
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_field59_enum_parse() {
        let field = Field59::parse(":59A:DEUTDEFFXXX").unwrap();
        assert!(matches!(field, Field59::A(_)));

        let field = Field59::parse(":59:MUELLER GMBH\nHAUPTSTRASSE 1").unwrap();
        assert!(matches!(field, Field59::NoOption(_)));
    }

    #[test]
    fn test_field59_parse_with_tag() {
        let field = Field59::parse_with_tag("59A", "DEUTDEFFXXX").unwrap();
        assert!(matches!(field, Field59::A(_)));

        let field = Field59::parse_with_tag("59", "MUELLER GMBH\nHAUPTSTRASSE 1").unwrap();
        assert!(matches!(field, Field59::NoOption(_)));
    }

    #[test]
    fn test_field59_tag() {
        let field = Field59::A(GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap());
        assert_eq!(field.tag(), "59A");

        let field = Field59::NoOption(Field59Basic::new(vec!["MUELLER GMBH".to_string()]).unwrap());
        assert_eq!(field.tag(), "59");
    }

    #[test]
    fn test_field59_validation() {
        let field = GenericBicField::new(None, None, "DEUTDEFF").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let field = Field59Basic::new(vec!["MUELLER GMBH".to_string()]).unwrap();
        let result = field.validate();
        assert!(result.is_valid);
    }

    #[test]
    fn test_field59_display() {
        let field59_1 =
            Field59::A(GenericBicField::new(None, Some("123456".to_string()), "DEUTDEFF").unwrap());
        assert_eq!(format!("{}", field59_1), "59A: /123456 DEUTDEFF");

        let field59_2 = Field59::A(GenericBicField::new(None, None, "DEUTDEFF").unwrap());
        assert_eq!(format!("{}", field59_2), "59A: DEUTDEFF");

        let field =
            Field59Basic::new(vec!["MUELLER GMBH".to_string(), "BERLIN".to_string()]).unwrap();
        let enum_field = Field59::NoOption(field);
        assert_eq!(format!("{}", enum_field), "59: MUELLER GMBH, BERLIN");
    }

    #[test]
    fn test_field59_json_serialization_flattened() {
        // Test Field59A
        let field59a = Field59::A(
            GenericBicField::new(None, Some("DE89370400440532013000".to_string()), "DEUTDEFF")
                .unwrap(),
        );

        let json = serde_json::to_string(&field59a).unwrap();
        println!("Field59A JSON: {}", json);

        // Should be flattened - no "A" wrapper
        assert!(json.contains("\"account_number\""));
        assert!(json.contains("\"bic\""));
        assert!(!json.contains("\"A\""));

        // Test Field59Basic (NoOption)
        let field59_basic = Field59::NoOption(
            Field59Basic::new(vec![
                "MUELLER GMBH".to_string(),
                "HAUPTSTRASSE 1".to_string(),
            ])
            .unwrap(),
        );

        let json = serde_json::to_string(&field59_basic).unwrap();
        println!("Field59Basic JSON: {}", json);

        // Should be flattened - no "NoOption" wrapper
        assert!(json.contains("\"beneficiary_customer\""));
        assert!(!json.contains("\"NoOption\""));
    }

    #[test]
    fn test_field59_json_deserialization() {
        // Test deserializing Field59A
        let json = r#"{"account_number":"DE89370400440532013000","bic":"DEUTDEFF"}"#;
        let field: Field59 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field59::A(_)));

        // Test deserializing Field59F
        let json = r#"{"party_identifier":"PARTYID123","name_and_address":["1/JOHN DOE","2/123 MAIN ST"]}"#;
        let field: Field59 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field59::F(_)));

        // Test deserializing Field59Basic
        let json = r#"{"beneficiary_customer":["MUELLER GMBH","HAUPTSTRASSE 1"]}"#;
        let field: Field59 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field59::NoOption(_)));
    }

    #[test]
    fn test_field59f_creation() {
        let field = Field59F::new(
            "PARTYID123",
            vec!["1/JOHN DOE".to_string(), "2/123 MAIN ST".to_string()],
        )
        .unwrap();
        assert_eq!(field.party_identifier(), "PARTYID123");
        assert_eq!(field.name_and_address(), &["1/JOHN DOE", "2/123 MAIN ST"]);
        assert_eq!(field.name_address_line(1), Some("JOHN DOE"));
        assert_eq!(field.name_address_line(2), Some("123 MAIN ST"));
        assert_eq!(field.name_address_line(3), None);
    }

    #[test]
    fn test_field59f_parse() {
        let field = Field59F::parse("PARTYID123\n1/JOHN DOE\n2/123 MAIN ST").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID123");
        assert_eq!(field.name_and_address(), &["1/JOHN DOE", "2/123 MAIN ST"]);
    }

    #[test]
    fn test_field59f_to_swift_string() {
        let field = Field59F::new(
            "PARTYID123",
            vec!["1/JOHN DOE".to_string(), "2/123 MAIN ST".to_string()],
        )
        .unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":59F:PARTYID123\n1/JOHN DOE\n2/123 MAIN ST"
        );
    }

    #[test]
    fn test_field59f_validation_errors() {
        // Empty party identifier
        let result = Field59F::new("", vec!["1/JOHN DOE".to_string()]);
        assert!(result.is_err());

        // Party identifier too long
        let result = Field59F::new("A".repeat(36), vec!["1/JOHN DOE".to_string()]);
        assert!(result.is_err());

        // Empty name and address
        let result = Field59F::new("PARTYID123", vec![]);
        assert!(result.is_err());

        // Too many name and address lines
        let result = Field59F::new(
            "PARTYID123",
            vec![
                "1/JOHN DOE".to_string(),
                "2/123 MAIN ST".to_string(),
                "3/CITY".to_string(),
                "4/COUNTRY".to_string(),
                "5/EXTRA".to_string(), // Too many
            ],
        );
        assert!(result.is_err());

        // Invalid name and address format
        let result = Field59F::new("PARTYID123", vec!["INVALID FORMAT".to_string()]);
        assert!(result.is_err());

        // Invalid line number
        let result = Field59F::new("PARTYID123", vec!["5/INVALID LINE NUMBER".to_string()]);
        assert!(result.is_err());

        // Name and address content too long
        let result = Field59F::new(
            "PARTYID123",
            vec![format!("1/{}", "A".repeat(34))], // 34 chars is too long (max 33)
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_field59f_display() {
        let field = Field59F::new(
            "PARTYID123",
            vec!["1/JOHN DOE".to_string(), "2/123 MAIN ST".to_string()],
        )
        .unwrap();
        assert_eq!(
            format!("{}", field),
            "Party: PARTYID123, Name/Address: 1/JOHN DOE / 2/123 MAIN ST"
        );
    }

    #[test]
    fn test_field59f_description() {
        let field = Field59F::new(
            "PARTYID123",
            vec!["1/JOHN DOE".to_string(), "2/123 MAIN ST".to_string()],
        )
        .unwrap();
        assert_eq!(
            field.description(),
            "Beneficiary Customer (Party ID: PARTYID123, Name/Address: 1/JOHN DOE, 2/123 MAIN ST)"
        );
    }

    #[test]
    fn test_field59_enum_with_f_variant() {
        let field = Field59::F(
            Field59F::new(
                "PARTYID123",
                vec!["1/JOHN DOE".to_string(), "2/123 MAIN ST".to_string()],
            )
            .unwrap(),
        );
        assert_eq!(field.tag(), "59F");
        assert_eq!(
            format!("{}", field),
            "59F: Party: PARTYID123, Name/Address: 1/JOHN DOE / 2/123 MAIN ST"
        );
    }

    #[test]
    fn test_field59_parse_with_tag_f() {
        let field =
            Field59::parse_with_tag("59F", "PARTYID123\n1/JOHN DOE\n2/123 MAIN ST").unwrap();
        assert!(matches!(field, Field59::F(_)));
        assert_eq!(field.tag(), "59F");
    }

    #[test]
    fn test_field59_parse_f_variant() {
        let field = Field59::parse(":59F:PARTYID123\n1/JOHN DOE\n2/123 MAIN ST").unwrap();
        assert!(matches!(field, Field59::F(_)));
        assert_eq!(
            field.to_swift_string(),
            ":59F:PARTYID123\n1/JOHN DOE\n2/123 MAIN ST"
        );
    }

    #[test]
    fn test_field59f_json_serialization() {
        let field = Field59::F(
            Field59F::new(
                "PARTYID123",
                vec!["1/JOHN DOE".to_string(), "2/123 MAIN ST".to_string()],
            )
            .unwrap(),
        );

        let json = serde_json::to_string(&field).unwrap();
        println!("Field59F JSON: {}", json);

        // Should be flattened - no "F" wrapper
        assert!(json.contains("\"party_identifier\""));
        assert!(json.contains("\"name_and_address\""));
        assert!(!json.contains("\"F\""));
    }
}
