use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 54A: Receiver's Correspondent
///
/// Format: [/34x]4!a2!a2!c[3!c] (optional account + BIC)
///
/// This field specifies the receiver's correspondent institution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field54A {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

impl Field54A {
    /// Create a new Field54A with validation
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
                    field_tag: "54A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Validate BIC
        Self::validate_bic(&bic)?;

        Ok(Field54A {
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

    /// Validate BIC according to SWIFT standards
    fn validate_bic(bic: &str) -> Result<(), crate::ParseError> {
        if bic.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC cannot be empty".to_string(),
            });
        }

        if bic.len() != 8 && bic.len() != 11 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        let bank_code = &bic[0..4];
        let country_code = &bic[4..6];
        let location_code = &bic[6..8];

        if !bank_code.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC bank code (first 4 characters) must be alphabetic".to_string(),
            });
        }

        if !country_code
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC country code (characters 5-6) must be alphabetic".to_string(),
            });
        }

        if !location_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC location code (characters 7-8) must be alphanumeric".to_string(),
            });
        }

        if bic.len() == 11 {
            let branch_code = &bic[8..11];
            if !branch_code
                .chars()
                .all(|c| c.is_alphanumeric() && c.is_ascii())
            {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "BIC branch code (characters 9-11) must be alphanumeric".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match &self.account_number {
            Some(account) => format!("Receiver's Correspondent: {} ({})", self.bic, account),
            None => format!("Receiver's Correspondent: {}", self.bic),
        }
    }
}

impl SwiftField for Field54A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if value.starts_with(":54A:") {
            &value[5..]
        } else if value.starts_with("54A:") {
            &value[4..]
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let mut account_line_indicator = None;
        let mut account_number = None;
        let bic;

        if content.starts_with('/') {
            let lines: Vec<&str> = content.lines().collect();

            if lines.len() == 1 {
                let parts: Vec<&str> = lines[0].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    account_line_indicator = Some(parts[0][1..].to_string());
                    account_number = Some(parts[1].to_string());
                    bic = parts[1].to_string();
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "54A".to_string(),
                        message: "Invalid format: expected account and BIC".to_string(),
                    });
                }
            } else if lines.len() == 2 {
                account_line_indicator = Some(lines[0][1..].to_string());
                account_number = Some(lines[1].to_string());
                bic = lines[1].to_string();
            } else {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Invalid format: too many lines".to_string(),
                });
            }
        } else {
            bic = content.to_string();
        }

        Self::new(account_line_indicator, account_number, bic)
    }

    fn to_swift_string(&self) -> String {
        match &self.account_number {
            Some(account) => format!(":54A:/{}\n{}", account, self.bic),
            None => format!(":54A:{}", self.bic),
        }
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(ref account) = self.account_number {
            if account.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "54A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "54A".to_string(),
                    expected: "max 34 characters".to_string(),
                    actual: account.len(),
                });
            }
        }

        if let Err(e) = Self::validate_bic(&self.bic) {
            if let crate::ParseError::InvalidFieldFormat { message, .. } = e {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "54A".to_string(),
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
        "[/34x]4!a2!a2!c[3!c]"
    }
}

impl std::fmt::Display for Field54A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.account_number {
            Some(account) => write!(f, "/{} {}", account, self.bic),
            None => write!(f, "{}", self.bic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field54a_creation() {
        let field = Field54A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field54a_with_account() {
        let field = Field54A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field54a_parse() {
        let field = Field54A::parse("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field54a_to_swift_string() {
        let field = Field54A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.to_swift_string(), ":54A:DEUTDEFF");
    }

    #[test]
    fn test_field54a_validation() {
        let field = Field54A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
    }
}
