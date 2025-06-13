use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 53A: Sender's Correspondent
///
/// Format: [/1!c][/34x]BIC
///
/// This field specifies the sender's correspondent institution.
/// The BIC format is: 4 letters (bank code) + 2 letters (country) + 2 alphanumeric (location) + optional 3 alphanumeric (branch)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field53A {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

impl Field53A {
    /// Create a new Field53A with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let bic = bic.into().to_uppercase();

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Validate BIC
        Self::validate_bic(&bic)?;

        Ok(Field53A {
            account_line_indicator,
            account_number,
            bic: bic.to_string(),
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

    /// Check if this is a full BIC (11 characters) or short BIC (8 characters)
    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }

    /// Get the bank code (first 4 characters of BIC)
    pub fn bank_code(&self) -> &str {
        &self.bic[0..4]
    }

    /// Get the country code (characters 5-6 of BIC)
    pub fn country_code(&self) -> &str {
        &self.bic[4..6]
    }

    /// Get the location code (characters 7-8 of BIC)
    pub fn location_code(&self) -> &str {
        &self.bic[6..8]
    }

    /// Get the branch code (characters 9-11 of BIC, if present)
    pub fn branch_code(&self) -> Option<&str> {
        if self.bic.len() == 11 {
            Some(&self.bic[8..11])
        } else {
            None
        }
    }

    /// Validate BIC according to SWIFT standards
    fn validate_bic(bic: &str) -> Result<(), crate::ParseError> {
        if bic.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53A".to_string(),
                message: "BIC cannot be empty".to_string(),
            });
        }

        // BIC must be 8 or 11 characters
        if bic.len() != 8 && bic.len() != 11 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        // Validate BIC structure: 4!a2!a2!c[3!c]
        let bank_code = &bic[0..4];
        let country_code = &bic[4..6];
        let location_code = &bic[6..8];

        if !bank_code.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53A".to_string(),
                message: "BIC bank code (first 4 characters) must be alphabetic".to_string(),
            });
        }

        if !country_code
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53A".to_string(),
                message: "BIC country code (characters 5-6) must be alphabetic".to_string(),
            });
        }

        if !location_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53A".to_string(),
                message: "BIC location code (characters 7-8) must be alphanumeric".to_string(),
            });
        }

        // If 11 characters, validate branch code
        if bic.len() == 11 {
            let branch_code = &bic[8..11];
            if !branch_code
                .chars()
                .all(|c| c.is_alphanumeric() && c.is_ascii())
            {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "BIC branch code (characters 9-11) must be alphanumeric".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match &self.account_number {
            Some(account) => format!("Sender's Correspondent: {} ({})", self.bic, account),
            None => format!("Sender's Correspondent: {}", self.bic),
        }
    }
}

impl SwiftField for Field53A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        // Handle input that includes field tag prefix (e.g., ":53A:/1234567890\nDEUTDEFF")
        let content = if value.starts_with(":53A:") {
            &value[5..] // Remove ":53A:" prefix
        } else if value.starts_with("53A:") {
            &value[4..] // Remove "53A:" prefix
        } else {
            value // Use as-is if no prefix
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Parse account and BIC
        let mut account_number = None;
        let bic;

        if content.starts_with('/') {
            // Has account number
            let lines: Vec<&str> = content.lines().collect();

            if lines.len() == 1 {
                // Account and BIC on same line: "/account BIC" or "/account\nBIC"
                let parts: Vec<&str> = lines[0].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    account_number = Some(parts[0][1..].to_string()); // Remove leading '/'
                    bic = parts[1].to_string();
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "53A".to_string(),
                        message: "Invalid format: expected account and BIC".to_string(),
                    });
                }
            } else if lines.len() == 2 {
                // Account and BIC on separate lines
                account_number = Some(lines[0][1..].to_string()); // Remove leading '/'
                bic = lines[1].to_string();
            } else {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Invalid format: too many lines".to_string(),
                });
            }
        } else {
            // No account number, just BIC
            bic = content.to_string();
        }

        Self::new(None, account_number, bic)
    }

    fn to_swift_string(&self) -> String {
        match &self.account_number {
            Some(account) => format!(":53A:/{}\n{}", account, self.bic),
            None => format!(":53A:{}", self.bic),
        }
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate account line indicator if present
        if let Some(ref indicator) = self.account_line_indicator {
            if indicator.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "53A".to_string(),
                    expected: "exactly 1 character".to_string(),
                    actual: indicator.len(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = self.account_number {
            if account.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "53A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "53A".to_string(),
                    expected: "max 34 characters".to_string(),
                    actual: account.len(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "53A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Validate BIC
        if let Err(e) = Self::validate_bic(&self.bic) {
            if let crate::ParseError::InvalidFieldFormat { message, .. } = e {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "53A".to_string(),
                    message,
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
        "[/1!c][/34x]BIC"
    }
}

impl std::fmt::Display for Field53A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.account_line_indicator, &self.account_number) {
            (Some(indicator), Some(account)) => write!(f, "/{}{} {}", indicator, account, self.bic),
            (None, Some(account)) => write!(f, "/{} {}", account, self.bic),
            _ => write!(f, "{}", self.bic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field53a_creation_bic_only() {
        let field = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert!(field.account_number().is_none());
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field53a_creation_with_account() {
        let field = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field53a_parse_bic_only() {
        let field = Field53A::parse("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field53a_parse_with_account_same_line() {
        let field = Field53A::parse("/1234567890 DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field53a_parse_with_account_separate_lines() {
        let field = Field53A::parse("/1234567890\nDEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field53a_parse_with_prefix() {
        let field = Field53A::parse(":53A:CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field53a_to_swift_string_bic_only() {
        let field = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.to_swift_string(), ":53A:DEUTDEFF");
    }

    #[test]
    fn test_field53a_to_swift_string_with_account() {
        let field = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.to_swift_string(), ":53A:/1234567890\nDEUTDEFF500");
    }

    #[test]
    fn test_field53a_bic_components() {
        let field = Field53A::new(None, None, "DEUTDEFF500").unwrap();
        assert_eq!(field.bank_code(), "DEUT");
        assert_eq!(field.country_code(), "DE");
        assert_eq!(field.location_code(), "FF");
        assert_eq!(field.branch_code(), Some("500"));
    }

    #[test]
    fn test_field53a_short_bic_components() {
        let field = Field53A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(field.bank_code(), "CHAS");
        assert_eq!(field.country_code(), "US");
        assert_eq!(field.location_code(), "33");
        assert_eq!(field.branch_code(), None);
    }

    #[test]
    fn test_field53a_invalid_bic_length() {
        let result = Field53A::new(None, None, "DEUT");
        assert!(result.is_err());

        let result = Field53A::new(None, None, "DEUTDEFF5001");
        assert!(result.is_err());
    }

    #[test]
    fn test_field53a_invalid_bic_format() {
        let result = Field53A::new(None, None, "123TDEFF");
        assert!(result.is_err());

        let result = Field53A::new(None, None, "DEUT12FF");
        assert!(result.is_err());

        let result = Field53A::new(None, None, "DEUTDE@F");
        assert!(result.is_err());
    }

    #[test]
    fn test_field53a_invalid_account() {
        let result = Field53A::new(None, Some("".to_string()), "DEUTDEFF");
        assert!(result.is_err());

        let result = Field53A::new(None, Some("A".repeat(35)), "DEUTDEFF");
        assert!(result.is_err());
    }

    #[test]
    fn test_field53a_validation() {
        let field = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field53a_display() {
        let field1 = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field1), "DEUTDEFF");

        let field2 = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field2), "/1234567890 DEUTDEFF");
    }

    #[test]
    fn test_field53a_description() {
        let field1 = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field1.description(), "Sender's Correspondent: DEUTDEFF");

        let field2 = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(
            field2.description(),
            "Sender's Correspondent: DEUTDEFF (1234567890)"
        );
    }

    #[test]
    fn test_field53a_case_normalization() {
        let field = Field53A::new(None, None, "deutdeff").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
    }
}
