use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Generic Text Field
///
/// Used for simple text fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericTextField {
    /// Text value
    #[component("35x", validate = ["reference_format"])]
    pub value: String,
}
