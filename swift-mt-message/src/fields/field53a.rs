use crate::common::BIC;
use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 53A: Sender's Correspondent
///
/// ## Overview
/// Field 53A identifies the sender's correspondent institution in SWIFT payment messages.
/// This field specifies the financial institution that acts as a correspondent for the
/// message sender, facilitating the payment routing and settlement process. The correspondent
/// relationship is crucial for cross-border payments and correspondent banking arrangements.
///
/// ## Format Specification
/// **Format**: `[/1!c][/34x]4!a2!a2!c[3!c]`
/// - **1!c**: Optional account line indicator (1 character)
/// - **34x**: Optional account number (up to 34 characters)
/// - **4!a2!a2!c[3!c]**: BIC code (8 or 11 characters)
///
/// ### BIC Structure
/// ```text
/// CHASUS33XXX
/// ││││││││└┴┴─ Branch Code (3 characters, optional)
/// ││││││└┴──── Location Code (2 characters)
/// ││││└┴────── Country Code (2 letters)
/// └┴┴┴──────── Bank Code (4 letters)
/// ```
///
/// ## Field Components
/// - **Account Line Indicator**: Optional qualifier for account type or purpose
/// - **Account Number**: Correspondent account number for settlement
/// - **BIC**: Bank Identifier Code of the correspondent institution
///
/// ## Usage Context
/// Field 53A is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
///
/// ### Business Applications
/// - **Correspondent banking**: Identifying correspondent bank relationships
/// - **Payment routing**: Providing routing instructions for payment processing
/// - **Settlement**: Facilitating settlement through correspondent accounts
/// - **Risk management**: Managing correspondent banking exposure and limits
/// - **Compliance**: Meeting regulatory requirements for correspondent relationships
///
/// ## Examples
/// ```text
/// :53A:CHASUS33XXX
/// └─── JPMorgan Chase New York as correspondent
///
/// :53A:/C/1234567890
/// DEUTDEFFXXX
/// └─── Deutsche Bank with checking account 1234567890
///
/// :53A:/N/LORO12345678901234567890
/// BNPAFRPPXXX
/// └─── BNP Paribas with nostro account identifier
///
/// :53A:/V/VOSTRO001234567890123456
/// ABCDEFGHJKL
/// └─── Correspondent with vostro account reference
/// ```
///
/// ## Account Line Indicators
/// Common account line indicators for correspondent relationships:
/// - **C**: Correspondent account (checking)
/// - **D**: Deposit account
/// - **L**: Loan account
/// - **N**: Nostro account (our account with them)
/// - **S**: Settlement account
/// - **V**: Vostro account (their account with us)
///
/// ## BIC Components Analysis
/// ### Bank Code (Characters 1-4)
/// - Must be 4 alphabetic characters
/// - Identifies the specific financial institution
/// - Assigned by SWIFT registration authority
///
/// ### Country Code (Characters 5-6)
/// - Must be valid ISO 3166-1 alpha-2 country code
/// - Identifies the country of the institution
/// - Must match BIC registration country
///
/// ### Location Code (Characters 7-8)
/// - Alphanumeric characters identifying location within country
/// - Often represents city or administrative division
/// - Used for routing within correspondent networks
///
/// ### Branch Code (Characters 9-11)
/// - Optional 3-character branch identifier
/// - Identifies specific branch or department
/// - XXX indicates head office if present
///
/// ## Validation Rules
/// 1. **BIC format**: Must be valid 8 or 11 character BIC
/// 2. **BIC structure**: 4!a2!a2!c[3!c] format required
/// 3. **Account line indicator**: If present, exactly 1 character
/// 4. **Account number**: If present, max 34 characters
/// 5. **Character validation**: All components must use valid character sets
///
/// ## Network Validated Rules (SWIFT Standards)
/// - BIC must be valid format and registered (Error: T27)
/// - Account line indicator must be single character (Error: T12)
/// - Account number cannot exceed 34 characters (Error: T15)
/// - BIC country code must be valid ISO country code (Error: T28)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Field 53A is conditional based on message type (Error: C53)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field53A {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    #[serde(flatten)]
    pub bic: BIC,
}

impl Field53A {
    /// Create a new Field53A with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let bic = bic.into().to_uppercase();

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Parse and validate BIC using the common structure
        let parsed_bic = BIC::parse(&bic, Some("53A"))?;

        Ok(Field53A {
            account_line_indicator,
            account_number,
            bic: parsed_bic,
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

    /// Get the BIC code
    pub fn bic(&self) -> &str {
        self.bic.value()
    }

    /// Check if this is a full BIC (11 characters) or short BIC (8 characters)
    pub fn is_full_bic(&self) -> bool {
        self.bic.is_full_bic()
    }

    /// Get the bank code (first 4 characters of BIC)
    pub fn bank_code(&self) -> &str {
        self.bic.bank_code()
    }

    /// Get the country code (characters 5-6 of BIC)
    pub fn country_code(&self) -> &str {
        self.bic.country_code()
    }

    /// Get the location code (characters 7-8 of BIC)
    pub fn location_code(&self) -> &str {
        self.bic.location_code()
    }

    /// Get the branch code (characters 9-11 of BIC, if present)
    pub fn branch_code(&self) -> Option<&str> {
        self.bic.branch_code()
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match &self.account_number {
            Some(account) => format!("Sender's Correspondent: {} ({})", self.bic, account),
            None => format!("Sender's Correspondent: {}", self.bic),
        }
    }
}

impl SwiftField for Field53A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":53A:") {
            stripped // Remove ":53A:" prefix
        } else if let Some(stripped) = value.strip_prefix("53A:") {
            stripped // Remove "53A:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Parse account and BIC
        let mut account_number = None;
        let bic;

        if content.starts_with('/') {
            // Has account number
            let lines: Vec<&str> = content.lines().collect();

            if lines.len() == 1 {
                // Account and BIC on same line: "/account BIC" or "/account\nBIC"
                let parts: Vec<&str> = lines[0].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    account_number = Some(parts[0][1..].to_string()); // Remove leading '/'
                    bic = parts[1].to_string();
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "53A".to_string(),
                        message: "Invalid format: expected account and BIC".to_string(),
                    });
                }
            } else if lines.len() == 2 {
                // Account and BIC on separate lines
                account_number = Some(lines[0][1..].to_string()); // Remove leading '/'
                bic = lines[1].to_string();
            } else {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53A".to_string(),
                    message: "Invalid format: too many lines".to_string(),
                });
            }
        } else {
            // No account number, just BIC
            bic = content.to_string();
        }

        let parsed_bic = BIC::parse(&bic, Some("53A"))?;

        Ok(Field53A {
            account_line_indicator: None,
            account_number,
            bic: parsed_bic,
        })
    }

    fn to_swift_string(&self) -> String {
        match &self.account_number {
            Some(account) => format!(":53A:/{}\n{}", account, self.bic.value()),
            None => format!(":53A:{}", self.bic.value()),
        }
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate account line indicator if present
        if let Some(ref indicator) = self.account_line_indicator {
            if indicator.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "53A".to_string(),
                    expected: "exactly 1 character".to_string(),
                    actual: indicator.len(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "53A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = self.account_number {
            if account.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "53A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "53A".to_string(),
                    expected: "max 34 characters".to_string(),
                    actual: account.len(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "53A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Validate BIC format using the common BIC validation
        let bic_validation = self.bic.validate();
        if !bic_validation.is_valid {
            errors.extend(bic_validation.errors);
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "[/1!c][/34x]BIC"
    }
}

impl std::fmt::Display for Field53A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.account_line_indicator, &self.account_number) {
            (Some(indicator), Some(account)) => write!(f, "/{}{} {}", indicator, account, self.bic),
            (None, Some(account)) => write!(f, "/{} {}", account, self.bic),
            _ => write!(f, "{}", self.bic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field53a_creation_bic_only() {
        let field = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert!(field.account_number().is_none());
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field53a_creation_with_account() {
        let field = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field53a_parse_bic_only() {
        let field = Field53A::parse("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field53a_parse_with_account_same_line() {
        let field = Field53A::parse("/1234567890 DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field53a_parse_with_account_separate_lines() {
        let field = Field53A::parse("/1234567890\nDEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field53a_parse_with_prefix() {
        let field = Field53A::parse(":53A:CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field53a_to_swift_string_bic_only() {
        let field = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.to_swift_string(), ":53A:DEUTDEFF");
    }

    #[test]
    fn test_field53a_to_swift_string_with_account() {
        let field = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.to_swift_string(), ":53A:/1234567890\nDEUTDEFF500");
    }

    #[test]
    fn test_field53a_bic_components() {
        let field = Field53A::new(None, None, "DEUTDEFF500").unwrap();
        assert_eq!(field.bank_code(), "DEUT");
        assert_eq!(field.country_code(), "DE");
        assert_eq!(field.location_code(), "FF");
        assert_eq!(field.branch_code(), Some("500"));
    }

    #[test]
    fn test_field53a_short_bic_components() {
        let field = Field53A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(field.bank_code(), "CHAS");
        assert_eq!(field.country_code(), "US");
        assert_eq!(field.location_code(), "33");
        assert_eq!(field.branch_code(), None);
    }

    #[test]
    fn test_field53a_invalid_bic_length() {
        let result = Field53A::new(None, None, "DEUT");
        assert!(result.is_err());

        let result = Field53A::new(None, None, "DEUTDEFF5001");
        assert!(result.is_err());
    }

    #[test]
    fn test_field53a_invalid_bic_format() {
        let result = Field53A::new(None, None, "123TDEFF");
        assert!(result.is_err());

        let result = Field53A::new(None, None, "DEUT12FF");
        assert!(result.is_err());

        let result = Field53A::new(None, None, "DEUTDE@F");
        assert!(result.is_err());
    }

    #[test]
    fn test_field53a_invalid_account() {
        let result = Field53A::new(None, Some("".to_string()), "DEUTDEFF");
        assert!(result.is_err());

        let result = Field53A::new(None, Some("A".repeat(35)), "DEUTDEFF");
        assert!(result.is_err());
    }

    #[test]
    fn test_field53a_validation() {
        let field = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field53a_display() {
        let field1 = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field1), "DEUTDEFF");

        let field2 = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field2), "/1234567890 DEUTDEFF");
    }

    #[test]
    fn test_field53a_description() {
        let field1 = Field53A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field1.description(), "Sender's Correspondent: DEUTDEFF");

        let field2 = Field53A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(
            field2.description(),
            "Sender's Correspondent: DEUTDEFF (1234567890)"
        );
    }

    #[test]
    fn test_field53a_case_normalization() {
        let field = Field53A::new(None, None, "deutdeff").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
    }
}
