use crate::{SwiftField, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 57B: Account With Institution (Option B)
///
/// ## Overview
/// Field 57B identifies the account with institution in SWIFT payment messages using a party
/// identifier. This field provides an alternative to BIC-based identification when the
/// beneficiary's bank is identified through a party identifier system. This option is
/// particularly useful for institutions that participate in specific clearing systems or
/// have established party identifier arrangements within correspondent banking networks.
///
/// ## Format Specification
/// **Format**: `[/1!a][/34x]35x`
/// - **1!a**: Optional account line indicator (1 character)
/// - **34x**: Optional account number (up to 34 characters)
/// - **35x**: Party identifier (up to 35 characters)
/// - **Structure**: Multi-line format with optional components
///
/// ## Structure
/// ```text
/// /D/1234567890123456789012345678901234
/// PARTYIDENTIFIER123456789012345678901234
/// │ │                                  │
/// │ └─ Account number (optional)       │
/// │                                    │
/// └─ Account line indicator (optional) │
///                                      │
/// └─────────────────────────────────────
///           Party identifier (required)
/// ```
///
/// ## Field Components
/// - **Account Line Indicator**: Single character indicator (optional)
/// - **Account Number**: Beneficiary's account identifier (optional)
/// - **Party Identifier**: Institution identification code (required)
///   - Maximum 35 characters
///   - Must comply with SWIFT character set
///
/// ## Usage Context
/// Field 57B is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Party identifier systems**: Using established identifier schemes
/// - **Clearing system integration**: Interfacing with clearing networks
/// - **Correspondent banking**: Party-based correspondent identification
/// - **Regional payments**: Supporting regional identifier systems
/// - **Cost optimization**: Reducing identification complexity
/// - **System interoperability**: Bridging different identifier systems
///
/// ## Examples
/// ```text
/// :57B:ACCOUNTWITHPARTYID123
/// └─── Simple party identifier for beneficiary's bank
///
/// :57B:/BENEFICIARYACCT987654321
/// ACCOUNTWITHPARTYID123
/// └─── Party identifier with beneficiary account
///
/// :57B:/D/SPECIALACCT123456
/// PARTYID789012345
/// └─── Party identifier with account line indicator and account
///
/// :57B:/IBAN12345678901234567890
/// CLEARINGPARTYID456
/// └─── Party identifier with IBAN account
/// ```
///
/// ## Party Identifier Types
/// - **Clearing codes**: National clearing system identifiers
/// - **Member codes**: Clearing system member identifiers
/// - **Registry codes**: Financial institution registry codes
/// - **Network identifiers**: Payment network specific codes
/// - **System codes**: Internal system identifiers
/// - **Custom identifiers**: Bilateral agreement identifiers
///
/// ## Account Line Indicator Usage
/// - **D**: Debit account indicator
/// - **C**: Credit account indicator
/// - **M**: Main account indicator
/// - **S**: Settlement account indicator
/// - **Other**: System-specific indicators
///
/// ## Validation Rules
/// 1. **Party identifier**: Required, maximum 35 characters
/// 2. **Account number**: Optional, maximum 34 characters
/// 3. **Account line indicator**: Optional, exactly 1 character
/// 4. **Character set**: SWIFT character set only
/// 5. **Format structure**: Must follow multi-line format rules
/// 6. **Content validation**: All components must be meaningful
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Party identifier cannot exceed 35 characters (Error: T50)
/// - Account number cannot exceed 34 characters (Error: T14)
/// - Account line indicator must be single character (Error: T26)
/// - Must use SWIFT character set only (Error: T61)
/// - Party identifier cannot be empty (Error: T13)
/// - Field 57B alternative to 57A/57C/57D (Error: C57)
/// - Party identifier must be valid for system (Error: T51)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field57B {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Party identifier (up to 35 characters)
    pub party_identifier: String,
}

impl Field57B {
    /// Create a new Field57B with validation
    pub fn new(
        account_line_indicator: Option<String>,
        account_number: Option<String>,
        party_identifier: impl Into<String>,
    ) -> Result<Self, crate::ParseError> {
        let party_identifier = party_identifier.into().trim().to_string();

        // Validate party identifier
        if party_identifier.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57B".to_string(),
                message: "Party identifier cannot be empty".to_string(),
            });
        }

        if party_identifier.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57B".to_string(),
                message: "Party identifier cannot exceed 35 characters".to_string(),
            });
        }

        if !party_identifier
            .chars()
            .all(|c| c.is_ascii() && !c.is_control())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57B".to_string(),
                message: "Party identifier contains invalid characters".to_string(),
            });
        }

        // Validate account line indicator if present
        if let Some(ref indicator) = account_line_indicator {
            if indicator.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57B".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57B".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57B".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57B".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57B".to_string(),
                    message: "Account number cannot exceed 34 characters".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "57B".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        Ok(Field57B {
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
            "Account With Institution (Party ID: {})",
            self.party_identifier
        )
    }
}

impl SwiftField for Field57B {
    fn parse(content: &str) -> crate::Result<Self> {
        let content = content.trim();
        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "57B".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let content = if let Some(stripped) = content.strip_prefix(":57B:") {
            stripped
        } else if let Some(stripped) = content.strip_prefix("57B:") {
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
                field_tag: "57B".to_string(),
                message: "Party identifier is required".to_string(),
            });
        }

        Field57B::new(account_line_indicator, account_number, party_identifier)
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

        format!(":57B:{}", result)
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

impl std::fmt::Display for Field57B {
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
    fn test_field57b_creation_party_only() {
        let field = Field57B::new(None, None, "ACCOUNTWITHPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "ACCOUNTWITHPARTYID123");
        assert!(field.account_number().is_none());
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field57b_creation_with_account() {
        let field = Field57B::new(
            None,
            Some("ACCT987654321".to_string()),
            "ACCOUNTWITHPARTYID123",
        )
        .unwrap();
        assert_eq!(field.party_identifier(), "ACCOUNTWITHPARTYID123");
        assert_eq!(field.account_number(), Some("ACCT987654321"));
        assert!(field.account_line_indicator().is_none());
    }

    #[test]
    fn test_field57b_creation_with_account_line_indicator() {
        let field = Field57B::new(
            Some("D".to_string()),
            Some("ACCT987654321".to_string()),
            "ACCOUNTWITHPARTYID123",
        )
        .unwrap();
        assert_eq!(field.party_identifier(), "ACCOUNTWITHPARTYID123");
        assert_eq!(field.account_number(), Some("ACCT987654321"));
        assert_eq!(field.account_line_indicator(), Some("D"));
    }

    #[test]
    fn test_field57b_parse_party_only() {
        let field = Field57B::parse("ACCOUNTWITHPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "ACCOUNTWITHPARTYID123");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field57b_parse_with_account() {
        let field = Field57B::parse("/ACCT987654321\nACCOUNTWITHPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "ACCOUNTWITHPARTYID123");
        assert_eq!(field.account_number(), Some("ACCT987654321"));
    }

    #[test]
    fn test_field57b_parse_with_tag() {
        let field = Field57B::parse(":57B:ACCOUNTWITHPARTYID123").unwrap();
        assert_eq!(field.party_identifier(), "ACCOUNTWITHPARTYID123");
    }

    #[test]
    fn test_field57b_to_swift_string() {
        let field = Field57B::new(None, None, "ACCOUNTWITHPARTYID123").unwrap();
        assert_eq!(field.to_swift_string(), ":57B:ACCOUNTWITHPARTYID123");

        let field = Field57B::new(
            None,
            Some("ACCT987654321".to_string()),
            "ACCOUNTWITHPARTYID123",
        )
        .unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":57B:/ACCT987654321\nACCOUNTWITHPARTYID123"
        );
    }

    #[test]
    fn test_field57b_display() {
        let field = Field57B::new(None, None, "ACCOUNTWITHPARTYID123").unwrap();
        assert_eq!(format!("{}", field), "Party: ACCOUNTWITHPARTYID123");

        let field = Field57B::new(
            None,
            Some("ACCT987654321".to_string()),
            "ACCOUNTWITHPARTYID123",
        )
        .unwrap();
        assert_eq!(
            format!("{}", field),
            "Account: ACCT987654321, Party: ACCOUNTWITHPARTYID123"
        );
    }

    #[test]
    fn test_field57b_description() {
        let field = Field57B::new(None, None, "ACCOUNTWITHPARTYID123").unwrap();
        assert_eq!(
            field.description(),
            "Account With Institution (Party ID: ACCOUNTWITHPARTYID123)"
        );
    }

    #[test]
    fn test_field57b_validation_empty_party() {
        let result = Field57B::new(None, None, "");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field57b_validation_party_too_long() {
        let party = "A".repeat(36); // 36 characters, max is 35
        let result = Field57B::new(None, None, party);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 35 characters")
        );
    }

    #[test]
    fn test_field57b_validation_invalid_characters() {
        let result = Field57B::new(None, None, "PARTY\x00ID"); // Contains null character
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_field57b_validation_account_too_long() {
        let account = "A".repeat(35); // 35 characters, max is 34
        let result = Field57B::new(None, Some(account), "ACCOUNTWITHPARTYID123");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot exceed 34 characters")
        );
    }

    #[test]
    fn test_field57b_validate() {
        let field = Field57B::new(None, None, "ACCOUNTWITHPARTYID123").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }
}
