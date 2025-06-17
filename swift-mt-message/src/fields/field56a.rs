use crate::common::BIC;
use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 56A: Intermediary Institution
///
/// ## Overview
/// Field 56A identifies an intermediary institution in SWIFT payment messages using a BIC code.
/// This field specifies a financial institution that acts as an intermediary in the payment
/// routing chain, facilitating the transfer of funds between the ordering institution and the
/// account with institution. Intermediary institutions play a crucial role in correspondent
/// banking networks and cross-border payment processing.
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
/// CHASUS33XXX
/// │       │││
/// │       │└┴┴ Branch code (optional, XXX)
/// │       └┴── Location code (2 chars, 33)
/// │     └┴──── Country code (2 chars, US)
/// │ └┴┴┴────── Bank code (4 chars, CHAS)
/// └─────────── Account number (optional)
/// ```
///
/// ## Field Components
/// - **Account Number**: Intermediary's account for settlement (optional)
/// - **BIC Code**: Business Identifier Code for intermediary identification
/// - **Bank Code**: 4-letter code identifying the intermediary bank
/// - **Country Code**: 2-letter ISO country code
/// - **Location Code**: 2-character location identifier
/// - **Branch Code**: 3-character branch identifier (optional)
///
/// ## Usage Context
/// Field 56A is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Payment routing**: Directing payments through intermediary banks
/// - **Correspondent banking**: Managing correspondent relationships
/// - **Cross-border payments**: Facilitating international transfers
/// - **Settlement optimization**: Using intermediaries for efficient settlement
/// - **Regulatory compliance**: Meeting routing requirements
/// - **Risk management**: Diversifying counterparty exposure
///
/// ## Examples
/// ```text
/// :56A:CHASUS33
/// └─── JPMorgan Chase Bank, New York (intermediary)
///
/// :56A:/INTERMEDIARYACCT123456
/// DEUTDEFF500
/// └─── Deutsche Bank AG, Frankfurt with intermediary account
///
/// :56A:BARCGB22
/// └─── Barclays Bank PLC, London (8-character BIC)
///
/// :56A:/NOSTRO001
/// BNPAFRPP
/// └─── BNP Paribas, Paris with nostro account
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
/// - **Content**: Intermediary account number or identifier
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
/// - Field 56A alternative to 56C/56D (Error: C56)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field56A {
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

impl Field56A {
    /// Create a new Field56A with validation
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
                    field_tag: "56A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Parse and validate BIC using the common structure
        let parsed_bic = BIC::parse(&bic, Some("56A"))?;

        Ok(Field56A {
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

    /// Get human-readable description
    pub fn description(&self) -> String {
        match &self.account_number {
            Some(account) => format!(
                "Intermediary Institution: {} ({})",
                self.bic.value(),
                account
            ),
            None => format!("Intermediary Institution: {}", self.bic.value()),
        }
    }
}

impl SwiftField for Field56A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":56A:") {
            stripped
        } else if let Some(stripped) = value.strip_prefix("56A:") {
            stripped
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "56A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let mut account_number = None;
        let bic;

        if content.starts_with('/') {
            let lines: Vec<&str> = content.lines().collect();

            if lines.len() == 1 {
                let parts: Vec<&str> = lines[0].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    account_number = Some(parts[0][1..].to_string());
                    bic = parts[1].to_string();
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "56A".to_string(),
                        message: "Invalid format: expected account and BIC".to_string(),
                    });
                }
            } else if lines.len() == 2 {
                account_number = Some(lines[0][1..].to_string());
                bic = lines[1].to_string();
            } else {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "56A".to_string(),
                    message: "Invalid format: too many lines".to_string(),
                });
            }
        } else {
            bic = content.to_string();
        }

        Self::new(None, account_number, bic)
    }

    fn to_swift_string(&self) -> String {
        match &self.account_number {
            Some(account) => format!(":56A:/{}\n{}", account, self.bic.value()),
            None => format!(":56A:{}", self.bic.value()),
        }
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        if let Some(ref account) = self.account_number {
            if account.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "56A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "56A".to_string(),
                    expected: "max 34 characters".to_string(),
                    actual: account.len(),
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
        "[/34x]4!a2!a2!c[3!c]"
    }
}

impl std::fmt::Display for Field56A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.account_number {
            Some(account) => write!(f, "/{} {}", account, self.bic.value()),
            None => write!(f, "{}", self.bic.value()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field56a_creation() {
        let field = Field56A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field56a_with_account() {
        let field = Field56A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field56a_parse() {
        let field = Field56A::parse("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field56a_to_swift_string() {
        let field = Field56A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.to_swift_string(), ":56A:DEUTDEFF");
    }

    #[test]
    fn test_field56a_validation() {
        let field = Field56A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
    }
}
