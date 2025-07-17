use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field86 {
    #[component("6*65x")]
    pub narrative: Vec<String>,
}
