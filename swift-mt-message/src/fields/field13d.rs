use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 13D: Date/Time Indication
/// Format: 6!n4!n1!x4!n (date + time + UTC offset)
/// Validation: date_format, valid_date_range, time_format, utc_offset_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13D {
    /// Date component (6!n format, YYMMDD)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub date: NaiveDate,
    /// Time component (4!n format, HHMM)
    #[component("4!n", validate = ["time_format"])]
    pub time: NaiveTime,
    /// UTC offset sign (1!x format: + or -)
    #[component("1!a", validate = ["utc_offset_format"])]
    pub offset_sign: char,
    /// UTC offset in total seconds
    #[component("4!n", validate = ["utc_offset_format"])]
    pub offset_seconds: i32,
}
