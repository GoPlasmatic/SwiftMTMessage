use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 58A: Beneficiary Institution
///
/// ## Overview
/// Field 58A identifies the beneficiary institution in SWIFT payment messages using a BIC code.
/// This field specifies the financial institution that will receive the funds on behalf of
/// the beneficiary customer. The beneficiary institution is the final institution in the
/// payment chain that will credit the beneficiary's account with the transferred funds.
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
/// - **Account Number**: Institution's account for beneficiary funds (optional)
/// - **BIC Code**: Business Identifier Code for beneficiary institution identification
/// - **Bank Code**: 4-letter code identifying the beneficiary bank
/// - **Country Code**: 2-letter ISO country code
/// - **Location Code**: 2-character location identifier
/// - **Branch Code**: 3-character branch identifier (optional)
///
/// ## Usage Context
/// Field 58A is used in:
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
/// - **MT103**: Single Customer Credit Transfer (institutional beneficiary)
/// - **MT200**: Financial Institution Transfer
///
/// ### Business Applications
/// - **Final settlement**: Identifying the institution that will receive funds
/// - **Beneficiary banking**: Establishing the beneficiary's banking relationship
/// - **Account crediting**: Directing funds to the correct institution for crediting
/// - **Cross-border payments**: Facilitating international transfers to beneficiaries
/// - **Correspondent banking**: Managing final leg of correspondent banking chains
/// - **Regulatory compliance**: Meeting beneficiary identification requirements
/// - **Risk management**: Identifying final counterparty in payment chain
///
/// ## Examples
/// ```text
/// :58A:CHASUS33
/// └─── JPMorgan Chase Bank, New York (beneficiary institution)
///
/// :58A:/BENEFICIARY123456789012345678901234
/// DEUTDEFF500
/// └─── Deutsche Bank AG, Frankfurt with beneficiary account
///
/// :58A:BARCGB22
/// └─── Barclays Bank PLC, London (8-character BIC)
///
/// :58A:/FINAL001
/// BNPAFRPP
/// └─── BNP Paribas, Paris with final settlement account
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
/// - **Content**: Beneficiary institution's account number or identifier
/// - **Usage**: When specific account designation is required for settlement
/// - **Omission**: When only institution identification is needed
/// - **Purpose**: Facilitates direct settlement to specific account
///
/// ## Payment Chain Context
/// In the payment chain hierarchy:
/// - **Field 50**: Ordering Customer (originator)
/// - **Field 52A/D**: Ordering Institution (sender's bank)
/// - **Field 53A/B/D**: Sender's Correspondent (intermediate institution)
/// - **Field 54A/B/D**: Receiver's Correspondent (intermediate institution)
/// - **Field 55A/B/D**: Third Reimbursement Institution (intermediate institution)
/// - **Field 56A/C/D**: Intermediary Institution (intermediate institution)
/// - **Field 57A/B/C/D**: Account With Institution (final institution)
/// - **Field 58A/D**: Beneficiary Institution (final beneficiary bank)
/// - **Field 59**: Beneficiary Customer (final recipient)
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
/// - Field 58A alternative to 58D (Error: C58)
/// - Institution must be capable of receiving funds (Error: C59)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field58A {
    /// Account line indicator (optional, 1 character)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_line_indicator: Option<String>,
    /// Account number (optional, up to 34 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// BIC code (8 or 11 characters)
    pub bic: String,
}

impl Field58A {
    /// Create a new Field58A with validation
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
                    field_tag: "58A".to_string(),
                    message: "Account line indicator cannot be empty if specified".to_string(),
                });
            }

            if indicator.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "58A".to_string(),
                    message: "Account line indicator must be exactly 1 character".to_string(),
                });
            }

            if !indicator.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "58A".to_string(),
                    message: "Account line indicator contains invalid characters".to_string(),
                });
            }
        }

        // Validate account number if present
        if let Some(ref account) = account_number {
            if account.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "58A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "58A".to_string(),
                    message: "Account number too long (max 34 characters)".to_string(),
                });
            }

            if !account.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "58A".to_string(),
                    message: "Account number contains invalid characters".to_string(),
                });
            }
        }

        // Validate BIC
        Self::validate_bic(&bic)?;

        Ok(Field58A {
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

    /// Get the bank code component of the BIC
    pub fn bank_code(&self) -> &str {
        if self.bic.len() >= 4 {
            &self.bic[0..4]
        } else {
            ""
        }
    }

    /// Get the country code component of the BIC
    pub fn country_code(&self) -> &str {
        if self.bic.len() >= 6 {
            &self.bic[4..6]
        } else {
            ""
        }
    }

    /// Get the location code component of the BIC
    pub fn location_code(&self) -> &str {
        if self.bic.len() >= 8 {
            &self.bic[6..8]
        } else {
            ""
        }
    }

    /// Get the branch code component of the BIC (if present)
    pub fn branch_code(&self) -> Option<&str> {
        if self.bic.len() == 11 {
            Some(&self.bic[8..11])
        } else {
            None
        }
    }

    /// Validate BIC according to SWIFT standards
    fn validate_bic(bic: &str) -> Result<(), crate::ParseError> {
        if bic.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "58A".to_string(),
                message: "BIC cannot be empty".to_string(),
            });
        }

        if bic.len() != 8 && bic.len() != 11 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "58A".to_string(),
                message: "BIC must be 8 or 11 characters".to_string(),
            });
        }

        let bank_code = &bic[0..4];
        let country_code = &bic[4..6];
        let location_code = &bic[6..8];

        if !bank_code.chars().all(|c| c.is_alphabetic() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "58A".to_string(),
                message: "BIC bank code (first 4 characters) must be alphabetic".to_string(),
            });
        }

        if !country_code
            .chars()
            .all(|c| c.is_alphabetic() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "58A".to_string(),
                message: "BIC country code (characters 5-6) must be alphabetic".to_string(),
            });
        }

        if !location_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "58A".to_string(),
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
                    field_tag: "58A".to_string(),
                    message: "BIC branch code (characters 9-11) must be alphanumeric".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match &self.account_number {
            Some(account) => format!("Beneficiary Institution: {} ({})", self.bic, account),
            None => format!("Beneficiary Institution: {}", self.bic),
        }
    }

    /// Check if this institution is in a major financial center
    pub fn is_major_financial_center(&self) -> bool {
        let country = self.country_code();
        let location = self.location_code();

        matches!(
            (country, location),
            ("US", "33") | // New York
            ("GB", "22") | // London
            ("DE", "FF") | // Frankfurt
            ("JP", "22") | // Tokyo
            ("HK", "HK") | // Hong Kong
            ("SG", "SG") | // Singapore
            ("FR", "PP") | // Paris
            ("CH", "ZZ") | // Zurich
            ("CA", "TT") | // Toronto
            ("AU", "MM") // Melbourne/Sydney
        )
    }

    /// Check if this is a retail banking institution
    pub fn is_retail_bank(&self) -> bool {
        let bank_code = self.bank_code();

        // Common retail bank codes (this is a simplified check)
        matches!(
            bank_code,
            "CHAS" | // Chase
            "BOFA" | // Bank of America
            "WELL" | // Wells Fargo
            "CITI" | // Citibank
            "HSBC" | // HSBC
            "BARC" | // Barclays
            "LLOY" | // Lloyds
            "NATS" | // NatWest
            "DEUT" | // Deutsche Bank
            "COMM" | // Commerzbank
            "BNPA" | // BNP Paribas
            "CRED" | // Credit Agricole
            "UBSW" | // UBS
            "CRSU" | // Credit Suisse
            "ROYA" | // Royal Bank of Canada
            "TDOM" | // TD Bank
            "ANZI" | // ANZ
            "CTBA" | // Commonwealth Bank
            "WEST" | // Westpac
            "MUFG" | // MUFG Bank
            "SMBC" | // Sumitomo Mitsui
            "MIZB" // Mizuho Bank
        )
    }

    /// Check if this institution supports real-time payments
    pub fn supports_real_time_payments(&self) -> bool {
        let country = self.country_code();

        // Countries with major real-time payment systems
        matches!(
            country,
            "US" | // FedNow, RTP
            "GB" | // Faster Payments
            "DE" | // Instant Payments
            "NL" | // iDEAL
            "SE" | // Swish
            "DK" | // MobilePay
            "AU" | // NPP
            "SG" | // FAST
            "IN" | // UPI
            "BR" | // PIX
            "MX" | // SPEI
            "JP" | // Zengin
            "KR" | // KFTC
            "CN" // CIPS
        )
    }

    /// Get the regulatory jurisdiction for this institution
    pub fn regulatory_jurisdiction(&self) -> &'static str {
        match self.country_code() {
            "US" => "Federal Reserve / OCC / FDIC",
            "GB" => "Bank of England / PRA / FCA",
            "DE" => "BaFin / ECB",
            "FR" => "ACPR / ECB",
            "JP" => "JFSA / Bank of Japan",
            "CH" => "FINMA / SNB",
            "CA" => "OSFI / Bank of Canada",
            "AU" => "APRA / RBA",
            "SG" => "MAS",
            "HK" => "HKMA",
            "CN" => "PBOC / CBIRC",
            "IN" => "RBI",
            "BR" => "Central Bank of Brazil",
            "MX" => "CNBV / Banxico",
            _ => "Other National Authority",
        }
    }
}

impl SwiftField for Field58A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":58A:") {
            stripped
        } else if let Some(stripped) = value.strip_prefix("58A:") {
            stripped
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "58A".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        let account_line_indicator = None;
        let mut account_number = None;
        let bic;

        if content.starts_with('/') {
            let lines: Vec<&str> = content.lines().collect();

            if lines.len() == 1 {
                // Single line format: /account BIC
                let parts: Vec<&str> = lines[0].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    account_number = Some(parts[0][1..].to_string());
                    bic = parts[1].to_string();
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "58A".to_string(),
                        message: "Invalid format: expected account and BIC".to_string(),
                    });
                }
            } else if lines.len() == 2 {
                // Two line format: /account \n BIC
                account_number = Some(lines[0][1..].to_string());
                bic = lines[1].to_string();
            } else {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "58A".to_string(),
                    message: "Invalid format: too many lines".to_string(),
                });
            }
        } else {
            // BIC only
            bic = content.to_string();
        }

        Self::new(account_line_indicator, account_number, bic)
    }

    fn to_swift_string(&self) -> String {
        match &self.account_number {
            Some(account) => format!(":58A:/{}\n{}", account, self.bic),
            None => format!(":58A:{}", self.bic),
        }
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if let Some(ref account) = self.account_number {
            if account.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "58A".to_string(),
                    message: "Account number cannot be empty if specified".to_string(),
                });
            }

            if account.len() > 34 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "58A".to_string(),
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
                field_tag: "58A".to_string(),
                message,
            });
        }

        // Add business logic warnings
        if !self.is_major_financial_center() {
            warnings.push("Beneficiary institution is not in a major financial center".to_string());
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    fn format_spec() -> &'static str {
        "[/34x]4!a2!a2!c[3!c]"
    }
}

impl std::fmt::Display for Field58A {
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
    fn test_field58a_creation() {
        let field = Field58A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF");
        assert!(field.account_number().is_none());
        assert!(!field.is_full_bic());
    }

    #[test]
    fn test_field58a_with_account() {
        let field = Field58A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.bic(), "DEUTDEFF500");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field58a_with_account_line_indicator() {
        let field = Field58A::new(
            Some("A".to_string()),
            Some("1234567890".to_string()),
            "CHASUS33XXX",
        )
        .unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
        assert_eq!(field.account_number(), Some("1234567890"));
        assert_eq!(field.account_line_indicator(), Some("A"));
        assert!(field.is_full_bic());
    }

    #[test]
    fn test_field58a_parse_bic_only() {
        let field = Field58A::parse("CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
        assert!(field.account_number().is_none());
    }

    #[test]
    fn test_field58a_parse_with_account_single_line() {
        let field = Field58A::parse("/1234567890 CHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field58a_parse_with_account_two_lines() {
        let field = Field58A::parse("/1234567890\nCHASUS33XXX").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
        assert_eq!(field.account_number(), Some("1234567890"));
    }

    #[test]
    fn test_field58a_parse_with_prefix() {
        let field = Field58A::parse(":58A:CHASUS33").unwrap();
        assert_eq!(field.bic(), "CHASUS33");
    }

    #[test]
    fn test_field58a_to_swift_string_bic_only() {
        let field = Field58A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.to_swift_string(), ":58A:DEUTDEFF");
    }

    #[test]
    fn test_field58a_to_swift_string_with_account() {
        let field = Field58A::new(None, Some("1234567890".to_string()), "DEUTDEFF500").unwrap();
        assert_eq!(field.to_swift_string(), ":58A:/1234567890\nDEUTDEFF500");
    }

    #[test]
    fn test_field58a_bic_components() {
        let field = Field58A::new(None, None, "CHASUS33XXX").unwrap();
        assert_eq!(field.bank_code(), "CHAS");
        assert_eq!(field.country_code(), "US");
        assert_eq!(field.location_code(), "33");
        assert_eq!(field.branch_code(), Some("XXX"));
    }

    #[test]
    fn test_field58a_short_bic_components() {
        let field = Field58A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(field.bank_code(), "DEUT");
        assert_eq!(field.country_code(), "DE");
        assert_eq!(field.location_code(), "FF");
        assert_eq!(field.branch_code(), None);
    }

    #[test]
    fn test_field58a_invalid_bic_length() {
        let result = Field58A::new(None, None, "INVALID");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("8 or 11 characters")
        );
    }

    #[test]
    fn test_field58a_invalid_bic_format() {
        // Invalid bank code (contains numbers)
        let result = Field58A::new(None, None, "123AUSGG");
        assert!(result.is_err());

        // Invalid country code (contains numbers)
        let result = Field58A::new(None, None, "DEUT1EFF");
        assert!(result.is_err());
    }

    #[test]
    fn test_field58a_invalid_account() {
        // Account too long
        let long_account = "A".repeat(35);
        let result = Field58A::new(None, Some(long_account), "DEUTDEFF");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too long"));

        // Empty account
        let result = Field58A::new(None, Some("".to_string()), "DEUTDEFF");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_field58a_validation() {
        let field = Field58A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
    }

    #[test]
    fn test_field58a_display() {
        let field_bic_only = Field58A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field_bic_only), "DEUTDEFF");

        let field_with_account =
            Field58A::new(None, Some("1234567890".to_string()), "DEUTDEFF").unwrap();
        assert_eq!(format!("{}", field_with_account), "/1234567890 DEUTDEFF");
    }

    #[test]
    fn test_field58a_description() {
        let field = Field58A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(field.description(), "Beneficiary Institution: CHASUS33");

        let field_with_account =
            Field58A::new(None, Some("1234567890".to_string()), "CHASUS33").unwrap();
        assert_eq!(
            field_with_account.description(),
            "Beneficiary Institution: CHASUS33 (1234567890)"
        );
    }

    #[test]
    fn test_field58a_major_financial_center() {
        let ny_field = Field58A::new(None, None, "CHASUS33").unwrap();
        assert!(ny_field.is_major_financial_center());

        let london_field = Field58A::new(None, None, "BARCGB22").unwrap();
        assert!(london_field.is_major_financial_center());

        let small_field = Field58A::new(None, None, "TESTAA11").unwrap();
        assert!(!small_field.is_major_financial_center());
    }

    #[test]
    fn test_field58a_retail_bank() {
        let chase_field = Field58A::new(None, None, "CHASUS33").unwrap();
        assert!(chase_field.is_retail_bank());

        let hsbc_field = Field58A::new(None, None, "HSBCGB2L").unwrap();
        assert!(hsbc_field.is_retail_bank());

        let unknown_field = Field58A::new(None, None, "TESTAA11").unwrap();
        assert!(!unknown_field.is_retail_bank());
    }

    #[test]
    fn test_field58a_real_time_payments() {
        let us_field = Field58A::new(None, None, "CHASUS33").unwrap();
        assert!(us_field.supports_real_time_payments());

        let uk_field = Field58A::new(None, None, "BARCGB22").unwrap();
        assert!(uk_field.supports_real_time_payments());

        let other_field = Field58A::new(None, None, "TESTAA11").unwrap();
        assert!(!other_field.supports_real_time_payments());
    }

    #[test]
    fn test_field58a_regulatory_jurisdiction() {
        let us_field = Field58A::new(None, None, "CHASUS33").unwrap();
        assert_eq!(
            us_field.regulatory_jurisdiction(),
            "Federal Reserve / OCC / FDIC"
        );

        let uk_field = Field58A::new(None, None, "BARCGB22").unwrap();
        assert_eq!(
            uk_field.regulatory_jurisdiction(),
            "Bank of England / PRA / FCA"
        );

        let de_field = Field58A::new(None, None, "DEUTDEFF").unwrap();
        assert_eq!(de_field.regulatory_jurisdiction(), "BaFin / ECB");

        let other_field = Field58A::new(None, None, "TESTAA11").unwrap();
        assert_eq!(
            other_field.regulatory_jurisdiction(),
            "Other National Authority"
        );
    }

    #[test]
    fn test_field58a_case_normalization() {
        let field = Field58A::new(None, None, "chasus33xxx").unwrap();
        assert_eq!(field.bic(), "CHASUS33XXX");
    }

    #[test]
    fn test_field58a_format_spec() {
        assert_eq!(Field58A::format_spec(), "[/34x]4!a2!a2!c[3!c]");
    }

    #[test]
    fn test_field58a_serialization() {
        let field = Field58A::new(None, Some("1234567890".to_string()), "CHASUS33XXX").unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&field).unwrap();
        let deserialized: Field58A = serde_json::from_str(&json).unwrap();

        assert_eq!(field, deserialized);
        assert_eq!(field.bic(), deserialized.bic());
        assert_eq!(field.account_number(), deserialized.account_number());
    }

    #[test]
    fn test_field58a_edge_cases() {
        // Test minimum BIC length
        let field = Field58A::new(None, None, "TESTAA11").unwrap();
        assert_eq!(field.bic().len(), 8);

        // Test maximum BIC length
        let field = Field58A::new(None, None, "TESTAA11XXX").unwrap();
        assert_eq!(field.bic().len(), 11);

        // Test maximum account length
        let max_account = "A".repeat(34);
        let field = Field58A::new(None, Some(max_account.clone()), "TESTAA11").unwrap();
        assert_eq!(field.account_number(), Some(max_account.as_str()));
    }

    #[test]
    fn test_field58a_comprehensive_validation() {
        // Test valid field
        let valid_field = Field58A::new(None, Some("1234567890".to_string()), "CHASUS33").unwrap();
        let validation = valid_field.validate();
        assert!(validation.is_valid);

        // Test invalid BIC
        let invalid_field = Field58A::new(None, None, "INVALID").unwrap_or_else(|_| {
            // This should fail in constructor, but if it somehow passes, validation should catch it
            Field58A {
                account_line_indicator: None,
                account_number: None,
                bic: "INVALID".to_string(),
            }
        });
        let validation = invalid_field.validate();
        assert!(!validation.is_valid);
    }

    #[test]
    fn test_field58a_business_logic_warnings() {
        let field = Field58A::new(None, None, "TESTAA11").unwrap();
        let validation = field.validate();

        // Should be valid but may have warnings
        assert!(validation.is_valid);
        // May have warnings about not being in major financial center
        // This depends on the implementation details
    }

    #[test]
    fn test_field58a_real_world_examples() {
        // Test major US bank
        let chase_field =
            Field58A::new(None, Some("1234567890".to_string()), "CHASUS33XXX").unwrap();
        assert_eq!(chase_field.bic(), "CHASUS33XXX");
        assert_eq!(chase_field.bank_code(), "CHAS");
        assert_eq!(chase_field.country_code(), "US");
        assert!(chase_field.is_major_financial_center());
        assert!(chase_field.is_retail_bank());
        assert!(chase_field.supports_real_time_payments());

        // Test major European bank
        let deutsche_field = Field58A::new(None, None, "DEUTDEFF500").unwrap();
        assert_eq!(deutsche_field.bic(), "DEUTDEFF500");
        assert_eq!(deutsche_field.bank_code(), "DEUT");
        assert_eq!(deutsche_field.country_code(), "DE");
        assert!(deutsche_field.is_major_financial_center());
        assert!(deutsche_field.is_retail_bank());
        assert!(deutsche_field.supports_real_time_payments());
    }
}
