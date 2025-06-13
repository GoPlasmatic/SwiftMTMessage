use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Valid instruction codes for Field 23E
pub const VALID_INSTRUCTION_CODES: &[&str] = &[
    "CHQB", "HOLD", "INTC", "PHOB", "PHOI", "PHON", "REPA", "SDVA", "TELB", "TELE", "TELI",
];

/// Field 23E: Instruction Code
///
/// Format: 4!c[/30x] (4 alphanumeric characters optionally followed by /30x)
///
/// Code specifying how the transaction should be processed.
/// Common values: CHQB, HOLD, INTC, PHOB, PHOI, PHON, REPA, SDVA, TELB, TELE, TELI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field23E {
    /// Instruction code (4 alphanumeric characters)
    pub instruction_code: String,

    /// Additional information (optional, up to 30 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl SwiftField for Field23E {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":23E:") {
            stripped // Remove ":23E:" prefix
        } else if let Some(stripped) = value.strip_prefix("23E:") {
            stripped // Remove "23E:" prefix
        } else {
            value
        };

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Field content cannot be empty after removing tag".to_string(),
            });
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

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate instruction code length
        if self.instruction_code.len() != 4 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "23E".to_string(),
                expected: "4 characters".to_string(),
                actual: self.instruction_code.len(),
            });
        }

        // Validate instruction code characters (alphanumeric)
        if !self
            .instruction_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "23E".to_string(),
                message: "Instruction code must contain only alphanumeric characters".to_string(),
            });
        }

        // Validate against known instruction codes
        if !VALID_INSTRUCTION_CODES.contains(&self.instruction_code.as_str()) {
            errors.push(ValidationError::ValueValidation {
                field_tag: "23E".to_string(),
                message: format!("Invalid instruction code: {}", self.instruction_code),
            });
        }

        // Validate additional info if present
        if let Some(ref info) = self.additional_info {
            if info.len() > 30 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "23E".to_string(),
                    expected: "max 30 characters".to_string(),
                    actual: info.len(),
                });
            }

            if info.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "23E".to_string(),
                    message: "Additional information cannot be empty if specified".to_string(),
                });
            }

            // Validate characters (printable ASCII)
            if !info.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "23E".to_string(),
                    message: "Additional information contains invalid characters".to_string(),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "4!c[/30x]"
    }
}

impl Field23E {
    /// Create a new Field23E with validation
    pub fn new(
        instruction_code: impl Into<String>,
        additional_info: Option<String>,
    ) -> crate::Result<Self> {
        let code = instruction_code.into().trim().to_uppercase();

        if code.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Instruction code cannot be empty".to_string(),
            });
        }

        if code.len() != 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Instruction code must be exactly 4 characters".to_string(),
            });
        }

        // Validate characters (alphanumeric)
        if !code.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Instruction code must contain only alphanumeric characters".to_string(),
            });
        }

        // Validate against known instruction codes
        if !VALID_INSTRUCTION_CODES.contains(&code.as_str()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: format!("Invalid instruction code: {}", code),
            });
        }

        // Validate additional info if present
        if let Some(ref info) = additional_info {
            if info.len() > 30 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Additional information too long (max 30 characters)".to_string(),
                });
            }

            if info.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Additional information cannot be empty if specified".to_string(),
                });
            }

            // Validate characters (printable ASCII)
            if !info.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Additional information contains invalid characters".to_string(),
                });
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

    /// Check if this is a valid instruction code
    pub fn is_valid_code(&self) -> bool {
        VALID_INSTRUCTION_CODES.contains(&self.instruction_code.as_str())
    }

    /// Get human-readable description of the instruction code
    pub fn description(&self) -> &'static str {
        match self.instruction_code.as_str() {
            "CHQB" => "Pay by cheque/banker's draft",
            "HOLD" => "Hold payment until further notice",
            "INTC" => "Intracompany payment",
            "PHOB" => "Phone ordering customer before payment",
            "PHOI" => "Phone intermediary bank before payment",
            "PHON" => "Phone all parties before payment",
            "REPA" => "Reimbursement payment",
            "SDVA" => "Same day value",
            "TELB" => "Telex beneficiary before payment",
            "TELE" => "Telex all parties before payment",
            "TELI" => "Telex intermediary bank before payment",
            _ => "Unknown instruction code",
        }
    }

    /// Validate against Field 23B business rules
    pub fn validate_with_field_23b(&self, field_23b_code: &str) -> crate::Result<()> {
        match field_23b_code {
            "SPRI" => {
                // If 23B = SPRI, 23E can only contain SDVA, TELB, PHOB, INTC
                if !["SDVA", "TELB", "PHOB", "INTC"].contains(&self.instruction_code.as_str()) {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "23E".to_string(),
                        message: format!(
                            "When Field 23B is SPRI, Field 23E can only be SDVA, TELB, PHOB, or INTC. Got: {}",
                            self.instruction_code
                        ),
                    });
                }
            }
            "SSTD" | "SPAY" => {
                // If 23B = SSTD/SPAY, 23E must not be used
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Field 23E must not be present when Field 23B is SSTD or SPAY"
                        .to_string(),
                });
            }
            _ => {
                // For other 23B values, 23E can contain any valid instruction code
            }
        }

        Ok(())
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

    #[test]
    fn test_field23e_creation_simple() {
        let field = Field23E::new("CHQB", None).unwrap();
        assert_eq!(field.instruction_code, "CHQB");
        assert_eq!(field.additional_info, None);
        assert_eq!(field.code(), "CHQB");
    }

    #[test]
    fn test_field23e_creation_with_info() {
        let field = Field23E::new("HOLD", Some("COMPLIANCE CHECK".to_string())).unwrap();
        assert_eq!(field.instruction_code, "HOLD");
        assert_eq!(field.additional_info, Some("COMPLIANCE CHECK".to_string()));
        assert_eq!(field.additional_info(), Some("COMPLIANCE CHECK"));
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
    fn test_field23e_parse_with_tag_prefix() {
        let field = Field23E::parse(":23E:HOLD/INFO").unwrap();
        assert_eq!(field.instruction_code, "HOLD");
        assert_eq!(field.additional_info, Some("INFO".to_string()));

        let field = Field23E::parse("23E:SDVA").unwrap();
        assert_eq!(field.instruction_code, "SDVA");
        assert_eq!(field.additional_info, None);
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
        let result = field.validate();
        assert!(result.is_valid);

        let invalid_field = Field23E {
            instruction_code: "INVALID".to_string(),
            additional_info: None,
        };
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field23e_format_spec() {
        assert_eq!(Field23E::format_spec(), "4!c[/30x]");
    }

    #[test]
    fn test_field23e_display() {
        let field = Field23E::new("PHOI", None).unwrap();
        assert_eq!(format!("{}", field), "PHOI");

        let field = Field23E::new("SDVA", Some("SAME DAY".to_string())).unwrap();
        assert_eq!(format!("{}", field), "SDVA/SAME DAY");
    }

    #[test]
    fn test_field23e_descriptions() {
        let field = Field23E::new("CHQB", None).unwrap();
        assert_eq!(field.description(), "Pay by cheque/banker's draft");

        let field = Field23E::new("HOLD", None).unwrap();
        assert_eq!(field.description(), "Hold payment until further notice");

        let field = Field23E::new("SDVA", None).unwrap();
        assert_eq!(field.description(), "Same day value");
    }

    #[test]
    fn test_field23e_is_valid_code() {
        let field = Field23E::new("CHQB", None).unwrap();
        assert!(field.is_valid_code());

        let field = Field23E {
            instruction_code: "XXXX".to_string(),
            additional_info: None,
        };
        assert!(!field.is_valid_code());
    }
}
