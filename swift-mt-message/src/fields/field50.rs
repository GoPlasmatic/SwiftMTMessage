use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 50F: Ordering Customer (Option F)
///
/// Ordering customer with party identifier and name/address.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50F {
    /// Party identifier
    #[component("34x", validate = ["party_identifier_format"])]
    pub party_identifier: String,
    /// Name and address lines
    #[component("4*35x", validate = ["line_count", "line_length", "structured_address"])]
    pub name_and_address: Vec<String>,
}

/// Field 50A: Ordering Customer (Option A)
///
/// Ordering customer with BIC-based identification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50A {
    /// Account line indicator (optional)
    #[component("[1!a]", optional)]
    pub account_line_indicator: Option<String>,
    /// Account number (optional)
    #[component("[34x]", optional)]
    pub account_number: Option<String>,
    /// BIC code
    #[component("4!a2!a2!c[3!c]", validate = ["bic"])]
    pub bic: String,
}

/// Field 50: Ordering Customer
///
/// Multi-option field with different format specifications per option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field50 {
    /// Option A: BIC-based identification
    A(Field50A),
    /// Option F: Party identifier with name/address
    F(Field50F),
    /// Option K: Name and address only
    K(Field50K),
}

/// Type alias for Field 50K: Ordering Customer (Option K)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50K {
    /// Account number (optional)
    #[component("[34x]", optional)]
    pub account_number: Option<String>,

    /// Name and address lines
    #[component("4*35x", validate = ["line_count", "line_length", "structured_address"])]
    pub name_and_address: Vec<String>,
}

impl crate::SwiftField for Field50 {
    fn parse(value: &str) -> crate::Result<Self> {
        let content = value.trim();

        // Determine option based on format pattern
        if content.contains('\n') || content.lines().count() > 1 {
            // Option K or F - multiple lines
            if content.starts_with('/') {
                // Option F - has party identifier
                let field_50f = Field50F::parse(value)?;
                Ok(Field50::F(field_50f))
            } else {
                // Option K - name and address only
                let field_50k = Field50K::parse(value)?;
                Ok(Field50::K(field_50k))
            }
        } else {
            // Option A - BIC format
            let field_50a = Field50A::parse(value)?;
            Ok(Field50::A(field_50a))
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field50::A(field_50a) => field_50a.to_swift_string(),
            Field50::F(field_50f) => field_50f.to_swift_string(),
            Field50::K(field_50k) => field_50k.to_swift_string(),
        }
    }

    fn validate(&self) -> crate::ValidationResult {
        match self {
            Field50::A(field_50a) => field_50a.validate(),
            Field50::F(field_50f) => field_50f.validate(),
            Field50::K(field_50k) => field_50k.validate(),
        }
    }

    fn format_spec() -> &'static str {
        "multi_option"
    }
}
