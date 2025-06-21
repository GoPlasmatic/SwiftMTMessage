use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Generic BIC Field
///
/// Used for institution fields with BIC code and optional account information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct GenericBicField {
    /// BIC code (8 or 11 characters)
    #[component("4!a2!a2!c[3!c]", validate = ["bic"])]
    pub bic: BIC,
    /// Account number (optional)
    #[component("35x", optional, validate = ["account_format"])]
    pub account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BIC {
    pub raw: String,
    pub bank_code: String,
    pub country_code: String,
    pub location_code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
}

impl std::fmt::Display for BIC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl std::str::FromStr for BIC {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 8 || s.len() > 11 {
            return Err("BIC must be 8 or 11 characters".to_string());
        }

        Ok(BIC {
            raw: s.to_string(),
            bank_code: s[0..4].to_string(),
            country_code: s[4..6].to_string(),
            location_code: s[6..8].to_string(),
            branch_code: if s.len() == 11 {
                Some(s[8..11].to_string())
            } else {
                None
            },
        })
    }
}
