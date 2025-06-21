use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 23E: Instruction Code
///
/// Instruction code with optional additional information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field23E {
    /// Instruction code
    #[component("4!c", validate = ["instruction_code"])]
    pub instruction_code: String,
    /// Additional information (optional)
    #[component("30x", optional, validate = ["reference_format"])]
    pub additional_info: Option<String>,
}
