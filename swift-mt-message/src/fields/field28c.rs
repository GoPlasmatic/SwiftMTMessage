use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 28C: Statement Number/Sequence Number
/// Format: 5n[/5n] (statement number + optional sequence)
/// Validation: positive_amount (for numbers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28C {
    /// Statement number (5n format)
    #[component("5n", validate = ["positive_amount"])]
    pub statement_number: u32,
    /// Optional sequence number (5n format)
    #[component("5n", optional, validate = ["positive_amount"])]
    pub sequence_number: Option<u32>,
}
