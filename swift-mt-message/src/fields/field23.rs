use serde::{Deserialize, Serialize};
use std::fmt;

/// # Field 23: Further Identification
///
/// Used in MT935 (Rate Change Advice) to specify the type of rate change.
///
/// ## Format
/// `3!a[2!n]11x` (3 letter function code, optional 2-digit days, 11 character reference)
///
/// ## Valid Function Codes
/// - `BASE`: Base rate
/// - `CALL`: Call rate
/// - `COMM`: Commercial rate
/// - `CURR`: Current rate
/// - `DEPO`: Deposit rate
/// - `NOTI`: Notice rate (requires days field)
/// - `PRIM`: Prime rate
///
/// ## Example Usage
/// ```rust
/// # use swift_mt_message::fields::Field23;
/// let field = Field23::new("BASE", None, "USD123456").unwrap();
/// assert_eq!(field.to_swift_string(), "BASEUSD123456");
///
/// let notice = Field23::new("NOTI", Some(7), "NOTICE7").unwrap();
/// assert_eq!(notice.to_swift_string(), "NOTI07NOTICE7");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Field23 {
    /// Function code (BASE, CALL, COMM, CURR, DEPO, NOTI, PRIM)
    pub function_code: String,
    /// Number of days (only used with NOTI function)
    pub days: Option<u8>,
    /// Reference information (up to 11 characters)
    pub reference: String,
}

impl Field23 {
    /// Valid function codes for Field23
    const VALID_FUNCTIONS: &'static [&'static str] =
        &["BASE", "CALL", "COMM", "CURR", "DEPO", "NOTI", "PRIM"];

    /// Creates a new Field23 with validation
    ///
    /// # Arguments
    /// * `function_code` - The function code (must be one of the valid codes)
    /// * `days` - Optional number of days (only for NOTI function)
    /// * `reference` - Reference information (up to 11 characters)
    ///
    /// # Returns
    /// * `Ok(Field23)` if all components are valid
    /// * `Err(String)` if validation fails
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field23;
    /// let field = Field23::new("BASE", None, "USD123456").unwrap();
    /// assert_eq!(field.function_code, "BASE");
    ///
    /// let notice = Field23::new("NOTI", Some(7), "NOTICE7").unwrap();
    /// assert_eq!(notice.days, Some(7));
    /// ```
    pub fn new(function_code: &str, days: Option<u8>, reference: &str) -> Result<Self, String> {
        let normalized_function = function_code.trim().to_uppercase();

        // Validate function code
        if !Self::VALID_FUNCTIONS.contains(&normalized_function.as_str()) {
            return Err(format!(
                "Invalid function code '{}'. Must be one of: {:?}",
                normalized_function,
                Self::VALID_FUNCTIONS
            ));
        }

        // Validate days field usage
        match normalized_function.as_str() {
            "NOTI" => {
                if days.is_none() {
                    return Err("NOTI function code requires days field".to_string());
                }
                if let Some(d) = days {
                    if d > 99 {
                        return Err("Days field cannot exceed 99".to_string());
                    }
                }
            }
            _ => {
                if days.is_some() {
                    return Err(format!(
                        "Days field can only be used with NOTI function, not {}",
                        normalized_function
                    ));
                }
            }
        }

        // Validate reference
        let trimmed_reference = reference.trim();
        if trimmed_reference.is_empty() {
            return Err("Reference cannot be empty".to_string());
        }
        if trimmed_reference.len() > 11 {
            return Err("Reference cannot exceed 11 characters".to_string());
        }

        // Validate reference contains only allowed characters (alphanumeric and basic symbols)
        if !trimmed_reference
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "/-?:().,'+ ".contains(c))
        {
            return Err("Reference contains invalid characters".to_string());
        }

        Ok(Field23 {
            function_code: normalized_function,
            days,
            reference: trimmed_reference.to_string(),
        })
    }

    /// Parses Field23 from a SWIFT message string
    ///
    /// # Arguments
    /// * `input` - The input string to parse
    ///
    /// # Returns
    /// * `Ok(Field23)` if parsing succeeds
    /// * `Err(String)` if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field23;
    /// let field = Field23::parse("BASEUSD123456").unwrap();
    /// assert_eq!(field.function_code, "BASE");
    ///
    /// let field = Field23::parse("NOTI07NOTICE7").unwrap();
    /// assert_eq!(field.days, Some(7));
    /// ```
    pub fn parse(input: &str) -> Result<Self, String> {
        let cleaned = input
            .trim()
            .strip_prefix(":23:")
            .or_else(|| input.strip_prefix("23:"))
            .unwrap_or(input);

        if cleaned.len() < 4 {
            return Err("Field23 must be at least 4 characters long".to_string());
        }

        // Extract function code (first 4 characters)
        let function_code = &cleaned[0..4];

        // Check if this is a NOTI function with days
        if function_code == "NOTI" && cleaned.len() >= 6 {
            let potential_days = &cleaned[4..6];
            if potential_days.chars().all(|c| c.is_ascii_digit()) {
                let days: u8 = potential_days.parse().map_err(|_| "Invalid days format")?;
                let reference = if cleaned.len() > 6 { &cleaned[6..] } else { "" };
                return Self::new(function_code, Some(days), reference);
            }
        }

        // Handle other function codes or NOTI without explicit days
        let reference = if cleaned.len() > 4 { &cleaned[4..] } else { "" };

        Self::new(function_code, None, reference)
    }

    /// Converts the field to its SWIFT string representation
    ///
    /// # Returns
    /// The field formatted for SWIFT messages
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field23;
    /// let field = Field23::new("BASE", None, "USD123456").unwrap();
    /// assert_eq!(field.to_swift_string(), "BASEUSD123456");
    ///
    /// let notice = Field23::new("NOTI", Some(7), "NOTICE7").unwrap();
    /// assert_eq!(notice.to_swift_string(), "NOTI07NOTICE7");
    /// ```
    pub fn to_swift_string(&self) -> String {
        match self.days {
            Some(days) => format!("{}{:02}{}", self.function_code, days, self.reference),
            None => format!("{}{}", self.function_code, self.reference),
        }
    }

    /// Returns the SWIFT field format specification
    ///
    /// # Returns
    /// The format specification string
    pub fn format_spec() -> &'static str {
        "3!a[2!n]11x"
    }

    /// Checks if this is a notice rate (requires days field)
    ///
    /// # Returns
    /// `true` if function code is NOTI, `false` otherwise
    pub fn is_notice_rate(&self) -> bool {
        self.function_code == "NOTI"
    }

    /// Gets a description of the function code
    ///
    /// # Returns
    /// A human-readable description of the function code
    pub fn get_function_description(&self) -> &'static str {
        match self.function_code.as_str() {
            "BASE" => "Base rate",
            "CALL" => "Call rate",
            "COMM" => "Commercial rate",
            "CURR" => "Current rate",
            "DEPO" => "Deposit rate",
            "NOTI" => "Notice rate",
            "PRIM" => "Prime rate",
            _ => "Unknown function",
        }
    }

    /// Validates the field according to SWIFT standards
    ///
    /// # Returns
    /// `true` if the field is valid, `false` otherwise
    pub fn is_valid(&self) -> bool {
        // Check function code
        if !Self::VALID_FUNCTIONS.contains(&self.function_code.as_str()) {
            return false;
        }

        // Check days field consistency
        let days_valid = match self.function_code.as_str() {
            "NOTI" => self.days.is_some() && self.days.unwrap() <= 99,
            _ => self.days.is_none(),
        };

        // Check reference
        let reference_valid = !self.reference.is_empty()
            && self.reference.len() <= 11
            && self
                .reference
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "/-?:().,'+ ".contains(c));

        days_valid && reference_valid
    }
}

impl fmt::Display for Field23 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.days {
            Some(days) => write!(
                f,
                "Field23: {} ({} days) - {}",
                self.get_function_description(),
                days,
                self.reference
            ),
            None => write!(
                f,
                "Field23: {} - {}",
                self.get_function_description(),
                self.reference
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field23_creation_valid() {
        let field = Field23::new("BASE", None, "USD123456").unwrap();
        assert_eq!(field.function_code, "BASE");
        assert_eq!(field.days, None);
        assert_eq!(field.reference, "USD123456");
        assert!(field.is_valid());
    }

    #[test]
    fn test_field23_creation_notice_with_days() {
        let field = Field23::new("NOTI", Some(7), "NOTICE7").unwrap();
        assert_eq!(field.function_code, "NOTI");
        assert_eq!(field.days, Some(7));
        assert_eq!(field.reference, "NOTICE7");
        assert!(field.is_notice_rate());
        assert!(field.is_valid());
    }

    #[test]
    fn test_field23_invalid_function_code() {
        let result = Field23::new("INVL", None, "TEST");
        assert!(result.is_err());
    }

    #[test]
    fn test_field23_notice_without_days() {
        let result = Field23::new("NOTI", None, "NOTICE");
        assert!(result.is_err());
    }

    #[test]
    fn test_field23_non_notice_with_days() {
        let result = Field23::new("BASE", Some(7), "TEST");
        assert!(result.is_err());
    }

    #[test]
    fn test_field23_invalid_reference() {
        let result = Field23::new("BASE", None, ""); // Empty reference
        assert!(result.is_err());

        let result = Field23::new("BASE", None, "THISISTOOLONG12"); // Too long (>11 chars)
        assert!(result.is_err());
    }

    #[test]
    fn test_field23_parse() {
        let field = Field23::parse("BASEUSD123456").unwrap();
        assert_eq!(field.function_code, "BASE");
        assert_eq!(field.reference, "USD123456");

        let field = Field23::parse("NOTI07NOTICE7").unwrap();
        assert_eq!(field.function_code, "NOTI");
        assert_eq!(field.days, Some(7));
        assert_eq!(field.reference, "NOTICE7");

        let field = Field23::parse(":23:PRIMGBP12345").unwrap();
        assert_eq!(field.function_code, "PRIM");
        assert_eq!(field.reference, "GBP12345");
    }

    #[test]
    fn test_field23_to_swift_string() {
        let field = Field23::new("BASE", None, "USD123456").unwrap();
        assert_eq!(field.to_swift_string(), "BASEUSD123456");

        let field = Field23::new("NOTI", Some(7), "NOTICE7").unwrap();
        assert_eq!(field.to_swift_string(), "NOTI07NOTICE7");
    }

    #[test]
    fn test_field23_format_spec() {
        assert_eq!(Field23::format_spec(), "3!a[2!n]11x");
    }

    #[test]
    fn test_field23_function_descriptions() {
        let functions = [
            ("BASE", "Base rate"),
            ("CALL", "Call rate"),
            ("COMM", "Commercial rate"),
            ("CURR", "Current rate"),
            ("DEPO", "Deposit rate"),
            ("NOTI", "Notice rate"),
            ("PRIM", "Prime rate"),
        ];

        for (code, desc) in &functions {
            let field = if *code == "NOTI" {
                Field23::new(code, Some(7), "TEST").unwrap()
            } else {
                Field23::new(code, None, "TEST").unwrap()
            };
            assert_eq!(field.get_function_description(), *desc);
        }
    }

    #[test]
    fn test_field23_display() {
        let field = Field23::new("BASE", None, "USD123456").unwrap();
        let display = format!("{}", field);
        assert!(display.contains("Base rate"));
        assert!(display.contains("USD123456"));

        let field = Field23::new("NOTI", Some(7), "NOTICE").unwrap();
        let display = format!("{}", field);
        assert!(display.contains("Notice rate"));
        assert!(display.contains("7 days"));
        assert!(display.contains("NOTICE"));
    }

    #[test]
    fn test_field23_serialization() {
        let field = Field23::new("PRIM", None, "GBP123456").unwrap();
        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: Field23 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field, deserialized);
    }
}
