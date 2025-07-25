//! Shared validation rules and data for SWIFT MT messages
//!
//! This module provides common validation data and rules that can be used by both
//! the main library (runtime validation) and the macro library (compile-time validation).
//!
//! By centralizing these rules, we ensure consistency between compile-time and runtime
//! validation and eliminate code duplication.

use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

/// Currency decimal places configuration
pub static CURRENCY_DECIMALS: Lazy<HashMap<&'static str, u8>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Most currencies use 2 decimal places (default)
    // Special cases with 0 decimals
    map.insert("JPY", 0);
    map.insert("KRW", 0);
    map.insert("VND", 0);
    map.insert("IDR", 0);
    map.insert("CLP", 0);
    map.insert("PYG", 0);
    map.insert("GNF", 0);
    map.insert("RWF", 0);
    map.insert("BIF", 0);
    map.insert("XPF", 0);
    map.insert("XOF", 0);
    map.insert("XAF", 0);
    map.insert("KMF", 0);
    map.insert("DJF", 0);
    map.insert("UGX", 0);
    map.insert("MGA", 0);

    // Special cases with 3 decimals
    map.insert("BHD", 3);
    map.insert("JOD", 3);
    map.insert("KWD", 3);
    map.insert("OMR", 3);
    map.insert("TND", 3);
    map.insert("IQD", 3);
    map.insert("LYD", 3);

    map
});

/// Default decimal places for currencies not in the special cases map
pub const DEFAULT_CURRENCY_DECIMALS: u8 = 2;

/// Valid ISO 4217 currency codes
pub static VALID_CURRENCIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK", "DKK", "PLN", "CZK",
        "HUF", "RUB", "CNY", "HKD", "SGD", "KRW", "THB", "INR", "MYR", "PHP", "IDR", "VND", "BRL",
        "MXN", "ARS", "CLP", "COP", "PEN", "UYU", "ZAR", "EGP", "NGN", "GHS", "KES", "TZS", "UGX",
        "ZMW", "BWP", "MUR", "SCR", "SZL", "LSL", "NAD", "AOA", "XOF", "XAF", "MAD", "TND", "DZD",
        "LYD", "ILS", "JOD", "LBP", "SYP", "IQD", "IRR", "SAR", "AED", "QAR", "BHD", "KWD", "OMR",
        "YER", "AFN", "PKR", "LKR", "BDT", "BTN", "NPR", "MMK", "LAK", "KHR", "MNT", "KZT", "UZS",
        "KGS", "TJS", "TMT", "AZN", "GEL", "AMD", "BGN", "RON", "HRK", "RSD", "BAM", "MKD", "ALL",
        "MDL", "UAH", "BYN", "LTL", "LVL", "EEK", "ISK", "TRY", "PYG", "GNF", "RWF", "BIF", "XPF",
        "KMF", "DJF", "MGA", // Commodity currencies
        "XAU", "XAG", "XPD", "XPT",
    ]
    .iter()
    .copied()
    .collect()
});

/// Valid ISO 3166-1 alpha-2 country codes
pub static VALID_COUNTRY_CODES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
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

/// SWIFT format pattern specifications
#[derive(Debug, Clone)]
pub struct FormatSpec {
    pub pattern: &'static str,
    pub description: &'static str,
    pub regex: Option<&'static str>,
}

/// Common SWIFT format patterns
pub static FORMAT_PATTERNS: Lazy<HashMap<&'static str, FormatSpec>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Fixed length patterns
    map.insert(
        "3!a",
        FormatSpec {
            pattern: "3!a",
            description: "Exactly 3 uppercase letters",
            regex: Some(r"^[A-Z]{3}$"),
        },
    );

    map.insert(
        "4!a",
        FormatSpec {
            pattern: "4!a",
            description: "Exactly 4 uppercase letters",
            regex: Some(r"^[A-Z]{4}$"),
        },
    );

    map.insert(
        "6!n",
        FormatSpec {
            pattern: "6!n",
            description: "Exactly 6 digits",
            regex: Some(r"^\d{6}$"),
        },
    );

    map.insert(
        "4!c",
        FormatSpec {
            pattern: "4!c",
            description: "Exactly 4 SWIFT characters",
            regex: Some(r"^[A-Za-z0-9\s/:?,()+.'-]{4}$"),
        },
    );

    // Variable length patterns
    map.insert(
        "35x",
        FormatSpec {
            pattern: "35x",
            description: "Up to 35 characters (any SWIFT character)",
            regex: Some(r"^.{1,35}$"),
        },
    );

    map.insert(
        "16x",
        FormatSpec {
            pattern: "16x",
            description: "Up to 16 characters",
            regex: Some(r"^.{1,16}$"),
        },
    );

    // Decimal patterns
    map.insert(
        "15d",
        FormatSpec {
            pattern: "15d",
            description: "Decimal up to 15 digits total",
            regex: Some(r"^\d{1,12}[,]\d{0,2}$"),
        },
    );

    // Multi-line patterns
    map.insert(
        "4*35x",
        FormatSpec {
            pattern: "4*35x",
            description: "Up to 4 lines of 35 characters each",
            regex: None, // Too complex for simple regex
        },
    );

    // Date patterns
    map.insert(
        "YYMMDD",
        FormatSpec {
            pattern: "YYMMDD",
            description: "Date in YYMMDD format",
            regex: Some(r"^\d{6}$"),
        },
    );

    map.insert(
        "YYYYMMDD",
        FormatSpec {
            pattern: "YYYYMMDD",
            description: "Date in YYYYMMDD format",
            regex: Some(r"^\d{8}$"),
        },
    );

    map
});

/// BIC structure validation rules
pub const BIC_STRUCTURE: BICStructure = BICStructure {
    bank_code_length: 4,
    country_code_length: 2,
    location_code_length: 2,
    branch_code_length: 3, // Optional
    valid_lengths: &[8, 11],
};

#[derive(Debug)]
pub struct BICStructure {
    pub bank_code_length: usize,
    pub country_code_length: usize,
    pub location_code_length: usize,
    pub branch_code_length: usize,
    pub valid_lengths: &'static [usize],
}

/// Get decimal places for a currency
pub fn get_currency_decimals(currency: &str) -> u8 {
    CURRENCY_DECIMALS
        .get(currency)
        .copied()
        .unwrap_or(DEFAULT_CURRENCY_DECIMALS)
}

/// Check if a currency code is valid
pub fn is_valid_currency(currency: &str) -> bool {
    VALID_CURRENCIES.contains(currency)
}

/// Check if a country code is valid
pub fn is_valid_country_code(code: &str) -> bool {
    VALID_COUNTRY_CODES.contains(code)
}

/// Validate BIC structure (basic validation without full checks)
pub fn is_valid_bic_structure(bic: &str) -> bool {
    if !BIC_STRUCTURE.valid_lengths.contains(&bic.len()) {
        return false;
    }

    // Check bank code (4 letters)
    let bank_code = &bic[0..4];
    if !bank_code.chars().all(|c| c.is_ascii_uppercase()) {
        return false;
    }

    // Check country code (2 letters)
    let country_code = &bic[4..6];
    if !is_valid_country_code(country_code) {
        return false;
    }

    // Check location code (2 alphanumeric)
    let location_code = &bic[6..8];
    if !location_code.chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }

    // If 11 characters, check branch code (3 alphanumeric)
    if bic.len() == 11 {
        let branch_code = &bic[8..11];
        if !branch_code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }
    }

    true
}

/// Common date format validation patterns
pub mod date_formats {
    /// Validate YYMMDD format
    pub fn is_valid_yymmdd(date: &str) -> bool {
        if date.len() != 6 || !date.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        // Basic month/day validation
        if let Ok(month) = date[2..4].parse::<u32>() {
            if !(1..=12).contains(&month) {
                return false;
            }
        }

        if let Ok(day) = date[4..6].parse::<u32>() {
            if !(1..=31).contains(&day) {
                return false;
            }
        }

        true
    }

    /// Validate YYYYMMDD format
    pub fn is_valid_yyyymmdd(date: &str) -> bool {
        if date.len() != 8 || !date.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        // Basic month/day validation
        if let Ok(month) = date[4..6].parse::<u32>() {
            if !(1..=12).contains(&month) {
                return false;
            }
        }

        if let Ok(day) = date[6..8].parse::<u32>() {
            if !(1..=31).contains(&day) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_decimals() {
        assert_eq!(get_currency_decimals("USD"), 2);
        assert_eq!(get_currency_decimals("EUR"), 2);
        assert_eq!(get_currency_decimals("JPY"), 0);
        assert_eq!(get_currency_decimals("BHD"), 3);
        assert_eq!(get_currency_decimals("XYZ"), 2); // Unknown defaults to 2
    }

    #[test]
    fn test_bic_validation() {
        assert!(is_valid_bic_structure("DEUTDEFF"));
        assert!(is_valid_bic_structure("DEUTDEFFXXX"));
        assert!(!is_valid_bic_structure("DEUT")); // Too short
        assert!(!is_valid_bic_structure("DEUTDEFFXXXX")); // Wrong length
        assert!(!is_valid_bic_structure("DEU1DEFF")); // Number in bank code
        assert!(!is_valid_bic_structure("DEUTZZFF")); // Invalid country
    }

    #[test]
    fn test_date_validation() {
        assert!(date_formats::is_valid_yymmdd("240315"));
        assert!(!date_formats::is_valid_yymmdd("241315")); // Invalid month
        assert!(!date_formats::is_valid_yymmdd("240332")); // Invalid day

        assert!(date_formats::is_valid_yyyymmdd("20240315"));
        assert!(!date_formats::is_valid_yyyymmdd("20241315")); // Invalid month
    }
}
