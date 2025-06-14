use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 54B: Receiver's Correspondent (Option B)
///
/// ## Overview
/// Field 54B identifies the receiver's correspondent institution using a party identifier
/// rather than a BIC code. This option is used when the receiver's correspondent institution
/// needs to be identified through an alternative identification scheme, such as a national
/// bank code, clearing code, or proprietary identifier system. This field is essential for
/// routing payments through correspondent networks when BIC codes are not available or sufficient.
///
/// ## Format Specification
/// **Format**: `[/1!a][/34x]35x`
/// - **1!a**: Optional account line indicator (1 character)
/// - **34x**: Optional account number (up to 34 characters)
/// - **35x**: Party identifier (up to 35 characters)
///
/// ## Structure
/// ```text
/// /C/9876543210
/// FEDWIRE021000021
/// │││ │         │
/// │││ │         └─ Party identifier (routing number)
/// │││ └─────────── Account number
/// ││└─────────────── Account separator
/// │└──────────────── Account line indicator
/// └───────────────── Field separator
/// ```
///
/// ## Field Components
/// - **Account Line Indicator**: Optional qualifier for account type
/// - **Account Number**: Institution's account for settlement
/// - **Party Identifier**: Alternative identification code or number
///
/// ## Usage Context
/// Field 54B is used in:
/// - **MT103**: Single Customer Credit Transfer (when BIC not available)
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Non-SWIFT institutions**: Identifying institutions without BIC codes
/// - **Domestic clearing**: Using national clearing codes or bank numbers
/// - **Regional networks**: Supporting regional payment network identifiers
/// - **Legacy systems**: Interfacing with older identification schemes
/// - **Regulatory requirements**: Meeting local identification standards
/// - **Correspondent routing**: Directing payments through specific correspondents
///
/// ## Examples
/// ```text
/// :54B:FEDWIRE021000021
/// └─── US Federal Reserve routing number
///
/// :54B:/C/9876543210
/// UKSC123456
/// └─── UK Sort Code with correspondent account
///
/// :54B:/S/SETTLEMENT009876543210
/// CANCLEAR005678
/// └─── Canadian clearing number with settlement account
///
/// :54B:CHIPS0456
/// └─── CHIPS participant identifier
/// ```
///
/// ## Party Identifier Types
/// Common party identifier formats for receiver's correspondents:
/// - **FEDWIRE**: US Federal Reserve routing numbers (9 digits)
/// - **UKSC**: UK Sort Codes (6 digits)
/// - **CANCLEAR**: Canadian clearing numbers
/// - **CHIPS**: Clearing House Interbank Payments System IDs
/// - **TARGET2**: European TARGET2 participant codes
/// - **CNAPS**: China National Advanced Payment System codes
/// - **RTGS**: Real-time gross settlement system codes
///
/// ## Account Line Indicators
/// Common indicators for receiver's correspondent accounts:
/// - **C**: Correspondent account (checking)
/// - **D**: Deposit account
/// - **S**: Settlement account
/// - **N**: Nostro account (our account with them)
/// - **V**: Vostro account (their account with us)
/// - **L**: Liquidity management account
/// - **R**: Reserve account
///
/// ## Validation Rules
/// 1. **Party identifier**: Cannot be empty, max 35 characters
/// 2. **Account line indicator**: If present, exactly 1 character
/// 3. **Account number**: If present, max 34 characters
/// 4. **Character validation**: All components must be printable ASCII
/// 5. **Content requirement**: Must contain meaningful identification
/// 6. **Format consistency**: Components must be properly structured
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Party identifier cannot be empty (Error: T11)
/// - Party identifier cannot exceed 35 characters (Error: T14)
/// - Account line indicator must be single character (Error: T12)
/// - Account number cannot exceed 34 characters (Error: T15)
/// - Characters must be from SWIFT character set (Error: T61)
/// - Field 54B alternative to 54A when BIC not available (Error: C54)
/// - Party identifier must be recognizable by receiver (Error: C55)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field54B {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Party identifier (up to 35 characters)
    pub party_identifier: String,
}

impl Field54B {
    /// Create a new Field54B with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        party_identifier: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let party_identifier = party_identifier.into().trim().to_string();

        // Validate party identifier
        if party_identifier.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54B".to_string(),
                message: "Party identifier cannot be empty".to_string(),
            });
        }

        if party_identifier.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54B".to_string(),
                message: "Party identifier cannot exceed 35 characters".to_string(),
            });
        }

        if !party_identifier
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54B".to_string(),
                message: "Party identifier contains invalid characters".to_string(),
            });
        }

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54B".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54B".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54B".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54B".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54B".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "54B".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        Ok(Field54B {
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

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "Receiver's Correspondent (Party ID: {})",
            self.party_identifier
        )
    }
}

impl SwiftField for Field54B {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54B".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":54B:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("54B:") {
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
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "54B".to_string(),
                message: "Party identifier is required".to_string(),
            });
        }

        Field54B::new(account_line_indicator, account_number, party_identifier)
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
        result.push_str(&self.party_identifier);

        format!(":54B:{}", result)
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

impl std::fmt::Display for Field54B {
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
    fn test_field54b_creation_party_only() {
        let field = Field54B::new(None, None, "RCVRPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "RCVRPARTYID123");
        assert!(field.account_number().is_none());
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field54b_creation_with_account() {
        let field = Field54B::new(None, Some("9876543210".to_string()), "RCVRPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "RCVRPARTYID123");
        assert_eq!(field.account_number(), Some("9876543210"));
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field54b_creation_with_account_line_indicator() {
        let field = Field54B::new(
            Some("B".to_string()),
            Some("9876543210".to_string()),
            "RCVRPARTYID123",
        )
        .unwrap();
        assert_eq!(field.party_identifier(), "RCVRPARTYID123");
        assert_eq!(field.account_number(), Some("9876543210"));
        assert_eq!(field.account_line_indicator(), Some("B"));
    }

    #[test]
    fn test_field54b_parse_party_only() {
        let field = Field54B::parse("RCVRPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "RCVRPARTYID123");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field54b_parse_with_account() {
        let field = Field54B::parse("/9876543210\nRCVRPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "RCVRPARTYID123");
        assert_eq!(field.account_number(), Some("9876543210"));
    }

    #[test]
    fn test_field54b_parse_with_tag() {
        let field = Field54B::parse(":54B:RCVRPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "RCVRPARTYID123");
    }

    #[test]
    fn test_field54b_to_swift_string() {
        let field = Field54B::new(None, None, "RCVRPARTYID123").unwrap();
        assert_eq!(field.to_swift_string(), ":54B:RCVRPARTYID123");

        let field = Field54B::new(None, Some("9876543210".to_string()), "RCVRPARTYID123").unwrap();
        assert_eq!(field.to_swift_string(), ":54B:/9876543210\nRCVRPARTYID123");
    }

    #[test]
    fn test_field54b_display() {
        let field = Field54B::new(None, None, "RCVRPARTYID123").unwrap();
        assert_eq!(format!("{}", field), "Party: RCVRPARTYID123");

        let field = Field54B::new(None, Some("9876543210".to_string()), "RCVRPARTYID123").unwrap();
        assert_eq!(
            format!("{}", field),
            "Account: 9876543210, Party: RCVRPARTYID123"
        );
    }

    #[test]
    fn test_field54b_description() {
        let field = Field54B::new(None, None, "RCVRPARTYID123").unwrap();
        assert_eq!(
            field.description(),
            "Receiver's Correspondent (Party ID: RCVRPARTYID123)"
        );
    }

    #[test]
    fn test_field54b_validation_empty_party() {
        let result = Field54B::new(None, None, "");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field54b_validation_party_too_long() {
        let party = "A".repeat(36); // 36 characters, max is 35
        let result = Field54B::new(None, None, party);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 35 characters")
        );
    }

    #[test]
    fn test_field54b_validation_invalid_characters() {
        let result = Field54B::new(None, None, "PARTY\x00ID"); // Contains null character
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field54b_validation_account_too_long() {
        let account = "A".repeat(35); // 35 characters, max is 34
        let result = Field54B::new(None, Some(account), "RCVRPARTYID123");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 34 characters")
        );
    }

    #[test]
    fn test_field54b_validate() {
        let field = Field54B::new(None, None, "RCVRPARTYID123").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
