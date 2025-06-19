use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// # Field 25: Authorisation
///
/// ## Overview
/// Field 25 contains authorisation information for authentication and validation of credit
/// transfer instructions in SWIFT MT messages. This field provides a mechanism for including
/// authentication codes, approval references, or other authorisation data required by the
/// receiving institution to validate and process the payment instructions.
///
/// ## Format Specification
/// **Format**: `35x`
/// - **35x**: Up to 35 alphanumeric characters
/// - **Character set**: A-Z, a-z, 0-9, and printable ASCII characters
/// - **Case sensitivity**: Preserved as entered
/// - **Spaces**: Allowed within the authorisation code
/// - **Special characters**: Limited to safe printable characters
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("35x")]
pub struct Field25 {
    /// Authorisation information (up to 35 characters)
    #[format("35x")]
    pub authorisation: String,
}

impl Field25 {
    /// Create a new Field25 with specified authorisation
    pub fn new(authorisation: String) -> Self {
        Self { authorisation }
    }

    /// Create an empty Field25 (no authorisation)
    pub fn empty() -> Self {
        Self {
            authorisation: String::new(),
        }
    }

    /// Get the authorisation information
    pub fn authorisation(&self) -> &str {
        &self.authorisation
    }

    /// Check if the authorisation is empty
    pub fn is_empty(&self) -> bool {
        self.authorisation.is_empty()
    }

    /// Get the length of the authorisation information
    pub fn length(&self) -> usize {
        self.authorisation.len()
    }

    /// Check if the authorisation contains a specific pattern
    pub fn contains(&self, pattern: &str) -> bool {
        self.authorisation.contains(pattern)
    }

    /// Check if the authorisation starts with a specific prefix
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.authorisation.starts_with(prefix)
    }

    /// Check if the authorisation ends with a specific suffix
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.authorisation.ends_with(suffix)
    }
}
