use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field57A {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field57B {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("[35x]")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field57C {
    #[component("/34x")]
    pub party_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field57D {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field57AccountWithInstitution {
    A(Field57A),
    B(Field57B),
    C(Field57C),
    D(Field57D),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field57DebtorBank {
    A(Field57A),
    C(Field57C),
    D(Field57D),
}
