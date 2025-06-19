use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Generic Party Field
///
/// ## Overview
/// A generic field structure for SWIFT party identification fields that follow the
/// `[/account_line_indicator][/account_number]party_identifier` pattern. This structure
/// consolidates the common functionality used by Field53B, Field54B, Field55B, and Field57B.
///
/// ## Format Specification
/// **Format**: `[/1!a][/34x]35x`
/// - **[/1!a]**: Optional account line indicator (1 character preceded by /)
/// - **[/34x]**: Optional account number (up to 34 characters preceded by /)
/// - **35x**: Party identifier (up to 35 characters)
///
/// ### Component Details
/// 1. **Account Line Indicator (Optional)**:
///    - Single character identifier for account type
///    - Common values: C (Correspondent), D (Deposit), S (Settlement), N (Nostro), V (Vostro)
///    - Must be exactly 1 character if present
///    - Preceded by forward slash (/)
///
/// 2. **Account Number (Optional)**:
///    - Account identifier at the institution
///    - Up to 34 characters
///    - Can be IBAN, account number, or other identifier
///    - Preceded by forward slash (/)
///
/// 3. **Party Identifier (Required)**:
///    - Institution or party identifier
///    - Up to 35 characters
///    - Can be clearing codes, member codes, registry codes, etc.
///    - Must be meaningful and recognizable
///
/// ## Usage Examples
/// ```text
/// PARTYID12345
/// └─── Simple party identifier
///
/// /1234567890
/// PARTYID12345
/// └─── Party with account number
///
/// /C/1234567890
/// PARTYID12345
/// └─── Party with account line indicator and account number
///
/// /S
/// PARTYID12345
/// └─── Party with account line indicator only
/// ```
///
/// ## Validation Rules
/// 1. **Party identifier**: Cannot be empty, max 35 characters
/// 2. **Account line indicator**: If present, exactly 1 character
/// 3. **Account number**: If present, max 34 characters
/// 4. **Character validation**: All components must be printable ASCII
/// 5. **Content requirement**: Must contain meaningful identification
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Party identifier cannot be empty (Error: T11)
/// - Party identifier cannot exceed 35 characters (Error: T14)
/// - Account line indicator must be single character (Error: T12)
/// - Account number cannot exceed 34 characters (Error: T15)
/// - Characters must be from SWIFT character set (Error: T61)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenericPartyField {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Party identifier (up to 35 characters)
    pub party_identifier: String,
}

impl GenericPartyField {
    /// Create a new GenericPartyField with validation
    ///
    /// # Arguments
    /// * `account_line_indicator` - Optional single character indicator
    /// * `account_number` - Optional account number (up to 34 characters)
    /// * `party_identifier` - Required party identifier (up to 35 characters)
    ///
    /// # Returns
    /// Result containing the GenericPartyField instance or validation error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::GenericPartyField;
    /// let field = GenericPartyField::new(
    ///     Some("C".to_string()),
    ///     Some("1234567890".to_string()),
    ///     "PARTYID12345"
    /// ).unwrap();
    /// ```
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        party_identifier: impl Into<String>,
    ) -> Result<Self, ParseError> {
        let party_identifier = party_identifier.into().trim().to_string();

        // Validate party identifier
        if party_identifier.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericPartyField".to_string(),
                message: "Party identifier cannot be empty".to_string(),
            });
        }

        if party_identifier.len() > 35 {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericPartyField".to_string(),
                message: "Party identifier cannot exceed 35 characters".to_string(),
            });
        }

        if !party_identifier
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: "GenericPartyField".to_string(),
                message: "Party identifier contains invalid characters".to_string(),
            });
        }

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericPartyField".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericPartyField".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericPartyField".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericPartyField".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericPartyField".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: "GenericPartyField".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        Ok(GenericPartyField {
            account_line_indicator,
            account_number,
            party_identifier,
        })
    }

    /// Get the account line indicator
    pub fn account_line_indicator(&self) -> Option<&str> {
        self.account_line_indicator.as_deref()
    }

    /// Get the account number
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }

    /// Get the party identifier
    pub fn party_identifier(&self) -> &str {
        &self.party_identifier
    }

    /// Parse content with custom field tag for error messages
    pub fn parse_with_tag(content: &str, field_tag: &str) -> Result<Self, ParseError> {
        let content = content.trim();
        if content.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present
        let content = if let Some(stripped) = content.strip_prefix(&format!(":{}:", field_tag)) {
            stripped
        } else if let Some(stripped) = content.strip_prefix(&format!("{}:", field_tag)) {
            stripped
        } else {
            content
        };

        let mut account_line_indicator = None;
        let mut account_number = None;
        let mut party_identifier_content = content;

        // Check for account line indicator (starts with /)
        if content.starts_with('/') {
            let lines: Vec<&str> = content.lines().collect();
            if !lines.is_empty() {
                let first_line = lines[0];

                if first_line.len() == 2 && first_line.starts_with('/') {
                    // Only account line indicator: /X
                    account_line_indicator = Some(first_line[1..].to_string());
                    party_identifier_content = if lines.len() > 1 { lines[1] } else { "" };
                } else if first_line.len() > 2 && first_line.starts_with('/') {
                    // Account line indicator + account number: /X/account or /account
                    let parts: Vec<&str> = first_line[1..].split('/').collect();
                    if parts.len() == 2 {
                        // /X/account format
                        account_line_indicator = Some(parts[0].to_string());
                        account_number = Some(parts[1].to_string());
                    } else {
                        // /account format
                        account_number = Some(parts[0].to_string());
                    }
                    party_identifier_content = if lines.len() > 1 { lines[1] } else { "" };
                }
            }
        }

        let party_identifier = party_identifier_content.trim().to_string();
        if party_identifier.is_empty() {
            return Err(ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Party identifier is required".to_string(),
            });
        }

        Self::new(account_line_indicator, account_number, party_identifier)
    }

    /// Convert to SWIFT string format with custom field tag
    pub fn to_swift_string_with_tag(&self, field_tag: &str) -> String {
        let mut result = String::new();

        if let Some(ref indicator) = self.account_line_indicator {
            result.push('/');
            result.push_str(indicator);
        }

        if let Some(ref account) = self.account_number {
            result.push('/');
            result.push_str(account);
        }

        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&self.party_identifier);

        format!(":{}:{}", field_tag, result)
    }

    /// Get human-readable description with custom context
    pub fn description(&self, context: &str) -> String {
        match (&self.account_line_indicator, &self.account_number) {
            (Some(indicator), Some(account)) => format!(
                "{} (Indicator: {}, Account: {}, Party: {})",
                context, indicator, account, self.party_identifier
            ),
            (None, Some(account)) => format!(
                "{} (Account: {}, Party: {})",
                context, account, self.party_identifier
            ),
            (Some(indicator), None) => format!(
                "{} (Indicator: {}, Party: {})",
                context, indicator, self.party_identifier
            ),
            (None, None) => format!("{} (Party: {})", context, self.party_identifier),
        }
    }
}

impl SwiftField for GenericPartyField {
    fn parse(content: &str) -> Result<Self, ParseError> {
        Self::parse_with_tag(content, "GenericPartyField")
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_string_with_tag("GenericPartyField")
    }

    fn validate(&self) -> ValidationResult {
        // Validation is done in constructor
        ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "[/1!a][/34x]35x"
    }
}

impl std::fmt::Display for GenericPartyField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.account_line_indicator, &self.account_number) {
            (Some(indicator), Some(account)) => write!(
                f,
                "Indicator: {}, Account: {}, Party: {}",
                indicator, account, self.party_identifier
            ),
            (None, Some(account)) => {
                write!(f, "Account: {}, Party: {}", account, self.party_identifier)
            }
            (Some(indicator), None) => write!(
                f,
                "Indicator: {}, Party: {}",
                indicator, self.party_identifier
            ),
            (None, None) => write!(f, "Party: {}", self.party_identifier),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_party_field_creation_party_only() {
        let field = GenericPartyField::new(None, None, "PARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert!(field.account_number().is_none());
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_generic_party_field_creation_with_account() {
        let field =
            GenericPartyField::new(None, Some("1234567890".to_string()), "PARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_generic_party_field_creation_with_account_line_indicator() {
        let field = GenericPartyField::new(
            Some("C".to_string()),
            Some("1234567890".to_string()),
            "PARTYID12345",
        )
        .unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.account_line_indicator(), Some("C"));
    }

    #[test]
    fn test_generic_party_field_parse_party_only() {
        let field = GenericPartyField::parse("PARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_generic_party_field_parse_with_account() {
        let field = GenericPartyField::parse("/1234567890\nPARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_generic_party_field_parse_with_account_line_indicator() {
        let field = GenericPartyField::parse("/C/1234567890\nPARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.account_line_indicator(), Some("C"));
    }

    #[test]
    fn test_generic_party_field_parse_with_tag() {
        let field = GenericPartyField::parse_with_tag(":53B:PARTYID12345", "53B").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
    }

    #[test]
    fn test_generic_party_field_to_swift_string_with_tag() {
        let field = GenericPartyField::new(
            Some("C".to_string()),
            Some("1234567890".to_string()),
            "PARTYID12345",
        )
        .unwrap();
        assert_eq!(
            field.to_swift_string_with_tag("53B"),
            ":53B:/C/1234567890\nPARTYID12345"
        );
    }

    #[test]
    fn test_generic_party_field_validation_errors() {
        // Empty party identifier
        let result = GenericPartyField::new(None, None, "");
        assert!(result.is_err());

        // Party identifier too long
        let result = GenericPartyField::new(None, None, "A".repeat(36));
        assert!(result.is_err());

        // Account line indicator too long
        let result = GenericPartyField::new(Some("AB".to_string()), None, "PARTYID12345");
        assert!(result.is_err());

        // Account number too long
        let result = GenericPartyField::new(None, Some("A".repeat(35)), "PARTYID12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_generic_party_field_display() {
        let field = GenericPartyField::new(
            Some("C".to_string()),
            Some("1234567890".to_string()),
            "PARTYID12345",
        )
        .unwrap();
        assert_eq!(
            format!("{}", field),
            "Indicator: C, Account: 1234567890, Party: PARTYID12345"
        );
    }

    #[test]
    fn test_generic_party_field_description() {
        let field = GenericPartyField::new(None, None, "PARTYID12345").unwrap();
        assert_eq!(
            field.description("Test Context"),
            "Test Context (Party: PARTYID12345)"
        );
    }
}
