use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 54A: Receiver's Correspondent
///
/// ## Overview
/// Field 54A identifies the receiver's correspondent institution in SWIFT payment messages.
/// This field specifies the financial institution that acts as a correspondent for the
/// message receiver, facilitating the payment routing and settlement process on the
/// receiving side. The correspondent relationship is essential for cross-border payments
/// and international correspondent banking arrangements.
///
/// ## Format Specification
/// **Format**: `[/34x]4!a2!a2!c[3!c]`
/// - **34x**: Optional account number (up to 34 characters)
/// - **4!a2!a2!c[3!c]**: BIC code (8 or 11 characters)
///   - **4!a**: Bank code (4 alphabetic characters)
///   - **2!a**: Country code (2 alphabetic characters, ISO 3166-1)
///   - **2!c**: Location code (2 alphanumeric characters)
///   - **3!c**: Optional branch code (3 alphanumeric characters)
///
/// ## Structure
/// ```text
/// /1234567890123456789012345678901234
/// DEUTDEFF500
/// │       │││
/// │       │└┴─ Branch code (optional, 500)
/// │       └┴── Location code (2 chars, FF)
/// │     └┴──── Country code (2 chars, DE)
/// │ └┴┴┴────── Bank code (4 chars, DEUT)
/// └─────────── Account number (optional)
/// ```
///
/// ## Field Components
/// - **Account Number**: Institution's account for settlement (optional)
/// - **BIC Code**: Business Identifier Code for institution identification
/// - **Bank Code**: 4-letter code identifying the bank
/// - **Country Code**: 2-letter ISO country code
/// - **Location Code**: 2-character location identifier
/// - **Branch Code**: 3-character branch identifier (optional)
///
/// ## Usage Context
/// Field 54A is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Correspondent banking**: Establishing correspondent relationships
/// - **Payment routing**: Directing payments through correspondent networks
/// - **Settlement coordination**: Managing settlement between institutions
/// - **Cross-border payments**: Facilitating international transfers
/// - **Liquidity management**: Optimizing correspondent account balances
/// - **Risk management**: Identifying counterparty institutions
///
/// ## Examples
/// ```text
/// :54A:CHASUS33
/// └─── JPMorgan Chase Bank, New York (BIC only)
///
/// :54A:/1234567890123456789012345678901234
/// DEUTDEFF500
/// └─── Deutsche Bank AG, Frankfurt with account number
///
/// :54A:BARCGB22
/// └─── Barclays Bank PLC, London (8-character BIC)
///
/// :54A:/CORRESPONDENT001
/// BNPAFRPP
/// └─── BNP Paribas, Paris with correspondent account
/// ```
///
/// ## BIC Code Structure
/// - **8-character BIC**: BANKCCLL (Bank-Country-Location)
/// - **11-character BIC**: BANKCCLLBBB (Bank-Country-Location-Branch)
/// - **Bank Code**: 4 letters identifying the institution
/// - **Country Code**: 2 letters (ISO 3166-1 alpha-2)
/// - **Location Code**: 2 alphanumeric characters
/// - **Branch Code**: 3 alphanumeric characters (optional)
///
/// ## Account Number Guidelines
/// - **Format**: Up to 34 alphanumeric characters
/// - **Content**: Correspondent account number or identifier
/// - **Usage**: When specific account designation is required
/// - **Omission**: When only institution identification is needed
///
/// ## Validation Rules
/// 1. **BIC format**: Must be valid 8 or 11 character BIC code
/// 2. **Bank code**: Must be 4 alphabetic characters
/// 3. **Country code**: Must be 2 alphabetic characters
/// 4. **Location code**: Must be 2 alphanumeric characters
/// 5. **Branch code**: Must be 3 alphanumeric characters (if present)
/// 6. **Account number**: Maximum 34 characters (if present)
/// 7. **Character validation**: All components must be printable ASCII
///
/// ## Network Validated Rules (SWIFT Standards)
/// - BIC must be valid and registered in SWIFT network (Error: T10)
/// - BIC format must comply with ISO 13616 standards (Error: T11)
/// - Account number cannot exceed 34 characters (Error: T14)
/// - Bank code must be alphabetic only (Error: T15)
/// - Country code must be valid ISO 3166-1 code (Error: T16)
/// - Location code must be alphanumeric (Error: T17)
/// - Branch code must be alphanumeric if present (Error: T18)
/// - Field 54A alternative to 54B/54D (Error: C54)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field54A {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

impl Field54A {
    /// Create a new Field54A with validation
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
                    field_tag: "54A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Validate BIC
        Self::validate_bic(&bic)?;

        Ok(Field54A {
            account_line_indicator,
            account_number,
            bic: bic.to_string(),
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
        &self.bic
    }

    /// Check if this is a full BIC (11 characters) or short BIC (8 characters)
    pub fn is_full_bic(&self) -> bool {
        self.bic.len() == 11
    }

    /// Validate BIC according to SWIFT standards
    fn validate_bic(bic: &str) -> Result<(), crate::ParseError> {
        if bic.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC cannot be empty".to_string(),
            });
        }

        if bic.len() != 8 && bic.len() != 11 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        let bank_code = &bic[0..4];
        let country_code = &bic[4..6];
        let location_code = &bic[6..8];

        if !bank_code.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC bank code (first 4 characters) must be alphabetic".to_string(),
            });
        }

        if !country_code
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC country code (characters 5-6) must be alphabetic".to_string(),
            });
        }

        if !location_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "BIC location code (characters 7-8) must be alphanumeric".to_string(),
            });
        }

        if bic.len() == 11 {
            let branch_code = &bic[8..11];
            if !branch_code
                .chars()
                .all(|c| c.is_alphanumeric() && c.is_ascii())
            {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "BIC branch code (characters 9-11) must be alphanumeric".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match &self.account_number {
            Some(account) => format!("Receiver's Correspondent: {} ({})", self.bic, account),
            None => format!("Receiver's Correspondent: {}", self.bic),
        }
    }
}

impl SwiftField for Field54A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":54A:") {
            stripped
        } else if let Some(stripped) = value.strip_prefix("54A:") {
            stripped
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let mut account_line_indicator = None;
        let mut account_number = None;
        let bic;

        if content.starts_with('/') {
            let lines: Vec<&str> = content.lines().collect();

            if lines.len() == 1 {
                let parts: Vec<&str> = lines[0].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    account_line_indicator = Some(parts[0][1..].to_string());
                    account_number = Some(parts[1].to_string());
                    bic = parts[1].to_string();
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "54A".to_string(),
                        message: "Invalid format: expected account and BIC".to_string(),
                    });
                }
            } else if lines.len() == 2 {
                account_line_indicator = Some(lines[0][1..].to_string());
                account_number = Some(lines[1].to_string());
                bic = lines[1].to_string();
            } else {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54A".to_string(),
                    message: "Invalid format: too many lines".to_string(),
                });
            }
        } else {
            bic = content.to_string();
        }

        Self::new(account_line_indicator, account_number, bic)
    }

    fn to_swift_string(&self) -> String {
        match &self.account_number {
            Some(account) => format!(":54A:/{}\n{}", account, self.bic),
            None => format!(":54A:{}", self.bic),
        }
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(ref account) = self.account_number {
            if account.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "54A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "54A".to_string(),
                    expected: "max 34 characters".to_string(),
                    actual: account.len(),
                });
            }
        }

        // Validate BIC
        if let Err(crate::ParseError::InvalidFieldFormat { message, .. }) =
            Self::validate_bic(&self.bic)
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "54A".to_string(),
                message,
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "[/34x]4!a2!a2!c[3!c]"
    }
}

impl std::fmt::Display for Field54A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.account_number {
            Some(account) => write!(f, "/{} {}", account, self.bic),
            None => write!(f, "{}", self.bic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field54a_creation() {
        let field = Field54A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field54a_with_account() {
        let field = Field54A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field54a_parse() {
        let field = Field54A::parse("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field54a_to_swift_string() {
        let field = Field54A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.to_swift_string(), ":54A:DEUTDEFF");
    }

    #[test]
    fn test_field54a_validation() {
        let field = Field54A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
    }
}
