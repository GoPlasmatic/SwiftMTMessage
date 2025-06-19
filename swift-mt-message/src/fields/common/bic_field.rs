use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// Generic BIC-based field structure for institutional fields
///
/// This structure represents any SWIFT field that uses BIC (Bank Identifier Code)
/// for institutional identification. It supports optional account line indicators
/// and account numbers as defined in SWIFT MT message specifications.
///
/// ## Format Specification
/// **Option A Format**: `[/account_indicator][/account_number]<CR><LF>BIC`
///
/// Where:
/// - `account_indicator`: Optional single character (A-Z, 0-9)
/// - `account_number`: Optional account identifier (up to 34 characters)
/// - `BIC`: Bank Identifier Code (8 or 11 characters)
///
/// ## Examples
/// ```text
/// CHASUS33XXX                    // BIC only
/// /A/123456789<CR><LF>CHASUS33XXX // With account indicator and number
/// /123456789<CR><LF>CHASUS33XXX   // With account number only
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenericBicField {
    /// Optional account line indicator (single character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,

    /// Optional account number (up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Bank Identifier Code (8 or 11 characters)
    pub bic: String,
}

impl GenericBicField {
    /// Create a new GenericBicField with validation
    ///
    /// # Arguments
    /// * `account_line_indicator` - Optional single character indicator
    /// * `account_number` - Optional account number (max 34 chars)
    /// * `bic` - Bank Identifier Code (8 or 11 chars)
    ///
    /// # Returns
    /// * `Ok(GenericBicField)` - Successfully created field
    /// * `Err(String)` - Validation error message
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> Result<Self, String> {
        let bic = bic.into();

        // Validate BIC format (8 or 11 characters)
        if bic.len() != 8 && bic.len() != 11 {
            return Err(format!(
                "Invalid BIC length: expected 8 or 11 characters, got {}",
                bic.len()
            ));
        }

        // Validate BIC characters (alphanumeric only)
        if !bic.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err("BIC code must contain only alphanumeric characters".to_string());
        }

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.len() != 1 {
                return Err("Account line indicator must be exactly 1 character".to_string());
            }
            if !indicator.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err("Account line indicator must be alphanumeric".to_string());
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err("Account number cannot be empty".to_string());
            }
            if account.len() > 34 {
                return Err("Account number cannot exceed 34 characters".to_string());
            }
            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err("Account number contains invalid characters".to_string());
            }
        }

        Ok(GenericBicField {
            account_line_indicator,
            account_number,
            bic,
        })
    }

    /// Convert to SWIFT message format
    ///
    /// # Arguments
    /// * `field_tag` - SWIFT field tag (e.g., "52A", "53A")
    ///
    /// # Returns
    /// * `String` - Formatted SWIFT field content
    pub fn to_swift_format(&self, field_tag: &str) -> String {
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
        result.push_str(&self.bic);

        format!(":{}:{}", field_tag, result)
    }

    // Accessor methods

    /// Get the account line indicator
    pub fn account_line_indicator(&self) -> Option<&str> {
        self.account_line_indicator.as_deref()
    }

    /// Get the account number
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }

    /// Get the BIC code
    pub fn bic(&self) -> &str {
        &self.bic
    }

    /// Check if this is a full BIC (11 characters) or short BIC (8 characters)
    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }

    /// Get the bank code (first 4 characters of BIC)
    pub fn bank_code(&self) -> &str {
        &self.bic[0..4]
    }

    /// Get the country code (characters 5-6 of BIC)
    pub fn country_code(&self) -> &str {
        &self.bic[4..6]
    }

    /// Get the location code (characters 7-8 of BIC)
    pub fn location_code(&self) -> &str {
        &self.bic[6..8]
    }

    /// Get the branch code (characters 9-11 of BIC, if present)
    pub fn branch_code(&self) -> Option<&str> {
        if self.bic.len() == 11 {
            Some(&self.bic[8..11])
        } else {
            None
        }
    }

    /// Check if the BIC represents a test BIC (location code ends with '0')
    pub fn is_test_bic(&self) -> bool {
        self.location_code().ends_with('0')
    }

    /// Check if the BIC is passive (location code ends with '1')
    pub fn is_passive_bic(&self) -> bool {
        self.location_code().ends_with('1')
    }

    /// Check if the BIC is reverse billing (location code ends with '2')
    pub fn is_reverse_billing_bic(&self) -> bool {
        self.location_code().ends_with('2')
    }
}

impl std::fmt::Display for GenericBicField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        result.push_str(&self.bic);

        write!(f, "{}", result)
    }
}

impl SwiftField for GenericBicField {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let content = input.trim();
        if content.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Remove field tag prefix if present (e.g., ":52A:", ":53A:", etc.)
        let content = if let Some(colon_pos) = content.find(':') {
            if let Some(second_colon_pos) = content[colon_pos + 1..].find(':') {
                &content[colon_pos + second_colon_pos + 2..]
            } else {
                content
            }
        } else {
            content
        };

        let mut account_line_indicator = None;
        let mut account_number = None;
        let mut bic_content = content;

        // Check for account line indicator (starts with /)
        if content.starts_with('/') {
            let lines: Vec<&str> = content.lines().collect();
            if !lines.is_empty() {
                let first_line = lines[0];

                if first_line.len() == 2 && first_line.starts_with('/') {
                    // Only account line indicator: /X
                    account_line_indicator = Some(first_line[1..].to_string());
                    bic_content = if lines.len() > 1 { lines[1] } else { "" };
                } else if first_line.len() > 2 && first_line.starts_with('/') {
                    // Account line indicator + account number: /X/account or /account
                    let parts: Vec<&str> = first_line[1..].split('/').collect();
                    if parts.len() == 2
                        && parts[0].len() == 1
                        && parts[0].chars().all(|c| c.is_ascii_alphanumeric())
                    {
                        // /X/account format - only if first part is exactly 1 alphanumeric character
                        account_line_indicator = Some(parts[0].to_string());
                        account_number = Some(parts[1].to_string());
                    } else {
                        // /account format (treat everything after first / as account number)
                        account_number = Some(first_line[1..].to_string());
                    }
                    bic_content = if lines.len() > 1 { lines[1] } else { "" };
                }
            }
        }

        let bic_str = bic_content.trim();
        if bic_str.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "BIC code is required".to_string(),
            });
        }

        Self::new(account_line_indicator, account_number, bic_str)
            .map_err(|e| crate::errors::ParseError::InvalidFormat { message: e })
    }

    fn to_swift_string(&self) -> String {
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
        result.push_str(&self.bic);

        result
    }

    fn validate(&self) -> crate::ValidationResult {
        let mut errors = Vec::new();

        // Validate BIC format
        if self.bic.len() != 8 && self.bic.len() != 11 {
            errors.push(crate::errors::ValidationError::FormatValidation {
                field_tag: "BIC".to_string(),
                message: "Invalid BIC length: expected 8 or 11 characters".to_string(),
            });
        }

        if !self.bic.chars().all(|c| c.is_ascii_alphanumeric()) {
            errors.push(crate::errors::ValidationError::FormatValidation {
                field_tag: "BIC".to_string(),
                message: "BIC code must contain only alphanumeric characters".to_string(),
            });
        }

        // Validate account line indicator
        if let Some(indicator) = &self.account_line_indicator {
            if indicator.len() != 1 {
                errors.push(crate::errors::ValidationError::FormatValidation {
                    field_tag: "BIC".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }
            if !indicator.chars().all(|c| c.is_ascii_alphanumeric()) {
                errors.push(crate::errors::ValidationError::FormatValidation {
                    field_tag: "BIC".to_string(),
                    message: "Account line indicator must be alphanumeric".to_string(),
                });
            }
        }

        // Validate account number
        if let Some(account) = &self.account_number {
            if account.len() > 34 {
                errors.push(crate::errors::ValidationError::FormatValidation {
                    field_tag: "BIC".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }
            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(crate::errors::ValidationError::FormatValidation {
                    field_tag: "BIC".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        if errors.is_empty() {
            crate::ValidationResult::valid()
        } else {
            crate::ValidationResult::with_errors(errors)
        }
    }

    fn format_spec() -> &'static str {
        "BIC"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bic_only() {
        let field = GenericBicField::new(None, None, "CHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
        assert_eq!(field.account_line_indicator(), None);
        assert_eq!(field.account_number(), None);
        assert!(field.is_full_bic());
        assert_eq!(field.bank_code(), "CHAS");
        assert_eq!(field.country_code(), "US");
        assert_eq!(field.location_code(), "33");
        assert_eq!(field.branch_code(), Some("XXX"));
    }

    #[test]
    fn test_with_account_number() {
        let field = GenericBicField::new(None, Some("123456789".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert_eq!(field.account_number(), Some("123456789"));
        assert!(!field.is_full_bic());
        assert_eq!(field.branch_code(), None);
    }

    #[test]
    fn test_with_account_indicator_and_number() {
        let field = GenericBicField::new(
            Some("A".to_string()),
            Some("987654321".to_string()),
            "BNPAFRPPXXX",
        )
        .unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_line_indicator(), Some("A"));
        assert_eq!(field.account_number(), Some("987654321"));
    }

    #[test]
    fn test_parse_bic_only() {
        let field = GenericBicField::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
        assert_eq!(field.account_line_indicator(), None);
        assert_eq!(field.account_number(), None);
    }

    #[test]
    fn test_parse_with_account() {
        let field = GenericBicField::parse("/123456789\nDEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert_eq!(field.account_number(), Some("123456789"));
        assert_eq!(field.account_line_indicator(), None);
    }

    #[test]
    fn test_parse_with_indicator_and_account() {
        let field = GenericBicField::parse("/A/987654321\nBNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_line_indicator(), Some("A"));
        assert_eq!(field.account_number(), Some("987654321"));
    }

    #[test]
    fn test_to_swift_format() {
        let field = GenericBicField::new(
            Some("A".to_string()),
            Some("123456789".to_string()),
            "CHASUS33XXX",
        )
        .unwrap();
        assert_eq!(
            field.to_swift_format("52A"),
            ":52A:/A/123456789\nCHASUS33XXX"
        );
    }

    #[test]
    fn test_display() {
        let field = GenericBicField::new(None, Some("123456789".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field), "/123456789\nDEUTDEFF");
    }

    #[test]
    fn test_invalid_bic_length() {
        let result = GenericBicField::new(None, None, "INVALID");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid BIC length"));
    }

    #[test]
    fn test_invalid_account_indicator() {
        let result = GenericBicField::new(Some("AB".to_string()), None, "CHASUS33XXX");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Account line indicator must be exactly 1 character")
        );
    }

    #[test]
    fn test_invalid_account_number_length() {
        let long_account = "A".repeat(35);
        let result = GenericBicField::new(None, Some(long_account), "CHASUS33XXX");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Account number cannot exceed 34 characters")
        );
    }

    #[test]
    fn test_bic_properties() {
        let field = GenericBicField::new(None, None, "CHASUS33XXX").unwrap();
        assert!(!field.is_test_bic());
        assert!(!field.is_passive_bic());
        assert!(!field.is_reverse_billing_bic());

        let test_bic = GenericBicField::new(None, None, "CHASUS30XXX").unwrap();
        assert!(test_bic.is_test_bic());
    }

    #[test]
    fn test_validation() {
        let field = GenericBicField::new(None, None, "CHASUS33XXX").unwrap();
        let validation_result = field.validate();
        assert!(validation_result.is_valid);

        // Test with invalid data (bypassing constructor validation)
        let invalid_field = GenericBicField {
            account_line_indicator: Some("AB".to_string()), // Invalid: too long
            account_number: Some("A".repeat(35)),           // Invalid: too long
            bic: "INVALID".to_string(),                     // Invalid: wrong length
        };
        let validation_result = invalid_field.validate();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.len() >= 3);
    }
}
