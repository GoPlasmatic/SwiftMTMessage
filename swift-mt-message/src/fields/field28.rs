use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 28: Statement Number/Sequence Number
/// Format: 5n[/2n] (statement number + optional sequence)
/// Validation: positive_amount (for numbers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28 {
    /// Statement number (5n format, 1-5 digits)
    #[component("5n", validate = ["positive_amount"])]
    pub statement_number: u32,

    /// Optional sequence number (2n format, 1-2 digits)
    #[component("2n", optional, validate = ["positive_amount"])]
    pub sequence_number: Option<u8>,
}
