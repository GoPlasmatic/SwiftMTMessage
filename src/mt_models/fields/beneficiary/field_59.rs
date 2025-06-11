//! Field 59: Beneficiary Customer
//!
//! Format: 4*35x (up to 4 lines of 35 characters each)

use crate::errors::{Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 59: Beneficiary Customer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field59 {
    /// Beneficiary customer information (up to 4 lines)
    pub beneficiary_customer: Vec<String>,
}

impl SwiftField for Field59 {
    const TAG: &'static str = "59";

    fn parse(content: &str) -> Result<Self> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        Ok(Field59 {
            beneficiary_customer: lines,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":59:{}", self.beneficiary_customer.join("\n"))
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        Ok(())
    }

    fn description() -> &'static str {
        "Beneficiary Customer"
    }
}
