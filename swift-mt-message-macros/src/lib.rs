use proc_macro::TokenStream;

// Module declarations
mod component;
mod derive;
mod serde;
mod utils;

use derive::field::derive_swift_field_impl;
use derive::message::derive_swift_message_impl;

/// Derive macro for SwiftField trait implementation
/// Supports the new component-based approach:
/// ```rust
/// #[derive(SwiftField)]
/// pub struct Field32A {
///     #[component("6!n", validate = ["date_format", "valid_date_range"])]
///     pub value_date: NaiveDate,
///     #[component("3!a", validate = ["currency_code"])]
///     pub currency: String,
///     #[component("15d", validate = ["amount_format", "positive_amount"])]
///     pub amount: f64,
/// }
/// ```
#[proc_macro_derive(SwiftField, attributes(component))]
pub fn derive_swift_field(input: TokenStream) -> TokenStream {
    derive_swift_field_impl(input)
}

/// Derive macro for SwiftMessage trait implementation
/// Supports message-level business validation:
/// ```rust
/// #[derive(SwiftMessage)]
/// #[validation_rules(MT103_VALIDATION_RULES)]
/// pub struct MT103 {
///     #[field("20", mandatory)]
///     pub field_20: GenericReferenceField,
///     #[field("33B", optional)]
///     pub field_33b: Option<GenericCurrencyAmountField>,
/// }
/// ```
#[proc_macro_derive(SwiftMessage, attributes(field, sequence, validation_rules))]
pub fn derive_swift_message(input: TokenStream) -> TokenStream {
    derive_swift_message_impl(input)
}

/// Attribute macro for field specifications
#[proc_macro_attribute]
pub fn field(_args: TokenStream, input: TokenStream) -> TokenStream {
    // For now, this is a no-op that just preserves the field definition
    // The actual field metadata is extracted by the SwiftMessage derive macro
    input
}

/// Attribute macro for message type specifications
#[proc_macro_attribute]
pub fn swift_message(_args: TokenStream, input: TokenStream) -> TokenStream {
    // For now, this is a no-op that just preserves the struct definition
    // The actual message type metadata is extracted by the SwiftMessage derive macro
    input
}

/// Attribute macro that automatically adds serde attributes based on field configurations
/// Usage: #[serde_swift_fields] before #[derive(...)]
#[proc_macro_attribute]
pub fn serde_swift_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    serde::serde_swift_fields_impl(input)
}
