//! # Swift MT Message Parser - Enhanced Architecture
//!
//! A comprehensive Rust library for parsing SWIFT MT (Message Type) messages with strong typing,
//! complex field structures, comprehensive validation, and flattened JSON serialization.
//!
//! ## Key Features
//!
//! - **Complex Field Structures**: Full enum-based field variants (Field50: A/F/K, Field59: A/Basic)
//! - **Flattened JSON Serialization**: Clean JSON output without enum wrapper layers
//! - **Type-safe field parsing** with dedicated field structs and automatic validation
//! - **Comprehensive Field Support**: All MT103 fields with proper SWIFT compliance
//! - **Bidirectional Serialization**: Perfect round-trip JSON serialization/deserialization
//! - **Extensive Validation**: BIC validation, field length checks, format compliance
//!
//! ## Supported Field Types
//!
//! ### Complex Enum Fields
//! - **Field50** (Ordering Customer): 50A (Account+BIC), 50F (Party+Address), 50K (Name+Address)
//! - **Field59** (Beneficiary Customer): 59A (Account+BIC), 59 (Basic lines)
//!
//! ### Institution Fields (with account_line_indicator)
//! - **Field52A** (Ordering Institution): BIC + optional account + account_line_indicator
//! - **Field53A-57A** (Correspondent/Intermediary): All with account_line_indicator support
//!
//! ### Simple Type Fields
//! - **Field32A** (Value Date/Currency/Amount): NaiveDate + String + f64
//! - **Field20, 23B, 70, 71A**: Proper field name alignment with old version
//!
//! ## Example Usage
//!
//! ```rust
//! use swift_mt_message::{SwiftParser, SwiftMessage, messages::MT103};
//!
//! let raw_mt103 = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\n:52A:BANKDEFF\n:57A:DEUTDEFF\n:59A:/DE89370400440532013000\nDEUTDEFF\n:70:PAYMENT\n:71A:OUR\n-}";
//! let parsed: SwiftMessage<MT103> = SwiftParser::parse(raw_mt103)?;
//! let json_output = serde_json::to_string_pretty(&parsed)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## JSON Output Structure
//!
//! The library produces clean, flattened JSON without enum wrapper layers:
//!
//! ```json
//! {
//!   "50": {
//!     "name_and_address": ["JOHN DOE", "123 MAIN ST"]
//!   },
//!   "59": {
//!     "account": "DE89370400440532013000",
//!     "bic": "DEUTDEFFXXX"
//!   }
//! }
//! ```
//!
//! Instead of nested enum structures like `{"50": {"K": {...}}}`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

pub mod errors;
pub mod fields;
pub mod headers;
pub mod messages;
pub mod parser;

// Re-export core types
pub use errors::{ParseError, Result, ValidationError};
pub use headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
pub use parser::SwiftParser;

// Re-export derive macros
pub use swift_mt_message_macros::{SwiftField, SwiftMessage, swift_serde};

/// Simplified result type for SWIFT operations
pub type SwiftResult<T> = std::result::Result<T, crate::errors::ParseError>;

/// Core trait for all Swift field types
pub trait SwiftField: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {
    /// Parse field value from string representation
    fn parse(value: &str) -> Result<Self>
    where
        Self: Sized;

    /// Convert field back to SWIFT string format
    fn to_swift_string(&self) -> String;

    /// Validate field according to SWIFT format rules
    fn validate(&self) -> ValidationResult;

    /// Get field format specification
    fn format_spec() -> &'static str;
}

/// Core trait for Swift message types
pub trait SwiftMessageBody: Debug + Clone + Send + Sync + Serialize {
    /// Get the message type identifier (e.g., "103", "202")
    fn message_type() -> &'static str;

    /// Create from field map
    fn from_fields(fields: HashMap<String, Vec<String>>) -> SwiftResult<Self>
    where
        Self: Sized;

    /// Convert to field map
    fn to_fields(&self) -> HashMap<String, Vec<String>>;

    /// Get required field tags for this message type
    fn required_fields() -> Vec<&'static str>;

    /// Get optional field tags for this message type
    fn optional_fields() -> Vec<&'static str>;
}

/// Complete SWIFT message with headers and body
#[derive(Debug, Clone, Serialize)]
pub struct SwiftMessage<T: SwiftMessageBody> {
    /// Basic Header (Block 1)
    pub basic_header: BasicHeader,

    /// Application Header (Block 2)
    pub application_header: ApplicationHeader,

    /// User Header (Block 3) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_header: Option<UserHeader>,

    /// Trailer (Block 5) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailer: Option<Trailer>,

    /// Raw message blocks for preservation
    pub blocks: RawBlocks,

    /// Message type identifier
    pub message_type: String,

    /// Field order as they appeared in the original message
    pub field_order: Vec<String>,

    /// Parsed message body with typed fields
    pub fields: T,
}

/// Raw message blocks for preservation and reconstruction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawBlocks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block3: Option<String>,
    pub block4: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block5: Option<String>,
}

/// Validation result for field and message validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_error(error: ValidationError) -> Self {
        Self {
            is_valid: false,
            errors: vec![error],
            warnings: Vec::new(),
        }
    }

    pub fn with_errors(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }
}

/// Common data types used across multiple fields
pub mod common {
    use crate::ValidationResult;
    use crate::errors::{ParseError, ValidationError};
    use serde::{Deserialize, Serialize};

    /// SWIFT BIC (Bank Identifier Code) with comprehensive validation and utilities
    ///
    /// A Bank Identifier Code (BIC) is used to identify financial institutions in SWIFT messages.
    /// It consists of either 8 or 11 characters:
    /// - Bank Code (4 characters): Alphabetic
    /// - Country Code (2 characters): Alphabetic (ISO 3166-1 alpha-2)
    /// - Location Code (2 characters): Alphanumeric
    /// - Branch Code (3 characters, optional): Alphanumeric
    ///
    /// # Examples
    /// - `CHASUS33XXX` - Chase Bank, US, New York, Branch: XXX
    /// - `DEUTDEFF` - Deutsche Bank, Germany, Frankfurt (8-character format)
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub struct BIC {
        /// The full BIC code (8 or 11 characters)
        #[serde(rename = "bic")]
        pub value: String,
    }

    impl BIC {
        /// Create a new BIC with validation
        ///
        /// # Arguments
        /// * `value` - The BIC string to validate and store
        ///
        /// # Returns
        /// * `Result<BIC, ParseError>` - The validated BIC or an error
        ///
        /// # Examples
        /// ```
        /// use swift_mt_message::common::BIC;
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// let bic = BIC::new("CHASUS33XXX".to_string())?;
        /// assert_eq!(bic.bank_code(), "CHAS");
        /// # Ok(())
        /// # }
        /// ```
        pub fn new(value: impl Into<String>) -> Result<Self, ParseError> {
            let value = value.into().to_uppercase();
            let bic = Self { value };
            bic.validate_strict()?;
            Ok(bic)
        }

        /// Create a BIC without validation (for internal use)
        pub fn new_unchecked(value: impl Into<String>) -> Self {
            Self {
                value: value.into().to_uppercase(),
            }
        }

        /// Parse a BIC from a string with optional field tag context
        pub fn parse(value: &str, field_tag: Option<&str>) -> Result<Self, ParseError> {
            let value = value.trim().to_uppercase();

            if value.is_empty() {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: field_tag.unwrap_or("BIC").to_string(),
                    message: "BIC cannot be empty".to_string(),
                });
            }

            let bic = Self::new_unchecked(value);
            bic.validate_with_context(field_tag)?;
            Ok(bic)
        }

        /// Strict validation that returns ParseError for use in constructors
        fn validate_strict(&self) -> Result<(), ParseError> {
            self.validate_with_context(None)
        }

        /// Validation with field context for better error messages
        fn validate_with_context(&self, field_tag: Option<&str>) -> Result<(), ParseError> {
            let field_name = field_tag.unwrap_or("BIC");

            if self.value.len() != 8 && self.value.len() != 11 {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: field_name.to_string(),
                    message: "BIC must be 8 or 11 characters".to_string(),
                });
            }

            let bank_code = &self.value[0..4];
            let country_code = &self.value[4..6];
            let location_code = &self.value[6..8];

            // Validate bank code (4 alphabetic characters)
            if !bank_code.chars().all(|c| c.is_ascii_alphabetic()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: field_name.to_string(),
                    message: "BIC bank code (first 4 characters) must be alphabetic".to_string(),
                });
            }

            // Validate country code (2 alphabetic characters)
            if !country_code.chars().all(|c| c.is_ascii_alphabetic()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: field_name.to_string(),
                    message: "BIC country code (characters 5-6) must be alphabetic".to_string(),
                });
            }

            // Validate location code (2 alphanumeric characters)
            if !location_code.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(ParseError::InvalidFieldFormat {
                    field_tag: field_name.to_string(),
                    message: "BIC location code (characters 7-8) must be alphanumeric".to_string(),
                });
            }

            // Validate branch code if present (3 alphanumeric characters)
            if self.value.len() == 11 {
                let branch_code = &self.value[8..11];
                if !branch_code.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(ParseError::InvalidFieldFormat {
                        field_tag: field_name.to_string(),
                        message: "BIC branch code (characters 9-11) must be alphanumeric"
                            .to_string(),
                    });
                }
            }

            Ok(())
        }

        /// Validate BIC and return ValidationResult for field validation
        pub fn validate(&self) -> ValidationResult {
            match self.validate_strict() {
                Ok(()) => ValidationResult::valid(),
                Err(ParseError::InvalidFieldFormat { message, .. }) => {
                    ValidationResult::with_error(ValidationError::FormatValidation {
                        field_tag: "BIC".to_string(),
                        message,
                    })
                }
                Err(e) => ValidationResult::with_error(ValidationError::FormatValidation {
                    field_tag: "BIC".to_string(),
                    message: e.to_string(),
                }),
            }
        }

        /// Get the full BIC value
        pub fn value(&self) -> &str {
            &self.value
        }

        /// Get the bank code (first 4 characters)
        pub fn bank_code(&self) -> &str {
            &self.value[0..4]
        }

        /// Get the country code (characters 5-6)
        pub fn country_code(&self) -> &str {
            &self.value[4..6]
        }

        /// Get the location code (characters 7-8)
        pub fn location_code(&self) -> &str {
            &self.value[6..8]
        }

        /// Get the branch code if present (characters 9-11)
        pub fn branch_code(&self) -> Option<&str> {
            if self.value.len() == 11 {
                Some(&self.value[8..11])
            } else {
                None
            }
        }

        /// Check if this is a full BIC (11 characters) vs short BIC (8 characters)
        pub fn is_full_bic(&self) -> bool {
            self.value.len() == 11
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

        /// Check if this is a retail banking institution (heuristic based on common bank codes)
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

        /// Check if this institution's country supports real-time payments
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

        /// Get a human-readable description of this BIC
        pub fn description(&self) -> String {
            format!(
                "Bank: {} | Country: {} | Location: {} | Branch: {}",
                self.bank_code(),
                self.country_code(),
                self.location_code(),
                self.branch_code().unwrap_or("XXX (Head Office)")
            )
        }
    }

    impl std::fmt::Display for BIC {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl std::str::FromStr for BIC {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::parse(s, None)
        }
    }

    impl From<BIC> for String {
        fn from(bic: BIC) -> String {
            bic.value
        }
    }

    impl AsRef<str> for BIC {
        fn as_ref(&self) -> &str {
            &self.value
        }
    }

    /// SWIFT Currency Code (ISO 4217)
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Currency {
        pub code: String,
    }

    impl Currency {
        pub fn new(code: String) -> Self {
            Self {
                code: code.to_uppercase(),
            }
        }
    }

    /// SWIFT Amount with decimal handling
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Amount {
        pub value: String,
        pub decimal_places: u8,
    }

    impl Amount {
        pub fn new(value: String) -> Self {
            let decimal_places = if value.contains(',') {
                value.split(',').nth(1).map(|s| s.len() as u8).unwrap_or(0)
            } else {
                0
            };

            Self {
                value,
                decimal_places,
            }
        }

        pub fn to_decimal(&self) -> Result<f64, std::num::ParseFloatError> {
            self.value.replace(',', ".").parse()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bic_creation() {
            let bic = BIC::new("CHASUS33XXX").unwrap();
            assert_eq!(bic.value(), "CHASUS33XXX");
            assert_eq!(bic.bank_code(), "CHAS");
            assert_eq!(bic.country_code(), "US");
            assert_eq!(bic.location_code(), "33");
            assert_eq!(bic.branch_code(), Some("XXX"));
            assert!(bic.is_full_bic());
        }

        #[test]
        fn test_bic_short_format() {
            let bic = BIC::new("DEUTDEFF").unwrap();
            assert_eq!(bic.value(), "DEUTDEFF");
            assert_eq!(bic.bank_code(), "DEUT");
            assert_eq!(bic.country_code(), "DE");
            assert_eq!(bic.location_code(), "FF");
            assert_eq!(bic.branch_code(), None);
            assert!(!bic.is_full_bic());
        }

        #[test]
        fn test_bic_validation_errors() {
            // Too short
            assert!(BIC::new("CHAS").is_err());

            // Too long
            assert!(BIC::new("CHASUS33XXXX").is_err());

            // Invalid bank code (numeric)
            assert!(BIC::new("1234US33").is_err());

            // Invalid country code (numeric)
            assert!(BIC::new("CHAS22XX").is_err());

            // Invalid location code (special chars)
            assert!(BIC::new("CHASUS@#").is_err());
        }

        #[test]
        fn test_bic_case_normalization() {
            let bic = BIC::new("chasus33xxx").unwrap();
            assert_eq!(bic.value(), "CHASUS33XXX");
        }

        #[test]
        fn test_bic_utilities() {
            let bic = BIC::new("CHASUS33XXX").unwrap();
            assert!(bic.is_major_financial_center());
            assert!(bic.is_retail_bank());
            assert!(bic.supports_real_time_payments());
            assert_eq!(
                bic.regulatory_jurisdiction(),
                "Federal Reserve / OCC / FDIC"
            );
        }

        #[test]
        fn test_bic_display_and_description() {
            let bic = BIC::new("CHASUS33XXX").unwrap();
            assert_eq!(bic.to_string(), "CHASUS33XXX");
            assert!(bic.description().contains("CHAS"));
            assert!(bic.description().contains("US"));
        }

        #[test]
        fn test_bic_from_str() {
            let bic: BIC = "CHASUS33XXX".parse().unwrap();
            assert_eq!(bic.value(), "CHASUS33XXX");
        }
    }
}

/// Enumeration of all supported SWIFT message types for automatic parsing
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "message_type")]
pub enum ParsedSwiftMessage {
    #[serde(rename = "103")]
    MT103(Box<SwiftMessage<messages::MT103>>),
    #[serde(rename = "202")]
    MT202(Box<SwiftMessage<messages::MT202>>),
}

impl ParsedSwiftMessage {
    /// Get the message type as a string
    pub fn message_type(&self) -> &'static str {
        match self {
            ParsedSwiftMessage::MT103(_) => "103",
            ParsedSwiftMessage::MT202(_) => "202",
        }
    }

    /// Convert to a specific message type if it matches
    pub fn as_mt103(&self) -> Option<&SwiftMessage<messages::MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(msg),
            _ => None,
        }
    }

    pub fn as_mt202(&self) -> Option<&SwiftMessage<messages::MT202>> {
        match self {
            ParsedSwiftMessage::MT202(msg) => Some(msg),
            _ => None,
        }
    }

    /// Convert into a specific message type if it matches
    pub fn into_mt103(self) -> Option<SwiftMessage<messages::MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(*msg),
            _ => None,
        }
    }

    pub fn into_mt202(self) -> Option<SwiftMessage<messages::MT202>> {
        match self {
            ParsedSwiftMessage::MT202(msg) => Some(*msg),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::mt103::MT103;

    #[test]
    fn test_full_mt103_parsing() {
        let raw_message = r#"{1:F01BNPAFRPPXXX0000000000}{2:O1031234240101DEUTDEFFXXXX12345678952401011234N}{3:{103:EBA}}{4:
:20:FT21001234567890
:23B:CRED
:32A:240101USD1000,00
:50K:/1234567890
ACME CORPORATION
123 MAIN STREET
NEW YORK NY 10001
:52A:BNPAFRPPXXX
:57A:DEUTDEFFXXX
:59:/DE89370400440532013000
MUELLER GMBH
HAUPTSTRASSE 1
10115 BERLIN
:70:PAYMENT FOR INVOICE 12345
:71A:OUR
-}"#;

        let result = SwiftParser::parse::<MT103>(raw_message);
        assert!(result.is_ok(), "Parsing should succeed: {:?}", result.err());

        let parsed = result.unwrap();
        assert_eq!(parsed.message_type, "103");

        // Test JSON serialization
        let json = serde_json::to_string_pretty(&parsed);
        assert!(json.is_ok(), "JSON serialization should work");
        println!("Parsed MT103 JSON:\n{}", json.unwrap());
    }

    #[test]
    fn test_field_parsing() {
        use crate::fields::field20::Field20;

        let result = Field20::parse(":20:FT21001234567890");
        assert!(result.is_ok());

        let field = result.unwrap();
        assert_eq!(field.to_swift_string(), ":20:FT21001234567890");
    }
}
