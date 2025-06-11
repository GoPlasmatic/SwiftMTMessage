//! Beneficiary customer field implementations

pub mod field_59;
pub mod field_59a;

pub use field_59::Field59 as Field59Basic;
pub use field_59a::Field59A;

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 59: Beneficiary Customer (with options A and no letter option)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Field59 {
    A(Field59A),
    NoOption(Field59Basic),
}

impl Field59 {
    pub fn parse(tag: &str, content: &str) -> Result<Self> {
        match tag {
            "59A" => Ok(Field59::A(Field59A::parse(content)?)),
            "59" => Ok(Field59::NoOption(Field59Basic::parse(content)?)),
            _ => Err(
                FieldParseError::InvalidUsage(format!("Unknown Field59 option: {}", tag)).into(),
            ),
        }
    }
}

impl std::fmt::Display for Field59 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field59::A(field) => write!(f, "59A: {}", field),
            Field59::NoOption(field) => {
                write!(f, "59: {}", field.beneficiary_customer.join(", "))
            }
        }
    }
}

impl SwiftField for Field59 {
    const TAG: &'static str = "59";

    fn parse(_: &str) -> Result<Self> {
        // This shouldn't be called directly; use Field59::parse(tag, content) instead
        Err(
            FieldParseError::InvalidUsage("Use Field59::parse(tag, content) instead".to_string())
                .into(),
        )
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field59::A(field) => field.to_swift_string(),
            Field59::NoOption(field) => field.to_swift_string(),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        match self {
            Field59::A(field) => field.validate(rules),
            Field59::NoOption(field) => field.validate(rules),
        }
    }

    fn options() -> Vec<&'static str> {
        vec!["A"]
    }

    fn description() -> &'static str {
        "Beneficiary Customer"
    }
}
