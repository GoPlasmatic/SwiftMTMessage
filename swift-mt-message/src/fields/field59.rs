use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Field 59A: Beneficiary Customer (Option A)
///
/// Format: [/account]\nBIC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field59A {
    /// Account number (optional)
    pub account: Option<String>,
    /// BIC (Bank Identifier Code)
    pub bic: String,
}

impl Field59A {
    /// Create a new Field59A with validation
    pub fn new(account: Option<String>, bic: impl Into<String>) -> Result<Self, ParseError> {
        let bic = bic.into().trim().to_string();

        if bic.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59A".to_string(),
                message: "BIC cannot be empty".to_string(),
            });
        }

        // Basic BIC validation (8 or 11 characters)
        if bic.len() != 8 && bic.len() != 11 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        // Validate account if present
        if let Some(ref acc) = account {
            if acc.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59A".to_string(),
                    message: "Account cannot be empty if specified".to_string(),
                });
            }

            if acc.len() > 34 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }
        }

        Ok(Field59A { account, bic })
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

impl std::fmt::Display for Field59A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.account {
            Some(account) => write!(f, "Account: {}, BIC: {}", account, self.bic),
            None => write!(f, "BIC: {}", self.bic),
        }
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

impl Field59Basic {
    /// Create a new Field59Basic with validation
    pub fn new(beneficiary_customer: Vec<String>) -> Result<Self, ParseError> {
        if beneficiary_customer.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59".to_string(),
                message: "Beneficiary customer information cannot be empty".to_string(),
            });
        }

        if beneficiary_customer.len() > 4 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "59".to_string(),
                message: "Too many lines (max 4)".to_string(),
            });
        }

        for (i, line) in beneficiary_customer.iter().enumerate() {
            if line.len() > 35 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "59".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }
        }

        Ok(Field59Basic {
            beneficiary_customer,
        })
    }

    /// Get the beneficiary customer lines
    pub fn beneficiary_customer(&self) -> &[String] {
        &self.beneficiary_customer
    }
}

/// Field 59: Beneficiary Customer (with options A and no letter option)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field59 {
    A(Field59A),
    NoOption(Field59Basic),
}

impl Field59 {
    /// Parse Field59 with a specific tag (59A or 59)
    pub fn parse_with_tag(tag: &str, content: &str) -> Result<Self, ParseError> {
        match tag {
            "59A" => Ok(Field59::A(Field59A::parse(content)?)),
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
            Field59::NoOption(_) => "59",
        }
    }
}

impl std::fmt::Display for Field59 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field59::A(field) => write!(f, "59A: {}", field),
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
                        .get("account")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let bic = fields
                        .get("bic")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| de::Error::missing_field("bic"))?
                        .to_string();

                    Field59A::new(account, bic)
                        .map(Field59::A)
                        .map_err(|e| de::Error::custom(format!("Field59A validation error: {}", e)))
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

impl SwiftField for Field59A {
    fn parse(content: &str) -> Result<Self, ParseError> {
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

        Field59A::new(account, bic_line)
    }

    fn to_swift_string(&self) -> String {
        match &self.account {
            Some(account) => format!(":59A:/{}\n{}", account, self.bic),
            None => format!(":59A:{}", self.bic),
        }
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        // Validate BIC length
        if self.bic.len() != 8 && self.bic.len() != 11 {
            errors.push(ValidationError::FormatValidation {
                field_tag: "59A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        // Validate account length if present
        if let Some(ref account) = self.account {
            if account.len() > 34 {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "59A".to_string(),
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

impl SwiftField for Field59Basic {
    fn parse(content: &str) -> Result<Self, ParseError> {
        let content = if let Some(stripped) = content.strip_prefix(":59:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("59:") {
            stripped
        } else {
            content
        };

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        Field59Basic::new(lines)
    }

    fn to_swift_string(&self) -> String {
        format!(":59:{}", self.beneficiary_customer.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        use crate::errors::ValidationError;

        let mut errors = Vec::new();

        if self.beneficiary_customer.is_empty() {
            errors.push(ValidationError::FormatValidation {
                field_tag: "59".to_string(),
                message: "Beneficiary customer information cannot be empty".to_string(),
            });
        }

        if self.beneficiary_customer.len() > 4 {
            errors.push(ValidationError::FormatValidation {
                field_tag: "59".to_string(),
                message: "Too many lines (max 4)".to_string(),
            });
        }

        for (i, line) in self.beneficiary_customer.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "59".to_string(),
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

impl SwiftField for Field59 {
    fn parse(input: &str) -> Result<Self, ParseError> {
        // Try to determine the variant from the input
        if input.starts_with(":59A:") || input.starts_with("59A:") {
            Ok(Field59::A(Field59A::parse(input)?))
        } else if input.starts_with(":59:") || input.starts_with("59:") {
            Ok(Field59::NoOption(Field59Basic::parse(input)?))
        } else {
            // Default to NoOption if no clear indicator
            Ok(Field59::NoOption(Field59Basic::parse(input)?))
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field59::A(field) => field.to_swift_string(),
            Field59::NoOption(field) => field.to_swift_string(),
        }
    }

    fn validate(&self) -> ValidationResult {
        match self {
            Field59::A(field) => field.validate(),
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
        let field = Field59A::new(Some("123456789".to_string()), "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), Some("123456789"));
        assert_eq!(field.bic(), "DEUTDEFFXXX");
        assert_eq!(field.to_swift_string(), ":59A:/123456789\nDEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_without_account() {
        let field = Field59A::new(None, "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), None);
        assert_eq!(field.bic(), "DEUTDEFFXXX");
        assert_eq!(field.to_swift_string(), ":59A:DEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_parse() {
        let field = Field59A::parse("/123456789\nDEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), Some("123456789"));
        assert_eq!(field.bic(), "DEUTDEFFXXX");

        let field = Field59A::parse("DEUTDEFFXXX").unwrap();
        assert_eq!(field.account(), None);
        assert_eq!(field.bic(), "DEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_invalid_bic() {
        let result = Field59A::new(None, "INVALID"); // Too short
        assert!(result.is_err());

        let result = Field59A::new(None, "TOOLONGBICCODE"); // Too long
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
        let field = Field59::A(Field59A::new(None, "DEUTDEFFXXX").unwrap());
        assert_eq!(field.tag(), "59A");

        let field = Field59::NoOption(Field59Basic::new(vec!["MUELLER GMBH".to_string()]).unwrap());
        assert_eq!(field.tag(), "59");
    }

    #[test]
    fn test_field59_validation() {
        let field = Field59A::new(None, "DEUTDEFF").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let field = Field59Basic::new(vec!["MUELLER GMBH".to_string()]).unwrap();
        let result = field.validate();
        assert!(result.is_valid);
    }

    #[test]
    fn test_field59_display() {
        let field = Field59A::new(Some("123456".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field), "Account: 123456, BIC: DEUTDEFF");

        let field = Field59A::new(None, "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field), "BIC: DEUTDEFF");

        let field =
            Field59Basic::new(vec!["MUELLER GMBH".to_string(), "BERLIN".to_string()]).unwrap();
        let enum_field = Field59::NoOption(field);
        assert_eq!(format!("{}", enum_field), "59: MUELLER GMBH, BERLIN");
    }

    #[test]
    fn test_field59_json_serialization_flattened() {
        // Test Field59A
        let field59a = Field59::A(
            Field59A::new(Some("DE89370400440532013000".to_string()), "DEUTDEFF").unwrap(),
        );

        let json = serde_json::to_string(&field59a).unwrap();
        println!("Field59A JSON: {}", json);

        // Should be flattened - no "A" wrapper
        assert!(json.contains("\"account\""));
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
        let json = r#"{"account":"DE89370400440532013000","bic":"DEUTDEFF"}"#;
        let field: Field59 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field59::A(_)));

        // Test deserializing Field59Basic
        let json = r#"{"beneficiary_customer":["MUELLER GMBH","HAUPTSTRASSE 1"]}"#;
        let field: Field59 = serde_json::from_str(json).unwrap();
        assert!(matches!(field, Field59::NoOption(_)));
    }
}
