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
mod slash_handler_utils;
mod utils;

use error::MacroError;
use utils::serde_attributes::add_serde_attributes_to_optional_fields;

/// Derive macro for SwiftField trait implementation
///
/// Generates parsing, serialization, and validation code for SWIFT field structures.
/// Supports component-based field definitions with format specifications.
///
/// ## Basic Usage
///
/// ### Simple Field
/// ```logic
/// use swift_mt_message_macros::SwiftField;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
/// struct Field20 {
///     #[component("16x")]
///     reference: String,
/// }
/// ```
///
/// ### Multi-Component Field
/// ```logic
/// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
/// struct Field32A {
///     #[component("6!n")]     // Date: YYMMDD
///     date: String,
///     #[component("3!a")]     // Currency: ISO code
///     currency: String,
///     #[component("15d")]     // Amount: decimal
///     amount: f64,
/// }
/// ```
///
/// ### Enum Field (Multiple Variants)
/// ```logic
/// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
/// enum Field50 {
///     A(Field50A),
///     F(Field50F),
///     K(Field50K),
/// }
/// ```
///
/// ## Format Specifications
///
/// The `#[component("format")]` attribute supports SWIFT format specifications:
///
/// - **Fixed length**: `3!a` (exactly 3 alphabetic chars)
/// - **Variable length**: `35x` (up to 35 any chars)
/// - **Optional**: `[/34x]` (optional account with slash prefix)
/// - **Repetitive**: `4*35x` (up to 4 lines of 35 chars each)
/// - **Decimal**: `15d` (decimal number up to 15 digits)
///
/// ## Generated Methods
///
/// The macro generates these methods for the SwiftField trait:
/// - `parse(value: &str) -> Result<Self>` - Parse from SWIFT format string
/// - `to_swift_string(&self) -> String` - Convert to SWIFT format string
/// - `format_spec() -> &'static str` - Return format specification
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
/// ## Basic Usage
///
/// ```logic
/// use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};
/// use serde::{Deserialize, Serialize};
/// use crate::fields::*;
///
/// #[serde_swift_fields]
/// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
/// struct MT103 {
///     // Mandatory fields (no Option wrapper)
///     #[field("20")]
///     field_20: Field20,
///     
///     #[field("32A")]
///     field_32a: Field32A,
///     
///     // Optional fields (wrapped in Option)
///     #[field("50A")]
///     field_50a: Option<Field50A>,
///     
///     // Repetitive fields (wrapped in Vec)
///     #[field("71A")]
///     field_71a: Option<Vec<Field71A>>,
/// }
/// ```
///
/// ## Field Attributes
///
/// The `#[field("tag")]` attribute maps struct fields to SWIFT field tags:
/// - `#[field("20")]` - Maps to SWIFT field 20
/// - `#[field("32A")]` - Maps to SWIFT field 32A with option
/// - Field tags must match the SWIFT standard exactly
///
/// ## Field Types
///
/// - **Mandatory**: `field_20: Field20` - Required field
/// - **Optional**: `field_50: Option<Field50>` - Optional field  
/// - **Repetitive**: `field_71a: Vec<Field71A>` - Multiple occurrences
/// - **Optional Repetitive**: `field_71a: Option<Vec<Field71A>>` - Optional multiple
///
/// ## Generated Methods
///
/// The macro generates these methods for the SwiftMessageBody trait:
/// - `message_type() -> &'static str` - Returns message type (e.g., "103")
/// - `from_fields(fields: HashMap<String, Vec<String>>) -> Result<Self>` - Parse from field map
/// - `to_fields(&self) -> HashMap<String, Vec<String>>` - Convert to field map
/// - `required_fields() -> Vec<&'static str>` - List required field tags
/// - `optional_fields() -> Vec<&'static str>` - List optional field tags
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
///
/// ## Usage
///
/// Apply this attribute to message structs before the derive attributes:
///
/// ```logic
/// use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};
/// use serde::{Deserialize, Serialize};
///
/// #[serde_swift_fields]  // Apply this first
/// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
/// struct MT103 {
///     #[field("20")]
///     field_20: Field20,
/// }
/// ```
///
/// This macro automatically:
/// - Adds `#[serde(skip_serializing_if = "Option::is_none")]` to optional fields
/// - Configures proper field naming for JSON output
/// - Ensures clean serialization without enum variant wrappers
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
