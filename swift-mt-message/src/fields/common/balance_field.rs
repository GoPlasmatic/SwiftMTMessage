use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Generic Balance Field
/// Used for balance fields like Field60F, Field62F, Field64, Field65, etc.
/// Format: 1!a6!n3!a15d (D/C indicator + date + currency + amount)
/// Validation: debit_credit_indicator, date_format, valid_date_range, currency_code, amount_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericBalanceField {
    /// Debit/Credit indicator (1!a format: D, C, RD, RC)
    #[component("1!a", validate = ["debit_credit_indicator"])]
    pub indicator: String,
    /// Date (6!n format, YYMMDD)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub date: NaiveDate,
    /// Currency code (3!a format, ISO 4217)
    #[component("3!a", validate = ["currency_code"])]
    pub currency: String,
    /// Amount value (15d format)
    #[component("15d", validate = ["amount_format"])]
    pub amount: f64,
}
