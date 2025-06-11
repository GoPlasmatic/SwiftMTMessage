//! Field 59A: Beneficiary Customer (Option A)
//!
//! Beneficiary customer with account and BIC.
//! Format: [/account]\nBIC

use crate::errors::{Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 59A: Beneficiary Customer (Option A)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field59A {
    /// Account number (optional)
    pub account: Option<String>,
    /// BIC (Bank Identifier Code)
    pub bic: String,
}

impl Field59A {
    /// Create a new Field59A with validation
    pub fn new(account: Option<String>, bic: impl Into<String>) -> Result<Self> {
        let bic = bic.into().trim().to_string();

        if bic.is_empty() {
            return Err(
                crate::errors::FieldParseError::missing_data("59A", "BIC cannot be empty").into(),
            );
        }

        // Basic BIC validation (8 or 11 characters)
        if bic.len() != 8 && bic.len() != 11 {
            return Err(crate::errors::FieldParseError::invalid_format(
                "59A",
                "BIC must be 8 or 11 characters",
            )
            .into());
        }

        // Validate account if present
        if let Some(ref acc) = account {
            if acc.is_empty() {
                return Err(crate::errors::FieldParseError::invalid_format(
                    "59A",
                    "Account cannot be empty if specified",
                )
                .into());
            }

            if acc.len() > 34 {
                return Err(crate::errors::FieldParseError::invalid_format(
                    "59A",
                    "Account number too long (max 34 characters)",
                )
                .into());
            }
        }

        Ok(Field59A { account, bic })
    }
}

impl SwiftField for Field59A {
    const TAG: &'static str = "59A";

    fn parse(content: &str) -> Result<Self> {
        let mut lines = content.lines();

        let first_line = lines.next().unwrap_or_default();

        let (account, bic_line) = if let Some(stripped) = first_line.strip_prefix('/') {
            (Some(stripped.to_string()), lines.next().unwrap_or_default())
        } else {
            (None, first_line)
        };

        Self::new(account, bic_line)
    }

    fn to_swift_string(&self) -> String {
        match &self.account {
            Some(account) => format!(":59A:/{}\n{}", account, self.bic),
            None => format!(":59A:{}", self.bic),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        let content = match &self.account {
            Some(account) => format!("/{}\n{}", account, self.bic),
            None => self.bic.clone(),
        };
        rules.validate_field("59A", &content)
    }

    fn description() -> &'static str {
        "Beneficiary Customer (Option A) - Account and BIC of the beneficiary"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field59a_with_account() {
        let field = Field59A::new(Some("123456789".to_string()), "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account, Some("123456789".to_string()));
        assert_eq!(field.bic, "DEUTDEFFXXX");
        assert_eq!(field.to_swift_string(), ":59A:/123456789\nDEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_without_account() {
        let field = Field59A::new(None, "DEUTDEFFXXX").unwrap();
        assert_eq!(field.account, None);
        assert_eq!(field.bic, "DEUTDEFFXXX");
        assert_eq!(field.to_swift_string(), ":59A:DEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_parse_with_account() {
        let field = Field59A::parse("/123456789\nDEUTDEFFXXX").unwrap();
        assert_eq!(field.account, Some("123456789".to_string()));
        assert_eq!(field.bic, "DEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_parse_without_account() {
        let field = Field59A::parse("DEUTDEFFXXX").unwrap();
        assert_eq!(field.account, None);
        assert_eq!(field.bic, "DEUTDEFFXXX");
    }

    #[test]
    fn test_field59a_invalid_bic() {
        let result = Field59A::new(None, "INVALID"); // Too short
        assert!(result.is_err());

        let result = Field59A::new(None, "TOOLONGBICCODE"); // Too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field59a_validation() {
        let field = Field59A::new(Some("12345".to_string()), "DEUTDEFF").unwrap();
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field59a_display() {
        let field = Field59A::new(Some("123456".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field), "Account: 123456, BIC: DEUTDEFF");

        let field = Field59A::new(None, "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field), "BIC: DEUTDEFF");
    }
}
