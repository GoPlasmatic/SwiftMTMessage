use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 32A: Value Date, Currency Code, Amount
/// Format: 6!n3!a15d (date + currency + amount)
/// Validation: date_format, valid_date_range, currency_code, amount_format, positive_amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field32A {
    /// Value date (6!n format, YYMMDD)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub value_date: NaiveDate,
    /// Currency code (3!a format, ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Amount (15d format)
    #[component("15d", validate = ["amount_format", "positive_amount"])]
    pub amount: f64,
}
