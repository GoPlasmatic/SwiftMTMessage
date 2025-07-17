use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 37H: Interest Rate
/// Format: 1!a[N]12d (indicator + optional negative + rate)
/// Validation: rate_format, positive_rate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field37H {
    /// Rate type indicator (1!a format: C or D)
    #[component("1!a")]
    pub rate_indicator: char,
    /// Whether this is a negative rate
    #[component("[1!a]")]
    pub is_negative: Option<bool>,
    /// Rate value (12d format, percentage)
    #[component("12d")]
    pub rate: f64,
}
