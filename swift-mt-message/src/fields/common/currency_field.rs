use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Generic Currency Amount Field
/// Used for fields like Field33B (Instructed Amount), Field71F (Sender's Charges), Field71G (Receiver's Charges), etc.
/// Format: 3!a15d (3-letter currency code + decimal amount)
/// Validation: currency_code, amount_format, positive_amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericCurrencyAmountField {
    /// Currency code (3!a format, ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Amount value (15d format)
    #[component("15d", validate = ["amount_format", "positive_amount"])]
    pub amount: f64,
}
