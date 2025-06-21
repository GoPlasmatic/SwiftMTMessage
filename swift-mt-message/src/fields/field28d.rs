use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 28D: Message Index/Total
/// Format: 5n/5n (index + total)
/// Validation: positive_amount (for numbers)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field28D {
    /// Message index (5n format)
    #[component("5n", validate = ["positive_amount"])]
    pub index: u32,
    /// Total message count (5n format)
    #[component("5n", validate = ["positive_amount"])]
    pub total: u32,
}
