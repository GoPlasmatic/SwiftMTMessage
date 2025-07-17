use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field71A {
    #[component("3!a")]
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field71F {
    #[component("3!a")]
    pub currency: String,

    #[component("15d")]
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field71G {
    #[component("3!a")]
    pub currency: String,

    #[component("15d")]
    pub amount: f64,
}
