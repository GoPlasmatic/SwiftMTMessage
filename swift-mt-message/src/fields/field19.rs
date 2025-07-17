use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field19 {
    /// Amount (17d format)
    #[component("17d")]
    pub amount: f64,
}
