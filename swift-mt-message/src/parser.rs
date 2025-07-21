//! # SWIFT MT Message Parser
//!
//! ## Purpose
//! Comprehensive parser for SWIFT MT (Message Type) messages that converts raw SWIFT message strings
//! into typed, structured data with full validation and field consumption tracking.
//!
//! ## Features
//! - **Multi-Format Support**: Handles all supported MT message types (101, 103, 104, etc.)
//! - **Block Structure Parsing**: Extracts and validates all 5 SWIFT message blocks
//! - **Field Consumption Tracking**: Sequential processing of duplicate fields with position tracking
//! - **Type-Safe Parsing**: Converts raw strings to strongly-typed field structures
//! - **Automatic Message Detection**: Auto-detects message type from application header
//! - **Comprehensive Validation**: Validates message structure, field formats, and business rules
//!
//! ## Architecture
//! The parser follows a layered approach:
//! 1. **Block Extraction**: Extracts blocks 1-5 from raw SWIFT message
//! 2. **Header Parsing**: Parses blocks 1, 2, 3, and 5 into header structures
//! 3. **Field Parsing**: Parses block 4 fields with position and variant tracking
//! 4. **Message Construction**: Builds typed message structures with sequential field consumption
//! 5. **Validation**: Applies format and business rule validation
//!
//! ## Usage Examples
//! ```rust
//! use swift_mt_message::parser::SwiftParser;
//! use swift_mt_message::messages::MT103;
//! use swift_mt_message::ParsedSwiftMessage;
//!
//! # fn main() -> swift_mt_message::Result<()> {
//! # let swift_message_string = "{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:\n:20:TXN123456\n:23B:CRED\n:32A:240315USD1000,00\n:50K:JOHN DOE\n123 MAIN ST\n:59:DE89370400440532013000\nBENEFICIARY NAME\n:71A:SHA\n-}";
//! // Parse specific message type
//! let mt103 = SwiftParser::parse::<MT103>(&swift_message_string)?;
//!
//! // Auto-detect message type
//! let parsed_message = SwiftParser::parse_auto(&swift_message_string)?;
//! match parsed_message {
//!     ParsedSwiftMessage::MT103(msg) => println!("Parsed MT103: {:?}", msg),
//!     ParsedSwiftMessage::MT202(msg) => println!("Parsed MT202: {:?}", msg),
//!     _ => println!("Other message type"),
//! }
//! # Ok(())
//! # }
//! ```

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use crate::errors::{ParseError, Result};
use crate::headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
use crate::messages::{
    MT101, MT103, MT104, MT107, MT110, MT111, MT112, MT192, MT196, MT199, MT202, MT205, MT210,
    MT292, MT296, MT299, MT900, MT910, MT920, MT935, MT940, MT941, MT942, MT950,
};
use crate::{ParsedSwiftMessage, SwiftMessage, SwiftMessageBody};

/// Type alias for the field parsing result with position tracking
///
/// Maps field tags to vectors of (field_value, position_in_message) tuples.
/// This enables sequential consumption of duplicate fields while maintaining message order.
type FieldParseResult = Result<HashMap<String, Vec<(String, usize)>>>;

/// Parsing context that flows through the parsing pipeline
#[derive(Debug, Clone)]
pub struct ParsingContext {
    /// Current field being parsed
    pub current_field: Option<String>,
    /// Current component being parsed
    pub current_component: Option<String>,
    /// Message type
    pub message_type: String,
    /// Original message for context
    pub original_message: String,
}

impl ParsingContext {
    /// Create a new parsing context
    pub fn new(message_type: String, original_message: String) -> Self {
        Self {
            current_field: None,
            current_component: None,
            message_type,
            original_message,
        }
    }

    /// Create a context with field information
    pub fn with_field(&self, field: String) -> Self {
        let mut ctx = self.clone();
        ctx.current_field = Some(field);
        ctx.current_component = None;
        ctx
    }

    /// Create a context with component information
    pub fn with_component(&self, component: String) -> Self {
        let mut ctx = self.clone();
        ctx.current_component = Some(component);
        ctx
    }
}

/// Field consumption tracker for sequential processing of duplicate fields
///
/// ## Purpose
/// Ensures that when a message contains multiple instances of the same field (e.g., multiple :50: fields),
/// they are consumed sequentially in the order they appear in the original message. This is critical
/// for messages like MT101 where sequence matters.
///
/// ## Implementation
/// - Tracks consumed field indices by tag
/// - Provides next available field value for sequential consumption
/// - Maintains message order integrity during field processing
///
/// ## Example
/// ```rust
/// use swift_mt_message::parser::FieldConsumptionTracker;
///
/// let mut tracker = FieldConsumptionTracker::new();
/// // Field "50" has values at positions [5, 15, 25] in message
/// let field_values = vec![
///     ("value1".to_string(), 5),
///     ("value2".to_string(), 15),
///     ("value3".to_string(), 25),
/// ];
/// let (value1, pos1) = tracker.get_next_available("50", &field_values).unwrap();
/// tracker.mark_consumed("50", pos1);
/// let (value2, pos2) = tracker.get_next_available("50", &field_values).unwrap();
/// // Ensures value2 is from position 15, not 5 or 25
/// ```
#[derive(Debug, Clone)]
pub struct FieldConsumptionTracker {
    /// Maps field tags to sets of consumed position indices
    consumed_indices: HashMap<String, HashSet<usize>>,
}

impl Default for FieldConsumptionTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldConsumptionTracker {
    /// Create a new consumption tracker
    pub fn new() -> Self {
        Self {
            consumed_indices: HashMap::new(),
        }
    }

    /// Mark a field value at a specific position as consumed
    pub fn mark_consumed(&mut self, tag: &str, index: usize) {
        // Avoid allocation when the key already exists
        use std::collections::hash_map::Entry;
        match self.consumed_indices.entry(tag.to_string()) {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(index);
            }
            Entry::Vacant(e) => {
                let mut set = HashSet::new();
                set.insert(index);
                e.insert(set);
            }
        }
    }

    /// Get the next available (unconsumed) field value for a tag
    pub fn get_next_available<'a>(
        &self,
        tag: &str,
        values: &'a [(String, usize)],
    ) -> Option<(&'a str, usize)> {
        let consumed_set = self.consumed_indices.get(tag);

        // Find first unconsumed value in original message order
        values
            .iter()
            .find(|(_, pos)| consumed_set.is_none_or(|set| !set.contains(pos)))
            .map(|(value, pos)| (value.as_str(), *pos))
    }
}

/// Find field values by base tag with sequential consumption tracking
///
/// ## Purpose
/// Locates the next available field value for a given base tag, handling both simple fields
/// and complex enum fields with variants (e.g., 50A, 50F, 50K).
///
/// ## Parameters
/// - `fields`: HashMap of all parsed fields with position tracking
/// - `base_tag`: Base field tag (e.g., "50", "59")
/// - `tracker`: Mutable reference to consumption tracker for sequential processing
///
/// ## Returns
/// `Option<(field_value, variant, position)>` where:
/// - `field_value`: The actual field content
/// - `variant`: Optional variant letter (A, F, K, etc.) for enum fields
/// - `position`: Original position in the message for ordering
///
/// ## Behavior
/// 1. First attempts exact match for simple fields (e.g., "50" -> "50")
/// 2. Then searches for variant fields (e.g., "50" -> "50A", "50F", "50K")
/// 3. Marks found field as consumed to ensure sequential processing
/// 4. Returns None if no unconsumed fields are available
pub fn find_field_with_variant_sequential(
    fields: &HashMap<String, Vec<(String, usize)>>,
    base_tag: &str,
    tracker: &mut FieldConsumptionTracker,
) -> Option<(String, Option<String>, usize)> {
    // First try to find exact match (for non-variant fields)
    if let Some(values) = fields.get(base_tag) {
        if let Some((value, pos)) = tracker.get_next_available(base_tag, values) {
            tracker.mark_consumed(base_tag, pos);
            return Some((value.to_string(), None, pos));
        }
    }

    // For enum fields, look for variant tags (50A, 50F, 50K, etc.)
    for (tag, values) in fields {
        if tag.starts_with(base_tag) && tag.len() == base_tag.len() + 1 {
            let variant_char = tag.chars().last().unwrap();
            // Check if it's a valid variant letter (A-Z)
            if variant_char.is_ascii_alphabetic() && variant_char.is_ascii_uppercase() {
                if let Some((value, pos)) = tracker.get_next_available(tag, values) {
                    tracker.mark_consumed(tag, pos);
                    return Some((value.to_string(), Some(variant_char.to_string()), pos));
                }
            }
        }
    }

    None
}

/// Main parser for SWIFT MT messages
///
/// ## Purpose
/// Primary parsing engine that converts raw SWIFT message strings into typed, validated message structures.
/// Handles all aspects of SWIFT message processing including block extraction, header parsing, field parsing,
/// and type-safe message construction.
///
/// ## Capabilities
/// - **Multi-Message Support**: Parses all 24 supported MT message types
/// - **Flexible Parsing**: Both type-specific and auto-detection parsing modes
/// - **Robust Error Handling**: Comprehensive error reporting for malformed messages
/// - **Field Validation**: Format validation and business rule checking
/// - **Position Tracking**: Maintains field order for sequential processing requirements
///
/// ## Parsing Process
/// 1. **Block Extraction**: Identifies and extracts SWIFT blocks 1-5
/// 2. **Header Validation**: Parses and validates basic, application, user headers and trailer
/// 3. **Message Type Detection**: Determines message type from application header
/// 4. **Field Processing**: Parses block 4 fields with position and variant tracking
/// 5. **Type Construction**: Builds strongly-typed message structures
/// 6. **Validation**: Applies format and business rule validation
///
/// ## Thread Safety
/// SwiftParser is stateless and thread-safe. All methods are static and can be called
/// concurrently from multiple threads.
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

        // Validate message type matches expected type using SWIFT error codes
        if message_type != T::message_type() {
            crate::validation::validate_message_type(&message_type, T::message_type())?;
        }

        // Parse block 4 fields with position tracking
        let field_map_with_positions = Self::parse_block4_fields(&block4.unwrap_or_default())?;

        // Use sequential parsing for all message types
        let fields = T::from_fields(field_map_with_positions)?;

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
                // Parse MT101 with custom sequence handling
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

                // Parse block 4 fields with position tracking
                let field_map_with_positions =
                    Self::parse_block4_fields(&block4.unwrap_or_default())?;

                // Use custom parsing for MT101
                let fields = MT101::parse_with_sequences(field_map_with_positions)?;

                let parsed = SwiftMessage {
                    basic_header,
                    application_header,
                    user_header,
                    trailer,
                    message_type: "101".to_string(),
                    fields,
                };

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

    /// Extract a specific message block from raw SWIFT message with SWIFT validation
    pub fn extract_block(raw_message: &str, block_index: u8) -> Result<Option<String>> {
        // Validate block index using SWIFT error codes
        if !(1..=5).contains(&block_index) {
            return Err(ParseError::SwiftValidation(Box::new(
                crate::errors::SwiftValidationError::format_error(
                    crate::swift_error_codes::t_series::T01,
                    "BLOCK_INDEX",
                    &block_index.to_string(),
                    "1-5",
                    &format!("Invalid block index: {block_index}"),
                ),
            )));
        }

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
                _ => Err(ParseError::SwiftValidation(Box::new(
                    crate::errors::SwiftValidationError::format_error(
                        crate::swift_error_codes::t_series::T02,
                        "BLOCK",
                        &block_index.to_string(),
                        "1-5",
                        &format!("Invalid block index: {block_index}"),
                    ),
                ))),
            }
        } else {
            Ok(None)
        }
    }

    /// Parse block 4 fields into a field map with enhanced position tracking
    fn parse_block4_fields(block4: &str) -> FieldParseResult {
        // Pre-allocate HashMap with estimated capacity based on typical field count
        // Most messages have between 10-60 fields
        let estimated_fields = block4.matches("\n:").count().max(10);
        let mut field_map: HashMap<String, Vec<(String, usize)>> =
            HashMap::with_capacity(estimated_fields);

        // Remove leading/trailing whitespace and newlines
        let content = block4.trim();

        // Split by field markers (:XX:)
        let mut current_pos = 0;
        let mut field_position = 0; // Track sequential position for consumption ordering
        let mut line_number = 1;

        while current_pos < content.len() {
            // Track line numbers for better error reporting
            if current_pos > 0 && content.chars().nth(current_pos - 1) == Some('\n') {
                line_number += 1;
            }

            // Find next field marker
            if let Some(field_start) = content[current_pos..].find(':') {
                let field_start = current_pos + field_start;

                // Extract field tag (characters after : until next :)
                if let Some(tag_end) = content[field_start + 1..].find(':') {
                    let tag_end = field_start + 1 + tag_end;
                    let raw_field_tag = &content[field_start + 1..tag_end];

                    // Normalize field tag by removing option letters (A, F, K, etc.)
                    let field_tag = Self::normalize_field_tag(raw_field_tag);

                    // Find the end of field value (next field marker or end of content)
                    let value_start = tag_end + 1;
                    let value_end = if let Some(next_field) = content[value_start..].find("\n:") {
                        value_start + next_field
                    } else {
                        content.len()
                    };

                    // Avoid unnecessary string allocation - trim inline during push
                    let field_value_slice = &content[value_start..value_end];
                    let trimmed_value = field_value_slice.trim();

                    // Store field value with enhanced position info (line number encoded with field position)
                    // High 16 bits: line number, Low 16 bits: field position
                    let position_info = (line_number << 16) | (field_position & 0xFFFF);

                    // Add to existing Vec or create new Vec for this field tag
                    field_map
                        .entry(field_tag.into_owned())
                        .or_default()
                        .push((trimmed_value.to_string(), position_info));

                    field_position += 1; // Increment position for next field
                    current_pos = value_end;
                } else {
                    // Last field or malformed - provide detailed error
                    return Err(ParseError::InvalidBlockStructure {
                        block: "4".to_string(),
                        message: format!(
                            "Malformed field tag at line {line_number}, position {current_pos}"
                        ),
                    });
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
    fn normalize_field_tag(raw_tag: &str) -> Cow<'_, str> {
        // Find where the numeric part ends
        let numeric_end = raw_tag
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(raw_tag.len());

        // If no suffix, return the tag as-is
        if numeric_end == raw_tag.len() {
            return Cow::Borrowed(raw_tag);
        }

        let numeric_part = &raw_tag[..numeric_end];
        let suffix = &raw_tag[numeric_end..];

        // For certain field numbers, preserve the option letter to avoid conflicts
        match numeric_part {
            "11" | "13" | "23" | "26" | "28" | "32" | "33" | "50" | "52" | "53" | "54" | "55"
            | "56" | "57" | "58" | "59" | "71" | "77" => {
                // Keep option letters for fields that have multiple variants or specific formats
                // 11A (MT and Date - Option A), 11S (MT and Date - Option S)
                // 13C (Time Indication)
                // 23B (Bank Operation Code) vs 23E (Instruction Code)
                // 26T (Transaction Type Code)
                // 32A (Value Date/Currency/Amount)
                // 33B (Currency/Instructed Amount)
                // 50A/F/K (Ordering Customer)
                // 59A/F (Beneficiary Customer)
                // 52A (Ordering Institution)
                // 53A (Sender's Correspondent)
                // 54A (Receiver's Correspondent)
                // 55A (Third Reimbursement Institution)
                // 56A (Intermediary Institution)
                // 57A (Account With Institution)
                // 71A (Details of Charges) vs 71F (Sender's Charges) vs 71G (Receiver's Charges)
                // 77B (Regulatory Reporting)
                Cow::Borrowed(raw_tag)
            }
            _ => {
                // For other fields, check if suffix is just uppercase letters
                if suffix.chars().all(|c| c.is_ascii_uppercase()) {
                    // It's an option letter, return just the numeric part
                    Cow::Owned(numeric_part.to_string())
                } else {
                    // Not a simple option letter, keep the full tag
                    Cow::Borrowed(raw_tag)
                }
            }
        }
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

        // Test invalid block index (should return SWIFT validation error)
        let result_none = SwiftParser::extract_block(raw_message, 6);
        assert!(result_none.is_err());

        // Test valid block that doesn't exist (should return Ok(None))
        let result_block3 = SwiftParser::extract_block(raw_message, 3);
        assert!(result_block3.is_ok());
        assert!(result_block3.unwrap().is_none());
    }

    #[test]
    fn test_parse_block4_fields() {
        let block4 = "\n:20:FT21234567890\n:23B:CRED\n:32A:210315EUR1234567,89\n";
        let field_map = SwiftParser::parse_block4_fields(block4).unwrap();

        // Debug: print the actual values
        println!("Field 20: {:?}", field_map.get("20"));
        println!("Field 23B: {:?}", field_map.get("23B"));
        println!("Field 32A: {:?}", field_map.get("32A"));

        // Extract line and position info for debugging
        if let Some(values) = field_map.get("20") {
            let (_, pos) = &values[0];
            let line = pos >> 16;
            let field_pos = pos & 0xFFFF;
            println!("Field 20 - Line: {line}, Field position: {field_pos}");
        }

        // The position info encodes line number (high 16 bits) and field position (low 16 bits)
        // But we need to check what the actual values are
        assert!(field_map.contains_key("20"));
        assert!(field_map.contains_key("23B"));
        assert!(field_map.contains_key("32A"));

        // Check field values only, not positions for now
        assert_eq!(field_map.get("20").unwrap()[0].0, "FT21234567890");
        assert_eq!(field_map.get("23B").unwrap()[0].0, "CRED");
        assert_eq!(field_map.get("32A").unwrap()[0].0, "210315EUR1234567,89");
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
        assert!(field_map.contains_key("50K")); // Option letters are preserved
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
:52A:BANKDEFFXXX
:57A:DEUTDEFFXXX
:59:/DE89370400440532013000
DEUTDEFFXXX
:70:PAYMENT FOR SERVICES
:71A:OUR
-}"#;

        match SwiftParser::parse_auto(raw_mt103) {
            Ok(parsed) => {
                // Check that it detected the correct message type
                assert_eq!(parsed.message_type(), "103");

                // Check that we can extract the MT103 message
                let mt103_msg = parsed.as_mt103().unwrap();
                assert_eq!(mt103_msg.message_type, "103");

                println!("Successfully parsed MT103 message with auto-detection");
            }
            Err(e) => {
                println!("Error parsing MT103: {e:?}");

                // Let's debug by parsing the fields manually
                let block4 = SwiftParser::extract_block(raw_mt103, 4).unwrap().unwrap();
                let field_map = SwiftParser::parse_block4_fields(&block4).unwrap();

                println!("Parsed fields:");
                for (tag, values) in &field_map {
                    println!("  {tag}: {values:?}");
                }

                panic!("Failed to parse MT103");
            }
        }
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
