use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 59F: Beneficiary Customer (Option F)
///
/// Beneficiary customer with party identifier and name/address.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field59F {
    /// Party identifier
    #[component("34x", validate = ["party_identifier_format"])]
    pub party_identifier: String,
    /// Name and address lines
    #[component("4*35x", validate = ["line_count", "line_length", "structured_address"])]
    pub name_and_address: Vec<String>,
}

/// Field 59A: Beneficiary Customer (Option A)
///
/// Beneficiary customer with BIC-based identification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field59A {
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

/// Field 59: Beneficiary Customer
///
/// Multi-option field with different format specifications per option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field59 {
    /// Option A: BIC-based identification
    A(Field59A),
    /// Option F: Party identifier with name/address
    F(Field59F),
    /// No option: Name and address only
    NoOption(Field59Basic),
}

/// Type alias for Field 59 Basic: Beneficiary Customer (Basic)
pub type Field59Basic = crate::fields::common::GenericMultiLine4x35;

impl crate::SwiftField for Field59 {
    fn parse(value: &str) -> crate::Result<Self> {
        let content = value.trim();

        // Determine option based on format pattern
        if content.contains('\n') || content.lines().count() > 1 {
            // Option F or NoOption - multiple lines
            if content.starts_with('/') {
                // Option F - has party identifier
                let field_59f = Field59F::parse(value)?;
                Ok(Field59::F(field_59f))
            } else {
                // NoOption - name and address only
                let field_59_basic = Field59Basic::parse(value)?;
                Ok(Field59::NoOption(field_59_basic))
            }
        } else {
            // Option A - BIC format
            let field_59a = Field59A::parse(value)?;
            Ok(Field59::A(field_59a))
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field59::A(field_59a) => field_59a.to_swift_string(),
            Field59::F(field_59f) => field_59f.to_swift_string(),
            Field59::NoOption(field_59_basic) => field_59_basic.to_swift_string(),
        }
    }

    fn validate(&self) -> crate::ValidationResult {
        match self {
            Field59::A(field_59a) => field_59a.validate(),
            Field59::F(field_59f) => field_59f.validate(),
            Field59::NoOption(field_59_basic) => field_59_basic.validate(),
        }
    }

    fn format_spec() -> &'static str {
        "multi_option"
    }
}
