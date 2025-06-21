use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 32A: Value Date, Currency Code, Amount
///
/// Core transaction field with value date, currency, and amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field32A {
    /// Value date (YYMMDD)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub value_date: NaiveDate,
    /// Currency code (ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Amount
    #[component("15d", validate = ["amount_format", "positive_amount"])]
    pub amount: f64,
}
