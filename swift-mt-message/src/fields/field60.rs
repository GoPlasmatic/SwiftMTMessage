use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field60F {
    #[component("1!a")]
    pub debit_credit_mark: String,

    #[component("6!n")]
    pub value_date: NaiveDate,

    #[component("3!a")]
    pub currency: String,

    #[component("15d")]
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field60M {
    #[component("1!a")]
    pub debit_credit_mark: String,

    #[component("6!n")]
    pub value_date: NaiveDate,

    #[component("3!a")]
    pub currency: String,

    #[component("15d")]
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field60 {
    F(Field60F),
    M(Field60M),
}
