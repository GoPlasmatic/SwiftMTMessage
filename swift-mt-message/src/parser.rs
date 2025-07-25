use std::collections::HashMap;

use crate::errors::{ParseError, Result};
use crate::headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
use crate::messages::{
    MT101, MT103, MT104, MT107, MT110, MT111, MT112, MT192, MT196, MT199, MT202, MT205, MT210,
    MT292, MT296, MT299, MT900, MT910, MT920, MT935, MT940, MT941, MT942, MT950,
};
use crate::{ParsedSwiftMessage, SwiftMessage, SwiftMessageBody};

/// Type alias for the field parsing result
type FieldParseResult = Result<HashMap<String, Vec<String>>>;

/// Main parser for SWIFT MT messages
pub struct SwiftParser;

impl SwiftParser {
    /// Parse a raw SWIFT message string into a typed message
    pub fn parse<T: SwiftMessageBody>(raw_message: &str) -> Result<SwiftMessage<T>> {
        let block1 = Self::extract_block(raw_message, 1)?;
        let block2 = Self::extract_block(raw_message, 2)?;
        let block3 = Self::extract_block(raw_message, 3)?;
        let block4 = Self::extract_block(raw_message, 4)?;
        let block5 = Self::extract_block(raw_message, 5)?;

        // Parse headers
        let basic_header = BasicHeader::parse(&block1.unwrap_or_default())?;
        let application_header = ApplicationHeader::parse(&block2.unwrap_or_default())?;
        let user_header = block3.map(|b| UserHeader::parse(&b)).transpose()?;
        let trailer = block5.map(|b| Trailer::parse(&b)).transpose()?;

        // Extract message type from application header
        let message_type = application_header.message_type.clone();

        // Validate message type matches expected type
        if message_type != T::message_type() {
            return Err(ParseError::WrongMessageType {
                expected: T::message_type().to_string(),
                actual: message_type,
            });
        }

        // Parse block 4 fields
        let field_map = Self::parse_block4_fields(&block4.unwrap_or_default())?;

        // Parse message body using the field map
        let fields = T::from_fields(field_map)?;

        Ok(SwiftMessage {
            basic_header,
            application_header,
            user_header,
            trailer,
            message_type,
            fields,
        })
    }

    /// Parse a raw SWIFT message string with automatic message type detection
    pub fn parse_auto(raw_message: &str) -> Result<ParsedSwiftMessage> {
        // First, extract blocks to get the message type
        let block2 = Self::extract_block(raw_message, 2)?;

        // Parse application header to get message type
        let application_header = ApplicationHeader::parse(&block2.unwrap_or_default())?;
        let message_type = &application_header.message_type;

        // Route to appropriate parser based on message type
        match message_type.as_str() {
            "101" => {
                let parsed = Self::parse::<MT101>(raw_message)?;
                Ok(ParsedSwiftMessage::MT101(Box::new(parsed)))
            }
            "103" => {
                let parsed = Self::parse::<MT103>(raw_message)?;
                Ok(ParsedSwiftMessage::MT103(Box::new(parsed)))
            }
            "104" => {
                let parsed = Self::parse::<MT104>(raw_message)?;
                Ok(ParsedSwiftMessage::MT104(Box::new(parsed)))
            }
            "107" => {
                let parsed = Self::parse::<MT107>(raw_message)?;
                Ok(ParsedSwiftMessage::MT107(Box::new(parsed)))
            }
            "110" => {
                let parsed = Self::parse::<MT110>(raw_message)?;
                Ok(ParsedSwiftMessage::MT110(Box::new(parsed)))
            }
            "111" => {
                let parsed = Self::parse::<MT111>(raw_message)?;
                Ok(ParsedSwiftMessage::MT111(Box::new(parsed)))
            }
            "112" => {
                let parsed = Self::parse::<MT112>(raw_message)?;
                Ok(ParsedSwiftMessage::MT112(Box::new(parsed)))
            }
            "202" => {
                let parsed = Self::parse::<MT202>(raw_message)?;
                Ok(ParsedSwiftMessage::MT202(Box::new(parsed)))
            }
            "205" => {
                let parsed = Self::parse::<MT205>(raw_message)?;
                Ok(ParsedSwiftMessage::MT205(Box::new(parsed)))
            }
            "210" => {
                let parsed = Self::parse::<MT210>(raw_message)?;
                Ok(ParsedSwiftMessage::MT210(Box::new(parsed)))
            }
            "900" => {
                let parsed = Self::parse::<MT900>(raw_message)?;
                Ok(ParsedSwiftMessage::MT900(Box::new(parsed)))
            }
            "910" => {
                let parsed = Self::parse::<MT910>(raw_message)?;
                Ok(ParsedSwiftMessage::MT910(Box::new(parsed)))
            }
            "920" => {
                let parsed = Self::parse::<MT920>(raw_message)?;
                Ok(ParsedSwiftMessage::MT920(Box::new(parsed)))
            }
            "935" => {
                let parsed = Self::parse::<MT935>(raw_message)?;
                Ok(ParsedSwiftMessage::MT935(Box::new(parsed)))
            }
            "940" => {
                let parsed = Self::parse::<MT940>(raw_message)?;
                Ok(ParsedSwiftMessage::MT940(Box::new(parsed)))
            }
            "941" => {
                let parsed = Self::parse::<MT941>(raw_message)?;
                Ok(ParsedSwiftMessage::MT941(Box::new(parsed)))
            }
            "942" => {
                let parsed = Self::parse::<MT942>(raw_message)?;
                Ok(ParsedSwiftMessage::MT942(Box::new(parsed)))
            }
            "950" => {
                let parsed = Self::parse::<MT950>(raw_message)?;
                Ok(ParsedSwiftMessage::MT950(Box::new(parsed)))
            }
            "192" => {
                let parsed = Self::parse::<MT192>(raw_message)?;
                Ok(ParsedSwiftMessage::MT192(Box::new(parsed)))
            }
            "196" => {
                let parsed = Self::parse::<MT196>(raw_message)?;
                Ok(ParsedSwiftMessage::MT196(Box::new(parsed)))
            }
            "292" => {
                let parsed = Self::parse::<MT292>(raw_message)?;
                Ok(ParsedSwiftMessage::MT292(Box::new(parsed)))
            }
            "296" => {
                let parsed = Self::parse::<MT296>(raw_message)?;
                Ok(ParsedSwiftMessage::MT296(Box::new(parsed)))
            }
            "199" => {
                let parsed = Self::parse::<MT199>(raw_message)?;
                Ok(ParsedSwiftMessage::MT199(Box::new(parsed)))
            }
            "299" => {
                let parsed = Self::parse::<MT299>(raw_message)?;
                Ok(ParsedSwiftMessage::MT299(Box::new(parsed)))
            }
            _ => Err(ParseError::UnsupportedMessageType {
                message_type: message_type.clone(),
            }),
        }
    }

    /// Extract a specific message block from raw SWIFT message
    pub fn extract_block(raw_message: &str, block_index: u8) -> Result<Option<String>> {
        let block_marker = format!("{{{block_index}:");

        if let Some(start) = raw_message.find(&block_marker) {
            let content_start = start + block_marker.len();

            match block_index {
                1 | 2 | 5 => {
                    // Blocks 1, 2, and 5 end with simple closing brace
                    if let Some(end) = raw_message[start..].find('}') {
                        let end = start + end;
                        Ok(Some(raw_message[content_start..end].to_string()))
                    } else {
                        Ok(None)
                    }
                }
                3 => {
                    // Block 3 may have nested braces
                    if let Some(end) = Self::find_matching_brace(&raw_message[start..]) {
                        let end = start + end;
                        Ok(Some(raw_message[content_start..end].to_string()))
                    } else {
                        Ok(None)
                    }
                }
                4 => {
                    // Block 4 ends with "-}"
                    if let Some(end) = raw_message[start..].find("-}") {
                        let end = start + end;
                        Ok(Some(raw_message[content_start..end].to_string()))
                    } else {
                        Ok(None)
                    }
                }
                _ => Err(ParseError::InvalidBlockStructure {
                    message: format!("Invalid block index: {block_index}"),
                }),
            }
        } else {
            Ok(None)
        }
    }

    /// Parse block 4 fields into a field map
    fn parse_block4_fields(block4: &str) -> FieldParseResult {
        let mut field_map: HashMap<String, Vec<String>> = HashMap::new();

        // Remove leading/trailing whitespace and newlines
        let content = block4.trim();

        // Split by field markers (:XX:)
        let mut current_pos = 0;

        while current_pos < content.len() {
            // Find next field marker
            if let Some(field_start) = content[current_pos..].find(':') {
                let field_start = current_pos + field_start;

                // Extract field tag (characters after : until next :)
                if let Some(tag_end) = content[field_start + 1..].find(':') {
                    let tag_end = field_start + 1 + tag_end;
                    let raw_field_tag = content[field_start + 1..tag_end].to_string();

                    // Normalize field tag by removing option letters (A, F, K, etc.)
                    let field_tag = Self::normalize_field_tag(&raw_field_tag);

                    // Find the end of field value (next field marker or end of content)
                    let value_start = tag_end + 1;
                    let value_end = if let Some(next_field) = content[value_start..].find("\n:") {
                        value_start + next_field
                    } else {
                        content.len()
                    };

                    let field_value = content[value_start..value_end].trim().to_string();

                    // Store the complete field string including tag prefix for compatibility
                    let complete_field_string = format!(":{raw_field_tag}:{field_value}");

                    // Add to existing Vec or create new Vec for this field tag
                    field_map
                        .entry(field_tag.clone())
                        .or_default()
                        .push(complete_field_string);

                    current_pos = value_end;
                } else {
                    // Last field or malformed
                    break;
                }
            } else {
                break;
            }
        }

        Ok(field_map)
    }

    /// Normalize field tag by removing option letters (A, F, K, etc.)
    /// Example: "50K" -> "50", "59A" -> "59", "20" -> "20"
    /// But preserve option letters for fields that have multiple variants like 23B/23E, 71A/71F/71G
    fn normalize_field_tag(raw_tag: &str) -> String {
        // Extract the numeric part at the beginning
        let mut numeric_part = String::new();
        for ch in raw_tag.chars() {
            if ch.is_ascii_digit() {
                numeric_part.push(ch);
            } else {
                break;
            }
        }

        // If we have letters after the number, check if it's a known option
        if numeric_part.len() < raw_tag.len() {
            let remaining = &raw_tag[numeric_part.len()..];

            // For certain field numbers, preserve the option letter to avoid conflicts
            match numeric_part.as_str() {
                "11" | "13" | "23" | "26" | "32" | "33" | "52" | "53" | "54" | "55" | "56"
                | "57" | "58" | "71" | "77" => {
                    // Keep option letters for fields that have multiple variants or specific formats
                    // 11A (MT and Date - Option A), 11S (MT and Date - Option S)
                    // 13C (Time Indication)
                    // 23B (Bank Operation Code) vs 23E (Instruction Code)
                    // 26T (Transaction Type Code)
                    // 32A (Value Date/Currency/Amount)
                    // 33B (Currency/Instructed Amount)
                    // 52A (Ordering Institution)
                    // 53A (Sender's Correspondent)
                    // 54A (Receiver's Correspondent)
                    // 55A (Third Reimbursement Institution)
                    // 56A (Intermediary Institution)
                    // 57A (Account With Institution)
                    // 71A (Details of Charges) vs 71F (Sender's Charges) vs 71G (Receiver's Charges)
                    // 77B (Regulatory Reporting)
                    return raw_tag.to_string();
                }
                _ => {
                    // For other fields, remove option letters as before
                    if remaining
                        .chars()
                        .all(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
                    {
                        // It's an option letter, return just the numeric part
                        return numeric_part;
                    }
                }
            }
        }

        // If no option letter found, return the original tag
        raw_tag.to_string()
    }

    /// Find the matching closing brace for a block that starts with an opening brace
    /// Handles nested braces correctly
    fn find_matching_brace(text: &str) -> Option<usize> {
        let mut chars = text.char_indices();

        // Skip the first character (should be '{')
        let mut brace_count = if let Some((_, '{')) = chars.next() {
            1
        } else {
            return None;
        };

        for (i, ch) in chars {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        None
    }
}

/// Parse a SwiftMessage from a string representation
/// This is a placeholder implementation for the macro system
pub fn parse_swift_message_from_string(value: &str) -> Result<HashMap<String, Vec<String>>> {
    // For now, this is a stub implementation
    // In a real implementation, this would parse the string representation
    // of a SwiftMessage back into a field map

    // As a temporary solution, we'll assume the value is a simple field representation
    // and try to parse it as a mini SWIFT block
    let mut field_map = HashMap::new();

    // Split by lines and parse each field
    for line in value.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Look for field pattern :XX:value
        if let Some(colon_pos) = line.find(':') {
            if let Some(second_colon) = line[colon_pos + 1..].find(':') {
                let second_colon_pos = colon_pos + 1 + second_colon;
                let field_tag = line[colon_pos + 1..second_colon_pos].to_string();
                let _field_value = line[second_colon_pos + 1..].to_string();

                field_map
                    .entry(field_tag)
                    .or_insert_with(Vec::new)
                    .push(format!(":{}", &line[colon_pos + 1..]));
            }
        }
    }

    Ok(field_map)
}

/// Serialize a SwiftMessage field map to a string representation
/// This is a placeholder implementation for the macro system
pub fn serialize_swift_message_to_string(fields: &HashMap<String, Vec<String>>) -> String {
    // For now, this is a stub implementation
    // In a real implementation, this would serialize the field map
    // into a string representation of a SwiftMessage

    let mut result = String::new();

    // Simple serialization: just join all field values with newlines
    for field_values in fields.values() {
        for field_value in field_values {
            // field_value should already be in the format ":XX:value"
            result.push_str(field_value);
            result.push('\n');
        }
    }

    // Remove trailing newline
    if result.ends_with('\n') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_blocks_legacy() {
        let raw_message = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n-}";

        // Test using extract_block for each block
        let block1 = SwiftParser::extract_block(raw_message, 1).unwrap();
        let block2 = SwiftParser::extract_block(raw_message, 2).unwrap();
        let block3 = SwiftParser::extract_block(raw_message, 3).unwrap();
        let block4 = SwiftParser::extract_block(raw_message, 4).unwrap();
        let block5 = SwiftParser::extract_block(raw_message, 5).unwrap();

        assert!(block1.is_some());
        assert!(block2.is_some());
        assert!(block3.is_none());
        assert!(block4.is_some());
        assert!(block5.is_none());
        assert_eq!(block1.as_ref().unwrap(), "F01BANKDEFFAXXX0123456789");
        assert_eq!(block2.as_ref().unwrap(), "I103BANKDEFFAXXXU3003");
    }

    #[test]
    fn test_extract_block() {
        let raw_message = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n-}";

        // Test individual block extraction
        let block1 = SwiftParser::extract_block(raw_message, 1).unwrap();
        let block2 = SwiftParser::extract_block(raw_message, 2).unwrap();
        let block3 = SwiftParser::extract_block(raw_message, 3).unwrap();
        let block4 = SwiftParser::extract_block(raw_message, 4).unwrap();
        let block5 = SwiftParser::extract_block(raw_message, 5).unwrap();

        assert!(block1.is_some());
        assert!(block2.is_some());
        assert!(block3.is_none());
        assert!(block4.is_some());
        assert!(block5.is_none());

        assert_eq!(block1.unwrap(), "F01BANKDEFFAXXX0123456789");
        assert_eq!(block2.unwrap(), "I103BANKDEFFAXXXU3003");
        assert_eq!(block4.unwrap(), "\n:20:FT21234567890\n:23B:CRED\n");

        // Test invalid block index with a message that contains the block marker
        let invalid_message = "{6:INVALID}";
        let result = SwiftParser::extract_block(invalid_message, 6);
        assert!(result.is_err());

        // Test block that doesn't exist (should return Ok(None))
        let result_none = SwiftParser::extract_block(raw_message, 6);
        assert!(result_none.is_ok());
        assert!(result_none.unwrap().is_none());
    }

    #[test]
    fn test_parse_block4_fields() {
        let block4 = "\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n";
        let field_map = SwiftParser::parse_block4_fields(block4).unwrap();

        assert_eq!(
            field_map.get("20"),
            Some(&vec![":20:FT21234567890".to_string()])
        );
        assert_eq!(field_map.get("23B"), Some(&vec![":23B:CRED".to_string()]));
        assert_eq!(
            field_map.get("32A"),
            Some(&vec![":32A:210315EUR1234567,89".to_string()])
        );
    }

    #[test]
    fn test_debug_mt103_fields() {
        let block4 = r#"
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
"#;
        let field_map = SwiftParser::parse_block4_fields(block4).unwrap();

        println!("Extracted fields:");
        for (tag, values) in &field_map {
            println!("  {tag}: {values:?}");
        }

        // Check specific fields
        assert!(field_map.contains_key("20"));
        assert!(field_map.contains_key("23B"));
        assert!(field_map.contains_key("32A"));
        assert!(field_map.contains_key("50"));
        assert!(field_map.contains_key("52A"));
        assert!(field_map.contains_key("57A"));
        assert!(field_map.contains_key("59"));
        assert!(field_map.contains_key("70"));
        assert!(field_map.contains_key("71A"));
    }

    #[test]
    fn test_block3_parsing_with_nested_tags() {
        let raw_message = r#"{1:F01BNPAFRPPXXX0000000000}{2:O1031234240101DEUTDEFFXXXX12345678952401011234N}{3:{103:EBA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}}{4:
:20:FT21001234567890
:23B:CRED
-}"#;

        let block3 = SwiftParser::extract_block(raw_message, 3).unwrap();

        assert!(block3.is_some());
        let block3_content = block3.unwrap();
        println!("Block 3 content: '{block3_content}'");

        // Should contain both tags
        assert_eq!(
            block3_content,
            "{103:EBA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}"
        );
        assert!(block3_content.contains("103:EBA"));
        assert!(block3_content.contains("121:180f1e65-90e0-44d5-a49a-92b55eb3025f"));
    }

    #[test]
    fn test_find_matching_brace() {
        // Simple case: "{simple}" -> closing } at position 7
        assert_eq!(SwiftParser::find_matching_brace("{simple}"), Some(7));

        // Nested braces: "{outer{inner}outer}" -> closing } at position 18
        assert_eq!(
            SwiftParser::find_matching_brace("{outer{inner}outer}"),
            Some(18)
        );

        // SWIFT block 3 case: "{{103:EBA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}}"
        let test_str = "{{103:EBA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}}";
        let expected_pos = test_str.len() - 1; // Last character position
        assert_eq!(
            SwiftParser::find_matching_brace(test_str),
            Some(expected_pos)
        );

        // Simple nested: "{103:EBA}" -> closing } at position 8
        assert_eq!(SwiftParser::find_matching_brace("{103:EBA}"), Some(8));

        // No closing brace
        assert_eq!(SwiftParser::find_matching_brace("{no_close"), None);

        // Not starting with brace
        assert_eq!(SwiftParser::find_matching_brace("no_brace"), None);
    }

    #[test]
    fn debug_find_matching_brace() {
        let test_str = "{103:EBA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}";
        println!("Test string: '{test_str}'");
        println!("Length: {}", test_str.len());

        let result = SwiftParser::find_matching_brace(test_str);
        println!("Result: {result:?}");

        // Let's manually check what character is at different positions
        for (i, ch) in test_str.char_indices() {
            println!("Position {i}: '{ch}'");
        }
    }

    #[test]
    fn test_parse_auto_mt103() {
        let raw_mt103 = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:ACME CORPORATION
123 BUSINESS AVENUE
NEW YORK NY 10001
:52A:BANKDEFF
:57A:DEUTDEFF
:59A:/DE89370400440532013000
DEUTDEFF
:70:PAYMENT FOR SERVICES
:71A:OUR
-}"#;

        let parsed = SwiftParser::parse_auto(raw_mt103).unwrap();

        // Check that it detected the correct message type
        assert_eq!(parsed.message_type(), "103");

        // Check that we can extract the MT103 message
        let mt103_msg = parsed.as_mt103().unwrap();
        assert_eq!(mt103_msg.message_type, "103");

        println!("Successfully parsed MT103 message with auto-detection");
    }

    #[test]
    fn test_parse_auto_unsupported_type() {
        let raw_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I999BANKDEFFAXXXU3003}{4:
:20:FT21234567890
-}"#;

        let result = SwiftParser::parse_auto(raw_message);
        assert!(result.is_err());

        if let Err(ParseError::UnsupportedMessageType { message_type }) = result {
            assert_eq!(message_type, "999");
        } else {
            panic!("Expected UnsupportedMessageType error");
        }
    }
}
