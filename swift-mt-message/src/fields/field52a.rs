use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 52A: Ordering Institution
///
/// Format: [/1!a][/34x]4!a2!a2!c[3!c] (optional account line indicator + account number + BIC)
///
/// This field identifies the ordering institution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field52A {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

impl Field52A {
    /// Create a new Field52A with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let bic = bic.into().to_uppercase();

        // Validate BIC
        if bic.len() != 8 && bic.len() != 11 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        if !bic.chars().all(|c| c.is_alphanumeric()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52A".to_string(),
                message: "BIC must contain only alphanumeric characters".to_string(),
            });
        }

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account line indicator cannot be empty".to_string(),
                });
            }
            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }
            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account number cannot be empty".to_string(),
                });
            }
            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }
            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        Ok(Field52A {
            account_line_indicator,
            account_number,
            bic,
        })
    }

    /// Get the account line indicator
    pub fn account_line_indicator(&self) -> Option<&str> {
        self.account_line_indicator.as_deref()
    }

    /// Get the account number
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }

    /// Get the BIC code
    pub fn bic(&self) -> &str {
        &self.bic
    }

    /// Check if this is a full BIC (11 characters)
    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }
}

impl SwiftField for Field52A {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Handle input that includes field tag prefix (e.g., ":52A:BNPAFRPPXXX")
        let content = if content.starts_with(":52A:") {
            &content[5..] // Remove ":52A:" prefix
        } else if content.starts_with("52A:") {
            &content[4..] // Remove "52A:" prefix
        } else {
            content // Use as-is if no prefix
        }
        .trim();

        let mut account_line_indicator = None;
        let mut account_number = None;
        let mut bic_content = content;

        // Check for account line indicator (starts with /)
        if content.starts_with('/') {
            let lines: Vec<&str> = content.lines().collect();
            if !lines.is_empty() {
                let first_line = lines[0];

                if first_line.len() == 2 && first_line.starts_with('/') {
                    // Only account line indicator: /X
                    account_line_indicator = Some(first_line[1..].to_string());
                    bic_content = if lines.len() > 1 { lines[1] } else { "" };
                } else if first_line.len() > 2 && first_line.starts_with('/') {
                    // Account line indicator + account number: /X/account or /account
                    let parts: Vec<&str> = first_line[1..].split('/').collect();
                    if parts.len() == 2 {
                        // /X/account format
                        account_line_indicator = Some(parts[0].to_string());
                        account_number = Some(parts[1].to_string());
                    } else {
                        // /account format
                        account_number = Some(parts[0].to_string());
                    }
                    bic_content = if lines.len() > 1 { lines[1] } else { "" };
                }
            }
        }

        let bic = bic_content.trim().to_string();
        if bic.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52A".to_string(),
                message: "BIC code is required".to_string(),
            });
        }

        Field52A::new(account_line_indicator, account_number, bic)
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();

        if let Some(ref indicator) = self.account_line_indicator {
            result.push('/');
            result.push_str(indicator);
        }

        if let Some(ref account) = self.account_number {
            result.push('/');
            result.push_str(account);
        }

        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&self.bic);

        format!(":52A:{}", result)
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
        "[/1!a][/34x]4!a2!a2!c[3!c]"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field52a_creation_bic_only() {
        let field = Field52A::new(None, None, "BNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert!(field.account_number().is_none());
        assert!(field.account_line_indicator().is_none());
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field52a_creation_with_account() {
        let field = Field52A::new(None, Some("1234567890".to_string()), "BNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field52a_creation_with_account_line_indicator() {
        let field = Field52A::new(
            Some("A".to_string()),
            Some("1234567890".to_string()),
            "BNPAFRPPXXX",
        )
        .unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.account_line_indicator(), Some("A"));
    }

    #[test]
    fn test_field52a_parse_bic_only() {
        let field = Field52A::parse("BNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field52a_parse_with_account() {
        let field = Field52A::parse("/1234567890\nBNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field52a_to_swift_string() {
        let field = Field52A::new(
            Some("A".to_string()),
            Some("1234567890".to_string()),
            "BNPAFRPPXXX",
        )
        .unwrap();
        assert_eq!(field.to_swift_string(), ":52A:/A/1234567890\nBNPAFRPPXXX");
    }
}
