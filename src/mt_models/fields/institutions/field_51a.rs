//! Field 51A: Sending Institution
//!
//! Institution which sends the message.
//! Format: [/1!a][/34x] 4!a2!a2!c[3!c] (optional account line/account number + BIC)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 51A: Sending Institution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field51A {
    /// Account line indicator (optional, 1 character)
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

impl Field51A {
    /// Create a new Field51A with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> Result<Self> {
        let bic = bic.into().to_uppercase();

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.len() != 1 {
                return Err(FieldParseError::invalid_format(
                    "51A",
                    "Account line indicator must be exactly 1 character",
                )
                .into());
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "51A",
                    "Account line indicator contains invalid characters",
                )
                .into());
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(FieldParseError::invalid_format(
                    "51A",
                    "Account number cannot be empty if specified",
                )
                .into());
            }

            if account.len() > 34 {
                return Err(FieldParseError::invalid_format(
                    "51A",
                    "Account number too long (max 34 characters)",
                )
                .into());
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(FieldParseError::invalid_format(
                    "51A",
                    "Account number contains invalid characters",
                )
                .into());
            }
        }

        // Validate BIC
        if bic.len() != 8 && bic.len() != 11 {
            return Err(
                FieldParseError::invalid_format("51A", "BIC must be 8 or 11 characters").into(),
            );
        }

        if !bic.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(FieldParseError::invalid_format(
                "51A",
                "BIC must contain only alphanumeric characters",
            )
            .into());
        }

        // Validate BIC structure: 4!a2!a2!c[3!c]
        let bank_code = &bic[0..4];
        let country_code = &bic[4..6];
        let location_code = &bic[6..8];

        if !bank_code.chars().all(|c| c.is_alphabetic()) {
            return Err(FieldParseError::invalid_format(
                "51A",
                "BIC bank code (first 4 characters) must be alphabetic",
            )
            .into());
        }

        if !country_code.chars().all(|c| c.is_alphabetic()) {
            return Err(FieldParseError::invalid_format(
                "51A",
                "BIC country code (characters 5-6) must be alphabetic",
            )
            .into());
        }

        if !location_code.chars().all(|c| c.is_alphanumeric()) {
            return Err(FieldParseError::invalid_format(
                "51A",
                "BIC location code (characters 7-8) must be alphanumeric",
            )
            .into());
        }

        // If 11 characters, validate branch code
        if bic.len() == 11 {
            let branch_code = &bic[8..11];
            if !branch_code.chars().all(|c| c.is_alphanumeric()) {
                return Err(FieldParseError::invalid_format(
                    "51A",
                    "BIC branch code (characters 9-11) must be alphanumeric",
                )
                .into());
            }
        }

        Ok(Field51A {
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

    /// Check if this is a full BIC (11 characters) or short BIC (8 characters)
    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }
}

impl SwiftField for Field51A {
    const TAG: &'static str = "51A";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();

        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("51A", "Field content cannot be empty").into(),
            );
        }

        let mut account_line_indicator = None;
        let mut account_number = None;
        let mut remaining = content;

        // Parse optional account line indicator (starts with /)
        if content.starts_with('/') {
            let slash_pos = content.find('/').unwrap();
            let next_slash = content[slash_pos + 1..].find('/');

            if let Some(next_pos) = next_slash {
                // Two slashes: /indicator/account
                let indicator = &content[slash_pos + 1..slash_pos + 1 + next_pos];
                let account_start = slash_pos + 1 + next_pos + 1;

                // Find where the account ends and BIC begins
                let lines: Vec<&str> = content[account_start..].lines().collect();
                if lines.len() >= 2 {
                    let account = lines[0];
                    let bic_line = lines[1];

                    if !indicator.is_empty() {
                        account_line_indicator = Some(indicator.to_string());
                    }
                    if !account.is_empty() {
                        account_number = Some(account.to_string());
                    }
                    remaining = bic_line;
                } else {
                    return Err(FieldParseError::invalid_format(
                        "51A",
                        "Missing BIC after account information",
                    )
                    .into());
                }
            } else {
                // Single slash: could be /indicator or /account
                let lines: Vec<&str> = content.lines().collect();
                if lines.len() >= 2 {
                    let first_line = lines[0];
                    let bic_line = lines[1];

                    // Check if first line after / is just 1 character (indicator) or longer (account)
                    let after_slash = &first_line[1..];
                    if after_slash.len() == 1 {
                        account_line_indicator = Some(after_slash.to_string());
                    } else {
                        account_number = Some(after_slash.to_string());
                    }
                    remaining = bic_line;
                } else {
                    return Err(FieldParseError::invalid_format(
                        "51A",
                        "Missing BIC after account information",
                    )
                    .into());
                }
            }
        }

        // The remaining content should be the BIC
        let bic = remaining.trim();
        if bic.is_empty() {
            return Err(FieldParseError::missing_data("51A", "BIC code is required").into());
        }

        Self::new(account_line_indicator, account_number, bic)
    }

    fn to_swift_string(&self) -> String {
        let mut content = String::new();

        if let Some(ref indicator) = self.account_line_indicator {
            content.push('/');
            content.push_str(indicator);
        }

        if let Some(ref account) = self.account_number {
            content.push('/');
            content.push_str(account);
        }

        if !content.is_empty() {
            content.push('\n');
        }

        content.push_str(&self.bic);

        format!(":51A:{}", content)
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        let mut content = String::new();

        if let Some(ref indicator) = self.account_line_indicator {
            content.push('/');
            content.push_str(indicator);
        }

        if let Some(ref account) = self.account_number {
            content.push('/');
            content.push_str(account);
        }

        if !content.is_empty() {
            content.push('\n');
        }

        content.push_str(&self.bic);

        rules.validate_field("51A", &content)
    }

    fn description() -> &'static str {
        "Sending Institution - Institution which sends the message"
    }
}

impl std::fmt::Display for Field51A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();

        if let Some(ref indicator) = self.account_line_indicator {
            parts.push(format!("/{}", indicator));
        }

        if let Some(ref account) = self.account_number {
            parts.push(format!("/{}", account));
        }

        parts.push(self.bic.clone());

        write!(f, "{}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field51a_bic_only() {
        let field = Field51A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(field.account_line_indicator, None);
        assert_eq!(field.account_number, None);
        assert_eq!(field.bic, "CHASUS33");
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field51a_with_account() {
        let field = Field51A::new(None, Some("123456789".to_string()), "CHASUS33XXX").unwrap();
        assert_eq!(field.account_line_indicator, None);
        assert_eq!(field.account_number, Some("123456789".to_string()));
        assert_eq!(field.bic, "CHASUS33XXX");
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field51a_with_indicator_and_account() {
        let field = Field51A::new(
            Some("A".to_string()),
            Some("987654321".to_string()),
            "DEUTDEFF",
        )
        .unwrap();
        assert_eq!(field.account_line_indicator, Some("A".to_string()));
        assert_eq!(field.account_number, Some("987654321".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");
    }

    #[test]
    fn test_field51a_parse_bic_only() {
        let field = Field51A::parse("CHASUS33").unwrap();
        assert_eq!(field.bic, "CHASUS33");
        assert_eq!(field.account_line_indicator, None);
        assert_eq!(field.account_number, None);
    }

    #[test]
    fn test_field51a_invalid_bic() {
        let result = Field51A::new(None, None, "SHORT"); // Too short
        assert!(result.is_err());

        let result = Field51A::new(None, None, "TOOLONGBIC123"); // Too long
        assert!(result.is_err());

        let result = Field51A::new(None, None, "123ABCDE"); // Bank code not alphabetic
        assert!(result.is_err());

        let result = Field51A::new(None, None, "ABCD12FF"); // Country code not alphabetic
        assert!(result.is_err());
    }

    #[test]
    fn test_field51a_invalid_account_line_indicator() {
        let result = Field51A::new(Some("AB".to_string()), None, "CHASUS33"); // Too long
        assert!(result.is_err());

        let result = Field51A::new(Some("".to_string()), None, "CHASUS33"); // Empty
        assert!(result.is_err());
    }

    #[test]
    fn test_field51a_invalid_account_number() {
        let result = Field51A::new(None, Some("A".repeat(35)), "CHASUS33"); // Too long
        assert!(result.is_err());

        let result = Field51A::new(None, Some("".to_string()), "CHASUS33"); // Empty
        assert!(result.is_err());
    }

    #[test]
    fn test_field51a_to_swift_string() {
        let field = Field51A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(field.to_swift_string(), ":51A:CHASUS33");

        let field = Field51A::new(None, Some("123456".to_string()), "CHASUS33").unwrap();
        assert_eq!(field.to_swift_string(), ":51A:/123456\nCHASUS33");
    }

    #[test]
    fn test_field51a_validation() {
        let field = Field51A::new(None, None, "CHASUS33").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field51a_display() {
        let field = Field51A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(format!("{}", field), "CHASUS33");

        let field = Field51A::new(
            Some("A".to_string()),
            Some("123456".to_string()),
            "CHASUS33",
        )
        .unwrap();
        assert_eq!(format!("{}", field), "/A /123456 CHASUS33");
    }

    #[test]
    fn test_field51a_accessors() {
        let field = Field51A::new(
            Some("B".to_string()),
            Some("ACCOUNT123".to_string()),
            "DEUTDEFFXXX",
        )
        .unwrap();
        assert_eq!(field.account_line_indicator(), Some("B"));
        assert_eq!(field.account_number(), Some("ACCOUNT123"));
        assert_eq!(field.bic(), "DEUTDEFFXXX");
        assert!(field.is_full_bic());
    }
}
