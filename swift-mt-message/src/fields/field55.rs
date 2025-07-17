use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field55A {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field55B {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("[35x]")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field55D {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field55ThirdReimbursementInstitution {
    A(Field55A),
    B(Field55B),
    D(Field55D),
}
