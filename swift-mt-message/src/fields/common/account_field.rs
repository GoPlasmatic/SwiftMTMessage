use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Generic Account Field
///
/// Used for account identification fields with up to 35 alphanumeric characters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericAccountField {
    /// Account number or identifier (35x format)
    #[component("35x", validate = ["account_format"])]
    pub account_number: String,
}
