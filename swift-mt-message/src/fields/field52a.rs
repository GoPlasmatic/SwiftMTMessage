use crate::common::BIC;
use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 52A: Ordering Institution
///
/// ## Overview
/// Field 52A identifies the ordering institution in SWIFT payment messages. This field
/// specifies the financial institution that is acting on behalf of the ordering customer
/// (Field 50) to initiate the payment. It represents the first institution in the payment
/// chain and is crucial for routing, settlement, and compliance purposes.
///
/// ## Format Specification
/// **Format**: `[/1!a][/34x]4!a2!a2!c[3!c]`
/// - **1!a**: Optional account line indicator (1 character)
/// - **34x**: Optional account number (up to 34 characters)
/// - **4!a2!a2!c[3!c]**: BIC code (8 or 11 characters)
///
/// ### BIC Structure
/// ```text
/// DEUTDEFF500
/// ││││││││└┴┴─ Branch Code (3 characters, optional)
/// ││││││└┴──── Location Code (2 characters)
/// ││││└┴────── Country Code (2 letters)
/// └┴┴┴──────── Bank Code (4 letters)
/// ```
///
/// ## Field Components
/// - **Account Line Indicator**: Single character qualifier for account number type
/// - **Account Number**: Institution's account number for settlement
/// - **BIC**: Bank Identifier Code uniquely identifying the institution
///
/// ## Usage Context
/// Field 52A is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
///
/// ### Business Applications
/// - **Payment routing**: Identifying the institution to route payment through
/// - **Settlement**: Providing account information for settlement processes
/// - **Compliance**: Meeting regulatory requirements for institution identification
/// - **Correspondent banking**: Managing relationships between correspondent banks
/// - **Risk management**: Assessing counterparty risk and limits
///
/// ## Examples
/// ```text
/// :52A:DEUTDEFFXXX
/// └─── Deutsche Bank Frankfurt (no account information)
///
/// :52A:/C/1234567890
/// CHASUS33XXX
/// └─── JPMorgan Chase New York with checking account 1234567890
///
/// :52A:/A/GB12ABCD12345678901234
/// ABCDEFGHJKL
/// └─── Bank with account line indicator A and IBAN account
///
/// :52A:/S/SWIFT001234567890
/// BNPAFRPPXXX
/// └─── BNP Paribas with SWIFT account identifier
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
/// - Field 52A is mandatory in most payment message types (Error: C52)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field52A {
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

impl Field52A {
    /// Create a new Field52A with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        bic: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        // Parse and validate BIC using the common structure
        let bic = BIC::parse(&bic.into(), Some("52A"))?;

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account line indicator cannot be empty".to_string(),
                });
            }
            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }
            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account number cannot be empty".to_string(),
                });
            }
            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }
            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "52A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        Ok(Field52A {
            account_line_indicator,
            account_number,
            bic,
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

    /// Check if this is a full BIC (11 characters)
    pub fn is_full_bic(&self) -> bool {
        self.bic.is_full_bic()
    }
}

impl SwiftField for Field52A {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":52A:") {
            stripped // Remove ":52A:" prefix
        } else if let Some(stripped) = content.strip_prefix("52A:") {
            stripped // Remove "52A:" prefix
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
                    if parts.len() == 2 {
                        // /X/account format
                        account_line_indicator = Some(parts[0].to_string());
                        account_number = Some(parts[1].to_string());
                    } else {
                        // /account format
                        account_number = Some(parts[0].to_string());
                    }
                    bic_content = if lines.len() > 1 { lines[1] } else { "" };
                }
            }
        }

        let bic_str = bic_content.trim();
        if bic_str.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "52A".to_string(),
                message: "BIC code is required".to_string(),
            });
        }

        let bic = BIC::parse(bic_str, Some("52A"))?;

        Ok(Field52A {
            account_line_indicator,
            account_number,
            bic,
        })
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
        result.push_str(self.bic.value());

        format!(":52A:{}", result)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate BIC format using the common BIC validation
        let bic_validation = self.bic.validate();
        if !bic_validation.is_valid {
            errors.extend(bic_validation.errors);
        }

        // Additional validation for account components
        if let Some(indicator) = &self.account_line_indicator {
            if indicator.len() != 1 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "52A".to_string(),
                    expected: "1 character".to_string(),
                    actual: indicator.len(),
                });
            }
        }

        if let Some(account) = &self.account_number {
            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "52A".to_string(),
                    expected: "max 34 characters".to_string(),
                    actual: account.len(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field52a_creation_bic_only() {
        let field = Field52A::new(None, None, "BNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert!(field.account_number().is_none());
        assert!(field.account_line_indicator().is_none());
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field52a_creation_with_account() {
        let field = Field52A::new(None, Some("1234567890".to_string()), "BNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field52a_creation_with_account_line_indicator() {
        let field = Field52A::new(
            Some("A".to_string()),
            Some("1234567890".to_string()),
            "BNPAFRPPXXX",
        )
        .unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.account_line_indicator(), Some("A"));
    }

    #[test]
    fn test_field52a_parse_bic_only() {
        let field = Field52A::parse("BNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field52a_parse_with_account() {
        let field = Field52A::parse("/1234567890\nBNPAFRPPXXX").unwrap();
        assert_eq!(field.bic(), "BNPAFRPPXXX");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field52a_to_swift_string() {
        let field = Field52A::new(
            Some("A".to_string()),
            Some("1234567890".to_string()),
            "BNPAFRPPXXX",
        )
        .unwrap();
        assert_eq!(field.to_swift_string(), ":52A:/A/1234567890\nBNPAFRPPXXX");
    }
}
