//! Core parsing logic for SWIFT MT messages

use regex::Regex;
use std::collections::HashMap;

use crate::common::{Field, MessageBlock};
use crate::error::{MTError, Result};
use crate::messages::{MTMessage, MTMessageType};

/// Main parser for SWIFT MT messages
pub struct MTParser {
    block_regex: Regex,
}

impl MTParser {
    pub fn new() -> Result<Self> {
        let block_regex = Regex::new(r"\{(\d):([^}]*)\}")?;

        Ok(Self { block_regex })
    }

    /// Parse a complete SWIFT MT message
    pub fn parse(&self, input: &str) -> Result<MTMessage> {
        let blocks = self.parse_blocks(input)?;
        let message_type = self.extract_message_type(&blocks)?;

        match message_type.as_str() {
            "103" => {
                let mt103 = crate::messages::mt103::MT103::from_blocks(blocks)?;
                Ok(MTMessage::MT103(mt103))
            }
            "102" => {
                let mt102 = crate::messages::mt102::MT102::from_blocks(blocks)?;
                Ok(MTMessage::MT102(mt102))
            }
            "202" => {
                // Check if this is a COV message by looking for customer fields
                let text_block = self.extract_text_block(&blocks)?;
                let has_ordering_customer = text_block.iter().any(|f| {
                    f.tag.as_str() == "50K" || f.tag.as_str() == "50A" || f.tag.as_str() == "50F"
                });
                let has_beneficiary_customer = text_block.iter().any(|f| {
                    f.tag.as_str() == "59" || f.tag.as_str() == "59A" || f.tag.as_str() == "59F"
                });

                if has_ordering_customer && has_beneficiary_customer {
                    // This is MT202COV
                    let mt202cov = crate::messages::mt202cov::MT202COV::from_blocks(blocks)?;
                    Ok(MTMessage::MT202COV(mt202cov))
                } else {
                    // This is regular MT202
                    let mt202 = crate::messages::mt202::MT202::from_blocks(blocks)?;
                    Ok(MTMessage::MT202(mt202))
                }
            }
            "210" => {
                let mt210 = crate::messages::mt210::MT210::from_blocks(blocks)?;
                Ok(MTMessage::MT210(mt210))
            }
            "940" => {
                let mt940 = crate::messages::mt940::MT940::from_blocks(blocks)?;
                Ok(MTMessage::MT940(mt940))
            }
            "941" => {
                let mt941 = crate::messages::mt941::MT941::from_blocks(blocks)?;
                Ok(MTMessage::MT941(mt941))
            }
            "942" => {
                let mt942 = crate::messages::mt942::MT942::from_blocks(blocks)?;
                Ok(MTMessage::MT942(mt942))
            }
            "192" => {
                let mt192 = crate::messages::mt192::MT192::from_blocks(blocks)?;
                Ok(MTMessage::MT192(mt192))
            }
            "195" => {
                let mt195 = crate::messages::mt195::MT195::from_blocks(blocks)?;
                Ok(MTMessage::MT195(mt195))
            }
            "196" => {
                let mt196 = crate::messages::mt196::MT196::from_blocks(blocks)?;
                Ok(MTMessage::MT196(mt196))
            }
            "197" => {
                let mt197 = crate::messages::mt197::MT197::from_blocks(blocks)?;
                Ok(MTMessage::MT197(mt197))
            }
            "199" => {
                let mt199 = crate::messages::mt199::MT199::from_blocks(blocks)?;
                Ok(MTMessage::MT199(mt199))
            }
            _ => Err(MTError::UnsupportedMessageType {
                message_type: message_type.clone(),
            }),
        }
    }

    /// Parse message blocks from input
    pub fn parse_blocks(&self, input: &str) -> Result<Vec<MessageBlock>> {
        let mut blocks = Vec::new();

        for cap in self.block_regex.captures_iter(input) {
            let block_number = cap
                .get(1)
                .ok_or_else(|| MTError::ParseError {
                    line: 1,
                    column: 1,
                    message: "Invalid block format".to_string(),
                })?
                .as_str();

            let block_content = cap
                .get(2)
                .ok_or_else(|| MTError::ParseError {
                    line: 1,
                    column: 1,
                    message: "Invalid block content".to_string(),
                })?
                .as_str();

            match block_number {
                "1" => blocks.push(self.parse_basic_header(block_content)?),
                "2" => blocks.push(self.parse_application_header(block_content)?),
                "3" => blocks.push(self.parse_user_header(block_content)?),
                "4" => blocks.push(self.parse_text_block(block_content)?),
                "5" => blocks.push(self.parse_trailer_block(block_content)?),
                _ => {
                    return Err(MTError::ParseError {
                        line: 1,
                        column: 1,
                        message: format!("Unknown block number: {}", block_number),
                    });
                }
            }
        }

        if blocks.is_empty() {
            return Err(MTError::ParseError {
                line: 1,
                column: 1,
                message: "No blocks found in message".to_string(),
            });
        }

        Ok(blocks)
    }

    /// Parse basic header block (Block 1)
    fn parse_basic_header(&self, content: &str) -> Result<MessageBlock> {
        // Format: F01BANKDEFFAXXX0123456789
        if content.len() < 21 {
            return Err(MTError::InvalidMessageStructure {
                message: "Basic header block too short".to_string(),
            });
        }

        let application_id = content[0..1].to_string();
        let service_id = content[1..3].to_string();
        let logical_terminal = content[3..15].to_string();
        let session_number = content[15..19].to_string();
        let sequence_number = content[19..].to_string();

        Ok(MessageBlock::BasicHeader {
            application_id,
            service_id,
            logical_terminal,
            session_number,
            sequence_number,
        })
    }

    /// Parse application header block (Block 2)
    fn parse_application_header(&self, content: &str) -> Result<MessageBlock> {
        // Format: I103BANKDEFFAXXXU3003 or O1031535010605BANKDEFFAXXXU3003
        if content.is_empty() {
            return Err(MTError::InvalidMessageStructure {
                message: "Application header block is empty".to_string(),
            });
        }

        let input_output_identifier = content[0..1].to_string();

        if content.len() < 4 {
            return Err(MTError::InvalidMessageStructure {
                message: "Application header block too short".to_string(),
            });
        }

        let message_type = content[1..4].to_string();
        let remaining = &content[4..];

        // Parse the rest based on input/output identifier
        let (destination_address, priority, delivery_monitoring, obsolescence_period) =
            if input_output_identifier == "I" {
                // Input message format
                if remaining.len() >= 12 {
                    let dest = remaining[0..12].to_string();
                    let prio = remaining.get(12..13).unwrap_or("").to_string();
                    let del_mon = remaining.get(13..14).map(|s| s.to_string());
                    let obs_per = remaining.get(14..17).map(|s| s.to_string());
                    (dest, prio, del_mon, obs_per)
                } else {
                    (remaining.to_string(), String::new(), None, None)
                }
            } else {
                // Output message format
                (remaining.to_string(), String::new(), None, None)
            };

        Ok(MessageBlock::ApplicationHeader {
            input_output_identifier,
            message_type,
            destination_address,
            priority,
            delivery_monitoring,
            obsolescence_period,
        })
    }

    /// Parse user header block (Block 3)
    fn parse_user_header(&self, content: &str) -> Result<MessageBlock> {
        let mut fields = HashMap::new();

        // Parse user header fields (format: {tag:value})
        let user_field_regex = Regex::new(r"\{(\w+):([^}]*)\}")?;
        for captures in user_field_regex.captures_iter(content) {
            let tag = captures[1].to_string();
            let value = captures[2].to_string();
            fields.insert(tag, value);
        }

        Ok(MessageBlock::UserHeader { fields })
    }

    /// Parse text block (Block 4)
    fn parse_text_block(&self, content: &str) -> Result<MessageBlock> {
        let mut fields = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_tag = String::new();
        let mut current_value = String::new();

        for line in lines {
            let line = line.trim();
            if line.is_empty() || line == "-" {
                continue;
            }

            if line.starts_with(':') && line.contains(':') {
                // Save previous field if exists
                if !current_tag.is_empty() {
                    fields.push(Field::new(
                        current_tag.clone(),
                        current_value.trim().to_string(),
                    ));
                }

                // Parse new field
                if let Some(colon_pos) = line[1..].find(':') {
                    current_tag = line[1..colon_pos + 1].to_string();
                    current_value = line[colon_pos + 2..].to_string();
                } else {
                    return Err(MTError::ParseError {
                        line: 0,
                        column: 0,
                        message: format!("Invalid field format: {}", line),
                    });
                }
            } else {
                // Continuation of previous field
                if !current_value.is_empty() {
                    current_value.push('\n');
                }
                current_value.push_str(line);
            }
        }

        // Save last field
        if !current_tag.is_empty() {
            fields.push(Field::new(current_tag, current_value.trim().to_string()));
        }

        Ok(MessageBlock::TextBlock { fields })
    }

    /// Parse trailer block (Block 5)
    fn parse_trailer_block(&self, content: &str) -> Result<MessageBlock> {
        let mut fields = HashMap::new();

        // Parse trailer fields (format: {tag:value})
        let trailer_field_regex = Regex::new(r"\{(\w+):([^}]*)\}")?;
        for captures in trailer_field_regex.captures_iter(content) {
            let tag = captures[1].to_string();
            let value = captures[2].to_string();
            fields.insert(tag, value);
        }

        Ok(MessageBlock::TrailerBlock { fields })
    }

    /// Extract message type from blocks
    pub fn extract_message_type(&self, blocks: &[MessageBlock]) -> Result<String> {
        for block in blocks {
            if let MessageBlock::ApplicationHeader { message_type, .. } = block {
                return Ok(message_type.clone());
            }
        }

        Err(MTError::ParseError {
            line: 1,
            column: 1,
            message: "No application header block found".to_string(),
        })
    }

    /// Extract text block fields for message type detection
    fn extract_text_block(&self, blocks: &[MessageBlock]) -> Result<Vec<Field>> {
        for block in blocks {
            if let MessageBlock::TextBlock { fields } = block {
                return Ok(fields.clone());
            }
        }

        Err(MTError::ParseError {
            line: 1,
            column: 1,
            message: "No text block found".to_string(),
        })
    }
}

impl Default for MTParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default parser")
    }
}

/// Parse a SWIFT MT message from text
pub fn parse_message(input: &str) -> Result<MTMessage> {
    let parser = MTParser::new()?;
    parser.parse(input)
}

/// Extract fields from a text block
pub fn extract_fields(text_block: &str) -> Result<Vec<Field>> {
    let parser = MTParser::new()?;
    if let MessageBlock::TextBlock { fields } = parser.parse_text_block(text_block)? {
        Ok(fields)
    } else {
        Err(MTError::InvalidMessageStructure {
            message: "Failed to parse text block".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_header_parsing() {
        let parser = MTParser::new().unwrap();
        let result = parser
            .parse_basic_header("F01BANKDEFFAXXX0123456789")
            .unwrap();

        if let MessageBlock::BasicHeader {
            application_id,
            service_id,
            logical_terminal,
            session_number,
            sequence_number,
        } = result
        {
            assert_eq!(application_id, "F");
            assert_eq!(service_id, "01");
            assert_eq!(logical_terminal, "BANKDEFFAXXX");
            assert_eq!(session_number, "0123");
            assert_eq!(sequence_number, "456789");
        } else {
            panic!("Expected BasicHeader block");
        }
    }

    #[test]
    fn test_application_header_parsing() {
        let parser = MTParser::new().unwrap();
        let result = parser
            .parse_application_header("I103BANKDEFFAXXXU3003")
            .unwrap();

        if let MessageBlock::ApplicationHeader {
            input_output_identifier,
            message_type,
            destination_address,
            priority,
            ..
        } = result
        {
            assert_eq!(input_output_identifier, "I");
            assert_eq!(message_type, "103");
            assert_eq!(destination_address, "BANKDEFFAXXX");
            assert_eq!(priority, "U");
        } else {
            panic!("Expected ApplicationHeader block");
        }
    }

    #[test]
    fn test_text_block_parsing() {
        let parser = MTParser::new().unwrap();
        let text =
            ":20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n:50K:JOHN DOE\n:59:JANE SMITH";
        let result = parser.parse_text_block(text).unwrap();

        if let MessageBlock::TextBlock { fields } = result {
            assert_eq!(fields.len(), 5);
            assert_eq!(fields[0].tag.as_str(), "20");
            assert_eq!(fields[0].value(), "FT21234567890");
            assert_eq!(fields[1].tag.as_str(), "23B");
            assert_eq!(fields[1].value(), "CRED");
        } else {
            panic!("Expected TextBlock");
        }
    }
}
