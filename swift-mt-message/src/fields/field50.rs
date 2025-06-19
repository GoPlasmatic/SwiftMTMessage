use crate::fields::{GenericBicField, MultiLineField};
use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// # Field 50: Ordering Customer
///
/// ## Overview
/// Field 50 identifies the ordering customer (originator) of a SWIFT payment message.
/// This field is crucial for identifying the party who initiated the payment instruction
/// and is used for compliance, audit, and customer relationship purposes. The field
/// supports multiple format options (A, F, K) to accommodate different identification
/// methods and regulatory requirements.
///
/// ## Format Specification
/// Field 50 supports three format options:
///
/// ### Option A (50A): BIC-based Identification
/// **Format**: `[/account]BIC`
/// - **account**: Optional account number (max 34 characters, preceded by /)
/// - **BIC**: Bank Identifier Code (8 or 11 characters)
/// - **Structure**: Account and BIC on separate lines if account present
///
/// ### Option F (50F): Party Identifier with Name/Address
/// **Format**: `party_identifier + 4*35x`
/// - **party_identifier**: Unique party identifier (max 35 characters)
/// - **name_and_address**: Up to 4 lines of 35 characters each
/// - **Usage**: For structured party identification with full name/address
///
/// ### Option K (50K): Name and Address Only
/// **Format**: `4*35x`
/// - **name_and_address**: Up to 4 lines of 35 characters each
/// - **Usage**: Simple name/address format without structured identifiers
/// - **Most common**: Default option when no BIC or party ID available
///
/// ## Usage Context
/// Field 50 is mandatory in most SWIFT payment messages:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
///
/// ### Regulatory Applications
/// - **AML/KYC**: Customer identification for anti-money laundering compliance
/// - **Sanctions screening**: Identifying parties for sanctions compliance
/// - **FATCA/CRS**: Tax reporting requirements
/// - **Audit trails**: Maintaining originator information for investigations
/// - **Customer due diligence**: Enhanced due diligence requirements
///
/// ## Examples
/// ```text
/// :50A:/1234567890
/// DEUTDEFFXXX
/// └─── Customer with account 1234567890 at Deutsche Bank Frankfurt
///
/// :50A:CHASUS33XXX
/// └─── Customer identified by BIC only (JPMorgan Chase New York)
///
/// :50F:PARTY123456789
/// JOHN DOE ENTERPRISES
/// 123 BUSINESS AVENUE
/// NEW YORK NY 10001
/// UNITED STATES
/// └─── Customer with party identifier and full address
///
/// :50K:ACME CORPORATION
/// 456 INDUSTRIAL DRIVE
/// CHICAGO IL 60601
/// UNITED STATES
/// └─── Customer with name and address only
/// ```
///
/// ## Option Selection Guidelines
/// - **Use 50A when**: Customer has account at known financial institution with BIC
/// - **Use 50F when**: Structured party identification required (regulatory compliance)
/// - **Use 50K when**: Simple name/address sufficient, no structured ID available
/// - **Preference order**: 50A > 50F > 50K (from most to least structured)
///
/// ## Validation Rules
/// ### Option A (50A):
/// 1. **BIC validation**: Must be valid 8 or 11 character BIC format
/// 2. **Account format**: If present, max 34 characters, must start with /
/// 3. **BIC structure**: 4!a2!a2!c[3!c] format required
///
/// ### Option F (50F):
/// 1. **Party identifier**: Cannot be empty, max 35 characters
/// 2. **Address lines**: Minimum 1, maximum 4 lines
/// 3. **Line length**: Each line max 35 characters
/// 4. **Character set**: Printable ASCII characters only
///
/// ### Option K (50K):
/// 1. **Address lines**: Minimum 1, maximum 4 lines
/// 2. **Line length**: Each line max 35 characters
/// 3. **Character set**: Printable ASCII characters only
/// 4. **Content validation**: Must contain meaningful customer information
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Field 50 is mandatory in customer payment messages (Error: C23)
/// - BIC in option A must be valid format (Error: T27)
/// - Account number cannot exceed 34 characters (Error: T15)
/// - Name/address lines cannot exceed 35 characters each (Error: T14)
/// - Maximum 4 name/address lines allowed (Error: T16)
/// - Characters must be from SWIFT character set (Error: T61)
///
///
/// Field 50F: Ordering Customer (Option F)
///
/// Format: Party identifier and name/address lines
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field50F {
    /// Party identifier
    pub party_identifier: String,
    /// Name and address lines (up to 4 lines)
    pub name_and_address: Vec<String>,
}

impl Field50F {
    /// Create a new Field50F with validation
    pub fn new(
        party_identifier: impl Into<String>,
        name_and_address: Vec<String>,
    ) -> Result<Self, ParseError> {
        let party_identifier = party_identifier.into().trim().to_string();

        if party_identifier.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50F".to_string(),
                message: "Party identifier cannot be empty".to_string(),
            });
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50F".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50F".to_string(),
                message: "Too many name/address lines (max 4)".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "50F".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }
        }

        Ok(Field50F {
            party_identifier,
            name_and_address,
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
}

/// Field 50K: Ordering Customer (Option K)
///
/// Format: Name and address lines (up to 4 lines)
///
/// This field now implements the MultiLineField trait for consistent validation
/// and processing while maintaining the same public API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field50K {
    /// Name and address lines (up to 4 lines)
    pub name_and_address: Vec<String>,
}

impl MultiLineField for Field50K {
    const MAX_LINES: usize = 4;
    const FIELD_TAG: &'static str = "50K";

    fn lines(&self) -> &[String] {
        &self.name_and_address
    }

    fn lines_mut(&mut self) -> &mut Vec<String> {
        &mut self.name_and_address
    }

    fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
        Ok(Field50K {
            name_and_address: lines,
        })
    }
}

impl Field50K {
    /// Create a new Field50K with validation
    pub fn new(name_and_address: Vec<String>) -> Result<Self, ParseError> {
        <Self as MultiLineField>::new(name_and_address)
    }

    /// Get the name and address lines
    pub fn name_and_address(&self) -> &[String] {
        &self.name_and_address
    }
}

/// Field 50: Ordering Customer (with options A, F, and K)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field50 {
    A(GenericBicField),
    F(Field50F),
    K(Field50K),
}

impl Field50 {
    /// Parse Field50 with a specific tag (50A, 50F, or 50K)
    pub fn parse_with_tag(tag: &str, content: &str) -> Result<Self, ParseError> {
        match tag {
            "50A" => Ok(Field50::A(GenericBicField::parse(content).map_err(
                |e| ParseError::InvalidFieldFormat {
                    field_tag: "50A".to_string(),
                    message: format!("GenericBicField parse error: {}", e),
                },
            )?)),
            "50F" => Ok(Field50::F(Field50F::parse(content)?)),
            "50K" => Ok(Field50::K(Field50K::parse(content)?)),
            _ => Err(ParseError::InvalidFieldFormat {
                field_tag: "50".to_string(),
                message: format!("Unknown Field50 option: {}", tag),
            }),
        }
    }

    /// Get the tag for this field variant
    pub fn tag(&self) -> &'static str {
        match self {
            Field50::A(_) => "50A",
            Field50::F(_) => "50F",
            Field50::K(_) => "50K",
        }
    }
}

impl std::fmt::Display for Field50 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field50::A(field) => write!(f, "50A: {}", field),
            Field50::F(field) => write!(
                f,
                "50F: {} - {}",
                field.party_identifier,
                field.name_and_address.join(", ")
            ),
            Field50::K(field) => write!(f, "50K: {}", field.name_and_address().join(", ")),
        }
    }
}

// Custom serialization for Field50 to flatten the JSON structure
impl Serialize for Field50 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Field50::A(field) => field.serialize(serializer),
            Field50::F(field) => field.serialize(serializer),
            Field50::K(field) => field.serialize(serializer),
        }
    }
}

// Custom deserialization for Field50
impl<'de> Deserialize<'de> for Field50 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Field50Visitor;

        impl<'de> Visitor<'de> for Field50Visitor {
            type Value = Field50;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Field50 variant")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Field50, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut fields = std::collections::HashMap::new();

                while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
                    fields.insert(key, value);
                }

                // Try to determine the variant based on the fields present
                if fields.contains_key("bic") {
                    // Field50A variant - deserialize as GenericBicField
                    let account_number = fields
                        .get("account_number")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let account_line_indicator = fields
                        .get("account_line_indicator")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let bic = fields
                        .get("bic")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| de::Error::missing_field("bic"))?
                        .to_string();

                    GenericBicField::new(account_line_indicator, account_number, bic)
                        .map(Field50::A)
                        .map_err(|e| {
                            de::Error::custom(format!("GenericBicField validation error: {}", e))
                        })
                } else if fields.contains_key("party_identifier") {
                    // Field50F variant
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

                    Field50F::new(party_identifier, name_and_address)
                        .map(Field50::F)
                        .map_err(|e| de::Error::custom(format!("Field50F validation error: {}", e)))
                } else if fields.contains_key("name_and_address") {
                    // Field50K variant
                    let name_and_address = fields
                        .get("name_and_address")
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| de::Error::missing_field("name_and_address"))?
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect();

                    Field50K::new(name_and_address)
                        .map(Field50::K)
                        .map_err(|e| de::Error::custom(format!("Field50K validation error: {}", e)))
                } else {
                    Err(de::Error::custom("Unable to determine Field50 variant"))
                }
            }
        }

        deserializer.deserialize_map(Field50Visitor)
    }
}

impl SwiftField for Field50F {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let content = if let Some(stripped) = content.strip_prefix(":50F:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("50F:") {
            stripped
        } else {
            content
        };

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50F".to_string(),
                message: "No content provided".to_string(),
            });
        }

        Field50F::new(
            lines[0].to_string(),
            lines[1..].iter().map(|s| s.to_string()).collect(),
        )
    }

    fn to_swift_string(&self) -> String {
        let mut content = self.party_identifier.clone();
        for line in &self.name_and_address {
            content.push('\n');
            content.push_str(line);
        }
        format!(":50F:{}", content)
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let errors = if self.party_identifier.is_empty() {
            vec![ValidationError::FormatValidation {
                field_tag: "50F".to_string(),
                message: "Party identifier cannot be empty".to_string(),
            }]
        } else {
            vec![]
        };

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: vec![],
        }
    }

    fn format_spec() -> &'static str {
        "party_identifier_and_name"
    }
}

impl SwiftField for Field50K {
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

impl SwiftField for Field50 {
    fn parse(input: &str) -> Result<Self, ParseError> {
        // Try to determine the variant from the input
        if input.starts_with(":50A:") || input.starts_with("50A:") {
            Ok(Field50::A(GenericBicField::parse(input).map_err(|e| {
                ParseError::InvalidFieldFormat {
                    field_tag: "50A".to_string(),
                    message: format!("GenericBicField parse error: {}", e),
                }
            })?))
        } else if input.starts_with(":50F:") || input.starts_with("50F:") {
            Ok(Field50::F(Field50F::parse(input)?))
        } else if input.starts_with(":50K:") || input.starts_with("50K:") {
            Ok(Field50::K(Field50K::parse(input)?))
        } else {
            // Default to 50K if no clear indicator
            Ok(Field50::K(Field50K::parse(input)?))
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50::A(field) => {
                // Convert GenericBicField to 50A format
                let content = if let Some(account) = field.account_number() {
                    format!("/{}\n{}", account, field.bic())
                } else {
                    field.bic().to_string()
                };
                format!(":50A:{}", content)
            }
            Field50::F(field) => field.to_swift_string(),
            Field50::K(field) => field.to_swift_string(),
        }
    }

    fn validate(&self) -> ValidationResult {
        match self {
            Field50::A(field) => field.validate(),
            Field50::F(field) => field.validate(),
            Field50::K(field) => field.validate(),
        }
    }

    fn format_spec() -> &'static str {
        "ordering_customer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field50a_creation() {
        let field =
            GenericBicField::new(None, Some("123456789".to_string()), "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account_number(), Some("123456789"));
        assert_eq!(field.bic(), "DEUTDEFFXXX");

        let field50 = Field50::A(field);
        assert_eq!(field50.to_swift_string(), ":50A:/123456789\nDEUTDEFFXXX");
    }

    #[test]
    fn test_field50a_without_account() {
        let field = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account_number(), None);
        assert_eq!(field.bic(), "DEUTDEFFXXX");

        let field50 = Field50::A(field);
        assert_eq!(field50.to_swift_string(), ":50A:DEUTDEFFXXX");
    }

    #[test]
    fn test_field50a_parse() {
        let field = Field50::parse(":50A:/123456789\nDEUTDEFFXXX").unwrap();
        if let Field50::A(bic_field) = field {
            assert_eq!(bic_field.account_number(), Some("123456789"));
            assert_eq!(bic_field.bic(), "DEUTDEFFXXX");
        } else {
            panic!("Expected Field50::A variant");
        }

        let field = Field50::parse(":50A:DEUTDEFFXXX").unwrap();
        if let Field50::A(bic_field) = field {
            assert_eq!(bic_field.account_number(), None);
            assert_eq!(bic_field.bic(), "DEUTDEFFXXX");
        } else {
            panic!("Expected Field50::A variant");
        }
    }

    #[test]
    fn test_field50f_creation() {
        let field = Field50F::new(
            "PARTY123",
            vec!["JOHN DOE".to_string(), "123 MAIN ST".to_string()],
        )
        .unwrap();
        assert_eq!(field.party_identifier(), "PARTY123");
        assert_eq!(field.name_and_address(), &["JOHN DOE", "123 MAIN ST"]);
    }

    #[test]
    fn test_field50f_parse() {
        let field = Field50F::parse("PARTY123\nJOHN DOE\n123 MAIN ST").unwrap();
        assert_eq!(field.party_identifier(), "PARTY123");
        assert_eq!(field.name_and_address(), &["JOHN DOE", "123 MAIN ST"]);
    }

    #[test]
    fn test_field50k_creation() {
        let field = Field50K::new(vec!["JOHN DOE".to_string(), "123 MAIN ST".to_string()]).unwrap();
        assert_eq!(field.name_and_address(), &["JOHN DOE", "123 MAIN ST"]);
    }

    #[test]
    fn test_field50k_parse() {
        let field = Field50K::parse("JOHN DOE\n123 MAIN ST").unwrap();
        assert_eq!(field.name_and_address(), &["JOHN DOE", "123 MAIN ST"]);
    }

    #[test]
    fn test_field50_enum_parse() {
        let field = Field50::parse(":50A:DEUTDEFFXXX").unwrap();
        assert!(matches!(field, Field50::A(_)));

        let field = Field50::parse(":50F:PARTY123\nJOHN DOE").unwrap();
        assert!(matches!(field, Field50::F(_)));

        let field = Field50::parse(":50K:JOHN DOE\n123 MAIN ST").unwrap();
        assert!(matches!(field, Field50::K(_)));
    }

    #[test]
    fn test_field50_parse_with_tag() {
        let field = Field50::parse_with_tag("50A", "DEUTDEFFXXX").unwrap();
        assert!(matches!(field, Field50::A(_)));

        let field = Field50::parse_with_tag("50F", "PARTY123\nJOHN DOE").unwrap();
        assert!(matches!(field, Field50::F(_)));

        let field = Field50::parse_with_tag("50K", "JOHN DOE\n123 MAIN ST").unwrap();
        assert!(matches!(field, Field50::K(_)));
    }

    #[test]
    fn test_field50_tag() {
        let field = Field50::A(GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap());
        assert_eq!(field.tag(), "50A");

        let field = Field50::F(Field50F::new("PARTY123", vec!["JOHN DOE".to_string()]).unwrap());
        assert_eq!(field.tag(), "50F");

        let field = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        assert_eq!(field.tag(), "50K");
    }

    #[test]
    fn test_field50_validation() {
        let field = GenericBicField::new(None, None, "DEUTDEFF").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let field = Field50F::new("PARTY123", vec!["JOHN DOE".to_string()]).unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let field = Field50K::new(vec!["JOHN DOE".to_string()]).unwrap();
        let result = field.validate();
        assert!(result.is_valid);
    }

    #[test]
    fn test_field50_json_serialization_flattened() {
        // Test Field50K (most common case)
        let field50k = Field50::K(
            Field50K::new(vec!["JOHN DOE".to_string(), "123 MAIN ST".to_string()]).unwrap(),
        );

        let json = serde_json::to_string(&field50k).unwrap();
        println!("Field50K JSON: {}", json);

        // Should be flattened - no "K" wrapper
        assert!(json.contains("\"name_and_address\""));
        assert!(!json.contains("\"K\""));

        // Test Field50A
        let field50a = Field50::A(
            GenericBicField::new(None, Some("123456789".to_string()), "DEUTDEFFXXX").unwrap(),
        );

        let json = serde_json::to_string(&field50a).unwrap();
        println!("Field50A JSON: {}", json);

        // Should be flattened - no "A" wrapper
        assert!(json.contains("\"account_number\""));
        assert!(json.contains("\"bic\""));
        assert!(!json.contains("\"A\""));

        // Test Field50F
        let field50f = Field50::F(Field50F::new("PARTY123", vec!["JOHN DOE".to_string()]).unwrap());

        let json = serde_json::to_string(&field50f).unwrap();
        println!("Field50F JSON: {}", json);

        // Should be flattened - no "F" wrapper
        assert!(json.contains("\"party_identifier\""));
        assert!(json.contains("\"name_and_address\""));
        assert!(!json.contains("\"F\""));
    }

    #[test]
    fn test_field50_json_deserialization() {
        // Test deserializing Field50K
        let json = r#"{"name_and_address":["JOHN DOE","123 MAIN ST"]}"#;
        let field: Field50 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field50::K(_)));

        // Test deserializing Field50A
        let json = r#"{"account_number":"123456789","bic":"DEUTDEFFXXX"}"#;
        let field: Field50 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field50::A(_)));

        // Test deserializing Field50F
        let json = r#"{"party_identifier":"PARTY123","name_and_address":["JOHN DOE"]}"#;
        let field: Field50 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field50::F(_)));
    }
}
