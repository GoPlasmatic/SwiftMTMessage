use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field11R {
    /// Message type of the original message (3 digits)
    #[component("3!n")]
    pub message_type: String,

    /// Date of the original message in YYMMDD format (6 digits)
    #[component("6!n")]
    pub date: NaiveDate,

    /// Session number of the original message (4 digits)
    #[component("[4!n]")]
    pub session_number: Option<String>,

    /// Input Sequence Number of the original message (4 digits)
    #[component("[6!n]")]
    pub input_sequence_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field11S {
    /// Message type of the original message (3 digits)
    #[component("3!n")]
    pub message_type: String,

    /// Date of the original message in YYMMDD format (6 digits)
    #[component("6!n")]
    pub date: NaiveDate,

    /// Session number of the original message (4 digits)
    #[component("[4!n]")]
    pub session_number: Option<String>,

    /// Input Sequence Number of the original message (4 digits)
    #[component("[6!n]")]
    pub input_sequence_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field11 {
    R(Field11R),
    S(Field11S),
}
