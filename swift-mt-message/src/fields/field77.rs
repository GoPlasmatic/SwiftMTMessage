use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// # Field 77T: Envelope Contents
/// Format: 1!a1!a35x (envelope type + format + identifier)
/// Validation: reference_format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field77T {
    #[component("9000z")]
    pub envelope_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field77A {
    #[component("20*35x")]
    pub narrative: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field77B {
    #[component("3*35x")]
    pub narrative: Vec<String>,
}
