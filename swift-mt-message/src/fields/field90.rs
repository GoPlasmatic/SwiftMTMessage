use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field90D {
    #[component("5n")]
    pub number: u32,

    #[component("3!a")]
    pub currency: String,

    #[component("15d")]
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field90C {
    #[component("5n")]
    pub number: u32,

    #[component("3!a")]
    pub currency: String,

    #[component("15d")]
    pub amount: f64,
}
