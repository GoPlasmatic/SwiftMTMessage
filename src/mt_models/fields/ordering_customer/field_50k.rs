//! Field 50K: Ordering Customer (Option K)
//!
//! Format: 4*35x (up to 4 lines of 35 characters each)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 50K: Ordering Customer (Option K)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field50K {
    /// Name and address lines (up to 4 lines)
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field50K {
    const TAG: &'static str = "50K";

    fn parse(content: &str) -> Result<Self> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        if lines.len() > 4 {
            return Err(FieldParseError::invalid_format("50K", "Too many lines (max 4)").into());
        }

        Ok(Field50K {
            name_and_address: lines,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":50K:{}", self.name_and_address.join("\n"))
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        if self.name_and_address.len() > 4 {
            return Err(ValidationError::field_validation_failed(
                "50K",
                "Too many lines (max 4)",
            ));
        }
        Ok(())
    }

    fn description() -> &'static str {
        "Ordering Customer (Option K)"
    }
}
