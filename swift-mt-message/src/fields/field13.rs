use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 13C: Time Indication
///
/// Time indication with code, time, sign, and time offset.
/// Format: /8c/4!n1!x4!n
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13C {
    /// Time indication code with slashes (e.g., /SNDTIME/, /CLSTIME/, /RNCTIME/)
    #[component("/8c/")]
    pub code: String,
    /// Time indication (HHMM)
    #[component("4!n")]
    pub time: String,
    /// Sign (+ or -)
    #[component("1!x")]
    pub sign: String,
    /// Time offset (HHMM)
    #[component("4!n")]
    pub offset: String,
}

/// Field 13D: Date/Time Indication
///
/// Date and time indication with UTC offset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13D {
    /// Date component (YYMMDD)
    #[component("6!n")]
    pub date: NaiveDate,
    /// Time component (HHMM)
    #[component("4!n")]
    pub time: NaiveTime,
    /// UTC offset sign
    #[component("1!x")]
    pub offset_sign: char,
    /// UTC offset in total seconds
    #[component("4!n")]
    pub offset_seconds: String,
}
