//! Ordering Customer fields (Field 50 with options A, F, K)

pub mod field_50a;
pub mod field_50f;
pub mod field_50k;

// Re-export field types
pub use field_50a::Field50A;
pub use field_50f::Field50F;
pub use field_50k::Field50K;

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 50: Ordering Customer (with options A, F, K)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Field50 {
    A(Field50A),
    F(Field50F),
    K(Field50K),
}

impl Field50 {
    pub fn parse(tag: &str, content: &str) -> Result<Self> {
        match tag {
            "50A" => Ok(Field50::A(Field50A::parse(content)?)),
            "50F" => Ok(Field50::F(Field50F::parse(content)?)),
            "50K" => Ok(Field50::K(Field50K::parse(content)?)),
            _ => Err(
                FieldParseError::InvalidUsage(format!("Unknown Field50 option: {}", tag)).into(),
            ),
        }
    }

    pub fn tag(&self) -> &'static str {
        match self {
            Field50::A(_) => "50A",
            Field50::F(_) => "50F",
            Field50::K(_) => "50K",
        }
    }
}

impl std::fmt::Display for Field50 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field50::A(field) => write!(f, "50A: {}", field),
            Field50::F(field) => write!(f, "50F: {}", field.party_identifier),
            Field50::K(field) => write!(f, "50K: {}", field.name_and_address.join(", ")),
        }
    }
}

impl SwiftField for Field50 {
    const TAG: &'static str = "50";

    fn parse(_content: &str) -> Result<Self> {
        // This shouldn't be called directly; use Field50::parse(tag, content) instead
        Err(
            FieldParseError::InvalidUsage("Use Field50::parse(tag, content) instead".to_string())
                .into(),
        )
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50::A(field) => field.to_swift_string(),
            Field50::F(field) => field.to_swift_string(),
            Field50::K(field) => field.to_swift_string(),
        }
    }

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        match self {
            Field50::A(field) => field.validate(rules),
            Field50::F(field) => field.validate(rules),
            Field50::K(field) => field.validate(rules),
        }
    }

    fn options() -> Vec<&'static str> {
        vec!["A", "F", "K"]
    }

    fn description() -> &'static str {
        "Ordering Customer"
    }
}
