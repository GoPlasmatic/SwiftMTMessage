use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// # Field 28D: Message Index/Total
///
/// ## Overview
/// Field 28D contains the message index and total count when a large batch of transactions
/// is split across multiple SWIFT MT messages. This field enables proper sequencing and
/// reassembly of message parts at the receiving end, ensuring all related messages are
/// processed together and in the correct order.
///
/// ## Format Specification
/// **Format**: `5n/5n`
/// - **5n**: Index - 5 numeric characters (00001-99999)
/// - **/** - Fixed separator
/// - **5n**: Total - 5 numeric characters (00001-99999)
/// - **Validation**: Index must be ≤ Total, both must be positive
/// - **Leading zeros**: Required to maintain 5-digit format
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("5n/5n")]
pub struct Field28D {
    /// Message index (1-based, padded to 5 digits)
    #[format("5n")]
    pub index: String,

    /// Total message count in the sequence (padded to 5 digits)
    #[format("5n")]
    pub total: String,
}

impl Field28D {
    /// Create a new Field28D with specified index and total
    pub fn new(index: &str, total: &str) -> Self {
        Self {
            index: index.to_string(),
            total: total.to_string(),
        }
    }

    /// Create a new Field28D from numeric values
    pub fn from_numbers(index: u32, total: u32) -> Self {
        let index_str = format!("{:05}", index);
        let total_str = format!("{:05}", total);
        Self::new(&index_str, &total_str)
    }

    /// Get the message index as string
    pub fn index(&self) -> &str {
        &self.index
    }

    /// Get the total message count as string
    pub fn total(&self) -> &str {
        &self.total
    }

    /// Get the message index as numeric value
    pub fn index_number(&self) -> u32 {
        self.index.parse().unwrap_or(0)
    }

    /// Get the total message count as numeric value
    pub fn total_number(&self) -> u32 {
        self.total.parse().unwrap_or(0)
    }

    /// Check if this is the first message in the sequence
    pub fn is_first(&self) -> bool {
        self.index == "00001"
    }

    /// Check if this is the last message in the sequence
    pub fn is_last(&self) -> bool {
        self.index == self.total
    }

    /// Check if this represents a single message (not part of a sequence)
    pub fn is_single_message(&self) -> bool {
        self.total == "00001"
    }

    /// Get the progress percentage of this message in the sequence
    pub fn progress_percentage(&self) -> f64 {
        if self.total_number() == 0 {
            0.0
        } else {
            (self.index_number() as f64 / self.total_number() as f64) * 100.0
        }
    }

    /// Format as SWIFT field value (index/total)
    pub fn format_value(&self) -> String {
        format!("{}/{}", self.index, self.total)
    }

    /// Get the index and total as a tuple
    pub fn message_index_total(&self) -> (&str, &str) {
        (&self.index, &self.total)
    }
}
