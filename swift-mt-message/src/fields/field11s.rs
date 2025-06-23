use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 11S: MT and Date of the Original Message
///
/// For Cancellation and Request for Payment messages, this field contains
/// the message type and date of the original message that is being cancelled or referenced.
///
/// **Format**: 3!n6!n4!n4!n
/// - Message Type (3 digits): The original message type (e.g., 103, 202)
/// - Date (6 digits): YYMMDD format
/// - Session Number (4 digits): Session of original message
/// - Input Sequence Number (4 digits): ISN of original message
///
/// Note: The format may include an optional slash (/) before the input sequence number,
/// but this is handled by the parser which strips it out before processing.
///
/// ## Usage
/// - **MT192**: Request for Cancellation (Customer Transfer)
/// - **MT292**: Request for Cancellation (Financial Institution Transfer)
/// - References original message for processing
///
/// ## Validation
/// - Message type must be valid 3-digit MT number
/// - Date must be valid YYMMDD format within acceptable range
/// - Session and ISN must be 4-digit numbers
///
/// Example: 103231215001/0123 (MT103 dated 2023-12-15, session 0001, ISN 0123)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field11S {
    /// Message type of the original message (3 digits)
    #[component("3!n", validate = ["message_type"])]
    pub message_type: String,

    /// Date of the original message in YYMMDD format (6 digits)
    #[component("6!n", validate = ["date_format", "valid_date_range"])]
    pub date: NaiveDate,

    /// Session number of the original message (4 digits)
    #[component("4!n", validate = ["session_number"])]
    pub session_number: String,

    /// Input Sequence Number of the original message (4 digits)
    #[component("4!n", validate = ["input_sequence_number"])]
    pub input_sequence_number: String,
}
