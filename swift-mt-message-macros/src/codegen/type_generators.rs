//! Type-specific code generators for field conversion and serialization
//!
//! This module provides specialized code generators for different field types,
//! eliminating code duplication and improving maintainability.

use crate::ast::Component;
use crate::format::FormatType;
use crate::utils::types::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

/// Generate to_swift_string code for a specific component based on its type
pub fn generate_to_swift_string_for_component(component: &Component) -> TokenStream {
    let field_name = &component.name;
    let field_type = &component.field_type;
    
    // Handle special format patterns that require delimiters
    match component.format.pattern.as_str() {
        "/8c/" => {
            // For /8c/ format, wrap the value in slashes
            quote! {
                format!("/{}/", self.#field_name)
            }
        }
        _ => {
            generate_to_swift_string_for_type(field_name, field_type)
        }
    }
}

/// Generate to_swift_string code for a field with a specific type
pub fn generate_to_swift_string_for_type(field_name: &syn::Ident, field_type: &Type) -> TokenStream {
    // Generate conversion based on field type
    if is_naive_date_type(field_type) {
        quote! {
            self.#field_name.format("%y%m%d").to_string()
        }
    } else if is_naive_time_type(field_type) {
        quote! {
            self.#field_name.format("%H%M").to_string()
        }
    } else if is_char_type(field_type) {
        quote! {
            self.#field_name.to_string()
        }
    } else if is_f64_type(field_type) {
        quote! {
            format!("{:.2}", self.#field_name).replace('.', ",")
        }
    } else if is_u32_type(field_type) || is_u8_type(field_type) {
        quote! {
            self.#field_name.to_string()
        }
    } else if is_bool_type(field_type) {
        quote! {
            if self.#field_name { "true".to_string() } else { "false".to_string() }
        }
    } else if is_option_naive_date_type(field_type) {
        quote! {
            self.#field_name.as_ref()
                .map(|d| d.format("%y%m%d").to_string())
                .unwrap_or_default()
        }
    } else if is_option_string_type(field_type) {
        quote! {
            self.#field_name.as_ref().unwrap_or(&String::new()).clone()
        }
    } else if is_option_u32_type(field_type) || is_option_u8_type(field_type) {
        quote! {
            self.#field_name.map(|n| n.to_string()).unwrap_or_default()
        }
    } else if is_option_bool_type(field_type) {
        quote! {
            self.#field_name.map(|b| if b { "true".to_string() } else { "false".to_string() }).unwrap_or_default()
        }
    } else if is_option_char_type(field_type) {
        quote! {
            self.#field_name.map(|c| c.to_string()).unwrap_or_default()
        }
    } else if is_vec_string_type(field_type) {
        quote! {
            self.#field_name.join("")
        }
    } else if is_vec_type(field_type) {
        quote! {
            self.#field_name.iter()
                .map(|item| item.to_swift_string())
                .collect::<Vec<_>>()
                .join("")
        }
    } else if is_option_field_type(field_type) {
        quote! {
            self.#field_name.as_ref()
                .map(|item| item.to_swift_string())
                .unwrap_or_default()
        }
    } else if is_field_type(field_type) {
        quote! {
            self.#field_name.to_swift_string()
        }
    } else if is_string_type(field_type) {
        quote! {
            self.#field_name.clone()
        }
    } else {
        // Default to calling to_swift_string for unknown types
        quote! {
            self.#field_name.to_swift_string()
        }
    }
}

/// Generate sample code for a specific component based on its type
pub fn generate_sample_for_component(component: &Component) -> TokenStream {
    let field_name = &component.name;
    let sample_expr = generate_sample_expr_for_component(component);
    
    quote! {
        #field_name: #sample_expr
    }
}

/// Generate sample expression for a specific component
pub fn generate_sample_expr_for_component(component: &Component) -> TokenStream {
    let field_name = &component.name;
    let field_type = &component.field_type;
    let format_spec = &component.format;
    
    // Handle special cases based on field name and format
    if is_string_type(field_type) {
        match (field_name.to_string().as_str(), format_spec.pattern.as_str()) {
            ("offset", "4!n") => {
                // For offset fields, use "0000" 
                return quote! { "0000".to_string() };
            }
            ("time", "4!n") => {
                // For time fields, use "1200"
                return quote! { "1200".to_string() };
            }
            _ => {}
        }
    }
    
    // Fall back to the original logic
    generate_sample_expr_for_type(field_type, format_spec)
}

/// Generate sample expression for a field type
pub fn generate_sample_expr_for_type(field_type: &Type, format_spec: &crate::format::FormatSpec) -> TokenStream {
    
    // Generate sample code based on the actual field type
    if is_naive_date_type(field_type) {
        quote! {
            chrono::NaiveDate::from_ymd_opt(2023, 1, 15).unwrap()
        }
    } else if is_naive_time_type(field_type) {
        quote! {
            chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap()
        }
    } else if is_char_type(field_type) {
        quote! { 'D' }
    } else if is_f64_type(field_type) {
        quote! { 1234.56 }
    } else if is_u32_type(field_type) {
        quote! { 12345u32 }
    } else if is_u8_type(field_type) {
        quote! { 42u8 }
    } else if is_bool_type(field_type) {
        quote! { true }
    } else if is_option_naive_date_type(field_type) {
        quote! {
            Some(chrono::NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
        }
    } else if is_option_string_type(field_type) {
        // Generate sample string based on format
        let sample_string = generate_sample_string_literal(format_spec);
        quote! { Some(#sample_string.to_string()) }
    } else if is_option_u32_type(field_type) {
        quote! { Some(67890u32) }
    } else if is_option_u8_type(field_type) {
        quote! { Some(99u8) }
    } else if is_option_bool_type(field_type) {
        quote! { Some(false) }
    } else if is_option_char_type(field_type) {
        quote! { Some('X') }
    } else if is_vec_string_type(field_type) {
        // Generate sample Vec<String>
        quote! { vec!["SAMPLE".to_string()] }
    } else if is_vec_type(field_type) {
        // Generate empty Vec for now - should be populated based on content
        quote! { Vec::new() }
    } else if is_option_field_type(field_type) {
        // Generate Some with sample for optional field types
        if let syn::Type::Path(type_path) = field_type {
            if let Some(segment) = type_path.path.segments.last() {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return quote! { Some(<#inner_ty as crate::SwiftField>::sample()) };
                    }
                }
            }
        }
        quote! { None }
    } else if is_field_type(field_type) {
        // Generate sample for field types
        quote! { <#field_type as crate::SwiftField>::sample() }
    } else {
        // Default to String
        let sample_string = generate_sample_string_literal(format_spec);
        quote! { #sample_string.to_string() }
    }
}

/// Generate a sample string literal based on format spec
fn generate_sample_string_literal(format_spec: &crate::format::FormatSpec) -> String {
    // Handle special cases for complex format patterns
    match format_spec.pattern.as_str() {
        "/8c/" => {
            // Time indication codes commonly used in SWIFT
            "SNDTIME".to_string()
        }
        "1!x" => {
            // Sign characters - default to positive
            "+".to_string()
        }
        _ => {
            // Default behavior for other patterns
            match &format_spec.format_type {
                FormatType::AnyCharacter => {
                    let length = format_spec.length.unwrap_or(16);
                    "A".repeat(length)
                }
                FormatType::Numeric => {
                    let length = format_spec.length.unwrap_or(6);
                    match length {
                        4 => "1200".to_string(), // Time format HHMM or offset
                        6 => "123456".to_string(), // Date format YYMMDD
                        _ => "1".repeat(length),
                    }
                }
                FormatType::Alpha => {
                    let length = format_spec.length.unwrap_or(4);
                    "TEST".chars().cycle().take(length).collect()
                }
                FormatType::CharacterSet => {
                    let length = format_spec.length.unwrap_or(4);
                    "ABC1".chars().cycle().take(length).collect()
                }
                _ => {
                    let length = format_spec.length.unwrap_or(8);
                    "SAMPLE".chars().cycle().take(length).collect()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::FormatSpec;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_generate_to_swift_string_for_string_type() {
        let field_name: syn::Ident = parse_quote!(test_field);
        let field_type: syn::Type = parse_quote!(String);
        
        let result = generate_to_swift_string_for_type(&field_name, &field_type);
        let expected = quote! {
            self.test_field.clone()
        };
        
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_to_swift_string_for_option_string_type() {
        let field_name: syn::Ident = parse_quote!(test_field);
        let field_type: syn::Type = parse_quote!(Option<String>);
        
        let result = generate_to_swift_string_for_type(&field_name, &field_type);
        let expected = quote! {
            self.test_field.as_ref().unwrap_or(&String::new()).clone()
        };
        
        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_sample_expr_for_string_type() {
        let field_type: syn::Type = parse_quote!(String);
        let format_spec = FormatSpec::parse("4!a").unwrap();
        
        let result = generate_sample_expr_for_type(&field_type, &format_spec);
        let expected = quote! {
            "TEST".to_string()
        };
        
        assert_eq!(result.to_string(), expected.to_string());
    }
}