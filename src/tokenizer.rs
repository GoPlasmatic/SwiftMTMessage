//! Block and field tokenizer for SWIFT MT messages
//!
//! This module handles the extraction of SWIFT message blocks and parsing of individual fields
//! from Block 4 (Text Block). It provides efficient tokenization with proper error handling
//! and supports streaming for large messages.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::{ParseError, Result};

/// SWIFT message blocks structure
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SwiftMessageBlocks {
    pub block_1: Option<String>,
    pub block_2: Option<String>,
    pub block_3: Option<String>,
    pub block_4: Option<String>,
    pub block_5: Option<String>,
}

/// Parsed field structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedField {
    pub tag: String,        // e.g., "50A", "20", "32A"
    pub content: String,    // Raw field content
    pub line_number: usize, // Line number for error reporting
    pub column: usize,      // Column position for error reporting
}

/// Static regex patterns for block and field extraction
static BLOCK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{(\d):([^}]*)\}").expect("Invalid block regex"));

static FIELD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r":(\d{2}[A-Z]?):").expect("Invalid field regex"));

/// Extract blocks from a SWIFT message
pub fn extract_blocks(message: &str) -> Result<SwiftMessageBlocks> {
    let mut blocks = SwiftMessageBlocks::default();

    for cap in BLOCK_REGEX.captures_iter(message) {
        let block_number = cap
            .get(1)
            .ok_or_else(|| ParseError::InvalidBlockFormat {
                message: "Missing block number".to_string(),
                line: 1,
                column: 1,
            })?
            .as_str();

        let block_content = cap
            .get(2)
            .ok_or_else(|| ParseError::InvalidBlockFormat {
                message: "Missing block content".to_string(),
                line: 1,
                column: 1,
            })?
            .as_str();

        match block_number {
            "1" => blocks.block_1 = Some(block_content.to_string()),
            "2" => blocks.block_2 = Some(block_content.to_string()),
            "3" => blocks.block_3 = Some(block_content.to_string()),
            "4" => blocks.block_4 = Some(block_content.to_string()),
            "5" => blocks.block_5 = Some(block_content.to_string()),
            _ => {
                return Err(ParseError::UnknownBlockNumber {
                    block_number: block_number.to_string(),
                    line: 1, // TODO: Calculate actual line number
                    column: 1,
                });
            }
        }
    }

    if blocks.block_1.is_none() && blocks.block_2.is_none() && blocks.block_4.is_none() {
        return Err(ParseError::NoBlocksFound {
            message: "No valid SWIFT blocks found in message".to_string(),
        });
    }

    Ok(blocks)
}

/// Parse Block 4 fields into individual field structures
pub fn parse_block4_fields(block4_content: &str) -> Result<Vec<ParsedField>> {
    if block4_content.is_empty() {
        return Ok(Vec::new());
    }

    let mut fields = Vec::new();
    let _current_line = 1usize;
    let lines = block4_content.lines().enumerate();

    // Find all field tags and their positions
    let mut field_positions = Vec::new();

    for (line_idx, line) in lines {
        if let Some(captures) = FIELD_REGEX.captures(line) {
            if let Some(tag_match) = captures.get(1) {
                field_positions.push((
                    tag_match.as_str().to_string(),
                    line_idx + 1,
                    tag_match.start(),
                ));
            }
        }
    }

    if field_positions.is_empty() {
        return Ok(Vec::new());
    }

    // Split content by field boundaries
    let content_lines: Vec<&str> = block4_content.lines().collect();

    for (i, (tag, line_number, column)) in field_positions.iter().enumerate() {
        let start_line = line_number - 1; // Convert to 0-based
        let end_line = if i + 1 < field_positions.len() {
            field_positions[i + 1].1 - 1 // Next field's line - 1
        } else {
            content_lines.len() // End of content
        };

        // Extract field content
        let mut field_content = String::new();

        // First line: content after the tag
        if start_line < content_lines.len() {
            let first_line = content_lines[start_line];
            if let Some(tag_end) = first_line.find(&format!(":{}:", tag)) {
                let content_start = tag_end + tag.len() + 2; // Skip ":tag:"
                if content_start < first_line.len() {
                    field_content.push_str(&first_line[content_start..]);
                }
            }
        }

        // Subsequent lines until next field or end
        for line_idx in (start_line + 1)..end_line {
            if line_idx < content_lines.len() {
                if !field_content.is_empty() {
                    field_content.push('\n');
                }
                field_content.push_str(content_lines[line_idx]);
            }
        }

        // Clean up field content - remove message terminator and trim
        let cleaned_content = field_content
            .trim()
            .trim_end_matches('-') // Remove trailing dash (message terminator)
            .trim() // Trim again after removing dash
            .to_string();

        fields.push(ParsedField {
            tag: tag.clone(),
            content: cleaned_content,
            line_number: *line_number,
            column: *column,
        });
    }

    Ok(fields)
}

/// Extract message type from blocks
pub fn extract_message_type(blocks: &SwiftMessageBlocks) -> Result<String> {
    let block2 = blocks
        .block_2
        .as_ref()
        .ok_or_else(|| ParseError::MissingRequiredBlock {
            block: "2".to_string(),
            message: "Block 2 is required to determine message type".to_string(),
            line: 1,
            column: 1,
        })?;

    if block2.len() < 4 {
        return Err(ParseError::InvalidBlockFormat {
            message: "Block 2 too short to contain message type".to_string(),
            line: 1,
            column: 1,
        });
    }

    // Message type is characters 1-3 in block 2 (after I/O identifier)
    Ok(block2[1..4].to_string())
}

/// Basic header information extracted from Block 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicHeader {
    pub application_id: String,
    pub service_id: String,
    pub logical_terminal: String,
    pub session_number: String,
    pub sequence_number: String,
}

/// Parse basic header from Block 1
pub fn parse_basic_header(block1_content: &str) -> Result<BasicHeader> {
    if block1_content.len() < 21 {
        return Err(ParseError::InvalidBlockFormat {
            message: format!(
                "Basic header block too short: expected at least 21 characters, got {}",
                block1_content.len()
            ),
            line: 1,
            column: 1,
        });
    }

    Ok(BasicHeader {
        application_id: block1_content[0..1].to_string(),
        service_id: block1_content[1..3].to_string(),
        logical_terminal: block1_content[3..15].to_string(),
        session_number: block1_content[15..19].to_string(),
        sequence_number: block1_content[19..].to_string(),
    })
}

/// Application header information extracted from Block 2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationHeader {
    pub input_output_identifier: String,
    pub message_type: String,
    pub destination_address: Option<String>,
    pub priority: Option<String>,
    pub delivery_monitoring: Option<String>,
    pub obsolescence_period: Option<String>,
}

/// Parse application header from Block 2
pub fn parse_application_header(block2_content: &str) -> Result<ApplicationHeader> {
    if block2_content.is_empty() {
        return Err(ParseError::InvalidBlockFormat {
            message: "Application header block is empty".to_string(),
            line: 1,
            column: 1,
        });
    }

    if block2_content.len() < 4 {
        return Err(ParseError::InvalidBlockFormat {
            message: format!(
                "Application header block too short: expected at least 4 characters, got {}",
                block2_content.len()
            ),
            line: 1,
            column: 1,
        });
    }

    let input_output_identifier = block2_content[0..1].to_string();
    let message_type = block2_content[1..4].to_string();
    let remaining = &block2_content[4..];

    let (destination_address, priority, delivery_monitoring, obsolescence_period) =
        if input_output_identifier == "I" {
            // Input message format
            if remaining.len() >= 12 {
                let dest = Some(remaining[0..12].to_string());
                let prio = remaining.get(12..13).map(|s| s.to_string());
                let del_mon = remaining.get(13..14).map(|s| s.to_string());
                let obs_per = remaining.get(14..17).map(|s| s.to_string());
                (dest, prio, del_mon, obs_per)
            } else {
                (None, None, None, None)
            }
        } else if input_output_identifier == "O" {
            // Output message format - different structure
            (None, None, None, None) // TODO: Implement output message parsing
        } else {
            return Err(ParseError::InvalidBlockFormat {
                message: format!("Invalid I/O identifier: {}", input_output_identifier),
                line: 1,
                column: 1,
            });
        };

    Ok(ApplicationHeader {
        input_output_identifier,
        message_type,
        destination_address,
        priority,
        delivery_monitoring,
        obsolescence_period,
    })
}

/// Parse user header from Block 3 (optional)
pub fn parse_user_header(block3_content: &str) -> Result<HashMap<String, String>> {
    let mut fields = HashMap::new();

    // Block 3 contains optional user header fields in format {tag:value}
    let tag_value_regex =
        Regex::new(r"\{([^:]+):([^}]*)\}").map_err(|e| ParseError::RegexError {
            message: e.to_string(),
        })?;

    for cap in tag_value_regex.captures_iter(block3_content) {
        if let (Some(tag), Some(value)) = (cap.get(1), cap.get(2)) {
            fields.insert(tag.as_str().to_string(), value.as_str().to_string());
        }
    }

    Ok(fields)
}

/// Parse trailer block from Block 5 (optional)
pub fn parse_trailer_block(block5_content: &str) -> Result<HashMap<String, String>> {
    let mut fields = HashMap::new();

    // Block 5 contains optional trailer fields in format {tag:value}
    let tag_value_regex =
        Regex::new(r"\{([^:]+):([^}]*)\}").map_err(|e| ParseError::RegexError {
            message: e.to_string(),
        })?;

    for cap in tag_value_regex.captures_iter(block5_content) {
        if let (Some(tag), Some(value)) = (cap.get(1), cap.get(2)) {
            fields.insert(tag.as_str().to_string(), value.as_str().to_string());
        }
    }

    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_blocks() {
        let message = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n-}";

        let blocks = extract_blocks(message).unwrap();

        assert!(blocks.block_1.is_some());
        assert_eq!(blocks.block_1.unwrap(), "F01BANKDEFFAXXX0123456789");

        assert!(blocks.block_2.is_some());
        assert_eq!(blocks.block_2.unwrap(), "I103BANKDEFFAXXXU3003");

        assert!(blocks.block_4.is_some());
        assert!(blocks.block_4.unwrap().contains(":20:FT21234567890"));
    }

    #[test]
    fn test_parse_block4_fields() {
        let block4 = ":20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\nACME CORP\n123 MAIN ST\n:59:JANE SMITH\nXYZ COMPANY";

        let fields = parse_block4_fields(block4).unwrap();

        assert_eq!(fields.len(), 5);
        assert_eq!(fields[0].tag, "20");
        assert_eq!(fields[0].content, "FT21234567890");
        assert_eq!(fields[1].tag, "23B");
        assert_eq!(fields[1].content, "CRED");
        assert_eq!(fields[3].tag, "50K");
        assert_eq!(fields[3].content, "JOHN DOE\nACME CORP\n123 MAIN ST");
    }

    #[test]
    fn test_extract_message_type() {
        let blocks = SwiftMessageBlocks {
            block_1: Some("F01BANKDEFFAXXX0123456789".to_string()),
            block_2: Some("I103BANKDEFFAXXXU3003".to_string()),
            block_3: None,
            block_4: Some(":20:TEST".to_string()),
            block_5: None,
        };

        let message_type = extract_message_type(&blocks).unwrap();
        assert_eq!(message_type, "103");
    }

    #[test]
    fn test_parse_basic_header() {
        let block1 = "F01BANKDEFFAXXX0123456789";
        let header = parse_basic_header(block1).unwrap();

        assert_eq!(header.application_id, "F");
        assert_eq!(header.service_id, "01");
        assert_eq!(header.logical_terminal, "BANKDEFFAXXX");
        assert_eq!(header.session_number, "0123");
        assert_eq!(header.sequence_number, "456789");
    }

    #[test]
    fn test_parse_application_header() {
        let block2 = "I103BANKDEFFAXXXU3003";
        let header = parse_application_header(block2).unwrap();

        assert_eq!(header.input_output_identifier, "I");
        assert_eq!(header.message_type, "103");
        assert_eq!(header.destination_address, Some("BANKDEFFAXXX".to_string()));
        assert_eq!(header.priority, Some("U".to_string()));
    }
}
