use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 34F: Floor Limit
/// Format: 3!a[1!a]15d (currency + indicator + amount)
/// Validation: currency_code, floor_limit_indicator, amount_format, positive_amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field34F {
    /// Currency code (3!a format, ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Floor limit indicator (1!a format: D or C)
    #[component("1!a", validate = ["floor_limit_indicator"])]
    pub indicator: String,
    /// Amount (15d format)
    #[component("15d", validate = ["amount_format", "positive_amount"])]
    pub amount: f64,
}
