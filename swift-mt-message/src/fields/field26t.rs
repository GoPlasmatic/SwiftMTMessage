use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 26T: Transaction Type Code
///
/// Format: 3!c (3 alphanumeric characters)
///
/// Code indicating the transaction type according to EUROSTAT Balance of Payments guidelines.
/// This field is used to categorize the nature of the transaction for statistical purposes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field26T {
    /// Transaction type code (3 alphanumeric characters)
    pub transaction_type_code: String,
}

impl SwiftField for Field26T {
    fn parse(value: &str) -> crate::Result<Self> {
        let value = value.trim();

        if value.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Handle input that includes field tag prefix (e.g., ":26T:A01")
        let content = if value.starts_with(":26T:") {
            &value[5..] // Remove ":26T:" prefix
        } else if value.starts_with("26T:") {
            &value[4..] // Remove "26T:" prefix
        } else {
            value // Use as-is if no prefix
        };

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Field content cannot be empty after removing tag".to_string(),
            });
        }

        Self::new(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":26T:{}", self.transaction_type_code)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate length (3 characters)
        if self.transaction_type_code.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "26T".to_string(),
                expected: "3 characters".to_string(),
                actual: self.transaction_type_code.len(),
            });
        }

        // Validate not empty
        if self.transaction_type_code.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "26T".to_string(),
                message: "Transaction type code cannot be empty".to_string(),
            });
        }

        // Validate characters (alphanumeric)
        if !self
            .transaction_type_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "26T".to_string(),
                message: "Transaction type code must contain only alphanumeric characters"
                    .to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "3!c"
    }
}

impl Field26T {
    /// Create a new Field26T with validation
    pub fn new(transaction_type_code: impl Into<String>) -> crate::Result<Self> {
        let code = transaction_type_code.into().trim().to_uppercase();

        if code.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Transaction type code cannot be empty".to_string(),
            });
        }

        if code.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Transaction type code must be exactly 3 characters".to_string(),
            });
        }

        // Validate characters (alphanumeric)
        if !code.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Transaction type code must contain only alphanumeric characters"
                    .to_string(),
            });
        }

        Ok(Field26T {
            transaction_type_code: code,
        })
    }

    /// Get the transaction type code
    pub fn code(&self) -> &str {
        &self.transaction_type_code
    }

    /// Check if this is a valid EUROSTAT BoP code format
    pub fn is_valid_format(&self) -> bool {
        self.transaction_type_code.len() == 3
            && self
                .transaction_type_code
                .chars()
                .all(|c| c.is_alphanumeric() && c.is_ascii())
    }

    /// Get human-readable description based on common EUROSTAT BoP codes
    pub fn description(&self) -> &'static str {
        match self.transaction_type_code.as_str() {
            // Common EUROSTAT Balance of Payments codes
            "A01" => "Goods - General merchandise on a gross basis",
            "A02" => "Goods - Goods for processing",
            "A03" => "Goods - Repairs on goods",
            "B01" => "Services - Manufacturing services on physical inputs owned by others",
            "B02" => "Services - Maintenance and repair services n.i.e.",
            "B03" => "Services - Transport",
            "C01" => "Primary income - Compensation of employees",
            "C02" => "Primary income - Investment income",
            "D01" => "Secondary income - General government",
            "D02" => "Secondary income - Other sectors",
            "E01" => "Capital account - Capital transfers",
            "E02" => "Capital account - Acquisition/disposal of non-produced, non-financial assets",
            "F01" => "Financial account - Direct investment",
            "F02" => "Financial account - Portfolio investment",
            "F03" => "Financial account - Financial derivatives",
            "F04" => "Financial account - Other investment",
            "F05" => "Financial account - Reserve assets",
            _ => "Transaction type code (refer to EUROSTAT BoP guidelines)",
        }
    }
}

impl std::fmt::Display for Field26T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.transaction_type_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field26t_creation() {
        let field = Field26T::new("A01").unwrap();
        assert_eq!(field.transaction_type_code, "A01");
        assert_eq!(field.code(), "A01");
    }

    #[test]
    fn test_field26t_parse() {
        let field = Field26T::parse("B02").unwrap();
        assert_eq!(field.transaction_type_code, "B02");
    }

    #[test]
    fn test_field26t_parse_with_prefix() {
        let field = Field26T::parse(":26T:C01").unwrap();
        assert_eq!(field.transaction_type_code, "C01");

        let field = Field26T::parse("26T:D02").unwrap();
        assert_eq!(field.transaction_type_code, "D02");
    }

    #[test]
    fn test_field26t_case_normalization() {
        let field = Field26T::new("c03").unwrap();
        assert_eq!(field.transaction_type_code, "C03");
    }

    #[test]
    fn test_field26t_invalid_length() {
        let result = Field26T::new("AB"); // Too short
        assert!(result.is_err());

        let result = Field26T::new("ABCD"); // Too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_invalid_characters() {
        let result = Field26T::new("A@1"); // Invalid character @
        assert!(result.is_err());

        let result = Field26T::new("A-1"); // Invalid character -
        assert!(result.is_err());

        let result = Field26T::new("A.1"); // Invalid character .
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_empty() {
        let result = Field26T::new("");
        assert!(result.is_err());

        let result = Field26T::new("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_to_swift_string() {
        let field = Field26T::new("D04").unwrap();
        assert_eq!(field.to_swift_string(), ":26T:D04");
    }

    #[test]
    fn test_field26t_validation() {
        let field = Field26T::new("E05").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let invalid_field = Field26T {
            transaction_type_code: "INVALID".to_string(),
        };
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field26t_format_spec() {
        assert_eq!(Field26T::format_spec(), "3!c");
    }

    #[test]
    fn test_field26t_display() {
        let field = Field26T::new("F06").unwrap();
        assert_eq!(format!("{}", field), "F06");
    }

    #[test]
    fn test_field26t_is_valid_format() {
        let field = Field26T::new("A01").unwrap();
        assert!(field.is_valid_format());

        let invalid_field = Field26T {
            transaction_type_code: "INVALID".to_string(),
        };
        assert!(!invalid_field.is_valid_format());
    }

    #[test]
    fn test_field26t_descriptions() {
        let field = Field26T::new("A01").unwrap();
        assert_eq!(
            field.description(),
            "Goods - General merchandise on a gross basis"
        );

        let field = Field26T::new("B03").unwrap();
        assert_eq!(field.description(), "Services - Transport");

        let field = Field26T::new("F01").unwrap();
        assert_eq!(field.description(), "Financial account - Direct investment");

        let field = Field26T::new("XYZ").unwrap();
        assert_eq!(
            field.description(),
            "Transaction type code (refer to EUROSTAT BoP guidelines)"
        );
    }

    #[test]
    fn test_field26t_common_codes() {
        // Test some common EUROSTAT BoP codes
        let codes = ["A01", "A02", "B01", "C01", "D01", "E01", "F01"];

        for code in codes {
            let field = Field26T::new(code).unwrap();
            assert_eq!(field.code(), code);
            assert!(field.is_valid_format());
            assert!(!field.description().is_empty());
        }
    }
}
