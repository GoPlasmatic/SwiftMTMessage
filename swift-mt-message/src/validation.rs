//! # SWIFT Validation Utilities
//!
//! ## Purpose
//! Comprehensive validation functions for SWIFT-specific data types including BIC codes, currency codes,
//! account numbers, and other financial identifiers used in MT message processing.
//!
//! ## Features
//! - **BIC Code Validation**: Full SWIFT BIC format validation with ISO 3166-1 country code verification
//! - **Currency Code Validation**: ISO 4217 currency code validation for major global currencies
//! - **Format Validation**: SWIFT field format validation (4!c, 6!n, 35x, etc.)
//! - **Account Validation**: IBAN and generic account number format validation
//! - **Reference Validation**: Transaction reference and message reference format checking
//!
//! ## Validation Standards
//! - **BIC Codes**: ISO 9362 (SWIFT BIC) standard compliance
//! - **Currency Codes**: ISO 4217 standard with comprehensive currency support
//! - **Country Codes**: ISO 3166-1 alpha-2 standard for geographic validation
//! - **SWIFT Formats**: Official SWIFT User Handbook format specifications
//!
//! ## Usage Examples
//! ```rust
//! use swift_mt_message::validation::{is_valid_bic, is_valid_currency, validate_field_format};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // BIC validation
//! assert!(is_valid_bic("DEUTDEFFXXX")); // Valid 11-character BIC
//! assert!(is_valid_bic("DEUTDEFF"));    // Valid 8-character BIC
//! assert!(!is_valid_bic("INVALID"));    // Invalid format
//!
//! // Currency validation
//! assert!(is_valid_currency("USD"));    // Valid ISO 4217 code
//! assert!(is_valid_currency("EUR"));    // Valid ISO 4217 code
//! assert!(!is_valid_currency("XYZ"));   // Invalid currency
//!
//! // SWIFT format validation using validate_field_format
//! // Note: validate_field_format has a different signature and returns Result
//! assert!(validate_field_format("DEUTDEFFXXX", "52A", "BIC", None).is_ok());
//! assert!(validate_field_format("USD", "32A", "currency", None).is_ok());
//! assert!(validate_field_format("240315", "32A", "date", None).is_ok());
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance
//! All validation functions use pre-compiled static data structures for optimal performance.
//! Currency and country code lookups use HashSet for O(1) validation time.

use crate::errors::{SwiftValidationError, SwiftValidationResult};
use crate::shared_validation::{VALID_COUNTRY_CODES, VALID_CURRENCIES};
use crate::swift_error_codes::{currencies, t_series};

/// Validate a BIC (Bank Identifier Code) according to SWIFT standards
///
/// BIC format: AAAA BB CC DDD
/// - AAAA: Institution Code (4 letters)
/// - BB: Country Code (2 letters, ISO 3166-1 alpha-2)
/// - CC: Location Code (2 alphanumeric)
/// - DDD: Branch Code (3 alphanumeric, optional)
///
/// Length: 8 characters (no branch) or 11 characters (with branch)
pub fn is_valid_bic(bic: &str) -> bool {
    crate::shared_validation::is_valid_bic_structure(bic)
}

/// Validate a currency code according to ISO 4217 standards
/// Currency codes are 3-letter uppercase codes
pub fn is_valid_currency(currency: &str) -> bool {
    if currency.len() != 3 {
        return false;
    }

    if !currency
        .chars()
        .all(|c| c.is_alphabetic() && c.is_uppercase())
    {
        return false;
    }

    crate::shared_validation::is_valid_currency(currency)
}

/// Validate an account number according to SWIFT standards
/// Account numbers can be up to 34 characters alphanumeric
pub fn is_valid_account_number(account: &str) -> bool {
    if account.is_empty() || account.len() > 34 {
        return false;
    }

    // Allow alphanumeric characters and some special characters
    account
        .chars()
        .all(|c| c.is_alphanumeric() || "/.,()-".contains(c))
}

/// Validate an amount format
/// SWIFT amounts are decimal numbers with up to 15 digits before decimal
/// and currency-specific decimal places
pub fn is_valid_amount(amount: &str) -> bool {
    if amount.is_empty() {
        return false;
    }

    // Parse as decimal number
    if let Ok(parsed) = amount.parse::<f64>() {
        if parsed < 0.0 {
            return false; // Negative amounts not allowed in most SWIFT contexts
        }

        // Check for reasonable limits (up to 999 trillion)
        if parsed >= 1_000_000_000_000_000.0 {
            return false;
        }

        // Check decimal places (max 5 for most currencies)
        if let Some(decimal_pos) = amount.find('.') {
            let decimal_part = &amount[decimal_pos + 1..];
            if decimal_part.len() > 5 {
                return false;
            }
        }

        true
    } else {
        false
    }
}

/// Validate a SWIFT date in YYMMDD format
pub fn is_valid_swift_date(date: &str) -> bool {
    if date.len() != 6 {
        return false;
    }

    if !date.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Basic range validation
    let _: u32 = date[0..2].parse().unwrap_or(99);
    let month: u32 = date[2..4].parse().unwrap_or(13);
    let day: u32 = date[4..6].parse().unwrap_or(32);

    if !(1..=12).contains(&month) {
        return false;
    }

    if !(1..=31).contains(&day) {
        return false;
    }

    // More sophisticated date validation could be added here
    // For now, this covers basic format validation
    true
}

/// Validate a SWIFT time in HHMM or HHMMSS format
pub fn is_valid_swift_time(time: &str) -> bool {
    if time.len() != 4 && time.len() != 6 {
        return false;
    }

    if !time.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let hour: u32 = time[0..2].parse().unwrap_or(25);
    let minute: u32 = time[2..4].parse().unwrap_or(61);

    if hour > 23 || minute > 59 {
        return false;
    }

    if time.len() == 6 {
        let second: u32 = time[4..6].parse().unwrap_or(61);
        if second > 59 {
            return false;
        }
    }

    true
}

/// Validate a reference field (typically 16 characters max, alphanumeric)
pub fn is_valid_reference(reference: &str) -> bool {
    if reference.is_empty() || reference.len() > 16 {
        return false;
    }

    // Allow alphanumeric characters and common separators
    reference
        .chars()
        .all(|c| c.is_alphanumeric() || "/.,()-".contains(c))
}

/// Generate a valid sample BIC for testing
pub fn generate_sample_bic() -> String {
    "DEUTDEFFXXX".to_string()
}

/// Generate a valid sample currency for testing
pub fn generate_sample_currency() -> String {
    "USD".to_string()
}

/// Generate a valid sample account number for testing
pub fn generate_sample_account() -> String {
    "GB29NWBK60161331926819".to_string()
}

/// Generate a valid sample amount for testing
pub fn generate_sample_amount() -> String {
    "1234.56".to_string()
}

/// Generate a valid sample date for testing
pub fn generate_sample_date() -> String {
    "231215".to_string() // December 15, 2023
}

/// Generate a valid sample reference for testing
pub fn generate_sample_reference() -> String {
    "TXN12345".to_string()
}

// ====================================================================
// SWIFT T-Series Format Validation Functions
// ====================================================================

/// Validate BIC with proper SWIFT T-series error reporting
/// Returns T27 for format errors, T28 for length errors, T29 for structure errors
pub fn validate_bic_field(bic: &str, field_tag: &str) -> SwiftValidationResult<()> {
    if bic.len() != 8 && bic.len() != 11 {
        return Err(SwiftValidationError::format_error(
            t_series::T28,
            field_tag,
            bic,
            "8 or 11 characters",
            "Invalid BIC code length (must be 8 or 11 characters)",
        ));
    }

    // Institution Code: 4 letters
    let institution_code = &bic[0..4];
    if !institution_code
        .chars()
        .all(|c| c.is_alphabetic() && c.is_uppercase())
    {
        return Err(SwiftValidationError::format_error(
            t_series::T29,
            field_tag,
            bic,
            "4 uppercase letters for institution code",
            "Invalid BIC structure: institution code must be 4 uppercase letters",
        ));
    }

    // Country Code: 2 letters (must be valid ISO 3166-1 alpha-2)
    let country_code = &bic[4..6];
    if !VALID_COUNTRY_CODES.contains(country_code) {
        return Err(SwiftValidationError::format_error(
            t_series::T73,
            field_tag,
            bic,
            "valid ISO 3166-1 country code",
            "Invalid country code in BIC",
        ));
    }

    // Location Code: 2 alphanumeric characters
    let location_code = &bic[6..8];
    if !location_code
        .chars()
        .all(|c| c.is_alphanumeric() && c.is_uppercase())
    {
        return Err(SwiftValidationError::format_error(
            t_series::T29,
            field_tag,
            bic,
            "2 uppercase alphanumeric characters for location code",
            "Invalid BIC structure: location code must be 2 uppercase alphanumeric characters",
        ));
    }

    // Branch Code: 3 alphanumeric characters (if present)
    if bic.len() == 11 {
        let branch_code = &bic[8..11];
        if !branch_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_uppercase())
        {
            return Err(SwiftValidationError::format_error(
                t_series::T29,
                field_tag,
                bic,
                "3 uppercase alphanumeric characters for branch code",
                "Invalid BIC structure: branch code must be 3 uppercase alphanumeric characters",
            ));
        }
    }

    Ok(())
}

/// Validate currency code with proper SWIFT T-series error reporting
/// Returns T52 for invalid currency codes, T08 for commodity currencies in payment messages
pub fn validate_currency_field(
    currency: &str,
    field_tag: &str,
    allow_commodity: bool,
) -> SwiftValidationResult<()> {
    if currency.len() != 3 {
        return Err(SwiftValidationError::format_error(
            t_series::T52,
            field_tag,
            currency,
            "3-character ISO 4217 currency code",
            "Invalid currency code length (must be 3 characters)",
        ));
    }

    if !currency
        .chars()
        .all(|c| c.is_alphabetic() && c.is_uppercase())
    {
        return Err(SwiftValidationError::format_error(
            t_series::T52,
            field_tag,
            currency,
            "3 uppercase letters",
            "Invalid currency code format (must be 3 uppercase letters)",
        ));
    }

    if !VALID_CURRENCIES.contains(currency) {
        return Err(SwiftValidationError::format_error(
            t_series::T52,
            field_tag,
            currency,
            "valid ISO 4217 currency code",
            "Invalid currency code (not recognized in ISO 4217)",
        ));
    }

    // Check for commodity currencies in payment messages
    if !allow_commodity && currencies::is_commodity_currency(currency) {
        return Err(SwiftValidationError::format_error(
            t_series::T08,
            field_tag,
            currency,
            "non-commodity currency",
            "Commodity currency codes (XAU, XAG, XPD, XPT) not allowed in payment messages",
        ));
    }

    Ok(())
}

/// Validate date in YYMMDD format with proper SWIFT T-series error reporting
/// Returns T50 for invalid date format
pub fn validate_date_field(date: &str, field_tag: &str) -> SwiftValidationResult<()> {
    if date.len() != 6 {
        return Err(SwiftValidationError::format_error(
            t_series::T50,
            field_tag,
            date,
            "YYMMDD format (6 digits)",
            "Invalid date format (must be YYMMDD)",
        ));
    }

    if !date.chars().all(|c| c.is_ascii_digit()) {
        return Err(SwiftValidationError::format_error(
            t_series::T50,
            field_tag,
            date,
            "6 numeric digits",
            "Invalid date format (must contain only digits)",
        ));
    }

    // Basic range validation
    let month: u32 = date[2..4].parse().unwrap_or(13);
    let day: u32 = date[4..6].parse().unwrap_or(32);

    if !(1..=12).contains(&month) {
        return Err(SwiftValidationError::format_error(
            t_series::T50,
            field_tag,
            date,
            "valid month (01-12)",
            "Invalid date: month must be between 01 and 12",
        ));
    }

    if !(1..=31).contains(&day) {
        return Err(SwiftValidationError::format_error(
            t_series::T50,
            field_tag,
            date,
            "valid day (01-31)",
            "Invalid date: day must be between 01 and 31",
        ));
    }

    Ok(())
}

/// Validate amount format with proper SWIFT T-series error reporting
/// Returns T40 for format errors, T43 for exceeding maximum digits
pub fn validate_amount_field(
    amount: &str,
    field_tag: &str,
    currency: Option<&str>,
) -> SwiftValidationResult<()> {
    if amount.is_empty() {
        return Err(SwiftValidationError::format_error(
            t_series::T40,
            field_tag,
            amount,
            "non-empty decimal amount",
            "Amount cannot be empty",
        ));
    }

    // Parse as decimal number
    let parsed = amount.parse::<f64>().map_err(|_| {
        SwiftValidationError::format_error(
            t_series::T40,
            field_tag,
            amount,
            "valid decimal number",
            "Invalid amount format (must be a valid decimal number)",
        )
    })?;

    if parsed < 0.0 {
        return Err(SwiftValidationError::format_error(
            t_series::T40,
            field_tag,
            amount,
            "positive amount",
            "Negative amounts not allowed",
        ));
    }

    // Check for reasonable limits (up to 999 trillion with 15 total digits)
    if parsed >= 1_000_000_000_000_000.0 {
        return Err(SwiftValidationError::format_error(
            t_series::T43,
            field_tag,
            amount,
            "amount under 15 digits",
            "Amount exceeds maximum allowed digits (15 total digits including decimals)",
        ));
    }

    // Validate decimal places based on currency
    if let Some(decimal_pos) = amount.find('.') {
        let decimal_part = &amount[decimal_pos + 1..];
        let max_decimals = match currency {
            Some("JPY") | Some("KRW") | Some("VND") => 0, // No decimal places
            Some("BHD") | Some("IQD") | Some("JOD") | Some("KWD") | Some("LYD") | Some("OMR")
            | Some("TND") => 3, // 3 decimal places
            _ => 2,                                       // Most currencies use 2 decimal places
        };

        if decimal_part.len() > max_decimals {
            return Err(SwiftValidationError::format_error(
                t_series::T40,
                field_tag,
                amount,
                &format!("maximum {max_decimals} decimal places for currency"),
                &format!("Too many decimal places for currency (max {max_decimals} allowed)"),
            ));
        }
    }

    Ok(())
}

/// Validate field for slash usage (T26 validation)
/// Fields must not contain consecutive slashes, but leading slashes are allowed for structured data
pub fn validate_slash_usage(value: &str, field_tag: &str) -> SwiftValidationResult<()> {
    // Check for consecutive slashes (not allowed in SWIFT)
    if value.contains("//") {
        return Err(SwiftValidationError::format_error(
            t_series::T26,
            field_tag,
            value,
            "field without consecutive slashes",
            "Field must not contain consecutive slashes '//'",
        ));
    }

    // Check for trailing slash (not allowed in SWIFT)
    if value.ends_with('/') {
        return Err(SwiftValidationError::format_error(
            t_series::T26,
            field_tag,
            value,
            "field not ending with '/'",
            "Field must not end with slash '/'",
        ));
    }

    // Leading slash is allowed in SWIFT for structured data (account numbers, party identifiers, etc.)
    // So we don't check for starts_with('/')

    Ok(())
}

/// Validate structured address format (T56 validation)
/// Addresses should follow SWIFT structured format rules
pub fn validate_address_format(address: &str, field_tag: &str) -> SwiftValidationResult<()> {
    if address.is_empty() {
        return Err(SwiftValidationError::format_error(
            t_series::T56,
            field_tag,
            address,
            "non-empty address",
            "Address cannot be empty",
        ));
    }

    // Check for invalid characters (control characters, etc.)
    if address.chars().any(|c| c.is_control()) {
        return Err(SwiftValidationError::format_error(
            t_series::T56,
            field_tag,
            address,
            "address without control characters",
            "Address contains invalid control characters",
        ));
    }

    // Validate slash usage in address
    validate_slash_usage(address, field_tag)?;

    Ok(())
}

/// Validate identifier code format (T45 validation)
/// Party identifiers should follow SWIFT format requirements
pub fn validate_identifier_format(identifier: &str, field_tag: &str) -> SwiftValidationResult<()> {
    if identifier.is_empty() {
        return Err(SwiftValidationError::format_error(
            t_series::T45,
            field_tag,
            identifier,
            "non-empty identifier",
            "Identifier cannot be empty",
        ));
    }

    // Check length constraints (typically max 34 characters for account numbers)
    if identifier.len() > 34 {
        return Err(SwiftValidationError::format_error(
            t_series::T45,
            field_tag,
            identifier,
            "identifier with max 34 characters",
            "Identifier exceeds maximum length of 34 characters",
        ));
    }

    // Allow alphanumeric characters and common separators
    if !identifier
        .chars()
        .all(|c| c.is_alphanumeric() || "/.,()-".contains(c))
    {
        return Err(SwiftValidationError::format_error(
            t_series::T45,
            field_tag,
            identifier,
            "alphanumeric characters and common separators (/.,()-)",
            "Identifier contains invalid characters",
        ));
    }

    Ok(())
}

/// Validate enumerated field values (T08 validation)
/// Check if field contains valid enumerated values
pub fn validate_enumerated_field(
    value: &str,
    field_tag: &str,
    valid_values: &[&str],
) -> SwiftValidationResult<()> {
    if !valid_values.contains(&value) {
        return Err(SwiftValidationError::format_error(
            t_series::T08,
            field_tag,
            value,
            &format!("one of: {}", valid_values.join(", ")),
            &format!(
                "Invalid code '{}', must be one of: {}",
                value,
                valid_values.join(", ")
            ),
        ));
    }

    Ok(())
}

/// Validate charge codes (BEN, OUR, SHA)
pub fn validate_charge_code(code: &str, field_tag: &str) -> SwiftValidationResult<()> {
    validate_enumerated_field(code, field_tag, &["BEN", "OUR", "SHA"])
}

/// Validate instruction codes for field 23E
pub fn validate_instruction_code(
    code: &str,
    field_tag: &str,
    context: Option<&str>,
) -> SwiftValidationResult<()> {
    // Valid instruction codes depend on context (e.g., SPRI restrictions)
    let valid_codes = match context {
        Some("SPRI") => &["SDVA", "TELB", "PHOB", "INTC"][..],
        _ => &["HOLD", "PHOB", "TELB", "SDVA", "INTC", "RETN", "CHQB"][..],
    };

    validate_enumerated_field(code, field_tag, valid_codes)
}

/// Comprehensive field format validation dispatcher
/// Routes to appropriate T-series validation based on field type
pub fn validate_field_format(
    value: &str,
    field_tag: &str,
    format_spec: &str,
    context: Option<&std::collections::HashMap<String, String>>,
) -> SwiftValidationResult<()> {
    match format_spec {
        "BIC" => validate_bic_field(value, field_tag),
        "currency" => {
            let allow_commodity = context
                .and_then(|c| c.get("allow_commodity"))
                .map(|v| v == "true")
                .unwrap_or(false);
            validate_currency_field(value, field_tag, allow_commodity)
        }
        "date" => validate_date_field(value, field_tag),
        "amount" => {
            let currency = context.and_then(|c| c.get("currency")).map(|s| s.as_str());
            validate_amount_field(value, field_tag, currency)
        }
        "address" => validate_address_format(value, field_tag),
        "identifier" => validate_identifier_format(value, field_tag),
        "charge_code" => validate_charge_code(value, field_tag),
        "instruction_code" => {
            let inst_context = context
                .and_then(|c| c.get("instruction_context"))
                .map(|s| s.as_str());
            validate_instruction_code(value, field_tag, inst_context)
        }
        _ => {
            // Generic format validation for other field types
            validate_slash_usage(value, field_tag)?;
            Ok(())
        }
    }
}

/// Validate message type consistency
pub fn validate_message_type(actual: &str, expected: &str) -> SwiftValidationResult<()> {
    if actual != expected {
        Err(SwiftValidationError::format_error(
            t_series::T03,
            "MESSAGE_TYPE",
            actual,
            expected,
            &format!("Message type mismatch: expected {expected}, got {actual}"),
        ))
    } else {
        Ok(())
    }
}

// ====================================================================
// SWIFT C/D/E-Series Business Rule Validation Functions
// ====================================================================

use crate::swift_error_codes::{c_series, d_series, e_series};

/// Message validation context containing all fields for cross-field validation
#[derive(Debug, Clone)]
pub struct MessageValidationContext {
    pub fields: std::collections::HashMap<String, String>,
    pub message_type: String,
    pub sender_country: Option<String>,
    pub receiver_country: Option<String>,
}

/// C-Series: Currency code mismatch validation (C02)
/// Validates that related fields have matching currencies
pub fn validate_currency_consistency(
    field1_tag: &str,
    field1_currency: &str,
    field2_tag: &str,
    field2_currency: &str,
) -> SwiftValidationResult<()> {
    if field1_currency != field2_currency {
        return Err(SwiftValidationError::business_error(
            c_series::C02,
            field1_tag,
            vec![field2_tag.to_string()],
            &format!(
                "Currency mismatch: {field1_tag} has '{field1_currency}' but {field2_tag} has '{field2_currency}'"
            ),
            "Related fields must have matching currency codes",
        ));
    }
    Ok(())
}

/// C-Series: Amount format validation for currency-specific rules (C03)
/// Validates decimal requirements based on currency
pub fn validate_amount_currency_rules(
    amount: &str,
    currency: &str,
    field_tag: &str,
) -> SwiftValidationResult<()> {
    // Check decimal places for specific currencies
    if let Some(decimal_pos) = amount.find('.') {
        let decimal_part = &amount[decimal_pos + 1..];
        let expected_decimals = match currency {
            "JPY" | "KRW" | "VND" => 0,
            "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
            _ => 2,
        };

        if decimal_part.len() > expected_decimals {
            return Err(SwiftValidationError::business_error(
                c_series::C03,
                field_tag,
                vec![],
                &format!(
                    "Amount '{}' has {} decimal places but currency '{}' allows maximum {}",
                    amount,
                    decimal_part.len(),
                    currency,
                    expected_decimals
                ),
                "Amount decimal places must match currency requirements",
            ));
        }

        // For currencies that don't allow decimals, fail if decimals are present
        if expected_decimals == 0 && !decimal_part.is_empty() {
            return Err(SwiftValidationError::business_error(
                c_series::C03,
                field_tag,
                vec![],
                &format!(
                    "Currency '{currency}' does not allow decimal places but amount '{amount}' has decimals"
                ),
                "Amount must be integer for this currency",
            ));
        }
    }

    Ok(())
}

/// C-Series: Conditional field dependency validation (C81)
/// Validates that if field A is present, field B must also be present
pub fn validate_conditional_field_presence(
    context: &MessageValidationContext,
    trigger_field: &str,
    required_field: &str,
) -> SwiftValidationResult<()> {
    if context.fields.contains_key(trigger_field) && !context.fields.contains_key(required_field) {
        return Err(SwiftValidationError::business_error(
            c_series::C81,
            trigger_field,
            vec![required_field.to_string()],
            &format!(
                "Field {trigger_field} is present but required field {required_field} is missing"
            ),
            "Conditional field dependency violated",
        ));
    }
    Ok(())
}

/// D-Series: IBAN mandatory for SEPA validation (D19)
/// Validates IBAN requirement for European country combinations
pub fn validate_sepa_iban_requirement(
    beneficiary_field: &str,
    beneficiary_country: Option<&str>,
    sender_country: Option<&str>,
    field_tag: &str,
) -> SwiftValidationResult<()> {
    use crate::swift_error_codes::regional;

    // Check if both countries are SEPA countries
    let both_sepa = beneficiary_country
        .zip(sender_country)
        .map(|(ben_country, sen_country)| {
            regional::is_sepa_country(ben_country) && regional::is_sepa_country(sen_country)
        })
        .unwrap_or(false);

    if both_sepa {
        // For SEPA payments, IBAN should be present (simplified check for demo)
        if !beneficiary_field.contains("IBAN")
            && !beneficiary_field.starts_with("DE")
            && !beneficiary_field.starts_with("FR")
            && !beneficiary_field.starts_with("IT")
        {
            return Err(SwiftValidationError::content_error(
                d_series::D19,
                field_tag,
                beneficiary_field,
                "IBAN is mandatory for SEPA (Single Euro Payments Area) transactions",
                "Payments between SEPA countries must include IBAN",
            ));
        }
    }

    Ok(())
}

/// D-Series: Field 33B mandatory for EU transfers (D49)
/// Validates that instructed amount is present for European transfers
pub fn validate_eu_instructed_amount_requirement(
    context: &MessageValidationContext,
    sender_country: Option<&str>,
    receiver_country: Option<&str>,
) -> SwiftValidationResult<()> {
    use crate::swift_error_codes::regional;

    // Check if both countries are in SEPA region
    let both_eu = sender_country
        .zip(receiver_country)
        .map(|(sen_country, rec_country)| {
            regional::is_sepa_country(sen_country) && regional::is_sepa_country(rec_country)
        })
        .unwrap_or(false);

    if both_eu && !context.fields.contains_key("33B") {
        return Err(SwiftValidationError::content_error(
            d_series::D49,
            "33B",
            "",
            "Field 33B (Instructed Amount) is mandatory for intra-European transfers",
            "European country combination requires instructed amount",
        ));
    }

    Ok(())
}

/// D-Series: SHA charge handling restrictions (D50)
/// Validates field restrictions when SHA charge code is used
pub fn validate_sha_charge_restrictions(
    context: &MessageValidationContext,
    charge_code: &str,
) -> SwiftValidationResult<()> {
    if charge_code == "SHA" {
        // Field 71F is optional, 71G is not allowed with SHA
        if context.fields.contains_key("71G") {
            return Err(SwiftValidationError::content_error(
                d_series::D50,
                "71G",
                context.fields.get("71G").unwrap_or(&"".to_string()),
                "Field 71G (Receiver's Charges) not allowed with SHA charge code",
                "SHA (shared) charges prohibit receiver charges field",
            ));
        }
    }

    Ok(())
}

/// D-Series: Exchange rate mandatory when currencies differ (D75)
/// Validates that exchange rate is present when field currencies differ
pub fn validate_exchange_rate_requirement(
    context: &MessageValidationContext,
    primary_currency: &str,
    secondary_currency: &str,
) -> SwiftValidationResult<()> {
    if primary_currency != secondary_currency && !context.fields.contains_key("36") {
        return Err(SwiftValidationError::content_error(
            d_series::D75,
            "36",
            "",
            &format!(
                "Exchange rate field 36 is mandatory when currencies differ ('{primary_currency}' vs '{secondary_currency}')"
            ),
            "Different currencies require exchange rate specification",
        ));
    }

    Ok(())
}

/// E-Series: Instruction code restrictions for SPRI (E01)
/// Validates that SPRI context only allows specific instruction codes
pub fn validate_spri_instruction_restrictions(
    instruction_code: &str,
    bank_operation_code: Option<&str>,
    field_tag: &str,
) -> SwiftValidationResult<()> {
    if let Some("SPRI") = bank_operation_code {
        let allowed_codes = ["SDVA", "TELB", "PHOB", "INTC"];
        if !allowed_codes.contains(&instruction_code) {
            return Err(SwiftValidationError::relation_error(
                e_series::E01,
                field_tag,
                vec!["23B".to_string()],
                &format!(
                    "Instruction code '{}' not allowed with SPRI. Allowed codes: {}",
                    instruction_code,
                    allowed_codes.join(", ")
                ),
                "SPRI bank operation code restricts allowed instruction codes",
            ));
        }
    }

    Ok(())
}

/// E-Series: Field option restrictions (E03)
/// Validates that certain field options are not allowed in specific contexts
pub fn validate_field_option_restrictions(
    field_option: &str,
    context_code: Option<&str>,
    field_tag: &str,
) -> SwiftValidationResult<()> {
    // Field 53a cannot use option D with SPRI/SSTD/SPAY
    if field_tag == "53A" && field_option == "D" {
        if let Some(code) = context_code {
            if ["SPRI", "SSTD", "SPAY"].contains(&code) {
                return Err(SwiftValidationError::relation_error(
                    e_series::E03,
                    field_tag,
                    vec!["23B".to_string()],
                    &format!("Field {field_tag}a option D not allowed with {code}"),
                    "Field option restrictions apply for specific bank operation codes",
                ));
            }
        }
    }

    Ok(())
}

/// E-Series: Multiple field dependency (E06)
/// Validates that if field 55a is present, both 53a and 54a are required
pub fn validate_multiple_field_dependency(
    context: &MessageValidationContext,
) -> SwiftValidationResult<()> {
    if context.fields.contains_key("55A") {
        let missing_fields: Vec<String> = ["53A", "54A"]
            .iter()
            .filter(|&&field| !context.fields.contains_key(field))
            .map(|&field| field.to_string())
            .collect();

        if !missing_fields.is_empty() {
            return Err(SwiftValidationError::relation_error(
                e_series::E06,
                "55A",
                missing_fields.clone(),
                &format!(
                    "Field 55A is present but required fields are missing: {}",
                    missing_fields.join(", ")
                ),
                "Field 55A presence requires both fields 53A and 54A",
            ));
        }
    }

    Ok(())
}

/// E-Series: OUR charge handling restrictions (E13)
/// Validates field restrictions when OUR charge code is used
pub fn validate_our_charge_restrictions(
    context: &MessageValidationContext,
    charge_code: &str,
) -> SwiftValidationResult<()> {
    if charge_code == "OUR" {
        // Field 71F is not allowed, 71G is optional with OUR
        if context.fields.contains_key("71F") {
            return Err(SwiftValidationError::relation_error(
                e_series::E13,
                "71F",
                vec!["71A".to_string()],
                "Field 71F (Sender's Charges) not allowed with OUR charge code",
                "OUR (payer) charges prohibit sender charges field",
            ));
        }
    }

    Ok(())
}

/// E-Series: BEN charge handling requirements (E15)
/// Validates field requirements when BEN charge code is used
pub fn validate_ben_charge_requirements(
    context: &MessageValidationContext,
    charge_code: &str,
) -> SwiftValidationResult<()> {
    if charge_code == "BEN" {
        // Field 71F is mandatory, 71G is not allowed with BEN
        if !context.fields.contains_key("71F") {
            return Err(SwiftValidationError::relation_error(
                e_series::E15,
                "71F",
                vec!["71A".to_string()],
                "Field 71F (Sender's Charges) is mandatory with BEN charge code",
                "BEN (beneficiary) charges require sender charges field",
            ));
        }

        if context.fields.contains_key("71G") {
            return Err(SwiftValidationError::relation_error(
                e_series::E15,
                "71G",
                vec!["71A".to_string()],
                "Field 71G (Receiver's Charges) not allowed with BEN charge code",
                "BEN (beneficiary) charges prohibit receiver charges field",
            ));
        }
    }

    Ok(())
}

/// Comprehensive message-level business rule validation
/// Validates all C/D/E-series business rules for a complete message
pub fn validate_message_business_rules(
    context: &MessageValidationContext,
) -> Vec<SwiftValidationError> {
    let mut errors = Vec::new();

    // Example validations - in practice, this would include all relevant business rules

    // Validate charge code restrictions
    if let Some(charge_code) = context.fields.get("71A") {
        if let Err(error) = validate_sha_charge_restrictions(context, charge_code) {
            errors.push(error);
        }
        if let Err(error) = validate_our_charge_restrictions(context, charge_code) {
            errors.push(error);
        }
        if let Err(error) = validate_ben_charge_requirements(context, charge_code) {
            errors.push(error);
        }
    }

    // Validate currency consistency between fields 32A and 33B
    if let (Some(field_32a), Some(field_33b)) =
        (context.fields.get("32A"), context.fields.get("33B"))
    {
        // Extract currency from amount fields (simplified - in practice would parse field properly)
        if field_32a.len() >= 9 && field_33b.len() >= 3 {
            let currency_32a = &field_32a[6..9]; // Simplified extraction
            let currency_33b = &field_33b[0..3]; // Simplified extraction
            if let Err(error) =
                validate_currency_consistency("32A", currency_32a, "33B", currency_33b)
            {
                errors.push(error);
            }
        }
    }

    // Validate multiple field dependencies
    if let Err(error) = validate_multiple_field_dependency(context) {
        errors.push(error);
    }

    // Validate conditional field presence
    if let Err(error) = validate_conditional_field_presence(context, "56A", "57A") {
        errors.push(error);
    }

    // Validate SEPA requirements for European transfers
    if let (Some(sender), Some(receiver)) = (&context.sender_country, &context.receiver_country) {
        if let Err(error) =
            validate_eu_instructed_amount_requirement(context, Some(sender), Some(receiver))
        {
            errors.push(error);
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bic_validation() {
        assert!(is_valid_bic("DEUTDEFFXXX")); // 11 characters with branch
        assert!(is_valid_bic("DEUTDEFF")); // 8 characters without branch
        assert!(!is_valid_bic("DEUT")); // Too short
        assert!(!is_valid_bic("deutdeff")); // Lowercase
        assert!(!is_valid_bic("DEUT99FF")); // Invalid country code
        assert!(!is_valid_bic("DE1TDEFF")); // Invalid institution code
    }

    #[test]
    fn test_currency_validation() {
        assert!(is_valid_currency("USD"));
        assert!(is_valid_currency("EUR"));
        assert!(is_valid_currency("GBP"));
        assert!(!is_valid_currency("usd")); // Lowercase
        assert!(!is_valid_currency("FAKE")); // Invalid currency
        assert!(!is_valid_currency("US")); // Too short
        assert!(!is_valid_currency("USDT")); // Too long
    }

    #[test]
    fn test_account_validation() {
        assert!(is_valid_account_number("1234567890"));
        assert!(is_valid_account_number("GB29NWBK60161331926819"));
        assert!(is_valid_account_number("DE89370400440532013000"));
        assert!(!is_valid_account_number("")); // Empty
        assert!(!is_valid_account_number(&"A".repeat(35))); // Too long
    }

    #[test]
    fn test_amount_validation() {
        assert!(is_valid_amount("1234.56"));
        assert!(is_valid_amount("0.01"));
        assert!(is_valid_amount("1000000"));
        assert!(!is_valid_amount("-100")); // Negative
        assert!(!is_valid_amount("invalid")); // Not a number
        assert!(!is_valid_amount("123.123456")); // Too many decimal places
    }

    #[test]
    fn test_date_validation() {
        assert!(is_valid_swift_date("231215")); // Valid date
        assert!(is_valid_swift_date("200229")); // Leap year
        assert!(!is_valid_swift_date("23121")); // Too short
        assert!(!is_valid_swift_date("2312ab")); // Non-numeric
        assert!(!is_valid_swift_date("231300")); // Invalid month
        assert!(!is_valid_swift_date("231232")); // Invalid day
    }

    #[test]
    fn test_time_validation() {
        assert!(is_valid_swift_time("1430")); // Valid 24h time
        assert!(is_valid_swift_time("143045")); // With seconds
        assert!(!is_valid_swift_time("2500")); // Invalid hour
        assert!(!is_valid_swift_time("1260")); // Invalid minute
        assert!(!is_valid_swift_time("123")); // Too short
    }

    // ====================================================================
    // Tests for SWIFT T-Series Format Validation Functions
    // ====================================================================

    #[test]
    fn test_validate_bic_field() {
        // Valid BICs
        assert!(validate_bic_field("DEUTDEFFXXX", "57A").is_ok());
        assert!(validate_bic_field("DEUTDEFF", "52A").is_ok());

        // Invalid length (T28)
        let result = validate_bic_field("DEUT", "57A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T28");

        // Invalid structure (T29)
        let result = validate_bic_field("DE1TDEFF", "57A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T29");

        // Invalid country code (T73)
        let result = validate_bic_field("DEUT99FF", "57A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T73");
    }

    #[test]
    fn test_validate_currency_field() {
        // Valid currencies
        assert!(validate_currency_field("USD", "32A", false).is_ok());
        assert!(validate_currency_field("EUR", "32A", false).is_ok());

        // Invalid length (T52)
        let result = validate_currency_field("US", "32A", false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T52");

        // Invalid format (T52)
        let result = validate_currency_field("usd", "32A", false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T52");

        // Invalid currency (T52)
        let result = validate_currency_field("XYZ", "32A", false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T52");

        // Commodity currency not allowed (T08)
        let result = validate_currency_field("XAU", "32A", false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T08");

        // Commodity currency allowed
        assert!(validate_currency_field("XAU", "32A", true).is_ok());
    }

    #[test]
    fn test_validate_date_field() {
        // Valid dates
        assert!(validate_date_field("231215", "30").is_ok());
        assert!(validate_date_field("200229", "30").is_ok());

        // Invalid length (T50)
        let result = validate_date_field("23121", "30");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T50");

        // Invalid format (T50)
        let result = validate_date_field("2312ab", "30");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T50");

        // Invalid month (T50)
        let result = validate_date_field("231300", "30");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T50");

        // Invalid day (T50)
        let result = validate_date_field("231232", "30");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T50");
    }

    #[test]
    fn test_validate_amount_field() {
        // Valid amounts
        assert!(validate_amount_field("1234.56", "32A", Some("USD")).is_ok());
        assert!(validate_amount_field("1000", "32A", Some("JPY")).is_ok());

        // Empty amount (T40)
        let result = validate_amount_field("", "32A", None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T40");

        // Invalid format (T40)
        let result = validate_amount_field("invalid", "32A", None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T40");

        // Negative amount (T40)
        let result = validate_amount_field("-100", "32A", None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T40");

        // Too many decimals for currency (T40)
        let result = validate_amount_field("123.567", "32A", Some("USD"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T40");

        // Decimal places for JPY (should fail)
        let result = validate_amount_field("123.45", "32A", Some("JPY"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T40");

        // Exceeds maximum digits (T43)
        let result = validate_amount_field("1000000000000000", "32A", None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T43");
    }

    #[test]
    fn test_validate_slash_usage() {
        // Valid values
        assert!(validate_slash_usage("ABC/DEF", "70").is_ok());
        assert!(validate_slash_usage("ABCDEF", "70").is_ok());
        assert!(validate_slash_usage("/ABCDEF", "70").is_ok()); // Leading slash is valid for structured data

        // Invalid: ends with slash (T26)
        let result = validate_slash_usage("ABCDEF/", "70");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T26");

        // Invalid: consecutive slashes (T26)
        let result = validate_slash_usage("ABC//DEF", "70");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T26");
    }

    #[test]
    fn test_validate_enumerated_field() {
        // Valid value
        assert!(validate_enumerated_field("BEN", "71A", &["BEN", "OUR", "SHA"]).is_ok());

        // Invalid value (T08)
        let result = validate_enumerated_field("INVALID", "71A", &["BEN", "OUR", "SHA"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T08");
    }

    #[test]
    fn test_validate_charge_code() {
        // Valid charge codes
        assert!(validate_charge_code("BEN", "71A").is_ok());
        assert!(validate_charge_code("OUR", "71A").is_ok());
        assert!(validate_charge_code("SHA", "71A").is_ok());

        // Invalid charge code (T08)
        let result = validate_charge_code("INVALID", "71A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T08");
    }

    #[test]
    fn test_validate_instruction_code() {
        // Valid instruction codes
        assert!(validate_instruction_code("HOLD", "23E", None).is_ok());
        assert!(validate_instruction_code("SDVA", "23E", Some("SPRI")).is_ok());

        // Invalid instruction code (T08)
        let result = validate_instruction_code("INVALID", "23E", None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T08");

        // Invalid instruction code for SPRI context (T08)
        let result = validate_instruction_code("HOLD", "23E", Some("SPRI"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T08");
    }

    #[test]
    fn test_validate_field_format() {
        use std::collections::HashMap;

        // Test BIC validation
        assert!(validate_field_format("DEUTDEFFXXX", "57A", "BIC", None).is_ok());

        // Test currency validation
        assert!(validate_field_format("USD", "32A", "currency", None).is_ok());

        // Test currency with context
        let mut context = HashMap::new();
        context.insert("allow_commodity".to_string(), "true".to_string());
        assert!(validate_field_format("XAU", "32A", "currency", Some(&context)).is_ok());

        // Test amount validation with currency context
        let mut context = HashMap::new();
        context.insert("currency".to_string(), "USD".to_string());
        assert!(validate_field_format("1234.56", "32A", "amount", Some(&context)).is_ok());

        // Test date validation
        assert!(validate_field_format("231215", "30", "date", None).is_ok());

        // Test charge code validation
        assert!(validate_field_format("BEN", "71A", "charge_code", None).is_ok());

        // Test instruction code validation with context
        let mut context = HashMap::new();
        context.insert("instruction_context".to_string(), "SPRI".to_string());
        assert!(validate_field_format("SDVA", "23E", "instruction_code", Some(&context)).is_ok());

        // Test generic validation (slash usage)
        assert!(validate_field_format("ABC/DEF", "70", "generic", None).is_ok());
        assert!(validate_field_format("/VALID", "70", "generic", None).is_ok()); // Leading slash is now valid
        let result = validate_field_format("INVALID/", "70", "generic", None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "T26");
    }

    // ====================================================================
    // Tests for SWIFT C/D/E-Series Business Rule Validation Functions
    // ====================================================================

    #[test]
    fn test_validate_currency_consistency() {
        // Matching currencies - should pass
        assert!(validate_currency_consistency("32A", "USD", "33B", "USD").is_ok());

        // Different currencies - should fail with C02
        let result = validate_currency_consistency("32A", "USD", "33B", "EUR");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "C02");
    }

    #[test]
    fn test_validate_amount_currency_rules() {
        // Valid amounts for different currencies
        assert!(validate_amount_currency_rules("1234.56", "USD", "32A").is_ok());
        assert!(validate_amount_currency_rules("1234", "JPY", "32A").is_ok());
        assert!(validate_amount_currency_rules("1234.567", "BHD", "32A").is_ok());

        // Too many decimals for USD (C03)
        let result = validate_amount_currency_rules("1234.567", "USD", "32A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "C03");

        // Decimals not allowed for JPY (C03)
        let result = validate_amount_currency_rules("1234.56", "JPY", "32A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "C03");
    }

    #[test]
    fn test_validate_conditional_field_presence() {
        use std::collections::HashMap;

        // Both fields present - should pass
        let mut fields = HashMap::new();
        fields.insert("56A".to_string(), "DEUTDEFFXXX".to_string());
        fields.insert("57A".to_string(), "BANKDEFFXXX".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        assert!(validate_conditional_field_presence(&context, "56A", "57A").is_ok());

        // Trigger field present but required field missing - should fail with C81
        let mut fields = HashMap::new();
        fields.insert("56A".to_string(), "DEUTDEFFXXX".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        let result = validate_conditional_field_presence(&context, "56A", "57A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "C81");

        // Neither field present - should pass
        let context = MessageValidationContext {
            fields: HashMap::new(),
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        assert!(validate_conditional_field_presence(&context, "56A", "57A").is_ok());
    }

    #[test]
    fn test_validate_sha_charge_restrictions() {
        use std::collections::HashMap;

        // SHA with no 71G field - should pass
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "SHA".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        assert!(validate_sha_charge_restrictions(&context, "SHA").is_ok());

        // SHA with 71G field - should fail with D50
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "SHA".to_string());
        fields.insert("71G".to_string(), "USD10,00".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        let result = validate_sha_charge_restrictions(&context, "SHA");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "D50");
    }

    #[test]
    fn test_validate_our_charge_restrictions() {
        use std::collections::HashMap;

        // OUR with no 71F field - should pass
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "OUR".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        assert!(validate_our_charge_restrictions(&context, "OUR").is_ok());

        // OUR with 71F field - should fail with E13
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "OUR".to_string());
        fields.insert("71F".to_string(), "USD10,00".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        let result = validate_our_charge_restrictions(&context, "OUR");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "E13");
    }

    #[test]
    fn test_validate_ben_charge_requirements() {
        use std::collections::HashMap;

        // BEN with 71F field and no 71G - should pass
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "BEN".to_string());
        fields.insert("71F".to_string(), "USD10,00".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        assert!(validate_ben_charge_requirements(&context, "BEN").is_ok());

        // BEN without 71F field - should fail with E15
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "BEN".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        let result = validate_ben_charge_requirements(&context, "BEN");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "E15");

        // BEN with 71G field - should fail with E15
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "BEN".to_string());
        fields.insert("71F".to_string(), "USD10,00".to_string());
        fields.insert("71G".to_string(), "USD5,00".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        let result = validate_ben_charge_requirements(&context, "BEN");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "E15");
    }

    #[test]
    fn test_validate_multiple_field_dependency() {
        use std::collections::HashMap;

        // All required fields present - should pass
        let mut fields = HashMap::new();
        fields.insert("55A".to_string(), "BANKDEFFXXX".to_string());
        fields.insert("53A".to_string(), "DEUTDEFFXXX".to_string());
        fields.insert("54A".to_string(), "CHASUS33XXX".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        assert!(validate_multiple_field_dependency(&context).is_ok());

        // 55A present but 53A missing - should fail with E06
        let mut fields = HashMap::new();
        fields.insert("55A".to_string(), "BANKDEFFXXX".to_string());
        fields.insert("54A".to_string(), "CHASUS33XXX".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        let result = validate_multiple_field_dependency(&context);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "E06");

        // 55A not present - should pass
        let mut fields = HashMap::new();
        fields.insert("53A".to_string(), "DEUTDEFFXXX".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: None,
            receiver_country: None,
        };
        assert!(validate_multiple_field_dependency(&context).is_ok());
    }

    #[test]
    fn test_validate_spri_instruction_restrictions() {
        // Valid instruction code with SPRI - should pass
        assert!(validate_spri_instruction_restrictions("SDVA", Some("SPRI"), "23E").is_ok());
        assert!(validate_spri_instruction_restrictions("TELB", Some("SPRI"), "23E").is_ok());

        // Invalid instruction code with SPRI - should fail with E01
        let result = validate_spri_instruction_restrictions("HOLD", Some("SPRI"), "23E");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "E01");

        // Any instruction code without SPRI - should pass
        assert!(validate_spri_instruction_restrictions("HOLD", Some("CRED"), "23E").is_ok());
        assert!(validate_spri_instruction_restrictions("HOLD", None, "23E").is_ok());
    }

    #[test]
    fn test_validate_field_option_restrictions() {
        // Valid option with SPRI - should pass (not option D)
        assert!(validate_field_option_restrictions("A", Some("SPRI"), "53A").is_ok());

        // Invalid option D with SPRI - should fail with E03
        let result = validate_field_option_restrictions("D", Some("SPRI"), "53A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), "E03");

        // Option D with non-restricted code - should pass
        assert!(validate_field_option_restrictions("D", Some("CRED"), "53A").is_ok());

        // Different field tag - should pass
        assert!(validate_field_option_restrictions("D", Some("SPRI"), "52A").is_ok());
    }

    #[test]
    fn test_validate_message_business_rules() {
        use std::collections::HashMap;

        // Valid message - should return no errors
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "SHA".to_string());
        fields.insert("32A".to_string(), "250615USD1000,00".to_string());
        fields.insert("33B".to_string(), "USD1000,00".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: Some("US".to_string()),
            receiver_country: Some("GB".to_string()),
        };
        let errors = validate_message_business_rules(&context);
        assert!(errors.is_empty());

        // Invalid message with SHA + 71G - should return D50 error
        let mut fields = HashMap::new();
        fields.insert("71A".to_string(), "SHA".to_string());
        fields.insert("71G".to_string(), "USD10,00".to_string());
        let context = MessageValidationContext {
            fields,
            message_type: "103".to_string(),
            sender_country: Some("US".to_string()),
            receiver_country: Some("GB".to_string()),
        };
        let errors = validate_message_business_rules(&context);
        assert!(!errors.is_empty());
        assert_eq!(errors[0].code(), "D50");
    }
}
