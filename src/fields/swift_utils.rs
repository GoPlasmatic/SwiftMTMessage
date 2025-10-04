//! # Core SWIFT Utility Functions
//!
//! Low-level parsing utilities for SWIFT MT message primitive data types.
//! These utilities handle basic SWIFT data formats like BIC codes, currency codes,
//! dates, amounts, and character validation.

use crate::errors::ParseError;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

/// Parse a string with exact length requirement
pub fn parse_exact_length(
    input: &str,
    expected_len: usize,
    field_name: &str,
) -> Result<String, ParseError> {
    if input.len() != expected_len {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "{} must be exactly {} characters, found {}",
                field_name,
                expected_len,
                input.len()
            ),
        });
    }
    Ok(input.to_string())
}

/// Parse a string with maximum length limit
pub fn parse_max_length(
    input: &str,
    max_len: usize,
    field_name: &str,
) -> Result<String, ParseError> {
    if input.len() > max_len {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "{} must be at most {} characters, found {}",
                field_name,
                max_len,
                input.len()
            ),
        });
    }
    Ok(input.to_string())
}

/// Parse a string with minimum and maximum length
pub fn parse_length_range(
    input: &str,
    min_len: usize,
    max_len: usize,
    field_name: &str,
) -> Result<String, ParseError> {
    if input.len() < min_len || input.len() > max_len {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "{} must be between {} and {} characters, found {}",
                field_name,
                min_len,
                max_len,
                input.len()
            ),
        });
    }
    Ok(input.to_string())
}

/// Parse alphanumeric string (letters and digits only)
pub fn parse_alphanumeric(input: &str, field_name: &str) -> Result<String, ParseError> {
    if !input.chars().all(|c| c.is_alphanumeric()) {
        return Err(ParseError::InvalidFormat {
            message: format!("{} must contain only letters and digits", field_name),
        });
    }
    Ok(input.to_string())
}

/// Parse uppercase letters only
pub fn parse_uppercase(input: &str, field_name: &str) -> Result<String, ParseError> {
    if !input.chars().all(|c| c.is_uppercase() || c.is_whitespace()) {
        return Err(ParseError::InvalidFormat {
            message: format!("{} must contain only uppercase letters", field_name),
        });
    }
    Ok(input.to_string())
}

/// Parse numeric string (digits only)
pub fn parse_numeric(input: &str, field_name: &str) -> Result<String, ParseError> {
    if !input.chars().all(|c| c.is_numeric()) {
        return Err(ParseError::InvalidFormat {
            message: format!("{} must contain only digits", field_name),
        });
    }
    Ok(input.to_string())
}

/// Parse SWIFT digits format (digits only, used for numeric fields)
pub fn parse_swift_digits(input: &str, field_name: &str) -> Result<String, ParseError> {
    if !input.chars().all(|c| c.is_ascii_digit()) {
        return Err(ParseError::InvalidFormat {
            message: format!("{} must contain only digits", field_name),
        });
    }
    Ok(input.to_string())
}

/// Parse SWIFT character set (a-z, A-Z, 0-9, and special chars)
///
/// SWIFT 'x' character set includes: a-z, A-Z, 0-9, and special characters:
/// / - ? : ( ) . , ' + { } SPACE CR LF and other printable ASCII
pub fn parse_swift_chars(input: &str, field_name: &str) -> Result<String, ParseError> {
    // SWIFT x character set: alphanumeric + special characters
    // Common special chars: / - ? : ( ) . , ' + { } SPACE CR LF % & * ; < = > @ [ ] _ $ ! " # |
    const SWIFT_SPECIAL: &str = "/-?:().,'+{} \r\n%&*;<=>@[]_$!\"#|";

    if !input
        .chars()
        .all(|c| c.is_alphanumeric() || SWIFT_SPECIAL.contains(c))
    {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "{} contains invalid characters for SWIFT format",
                field_name
            ),
        });
    }
    Ok(input.to_string())
}

/// Parse BIC code (8 or 11 characters)
pub fn parse_bic(input: &str) -> Result<String, ParseError> {
    if input.len() != 8 && input.len() != 11 {
        return Err(ParseError::InvalidFormat {
            message: format!("BIC must be 8 or 11 characters, found {}", input.len()),
        });
    }

    // First 4 chars: Bank code (letters)
    if !input[0..4].chars().all(|c| c.is_alphabetic()) {
        return Err(ParseError::InvalidFormat {
            message: "BIC bank code (first 4 chars) must be letters".to_string(),
        });
    }

    // Next 2 chars: Country code (letters)
    if !input[4..6].chars().all(|c| c.is_alphabetic()) {
        return Err(ParseError::InvalidFormat {
            message: "BIC country code (chars 5-6) must be letters".to_string(),
        });
    }

    // Next 2 chars: Location code (alphanumeric)
    if !input[6..8].chars().all(|c| c.is_alphanumeric()) {
        return Err(ParseError::InvalidFormat {
            message: "BIC location code (chars 7-8) must be alphanumeric".to_string(),
        });
    }

    // Optional 3 chars: Branch code (alphanumeric)
    if input.len() == 11 && !input[8..11].chars().all(|c| c.is_alphanumeric()) {
        return Err(ParseError::InvalidFormat {
            message: "BIC branch code (chars 9-11) must be alphanumeric".to_string(),
        });
    }

    Ok(input.to_string())
}

/// Parse account number (max 34 characters)
pub fn parse_account(input: &str) -> Result<String, ParseError> {
    parse_max_length(input, 34, "Account")?;
    parse_swift_chars(input, "Account")?;
    Ok(input.to_string())
}

/// Get the number of decimal places for a currency according to ISO 4217
///
/// Returns the standard number of decimal places for the given currency code.
/// Most currencies use 2 decimal places, but there are notable exceptions.
///
/// # Examples
/// - JPY, KRW: 0 decimal places (yen, won are not subdivided)
/// - BHD, KWD, OMR, TND: 3 decimal places (dinars subdivided into 1000 fils)
/// - Most others: 2 decimal places (USD, EUR, GBP, etc.)
///
/// # Arguments
/// * `currency` - ISO 4217 three-letter currency code
///
/// # Returns
/// Number of decimal places (0, 2, or 3)
pub fn get_currency_decimals(currency: &str) -> u8 {
    match currency {
        // Zero decimal currencies (not subdivided)
        "BIF" | // Burundian Franc
        "CLP" | // Chilean Peso
        "DJF" | // Djiboutian Franc
        "GNF" | // Guinean Franc
        "ISK" | // Icelandic Króna
        "JPY" | // Japanese Yen
        "KMF" | // Comorian Franc
        "KRW" | // South Korean Won
        "PYG" | // Paraguayan Guaraní
        "RWF" | // Rwandan Franc
        "UGX" | // Ugandan Shilling
        "UYI" | // Uruguay Peso en Unidades Indexadas
        "VND" | // Vietnamese Đồng
        "VUV" | // Vanuatu Vatu
        "XAF" | // Central African CFA Franc
        "XOF" | // West African CFA Franc
        "XPF"   // CFP Franc
        => 0,

        // Three decimal currencies (subdivided into 1000)
        "BHD" | // Bahraini Dinar
        "IQD" | // Iraqi Dinar
        "JOD" | // Jordanian Dinar
        "KWD" | // Kuwaiti Dinar
        "LYD" | // Libyan Dinar
        "OMR" | // Omani Rial
        "TND"   // Tunisian Dinar
        => 3,

        // Four decimal currencies (rare)
        "CLF" | // Unidad de Fomento (Chile)
        "UYW"   // Unidad Previsional (Uruguay)
        => 4,

        // Default: two decimal places (USD, EUR, GBP, CHF, CAD, AUD, etc.)
        _ => 2,
    }
}

/// Commodity currency codes that are not allowed in payment messages (C08 validation)
const COMMODITY_CURRENCIES: &[&str] = &[
    "XAU", // Gold
    "XAG", // Silver
    "XPD", // Palladium
    "XPT", // Platinum
];

/// Validate that currency is not a commodity code (C08 validation)
///
/// SWIFT network validation rule C08 prohibits the use of commodity currency codes
/// (XAU, XAG, XPD, XPT) in payment message amount fields.
///
/// # Arguments
/// * `currency` - ISO 4217 currency code to validate
///
/// # Returns
/// Ok(()) if valid, Err(ParseError) if commodity currency
///
/// # Errors
/// Returns ParseError::InvalidFormat with C08 error if commodity currency detected
pub fn validate_non_commodity_currency(currency: &str) -> Result<(), ParseError> {
    if COMMODITY_CURRENCIES.contains(&currency) {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Commodity currency code {} not allowed in payment messages (Error code: C08)",
                currency
            ),
        });
    }
    Ok(())
}

/// Parse currency code (3 uppercase letters)
pub fn parse_currency(input: &str) -> Result<String, ParseError> {
    if input.len() != 3 {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Currency code must be exactly 3 characters, found {}",
                input.len()
            ),
        });
    }

    if !input.chars().all(|c| c.is_uppercase()) {
        return Err(ParseError::InvalidFormat {
            message: "Currency code must be uppercase letters".to_string(),
        });
    }

    Ok(input.to_string())
}

/// Parse currency code with commodity validation (enforces C08 rule)
///
/// This is a stricter version of parse_currency that also validates against
/// commodity currencies. Use this for amount fields (32A, 32B, 33B, 71F, 71G, etc.)
///
/// # Arguments
/// * `input` - Currency code string to parse
///
/// # Returns
/// Validated currency code string
///
/// # Errors
/// Returns error if:
/// - Not exactly 3 characters (T52)
/// - Contains non-uppercase letters (T52)
/// - Is a commodity currency code (C08)
pub fn parse_currency_non_commodity(input: &str) -> Result<String, ParseError> {
    let currency = parse_currency(input)?;
    validate_non_commodity_currency(&currency)?;
    Ok(currency)
}

/// Parse amount with optional decimal places
pub fn parse_amount(input: &str) -> Result<f64, ParseError> {
    // Remove any commas (European decimal separator handling)
    let normalized = input.replace(',', ".");

    normalized
        .parse::<f64>()
        .map_err(|e| ParseError::InvalidFormat {
            message: format!("Invalid amount format: {}", e),
        })
}

/// Validate amount decimal precision for a specific currency (C03 validation)
///
/// SWIFT network validation rule C03 requires that the number of decimal places
/// in an amount must not exceed the maximum allowed for the currency.
///
/// # Arguments
/// * `amount` - The amount value to validate
/// * `currency` - ISO 4217 currency code
///
/// # Returns
/// Ok(()) if decimal precision is valid, Err(ParseError) if exceeds limit
///
/// # Errors
/// Returns ParseError::InvalidFormat with C03 error if decimal precision exceeded
///
/// # Examples
/// ```
/// validate_amount_decimals(100.50, "USD") // Ok - 2 decimals allowed
/// validate_amount_decimals(100.0, "JPY")  // Ok - 0 decimals allowed
/// validate_amount_decimals(100.50, "JPY") // Err - JPY allows 0 decimals only
/// validate_amount_decimals(100.505, "BHD") // Err - BHD allows 3 decimals max
/// ```
pub fn validate_amount_decimals(amount: f64, currency: &str) -> Result<(), ParseError> {
    let max_decimals = get_currency_decimals(currency);

    // Calculate actual decimal places in the amount
    // Use string representation to avoid floating point precision issues
    let amount_str = format!("{:.10}", amount); // Format with high precision
    let decimal_places = if let Some(dot_pos) = amount_str.find('.') {
        let after_dot = &amount_str[dot_pos + 1..];
        // Count non-zero digits after decimal point
        after_dot.trim_end_matches('0').len()
    } else {
        0
    };

    if decimal_places > max_decimals as usize {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Amount has {} decimal places but currency {} allows maximum {} (Error code: C03)",
                decimal_places, currency, max_decimals
            ),
        });
    }

    Ok(())
}

/// Parse amount with currency-specific decimal validation
///
/// This combines amount parsing with currency-specific decimal precision validation.
/// Use this for amount fields where the currency is known (Field 32A, 32B, etc.)
///
/// # Arguments
/// * `input` - Amount string to parse
/// * `currency` - ISO 4217 currency code for decimal validation
///
/// # Returns
/// Parsed amount as f64
///
/// # Errors
/// Returns error if:
/// - Amount format is invalid
/// - Decimal precision exceeds currency limit (C03)
pub fn parse_amount_with_currency(input: &str, currency: &str) -> Result<f64, ParseError> {
    let amount = parse_amount(input)?;
    validate_amount_decimals(amount, currency)?;
    Ok(amount)
}

/// Format amount for SWIFT output with comma decimal separator
///
/// This function ensures SWIFT-compliant amount formatting:
/// - Uses comma (,) as decimal separator instead of period (.)
/// - Maintains proper decimal precision (typically 2 decimal places)
/// - Ensures at least one digit in the integer part
/// - Removes trailing zeros after decimal for cleaner output
///
/// # Arguments
/// * `amount` - The amount to format
/// * `decimals` - Number of decimal places (typically 2 for most currencies)
///
/// # Returns
/// SWIFT-formatted amount string with comma separator
///
/// # Examples
/// ```
/// assert_eq!(format_swift_amount(1234.56, 2), "1234,56");
/// assert_eq!(format_swift_amount(1000.00, 2), "1000");
/// assert_eq!(format_swift_amount(1000.50, 2), "1000,5");
/// ```
pub fn format_swift_amount(amount: f64, decimals: usize) -> String {
    let formatted = format!("{:.width$}", amount, width = decimals);
    let with_comma = formatted.replace('.', ",");

    // Remove trailing zeros after comma for cleaner output
    // e.g., "1000,00" -> "1000", "1000,50" -> "1000,5"
    if with_comma.contains(',') {
        let trimmed = with_comma.trim_end_matches('0');
        if trimmed.ends_with(',') {
            trimmed.trim_end_matches(',').to_string()
        } else {
            trimmed.to_string()
        }
    } else {
        with_comma
    }
}

/// Format amount for SWIFT output with currency-specific decimal precision
///
/// This is a currency-aware version of format_swift_amount that automatically
/// determines the correct number of decimal places based on the currency code.
///
/// # Arguments
/// * `amount` - The amount to format
/// * `currency` - ISO 4217 currency code
///
/// # Returns
/// SWIFT-formatted amount string with currency-appropriate precision
///
/// # Examples
/// ```
/// assert_eq!(format_swift_amount_for_currency(1234.56, "USD"), "1234,56");
/// assert_eq!(format_swift_amount_for_currency(1500000.0, "JPY"), "1500000");
/// assert_eq!(format_swift_amount_for_currency(123.456, "BHD"), "123,456");
/// ```
pub fn format_swift_amount_for_currency(amount: f64, currency: &str) -> String {
    let decimals = get_currency_decimals(currency);
    format_swift_amount(amount, decimals as usize)
}

/// Parse date in YYMMDD format
pub fn parse_date_yymmdd(input: &str) -> Result<NaiveDate, ParseError> {
    if input.len() != 6 {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Date must be in YYMMDD format (6 digits), found {} characters",
                input.len()
            ),
        });
    }

    let year = input[0..2]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid year in date".to_string(),
        })?;
    let month = input[2..4]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid month in date".to_string(),
        })?;
    let day = input[4..6]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid day in date".to_string(),
        })?;

    // Determine century: 00-49 -> 2000-2049, 50-99 -> 1950-1999
    let full_year = if year <= 49 { 2000 + year } else { 1900 + year };

    NaiveDate::from_ymd_opt(full_year as i32, month, day).ok_or_else(|| ParseError::InvalidFormat {
        message: format!("Invalid date: {}/{}/{}", full_year, month, day),
    })
}

/// Parse date in YYYYMMDD format
pub fn parse_date_yyyymmdd(input: &str) -> Result<NaiveDate, ParseError> {
    if input.len() != 8 {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Date must be in YYYYMMDD format (8 digits), found {} characters",
                input.len()
            ),
        });
    }

    let year = input[0..4]
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid year in date".to_string(),
        })?;
    let month = input[4..6]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid month in date".to_string(),
        })?;
    let day = input[6..8]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid day in date".to_string(),
        })?;

    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| ParseError::InvalidFormat {
        message: format!("Invalid date: {}/{}/{}", year, month, day),
    })
}

/// Parse time in HHMM format
pub fn parse_time_hhmm(input: &str) -> Result<NaiveTime, ParseError> {
    if input.len() != 4 {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Time must be in HHMM format (4 digits), found {} characters",
                input.len()
            ),
        });
    }

    let hour = input[0..2]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid hour in time".to_string(),
        })?;
    let minute = input[2..4]
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat {
            message: "Invalid minute in time".to_string(),
        })?;

    NaiveTime::from_hms_opt(hour, minute, 0).ok_or_else(|| ParseError::InvalidFormat {
        message: format!("Invalid time: {}:{}", hour, minute),
    })
}

/// Parse datetime in YYMMDDHHMM format
pub fn parse_datetime_yymmddhhmm(input: &str) -> Result<NaiveDateTime, ParseError> {
    if input.len() != 10 {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "DateTime must be in YYMMDDHHMM format (10 digits), found {} characters",
                input.len()
            ),
        });
    }

    let date = parse_date_yymmdd(&input[0..6])?;
    let time = parse_time_hhmm(&input[6..10])?;

    Ok(NaiveDateTime::new(date, time))
}

/// Parse reference (16x - alphanumeric, max 16 chars)
pub fn parse_reference(input: &str) -> Result<String, ParseError> {
    parse_max_length(input, 16, "Reference")?;
    parse_swift_chars(input, "Reference")?;
    Ok(input.to_string())
}

/// Split input at first occurrence of delimiter
pub fn split_at_first(input: &str, delimiter: char) -> (String, Option<String>) {
    if let Some(pos) = input.find(delimiter) {
        let (first, rest) = input.split_at(pos);
        let rest = &rest[1..]; // Skip the delimiter
        (
            first.to_string(),
            if rest.is_empty() {
                None
            } else {
                Some(rest.to_string())
            },
        )
    } else {
        (input.to_string(), None)
    }
}

/// Split input at first newline
pub fn split_at_newline(input: &str) -> (String, Option<String>) {
    if let Some(pos) = input.find('\n') {
        let (first, rest) = input.split_at(pos);
        let rest = &rest[1..]; // Skip the newline
        (
            first.to_string(),
            if rest.is_empty() {
                None
            } else {
                Some(rest.to_string())
            },
        )
    } else {
        (input.to_string(), None)
    }
}

/// Clean and normalize text (remove extra whitespace, normalize line endings)
pub fn normalize_text(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Validate IBAN format
pub fn validate_iban(iban: &str) -> Result<(), ParseError> {
    // Basic IBAN validation (simplified)
    if iban.len() < 15 || iban.len() > 34 {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "IBAN must be between 15 and 34 characters, found {}",
                iban.len()
            ),
        });
    }

    // First two characters must be country code (letters)
    if !iban[0..2].chars().all(|c| c.is_uppercase()) {
        return Err(ParseError::InvalidFormat {
            message: "IBAN country code must be uppercase letters".to_string(),
        });
    }

    // Next two characters must be check digits
    if !iban[2..4].chars().all(|c| c.is_numeric()) {
        return Err(ParseError::InvalidFormat {
            message: "IBAN check digits must be numeric".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_exact_length() {
        assert!(parse_exact_length("ABC", 3, "Test").is_ok());
        assert!(parse_exact_length("AB", 3, "Test").is_err());
        assert!(parse_exact_length("ABCD", 3, "Test").is_err());
    }

    #[test]
    fn test_parse_bic() {
        assert!(parse_bic("DEUTDEFF").is_ok());
        assert!(parse_bic("DEUTDEFFXXX").is_ok());
        assert!(parse_bic("DEUT").is_err()); // Too short
        assert!(parse_bic("DEUTDEFFXX").is_err()); // Wrong length
        assert!(parse_bic("1234DEFF").is_err()); // Invalid bank code
    }

    #[test]
    fn test_parse_currency() {
        assert!(parse_currency("USD").is_ok());
        assert!(parse_currency("EUR").is_ok());
        assert!(parse_currency("US").is_err()); // Too short
        assert!(parse_currency("usd").is_err()); // Not uppercase
    }

    #[test]
    fn test_parse_date_yymmdd() {
        let date = parse_date_yymmdd("231225").unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 25);

        let date = parse_date_yymmdd("991231").unwrap();
        assert_eq!(date.year(), 1999);
    }

    #[test]
    fn test_parse_amount() {
        assert!(parse_amount("1234.56").is_ok());
        assert!(parse_amount("1234,56").is_ok()); // European format
        assert!(parse_amount("1234").is_ok());
        assert!(parse_amount("abc").is_err());
    }

    #[test]
    fn test_format_swift_amount() {
        // Test standard 2 decimal formatting
        assert_eq!(format_swift_amount(1234.56, 2), "1234,56");
        assert_eq!(format_swift_amount(1000.00, 2), "1000");
        assert_eq!(format_swift_amount(1000.50, 2), "1000,5");

        // Test trailing zero removal
        assert_eq!(format_swift_amount(5000.00, 2), "5000");
        assert_eq!(format_swift_amount(2500.00, 2), "2500");

        // Test with single decimal
        assert_eq!(format_swift_amount(250.75, 2), "250,75");
        assert_eq!(format_swift_amount(99.99, 2), "99,99");

        // Test large amounts
        assert_eq!(format_swift_amount(1000000.0, 2), "1000000");
        assert_eq!(format_swift_amount(1234567.89, 2), "1234567,89");

        // Test zero decimals (for currencies like JPY)
        assert_eq!(format_swift_amount(1500000.0, 0), "1500000");

        // Test three decimals (for currencies like BHD)
        assert_eq!(format_swift_amount(123.456, 3), "123,456");
    }

    #[test]
    fn test_split_at_first() {
        let (first, rest) = split_at_first("ABC/DEF/GHI", '/');
        assert_eq!(first, "ABC");
        assert_eq!(rest, Some("DEF/GHI".to_string()));

        let (first, rest) = split_at_first("ABCDEF", '/');
        assert_eq!(first, "ABCDEF");
        assert_eq!(rest, None);
    }

    #[test]
    fn test_validate_iban() {
        assert!(validate_iban("DE89370400440532013000").is_ok());
        assert!(validate_iban("GB82WEST12345698765432").is_ok());
        assert!(validate_iban("DE89").is_err()); // Too short
        assert!(validate_iban("1234567890123456").is_err()); // Invalid country code
    }

    #[test]
    fn test_get_currency_decimals() {
        // Test zero decimal currencies
        assert_eq!(get_currency_decimals("JPY"), 0);
        assert_eq!(get_currency_decimals("KRW"), 0);
        assert_eq!(get_currency_decimals("VND"), 0);
        assert_eq!(get_currency_decimals("CLP"), 0);

        // Test two decimal currencies (default)
        assert_eq!(get_currency_decimals("USD"), 2);
        assert_eq!(get_currency_decimals("EUR"), 2);
        assert_eq!(get_currency_decimals("GBP"), 2);
        assert_eq!(get_currency_decimals("CHF"), 2);

        // Test three decimal currencies
        assert_eq!(get_currency_decimals("BHD"), 3);
        assert_eq!(get_currency_decimals("KWD"), 3);
        assert_eq!(get_currency_decimals("OMR"), 3);
        assert_eq!(get_currency_decimals("TND"), 3);

        // Test four decimal currencies
        assert_eq!(get_currency_decimals("CLF"), 4);

        // Test unknown currency (defaults to 2)
        assert_eq!(get_currency_decimals("XXX"), 2);
    }

    #[test]
    fn test_validate_non_commodity_currency() {
        // Valid non-commodity currencies
        assert!(validate_non_commodity_currency("USD").is_ok());
        assert!(validate_non_commodity_currency("EUR").is_ok());
        assert!(validate_non_commodity_currency("JPY").is_ok());

        // Invalid commodity currencies (C08)
        assert!(validate_non_commodity_currency("XAU").is_err()); // Gold
        assert!(validate_non_commodity_currency("XAG").is_err()); // Silver
        assert!(validate_non_commodity_currency("XPD").is_err()); // Palladium
        assert!(validate_non_commodity_currency("XPT").is_err()); // Platinum

        // Verify error message contains C08
        let err = validate_non_commodity_currency("XAU").unwrap_err();
        if let ParseError::InvalidFormat { message } = err {
            assert!(message.contains("C08"));
            assert!(message.contains("XAU"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_parse_currency_non_commodity() {
        // Valid non-commodity currencies
        assert_eq!(parse_currency_non_commodity("USD").unwrap(), "USD");
        assert_eq!(parse_currency_non_commodity("EUR").unwrap(), "EUR");
        assert_eq!(parse_currency_non_commodity("JPY").unwrap(), "JPY");

        // Invalid commodity currencies
        assert!(parse_currency_non_commodity("XAU").is_err());
        assert!(parse_currency_non_commodity("XAG").is_err());

        // Invalid format
        assert!(parse_currency_non_commodity("US").is_err()); // Too short
        assert!(parse_currency_non_commodity("usd").is_err()); // Lowercase
    }

    #[test]
    fn test_validate_amount_decimals() {
        // USD (2 decimals allowed)
        assert!(validate_amount_decimals(100.0, "USD").is_ok());
        assert!(validate_amount_decimals(100.5, "USD").is_ok());
        assert!(validate_amount_decimals(100.50, "USD").is_ok());
        assert!(validate_amount_decimals(100.505, "USD").is_err()); // 3 decimals

        // JPY (0 decimals allowed)
        assert!(validate_amount_decimals(100.0, "JPY").is_ok());
        assert!(validate_amount_decimals(1500000.0, "JPY").is_ok());
        assert!(validate_amount_decimals(100.5, "JPY").is_err()); // Has decimals
        assert!(validate_amount_decimals(100.50, "JPY").is_err()); // Has decimals

        // BHD (3 decimals allowed)
        assert!(validate_amount_decimals(100.0, "BHD").is_ok());
        assert!(validate_amount_decimals(100.5, "BHD").is_ok());
        assert!(validate_amount_decimals(100.505, "BHD").is_ok());
        assert!(validate_amount_decimals(100.5055, "BHD").is_err()); // 4 decimals

        // Verify error message contains C03
        let err = validate_amount_decimals(100.505, "USD").unwrap_err();
        if let ParseError::InvalidFormat { message } = err {
            assert!(message.contains("C03"));
            assert!(message.contains("USD"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_parse_amount_with_currency() {
        // Valid amounts with correct decimals
        assert_eq!(parse_amount_with_currency("100.50", "USD").unwrap(), 100.50);
        assert_eq!(
            parse_amount_with_currency("1500000", "JPY").unwrap(),
            1500000.0
        );
        assert_eq!(
            parse_amount_with_currency("100.505", "BHD").unwrap(),
            100.505
        );

        // European format (comma separator)
        assert_eq!(parse_amount_with_currency("100,50", "EUR").unwrap(), 100.50);

        // Invalid: too many decimals
        assert!(parse_amount_with_currency("100.505", "USD").is_err());
        assert!(parse_amount_with_currency("100.5", "JPY").is_err());
        assert!(parse_amount_with_currency("100.5055", "BHD").is_err());

        // Invalid format
        assert!(parse_amount_with_currency("abc", "USD").is_err());
    }

    #[test]
    fn test_format_swift_amount_for_currency() {
        // USD (2 decimals)
        assert_eq!(format_swift_amount_for_currency(1234.56, "USD"), "1234,56");
        assert_eq!(format_swift_amount_for_currency(1000.00, "USD"), "1000");
        assert_eq!(format_swift_amount_for_currency(1000.50, "USD"), "1000,5");

        // JPY (0 decimals)
        assert_eq!(
            format_swift_amount_for_currency(1500000.0, "JPY"),
            "1500000"
        );
        assert_eq!(format_swift_amount_for_currency(1234.0, "JPY"), "1234");

        // BHD (3 decimals)
        assert_eq!(format_swift_amount_for_currency(123.456, "BHD"), "123,456");
        assert_eq!(format_swift_amount_for_currency(100.5, "BHD"), "100,5");
        assert_eq!(format_swift_amount_for_currency(100.0, "BHD"), "100");

        // EUR (2 decimals)
        assert_eq!(format_swift_amount_for_currency(5000.00, "EUR"), "5000");
        assert_eq!(format_swift_amount_for_currency(250.75, "EUR"), "250,75");
    }
}
