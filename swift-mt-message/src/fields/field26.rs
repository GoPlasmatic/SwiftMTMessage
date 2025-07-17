use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field26T {
    #[component("3!c")]
    pub type_code: String,
}
