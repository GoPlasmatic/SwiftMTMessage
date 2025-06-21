use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 36: Exchange Rate
/// Format: 12d (decimal rate)
/// Validation: rate_format, positive_rate, reasonable_exchange_rate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field36 {
    /// Exchange rate (12d format)
    #[component("12d", validate = ["rate_format", "positive_rate", "reasonable_exchange_rate"])]
    pub rate: f64,
}
