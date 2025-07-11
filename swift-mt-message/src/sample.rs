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

/// Generate string with any SWIFT-allowed character (reduced special chars for realism)
pub fn generate_any_character(length: usize) -> String {
    let mut rng = rand::thread_rng();
    // Reduce special characters to make output look more realistic
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 /-.,"
        .chars()
        .collect();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Generate decimal number with specified total length and decimal places
pub fn generate_decimal(length: usize, decimals: usize) -> String {
    generate_decimal_with_range(length, decimals, None, None)
}

/// Generate decimal number with optional min/max range
pub fn generate_decimal_with_range(
    length: usize,
    decimals: usize,
    min: Option<f64>,
    max: Option<f64>,
) -> String {
    if decimals >= length {
        return "0,00".to_string();
    }

    let mut rng = rand::thread_rng();

    // If min/max are provided, generate within that range
    if let (Some(min_val), Some(max_val)) = (min, max) {
        let amount = rng.gen_range(min_val..=max_val);
        let formatted = format!("{amount:.2}").replace('.', ",");
        if formatted.len() <= length {
            return formatted;
        }
    }

    // Generate realistic amounts based on typical transaction ranges
    let realistic_amounts = [
        "1250,00",
        "850,50",
        "2000,75",
        "500,25",
        "10000,00",
        "750,80",
        "3500,45",
        "125,60",
        "25000,00",
        "1875,90",
        "650,15",
        "4200,35",
        "50000,00",
        "75000,00",
        "100000,00",
        "250000,00",
        "500000,00",
        "1000000,00",
        "2500000,00",
        "5000000,00",
    ];

    // For shorter lengths, use predefined realistic amounts
    if length <= 10 {
        let amount = realistic_amounts[rng.gen_range(0..realistic_amounts.len())];
        if amount.len() <= length {
            return amount.to_string();
        }
    }

    // For longer amounts, generate realistic business transaction amounts
    // Typical ranges: small (100-9,999), medium (10,000-999,999), large (1M-100M)
    let amount_ranges = [
        (100.0, 9999.0),        // Small transactions
        (10000.0, 99999.0),     // Medium transactions
        (100000.0, 999999.0),   // Large transactions
        (1000000.0, 9999999.0), // Very large transactions
    ];

    let (min_amt, max_amt) = amount_ranges[rng.gen_range(0..amount_ranges.len())];
    let amount = rng.gen_range(min_amt..=max_amt);
    let formatted = format!("{amount:.2}").replace('.', ",");

    if formatted.len() <= length {
        return formatted;
    }

    // Fallback: generate basic structure if formatted amount is too long
    let integer_part_len = if length > decimals + 1 {
        length - decimals - 1 // -1 for comma
    } else {
        1 // Ensure at least 1 digit for integer part
    };

    let mut integer_part = String::new();
    if integer_part_len > 0 {
        // Generate smaller realistic amounts that fit the length constraint
        let max_val = 10_u64.pow(integer_part_len as u32) - 1;
        let amount = rng.gen_range(100..=max_val.min(999999)); // Cap at reasonable amount
        integer_part = amount.to_string();

        // Pad if necessary
        while integer_part.len() < integer_part_len {
            integer_part = format!("0{integer_part}");
        }
    } else {
        // Fallback: ensure at least one digit
        integer_part.push_str(&rng.gen_range(1..10).to_string());
    }

    let decimal_part = generate_numeric(decimals);
    format!("{integer_part},{decimal_part}")
}

/// Generate a valid BIC code
pub fn generate_valid_bic() -> String {
    let mut rng = rand::thread_rng();
    let bics = [
        // Major US banks (all 8 chars)
        "CHASUS33", "BOFAUS3N", "CITIUS33", "WFBIUS6W", "USBKUS44", "PNCCUS33",
        // Major European banks (all 8 chars)
        "DEUTDEFF", "HSBCGB2L", "BNPAFRPP", "UBSWCHZH", "ABNANL2A", "INGBNL2A", "CRESCHZZ",
        "BARCGB22", "LOYDGB2L", "NWBKGB2L", "RBOSGB2L",
        // Major Asian banks (all 8 chars)
        "SCBLSGSG", "DBSSSGSG", "OCBCSGSG", "HSBCHKHH", "CITIHKAX", "BOTKJPJT", "SMFGJPJT",
        "MHCBJPJT", // Major Canadian/Australian banks (all 8 chars)
        "ROYCCAT2", "BOFACATT", "ANZBAU3M", "CTBAAU2S",
        // Major international banks (all 8 chars)
        "ICICINBB", "HDFCINBB", "SBININBB", "BBVASPBX",
    ];
    bics[rng.gen_range(0..bics.len())].to_string()
}

/// Generate a valid currency code with realistic distribution
pub fn generate_valid_currency() -> String {
    let mut rng = rand::thread_rng();

    // Weight currencies by real-world usage in international payments
    let weighted_selection = rng.gen_range(1..=100);

    match weighted_selection {
        1..=30 => "USD".to_string(),  // 30% - Most common
        31..=45 => "EUR".to_string(), // 15% - Second most common
        46..=55 => "GBP".to_string(), // 10% - Third most common
        56..=60 => "JPY".to_string(), // 5%
        61..=64 => "CHF".to_string(), // 4%
        65..=67 => "CAD".to_string(), // 3%
        68..=70 => "AUD".to_string(), // 3%
        71..=73 => "SGD".to_string(), // 3%
        74..=76 => "HKD".to_string(), // 3%
        77..=79 => "CNY".to_string(), // 3%
        80..=82 => "SEK".to_string(), // 3%
        83..=85 => "NOK".to_string(), // 3%
        86..=87 => "DKK".to_string(), // 2%
        88..=89 => "NZD".to_string(), // 2%
        90..=91 => "INR".to_string(), // 2%
        92..=93 => "KRW".to_string(), // 2%
        94..=95 => "BRL".to_string(), // 2%
        96..=97 => "ZAR".to_string(), // 2%
        98..=99 => "AED".to_string(), // 2%
        _ => "MXN".to_string(),       // 1%
    }
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
    generate_by_format_spec_with_config(format, &FieldConfig::default())
}

/// Generate a value based on SWIFT format specification with configuration
pub fn generate_by_format_spec_with_config(format: &str, config: &FieldConfig) -> String {
    // Check if fixed values are provided
    if let Some(fixed_values) = &config.fixed_values {
        if !fixed_values.is_empty() {
            let mut rng = rand::thread_rng();
            return fixed_values[rng.gen_range(0..fixed_values.len())].clone();
        }
    }

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

    // Apply length preference from config
    let length = match &config.length_preference {
        Some(LengthPreference::Exact(len)) => *len.min(&max_length),
        Some(LengthPreference::Range(min, max)) => {
            let mut rng = rand::thread_rng();
            let actual_min = *min.min(&max_length);
            let actual_max = (*max).min(max_length);
            if actual_min <= actual_max {
                rng.gen_range(actual_min..=actual_max)
            } else {
                max_length
            }
        }
        Some(LengthPreference::Short) => {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..=(max_length / 2).max(1))
        }
        Some(LengthPreference::Long) => {
            let mut rng = rand::thread_rng();
            rng.gen_range((max_length / 2).max(1)..=max_length)
        }
        None => {
            if is_exact {
                max_length
            } else {
                // For decimal formats, use full length to ensure reasonable amounts
                if char_type == 'd' {
                    max_length
                } else {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(1..=max_length)
                }
            }
        }
    };

    match char_type {
        'n' => generate_numeric(length),
        'a' => generate_alphabetic(length),
        'c' => generate_alphanumeric(length),
        'd' => {
            // For decimal format, assume 2 decimal places if not specified
            let decimals = 2;
            // Check for amount range configuration
            if let Some(ValueRange::Amount { min, max, .. }) = &config.value_range {
                generate_decimal_with_range(length, decimals, Some(*min), Some(*max))
            } else {
                generate_decimal(length, decimals)
            }
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
        "GLOBAL TRADE SOLUTIONS LTD",
        "INTERNATIONAL EXPORT CORP",
        "PRIME FINANCIAL SERVICES",
        "METROPOLITAN TRADING CO",
        "CONSOLIDATED INDUSTRIES INC",
        "PACIFIC RIM ENTERPRISES",
        "EUROPEAN COMMERCE GROUP",
        "ATLANTIC BUSINESS PARTNERS",
        "CONTINENTAL HOLDINGS LLC",
        "WORLDWIDE LOGISTICS CORP",
        "STERLING INVESTMENT GROUP",
        "MERIDIAN COMMERCIAL LTD",
        "APEX TRADING COMPANY",
        "NEXUS FINANCIAL CORP",
        "HORIZON BUSINESS SOLUTIONS",
    ];

    let streets = [
        "125 CORPORATE PLAZA",
        "450 BUSINESS PARK DRIVE",
        "789 FINANCIAL DISTRICT",
        "1200 COMMERCE STREET",
        "650 EXECUTIVE BOULEVARD",
        "300 TRADE CENTER WAY",
        "850 INTERNATIONAL AVENUE",
        "1500 ENTERPRISE PARKWAY",
        "275 INVESTMENT PLAZA",
        "920 BANKING SQUARE",
        "1750 CORPORATE CENTER",
        "425 PROFESSIONAL DRIVE",
        "680 MARKET STREET",
        "1100 INDUSTRIAL WAY",
        "550 COMMERCIAL BOULEVARD",
    ];

    let cities = [
        "NEW YORK NY 10005",
        "LONDON EC2V 8RF",
        "ZURICH 8001",
        "SINGAPORE 048624",
        "TOKYO 100-6590",
        "FRANKFURT AM MAIN 60311",
        "PARIS 75001",
        "MILAN 20121",
        "GENEVA 1204",
        "DUBLIN 2",
        "AMSTERDAM 1017 XX",
        "BRUSSELS 1000",
        "MADRID 28001",
        "BARCELONA 08002",
        "VIENNA 1010",
    ];

    let mut result = vec![];

    if lines > 0 {
        result.push(names[rng.gen_range(0..names.len())].to_string());
    }
    if lines > 1 {
        result.push(streets[rng.gen_range(0..streets.len())].to_string());
    }
    if lines > 2 {
        result.push(cities[rng.gen_range(0..cities.len())].to_string());
    }
    if lines > 3 {
        result.push(generate_valid_country_code());
    }

    // Fill remaining lines with additional address details if needed
    while result.len() < lines {
        let additional_info = [
            "CORPORATE HEADQUARTERS",
            "MAIN OFFICE",
            "TREASURY DEPARTMENT",
            "INTERNATIONAL DIVISION",
            "FINANCIAL SERVICES",
        ];
        result.push(additional_info[rng.gen_range(0..additional_info.len())].to_string());
    }

    result
}

/// Generate a UETR (Unique End-to-End Transaction Reference) in UUID format
/// Used for CBPR+ compliance in Tag 121 of User Header
pub fn generate_uetr() -> String {
    uuid::Uuid::new_v4().to_string()
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

        // All characters should be SWIFT-allowed (reduced set for realism)
        let allowed_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 /-.,";
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
