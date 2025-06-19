use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// # Field 12: Message Requested
///
/// Used in MT920 (Request Message) to specify which type of statement message is being requested.
///
/// ## Format
/// `3!n` (exactly 3 numeric digits)
///
/// ## Valid Values
/// - `940`: Customer Statement Message
/// - `941`: Balance Report  
/// - `942`: Interim Transaction Report
/// - `950`: Customer Statement Message (consolidated)
///
/// ## Example Usage
/// ```rust
/// # use swift_mt_message::fields::Field12;
/// let field = Field12::new("942").unwrap();
/// assert_eq!(field.to_swift_string(), "942");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Field12 {
    /// The requested message type (940, 941, 942, or 950)
    pub message_type: String,
}

impl Field12 {
    /// Creates a new Field12 with validation
    ///
    /// # Arguments
    /// * `message_type` - The message type to request (must be 940, 941, 942, or 950)
    ///
    /// # Returns
    /// * `Ok(Field12)` if the message type is valid
    /// * `Err(String)` if validation fails
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field12;
    /// let field = Field12::new("940").unwrap();
    /// assert_eq!(field.message_type, "940");
    ///
    /// let invalid = Field12::new("999");
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(message_type: &str) -> Result<Self, String> {
        let normalized = message_type.trim();

        // Validate format: exactly 3 digits
        if normalized.len() != 3 {
            return Err("Message type must be exactly 3 digits".to_string());
        }

        if !normalized.chars().all(|c| c.is_ascii_digit()) {
            return Err("Message type must contain only digits".to_string());
        }

        // Validate allowed values
        match normalized {
            "940" | "941" | "942" | "950" => Ok(Field12 {
                message_type: normalized.to_string(),
            }),
            _ => Err(format!(
                "Invalid message type '{}'. Must be 940, 941, 942, or 950",
                normalized
            )),
        }
    }

    /// Parses Field12 from a SWIFT message string
    ///
    /// # Arguments
    /// * `input` - The input string to parse
    ///
    /// # Returns
    /// * `Ok(Field12)` if parsing succeeds
    /// * `Err(String)` if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field12;
    /// let field = Field12::parse("942").unwrap();
    /// assert_eq!(field.message_type, "942");
    ///
    /// let field = Field12::parse(":12:941").unwrap();
    /// assert_eq!(field.message_type, "941");
    /// ```
    pub fn parse(input: &str) -> Result<Self, String> {
        let cleaned = input
            .trim()
            .strip_prefix(":12:")
            .or_else(|| input.strip_prefix("12:"))
            .unwrap_or(input);

        Self::new(cleaned)
    }

    /// Converts the field to its SWIFT string representation
    ///
    /// # Returns
    /// The field formatted for SWIFT messages
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field12;
    /// let field = Field12::new("940").unwrap();
    /// assert_eq!(field.to_swift_string(), "940");
    /// ```
    pub fn to_swift_string(&self) -> String {
        self.message_type.clone()
    }

    /// Returns the SWIFT field format specification
    ///
    /// # Returns
    /// The format specification string
    pub fn format_spec() -> &'static str {
        "3!n"
    }

    /// Checks if this is a request for a customer statement
    ///
    /// # Returns
    /// `true` if requesting MT940 or MT950, `false` otherwise
    pub fn is_customer_statement_request(&self) -> bool {
        matches!(self.message_type.as_str(), "940" | "950")
    }

    /// Checks if this is a request for a balance report
    ///
    /// # Returns
    /// `true` if requesting MT941, `false` otherwise
    pub fn is_balance_report_request(&self) -> bool {
        self.message_type == "941"
    }

    /// Checks if this is a request for an interim transaction report
    ///
    /// # Returns
    /// `true` if requesting MT942, `false` otherwise
    pub fn is_interim_report_request(&self) -> bool {
        self.message_type == "942"
    }

    /// Returns a description of the requested message type
    ///
    /// # Returns
    /// A human-readable description of the message type
    pub fn get_description(&self) -> &'static str {
        match self.message_type.as_str() {
            "940" => "Customer Statement Message",
            "941" => "Balance Report",
            "942" => "Interim Transaction Report",
            "950" => "Customer Statement Message (Consolidated)",
            _ => "Unknown Message Type",
        }
    }

    /// Validates the field according to SWIFT standards
    ///
    /// # Returns
    /// `true` if the field is valid, `false` otherwise
    pub fn is_valid(&self) -> bool {
        matches!(self.message_type.as_str(), "940" | "941" | "942" | "950")
            && self.message_type.len() == 3
            && self.message_type.chars().all(|c| c.is_ascii_digit())
    }
}

impl SwiftField for Field12 {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        Self::parse(value).map_err(|e| crate::ParseError::InvalidFieldFormat {
            field_tag: "12".to_string(),
            message: e,
        })
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_string()
    }

    fn validate(&self) -> ValidationResult {
        if self.is_valid() {
            ValidationResult::valid()
        } else {
            ValidationResult::with_error(ValidationError::FormatValidation {
                field_tag: "12".to_string(),
                message: format!("Invalid message type: {}", self.message_type),
            })
        }
    }

    fn format_spec() -> &'static str {
        "3!n"
    }
}

impl fmt::Display for Field12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Field12: {} ({})",
            self.message_type,
            self.get_description()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field12_creation_valid() {
        let valid_types = ["940", "941", "942", "950"];

        for msg_type in &valid_types {
            let field = Field12::new(msg_type).unwrap();
            assert_eq!(field.message_type, *msg_type);
            assert!(field.is_valid());
        }
    }

    #[test]
    fn test_field12_creation_invalid() {
        let invalid_types = ["939", "943", "999", "12", "1234", "abc", ""];

        for msg_type in &invalid_types {
            let result = Field12::new(msg_type);
            assert!(result.is_err(), "Should fail for: {}", msg_type);
        }
    }

    #[test]
    fn test_field12_parse() {
        let field = Field12::parse("942").unwrap();
        assert_eq!(field.message_type, "942");

        let field = Field12::parse(":12:941").unwrap();
        assert_eq!(field.message_type, "941");

        let field = Field12::parse("12:950").unwrap();
        assert_eq!(field.message_type, "950");
    }

    #[test]
    fn test_field12_to_swift_string() {
        let field = Field12::new("940").unwrap();
        assert_eq!(field.to_swift_string(), "940");
    }

    #[test]
    fn test_field12_format_spec() {
        assert_eq!(Field12::format_spec(), "3!n");
    }

    #[test]
    fn test_field12_type_checks() {
        let stmt940 = Field12::new("940").unwrap();
        assert!(stmt940.is_customer_statement_request());
        assert!(!stmt940.is_balance_report_request());
        assert!(!stmt940.is_interim_report_request());

        let stmt950 = Field12::new("950").unwrap();
        assert!(stmt950.is_customer_statement_request());

        let balance = Field12::new("941").unwrap();
        assert!(balance.is_balance_report_request());
        assert!(!balance.is_customer_statement_request());

        let interim = Field12::new("942").unwrap();
        assert!(interim.is_interim_report_request());
        assert!(!interim.is_customer_statement_request());
    }

    #[test]
    fn test_field12_descriptions() {
        let field940 = Field12::new("940").unwrap();
        assert_eq!(field940.get_description(), "Customer Statement Message");

        let field941 = Field12::new("941").unwrap();
        assert_eq!(field941.get_description(), "Balance Report");

        let field942 = Field12::new("942").unwrap();
        assert_eq!(field942.get_description(), "Interim Transaction Report");

        let field950 = Field12::new("950").unwrap();
        assert_eq!(
            field950.get_description(),
            "Customer Statement Message (Consolidated)"
        );
    }

    #[test]
    fn test_field12_display() {
        let field = Field12::new("942").unwrap();
        let display = format!("{}", field);
        assert!(display.contains("942"));
        assert!(display.contains("Interim Transaction Report"));
    }

    #[test]
    fn test_field12_serialization() {
        let field = Field12::new("941").unwrap();
        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: Field12 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field, deserialized);
    }
}
