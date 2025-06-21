use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Generic Balance Field
///
/// Used for balance fields with debit/credit indicator, date, currency, and amount.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericBalanceField {
    /// Debit/Credit indicator (D, C, RD, RC)
    #[component("1!a", validate = ["debit_credit_indicator"])]
    pub indicator: String,
    /// Date (YYMMDD format)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub date: NaiveDate,
    /// Currency code (ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Amount value
    #[component("15d", validate = ["amount_format"])]
    pub amount: f64,
}
