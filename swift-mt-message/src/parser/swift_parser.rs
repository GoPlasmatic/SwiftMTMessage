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

use std::collections::{HashMap, HashSet};

use crate::errors::{ParseError, Result, SwiftValidationError};
use crate::headers::{ApplicationHeader, BasicHeader, Trailer, UserHeader};
use crate::messages::{
    MT101, MT103, MT104, MT107, MT110, MT111, MT112, MT190, MT191, MT192, MT196, MT199, MT200,
    MT202, MT204, MT205, MT210, MT290, MT291, MT292, MT296, MT299, MT900, MT910, MT920, MT935,
    MT940, MT941, MT942, MT950,
};
use crate::swift_error_codes::t_series;
use crate::{ParsedSwiftMessage, SwiftMessage, SwiftMessageBody};

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

/// Field routing strategy for numbered field tags
#[derive(Debug, Clone)]
enum FieldRoutingStrategy {
    /// For Field50InstructingParty - typically uses C, L variants
    InstructingParty,
    /// For Field50Creditor - typically uses A, F, K variants
    CreditorParty,
}

impl FieldRoutingStrategy {
    /// Get preferred variants for this routing strategy
    fn get_preferred_variants(&self, base_tag: &str) -> Option<Vec<&'static str>> {
        match (self, base_tag) {
            (FieldRoutingStrategy::InstructingParty, "50") => Some(vec!["C", "L"]),
            (FieldRoutingStrategy::CreditorParty, "50") => Some(vec!["A", "F", "K"]),
            _ => None,
        }
    }
}

/// Apply intelligent routing strategy for Field 50 numbered fields
///
/// This function implements a smart routing algorithm for Field 50 variants when dealing
/// with numbered field tags (50#1, 50#2). The strategy is:
///
/// 1. First call (no Field 50 consumed yet) -> Prefer C, L variants (InstructingParty)
/// 2. Second call (some Field 50 already consumed) -> Prefer A, F, K variants (Creditor)
///
/// This matches the typical SWIFT message pattern where:
/// - field_50_instructing (50#1) typically uses 50C or 50L
/// - field_50_creditor (50#2) typically uses 50A, 50F, or 50K
fn apply_field50_routing_strategy<'a>(
    mut candidates: Vec<(&'a String, &'a Vec<(String, usize)>)>,
    tracker: &FieldConsumptionTracker,
    base_tag: &str,
) -> Vec<(&'a String, &'a Vec<(String, usize)>)> {
    // Check how many Field 50 variants have been consumed already
    let consumed_field50_count = candidates
        .iter()
        .filter(|(tag, _)| {
            tracker
                .consumed_indices
                .get(*tag)
                .is_some_and(|set| !set.is_empty())
        })
        .count();

    #[cfg(debug_assertions)]
    eprintln!(
        "DEBUG: Field50 routing strategy - consumed_count={}, candidates={:?}",
        consumed_field50_count,
        candidates
            .iter()
            .map(|(tag, _)| tag.as_str())
            .collect::<Vec<_>>()
    );

    // Determine routing strategy based on consumption history
    let strategy = if consumed_field50_count == 0 {
        // First Field 50 access -> prefer InstructingParty variants (C, L)
        FieldRoutingStrategy::InstructingParty
    } else {
        // Subsequent Field 50 access -> prefer Creditor variants (A, F, K)
        FieldRoutingStrategy::CreditorParty
    };

    if let Some(preferred_variants) = strategy.get_preferred_variants(base_tag) {
        // Reorder candidates to prioritize preferred variants
        candidates.sort_by_key(|(tag, _)| {
            let variant_char = tag.chars().last().unwrap_or(' ');
            let variant_str = variant_char.to_string();

            // Check if this variant is in the preferred list
            let is_preferred = preferred_variants.contains(&variant_str.as_str());

            if is_preferred {
                0 // High priority
            } else {
                1 // Lower priority
            }
        });

        #[cfg(debug_assertions)]
        eprintln!(
            "DEBUG: Field50 routing - strategy={:?}, preferred_variants={:?}, reordered_candidates={:?}",
            strategy,
            preferred_variants,
            candidates
                .iter()
                .map(|(tag, _)| tag.as_str())
                .collect::<Vec<_>>()
        );
    }

    candidates
}

/// Find field values for numbered tags with intelligent routing
///
/// ## Purpose
/// Special handling for numbered field tags (e.g., "50#1", "50#2") that require
/// intelligent routing based on the specific numbered tag and its variant constraints.
///
/// ## Parameters
/// - `fields`: HashMap of all parsed fields with position tracking
/// - `base_tag`: Base field tag (e.g., "50", "59")
/// - `tracker`: Mutable reference to consumption tracker for sequential processing
/// - `valid_variants`: Optional list of valid variant letters specific to this numbered field
/// - `numbered_tag`: The full numbered tag (e.g., "50#1", "50#2") for routing context
///
/// ## Returns
/// `Option<(field_value, variant, position)>`
pub fn find_field_with_variant_sequential_numbered(
    fields: &HashMap<String, Vec<(String, usize)>>,
    base_tag: &str,
    tracker: &mut FieldConsumptionTracker,
    valid_variants: Option<Vec<&str>>,
    _numbered_tag: &str,
) -> Option<(String, Option<String>, usize)> {
    #[cfg(debug_assertions)]
    eprintln!(
        "DEBUG: find_field_with_variant_sequential_numbered for tag={}, base={}, variants={:?}",
        _numbered_tag, base_tag, valid_variants
    );

    // Use the variant constraints for this specific numbered field
    // This ensures that 50#1 uses its specific variants (e.g., C, L)
    // and 50#2 uses its specific variants (e.g., A, F, K)
    find_field_with_variant_sequential_constrained(
        fields,
        base_tag,
        tracker,
        valid_variants.as_deref(),
    )
}

/// Find field values by base tag with sequential consumption tracking and variant constraints
///
/// ## Purpose
/// Enhanced version of find_field_with_variant_sequential that accepts a list of valid variants
/// to constrain the search. This is crucial for numbered field tags like "50#1" and "50#2"
/// where different positions accept different variants.
///
/// ## Parameters
/// - `fields`: HashMap of all parsed fields with position tracking
/// - `base_tag`: Base field tag (e.g., "50", "59")
/// - `tracker`: Mutable reference to consumption tracker for sequential processing
/// - `valid_variants`: Optional list of valid variant letters (e.g., ["C", "L"] for Field50InstructingParty)
///
/// ## Returns
/// `Option<(field_value, variant, position)>` where:
/// - `field_value`: The actual field content
/// - `variant`: Optional variant letter (A, F, K, etc.) for enum fields
/// - `position`: Original position in the message for ordering
pub fn find_field_with_variant_sequential_constrained(
    fields: &HashMap<String, Vec<(String, usize)>>,
    base_tag: &str,
    tracker: &mut FieldConsumptionTracker,
    valid_variants: Option<&[&str]>,
) -> Option<(String, Option<String>, usize)> {
    #[cfg(debug_assertions)]
    {
        eprintln!(
            "DEBUG: find_field_with_variant called for base_tag={}, valid_variants={:?}",
            base_tag, valid_variants
        );
        eprintln!(
            "  Available fields: {:?}",
            fields.keys().collect::<Vec<_>>()
        );

        // Special debug for numbered field tags (50#1, 50#2, etc.)
        if base_tag == "50" {
            eprintln!(
                "DEBUG: Field50 routing - base_tag={}, constraints={:?}",
                base_tag, valid_variants
            );
            eprintln!(
                "DEBUG: Available Field50 variants: {:?}",
                fields
                    .keys()
                    .filter(|k| k.starts_with("50") && k.len() == 3)
                    .collect::<Vec<_>>()
            );
        }
    }

    // First try to find exact match (for non-variant fields)
    if let Some(values) = fields.get(base_tag)
        && let Some((value, pos)) = tracker.get_next_available(base_tag, values)
    {
        #[cfg(debug_assertions)]
        {
            eprintln!(
                "DEBUG: Found exact match for base_tag={}, marking as consumed at pos={}",
                base_tag, pos
            );
            if base_tag == "90D" || base_tag == "86" {
                eprintln!(
                    "DEBUG: Returning value for {}: '{}'",
                    base_tag,
                    if value.len() > 50 {
                        &value[..50]
                    } else {
                        value
                    }
                );
            }
        }
        tracker.mark_consumed(base_tag, pos);
        return Some((value.to_string(), None, pos));
    }

    // For enum fields, look for variant tags (50A, 50F, 50K, etc.)
    // Sort tags by position to ensure we process them in order
    let mut variant_candidates: Vec<(&String, &Vec<(String, usize)>)> = fields
        .iter()
        .filter(|(tag, _)| {
            tag.starts_with(base_tag)
                && tag.len() == base_tag.len() + 1
                && tag
                    .chars()
                    .last()
                    .is_some_and(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
        })
        .collect();

    // Implement intelligent routing for numbered field tags
    if base_tag == "50" && valid_variants.is_some() {
        // Apply numbered field routing strategy for Field 50
        variant_candidates = apply_field50_routing_strategy(variant_candidates, tracker, base_tag);
    }

    // Sort by the minimum unconsumed position in each tag's values
    variant_candidates.sort_by_key(|(tag, values)| {
        values
            .iter()
            .filter(|(_, pos)| {
                tracker
                    .consumed_indices
                    .get(*tag)
                    .is_none_or(|set| !set.contains(pos))
            })
            .map(|(_, pos)| *pos)
            .min()
            .unwrap_or(usize::MAX)
    });

    for (tag, values) in variant_candidates {
        let variant_char = tag.chars().last().unwrap();
        let variant_str = variant_char.to_string();

        // Check if this variant is allowed (if constraints are provided)
        if let Some(valid) = valid_variants
            && !valid.contains(&variant_str.as_str())
        {
            #[cfg(debug_assertions)]
            eprintln!(
                "DEBUG: Skipping field {} with variant {} - not in valid variants {:?}",
                tag, variant_str, valid
            );
            continue; // Skip variants that aren't in the valid list
        }

        if let Some((value, pos)) = tracker.get_next_available(tag, values) {
            #[cfg(debug_assertions)]
            eprintln!(
                "DEBUG: Found field {} with variant {} for base tag {}",
                tag, variant_str, base_tag
            );
            tracker.mark_consumed(tag, pos);
            return Some((value.to_string(), Some(variant_str), pos));
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
pub struct SwiftParser {}

impl Default for SwiftParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SwiftParser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a message and return ParseResult with all errors collected
    pub fn parse_with_errors<T: SwiftMessageBody>(
        &self,
        raw_message: &str,
    ) -> Result<crate::errors::ParseResult<SwiftMessage<T>>> {
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
        let message_type = application_header.message_type().to_string();

        // Validate message type matches expected type using SWIFT error codes
        if message_type != T::message_type() {
            return Err(ParseError::SwiftValidation(Box::new(
                SwiftValidationError::format_error(
                    t_series::T03,
                    "MESSAGE_TYPE",
                    &message_type,
                    T::message_type(),
                    &format!(
                        "Message type mismatch: expected {}, got {}",
                        T::message_type(),
                        message_type
                    ),
                ),
            )));
        }

        // Parse block 4 using MessageParser-based approach
        let fields = T::parse_from_block4(&block4.unwrap_or_default())?;

        Ok(crate::errors::ParseResult::Success(SwiftMessage {
            basic_header,
            application_header,
            user_header,
            trailer,
            message_type,
            fields,
        }))
    }
    /// Parse a raw SWIFT message string into a typed message (static method for backward compatibility)
    pub fn parse<T: SwiftMessageBody>(raw_message: &str) -> Result<SwiftMessage<T>> {
        Self::new().parse_message(raw_message)
    }

    /// Parse a raw SWIFT message string into a typed message with configuration support
    pub fn parse_message<T: SwiftMessageBody>(&self, raw_message: &str) -> Result<SwiftMessage<T>> {
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
        let message_type = application_header.message_type().to_string();

        // Validate message type matches expected type using SWIFT error codes
        if message_type != T::message_type() {
            return Err(ParseError::SwiftValidation(Box::new(
                SwiftValidationError::format_error(
                    t_series::T03,
                    "MESSAGE_TYPE",
                    &message_type,
                    T::message_type(),
                    &format!(
                        "Message type mismatch: expected {}, got {}",
                        T::message_type(),
                        message_type
                    ),
                ),
            )));
        }

        // Parse block 4 using MessageParser-based approach
        let fields = T::parse_from_block4(&block4.unwrap_or_default())?;

        Ok(SwiftMessage {
            basic_header,
            application_header,
            user_header,
            trailer,
            message_type,
            fields,
        })
    }

    /// Parse a raw SWIFT message string with automatic message type detection (static method for backward compatibility)
    pub fn parse_auto(raw_message: &str) -> Result<ParsedSwiftMessage> {
        Self::new().parse_message_auto(raw_message)
    }

    /// Parse a raw SWIFT message string with automatic message type detection and configuration support
    pub fn parse_message_auto(&self, raw_message: &str) -> Result<ParsedSwiftMessage> {
        // First, extract blocks to get the message type
        let block2 = Self::extract_block(raw_message, 2)?;

        // Parse application header to get message type
        let application_header = ApplicationHeader::parse(&block2.unwrap_or_default())?;
        let message_type = application_header.message_type();

        // Route to appropriate parser based on message type
        match message_type {
            "101" => {
                let parsed = self.parse_message::<MT101>(raw_message)?;
                Ok(ParsedSwiftMessage::MT101(Box::new(parsed)))
            }
            "103" => {
                let parsed = self.parse_message::<MT103>(raw_message)?;
                Ok(ParsedSwiftMessage::MT103(Box::new(parsed)))
            }
            "104" => {
                let parsed = self.parse_message::<MT104>(raw_message)?;
                Ok(ParsedSwiftMessage::MT104(Box::new(parsed)))
            }
            "107" => {
                let parsed = self.parse_message::<MT107>(raw_message)?;
                Ok(ParsedSwiftMessage::MT107(Box::new(parsed)))
            }
            "110" => {
                let parsed = self.parse_message::<MT110>(raw_message)?;
                Ok(ParsedSwiftMessage::MT110(Box::new(parsed)))
            }
            "111" => {
                let parsed = self.parse_message::<MT111>(raw_message)?;
                Ok(ParsedSwiftMessage::MT111(Box::new(parsed)))
            }
            "112" => {
                let parsed = self.parse_message::<MT112>(raw_message)?;
                Ok(ParsedSwiftMessage::MT112(Box::new(parsed)))
            }
            "190" => {
                let parsed = self.parse_message::<MT190>(raw_message)?;
                Ok(ParsedSwiftMessage::MT190(Box::new(parsed)))
            }
            "191" => {
                let parsed = self.parse_message::<MT191>(raw_message)?;
                Ok(ParsedSwiftMessage::MT191(Box::new(parsed)))
            }
            "200" => {
                let parsed = self.parse_message::<MT200>(raw_message)?;
                Ok(ParsedSwiftMessage::MT200(Box::new(parsed)))
            }
            "202" => {
                let parsed = self.parse_message::<MT202>(raw_message)?;
                Ok(ParsedSwiftMessage::MT202(Box::new(parsed)))
            }
            "204" => {
                let parsed = self.parse_message::<MT204>(raw_message)?;
                Ok(ParsedSwiftMessage::MT204(Box::new(parsed)))
            }
            "205" => {
                let parsed = self.parse_message::<MT205>(raw_message)?;
                Ok(ParsedSwiftMessage::MT205(Box::new(parsed)))
            }
            "210" => {
                let parsed = self.parse_message::<MT210>(raw_message)?;
                Ok(ParsedSwiftMessage::MT210(Box::new(parsed)))
            }
            "290" => {
                let parsed = self.parse_message::<MT290>(raw_message)?;
                Ok(ParsedSwiftMessage::MT290(Box::new(parsed)))
            }
            "291" => {
                let parsed = self.parse_message::<MT291>(raw_message)?;
                Ok(ParsedSwiftMessage::MT291(Box::new(parsed)))
            }
            "900" => {
                let parsed = self.parse_message::<MT900>(raw_message)?;
                Ok(ParsedSwiftMessage::MT900(Box::new(parsed)))
            }
            "910" => {
                let parsed = self.parse_message::<MT910>(raw_message)?;
                Ok(ParsedSwiftMessage::MT910(Box::new(parsed)))
            }
            "920" => {
                let parsed = self.parse_message::<MT920>(raw_message)?;
                Ok(ParsedSwiftMessage::MT920(Box::new(parsed)))
            }
            "935" => {
                let parsed = self.parse_message::<MT935>(raw_message)?;
                Ok(ParsedSwiftMessage::MT935(Box::new(parsed)))
            }
            "940" => {
                let parsed = self.parse_message::<MT940>(raw_message)?;
                Ok(ParsedSwiftMessage::MT940(Box::new(parsed)))
            }
            "941" => {
                let parsed = self.parse_message::<MT941>(raw_message)?;
                Ok(ParsedSwiftMessage::MT941(Box::new(parsed)))
            }
            "942" => {
                let parsed = self.parse_message::<MT942>(raw_message)?;
                Ok(ParsedSwiftMessage::MT942(Box::new(parsed)))
            }
            "950" => {
                let parsed = self.parse_message::<MT950>(raw_message)?;
                Ok(ParsedSwiftMessage::MT950(Box::new(parsed)))
            }
            "192" => {
                let parsed = self.parse_message::<MT192>(raw_message)?;
                Ok(ParsedSwiftMessage::MT192(Box::new(parsed)))
            }
            "196" => {
                let parsed = self.parse_message::<MT196>(raw_message)?;
                Ok(ParsedSwiftMessage::MT196(Box::new(parsed)))
            }
            "292" => {
                let parsed = self.parse_message::<MT292>(raw_message)?;
                Ok(ParsedSwiftMessage::MT292(Box::new(parsed)))
            }
            "296" => {
                let parsed = self.parse_message::<MT296>(raw_message)?;
                Ok(ParsedSwiftMessage::MT296(Box::new(parsed)))
            }
            "199" => {
                let parsed = self.parse_message::<MT199>(raw_message)?;
                Ok(ParsedSwiftMessage::MT199(Box::new(parsed)))
            }
            "299" => {
                let parsed = self.parse_message::<MT299>(raw_message)?;
                Ok(ParsedSwiftMessage::MT299(Box::new(parsed)))
            }
            _ => Err(ParseError::UnsupportedMessageType {
                message_type: message_type.to_string(),
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
                1 | 2 => {
                    // Blocks 1 and 2 end with simple closing brace (no nested content)
                    if let Some(end) = raw_message[start..].find('}') {
                        let end = start + end;
                        Ok(Some(raw_message[content_start..end].to_string()))
                    } else {
                        Ok(None)
                    }
                }
                3 | 5 => {
                    // Blocks 3 and 5 may have nested braces (e.g., {103:EBA} or {CHK:...})
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

/// Reconstruct a Block 4 string from a field map
///
/// Converts a HashMap of fields (with position tracking) into a Block 4 string
/// that can be parsed using parse_from_block4()
fn reconstruct_block4_from_fields(fields: &HashMap<String, Vec<(String, usize)>>) -> String {
    // Flatten and sort all fields by position
    let mut all_fields: Vec<(&str, &str, usize)> = Vec::new();
    for (tag, values) in fields {
        for (value, pos) in values {
            all_fields.push((tag, value, *pos));
        }
    }
    all_fields.sort_by_key(|(_, _, pos)| *pos);

    // Build the Block 4 string
    let mut result = String::new();
    for (tag, value, _) in all_fields {
        // Remove the tag prefix if it exists in the value
        let clean_value = if value.starts_with(&format!(":{tag}:")) {
            &value[tag.len() + 2..]
        } else if let Some(stripped) = value.strip_prefix(':') {
            // Handle variant fields like :50K:, :53A:, etc.
            if let Some(second_colon) = stripped.find(':') {
                &value[second_colon + 2..]
            } else {
                value
            }
        } else {
            value
        };

        result.push_str(&format!(":{tag}:{clean_value}\n"));
    }

    result
}

/// Parse sequence fields (e.g., transactions in MT101, MT104)
///
/// This function identifies sequence boundaries and parses each sequence into the target type.
/// Sequences typically start with a mandatory field (like Field 21) that marks the beginning
/// of each repetition.
pub fn parse_sequences<T>(
    fields: &HashMap<String, Vec<(String, usize)>>,
    tracker: &mut FieldConsumptionTracker,
) -> Result<Vec<T>>
where
    T: crate::SwiftMessageBody,
{
    // Use the enhanced sequence parser for messages that have complex sequences
    let message_type = std::any::type_name::<T>();

    if message_type.contains("MT104Transaction") {
        // For MT104, we need to handle the three-sequence structure
        use crate::parser::sequence_parser::{get_sequence_config, split_into_sequences};

        let config = get_sequence_config("MT104");
        let parsed_sequences = split_into_sequences(fields, &config)?;

        // Parse only sequence B (transactions)
        return parse_sequence_b_items::<T>(&parsed_sequences.sequence_b, tracker);
    }

    if message_type.contains("MT110Cheque") {
        // MT110 has repetitive cheque sequences starting with field 21
        use crate::parser::sequence_parser::{get_sequence_config, split_into_sequences};

        let config = get_sequence_config("MT110");
        let parsed_sequences = split_into_sequences(fields, &config)?;

        // Parse only sequence B (cheques)
        return parse_sequence_b_items::<T>(&parsed_sequences.sequence_b, tracker);
    }

    if message_type.contains("MT204Transaction") {
        // MT204 has a special structure where fields are grouped by type, not by transaction
        // We need to reconstruct transactions from the grouped fields

        // Count how many transactions we have (based on field 20 occurrences, excluding the first one)
        let field_20_count = fields.get("20").map(|v| v.len()).unwrap_or(0);
        if field_20_count <= 1 {
            return Ok(Vec::new()); // No transactions if only one or zero field 20s
        }

        let num_transactions = field_20_count - 1; // First field 20 is for sequence A

        // Build transactions by distributing fields
        let mut transactions = Vec::new();

        for i in 0..num_transactions {
            let mut tx_fields = HashMap::new();

            // Get field 20 (skip the first one which is for sequence A)
            if let Some(field_20_values) = fields.get("20")
                && i + 1 < field_20_values.len()
            {
                tx_fields.insert("20".to_string(), vec![field_20_values[i + 1].clone()]);
            }

            // Get field 21 if present (optional)
            if let Some(field_21_values) = fields.get("21")
                && i < field_21_values.len()
            {
                tx_fields.insert("21".to_string(), vec![field_21_values[i].clone()]);
            }

            // Get field 32B
            if let Some(field_32b_values) = fields.get("32B")
                && i < field_32b_values.len()
            {
                tx_fields.insert("32B".to_string(), vec![field_32b_values[i].clone()]);
            }

            // Get field 53 (various variants)
            for variant in ["53", "53A", "53B", "53D"] {
                if let Some(field_53_values) = fields.get(variant)
                    && i < field_53_values.len()
                {
                    tx_fields.insert(variant.to_string(), vec![field_53_values[i].clone()]);
                    break; // Only one variant per transaction
                }
            }

            // Get field 72 if present (optional)
            // MT204 structure: First Field 72 is for Sequence A, then one Field 72 per transaction in Sequence B
            if let Some(field_72_values) = fields.get("72") {
                // Skip the first Field 72 (belongs to Sequence A), then take one per transaction
                if i + 1 < field_72_values.len() {
                    tx_fields.insert("72".to_string(), vec![field_72_values[i + 1].clone()]);
                }
            }

            // Reconstruct block4 string from fields and parse using parse_from_block4
            let block4_str = reconstruct_block4_from_fields(&tx_fields);
            if let Ok(transaction) = T::parse_from_block4(&block4_str) {
                transactions.push(transaction);
            }
        }

        return Ok(transactions);
    }

    // Get all fields sorted by position
    let mut all_fields: Vec<(String, String, usize)> = Vec::new();
    for (tag, values) in fields {
        for (value, pos) in values {
            // Only include unconsumed fields
            if tracker
                .consumed_indices
                .get(tag)
                .is_none_or(|set| !set.contains(pos))
            {
                #[cfg(debug_assertions)]
                if tag.starts_with("50") {
                    eprintln!(
                        "DEBUG: Collecting field for sequence: tag='{}' from fields HashMap",
                        tag
                    );
                }
                all_fields.push((tag.clone(), value.clone(), *pos));
            }
        }
    }
    all_fields.sort_by_key(|(_, _, pos)| *pos);

    // Determine the sequence start marker based on message type
    let (primary_marker, secondary_marker) = if message_type.contains("MT920Sequence") {
        ("12", None)
    } else if message_type.contains("MT935RateChange") {
        ("23", Some("25"))
    } else if message_type.contains("MT940StatementLine")
        || message_type.contains("MT942StatementLine")
    {
        ("61", None)
    } else {
        ("21", None)
    };

    let mut sequences = Vec::new();
    let mut current_sequence_fields: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut in_sequence = false;

    // For MT942, fields that belong in the repeating section are only 61 and 86
    // Fields 90D, 90C, and other 86 occurrences after the repeating section should not be consumed
    let is_mt942_statement = message_type.contains("MT942StatementLine");

    // Track whether we've seen Field 61 in the current sequence to know if we should expect Field 86
    let mut has_field_61_in_sequence = false;
    let mut has_field_86_in_sequence = false;

    for (tag, value, pos) in all_fields {
        // Check if this is the start of a new sequence
        let is_sequence_start = (tag == primary_marker
            || secondary_marker.is_some_and(|m| tag == m))
            && !tag.ends_with("R")
            && !tag.ends_with("F")
            && !tag.ends_with("C")
            && !tag.ends_with("D");

        if is_sequence_start {
            // If we were already in a sequence, parse the previous one
            if in_sequence && !current_sequence_fields.is_empty() {
                let block4_str = reconstruct_block4_from_fields(&current_sequence_fields);
                if let Ok(sequence_item) = T::parse_from_block4(&block4_str) {
                    sequences.push(sequence_item);
                }
                current_sequence_fields.clear();
            }
            in_sequence = true;
            has_field_61_in_sequence = true; // Field 61 is the sequence start marker
            has_field_86_in_sequence = false;
        }

        // For MT942StatementLine, be smart about which fields to include
        let should_include_in_sequence = if is_mt942_statement && in_sequence {
            if tag == "61" {
                // New Field 61 - if we already have one in the sequence, start a new sequence
                if has_field_61_in_sequence && !current_sequence_fields.is_empty() {
                    // Complete the previous sequence
                    let block4_str = reconstruct_block4_from_fields(&current_sequence_fields);
                    if let Ok(sequence_item) = T::parse_from_block4(&block4_str) {
                        sequences.push(sequence_item);
                    }
                    current_sequence_fields.clear();
                    has_field_86_in_sequence = false;
                }
                has_field_61_in_sequence = true;
                true
            } else if tag == "86" && has_field_61_in_sequence && !has_field_86_in_sequence {
                // This is the Field 86 that pairs with the current Field 61
                has_field_86_in_sequence = true;
                true
            } else {
                // Any other field ends the sequence
                false
            }
        } else if !is_mt942_statement && in_sequence {
            // For non-MT942 messages, include all fields in the sequence
            true
        } else {
            false
        };

        // Add field to current sequence if we should include it
        if should_include_in_sequence {
            #[cfg(debug_assertions)]
            if tag.starts_with("50") || tag.starts_with("90") || tag.starts_with("86") {
                eprintln!(
                    "DEBUG: Adding field to sequence: tag='{}', value_start='{}'",
                    tag,
                    value.lines().next().unwrap_or("")
                );
            }
            current_sequence_fields
                .entry(tag.clone())
                .or_default()
                .push((value, pos));

            // Mark this field as consumed
            tracker.mark_consumed(&tag, pos);
        } else if is_mt942_statement && in_sequence && !should_include_in_sequence {
            // For MT942, if we encounter a field that shouldn't be in the sequence,
            // we should end the current sequence and not consume this field
            if !current_sequence_fields.is_empty() {
                let block4_str = reconstruct_block4_from_fields(&current_sequence_fields);
                if let Ok(sequence_item) = T::parse_from_block4(&block4_str) {
                    sequences.push(sequence_item);
                }
                current_sequence_fields.clear();
            }
            in_sequence = false;
            has_field_61_in_sequence = false;
            has_field_86_in_sequence = false;

            #[cfg(debug_assertions)]
            eprintln!("DEBUG: Ending MT942 sequence at field: tag='{}'", tag);
        }
    }

    // Parse the last sequence if there is one
    if in_sequence && !current_sequence_fields.is_empty() {
        let block4_str = reconstruct_block4_from_fields(&current_sequence_fields);
        match T::parse_from_block4(&block4_str) {
            Ok(sequence_item) => {
                sequences.push(sequence_item);
            }
            Err(_e) => {
                #[cfg(debug_assertions)]
                eprintln!("DEBUG: Failed to parse final sequence item: {_e:?}");
            }
        }
    }

    Ok(sequences)
}

/// Parse sequence B items from already split fields
fn parse_sequence_b_items<T>(
    fields: &HashMap<String, Vec<(String, usize)>>,
    tracker: &mut FieldConsumptionTracker,
) -> Result<Vec<T>>
where
    T: crate::SwiftMessageBody,
{
    let mut sequences = Vec::new();

    // Get all fields sorted by position
    let mut all_fields: Vec<(String, String, usize)> = Vec::new();
    for (tag, values) in fields {
        for (value, pos) in values {
            all_fields.push((tag.clone(), value.clone(), *pos));
        }
    }
    all_fields.sort_by_key(|(_, _, pos)| *pos);

    #[cfg(debug_assertions)]
    eprintln!(
        "DEBUG parse_sequence_b_items: found {} total fields to process",
        all_fields.len()
    );

    // Determine the sequence start tag based on message type
    let message_type = std::any::type_name::<T>();
    let sequence_start_tag = if message_type.contains("MT204Transaction") {
        "20" // MT204 transactions start with field 20
    } else {
        "21" // Most other transactions start with field 21
    };
    let mut current_sequence_fields: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut in_sequence = false;

    for (tag, value, pos) in all_fields {
        // Check if this is the start of a new sequence
        if tag == sequence_start_tag
            && !tag.ends_with("R")
            && !tag.ends_with("F")
            && !tag.ends_with("C")
            && !tag.ends_with("D")
        {
            #[cfg(debug_assertions)]
            eprintln!(
                "DEBUG: Found sequence start tag {} at position {}, in_sequence={}, current_fields_count={}",
                tag,
                pos,
                in_sequence,
                current_sequence_fields.len()
            );

            // If we were already in a sequence, parse the previous one
            if in_sequence && !current_sequence_fields.is_empty() {
                #[cfg(debug_assertions)]
                eprintln!(
                    "DEBUG: Parsing previous sequence with {} field types",
                    current_sequence_fields.len()
                );

                let block4_str = reconstruct_block4_from_fields(&current_sequence_fields);
                match T::parse_from_block4(&block4_str) {
                    Ok(sequence_item) => {
                        sequences.push(sequence_item);
                        #[cfg(debug_assertions)]
                        eprintln!("DEBUG: Successfully parsed sequence #{}", sequences.len());
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("DEBUG: Failed to parse sequence: {}", e);
                    }
                }
                current_sequence_fields.clear();
            }
            in_sequence = true;
        }

        // Add field to current sequence if we're in one
        if in_sequence {
            #[cfg(debug_assertions)]
            if tag.starts_with("50") {
                eprintln!(
                    "DEBUG: Adding field to sequence: tag='{}', value_start='{}'",
                    tag,
                    value.lines().next().unwrap_or("")
                );
            }
            current_sequence_fields
                .entry(tag.clone())
                .or_default()
                .push((value, pos));

            // Mark this field as consumed
            tracker.mark_consumed(&tag, pos);
        }
    }

    // Parse the last sequence if there is one
    if in_sequence && !current_sequence_fields.is_empty() {
        #[cfg(debug_assertions)]
        eprintln!(
            "DEBUG: Parsing final sequence with {} field types",
            current_sequence_fields.len()
        );

        let block4_str = reconstruct_block4_from_fields(&current_sequence_fields);
        match T::parse_from_block4(&block4_str) {
            Ok(sequence_item) => {
                sequences.push(sequence_item);
                #[cfg(debug_assertions)]
                eprintln!(
                    "DEBUG: Successfully parsed final sequence #{}",
                    sequences.len()
                );
            }
            Err(_e) => {
                #[cfg(debug_assertions)]
                eprintln!("DEBUG: Failed to parse final sequence item: {_e:?}");
            }
        }
    }

    #[cfg(debug_assertions)]
    eprintln!(
        "DEBUG: parse_sequence_b_items returning {} sequences",
        sequences.len()
    );

    Ok(sequences)
}
