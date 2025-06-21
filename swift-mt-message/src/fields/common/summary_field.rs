use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Generic Summary Field
/// Used for summary fields like Field90C (Sum of Credits), Field90D (Sum of Debits), etc.
/// Format: 5n3!a15d (entry count + currency + amount)
/// Validation: currency_code, amount_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericSummaryField {
    /// Number of entries (5n format, up to 5 digits)
    #[component("5n", validate = ["positive_amount"])]
    pub entry_count: u32,
    /// Currency code (3!a format, ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Total amount (15d format)
    #[component("15d", validate = ["amount_format"])]
    pub amount: f64,
}
