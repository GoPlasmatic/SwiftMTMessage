use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 77T: Envelope Contents
/// Format: 1!a1!a35x (envelope type + format + identifier)
/// Validation: reference_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field77T {
    /// Envelope type code (1!a format)
    #[component("1!a", validate = ["reference_format"])]
    pub envelope_type: String,
    /// Envelope format code (1!a format)
    #[component("1!a", validate = ["reference_format"])]
    pub envelope_format: String,
    /// Envelope identifier (35x format)
    #[component("35x", validate = ["reference_format"])]
    pub envelope_identifier: String,
}
