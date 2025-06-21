use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Generic Reference Field
///
/// Used for reference fields like transaction references and related references.
/// Handles 16x format with reference-specific validation rules.
/// Format: 16x (up to 16 alphanumeric characters)
/// Validation: reference_format, no_slashes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericReferenceField {
    /// Reference value
    #[component("16x", validate = ["reference_format", "no_slashes"])]
    pub value: String,
}
