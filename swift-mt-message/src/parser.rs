use std::collections::HashMap;

use crate::errors::{ParseError, Result};
use crate::headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
use crate::messages::{MT103, MT202, MT205};
use crate::{ParsedSwiftMessage, RawBlocks, SwiftMessage, SwiftMessageBody};

/// Type alias for the complex return type of field parsing
type FieldParseResult = Result<(HashMap<String, Vec<String>>, Vec<String>)>;

/// Main parser for SWIFT MT messages
pub struct SwiftParser;

impl SwiftParser {
    /// Parse a raw SWIFT message string into a typed message
    pub fn parse<T: SwiftMessageBody>(raw_message: &str) -> Result<SwiftMessage<T>> {
        let blocks = Self::extract_blocks(raw_message)?;

        // Parse headers
        let basic_header = BasicHeader::parse(&blocks.block1.clone().unwrap_or_default())?;
        let application_header =
            ApplicationHeader::parse(&blocks.block2.clone().unwrap_or_default())?;
        let user_header = blocks
            .block3
            .as_ref()
            .map(|b| UserHeader::parse(b))
            .transpose()?;
        let trailer = blocks
            .block5
            .as_ref()
            .map(|b| Trailer::parse(b))
            .transpose()?;

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
        let (field_map, field_order) = Self::parse_block4_fields(&blocks.block4)?;

        // Parse message body using the field map
        let fields = T::from_fields(field_map)?;

        Ok(SwiftMessage {
            basic_header,
            application_header,
            user_header,
            trailer,
            blocks,
            message_type,
            field_order,
            fields,
        })
    }

    /// Parse a raw SWIFT message string with automatic message type detection
    pub fn parse_auto(raw_message: &str) -> Result<ParsedSwiftMessage> {
        // First, extract blocks to get the message type
        let blocks = Self::extract_blocks(raw_message)?;

        // Parse application header to get message type
        let application_header =
            ApplicationHeader::parse(&blocks.block2.clone().unwrap_or_default())?;
        let message_type = &application_header.message_type;

        // Route to appropriate parser based on message type
        match message_type.as_str() {
            "103" => {
                let parsed = Self::parse::<MT103>(raw_message)?;
                Ok(ParsedSwiftMessage::MT103(Box::new(parsed)))
            }
            "202" => {
                let parsed = Self::parse::<MT202>(raw_message)?;
                Ok(ParsedSwiftMessage::MT202(Box::new(parsed)))
            }
            "205" => {
                let parsed = Self::parse::<MT205>(raw_message)?;
                Ok(ParsedSwiftMessage::MT205(Box::new(parsed)))
            }
            _ => Err(ParseError::UnsupportedMessageType {
                message_type: message_type.clone(),
            }),
        }
    }

    /// Extract message blocks from raw SWIFT message
    pub fn extract_blocks(raw_message: &str) -> Result<RawBlocks> {
        let mut blocks = RawBlocks::default();

        // Find block boundaries
        let mut current_pos = 0;

        // Block 1: Basic Header {1:...}
        if let Some(start) = raw_message[current_pos..].find("{1:") {
            let start = current_pos + start;
            if let Some(end) = raw_message[start..].find('}') {
                let end = start + end;
                blocks.block1 = Some(raw_message[start + 3..end].to_string());
                current_pos = end + 1;
            }
        }

        // Block 2: Application Header {2:...}
        if let Some(start) = raw_message[current_pos..].find("{2:") {
            let start = current_pos + start;
            if let Some(end) = raw_message[start..].find('}') {
                let end = start + end;
                blocks.block2 = Some(raw_message[start + 3..end].to_string());
                current_pos = end + 1;
            }
        }

        // Block 3: User Header {3:...} (optional)
        if let Some(start) = raw_message[current_pos..].find("{3:") {
            let start = current_pos + start;
            // Find matching closing brace for block 3
            if let Some(end) = Self::find_matching_brace(&raw_message[start..]) {
                let end = start + end;
                blocks.block3 = Some(raw_message[start + 3..end].to_string());
                current_pos = end + 1;
            }
        }

        // Block 4: Text Block {4:\n...-}
        if let Some(start) = raw_message[current_pos..].find("{4:") {
            let start = current_pos + start;
            if let Some(end) = raw_message[start..].find("-}") {
                let end = start + end;
                blocks.block4 = raw_message[start + 3..end].to_string();
                current_pos = end + 2;
            }
        }

        // Block 5: Trailer {5:...} (optional)
        if let Some(start) = raw_message[current_pos..].find("{5:") {
            let start = current_pos + start;
            if let Some(end) = raw_message[start..].find('}') {
                let end = start + end;
                blocks.block5 = Some(raw_message[start + 3..end].to_string());
            }
        }

        if blocks.block1.is_none() || blocks.block2.is_none() || blocks.block4.is_empty() {
            return Err(ParseError::InvalidBlockStructure {
                message: "Missing required blocks (1, 2, or 4)".to_string(),
            });
        }

        Ok(blocks)
    }

    /// Parse block 4 fields into a field map and preserve field order
    fn parse_block4_fields(block4: &str) -> FieldParseResult {
        let mut field_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut field_order = Vec::new();

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
                    let complete_field_string = format!(":{}:{}", raw_field_tag, field_value);

                    // Add to existing Vec or create new Vec for this field tag
                    field_map
                        .entry(field_tag.clone())
                        .or_default()
                        .push(complete_field_string);

                    // Only add to field_order if this is the first occurrence of this field
                    if !field_order.contains(&field_tag) {
                        field_order.push(field_tag);
                    }

                    current_pos = value_end;
                } else {
                    // Last field or malformed
                    break;
                }
            } else {
                break;
            }
        }

        Ok((field_map, field_order))
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
                "13" | "23" | "26" | "32" | "33" | "52" | "53" | "54" | "55" | "56" | "57"
                | "58" | "71" | "77" => {
                    // Keep option letters for fields that have multiple variants or specific formats
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_blocks() {
        let raw_message = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n-}";
        let blocks = SwiftParser::extract_blocks(raw_message).unwrap();

        assert!(blocks.block1.is_some());
        assert!(blocks.block2.is_some());
        assert!(!blocks.block4.is_empty());
        assert_eq!(blocks.block1.as_ref().unwrap(), "F01BANKDEFFAXXX0123456789");
        assert_eq!(blocks.block2.as_ref().unwrap(), "I103BANKDEFFAXXXU3003");
    }

    #[test]
    fn test_parse_block4_fields() {
        let block4 = "\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n";
        let (field_map, field_order) = SwiftParser::parse_block4_fields(block4).unwrap();

        assert_eq!(
            field_map.get("20"),
            Some(&vec![":20:FT21234567890".to_string()])
        );
        assert_eq!(field_map.get("23B"), Some(&vec![":23B:CRED".to_string()]));
        assert_eq!(
            field_map.get("32A"),
            Some(&vec![":32A:210315EUR1234567,89".to_string()])
        );

        assert_eq!(field_order, vec!["20", "23B", "32A"]);
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
        let (field_map, field_order) = SwiftParser::parse_block4_fields(block4).unwrap();

        println!("Extracted fields:");
        for (tag, values) in &field_map {
            println!("  {}: {:?}", tag, values);
        }
        println!("Field order: {:?}", field_order);

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

        let blocks = SwiftParser::extract_blocks(raw_message).unwrap();

        assert!(blocks.block3.is_some());
        let block3_content = blocks.block3.unwrap();
        println!("Block 3 content: '{}'", block3_content);

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
        println!("Test string: '{}'", test_str);
        println!("Length: {}", test_str.len());

        let result = SwiftParser::find_matching_brace(test_str);
        println!("Result: {:?}", result);

        // Let's manually check what character is at different positions
        for (i, ch) in test_str.char_indices() {
            println!("Position {}: '{}'", i, ch);
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
