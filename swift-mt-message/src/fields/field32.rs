use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 32A: Value Date, Currency Code, Amount
///
/// Core transaction field with value date, currency, and amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field32A {
    /// Value date (YYMMDD)
    #[component("6!n")]
    pub value_date: Option<NaiveDate>,
    /// Currency code (ISO 4217)
    #[component("3!a")]
    pub currency: String,
    /// Amount
    #[component("15d")]
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field32B {
    /// Currency code (ISO 4217)
    #[component("3!a")]
    pub currency: String,
    /// Amount
    #[component("15d")]
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field32 {
    A(Field32A),
    B(Field32B),
}
