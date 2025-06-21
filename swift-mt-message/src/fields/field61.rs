use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 61: Statement Line
/// Format: 6!n[4!n]2a[1!a]15d4!a2!a[16x][34x] (complex composite format)
/// Validation: date_format, valid_date_range, debit_credit_indicator, amount_format, reference_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field61 {
    /// Value date (6!n format, YYMMDD)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub value_date: NaiveDate,

    /// Optional entry date (4!n format, MMDD)
    #[component("4!n", optional, validate = ["date_format"])]
    pub entry_date: Option<String>,

    /// Debit/Credit mark (2a format: D, C, RD, RC)
    #[component("2a", validate = ["debit_credit_indicator"])]
    pub debit_credit_mark: String,

    /// Optional funds code (1!a format)
    #[component("1!a", optional)]
    pub funds_code: Option<char>,

    /// Amount (15d format)
    #[component("15d", validate = ["amount_format"])]
    pub amount: f64,

    /// Transaction type identification code (4!a format)
    #[component("4!a", validate = ["reference_format"])]
    pub transaction_type: String,

    /// Customer reference (2!a format)
    #[component("2!a", validate = ["reference_format"])]
    pub customer_reference: String,

    /// Optional bank reference (16x format, preceded by //)
    #[component("16x", optional, validate = ["reference_format"])]
    pub bank_reference: Option<String>,

    /// Optional supplementary details (34x format)
    #[component("34x", optional, validate = ["reference_format"])]
    pub supplementary_details: Option<String>,
}
