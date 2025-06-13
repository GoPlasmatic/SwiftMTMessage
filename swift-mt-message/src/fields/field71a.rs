use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// Field 71A: Details of Charges
///
/// Format: 3!a (3 alphabetic characters)
///
/// This field specifies who bears the charges.
/// Common values: BEN, OUR, SHA
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("3!a")]
pub struct Field71A {
    /// Charge details code (3 characters)
    #[format("3!a")]
    pub details_of_charges: String,
}

impl Field71A {
    /// Create a new Field71A with the given charge code
    pub fn new(details_of_charges: String) -> Self {
        Self {
            details_of_charges: details_of_charges.to_uppercase(),
        }
    }

    /// Get the charge code
    pub fn charge_code(&self) -> &str {
        &self.details_of_charges
    }

    /// Check if this is a standard charge code
    pub fn is_standard_code(&self) -> bool {
        matches!(self.details_of_charges.as_str(), "BEN" | "OUR" | "SHA")
    }

    /// Get human-readable description of the charge code
    pub fn description(&self) -> &'static str {
        match self.details_of_charges.as_str() {
            "BEN" => "Beneficiary bears all charges",
            "OUR" => "Ordering customer bears all charges",
            "SHA" => "Charges shared between ordering customer and beneficiary",
            _ => "Unknown charge code",
        }
    }
}

impl std::fmt::Display for Field71A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details_of_charges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field71a_creation() {
        let field = Field71A::new("OUR".to_string());
        assert_eq!(field.charge_code(), "OUR");
        assert!(field.is_standard_code());
        assert_eq!(field.description(), "Ordering customer bears all charges");
    }

    #[test]
    fn test_field71a_parse() {
        let field = Field71A::parse("SHA").unwrap();
        assert_eq!(field.details_of_charges, "SHA");
    }

    #[test]
    fn test_field71a_case_insensitive() {
        let field = Field71A::new("ben".to_string());
        assert_eq!(field.details_of_charges, "BEN");
    }
}
