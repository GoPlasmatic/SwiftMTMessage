use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 53B: Sender's Correspondent (Option B)
///
/// ## Overview
/// Field 53B identifies the sender's correspondent institution using a party identifier
/// rather than a BIC code. This option is used when the correspondent institution needs
/// to be identified through an alternative identification scheme, such as a national
/// bank code, clearing code, or proprietary identifier system.
///
/// ## Format Specification
/// **Format**: `[/1!a][/34x]35x`
/// - **1!a**: Optional account line indicator (1 character)
/// - **34x**: Optional account number (up to 34 characters)
/// - **35x**: Party identifier (up to 35 characters)
///
/// ## Structure
/// ```text
/// /C/1234567890
/// FEDWIRE021000021
/// │││          │
/// │││          └─ Party identifier (routing number)
/// ││└─────────────── Account number
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
/// Field 53B is used in:
/// - **MT103**: Single Customer Credit Transfer (when BIC not available)
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
///
/// ### Business Applications
/// - **Non-SWIFT institutions**: Identifying institutions without BIC codes
/// - **Domestic clearing**: Using national clearing codes or bank numbers
/// - **Regional networks**: Supporting regional payment network identifiers
/// - **Legacy systems**: Interfacing with older identification schemes
/// - **Regulatory requirements**: Meeting local identification standards
///
/// ## Examples
/// ```text
/// :53B:FEDWIRE021000021
/// └─── US Federal Reserve routing number
///
/// :53B:/C/1234567890
/// UKSC123456
/// └─── UK Sort Code with checking account
///
/// :53B:/S/SETTLEMENT001234567890
/// CANCLEAR001234
/// └─── Canadian clearing number with settlement account
///
/// :53B:CHIPS0123
/// └─── CHIPS participant identifier
/// ```
///
/// ## Party Identifier Types
/// Common party identifier formats:
/// - **FEDWIRE**: US Federal Reserve routing numbers (9 digits)
/// - **UKSC**: UK Sort Codes (6 digits)
/// - **CANCLEAR**: Canadian clearing numbers
/// - **CHIPS**: Clearing House Interbank Payments System IDs
/// - **TARGET2**: European TARGET2 participant codes
/// - **CNAPS**: China National Advanced Payment System codes
///
/// ## Account Line Indicators
/// Common indicators for correspondent accounts:
/// - **C**: Correspondent account (checking)
/// - **D**: Deposit account
/// - **S**: Settlement account
/// - **N**: Nostro account (our account with them)
/// - **V**: Vostro account (their account with us)
/// - **L**: Liquidity management account
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
/// - Field 53B alternative to 53A when BIC not available (Error: C53)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field53B {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Party identifier (up to 35 characters)
    pub party_identifier: String,
}

impl Field53B {
    /// Create a new Field53B with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        party_identifier: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let party_identifier = party_identifier.into().trim().to_string();

        // Validate party identifier
        if party_identifier.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53B".to_string(),
                message: "Party identifier cannot be empty".to_string(),
            });
        }

        if party_identifier.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53B".to_string(),
                message: "Party identifier cannot exceed 35 characters".to_string(),
            });
        }

        if !party_identifier
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53B".to_string(),
                message: "Party identifier contains invalid characters".to_string(),
            });
        }

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53B".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53B".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53B".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53B".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53B".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "53B".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        Ok(Field53B {
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
            "Sender's Correspondent (Party ID: {})",
            self.party_identifier
        )
    }
}

impl SwiftField for Field53B {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "53B".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":53B:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("53B:") {
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
                field_tag: "53B".to_string(),
                message: "Party identifier is required".to_string(),
            });
        }

        Field53B::new(account_line_indicator, account_number, party_identifier)
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

        format!(":53B:{}", result)
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

impl std::fmt::Display for Field53B {
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
    fn test_field53b_creation_party_only() {
        let field = Field53B::new(None, None, "PARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert!(field.account_number().is_none());
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field53b_creation_with_account() {
        let field = Field53B::new(None, Some("1234567890".to_string()), "PARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field53b_creation_with_account_line_indicator() {
        let field = Field53B::new(
            Some("A".to_string()),
            Some("1234567890".to_string()),
            "PARTYID12345",
        )
        .unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.account_line_indicator(), Some("A"));
    }

    #[test]
    fn test_field53b_parse_party_only() {
        let field = Field53B::parse("PARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field53b_parse_with_account() {
        let field = Field53B::parse("/1234567890\nPARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field53b_parse_with_tag() {
        let field = Field53B::parse(":53B:PARTYID12345").unwrap();
        assert_eq!(field.party_identifier(), "PARTYID12345");
    }

    #[test]
    fn test_field53b_to_swift_string() {
        let field = Field53B::new(None, None, "PARTYID12345").unwrap();
        assert_eq!(field.to_swift_string(), ":53B:PARTYID12345");

        let field = Field53B::new(None, Some("1234567890".to_string()), "PARTYID12345").unwrap();
        assert_eq!(field.to_swift_string(), ":53B:/1234567890\nPARTYID12345");
    }

    #[test]
    fn test_field53b_display() {
        let field = Field53B::new(None, None, "PARTYID12345").unwrap();
        assert_eq!(format!("{}", field), "Party: PARTYID12345");

        let field = Field53B::new(None, Some("1234567890".to_string()), "PARTYID12345").unwrap();
        assert_eq!(
            format!("{}", field),
            "Account: 1234567890, Party: PARTYID12345"
        );
    }

    #[test]
    fn test_field53b_description() {
        let field = Field53B::new(None, None, "PARTYID12345").unwrap();
        assert_eq!(
            field.description(),
            "Sender's Correspondent (Party ID: PARTYID12345)"
        );
    }

    #[test]
    fn test_field53b_validation_empty_party() {
        let result = Field53B::new(None, None, "");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field53b_validation_party_too_long() {
        let party = "A".repeat(36); // 36 characters, max is 35
        let result = Field53B::new(None, None, party);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 35 characters")
        );
    }

    #[test]
    fn test_field53b_validation_invalid_characters() {
        let result = Field53B::new(None, None, "PARTY\x00ID"); // Contains null character
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field53b_validation_account_too_long() {
        let account = "A".repeat(35); // 35 characters, max is 34
        let result = Field53B::new(None, Some(account), "PARTYID12345");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 34 characters")
        );
    }

    #[test]
    fn test_field53b_validate() {
        let field = Field53B::new(None, None, "PARTYID12345").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
