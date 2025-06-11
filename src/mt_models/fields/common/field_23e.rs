//! Field 23E: Instruction Code
//!
//! Code specifying how the transaction should be processed.
//! Format: 4!c[/30x] (4 alphanumeric characters optionally followed by /30x)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Valid instruction codes for Field 23E
pub const VALID_INSTRUCTION_CODES: &[&str] = &[
    "CHQB", "HOLD", "INTC", "PHOB", "PHOI", "PHON", "REPA", "SDVA", "TELB", "TELE", "TELI",
];

/// Field 23E: Instruction Code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field23E {
    /// Instruction code (4 alphanumeric characters)
    pub instruction_code: String,
    /// Additional information (optional, up to 30 characters)
    pub additional_info: Option<String>,
}

impl Field23E {
    /// Create a new Field23E with validation
    pub fn new(
        instruction_code: impl Into<String>,
        additional_info: Option<String>,
    ) -> Result<Self> {
        let code = instruction_code.into().trim().to_uppercase();

        if code.is_empty() {
            return Err(
                FieldParseError::missing_data("23E", "Instruction code cannot be empty").into(),
            );
        }

        if code.len() != 4 {
            return Err(FieldParseError::invalid_format(
                "23E",
                "Instruction code must be exactly 4 characters",
            )
            .into());
        }

        // Validate characters (alphanumeric)
        if !code.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                "23E",
                "Instruction code must contain only alphanumeric characters",
            )
            .into());
        }

        // Validate against known instruction codes
        if !VALID_INSTRUCTION_CODES.contains(&code.as_str()) {
            return Err(FieldParseError::invalid_format(
                "23E",
                &format!("Invalid instruction code: {}", code),
            )
            .into());
        }

        // Validate additional info if present
        if let Some(ref info) = additional_info {
            if info.len() > 30 {
                return Err(FieldParseError::invalid_format(
                    "23E",
                    "Additional information too long (max 30 characters)",
                )
                .into());
            }

            if info.is_empty() {
                return Err(FieldParseError::invalid_format(
                    "23E",
                    "Additional information cannot be empty if specified",
                )
                .into());
            }

            // Validate characters (printable ASCII)
            if !info.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "23E",
                    "Additional information contains invalid characters",
                )
                .into());
            }
        }

        Ok(Field23E {
            instruction_code: code,
            additional_info,
        })
    }

    /// Get the instruction code
    pub fn code(&self) -> &str {
        &self.instruction_code
    }

    /// Get the additional information
    pub fn additional_info(&self) -> Option<&str> {
        self.additional_info.as_deref()
    }

    /// Validate against Field 23B business rules
    pub fn validate_with_field_23b(&self, field_23b_code: &str) -> Result<()> {
        match field_23b_code {
            "SPRI" => {
                // If 23B = SPRI, 23E can only contain SDVA, TELB, PHOB, INTC
                if !["SDVA", "TELB", "PHOB", "INTC"].contains(&self.instruction_code.as_str()) {
                    return Err(FieldParseError::invalid_format(
                        "23E",
                        &format!(
                            "When Field 23B is SPRI, Field 23E can only be SDVA, TELB, PHOB, or INTC. Got: {}",
                            self.instruction_code
                        ),
                    )
                    .into());
                }
            }
            "SSTD" | "SPAY" => {
                // If 23B = SSTD/SPAY, 23E must not be used
                return Err(FieldParseError::invalid_format(
                    "23E",
                    "Field 23E must not be present when Field 23B is SSTD or SPAY",
                )
                .into());
            }
            _ => {
                // For other 23B values, 23E can contain any valid instruction code
            }
        }

        Ok(())
    }
}

impl SwiftField for Field23E {
    const TAG: &'static str = "23E";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();

        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("23E", "Field content cannot be empty").into(),
            );
        }

        // Check if there's additional info (indicated by /)
        if let Some(slash_pos) = content.find('/') {
            let instruction_code = &content[..slash_pos];
            let additional_info = &content[slash_pos + 1..];

            Self::new(instruction_code, Some(additional_info.to_string()))
        } else {
            Self::new(content, None)
        }
    }

    fn to_swift_string(&self) -> String {
        match &self.additional_info {
            Some(info) => format!(":23E:{}/{}", self.instruction_code, info),
            None => format!(":23E:{}", self.instruction_code),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        let content = match &self.additional_info {
            Some(info) => format!("{}/{}", self.instruction_code, info),
            None => self.instruction_code.clone(),
        };
        rules.validate_field("23E", &content)
    }

    fn description() -> &'static str {
        "Instruction Code - Code specifying how the transaction should be processed"
    }
}

impl std::fmt::Display for Field23E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.additional_info {
            Some(info) => write!(f, "{}/{}", self.instruction_code, info),
            None => write!(f, "{}", self.instruction_code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field23e_creation_simple() {
        let field = Field23E::new("CHQB", None).unwrap();
        assert_eq!(field.instruction_code, "CHQB");
        assert_eq!(field.additional_info, None);
    }

    #[test]
    fn test_field23e_creation_with_info() {
        let field = Field23E::new("HOLD", Some("COMPLIANCE CHECK".to_string())).unwrap();
        assert_eq!(field.instruction_code, "HOLD");
        assert_eq!(field.additional_info, Some("COMPLIANCE CHECK".to_string()));
    }

    #[test]
    fn test_field23e_parse_simple() {
        let field = Field23E::parse("INTC").unwrap();
        assert_eq!(field.instruction_code, "INTC");
        assert_eq!(field.additional_info, None);
    }

    #[test]
    fn test_field23e_parse_with_info() {
        let field = Field23E::parse("REPA/WEEKLY PAYMENT").unwrap();
        assert_eq!(field.instruction_code, "REPA");
        assert_eq!(field.additional_info, Some("WEEKLY PAYMENT".to_string()));
    }

    #[test]
    fn test_field23e_case_normalization() {
        let field = Field23E::new("phob", None).unwrap();
        assert_eq!(field.instruction_code, "PHOB");
    }

    #[test]
    fn test_field23e_invalid_code() {
        let result = Field23E::new("INVL", None); // Invalid code
        assert!(result.is_err());

        let result = Field23E::new("ABC", None); // Too short
        assert!(result.is_err());

        let result = Field23E::new("ABCDE", None); // Too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field23e_invalid_additional_info() {
        let result = Field23E::new("HOLD", Some("A".repeat(31))); // Too long
        assert!(result.is_err());

        let result = Field23E::new("HOLD", Some("".to_string())); // Empty
        assert!(result.is_err());
    }

    #[test]
    fn test_field23e_business_rules() {
        // SPRI with valid codes
        let field = Field23E::new("SDVA", None).unwrap();
        assert!(field.validate_with_field_23b("SPRI").is_ok());

        let field = Field23E::new("TELB", None).unwrap();
        assert!(field.validate_with_field_23b("SPRI").is_ok());

        // SPRI with invalid code
        let field = Field23E::new("CHQB", None).unwrap();
        assert!(field.validate_with_field_23b("SPRI").is_err());

        // SSTD/SPAY should not allow 23E
        let field = Field23E::new("HOLD", None).unwrap();
        assert!(field.validate_with_field_23b("SSTD").is_err());
        assert!(field.validate_with_field_23b("SPAY").is_err());

        // Other 23B values allow any valid code
        let field = Field23E::new("CHQB", None).unwrap();
        assert!(field.validate_with_field_23b("CRED").is_ok());
    }

    #[test]
    fn test_field23e_to_swift_string() {
        let field = Field23E::new("TELI", None).unwrap();
        assert_eq!(field.to_swift_string(), ":23E:TELI");

        let field = Field23E::new("PHON", Some("CALL BEFORE".to_string())).unwrap();
        assert_eq!(field.to_swift_string(), ":23E:PHON/CALL BEFORE");
    }

    #[test]
    fn test_field23e_validation() {
        let field = Field23E::new("TELE", None).unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field23e_display() {
        let field = Field23E::new("PHOI", None).unwrap();
        assert_eq!(format!("{}", field), "PHOI");

        let field = Field23E::new("SDVA", Some("SAME DAY".to_string())).unwrap();
        assert_eq!(format!("{}", field), "SDVA/SAME DAY");
    }

    #[test]
    fn test_field23e_accessors() {
        let field = Field23E::new("INTC", Some("INFO".to_string())).unwrap();
        assert_eq!(field.code(), "INTC");
        assert_eq!(field.additional_info(), Some("INFO"));
    }
}
