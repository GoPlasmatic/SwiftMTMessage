use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field33B {
    /// Currency code (3!a format, ISO 4217)
    #[component("3!a")]
    pub currency: String,
    /// Amount (15d format)
    #[component("15d")]
    pub amount: f64,
}
