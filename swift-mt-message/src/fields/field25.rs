use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field25NoOption {
    #[component("35x")]
    pub authorisation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field25A {
    #[component("/34x")]
    pub account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field25P {
    #[component("35x")]
    pub account: String,

    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field25AccountIdentification {
    NoOption(Field25NoOption),
    P(Field25P),
}
