use crate::errors::ParseError;
use crate::message_parser::MessageParser;
use crate::traits::SwiftField;

/// Extract Block 4 content from SWIFT message input.
/// If input starts with "{", attempts to extract Block 4.
/// Otherwise, assumes input is already Block 4 content.
pub fn extract_block4(input: &str) -> Result<String, ParseError> {
    if input.starts_with("{") {
        crate::parser::SwiftParser::extract_block(input, 4)?.ok_or_else(|| {
            ParseError::InvalidFormat {
                message: "Block 4 not found".to_string(),
            }
        })
    } else {
        Ok(input.to_string())
    }
}

/// Append a mandatory field to the result string with CRLF.
pub fn append_field<T: SwiftField>(result: &mut String, field: &T) {
    result.push_str(&field.to_swift_string());
    result.push_str("\r\n");
}

/// Append an optional field to the result string with CRLF if present.
pub fn append_optional_field<T: SwiftField>(result: &mut String, field: &Option<T>) {
    if let Some(f) = field {
        result.push_str(&f.to_swift_string());
        result.push_str("\r\n");
    }
}

/// Append a vector of fields to the result string, each with CRLF.
pub fn append_vec_field<T: SwiftField>(result: &mut String, fields: &Option<Vec<T>>) {
    if let Some(vec) = fields {
        for field in vec {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }
    }
}

/// Parse repeated fields and return as Option<Vec<T>>.
/// Returns None if no fields found, Some(vec) otherwise.
pub fn parse_repeated_field<T: crate::traits::SwiftField>(
    parser: &mut MessageParser,
    tag: &str,
) -> Result<Option<Vec<T>>, ParseError> {
    let mut fields = Vec::new();
    while let Ok(field) = parser.parse_field::<T>(tag) {
        fields.push(field);
    }
    Ok(if fields.is_empty() {
        None
    } else {
        Some(fields)
    })
}

/// Verify that all content in the parser has been consumed.
/// Returns error if unparsed content remains.
pub fn verify_parser_complete(parser: &MessageParser) -> Result<(), ParseError> {
    if !parser.is_complete() {
        return Err(ParseError::InvalidFormat {
            message: format!(
                "Unparsed content remaining in message: {}",
                parser.remaining()
            ),
        });
    }
    Ok(())
}

/// Remove trailing CRLF from result string if present.
pub fn remove_trailing_crlf(result: &mut String) {
    if result.ends_with("\r\n") {
        result.truncate(result.len() - 2);
    }
}

/// Finalize MT string by removing trailing CRLF and optionally adding terminator.
pub fn finalize_mt_string(mut result: String, add_terminator: bool) -> String {
    remove_trailing_crlf(&mut result);
    if add_terminator {
        result.push('-');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_block4_with_blocks() {
        let input = "{1:F01BANKFRPPAXXX0000000000}{4:\r\n:20:TEST123\r\n-}";
        let result = extract_block4(input);
        assert!(result.is_ok());
        assert!(result.unwrap().contains(":20:TEST123"));
    }

    #[test]
    fn test_extract_block4_plain() {
        let input = ":20:TEST123\r\n";
        let result = extract_block4(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_extract_block4_missing() {
        let input = "{1:F01BANKFRPPAXXX0000000000}";
        let result = extract_block4(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_trailing_crlf() {
        let mut s = String::from("test\r\n");
        remove_trailing_crlf(&mut s);
        assert_eq!(s, "test");

        let mut s2 = String::from("test");
        remove_trailing_crlf(&mut s2);
        assert_eq!(s2, "test");
    }

    #[test]
    fn test_finalize_mt_string() {
        let result = finalize_mt_string(String::from("test\r\n"), true);
        assert_eq!(result, "test-");

        let result2 = finalize_mt_string(String::from("test\r\n"), false);
        assert_eq!(result2, "test");
    }
}
