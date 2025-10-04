//! # Field-Specific Utility Functions
//!
//! Higher-level parsing utilities specific to SWIFT MT message fields.
//! These utilities handle complex field patterns like party identifiers, multiline text,
//! and field-specific validation logic.

use super::swift_utils::parse_swift_chars;
use crate::errors::ParseError;

/// Payment method codes used in Field 57 and similar fields
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodCode {
    /// Fedwire Transfer System
    FW,
    /// Real-Time Gross Settlement
    RT,
    /// Australian payments
    AU,
    /// Indian payments
    IN,
    /// Swiss Clearing
    SW,
    /// CHIPS
    CH,
    /// CHIPS Participant
    CP,
    /// Russian Central Bank
    RU,
}

impl PaymentMethodCode {
    /// Parse a payment method code from a string
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "FW" => Some(PaymentMethodCode::FW),
            "RT" => Some(PaymentMethodCode::RT),
            "AU" => Some(PaymentMethodCode::AU),
            "IN" => Some(PaymentMethodCode::IN),
            "SW" => Some(PaymentMethodCode::SW),
            "CH" => Some(PaymentMethodCode::CH),
            "CP" => Some(PaymentMethodCode::CP),
            "RU" => Some(PaymentMethodCode::RU),
            _ => None,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentMethodCode::FW => "FW",
            PaymentMethodCode::RT => "RT",
            PaymentMethodCode::AU => "AU",
            PaymentMethodCode::IN => "IN",
            PaymentMethodCode::SW => "SW",
            PaymentMethodCode::CH => "CH",
            PaymentMethodCode::CP => "CP",
            PaymentMethodCode::RU => "RU",
        }
    }
}

/// Parse payment method from //XX format
pub fn parse_payment_method(input: &str) -> Option<PaymentMethodCode> {
    if input.starts_with("//") && input.len() == 4 {
        PaymentMethodCode::parse(&input[2..])
    } else {
        None
    }
}

/// Transaction type codes used in Field 61 and similar fields
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionTypeCode {
    /// Book transfer
    BOK,
    /// Customer transfer
    MSC,
    /// Reversal of book transfer
    RTR,
    /// Customer cheque
    CHK,
    /// Draft
    DFT,
    /// Standing order
    STO,
    /// Loan transaction
    LDP,
    /// Foreign exchange
    FEX,
    /// Collection
    COL,
    /// Letter of credit
    LBX,
    /// Travellers cheques
    TCK,
    /// Documentary credit
    DCR,
    /// Cash letter
    CSH,
    /// Charges and other debit interest
    CHG,
    /// Interest
    INT,
    /// Dividend
    DIV,
}

impl TransactionTypeCode {
    /// Parse a transaction type code
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "BOK" => Some(TransactionTypeCode::BOK),
            "MSC" => Some(TransactionTypeCode::MSC),
            "RTR" => Some(TransactionTypeCode::RTR),
            "CHK" => Some(TransactionTypeCode::CHK),
            "DFT" => Some(TransactionTypeCode::DFT),
            "STO" => Some(TransactionTypeCode::STO),
            "LDP" => Some(TransactionTypeCode::LDP),
            "FEX" => Some(TransactionTypeCode::FEX),
            "COL" => Some(TransactionTypeCode::COL),
            "LBX" => Some(TransactionTypeCode::LBX),
            "TCK" => Some(TransactionTypeCode::TCK),
            "DCR" => Some(TransactionTypeCode::DCR),
            "CSH" => Some(TransactionTypeCode::CSH),
            "CHG" => Some(TransactionTypeCode::CHG),
            "INT" => Some(TransactionTypeCode::INT),
            "DIV" => Some(TransactionTypeCode::DIV),
            _ => None,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionTypeCode::BOK => "BOK",
            TransactionTypeCode::MSC => "MSC",
            TransactionTypeCode::RTR => "RTR",
            TransactionTypeCode::CHK => "CHK",
            TransactionTypeCode::DFT => "DFT",
            TransactionTypeCode::STO => "STO",
            TransactionTypeCode::LDP => "LDP",
            TransactionTypeCode::FEX => "FEX",
            TransactionTypeCode::COL => "COL",
            TransactionTypeCode::LBX => "LBX",
            TransactionTypeCode::TCK => "TCK",
            TransactionTypeCode::DCR => "DCR",
            TransactionTypeCode::CSH => "CSH",
            TransactionTypeCode::CHG => "CHG",
            TransactionTypeCode::INT => "INT",
            TransactionTypeCode::DIV => "DIV",
        }
    }
}

/// Bank operation codes used in Field 23 and similar fields
#[derive(Debug, Clone, PartialEq)]
pub enum BankOperationCode {
    /// Credit transfer
    CRED,
    /// Credit reversal
    CRTS,
    /// Debit transfer
    SPAY,
    /// Debit reversal
    SSTD,
    /// Priority payment
    SPRI,
    /// Related reference
    CHQB,
}

impl BankOperationCode {
    /// Parse a bank operation code
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "CRED" => Some(BankOperationCode::CRED),
            "CRTS" => Some(BankOperationCode::CRTS),
            "SPAY" => Some(BankOperationCode::SPAY),
            "SSTD" => Some(BankOperationCode::SSTD),
            "SPRI" => Some(BankOperationCode::SPRI),
            "CHQB" => Some(BankOperationCode::CHQB),
            _ => None,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            BankOperationCode::CRED => "CRED",
            BankOperationCode::CRTS => "CRTS",
            BankOperationCode::SPAY => "SPAY",
            BankOperationCode::SSTD => "SSTD",
            BankOperationCode::SPRI => "SPRI",
            BankOperationCode::CHQB => "CHQB",
        }
    }
}

/// Validate that a field option (like 50A, 50K) matches expected options
pub fn validate_field_option(
    field_number: &str,
    option: Option<char>,
    allowed_options: &[char],
) -> Result<(), ParseError> {
    if let Some(opt) = option
        && !allowed_options.contains(&opt)
    {
        return Err(ParseError::InvalidFormat {
            message: format!("Field {} does not support option {}", field_number, opt),
        });
    }
    Ok(())
}

/// Parse a field tag with optional variant (e.g., "50A" -> ("50", Some('A')))
pub fn parse_field_tag(tag: &str) -> (String, Option<char>) {
    if tag.len() >= 2 {
        let last_char = tag.chars().last().unwrap();
        if last_char.is_ascii_alphabetic()
            && tag[..tag.len() - 1].chars().all(|c| c.is_ascii_digit())
        {
            return (tag[..tag.len() - 1].to_string(), Some(last_char));
        }
    }
    (tag.to_string(), None)
}

/// Check if a line looks like a numbered field line (e.g., "1/" or "2/" at start)
pub fn is_numbered_line(line: &str) -> bool {
    if line.len() >= 2 {
        let mut chars = line.chars();
        if let Some(first) = chars.next()
            && let Some(second) = chars.next()
        {
            return first.is_ascii_digit() && second == '/';
        }
    }
    false
}

/// Parse numbered lines format (e.g., "1/ACCOUNT", "2/NAME")
pub fn parse_numbered_lines(lines: &[&str]) -> Result<Vec<(u8, String)>, ParseError> {
    let mut result = Vec::new();

    for line in lines {
        if !is_numbered_line(line) {
            return Err(ParseError::InvalidFormat {
                message: format!("Expected numbered line format (n/text), found: {}", line),
            });
        }

        let number = line.chars().next().unwrap().to_digit(10).unwrap() as u8;
        let content = &line[2..]; // Skip "n/"

        result.push((number, content.to_string()));
    }

    Ok(result)
}

/// Extract the numeric part from mixed alphanumeric field (e.g., "32A" -> "32")
pub fn extract_field_number(field_tag: &str) -> String {
    field_tag
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect()
}

/// Parse party identifier in format /1!a/34x, /2!a/34x, //XX, or /34x
/// Used in fields 51-59 for institutional and party identification
pub fn parse_party_identifier(input: &str) -> Result<Option<String>, ParseError> {
    if !input.starts_with('/') {
        return Ok(None);
    }

    let remaining = &input[1..];

    // Handle special //XX format (e.g., //FW, //RT, //AU, //IN)
    if let Some(special_code) = remaining.strip_prefix('/') {
        if !special_code.is_empty() && special_code.len() <= 34 {
            parse_swift_chars(special_code, "party identifier")?;
            return Ok(Some(format!("/{}", special_code)));
        }
        return Err(ParseError::InvalidFormat {
            message: format!("Invalid special party identifier format: {}", input),
        });
    }

    // Check for /code/identifier format
    if let Some(slash_pos) = remaining.find('/') {
        let code = &remaining[..slash_pos];
        let id = &remaining[slash_pos + 1..];

        // Handle /1!a/34x format (single alphabetic character)
        if code.len() == 1 && code.chars().all(|c| c.is_ascii_alphabetic()) {
            if id.len() > 34 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Party identifier exceeds 34 characters: {}", id.len()),
                });
            }
            parse_swift_chars(id, "party identifier")?;
            return Ok(Some(format!("{}/{}", code, id)));
        }

        // Handle /2!a/34x format (e.g., /CH/, /FW/, /CP/)
        if (1..=2).contains(&code.len())
            && code
                .chars()
                .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        {
            if id.len() > 34 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Party identifier exceeds 34 characters: {}", id.len()),
                });
            }
            parse_swift_chars(id, "party identifier")?;
            return Ok(Some(format!("{}/{}", code, id)));
        }
    } else if remaining.len() <= 34 {
        // Simple /34x format (no additional slash)
        parse_swift_chars(remaining, "party identifier")?;
        return Ok(Some(remaining.to_string()));
    }

    Err(ParseError::InvalidFormat {
        message: format!("Invalid party identifier format: {}", input),
    })
}

/// Parse and validate debit/credit mark (D or C)
pub fn parse_debit_credit_mark(input: char) -> Result<String, ParseError> {
    if input != 'D' && input != 'C' {
        return Err(ParseError::InvalidFormat {
            message: format!("Debit/credit mark must be 'D' or 'C', found: '{}'", input),
        });
    }
    Ok(input.to_string())
}

/// Validate multi-line text with specific constraints
/// Returns validated lines as Vec<String>
pub fn validate_multiline_text(
    lines: &[&str],
    max_lines: usize,
    max_line_length: usize,
    field_name: &str,
) -> Result<Vec<String>, ParseError> {
    if lines.is_empty() {
        return Err(ParseError::InvalidFormat {
            message: format!("{} must have at least one line", field_name),
        });
    }

    if lines.len() > max_lines {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "{} cannot have more than {} lines, found {}",
                field_name,
                max_lines,
                lines.len()
            ),
        });
    }

    let mut result = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.len() > max_line_length {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "{} line {} exceeds {} characters",
                    field_name,
                    i + 1,
                    max_line_length
                ),
            });
        }
        parse_swift_chars(line, &format!("{} line {}", field_name, i + 1))?;
        result.push(line.to_string());
    }

    Ok(result)
}

/// Parse name and address lines (4*35x format)
/// Used in fields 50-59 for party name and address information
pub fn parse_name_and_address(
    lines: &[&str],
    start_idx: usize,
    field_name: &str,
) -> Result<Vec<String>, ParseError> {
    let mut name_and_address = Vec::new();

    for (i, line) in lines.iter().enumerate().skip(start_idx) {
        if line.len() > 35 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "{} line {} exceeds 35 characters",
                    field_name,
                    i - start_idx + 1
                ),
            });
        }
        parse_swift_chars(line, &format!("{} line {}", field_name, i - start_idx + 1))?;
        name_and_address.push(line.to_string());
    }

    if name_and_address.is_empty() {
        return Err(ParseError::InvalidFormat {
            message: format!("{} must have at least one name/address line", field_name),
        });
    }

    if name_and_address.len() > 4 {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "{} cannot have more than 4 name/address lines, found {}",
                field_name,
                name_and_address.len()
            ),
        });
    }

    Ok(name_and_address)
}

/// Parse multiline text (4*35x format) - simpler version for basic multiline fields
pub fn parse_multiline_text(
    input: &str,
    max_lines: usize,
    max_line_length: usize,
) -> Result<Vec<String>, ParseError> {
    let lines: Vec<String> = input
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if lines.len() > max_lines {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Text exceeds maximum of {} lines, found {}",
                max_lines,
                lines.len()
            ),
        });
    }

    for (i, line) in lines.iter().enumerate() {
        if line.len() > max_line_length {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Line {} exceeds maximum length of {} characters",
                    i + 1,
                    max_line_length
                ),
            });
        }
    }

    Ok(lines)
}

/// Extract field option (e.g., "A" from ":50A:")
pub fn extract_field_option(tag: &str) -> Option<char> {
    // Format is :NNO: where NN is field number and O is optional letter
    if tag.len() >= 5 && tag.starts_with(':') && tag.ends_with(':') {
        let inner = &tag[1..tag.len() - 1];
        if inner.len() == 3 && inner[0..2].chars().all(|c| c.is_numeric()) {
            return inner.chars().nth(2);
        }
    }
    None
}

/// Parse field with optional suffix (e.g., "20C" -> ("20", Some('C')))
pub fn parse_field_with_suffix(input: &str) -> (String, Option<char>) {
    if let Some(last_char) = input.chars().last()
        && last_char.is_alphabetic()
        && input[..input.len() - 1].chars().all(|c| c.is_numeric())
    {
        return (input[..input.len() - 1].to_string(), Some(last_char));
    }
    (input.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_method_code() {
        assert_eq!(PaymentMethodCode::parse("FW"), Some(PaymentMethodCode::FW));
        assert_eq!(PaymentMethodCode::parse("RT"), Some(PaymentMethodCode::RT));
        assert_eq!(PaymentMethodCode::parse("XX"), None);

        assert_eq!(PaymentMethodCode::FW.as_str(), "FW");
    }

    #[test]
    fn test_parse_payment_method() {
        assert_eq!(parse_payment_method("//FW"), Some(PaymentMethodCode::FW));
        assert_eq!(parse_payment_method("//RT"), Some(PaymentMethodCode::RT));
        assert_eq!(parse_payment_method("FW"), None);
        assert_eq!(parse_payment_method("//XXX"), None);
    }

    #[test]
    fn test_transaction_type_code() {
        assert_eq!(
            TransactionTypeCode::parse("MSC"),
            Some(TransactionTypeCode::MSC)
        );
        assert_eq!(
            TransactionTypeCode::parse("CHK"),
            Some(TransactionTypeCode::CHK)
        );
        assert_eq!(TransactionTypeCode::parse("XXX"), None);

        assert_eq!(TransactionTypeCode::MSC.as_str(), "MSC");
    }

    #[test]
    fn test_validate_field_option() {
        assert!(validate_field_option("50", Some('A'), &['A', 'K', 'F']).is_ok());
        assert!(validate_field_option("50", Some('X'), &['A', 'K', 'F']).is_err());
        assert!(validate_field_option("50", None, &['A', 'K', 'F']).is_ok());
    }

    #[test]
    fn test_parse_field_tag() {
        assert_eq!(parse_field_tag("50A"), ("50".to_string(), Some('A')));
        assert_eq!(parse_field_tag("50"), ("50".to_string(), None));
        assert_eq!(parse_field_tag("32A"), ("32".to_string(), Some('A')));
        assert_eq!(parse_field_tag("ABC"), ("ABC".to_string(), None));
    }

    #[test]
    fn test_is_numbered_line() {
        assert!(is_numbered_line("1/ACCOUNT"));
        assert!(is_numbered_line("2/NAME"));
        assert!(is_numbered_line("9/TEXT"));
        assert!(!is_numbered_line("ACCOUNT"));
        assert!(!is_numbered_line("/ACCOUNT"));
        assert!(!is_numbered_line("A/ACCOUNT"));
    }

    #[test]
    fn test_parse_numbered_lines() {
        let lines = vec!["1/ACCOUNT", "2/NAME", "3/ADDRESS"];
        let result = parse_numbered_lines(&lines).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], (1, "ACCOUNT".to_string()));
        assert_eq!(result[1], (2, "NAME".to_string()));
        assert_eq!(result[2], (3, "ADDRESS".to_string()));

        let bad_lines = vec!["1/ACCOUNT", "NAME"];
        assert!(parse_numbered_lines(&bad_lines).is_err());
    }

    #[test]
    fn test_extract_field_number() {
        assert_eq!(extract_field_number("50A"), "50");
        assert_eq!(extract_field_number("32"), "32");
        assert_eq!(extract_field_number("103STP"), "103");
        assert_eq!(extract_field_number("ABC"), "");
    }

    #[test]
    fn test_parse_party_identifier() {
        // Test /1!a/34x format
        let result = parse_party_identifier("/D/12345678").unwrap();
        assert_eq!(result, Some("D/12345678".to_string()));

        // Test /2!a/34x format
        let result = parse_party_identifier("/CH/123456").unwrap();
        assert_eq!(result, Some("CH/123456".to_string()));

        // Test /34x format
        let result = parse_party_identifier("/ACCOUNT123").unwrap();
        assert_eq!(result, Some("ACCOUNT123".to_string()));

        // Test //XX format (special codes)
        let result = parse_party_identifier("//FW123456").unwrap();
        assert_eq!(result, Some("/FW123456".to_string()));

        let result = parse_party_identifier("//RT").unwrap();
        assert_eq!(result, Some("/RT".to_string()));

        // Test no party identifier
        let result = parse_party_identifier("NOTPARTY").unwrap();
        assert_eq!(result, None);

        // Test too long identifier
        assert!(parse_party_identifier("/D/12345678901234567890123456789012345").is_err());
    }

    #[test]
    fn test_parse_debit_credit_mark() {
        assert_eq!(parse_debit_credit_mark('D').unwrap(), "D");
        assert_eq!(parse_debit_credit_mark('C').unwrap(), "C");
        assert!(parse_debit_credit_mark('X').is_err());
        assert!(parse_debit_credit_mark('1').is_err());
    }

    #[test]
    fn test_validate_multiline_text() {
        let lines = vec!["LINE 1", "LINE 2", "LINE 3"];
        let result = validate_multiline_text(&lines, 4, 35, "Test Field").unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "LINE 1");

        // Test too many lines
        let lines = vec!["L1", "L2", "L3", "L4", "L5"];
        assert!(validate_multiline_text(&lines, 4, 35, "Test Field").is_err());

        // Test line too long
        let lines = vec!["THIS LINE IS TOO LONG AND EXCEEDS THE 35 CHARACTER LIMIT"];
        assert!(validate_multiline_text(&lines, 4, 35, "Test Field").is_err());

        // Test empty lines
        let lines: Vec<&str> = vec![];
        assert!(validate_multiline_text(&lines, 4, 35, "Test Field").is_err());
    }

    #[test]
    fn test_parse_name_and_address() {
        let lines = vec!["PARTY ID", "JOHN DOE", "123 MAIN ST", "NEW YORK"];
        let result = parse_name_and_address(&lines, 1, "Test Field").unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "JOHN DOE");
        assert_eq!(result[1], "123 MAIN ST");
        assert_eq!(result[2], "NEW YORK");

        // Test too many lines
        let lines = vec!["ID", "L1", "L2", "L3", "L4", "L5"];
        assert!(parse_name_and_address(&lines, 1, "Test Field").is_err());

        // Test line too long
        let lines = vec![
            "ID",
            "THIS LINE IS TOO LONG AND EXCEEDS THE 35 CHARACTER LIMIT",
        ];
        assert!(parse_name_and_address(&lines, 1, "Test Field").is_err());
    }

    #[test]
    fn test_extract_field_option() {
        assert_eq!(extract_field_option(":50A:"), Some('A'));
        assert_eq!(extract_field_option(":50K:"), Some('K'));
        assert_eq!(extract_field_option(":20:"), None);
        assert_eq!(extract_field_option("50A"), None);
    }

    #[test]
    fn test_parse_field_with_suffix() {
        assert_eq!(
            parse_field_with_suffix("20C"),
            ("20".to_string(), Some('C'))
        );
        assert_eq!(parse_field_with_suffix("20"), ("20".to_string(), None));
        assert_eq!(parse_field_with_suffix("ABC"), ("ABC".to_string(), None));
    }
}
