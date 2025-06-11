//! Field 50F: Ordering Customer (Option F)
//!
//! Format: Party identifier and name/address lines

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 50F: Ordering Customer (Option F)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field50F {
    /// Party identifier
    pub party_identifier: String,
    /// Name and address lines
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field50F {
    const TAG: &'static str = "50F";

    fn parse(content: &str) -> Result<Self> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(FieldParseError::missing_data("50F", "No content provided").into());
        }

        Ok(Field50F {
            party_identifier: lines[0].to_string(),
            name_and_address: lines[1..].iter().map(|s| s.to_string()).collect(),
        })
    }

    fn to_swift_string(&self) -> String {
        let mut content = self.party_identifier.clone();
        for line in &self.name_and_address {
            content.push('\n');
            content.push_str(line);
        }
        format!(":50F:{}", content)
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Ordering Customer (Option F)"
    }
}
