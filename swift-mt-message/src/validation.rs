//! SWIFT validation utilities
//!
//! This module provides validation functions for SWIFT-specific data types
//! like BIC codes, currency codes, and other financial identifiers.

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

/// Valid ISO 4217 currency codes (major currencies)
static VALID_CURRENCIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK", "DKK", "PLN", "CZK",
        "HUF", "RUB", "CNY", "HKD", "SGD", "KRW", "THB", "INR", "MYR", "PHP", "IDR", "VND", "BRL",
        "MXN", "ARS", "CLP", "COP", "PEN", "UYU", "ZAR", "EGP", "NGN", "GHS", "KES", "TZS", "UGX",
        "ZMW", "BWP", "MUR", "SCR", "SZL", "LSL", "NAD", "AOA", "XOF", "XAF", "MAD", "TND", "DZD",
        "LYD", "ILS", "JOD", "LBP", "SYP", "IQD", "IRR", "SAR", "AED", "QAR", "BHD", "KWD", "OMR",
        "YER", "AFN", "PKR", "LKR", "BDT", "BTN", "NPR", "MMK", "LAK", "KHR", "MNT", "KZT", "UZS",
        "KGS", "TJS", "TMT", "AZN", "GEL", "AMD", "BGN", "RON", "HRK", "RSD", "BAM", "MKD", "ALL",
        "MDL", "UAH", "BYN", "LTL", "LVL", "EEK", "ISK", "TRY",
    ]
    .iter()
    .copied()
    .collect()
});

/// Valid ISO 3166-1 alpha-2 country codes (subset for BIC validation)
static VALID_COUNTRY_CODES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW", "AX",
        "AZ", "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ",
        "BR", "BS", "BT", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CF", "CG", "CH", "CI", "CK",
        "CL", "CM", "CN", "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ", "DE", "DJ", "DK", "DM",
        "DO", "DZ", "EC", "EE", "EG", "EH", "ER", "ES", "ET", "FI", "FJ", "FK", "FM", "FO", "FR",
        "GA", "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL", "GM", "GN", "GP", "GQ", "GR", "GS",
        "GT", "GU", "GW", "GY", "HK", "HM", "HN", "HR", "HT", "HU", "ID", "IE", "IL", "IM", "IN",
        "IO", "IQ", "IR", "IS", "IT", "JE", "JM", "JO", "JP", "KE", "KG", "KH", "KI", "KM", "KN",
        "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC", "LI", "LK", "LR", "LS", "LT", "LU", "LV",
        "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK", "ML", "MM", "MN", "MO", "MP", "MQ",
        "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY", "MZ", "NA", "NC", "NE", "NF", "NG", "NI",
        "NL", "NO", "NP", "NR", "NU", "NZ", "OM", "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM",
        "PN", "PR", "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW", "SA", "SB", "SC",
        "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST", "SV",
        "SX", "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO", "TR",
        "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG", "VI",
        "VN", "VU", "WF", "WS", "YE", "YT", "ZA", "ZM", "ZW",
    ]
    .iter()
    .copied()
    .collect()
});

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
    if bic.len() != 8 && bic.len() != 11 {
        return false;
    }

    // Institution Code: 4 letters
    let institution_code = &bic[0..4];
    if !institution_code
        .chars()
        .all(|c| c.is_alphabetic() && c.is_uppercase())
    {
        return false;
    }

    // Country Code: 2 letters (must be valid ISO 3166-1 alpha-2)
    let country_code = &bic[4..6];
    if !VALID_COUNTRY_CODES.contains(country_code) {
        return false;
    }

    // Location Code: 2 alphanumeric characters
    let location_code = &bic[6..8];
    if !location_code
        .chars()
        .all(|c| c.is_alphanumeric() && c.is_uppercase())
    {
        return false;
    }

    // Branch Code: 3 alphanumeric characters (if present)
    if bic.len() == 11 {
        let branch_code = &bic[8..11];
        if !branch_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_uppercase())
        {
            return false;
        }
    }

    true
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

    VALID_CURRENCIES.contains(currency)
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
    let year: u32 = date[0..2].parse().unwrap_or(99);
    let month: u32 = date[2..4].parse().unwrap_or(13);
    let day: u32 = date[4..6].parse().unwrap_or(32);

    if month < 1 || month > 12 {
        return false;
    }

    if day < 1 || day > 31 {
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
}
