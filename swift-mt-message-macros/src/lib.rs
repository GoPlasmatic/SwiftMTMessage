use proc_macro::TokenStream;

// Module declarations
mod component;
mod derive;
mod serde;
mod utils;

use derive::field::derive_swift_field_impl;
use derive::message::derive_swift_message_impl;

/// Derive macro for SwiftField trait implementation
///
/// Supports the component-based approach for defining SWIFT field structures
/// with validation attributes and format specifications.
#[proc_macro_derive(SwiftField, attributes(component))]
pub fn derive_swift_field(input: TokenStream) -> TokenStream {
    derive_swift_field_impl(input)
}

/// Derive macro for SwiftMessage trait implementation
///
/// Supports message-level business validation with field specifications
/// and validation rules for complete SWIFT message types.
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
