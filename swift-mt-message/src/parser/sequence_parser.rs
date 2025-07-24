//! Generic sequence parser for SWIFT MT messages with multiple sequences
//!
//! Many SWIFT MT messages have multiple sequences:
//! - MT101: Sequence A (General Info), Sequence B (Transactions)
//! - MT104: Sequence A (General Info), Sequence B (Transactions), Sequence C (Settlement)
//! - MT107: Similar structure with multiple sequences
//!
//! This module provides generic parsing capabilities for such messages.

use crate::errors::Result;
use std::collections::HashMap;

/// Type alias for field storage to reduce complexity
pub type FieldMap = HashMap<String, Vec<(String, usize)>>;

/// Configuration for sequence parsing
#[derive(Debug, Clone)]
pub struct SequenceConfig {
    /// Field that marks the start of sequence B (usually "21")
    pub sequence_b_marker: String,
    /// Fields that belong exclusively to sequence C (if any)
    pub sequence_c_fields: Vec<String>,
    /// Whether sequence C exists for this message type
    pub has_sequence_c: bool,
}

impl Default for SequenceConfig {
    fn default() -> Self {
        Self {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        }
    }
}

/// Parsed sequences from a SWIFT message
#[derive(Debug)]
pub struct ParsedSequences {
    /// Sequence A fields (general information)
    pub sequence_a: FieldMap,
    /// Sequence B fields (repetitive items like transactions)
    pub sequence_b: FieldMap,
    /// Sequence C fields (optional settlement/summary information)
    pub sequence_c: FieldMap,
}

/// Split fields into sequences based on configuration
pub fn split_into_sequences(fields: &FieldMap, config: &SequenceConfig) -> Result<ParsedSequences> {
    let mut seq_a = HashMap::new();
    let mut seq_b = HashMap::new();
    let mut seq_c = HashMap::new();

    // Get all fields sorted by position
    let mut all_fields: Vec<(&str, &(String, usize))> = Vec::new();
    for (tag, values) in fields {
        for value in values {
            all_fields.push((tag.as_str(), value));
        }
    }
    all_fields.sort_by_key(|(_, (_, pos))| *pos);

    // Find sequence boundaries
    let mut first_b_marker_pos = None;
    let mut _last_b_marker_pos = None;

    // Special handling for MT935 which uses "23" or "25" as sequence markers
    let secondary_marker = if config.sequence_b_marker == "23" {
        Some("25")
    } else {
        None
    };

    // Fields that belong to sequence A even if they appear after sequence B
    let sequence_a_fields = ["72", "77E", "79"];

    for (tag, (_, pos)) in &all_fields {
        if is_sequence_b_marker(tag, &config.sequence_b_marker) 
            || (secondary_marker.is_some() && *tag == secondary_marker.unwrap()) {
            if first_b_marker_pos.is_none() {
                first_b_marker_pos = Some(*pos);
            }
            _last_b_marker_pos = Some(*pos);
        }
    }

    // Simpler approach: find all sequence B boundaries
    // Sequence B starts at first field 21 and includes all fields until sequence C
    let sequence_b_start_idx = all_fields
        .iter()
        .position(|(tag, _)| {
            is_sequence_b_marker(tag, &config.sequence_b_marker) 
                || (secondary_marker.is_some() && *tag == secondary_marker.unwrap())
        });

    // Find where sequence C would start (if it exists)
    // This is tricky: sequence C fields appear after ALL transactions
    // We need to find the last occurrence of transaction-ending fields
    let mut sequence_c_start_idx: Option<usize> = None;

    if config.has_sequence_c && sequence_b_start_idx.is_some() {
        // Look for sequence C fields that appear after transaction patterns
        // Transaction patterns typically end with fields like 59, 70, 71A
        let transaction_end_fields = ["59", "70", "71A", "77B", "36"];

        // Find the last occurrence of any transaction-ending field
        let mut last_trans_end_idx: Option<usize> = None;
        for (i, (tag, _)) in all_fields.iter().enumerate() {
            let base_tag = tag.trim_end_matches(char::is_alphabetic);
            if transaction_end_fields.contains(&base_tag) {
                last_trans_end_idx = Some(i);
            }
        }

        // Look for sequence C fields after the last transaction end
        if let Some(last_end) = last_trans_end_idx {
            for (i, (tag, _)) in all_fields.iter().enumerate().skip(last_end + 1) {
                if config.sequence_c_fields.contains(&tag.to_string()) {
                    sequence_c_start_idx = Some(i);
                    break;
                }
            }
        } else {
            // If no transaction-ending fields found, look for sequence C fields
            // after the sequence B start
            if let Some(seq_b_start) = sequence_b_start_idx {
                for (i, (tag, _)) in all_fields.iter().enumerate().skip(seq_b_start) {
                    if config.sequence_c_fields.contains(&tag.to_string()) {
                        sequence_c_start_idx = Some(i);
                        break;
                    }
                }
            }
        }
    }

    // Distribute fields to sequences based on boundaries
    for (i, (tag, (value, pos))) in all_fields.iter().enumerate() {
        // Check if this field should always be in sequence A
        if sequence_a_fields.contains(&tag) {
            seq_a
                .entry(tag.to_string())
                .or_insert_with(Vec::new)
                .push((value.clone(), *pos));
            continue;
        }
        
        if let Some(seq_b_start) = sequence_b_start_idx {
            if i < seq_b_start {
                // Before sequence B = Sequence A
                seq_a
                    .entry(tag.to_string())
                    .or_insert_with(Vec::new)
                    .push((value.clone(), *pos));
            } else if let Some(seq_c_start) = sequence_c_start_idx {
                if i >= seq_c_start {
                    // After sequence C start = Sequence C
                    seq_c
                        .entry(tag.to_string())
                        .or_insert_with(Vec::new)
                        .push((value.clone(), *pos));
                } else {
                    // Between sequence B start and C start = Sequence B
                    seq_b
                        .entry(tag.to_string())
                        .or_insert_with(Vec::new)
                        .push((value.clone(), *pos));
                }
            } else {
                // No sequence C, everything after sequence B start is sequence B
                seq_b
                    .entry(tag.to_string())
                    .or_insert_with(Vec::new)
                    .push((value.clone(), *pos));
            }
        } else {
            // No sequence B found, everything is sequence A
            seq_a
                .entry(tag.to_string())
                .or_insert_with(Vec::new)
                .push((value.clone(), *pos));
        }
    }

    Ok(ParsedSequences {
        sequence_a: seq_a,
        sequence_b: seq_b,
        sequence_c: seq_c,
    })
}

/// Parse repetitive sequence items (like transactions)
pub fn parse_repetitive_sequence<T>(fields: &FieldMap, marker_field: &str) -> Result<Vec<FieldMap>>
where
    T: crate::SwiftMessageBody,
{
    let mut items = Vec::new();

    // Get all fields sorted by position
    let mut all_fields: Vec<(String, String, usize)> = Vec::new();
    for (tag, values) in fields {
        for (value, pos) in values {
            all_fields.push((tag.clone(), value.clone(), *pos));
        }
    }
    all_fields.sort_by_key(|(_, _, pos)| *pos);

    // Group fields by item (each starting with marker field)
    let mut current_item_fields: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut in_item = false;

    for (tag, value, pos) in all_fields {
        // Check if this is the start of a new item
        if is_sequence_b_marker(&tag, marker_field) {
            // Save previous item if exists
            if in_item && !current_item_fields.is_empty() {
                items.push(current_item_fields.clone());
                current_item_fields.clear();
            }
            in_item = true;
        }

        // Add field to current item if we're in one
        if in_item {
            current_item_fields
                .entry(tag)
                .or_default()
                .push((value, pos));
        }
    }

    // Save the last item
    if in_item && !current_item_fields.is_empty() {
        items.push(current_item_fields);
    }

    Ok(items)
}

/// Check if a field tag is a sequence B marker
fn is_sequence_b_marker(tag: &str, marker: &str) -> bool {
    // Handle simple markers like "21"
    if tag == marker {
        return true;
    }

    // Handle numbered markers (e.g., "21" but not "21R", "21C", etc.)
    if marker == "21" && tag == "21" {
        return true;
    }

    false
}

/// Get sequence configuration for a specific message type
pub fn get_sequence_config(message_type: &str) -> SequenceConfig {
    match message_type {
        "MT101" => SequenceConfig {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
        "MT104" => SequenceConfig {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec![
                "32B".to_string(),
                "19".to_string(),
                "71F".to_string(),
                "71G".to_string(),
                "53".to_string(),
            ],
            has_sequence_c: true,
        },
        "MT107" => SequenceConfig {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
        _ => SequenceConfig::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_splitting() {
        let mut fields = HashMap::new();

        // Sequence A fields
        fields.insert("20".to_string(), vec![("REF123".to_string(), 100)]);
        fields.insert("30".to_string(), vec![("240722".to_string(), 200)]);

        // Sequence B fields
        fields.insert(
            "21".to_string(),
            vec![("TRANS1".to_string(), 300), ("TRANS2".to_string(), 500)],
        );
        fields.insert(
            "32B".to_string(),
            vec![("USD1000".to_string(), 350), ("USD2000".to_string(), 550)],
        );

        // Sequence C fields (for MT104)
        fields.insert("19".to_string(), vec![("USD3000".to_string(), 600)]);

        let config = SequenceConfig {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec!["19".to_string()],
            has_sequence_c: true,
        };

        let sequences = split_into_sequences(&fields, &config).unwrap();

        assert_eq!(sequences.sequence_a.len(), 2);
        assert_eq!(sequences.sequence_b.len(), 2);
        assert_eq!(sequences.sequence_c.len(), 1);
    }
}
