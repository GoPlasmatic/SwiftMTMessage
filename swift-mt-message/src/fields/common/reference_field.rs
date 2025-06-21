use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Generic Reference Field
/// Used for fields like Field20 (Transaction Reference), Field21 (Related Reference), etc.
/// Handles 16x format with reference-specific validation rules.
/// Format: 16x (up to 16 alphanumeric characters)
/// Validation: reference_format, no_slashes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericReferenceField {
    /// Reference value (16x format)
    #[component("16x", validate = ["reference_format", "no_slashes"])]
    pub value: String,
}
