//! Field 50A: Ordering Customer (Option A)
//!
//! Format: [/account]\nBIC

use crate::errors::{Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 50A: Ordering Customer (Option A)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field50A {
    /// Optional account number (starting with /)
    pub account: Option<String>,
    /// BIC code
    pub bic: String,
}

impl std::fmt::Display for Field50A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.account {
            Some(account) => write!(f, "Account: {}, BIC: {}", account, self.bic),
            None => write!(f, "BIC: {}", self.bic),
        }
    }
}

impl SwiftField for Field50A {
    const TAG: &'static str = "50A";

    fn parse(content: &str) -> Result<Self> {
        let mut lines = content.lines();
        let first_line = lines.next().unwrap_or_default();

        let (account, bic_line) = if let Some(stripped) = first_line.strip_prefix('/') {
            (Some(stripped.to_string()), lines.next().unwrap_or_default())
        } else {
            (None, first_line)
        };

        Ok(Field50A {
            account,
            bic: bic_line.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        let content = if let Some(ref account) = self.account {
            format!("/{}\n{}", account, self.bic)
        } else {
            self.bic.clone()
        };
        format!(":50A:{}", content)
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        // TODO: Implement BIC validation
        Ok(())
    }

    fn description() -> &'static str {
        "Ordering Customer (Option A)"
    }
}
