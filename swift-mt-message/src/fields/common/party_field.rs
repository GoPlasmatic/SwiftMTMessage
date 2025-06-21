use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Generic Party Field
/// Used for party identification fields.
/// Format: 35x (up to 35 alphanumeric characters)
/// Validation: party_identifier_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericPartyField {
    /// Party identifier (35x format)
    #[component("35x", validate = ["party_identifier_format"])]
    pub party_identifier: String,
}
