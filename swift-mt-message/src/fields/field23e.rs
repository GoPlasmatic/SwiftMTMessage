use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 23E: Instruction Code
/// Format: 4!c[/30x] (instruction code + optional additional info)
/// Validation: instruction_code, reference_format (for additional_info)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field23E {
    /// Instruction code (4!c format: CORT, INTC, REPA, SDVA, etc.)
    #[component("4!c", validate = ["instruction_code"])]
    pub instruction_code: String,
    /// Additional information (30x format, optional)
    #[component("30x", optional, validate = ["reference_format"])]
    pub additional_info: Option<String>,
}
