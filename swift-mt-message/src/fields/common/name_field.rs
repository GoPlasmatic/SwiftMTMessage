use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Generic Name and Address Field
/// Used for structured name and address information.
/// Format: 4*35x (up to 4 lines, 35 characters each)
/// Validation: line_count, line_length, structured_address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericNameAddressField {
    /// Lines of name and address information (4*35x format)
    #[component("4*35x", validate = ["line_count", "line_length", "structured_address"])]
    pub name_and_address: Vec<String>,
}
