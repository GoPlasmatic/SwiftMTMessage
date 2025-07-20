//! Sample generation utilities for procedural macro generation
//!
//! This module provides centralized sample generation functions to avoid code duplication
//! across the macro implementation. These functions help generate valid sample data
//! for testing and documentation purposes.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::utils::types::{categorize_type, extract_generic_inner_type, TypeCategory};

/// Generate a sample value expression for a given type
#[allow(dead_code)]
pub fn generate_sample_value(ty: &Type) -> TokenStream {
    match categorize_type(ty) {
        TypeCategory::String => quote! { "SAMPLE".to_string() },
        TypeCategory::NaiveDate => quote! { chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap() },
        TypeCategory::NaiveTime => quote! { chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap() },
        TypeCategory::F64 => quote! { 100.0 },
        TypeCategory::U32 => quote! { 123 },
        TypeCategory::U8 => quote! { 1 },
        TypeCategory::Bool => quote! { true },
        TypeCategory::Char => quote! { 'A' },
        TypeCategory::OptionString
        | TypeCategory::OptionNaiveDate
        | TypeCategory::OptionNaiveTime
        | TypeCategory::OptionF64
        | TypeCategory::OptionU32
        | TypeCategory::OptionU8
        | TypeCategory::OptionBool
        | TypeCategory::OptionChar
        | TypeCategory::OptionField => {
            if let Some(inner_ty) = extract_generic_inner_type(ty) {
                let inner_sample = generate_sample_value(&inner_ty);
                quote! { Some(#inner_sample) }
            } else {
                quote! { None }
            }
        }
        TypeCategory::Vec | TypeCategory::VecString => {
            if let Some(inner_ty) = extract_generic_inner_type(ty) {
                let inner_sample = generate_sample_value(&inner_ty);
                quote! { vec![#inner_sample] }
            } else {
                quote! { vec![] }
            }
        }
        TypeCategory::Field => {
            // For field types, try to call their sample method
            quote! { #ty::sample() }
        }
        _ => {
            // Default for unknown types
            quote! { #ty::sample() }
        }
    }
}

/// Generate a minimal sample value (for optional fields, use None)
#[allow(dead_code)]
pub fn generate_minimal_sample_value(ty: &Type) -> TokenStream {
    match categorize_type(ty) {
        TypeCategory::OptionString
        | TypeCategory::OptionNaiveDate
        | TypeCategory::OptionNaiveTime
        | TypeCategory::OptionF64
        | TypeCategory::OptionU32
        | TypeCategory::OptionU8
        | TypeCategory::OptionBool
        | TypeCategory::OptionChar
        | TypeCategory::OptionField
        | TypeCategory::OptionVec => quote! { None },
        TypeCategory::Vec | TypeCategory::VecString => quote! { vec![] },
        _ => generate_sample_value(ty),
    }
}

/// Generate a full sample value (for optional fields, use Some(value))
#[allow(dead_code)]
pub fn generate_full_sample_value(ty: &Type) -> TokenStream {
    match categorize_type(ty) {
        TypeCategory::OptionString
        | TypeCategory::OptionNaiveDate
        | TypeCategory::OptionNaiveTime
        | TypeCategory::OptionF64
        | TypeCategory::OptionU32
        | TypeCategory::OptionU8
        | TypeCategory::OptionBool
        | TypeCategory::OptionChar
        | TypeCategory::OptionField => {
            if let Some(inner_ty) = extract_generic_inner_type(ty) {
                let inner_sample = generate_sample_value(&inner_ty);
                quote! { Some(#inner_sample) }
            } else {
                quote! { None }
            }
        }
        TypeCategory::Vec | TypeCategory::VecString => {
            if let Some(inner_ty) = extract_generic_inner_type(ty) {
                let inner_sample = generate_sample_value(&inner_ty);
                quote! { vec![#inner_sample, #inner_sample] }
            } else {
                quote! { vec![] }
            }
        }
        _ => generate_sample_value(ty),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_basic_sample_generation() {
        let string_type: Type = parse_quote!(String);
        let sample = generate_sample_value(&string_type);
        assert!(sample.to_string().contains("SAMPLE"));
    }

    #[test]
    fn test_option_sample_generation() {
        let option_string: Type = parse_quote!(Option<String>);
        let sample = generate_sample_value(&option_string);
        assert!(sample.to_string().contains("Some"));

        let minimal = generate_minimal_sample_value(&option_string);
        assert!(minimal.to_string().contains("None"));
    }

    #[test]
    fn test_vec_sample_generation() {
        let vec_string: Type = parse_quote!(Vec<String>);
        let sample = generate_sample_value(&vec_string);
        let sample_str = sample.to_string();
        assert!(sample_str.contains("vec") && sample_str.contains("!"));
    }
}
