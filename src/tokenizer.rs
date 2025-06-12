//! Block and field tokenizer for SWIFT MT messages
//!
//! This module handles the extraction of SWIFT message blocks and parsing of individual fields
//! from Block 4 (Text Block). It provides efficient tokenization with proper error handling
//! and supports streaming for large messages.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};


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

/// Static regex patterns for field extraction
static FIELD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r":(\d{2}[A-Z]?):").expect("Invalid field regex"));

/// Extract blocks from a SWIFT message using proper brace matching
pub fn extract_blocks(message: &str) -> Result<SwiftMessageBlocks> {
    let mut blocks = SwiftMessageBlocks::default();
    let chars: Vec<char> = message.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '{' && i + 2 < chars.len() && chars[i + 2] == ':' {
            // Found potential block start like {1:, {2:, etc.
            let block_number = chars[i + 1];
            
            if block_number.is_ascii_digit() {
                // Find the matching closing brace using proper brace counting
                let mut brace_count = 1;
                let content_start = i + 3; // Skip "{N:"
                let mut j = content_start;
                
                while j < chars.len() && brace_count > 0 {
                    match chars[j] {
                        '{' => brace_count += 1,
                        '}' => brace_count -= 1,
                        _ => {}
                    }
                    j += 1;
                }
                
                if brace_count == 0 {
                    // Found matching closing brace
                    let content_end = j - 1; // Exclude the closing brace
                    let block_content: String = chars[content_start..content_end].iter().collect();
                    
                    match block_number {
                        '1' => blocks.block_1 = Some(block_content),
                        '2' => blocks.block_2 = Some(block_content),
                        '3' => blocks.block_3 = Some(block_content),
                        '4' => blocks.block_4 = Some(block_content),
                        '5' => blocks.block_5 = Some(block_content),
                        _ => {
                            return Err(ParseError::UnknownBlockNumber {
                                block_number: block_number.to_string(),
                                line: 1, // TODO: Calculate actual line number
                                column: i + 1,
                            });
                        }
                    }
                    
                    i = j; // Move past this block
                } else {
                    return Err(ParseError::InvalidBlockFormat {
                        message: format!("Unmatched opening brace for block {}", block_number),
                        line: 1,
                        column: i + 1,
                    });
                }
            } else {
                i += 1;
            }
        } else {
            i += 1;
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

/// Basic Header (Block 1) structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BasicHeader {
    pub application_id: String,        // F = FIN, A = GPA, L = Login
    pub service_id: String,           // 01 = FIN/GPA, 21 = ACK/NAK
    pub logical_terminal: String,     // 12 character LT address
    pub session_number: String,       // 4 digit session number
    pub sequence_number: String,      // 6 digit sequence number
}

/// Application Header (Block 2) structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplicationHeader {
    pub input_output_identifier: String, // I = Input, O = Output
    pub message_type: String,            // 3 digit message type
    pub destination_address: String,     // 12 character destination LT
    pub priority: String,               // N = Normal, U = Urgent, S = System
    pub delivery_monitoring: Option<String>, // 1 = Non-delivery notification
    pub obsolescence_period: Option<String>, // 3 digit period
}

/// User Header (Block 3) structure based on SWIFT MT standards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserHeader {
    /// Tag 103 - Service Identifier (3!a) - Mandatory for FINcopy Service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_identifier: Option<String>,
    
    /// Tag 113 - Banking Priority (4!x) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banking_priority: Option<String>,
    
    /// Tag 108 - Message User Reference (16!x) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_user_reference: Option<String>,
    
    /// Tag 119 - Validation Flag (8c) - Optional (STP, REMIT, RFDD, COV)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_flag: Option<String>,
    
    /// Tag 423 - Balance checkpoint date and time (YYMMDDHHMMSS[ss]) - Optional (MIRS only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_checkpoint: Option<BalanceCheckpoint>,
    
    /// Tag 106 - Message Input Reference MIR (28c) - Optional (MIRS only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_input_reference: Option<MessageInputReference>,
    
    /// Tag 424 - Related reference (16x) - Optional (MIRS only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_reference: Option<String>,
    
    /// Tag 111 - Service type identifier (3!n) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type_identifier: Option<String>,
    
    /// Tag 121 - Unique end-to-end transaction reference (UUID format) - Mandatory for GPI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_end_to_end_reference: Option<String>,
    
    /// Tag 115 - Addressee Information (32x) - Optional (FINCopy only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressee_information: Option<String>,
    
    /// Tag 165 - Payment release information receiver (3!c/34x) - Optional (FINInform only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_release_information: Option<PaymentReleaseInfo>,
    
    /// Tag 433 - Sanctions screening information (3!a/[20x]) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sanctions_screening_info: Option<SanctionsScreeningInfo>,
    
    /// Tag 434 - Payment controls information (3!a/[20x]) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_controls_info: Option<PaymentControlsInfo>,
}

/// Balance checkpoint structure for Tag 423
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BalanceCheckpoint {
    pub date: String,                    // YYMMDD
    pub time: String,                    // HHMMSS
    pub hundredths_of_second: Option<String>, // ss (optional)
}

/// Message Input Reference structure for Tag 106
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageInputReference {
    pub date: String,              // YYMMDD
    pub lt_identifier: String,     // 12 characters
    pub branch_code: String,       // 3!c
    pub session_number: String,    // 4!n
    pub sequence_number: String,   // 6!n
}

/// Payment release information structure for Tag 165
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentReleaseInfo {
    pub code: String,                    // 3!c
    pub additional_info: Option<String>, // 34x (optional)
}

/// Sanctions screening information structure for Tag 433
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SanctionsScreeningInfo {
    pub code_word: String,               // 3!a (AOK, FPO, NOK)
    pub additional_info: Option<String>, // 20x (optional)
}

/// Payment controls information structure for Tag 434
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentControlsInfo {
    pub code_word: String,               // 3!a
    pub additional_info: Option<String>, // 20x (optional)
}

/// Trailer (Block 5) structure based on SWIFT MT standards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Trailer {
    /// CHK - Checksum (12!h) - Mandatory
    pub checksum: String,
    
    /// TNG - Test & Training Message - Optional (empty tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_and_training: Option<bool>,
    
    /// PDE - Possible Duplicate Emission - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_duplicate_emission: Option<PossibleDuplicateEmission>,
    
    /// DLM - Delayed Message - Optional (empty tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delayed_message: Option<bool>,
    
    /// MRF - Message Reference - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    
    /// PDM - Possible Duplicate Message - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_duplicate_message: Option<PossibleDuplicateMessage>,
    
    /// SYS - System Originated Message - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_originated_message: Option<SystemOriginatedMessage>,
}

/// Possible Duplicate Emission structure for PDE tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PossibleDuplicateEmission {
    pub time: Option<String>,                        // HHMM (optional)
    pub message_input_reference: Option<MessageInputReference>, // MIR (optional)
}

/// Message Reference structure for MRF tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReference {
    pub date: String,              // YYMMDD
    pub full_time: String,         // HHMM
    pub message_input_reference: MessageInputReference, // MIR
}

/// Possible Duplicate Message structure for PDM tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PossibleDuplicateMessage {
    pub time: Option<String>,                           // HHMM (optional)
    pub message_output_reference: Option<MessageOutputReference>, // MOR (optional)
}

/// Message Output Reference structure (similar to MIR but for output)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOutputReference {
    pub date: String,              // YYMMDD
    pub lt_identifier: String,     // 12 characters
    pub branch_code: String,       // 3!c
    pub session_number: String,    // 4!n
    pub sequence_number: String,   // 6!n
}

/// System Originated Message structure for SYS tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemOriginatedMessage {
    pub time: Option<String>,                        // HHMM (optional)
    pub message_input_reference: Option<MessageInputReference>, // MIR (optional)
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
                let dest = remaining[0..12].to_string();
                let prio = remaining.get(12..13).map(|s| s.to_string()).unwrap_or_default();
                let del_mon = remaining.get(13..14).map(|s| s.to_string());
                let obs_per = remaining.get(14..17).map(|s| s.to_string());
                (dest, prio, del_mon, obs_per)
            } else {
                (String::new(), String::new(), None, None)
            }
        } else if input_output_identifier == "O" {
            // Output message format - different structure
            (String::new(), String::new(), None, None) // TODO: Implement output message parsing
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

/// Parse user header from Block 3 using structured parsing
pub fn parse_user_header(block3_content: &str) -> Result<UserHeader> {
    let mut user_header = UserHeader::default();

    // Parse nested tags in format {tag:value}
    let tag_value_regex = Regex::new(r"\{([^:]+):([^}]*)\}").map_err(|e| ParseError::RegexError {
        message: e.to_string(),
    })?;

    for cap in tag_value_regex.captures_iter(block3_content) {
        if let (Some(tag), Some(value)) = (cap.get(1), cap.get(2)) {
            let tag_str = tag.as_str();
            let value_str = value.as_str();

            match tag_str {
                "103" => user_header.service_identifier = Some(value_str.to_string()),
                "113" => user_header.banking_priority = Some(value_str.to_string()),
                "108" => user_header.message_user_reference = Some(value_str.to_string()),
                "119" => user_header.validation_flag = Some(value_str.to_string()),
                "423" => user_header.balance_checkpoint = parse_balance_checkpoint(value_str)?,
                "106" => user_header.message_input_reference = parse_message_input_reference(value_str)?,
                "424" => user_header.related_reference = Some(value_str.to_string()),
                "111" => user_header.service_type_identifier = Some(value_str.to_string()),
                "121" => user_header.unique_end_to_end_reference = Some(value_str.to_string()),
                "115" => user_header.addressee_information = Some(value_str.to_string()),
                "165" => user_header.payment_release_information = parse_payment_release_info(value_str)?,
                "433" => user_header.sanctions_screening_info = parse_sanctions_screening_info(value_str)?,
                "434" => user_header.payment_controls_info = parse_payment_controls_info(value_str)?,
                _ => {
                    // Unknown tag - could log a warning but continue parsing
                }
            }
        }
    }

    Ok(user_header)
}

/// Parse trailer block from Block 5 using structured parsing
pub fn parse_trailer_block(block5_content: &str) -> Result<Trailer> {
    let mut trailer = Trailer {
        checksum: String::new(), // Will be set below
        ..Default::default()
    };

    // Parse nested tags in format {tag:value}
    let tag_value_regex = Regex::new(r"\{([^:]+):([^}]*)\}").map_err(|e| ParseError::RegexError {
        message: e.to_string(),
    })?;

    for cap in tag_value_regex.captures_iter(block5_content) {
        if let (Some(tag), Some(value)) = (cap.get(1), cap.get(2)) {
            let tag_str = tag.as_str();
            let value_str = value.as_str();

            match tag_str {
                "CHK" => trailer.checksum = value_str.to_string(),
                "TNG" => trailer.test_and_training = Some(true),
                "PDE" => trailer.possible_duplicate_emission = parse_possible_duplicate_emission(value_str)?,
                "DLM" => trailer.delayed_message = Some(true),
                "MRF" => trailer.message_reference = parse_message_reference(value_str)?,
                "PDM" => trailer.possible_duplicate_message = parse_possible_duplicate_message(value_str)?,
                "SYS" => trailer.system_originated_message = parse_system_originated_message(value_str)?,
                _ => {
                    // Unknown tag - could log a warning but continue parsing
                }
            }
        }
    }

    Ok(trailer)
}

/// Parse balance checkpoint from Tag 423 value
fn parse_balance_checkpoint(value: &str) -> Result<Option<BalanceCheckpoint>> {
    if value.is_empty() {
        return Ok(None);
    }

    if value.len() < 12 {
        return Ok(None); // Invalid format
    }

    let date = value[0..6].to_string();
    let time = value[6..12].to_string();
    let hundredths = if value.len() > 12 {
        Some(value[12..].to_string())
    } else {
        None
    };

    Ok(Some(BalanceCheckpoint {
        date,
        time,
        hundredths_of_second: hundredths,
    }))
}

/// Parse Message Input Reference from Tag 106 value
fn parse_message_input_reference(value: &str) -> Result<Option<MessageInputReference>> {
    if value.is_empty() || value.len() < 28 {
        return Ok(None); // Invalid MIR format
    }

    Ok(Some(MessageInputReference {
        date: value[0..6].to_string(),
        lt_identifier: value[6..18].to_string(),
        branch_code: value[18..21].to_string(),
        session_number: value[21..25].to_string(),
        sequence_number: value[25..].to_string(),
    }))
}

/// Parse payment release information from Tag 165 value
fn parse_payment_release_info(value: &str) -> Result<Option<PaymentReleaseInfo>> {
    if value.is_empty() {
        return Ok(None);
    }

    // Format: /3!c/[34x]
    if value.starts_with('/') && value.len() >= 4 {
        let parts: Vec<&str> = value[1..].split('/').collect();
        if !parts.is_empty() {
            let code = parts[0].to_string();
            let additional_info = if parts.len() > 1 && !parts[1].is_empty() {
                Some(parts[1].to_string())
            } else {
                None
            };
            return Ok(Some(PaymentReleaseInfo { code, additional_info }));
        }
    }

    Ok(None)
}

/// Parse sanctions screening information from Tag 433 value
fn parse_sanctions_screening_info(value: &str) -> Result<Option<SanctionsScreeningInfo>> {
    if value.is_empty() {
        return Ok(None);
    }

    // Format: /3!a/[20x]
    if value.starts_with('/') && value.len() >= 4 {
        let parts: Vec<&str> = value[1..].split('/').collect();
        if !parts.is_empty() {
            let code_word = parts[0].to_string();
            let additional_info = if parts.len() > 1 && !parts[1].is_empty() {
                Some(parts[1].to_string())
            } else {
                None
            };
            return Ok(Some(SanctionsScreeningInfo { code_word, additional_info }));
        }
    }

    Ok(None)
}

/// Parse payment controls information from Tag 434 value
fn parse_payment_controls_info(value: &str) -> Result<Option<PaymentControlsInfo>> {
    if value.is_empty() {
        return Ok(None);
    }

    // Format: /3!a/[20x]
    if value.starts_with('/') && value.len() >= 4 {
        let parts: Vec<&str> = value[1..].split('/').collect();
        if !parts.is_empty() {
            let code_word = parts[0].to_string();
            let additional_info = if parts.len() > 1 && !parts[1].is_empty() {
                Some(parts[1].to_string())
            } else {
                None
            };
            return Ok(Some(PaymentControlsInfo { code_word, additional_info }));
        }
    }

    Ok(None)
}

/// Parse possible duplicate emission from PDE tag value
fn parse_possible_duplicate_emission(value: &str) -> Result<Option<PossibleDuplicateEmission>> {
    if value.is_empty() {
        return Ok(Some(PossibleDuplicateEmission {
            time: None,
            message_input_reference: None,
        }));
    }

    // Format can be: time+MIR, time only, MIR only, or empty
    let time = if value.len() >= 4 {
        Some(value[0..4].to_string())
    } else {
        None
    };

    let mir = if value.len() > 4 {
        parse_message_input_reference(&value[4..])?
    } else {
        None
    };

    Ok(Some(PossibleDuplicateEmission {
        time,
        message_input_reference: mir,
    }))
}

/// Parse message reference from MRF tag value
fn parse_message_reference(value: &str) -> Result<Option<MessageReference>> {
    if value.is_empty() || value.len() < 38 {
        return Ok(None); // Invalid format
    }

    let date = value[0..6].to_string();
    let full_time = value[6..10].to_string();
    let mir_part = &value[10..];

    let mir = parse_message_input_reference(mir_part)?
        .ok_or_else(|| ParseError::InvalidBlockFormat {
            message: "Invalid MIR in MRF tag".to_string(),
            line: 1,
            column: 1,
        })?;

    Ok(Some(MessageReference {
        date,
        full_time,
        message_input_reference: mir,
    }))
}

/// Parse possible duplicate message from PDM tag value
fn parse_possible_duplicate_message(value: &str) -> Result<Option<PossibleDuplicateMessage>> {
    if value.is_empty() {
        return Ok(Some(PossibleDuplicateMessage {
            time: None,
            message_output_reference: None,
        }));
    }

    // Format similar to PDE but with MOR instead of MIR
    let time = if value.len() >= 4 {
        Some(value[0..4].to_string())
    } else {
        None
    };

    let mor = if value.len() > 4 {
        parse_message_output_reference(&value[4..])?
    } else {
        None
    };

    Ok(Some(PossibleDuplicateMessage {
        time,
        message_output_reference: mor,
    }))
}

/// Parse message output reference (similar to MIR)
fn parse_message_output_reference(value: &str) -> Result<Option<MessageOutputReference>> {
    if value.is_empty() || value.len() < 28 {
        return Ok(None); // Invalid MOR format
    }

    Ok(Some(MessageOutputReference {
        date: value[0..6].to_string(),
        lt_identifier: value[6..18].to_string(),
        branch_code: value[18..21].to_string(),
        session_number: value[21..25].to_string(),
        sequence_number: value[25..].to_string(),
    }))
}

/// Parse system originated message from SYS tag value
fn parse_system_originated_message(value: &str) -> Result<Option<SystemOriginatedMessage>> {
    if value.is_empty() {
        return Ok(Some(SystemOriginatedMessage {
            time: None,
            message_input_reference: None,
        }));
    }

    // Format similar to PDE
    let time = if value.len() >= 4 {
        Some(value[0..4].to_string())
    } else {
        None
    };

    let mir = if value.len() > 4 {
        parse_message_input_reference(&value[4..])?
    } else {
        None
    };

    Ok(Some(SystemOriginatedMessage {
        time,
        message_input_reference: mir,
    }))
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
    fn test_extract_blocks_with_nested_braces() {
        // Test message with Block 3 containing nested braces (user header)
        let message = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{3:{113:SEPA}{108:MT103}{119:STP}}{4:\n:20:FT21234567890\n:23B:CRED\n-}{5:{MAC:12345678}{CHK:ABCDEF}}";

        let blocks = extract_blocks(message).unwrap();

        assert!(blocks.block_1.is_some());
        assert_eq!(blocks.block_1.unwrap(), "F01BANKDEFFAXXX0123456789");

        assert!(blocks.block_2.is_some());
        assert_eq!(blocks.block_2.unwrap(), "I103BANKDEFFAXXXU3003");

        assert!(blocks.block_3.is_some());
        assert_eq!(blocks.block_3.unwrap(), "{113:SEPA}{108:MT103}{119:STP}");

        assert!(blocks.block_4.is_some());
        assert!(blocks.block_4.unwrap().contains(":20:FT21234567890"));

        assert!(blocks.block_5.is_some());
        assert_eq!(blocks.block_5.unwrap(), "{MAC:12345678}{CHK:ABCDEF}");
    }

    #[test]
    fn test_extract_blocks_single_nested_brace() {
        // Test with just one nested tag in Block 3
        let message = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{3:{113:SEPA}}{4:\n:20:TEST\n-}";

        let blocks = extract_blocks(message).unwrap();

        assert!(blocks.block_3.is_some());
        assert_eq!(blocks.block_3.unwrap(), "{113:SEPA}");
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
        assert_eq!(header.destination_address, "BANKDEFFAXXX");
        assert_eq!(header.priority, "U");
    }
}
