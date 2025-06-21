use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 13C: Time Indication
/// Format: /8c/4!n1!x4!n (code between slashes + time + sign + offset)
/// Example: /CLSTIME/0915+0100
/// Validation: time_indication_code, time_format, utc_offset_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13C {
    /// Time indication code with slashes (/8c/ format: /CLSTIME/, /RNCTIME/, /SNDTIME/)
    #[component("/8c/", validate = ["time_indication_code"])]
    pub time_code: String,
    /// Time (4!n format: HHMM)
    #[component("4!n", validate = ["time_format"])]
    pub time: String,
    /// UTC offset with sign (1!x4!n format: +HHMM or -HHMM)
    #[component("1!x4!n", validate = ["utc_offset_format"])]
    pub utc_offset: String,
}
