use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Generic Currency Amount Field
///
/// Used for fields with currency code and decimal amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericCurrencyAmountField {
    /// Currency code (ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Amount value
    #[component("15d", validate = ["amount_format", "positive_amount"])]
    pub amount: f64,
}
