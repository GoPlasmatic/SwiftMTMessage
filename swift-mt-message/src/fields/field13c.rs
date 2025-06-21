use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 13C: Time Indication
///
/// Time indication with code, time, and UTC offset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13C {
    /// Time indication code with slashes
    #[component("/8c/", validate = ["time_indication_code"])]
    pub time_code: String,
    /// Time (HHMM)
    #[component("4!n", validate = ["time_format"])]
    pub time: String,
    /// UTC offset with sign
    #[component("1!x4!n", validate = ["utc_offset_format"])]
    pub utc_offset: String,
}
