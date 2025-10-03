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
}
