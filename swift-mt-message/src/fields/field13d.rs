use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 13D: Date/Time Indication
///
/// Date and time indication with UTC offset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13D {
    /// Date component (YYMMDD)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub date: NaiveDate,
    /// Time component (HHMM)
    #[component("4!n", validate = ["time_format"])]
    pub time: NaiveTime,
    /// UTC offset sign
    #[component("1!a", validate = ["utc_offset_format"])]
    pub offset_sign: char,
    /// UTC offset in total seconds
    #[component("4!n", validate = ["utc_offset_format"])]
    pub offset_seconds: i32,
}
