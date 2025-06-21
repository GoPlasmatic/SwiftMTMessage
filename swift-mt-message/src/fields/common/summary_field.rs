use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Generic Summary Field
///
/// Used for summary fields with entry count, currency, and amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericSummaryField {
    /// Number of entries
    #[component("5n", validate = ["positive_amount"])]
    pub entry_count: u32,
    /// Currency code (ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Total amount
    #[component("15d", validate = ["amount_format"])]
    pub amount: f64,
}
