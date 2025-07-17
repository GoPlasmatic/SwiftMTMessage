use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field51A {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}
