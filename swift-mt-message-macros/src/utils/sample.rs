//! Sample generation utilities for procedural macro generation
//!
//! This module provides centralized sample generation functions to avoid code duplication
//! across the macro implementation. These functions help generate valid sample data
//! for testing and documentation purposes.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::utils::types::*;

/// Generate a sample value expression for a given type
#[allow(dead_code)]
pub fn generate_sample_value(ty: &Type) -> TokenStream {
    if is_string_type(ty) {
        quote! { "SAMPLE".to_string() }
    } else if is_naive_date_type(ty) {
        quote! { chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap() }
    } else if is_naive_time_type(ty) {
        quote! { chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap() }
    } else if is_f64_type(ty) {
        quote! { 100.0 }
    } else if is_u32_type(ty) {
        quote! { 123 }
    } else if is_u8_type(ty) {
        quote! { 1 }
    } else if is_bool_type(ty) {
        quote! { true }
    } else if is_char_type(ty) {
        quote! { 'A' }
    } else if is_option_type(ty) {
        if let Some(inner_ty) = extract_generic_inner_type(ty) {
            let inner_sample = generate_sample_value(&inner_ty);
            quote! { Some(#inner_sample) }
        } else {
            quote! { None }
        }
    } else if is_vec_type(ty) {
        if let Some(inner_ty) = extract_generic_inner_type(ty) {
            let inner_sample = generate_sample_value(&inner_ty);
            quote! { vec![#inner_sample] }
        } else {
            quote! { vec![] }
        }
    } else {
        // For field types, try to call their sample method
        quote! { #ty::sample() }
    }
}

/// Generate a minimal sample value (for optional fields, use None)
#[allow(dead_code)]
pub fn generate_minimal_sample_value(ty: &Type) -> TokenStream {
    if is_option_type(ty) {
        quote! { None }
    } else if is_vec_type(ty) {
        quote! { vec![] }
    } else {
        generate_sample_value(ty)
    }
}

/// Generate a full sample value (for optional fields, use Some(value))
#[allow(dead_code)]
pub fn generate_full_sample_value(ty: &Type) -> TokenStream {
    if is_option_type(ty) {
        if let Some(inner_ty) = extract_generic_inner_type(ty) {
            let inner_sample = generate_sample_value(&inner_ty);
            quote! { Some(#inner_sample) }
        } else {
            quote! { None }
        }
    } else if is_vec_type(ty) {
        if let Some(inner_ty) = extract_generic_inner_type(ty) {
            let inner_sample = generate_sample_value(&inner_ty);
            quote! { vec![#inner_sample, #inner_sample] }
        } else {
            quote! { vec![] }
        }
    } else {
        generate_sample_value(ty)
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