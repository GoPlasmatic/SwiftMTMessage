//! Field 71A: Details of Charges
//!
//! Format: 3!a (exactly 3 letters: BEN, OUR, or SHA)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 71A: Details of Charges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field71A {
    /// Details of charges (BEN, OUR, or SHA)
    pub details_of_charges: String,
}

impl SwiftField for Field71A {
    const TAG: &'static str = "71A";

    fn parse(content: &str) -> Result<Self> {
        let details = content.trim().to_string();

        if !matches!(details.as_str(), "BEN" | "OUR" | "SHA") {
            return Err(FieldParseError::invalid_format("71A", "Must be BEN, OUR, or SHA").into());
        }

        Ok(Field71A {
            details_of_charges: details,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":71A:{}", self.details_of_charges)
    }

    fn validate(&self, _rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        if !matches!(self.details_of_charges.as_str(), "BEN" | "OUR" | "SHA") {
            return Err(ValidationError::field_validation_failed(
                "71A",
                "Must be BEN, OUR, or SHA",
            ));
        }
        Ok(())
    }

    fn description() -> &'static str {
        "Details of Charges"
    }
}
