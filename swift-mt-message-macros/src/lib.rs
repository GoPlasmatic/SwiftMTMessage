use proc_macro::TokenStream;

// Module declarations
mod component;
mod format;
mod utils;
mod business;
mod derive;
mod serde;

/// Derive macro for SwiftField trait implementation
#[proc_macro_derive(
    SwiftField,
    attributes(
        format,
        field_option,
        component,
        validation_rules,
        business_logic,
        preserve_raw
    )
)]
pub fn derive_swift_field(input: TokenStream) -> TokenStream {
    derive::field::derive_swift_field_impl(input)
}

/// Attribute macro for adding serde rename attributes based on field tags
#[proc_macro_attribute]
pub fn swift_serde(args: TokenStream, input: TokenStream) -> TokenStream {
    serde::swift_serde_impl(args, input)
}

/// Derive macro for SwiftMessage trait implementation
#[proc_macro_derive(SwiftMessage, attributes(swift_message, field))]
pub fn derive_swift_message(input: TokenStream) -> TokenStream {
    derive::message::derive_swift_message_impl(input)
}
