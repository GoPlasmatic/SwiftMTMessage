//! Sample generation utilities for SWIFT MT messages and fields

use rand::Rng;
use std::collections::HashMap;

/// Configuration for generating field samples
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FieldConfig {
    /// Preferred length for generated strings
    pub length_preference: Option<LengthPreference>,
    /// Constrain generated values
    pub value_range: Option<ValueRange>,
    /// Fixed values to choose from
    pub fixed_values: Option<Vec<String>>,
    /// Regex pattern to match (for validation)
    pub pattern: Option<String>,
}

/// Configuration for generating message samples
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MessageConfig {
    /// Whether to include optional fields
    pub include_optional: bool,
    /// Field-specific configurations
    pub field_configs: HashMap<String, FieldConfig>,
    /// Predefined scenario to use
    pub scenario: Option<MessageScenario>,
}

/// Predefined message generation scenarios
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageScenario {
    /// Basic compliant message
    Standard,
    /// Straight Through Processing compliant
    StpCompliant,
    /// Cover payment message format
    CoverPayment,
    /// Only mandatory fields
    Minimal,
    /// All fields populated
    Full,
}

/// Length generation preferences
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LengthPreference {
    /// Generate exactly N characters
    Exact(usize),
    /// Generate between min and max characters
    Range(usize, usize),
    /// Prefer shorter values
    Short,
    /// Prefer longer values
    Long,
}

/// Value constraints for generation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ValueRange {
    /// Constrain amount values
    Amount {
        min: f64,
        max: f64,
        currency: Option<String>,
    },
    /// Constrain date values
    Date { start: String, end: String },
    /// Constrain integer values
    Integer { min: i64, max: i64 },
}

/// Generate random numeric string of specified length
pub fn generate_numeric(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

/// Generate random alphabetic string of specified length (uppercase)
pub fn generate_alphabetic(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let ch = rng.gen_range(0..26) as u8 + b'A';
            ch as char
        })
        .collect()
}

/// Generate random alphanumeric string of specified length
pub fn generate_alphanumeric(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Generate string with any SWIFT-allowed character
pub fn generate_any_character(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-?:().,'+"
        .chars()
        .collect();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Generate decimal number with specified total length and decimal places
pub fn generate_decimal(length: usize, decimals: usize) -> String {
    if decimals >= length {
        return "0,00".to_string();
    }

    let integer_part_len = length - decimals - 1; // -1 for comma
    let integer_part = generate_numeric(integer_part_len);
    let decimal_part = generate_numeric(decimals);

    format!("{integer_part},{decimal_part}")
}

/// Generate a valid BIC code
pub fn generate_valid_bic() -> String {
    let mut rng = rand::thread_rng();
    let bics = [
        "ABNANL2A", "DEUTDEFF", "CHASUS33", "BOFAUS3N", "CITIUS33", "HSBCGB2L", "BNPAFRPP",
        "UBSWCHZH", "SCBLSGSG", "DBSSSGSG",
    ];
    bics[rng.gen_range(0..bics.len())].to_string()
}

/// Generate a valid currency code
pub fn generate_valid_currency() -> String {
    let mut rng = rand::thread_rng();
    let currencies = vec![
        "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK", "DKK", "SGD", "HKD",
        "CNY", "INR", "KRW", "MXN", "BRL", "ZAR", "AED",
    ];
    currencies[rng.gen_range(0..currencies.len())].to_string()
}

/// Generate a valid country code
pub fn generate_valid_country_code() -> String {
    let mut rng = rand::thread_rng();
    let countries = vec![
        "US", "GB", "DE", "FR", "IT", "ES", "NL", "BE", "CH", "AT", "JP", "CN", "IN", "AU", "CA",
        "BR", "MX", "SG", "HK", "KR",
    ];
    countries[rng.gen_range(0..countries.len())].to_string()
}

/// Generate a valid date in YYMMDD format
pub fn generate_date_yymmdd() -> String {
    let mut rng = rand::thread_rng();
    let year = rng.gen_range(20..30);
    let month = rng.gen_range(1..=12);
    let day = match month {
        2 => rng.gen_range(1..=28),
        4 | 6 | 9 | 11 => rng.gen_range(1..=30),
        _ => rng.gen_range(1..=31),
    };
    format!("{year:02}{month:02}{day:02}")
}

/// Generate a valid date in YYYYMMDD format
pub fn generate_date_yyyymmdd() -> String {
    let mut rng = rand::thread_rng();
    let year = rng.gen_range(2020..2030);
    let month = rng.gen_range(1..=12);
    let day = match month {
        2 => rng.gen_range(1..=28),
        4 | 6 | 9 | 11 => rng.gen_range(1..=30),
        _ => rng.gen_range(1..=31),
    };
    format!("{year:04}{month:02}{day:02}")
}

/// Generate a valid time in HHMM format
pub fn generate_time_hhmm() -> String {
    let mut rng = rand::thread_rng();
    let hour = rng.gen_range(0..24);
    let minute = rng.gen_range(0..60);
    format!("{hour:02}{minute:02}")
}

/// Generate a value based on SWIFT format specification
pub fn generate_by_format_spec(format: &str) -> String {
    // Parse format like "3!a", "6!n", "16x", "15d"
    let mut chars = format.chars().peekable();
    let mut length_str = String::new();
    let mut is_exact = false;
    let mut char_type = 'x';

    // Parse length
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            length_str.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    // Parse exact indicator
    if chars.peek() == Some(&'!') {
        is_exact = true;
        chars.next();
    }

    // Parse character type
    if let Some(ch) = chars.next() {
        char_type = ch;
    }

    let max_length: usize = length_str.parse().unwrap_or(1);
    let length = if is_exact {
        max_length
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=max_length)
    };

    match char_type {
        'n' => generate_numeric(length),
        'a' => generate_alphabetic(length),
        'c' => generate_alphanumeric(length),
        'd' => {
            // For decimal format, assume 2 decimal places if not specified
            let decimals = 2;
            generate_decimal(length, decimals)
        }
        _ => generate_any_character(length),
    }
}

/// Generate an account number (max 34 characters)
pub fn generate_account_number() -> String {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(10..=34);
    generate_alphanumeric(length)
}

/// Generate a reference number (16 characters)
pub fn generate_reference() -> String {
    generate_alphanumeric(16)
}

/// Generate a transaction code
pub fn generate_transaction_code() -> String {
    let mut rng = rand::thread_rng();
    let codes = ["NTRF", "CHQB", "PMNT", "MCOP", "DMCT"];
    codes[rng.gen_range(0..codes.len())].to_string()
}

/// Generate a bank operation code
pub fn generate_bank_operation_code() -> String {
    let mut rng = rand::thread_rng();
    let codes = ["CRED", "CRTS", "SPAY", "SSTD"];
    codes[rng.gen_range(0..codes.len())].to_string()
}

/// Generate a details of charges code
pub fn generate_details_of_charges() -> String {
    let mut rng = rand::thread_rng();
    let codes = ["BEN", "OUR", "SHA"];
    codes[rng.gen_range(0..codes.len())].to_string()
}

/// Generate an instruction code
pub fn generate_instruction_code() -> String {
    let mut rng = rand::thread_rng();
    let codes = ["REPA", "URGP", "CORT", "INTC", "PHON"];
    codes[rng.gen_range(0..codes.len())].to_string()
}

/// Generate name and address lines
pub fn generate_name_and_address(lines: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let names = [
        "ACME CORPORATION",
        "GLOBAL TRADING LTD",
        "INTERNATIONAL FINANCE INC",
        "SWIFT PAYMENTS CORP",
        "DIGITAL SOLUTIONS AG",
    ];

    let streets = [
        "123 MAIN STREET",
        "456 PARK AVENUE",
        "789 BROADWAY",
        "321 WALL STREET",
        "654 FIFTH AVENUE",
    ];

    let cities = [
        "NEW YORK NY 10001",
        "LONDON EC1A 1BB",
        "ZURICH 8001",
        "SINGAPORE 018956",
        "TOKYO 100-0001",
    ];

    let mut result = vec![];

    if lines > 0 {
        result.push(names[rng.gen_range(0..names.len())].to_string());
    }
    if lines > 1 && rng.gen_bool(0.7) {
        result.push(streets[rng.gen_range(0..streets.len())].to_string());
    }
    if lines > result.len() && rng.gen_bool(0.8) {
        result.push(cities[rng.gen_range(0..cities.len())].to_string());
    }
    if lines > result.len() {
        result.push(generate_valid_country_code());
    }

    // Fill remaining lines if needed
    while result.len() < lines {
        result.push(generate_any_character(rng.gen_range(10..30)));
    }

    result
}

/// Generate a UETR (Unique End-to-End Transaction Reference) in UUID format
/// Used for CBPR+ compliance in Tag 121 of User Header
pub fn generate_uetr() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Generate UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    // where x is any hexadecimal digit and y is one of 8, 9, A, or B
    let hex_chars = "0123456789abcdef";
    let hex_chars: Vec<char> = hex_chars.chars().collect();

    let mut uuid = String::new();

    // First segment: 8 hex chars
    for _ in 0..8 {
        uuid.push(hex_chars[rng.gen_range(0..16)]);
    }
    uuid.push('-');

    // Second segment: 4 hex chars
    for _ in 0..4 {
        uuid.push(hex_chars[rng.gen_range(0..16)]);
    }
    uuid.push('-');

    // Third segment: 4xxx where first char is '4'
    uuid.push('4');
    for _ in 0..3 {
        uuid.push(hex_chars[rng.gen_range(0..16)]);
    }
    uuid.push('-');

    // Fourth segment: yxxx where y is 8, 9, a, or b
    let y_chars = ['8', '9', 'a', 'b'];
    uuid.push(y_chars[rng.gen_range(0..4)]);
    for _ in 0..3 {
        uuid.push(hex_chars[rng.gen_range(0..16)]);
    }
    uuid.push('-');

    // Fifth segment: 12 hex chars
    for _ in 0..12 {
        uuid.push(hex_chars[rng.gen_range(0..16)]);
    }

    uuid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_numeric() {
        let result = generate_numeric(6);
        assert_eq!(result.len(), 6);
        assert!(result.chars().all(|c| c.is_ascii_digit()));

        // Test multiple generations for consistency
        for _ in 0..10 {
            let result = generate_numeric(8);
            assert_eq!(result.len(), 8);
            assert!(result.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_generate_alphabetic() {
        let result = generate_alphabetic(4);
        assert_eq!(result.len(), 4);
        assert!(result.chars().all(|c| c.is_ascii_uppercase()));

        // Test edge cases
        let empty = generate_alphabetic(0);
        assert_eq!(empty.len(), 0);

        let single = generate_alphabetic(1);
        assert_eq!(single.len(), 1);
        assert!(single.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_generate_alphanumeric() {
        let result = generate_alphanumeric(10);
        assert_eq!(result.len(), 10);
        assert!(
            result.chars().all(
                |c| c.is_ascii_alphanumeric() && (c.is_ascii_uppercase() || c.is_ascii_digit())
            )
        );
    }

    #[test]
    fn test_generate_any_character() {
        let result = generate_any_character(20);
        assert_eq!(result.len(), 20);

        // All characters should be SWIFT-allowed
        let allowed_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-?:().,'+";
        assert!(result.chars().all(|c| allowed_chars.contains(c)));
    }

    #[test]
    fn test_generate_decimal() {
        let result = generate_decimal(10, 2);
        assert!(result.contains(','));
        let parts: Vec<&str> = result.split(',').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1].len(), 2);
        assert!(parts[0].chars().all(|c| c.is_ascii_digit()));
        assert!(parts[1].chars().all(|c| c.is_ascii_digit()));

        // Test edge case - decimals >= length
        let edge_case = generate_decimal(3, 3);
        assert_eq!(edge_case, "0,00");
    }

    #[test]
    fn test_generate_valid_bic() {
        for _ in 0..20 {
            let bic = generate_valid_bic();
            assert!(bic.len() == 8 || bic.len() == 11);
            assert!(bic.chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }

    #[test]
    fn test_generate_valid_currency() {
        for _ in 0..20 {
            let currency = generate_valid_currency();
            assert_eq!(currency.len(), 3);
            assert!(currency.chars().all(|c| c.is_ascii_uppercase()));
        }

        // Check that it's from the expected list
        let currencies = vec![
            "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK", "DKK", "SGD",
            "HKD", "CNY", "INR", "KRW", "MXN", "BRL", "ZAR", "AED",
        ];
        let generated = generate_valid_currency();
        assert!(currencies.contains(&generated.as_str()));
    }

    #[test]
    fn test_generate_valid_country_code() {
        let country = generate_valid_country_code();
        assert_eq!(country.len(), 2);
        assert!(country.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_generate_date_yymmdd() {
        let date = generate_date_yymmdd();
        assert_eq!(date.len(), 6);
        assert!(date.chars().all(|c| c.is_ascii_digit()));

        // Validate format - YYMMDD
        let year: u32 = date[0..2].parse().unwrap();
        let month: u32 = date[2..4].parse().unwrap();
        let day: u32 = date[4..6].parse().unwrap();

        assert!((20..=29).contains(&year));
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
    }

    #[test]
    fn test_generate_date_yyyymmdd() {
        let date = generate_date_yyyymmdd();
        assert_eq!(date.len(), 8);
        assert!(date.chars().all(|c| c.is_ascii_digit()));

        // Validate format - YYYYMMDD
        let year: u32 = date[0..4].parse().unwrap();
        let month: u32 = date[4..6].parse().unwrap();
        let day: u32 = date[6..8].parse().unwrap();

        assert!((2020..=2029).contains(&year));
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
    }

    #[test]
    fn test_generate_time_hhmm() {
        let time = generate_time_hhmm();
        assert_eq!(time.len(), 4);
        assert!(time.chars().all(|c| c.is_ascii_digit()));

        // Validate format - HHMM
        let hour: u32 = time[0..2].parse().unwrap();
        let minute: u32 = time[2..4].parse().unwrap();

        assert!(hour <= 23);
        assert!(minute <= 59);
    }

    #[test]
    fn test_generate_by_format_spec() {
        // Test exact length formats
        let result1 = generate_by_format_spec("3!a");
        assert_eq!(result1.len(), 3);
        assert!(result1.chars().all(|c| c.is_ascii_uppercase()));

        let result2 = generate_by_format_spec("6!n");
        assert_eq!(result2.len(), 6);
        assert!(result2.chars().all(|c| c.is_ascii_digit()));

        let result3 = generate_by_format_spec("4!c");
        assert_eq!(result3.len(), 4);
        assert!(result3.chars().all(|c| c.is_ascii_alphanumeric()));

        // Test variable length formats
        let result4 = generate_by_format_spec("16x");
        assert!(!result4.is_empty() && result4.len() <= 16);

        let result5 = generate_by_format_spec("35a");
        assert!(!result5.is_empty() && result5.len() <= 35);
        assert!(result5.chars().all(|c| c.is_ascii_uppercase()));

        // Test decimal format
        let result6 = generate_by_format_spec("15d");
        assert!(result6.contains(','));
    }

    #[test]
    fn test_generate_account_number() {
        let account = generate_account_number();
        assert!(account.len() >= 10 && account.len() <= 34);
        assert!(account.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_generate_reference() {
        let reference = generate_reference();
        assert_eq!(reference.len(), 16);
        assert!(reference.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_generate_transaction_code() {
        let codes = ["NTRF", "CHQB", "PMNT", "MCOP", "DMCT"];
        let code = generate_transaction_code();
        assert!(codes.contains(&code.as_str()));
    }

    #[test]
    fn test_generate_bank_operation_code() {
        let codes = ["CRED", "CRTS", "SPAY", "SSTD"];
        let code = generate_bank_operation_code();
        assert!(codes.contains(&code.as_str()));
    }

    #[test]
    fn test_generate_details_of_charges() {
        let codes = ["BEN", "OUR", "SHA"];
        let code = generate_details_of_charges();
        assert!(codes.contains(&code.as_str()));
    }

    #[test]
    fn test_generate_instruction_code() {
        let codes = ["REPA", "URGP", "CORT", "INTC", "PHON"];
        let code = generate_instruction_code();
        assert!(codes.contains(&code.as_str()));
    }

    #[test]
    fn test_generate_name_and_address() {
        // Test different line counts
        for line_count in 1..=5 {
            let lines = generate_name_and_address(line_count);
            assert_eq!(lines.len(), line_count);
            assert!(lines.iter().all(|line| !line.is_empty()));
            assert!(lines.iter().all(|line| line.len() <= 35)); // SWIFT line length limit
        }

        // Test edge case - zero lines
        let empty_lines = generate_name_and_address(0);
        assert_eq!(empty_lines.len(), 0);
    }

    #[test]
    fn test_configuration_types() {
        // Test FieldConfig creation
        let field_config = FieldConfig {
            length_preference: Some(LengthPreference::Exact(10)),
            value_range: Some(ValueRange::Amount {
                min: 100.0,
                max: 1000.0,
                currency: Some("USD".to_string()),
            }),
            fixed_values: Some(vec!["TEST1".to_string(), "TEST2".to_string()]),
            pattern: Some(r"^[A-Z]{3}\d{7}$".to_string()),
        };

        assert!(field_config.length_preference.is_some());
        assert!(field_config.value_range.is_some());
        assert!(field_config.fixed_values.is_some());
        assert!(field_config.pattern.is_some());

        // Test MessageConfig creation
        let mut field_configs = std::collections::HashMap::new();
        field_configs.insert("32A".to_string(), field_config);

        let message_config = MessageConfig {
            include_optional: true,
            field_configs,
            scenario: Some(MessageScenario::StpCompliant),
        };

        assert!(message_config.include_optional);
        assert!(message_config.field_configs.contains_key("32A"));
        assert_eq!(message_config.scenario, Some(MessageScenario::StpCompliant));
    }

    #[test]
    fn test_message_scenarios() {
        // Test all scenario variants
        let scenarios = vec![
            MessageScenario::Standard,
            MessageScenario::StpCompliant,
            MessageScenario::CoverPayment,
            MessageScenario::Minimal,
            MessageScenario::Full,
        ];

        for scenario in scenarios {
            // Just test that they can be created and compared
            assert_eq!(scenario, scenario);
        }
    }

    #[test]
    fn test_value_range_variants() {
        // Test Amount range
        let amount_range = ValueRange::Amount {
            min: 100.0,
            max: 1000.0,
            currency: Some("EUR".to_string()),
        };

        match amount_range {
            ValueRange::Amount { min, max, currency } => {
                assert_eq!(min, 100.0);
                assert_eq!(max, 1000.0);
                assert_eq!(currency, Some("EUR".to_string()));
            }
            _ => panic!("Expected Amount variant"),
        }

        // Test Date range
        let date_range = ValueRange::Date {
            start: "20230101".to_string(),
            end: "20231231".to_string(),
        };

        match date_range {
            ValueRange::Date { start, end } => {
                assert_eq!(start, "20230101");
                assert_eq!(end, "20231231");
            }
            _ => panic!("Expected Date variant"),
        }

        // Test Integer range
        let integer_range = ValueRange::Integer { min: 1, max: 100 };

        match integer_range {
            ValueRange::Integer { min, max } => {
                assert_eq!(min, 1);
                assert_eq!(max, 100);
            }
            _ => panic!("Expected Integer variant"),
        }
    }

    #[test]
    fn test_length_preference_variants() {
        // Test all LengthPreference variants
        let exact = LengthPreference::Exact(10);
        assert_eq!(exact, LengthPreference::Exact(10));

        let range = LengthPreference::Range(5, 15);
        assert_eq!(range, LengthPreference::Range(5, 15));

        let short = LengthPreference::Short;
        assert_eq!(short, LengthPreference::Short);

        let long = LengthPreference::Long;
        assert_eq!(long, LengthPreference::Long);
    }

    #[test]
    fn test_randomness_distribution() {
        // Test that the generators produce different values over multiple runs
        let mut bics = std::collections::HashSet::new();
        let mut currencies = std::collections::HashSet::new();
        let mut references = std::collections::HashSet::new();

        for _ in 0..50 {
            bics.insert(generate_valid_bic());
            currencies.insert(generate_valid_currency());
            references.insert(generate_reference());
        }

        // We should have some variety (not all the same)
        assert!(bics.len() > 1, "BIC generation should have variety");
        assert!(
            currencies.len() > 1,
            "Currency generation should have variety"
        );
        assert!(
            references.len() > 1,
            "Reference generation should have variety"
        );
    }
}
