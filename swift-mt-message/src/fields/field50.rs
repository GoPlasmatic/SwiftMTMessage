use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50NoOption {
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50A {
    /// Party identifier
    #[component("[/34x]")]
    pub party_identifier: Option<String>,
    /// Name and address lines
    #[component("4*(1!n/33x)")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50F {
    #[component("35x")]
    pub account: String,
    /// Name and address lines
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50K {
    #[component("[/34x]")]
    pub account: Option<String>,
    /// Name and address lines
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50C {
    /// BIC code
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50L {
    #[component("35x")]
    pub party_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50G {
    #[component("/34x")]
    pub account: String,
    /// Name and address lines
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50H {
    #[component("/34x")]
    pub account: String,
    /// Name and address lines
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50InstructingParty {
    C(Field50C),
    L(Field50L),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50OrderingCustomerFGH {
    F(Field50F),
    G(Field50G),
    H(Field50H),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50OrderingCustomerAFK {
    A(Field50A),
    F(Field50F),
    K(Field50K),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50OrderingCustomerNCF {
    NoOption(Field50NoOption),
    C(Field50C),
    F(Field50F),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50Creditor {
    A(Field50A),
    K(Field50K),
}
