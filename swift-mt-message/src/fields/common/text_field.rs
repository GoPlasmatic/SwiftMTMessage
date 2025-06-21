use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Generic Text Field
/// Used for simple text fields like Field23B (Bank Operation Code), Field71A (Details of Charges), etc.
/// Format: 35x (up to 35 alphanumeric characters)
/// Validation: reference_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericTextField {
    /// Text value (35x format)
    #[component("35x", validate = ["reference_format"])]
    pub value: String,
}
