use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21NoOption {
    #[component("16x")]
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21C {
    #[component("35x")]
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21D {
    #[component("35x")]
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21E {
    #[component("35x")]
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21F {
    #[component("16x")]
    pub reference: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21R {
    #[component("16x")]
    pub reference: String,
}
