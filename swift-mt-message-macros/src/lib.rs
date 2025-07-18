//! # Swift MT Message Macros - Clean Rewrite
//!
//! This crate provides derive macros for generating SWIFT MT message parsing and serialization code.
//! The implementation focuses on clean architecture, robust error handling, and high performance.

use proc_macro::TokenStream;

// Module declarations
mod ast;
mod codegen;
mod error;
mod format;
mod utils;

use error::MacroError;
use utils::serde_attributes::add_serde_attributes_to_optional_fields;

/// Derive macro for SwiftField trait implementation
///
/// Generates parsing, serialization, and validation code for SWIFT field structures.
/// Supports component-based field definitions with format specifications.
///
/// # Example
/// ```rust
/// #[derive(SwiftField)]
/// struct Field20 {
///     #[component("16x")]
///     reference: String,
/// }
/// ```
#[proc_macro_derive(SwiftField, attributes(component))]
pub fn derive_swift_field(input: TokenStream) -> TokenStream {
    match derive_swift_field_impl(input) {
        Ok(result) => result,
        Err(error) => error.to_compile_error().into(),
    }
}

/// Derive macro for SwiftMessage trait implementation
///
/// Generates message-level parsing, validation, and field management code.
/// Supports field specifications and validation rules for complete SWIFT message types.
///
/// # Example
/// ```rust
/// #[derive(SwiftMessage)]
/// struct MT103 {
///     #[field("20")]
///     field_20: Field20,
///     
///     #[field("32A")]
///     field_32a: Field32A,
/// }
/// ```
#[proc_macro_derive(SwiftMessage, attributes(field, sequence, validation_rules))]
pub fn derive_swift_message(input: TokenStream) -> TokenStream {
    match derive_swift_message_impl(input) {
        Ok(result) => result,
        Err(error) => error.to_compile_error().into(),
    }
}

/// Attribute macro for automatic serde field generation
///
/// Automatically adds appropriate serde attributes based on field configurations
/// for clean JSON serialization without enum wrappers.
#[proc_macro_attribute]
pub fn serde_swift_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    match serde_swift_fields_impl(input) {
        Ok(result) => result,
        Err(error) => error.to_compile_error().into(),
    }
}



/// Internal implementation for SwiftField derive macro
fn derive_swift_field_impl(input: TokenStream) -> Result<TokenStream, MacroError> {
    let mut input: syn::DeriveInput = syn::parse(input)?;

    // Add serde attributes to optional fields
    add_serde_attributes_to_optional_fields(&mut input)?;

    let definition = ast::FieldDefinition::parse(&input)?;
    let tokens = codegen::field::generate_swift_field_impl(&definition)?;
    Ok(tokens.into())
}

/// Internal implementation for SwiftMessage derive macro
fn derive_swift_message_impl(input: TokenStream) -> Result<TokenStream, MacroError> {
    let input = syn::parse(input)?;
    let definition = ast::MessageDefinition::parse(&input)?;
    let tokens = codegen::message::generate_swift_message_impl(&definition)?;
    Ok(tokens.into())
}

/// Internal implementation for serde_swift_fields attribute macro
fn serde_swift_fields_impl(input: TokenStream) -> Result<TokenStream, MacroError> {
    let input = syn::parse(input)?;
    let tokens = codegen::serde::generate_serde_attributes(&input)?;
    Ok(tokens.into())
}

#[cfg(test)]
mod tests {
    // Note: We can't test procedural macro generation in unit tests
    // The actual testing should be done through integration tests
    // where the macro is used in a proper procedural macro environment.
}
