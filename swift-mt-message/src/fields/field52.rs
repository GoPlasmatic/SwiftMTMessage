use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52A {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52B {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("[35x]")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52C {
    #[component("/34x")]
    pub party_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52D {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52AccountServicingInstitution {
    A(Field52A),
    C(Field52C),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52OrderingInstitution {
    A(Field52A),
    D(Field52D),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52CreditorBank {
    A(Field52A),
    C(Field52C),
    D(Field52D),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52DrawerBank  {
    A(Field52A),
    B(Field52B),
    D(Field52D),
}