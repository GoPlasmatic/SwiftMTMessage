use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Field 50A: Ordering Customer (Option A)
///
/// Format: [/account]\nBIC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field50A {
    /// Optional account number (starting with /)
    pub account: Option<String>,
    /// BIC code
    pub bic: String,
}

impl Field50A {
    /// Create a new Field50A with validation
    pub fn new(account: Option<String>, bic: impl Into<String>) -> Result<Self, ParseError> {
        let bic = bic.into().trim().to_string();

        if bic.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50A".to_string(),
                message: "BIC cannot be empty".to_string(),
            });
        }

        // Basic BIC validation (8 or 11 characters)
        if bic.len() != 8 && bic.len() != 11 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        // Validate account if present
        if let Some(ref acc) = account {
            if acc.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "50A".to_string(),
                    message: "Account cannot be empty if specified".to_string(),
                });
            }

            if acc.len() > 34 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "50A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }
        }

        Ok(Field50A { account, bic })
    }

    /// Get the account number
    pub fn account(&self) -> Option<&str> {
        self.account.as_deref()
    }

    /// Get the BIC code
    pub fn bic(&self) -> &str {
        &self.bic
    }
}

impl std::fmt::Display for Field50A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.account {
            Some(account) => write!(f, "Account: {}, BIC: {}", account, self.bic),
            None => write!(f, "BIC: {}", self.bic),
        }
    }
}

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field50K {
    /// Name and address lines (up to 4 lines)
    pub name_and_address: Vec<String>,
}

impl Field50K {
    /// Create a new Field50K with validation
    pub fn new(name_and_address: Vec<String>) -> Result<Self, ParseError> {
        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50K".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if name_and_address.len() > 4 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "50K".to_string(),
                message: "Too many name/address lines (max 4)".to_string(),
            });
        }

        for (i, line) in name_and_address.iter().enumerate() {
            if line.len() > 35 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "50K".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }
        }

        Ok(Field50K { name_and_address })
    }

    /// Get the name and address lines
    pub fn name_and_address(&self) -> &[String] {
        &self.name_and_address
    }
}

/// Field 50: Ordering Customer (with options A, F, and K)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field50 {
    A(Field50A),
    F(Field50F),
    K(Field50K),
}

impl Field50 {
    /// Parse Field50 with a specific tag (50A, 50F, or 50K)
    pub fn parse_with_tag(tag: &str, content: &str) -> Result<Self, ParseError> {
        match tag {
            "50A" => Ok(Field50::A(Field50A::parse(content)?)),
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
            Field50::K(field) => write!(f, "50K: {}", field.name_and_address.join(", ")),
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
                    // Field50A variant
                    let account = fields
                        .get("account")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let bic = fields
                        .get("bic")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| de::Error::missing_field("bic"))?
                        .to_string();

                    Field50A::new(account, bic)
                        .map(Field50::A)
                        .map_err(|e| de::Error::custom(format!("Field50A validation error: {}", e)))
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

impl SwiftField for Field50A {
    fn parse(content: &str) -> Result<Self, ParseError> {
        // Handle input that includes field tag prefix
        let content = if content.starts_with(":50A:") {
            &content[5..]
        } else if content.starts_with("50A:") {
            &content[4..]
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

        Field50A::new(account, bic_line)
    }

    fn to_swift_string(&self) -> String {
        let content = if let Some(ref account) = self.account {
            format!("/{}\n{}", account, self.bic)
        } else {
            self.bic.clone()
        };
        format!(":50A:{}", content)
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        // Validate BIC length
        if self.bic.len() != 8 && self.bic.len() != 11 {
            errors.push(ValidationError::FormatValidation {
                field_tag: "50A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        // Validate account length if present
        if let Some(ref account) = self.account {
            if account.len() > 34 {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "50A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: vec![],
        }
    }

    fn format_spec() -> &'static str {
        "[/account]bic"
    }
}

impl SwiftField for Field50F {
    fn parse(content: &str) -> Result<Self, ParseError> {
        // Handle input that includes field tag prefix
        let content = if content.starts_with(":50F:") {
            &content[5..]
        } else if content.starts_with("50F:") {
            &content[4..]
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
        // Handle input that includes field tag prefix
        let content = if content.starts_with(":50K:") {
            &content[5..]
        } else if content.starts_with("50K:") {
            &content[4..]
        } else {
            content
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        Field50K::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":50K:{}", self.name_and_address.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        if self.name_and_address.is_empty() {
            errors.push(ValidationError::FormatValidation {
                field_tag: "50K".to_string(),
                message: "Name and address cannot be empty".to_string(),
            });
        }

        if self.name_and_address.len() > 4 {
            errors.push(ValidationError::FormatValidation {
                field_tag: "50K".to_string(),
                message: "Too many lines (max 4)".to_string(),
            });
        }

        for (i, line) in self.name_and_address.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "50K".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: vec![],
        }
    }

    fn format_spec() -> &'static str {
        "4*35x"
    }
}

impl SwiftField for Field50 {
    fn parse(input: &str) -> Result<Self, ParseError> {
        // Try to determine the variant from the input
        if input.starts_with(":50A:") || input.starts_with("50A:") {
            Ok(Field50::A(Field50A::parse(input)?))
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
            Field50::A(field) => field.to_swift_string(),
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
        let field = Field50A::new(Some("123456789".to_string()), "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), Some("123456789"));
        assert_eq!(field.bic(), "DEUTDEFFXXX");
        assert_eq!(field.to_swift_string(), ":50A:/123456789\nDEUTDEFFXXX");
    }

    #[test]
    fn test_field50a_without_account() {
        let field = Field50A::new(None, "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), None);
        assert_eq!(field.bic(), "DEUTDEFFXXX");
        assert_eq!(field.to_swift_string(), ":50A:DEUTDEFFXXX");
    }

    #[test]
    fn test_field50a_parse() {
        let field = Field50A::parse("/123456789\nDEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), Some("123456789"));
        assert_eq!(field.bic(), "DEUTDEFFXXX");

        let field = Field50A::parse("DEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), None);
        assert_eq!(field.bic(), "DEUTDEFFXXX");
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
        let field = Field50::A(Field50A::new(None, "DEUTDEFFXXX").unwrap());
        assert_eq!(field.tag(), "50A");

        let field = Field50::F(Field50F::new("PARTY123", vec!["JOHN DOE".to_string()]).unwrap());
        assert_eq!(field.tag(), "50F");

        let field = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        assert_eq!(field.tag(), "50K");
    }

    #[test]
    fn test_field50_validation() {
        let field = Field50A::new(None, "DEUTDEFF").unwrap();
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
        let field50a =
            Field50::A(Field50A::new(Some("123456789".to_string()), "DEUTDEFFXXX").unwrap());

        let json = serde_json::to_string(&field50a).unwrap();
        println!("Field50A JSON: {}", json);

        // Should be flattened - no "A" wrapper
        assert!(json.contains("\"account\""));
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
        let json = r#"{"account":"123456789","bic":"DEUTDEFFXXX"}"#;
        let field: Field50 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field50::A(_)));

        // Test deserializing Field50F
        let json = r#"{"party_identifier":"PARTY123","name_and_address":["JOHN DOE"]}"#;
        let field: Field50 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field50::F(_)));
    }
}
