use crate::common::BIC;
use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 51A: Sending Institution
///
/// ## Overview
/// Field 51A identifies the sending institution in SWIFT payment messages using a BIC code.
/// This field specifies the financial institution that is sending the payment message,
/// typically used in correspondent banking arrangements and institutional transfers.
/// The sending institution is distinct from the ordering institution and represents
/// the actual message sender in the SWIFT network, providing crucial information for
/// message routing, settlement processing, and regulatory compliance.
///
/// ## Format Specification
/// **Format**: `[/1!a][/34x]4!a2!a2!c[3!c]`
/// - **1!a**: Optional account line indicator (1 character)
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
/// - **Account Number**: Institution's account number for settlement
/// - **BIC**: Bank Identifier Code uniquely identifying the sending institution
///
/// ## Usage Context
/// Field 51A is used in:
/// - **MT103**: Single Customer Credit Transfer (optional)
/// - **MT103.REMIT**: Single Customer Credit Transfer with Remittance (optional)
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
///
/// ### Business Applications
/// - **Correspondent banking**: Identifying the actual message sender
/// - **Settlement**: Providing account information for settlement processes
/// - **Compliance**: Meeting regulatory requirements for sender identification
/// - **Audit trails**: Maintaining clear sender identification records
/// - **Message routing**: Supporting proper SWIFT network routing
/// - **Risk management**: Enabling counterparty risk assessment
///
/// ## MT103 Variant Support
/// - **MT103 Core**: Optional field
/// - **MT103.STP**: **Not allowed** (STP compliance restriction)
/// - **MT103.REMIT**: Optional field
///
/// ## Examples
/// ```rust
/// use swift_mt_message::fields::Field51A;
///
/// // BIC only
/// let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
///
/// // With account number
/// let field = Field51A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
///
/// // With account line indicator and account number
/// let field = Field51A::new(Some("C".to_string()), Some("1234567890".to_string()), "HSBCHKHH").unwrap();
/// ```
///
/// ## Account Line Indicators
/// Common account line indicators include:
/// - **A**: Account identifier (generic)
/// - **B**: Beneficiary account
/// - **C**: Checking account
/// - **D**: Deposit account
/// - **S**: SWIFT account identifier
/// - **T**: Trust account
/// - **N**: Nostro account
/// - **V**: Vostro account
///
/// ## Validation Rules
/// 1. **BIC format**: Must be valid 8 or 11 character BIC code
/// 2. **BIC structure**: 4!a2!a2!c[3!c] format required
/// 3. **Bank code**: Must be 4 alphabetic characters
/// 4. **Country code**: Must be 2 alphabetic characters
/// 5. **Location code**: Must be 2 alphanumeric characters
/// 6. **Branch code**: Must be 3 alphanumeric characters (if present)
/// 7. **Account line indicator**: If present, exactly 1 character
/// 8. **Account number**: If present, max 34 characters
/// 9. **Character validation**: All components must use valid character sets
///
/// ## Network Validated Rules (SWIFT Standards)
/// - BIC must be valid format and registered (Error: T27)
/// - BIC format must comply with ISO 13616 standards (Error: T11)
/// - Account line indicator must be single character (Error: T12)
/// - Account number cannot exceed 34 characters (Error: T14)
/// - Bank code must be alphabetic only (Error: T15)
/// - Country code must be valid ISO 3166-1 code (Error: T16)
/// - Location code must be alphanumeric (Error: T17)
/// - Branch code must be alphanumeric if present (Error: T18)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Field 51A not allowed in MT103.STP messages (Error: C51)
/// - Field 51A optional in MT103 Core and MT103.REMIT (Warning: W51)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field51A {
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

impl SwiftField for Field51A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":51A:") {
            stripped
        } else if let Some(stripped) = value.strip_prefix("51A:") {
            stripped
        } else {
            value
        };

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "51A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let lines: Vec<&str> = content.lines().collect();

        if lines.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "51A".to_string(),
                message: "No content found".to_string(),
            });
        }

        let mut account_line_indicator = None;
        let mut account_number = None;
        let mut bic_line = lines[0];

        // Check if first line contains account information
        if bic_line.starts_with('/') {
            // Parse account information from first line
            let account_part = &bic_line[1..]; // Remove leading '/'

            if let Some(second_slash) = account_part.find('/') {
                // Format: /indicator/account
                account_line_indicator = Some(account_part[..second_slash].to_string());
                account_number = Some(account_part[second_slash + 1..].to_string());
            } else {
                // Format: /account
                account_number = Some(account_part.to_string());
            }

            // BIC should be on second line
            if lines.len() < 2 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "51A".to_string(),
                    message: "BIC code missing after account information".to_string(),
                });
            }
            bic_line = lines[1];
        }

        let bic = BIC::parse(bic_line.trim(), Some("51A"))?;

        Ok(Self {
            account_line_indicator,
            account_number,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = ":51A:".to_string();

        if self.account_line_indicator.is_some() || self.account_number.is_some() {
            result.push('/');

            if let Some(indicator) = &self.account_line_indicator {
                result.push_str(indicator);
                result.push('/');
            }

            if let Some(account) = &self.account_number {
                result.push_str(account);
            }

            result.push('\n');
        }

        result.push_str(self.bic.value());
        result
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate BIC format using the common BIC validation
        let bic_validation = self.bic.validate();
        if !bic_validation.is_valid {
            errors.extend(bic_validation.errors);
        }

        // Validate account line indicator if present
        if let Some(indicator) = &self.account_line_indicator {
            if indicator.len() != 1 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "51A".to_string(),
                    expected: "1 character".to_string(),
                    actual: indicator.len(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii_alphanumeric()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "51A".to_string(),
                    message: "Account line indicator must be alphanumeric".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(account) = &self.account_number {
            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "51A".to_string(),
                    expected: "max 34 characters".to_string(),
                    actual: account.len(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "51A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "[/1!a][/34x]4!a2!a2!c[3!c]"
    }
}

impl Field51A {
    /// Create a new Field51A with validation
    ///
    /// # Arguments
    /// * `account_line_indicator` - Optional account line indicator (1 character)
    /// * `account_number` - Optional account number (up to 34 characters)
    /// * `bic` - BIC code (8 or 11 characters)
    ///
    /// # Examples
    /// ```rust
    /// use swift_mt_message::fields::Field51A;
    ///
    /// // BIC only
    /// let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
    ///
    /// // With account number
    /// let field = Field51A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
    ///
    /// // With account line indicator and account number
    /// let field = Field51A::new(Some("C".to_string()), Some("1234567890".to_string()), "HSBCHKHH").unwrap();
    /// ```
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> crate::Result<Self> {
        let account_line_indicator = account_line_indicator.map(|s| s.trim().to_string());
        let account_number = account_number.map(|s| s.trim().to_string());

        // Parse and validate BIC using the common structure
        let bic = BIC::parse(&bic.into(), Some("51A"))?;

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "51A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "51A".to_string(),
                    message: "Account line indicator must be alphanumeric".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "51A".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "51A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        Ok(Field51A {
            account_line_indicator,
            account_number,
            bic,
        })
    }

    /// Get the BIC code
    pub fn bic(&self) -> &str {
        self.bic.value()
    }

    /// Get the account line indicator if present
    pub fn account_line_indicator(&self) -> Option<&str> {
        self.account_line_indicator.as_deref()
    }

    /// Get the account number if present
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }

    /// Check if this field is allowed in MT103.STP messages
    ///
    /// Field 51A is NOT allowed in MT103.STP messages according to SWIFT standards
    pub fn is_stp_allowed(&self) -> bool {
        false
    }

    /// Get bank code from BIC
    pub fn bank_code(&self) -> &str {
        self.bic.bank_code()
    }

    /// Get country code from BIC
    pub fn country_code(&self) -> &str {
        self.bic.country_code()
    }

    /// Get location code from BIC
    pub fn location_code(&self) -> &str {
        self.bic.location_code()
    }

    /// Get branch code from BIC if present
    pub fn branch_code(&self) -> Option<&str> {
        self.bic.branch_code()
    }
}

impl std::fmt::Display for Field51A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(indicator) = &self.account_line_indicator {
            if let Some(account) = &self.account_number {
                write!(f, "/{}/{} {}", indicator, account, self.bic)
            } else {
                write!(f, "/{} {}", indicator, self.bic)
            }
        } else if let Some(account) = &self.account_number {
            write!(f, "/{} {}", account, self.bic)
        } else {
            write!(f, "{}", self.bic)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field51a_creation() {
        let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
        assert_eq!(field.account_line_indicator(), None);
        assert_eq!(field.account_number(), None);
    }

    #[test]
    fn test_field51a_with_account() {
        let field = Field51A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field51a_with_indicator_and_account() {
        let field = Field51A::new(
            Some("C".to_string()),
            Some("1234567890".to_string()),
            "HSBCHKHH",
        )
        .unwrap();
        assert_eq!(field.account_line_indicator(), Some("C"));
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.bic(), "HSBCHKHH");
    }

    #[test]
    fn test_field51a_parse() {
        let field = Field51A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");

        let field = Field51A::parse("/1234567890\nDEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));

        let field = Field51A::parse("/C/1234567890\nHSBCHKHH").unwrap();
        assert_eq!(field.bic(), "HSBCHKHH");
        assert_eq!(field.account_line_indicator(), Some("C"));
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field51a_parse_with_prefix() {
        let field = Field51A::parse(":51A:CHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");

        let field = Field51A::parse("51A:/1234567890\nDEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field51a_invalid_bic() {
        let result = Field51A::new(None, None, "INVALID");
        assert!(result.is_err());

        let result = Field51A::new(None, None, "CHAS1233");
        assert!(result.is_err());
    }

    #[test]
    fn test_field51a_invalid_account() {
        let result = Field51A::new(None, Some("a".repeat(35)), "CHASUS33XXX");
        assert!(result.is_err());

        let result = Field51A::new(Some("AB".to_string()), None, "CHASUS33XXX");
        assert!(result.is_err());
    }

    #[test]
    fn test_field51a_to_swift_string() {
        let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
        assert_eq!(field.to_swift_string(), ":51A:CHASUS33XXX");

        let field = Field51A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.to_swift_string(), ":51A:/1234567890\nDEUTDEFF500");

        let field = Field51A::new(
            Some("C".to_string()),
            Some("1234567890".to_string()),
            "HSBCHKHH",
        )
        .unwrap();
        assert_eq!(field.to_swift_string(), ":51A:/C/1234567890\nHSBCHKHH");
    }

    #[test]
    fn test_field51a_validation() {
        let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let invalid_field = Field51A {
            account_line_indicator: None,
            account_number: None,
            bic: BIC::new_unchecked("INVALID"),
        };
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field51a_stp_compliance() {
        let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
        assert!(!field.is_stp_allowed()); // Field 51A not allowed in STP
    }

    #[test]
    fn test_field51a_bic_components() {
        let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
        assert_eq!(field.bank_code(), "CHAS");
        assert_eq!(field.country_code(), "US");
        assert_eq!(field.location_code(), "33");
        assert_eq!(field.branch_code(), Some("XXX"));

        let field = Field51A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bank_code(), "DEUT");
        assert_eq!(field.country_code(), "DE");
        assert_eq!(field.location_code(), "FF");
        assert_eq!(field.branch_code(), None);
    }

    #[test]
    fn test_field51a_display() {
        let field = Field51A::new(None, None, "CHASUS33XXX").unwrap();
        assert_eq!(format!("{}", field), "CHASUS33XXX");

        let field = Field51A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(format!("{}", field), "/1234567890 DEUTDEFF500");

        let field = Field51A::new(
            Some("C".to_string()),
            Some("1234567890".to_string()),
            "HSBCHKHH",
        )
        .unwrap();
        assert_eq!(format!("{}", field), "/C/1234567890 HSBCHKHH");
    }

    #[test]
    fn test_field51a_format_spec() {
        assert_eq!(Field51A::format_spec(), "[/1!a][/34x]4!a2!a2!c[3!c]");
    }
}
