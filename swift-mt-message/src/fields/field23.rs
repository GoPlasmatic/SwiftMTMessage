use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 23: Further Identification
/// Format: 3!a[2!n]11x (function code + optional days + reference)
/// Validation: function_code, reference_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field23 {
    /// Function code (3!a format: BASE, CALL, COMMERCIAL, CURRENT, DEPOSIT, NOTICE, PRIME)
    #[component("3!a", validate = ["function_code"])]
    pub function_code: String,
    /// Number of days (2!n format, optional, only for NOTICE function)
    #[component("2!n", optional, validate = ["positive_amount"])]
    pub days: Option<u8>,
    /// Reference information (11x format)
    #[component("11x", validate = ["reference_format"])]
    pub reference: String,
}
