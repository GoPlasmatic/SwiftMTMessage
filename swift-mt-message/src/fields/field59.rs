use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 59F: Beneficiary Customer (Option F)
///
/// Beneficiary customer with party identifier and name/address.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field59F {
    /// Party identifier
    #[component("[/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address lines
    #[component("4*(1!n/33x)")]
    pub name_and_address: Vec<String>,
}

/// Field 59A: Beneficiary Customer (Option A)
///
/// Beneficiary customer with BIC-based identification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field59A {
    /// Account number (optional)
    #[component("[/34x]")]
    pub account_number: Option<String>,
    /// BIC code
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field59NoOption {
    /// Account number (optional)
    #[component("[/34x]")]
    pub account: Option<String>,

    /// Name and address lines
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

/// Field 59: Beneficiary Customer
///
/// Multi-option field with different format specifications per option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field59 {
    /// Option A: BIC-based identification
    A(Field59A),
    /// Option F: Party identifier with name/address
    F(Field59F),
    /// No option: Name and address only
    NoOption(Field59NoOption),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field59Debtor {
    A(Field59A),
    NoOption(Field59NoOption),
}
