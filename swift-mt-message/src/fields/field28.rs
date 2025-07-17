use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 28: Statement Number/Sequence Number
/// Format: 5n[/2n] (statement number + optional sequence)
/// Validation: positive_amount (for numbers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28 {
    /// Statement number (5n format, 1-5 digits)
    #[component("5n")]
    pub statement_number: u32,

    /// Optional sequence number (2n format, 1-2 digits)
    #[component("[/2n]")]
    pub sequence_number: Option<u8>,
}

/// # Field 28C: Statement Number/Sequence Number
/// Format: 5n[/5n] (statement number + optional sequence)
/// Validation: positive_amount (for numbers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28C {
    /// Statement number (5n format)
    #[component("5n")]
    pub statement_number: u32,
    /// Optional sequence number (5n format)
    #[component("[/5n]")]
    pub sequence_number: Option<u32>,
}

/// # Field 28D: Message Index/Total
/// Format: 5n/5n (index + total)
/// Validation: positive_amount (for numbers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28D {
    /// Message index (5n format)
    #[component("5n")]
    pub index: u32,
    /// Total message count (5n format)
    #[component("5n")]
    pub total: u32,
}
