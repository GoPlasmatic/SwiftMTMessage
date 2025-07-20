//! Attribute parsing utilities for procedural macro generation
//!
//! This module provides centralized attribute parsing functions to avoid code duplication
//! across the macro implementation. These functions help parse and extract values from
//! various attribute types used in SWIFT field and message definitions.

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Attribute, Lit, Meta};

use crate::error::{MacroError, MacroResult};

/// Extract string value from a named attribute (e.g., #[component("format")])
pub fn extract_string_attribute(attrs: &[Attribute], attr_name: &str) -> MacroResult<String> {
    for attr in attrs {
        if attr.path().is_ident(attr_name) {
            match &attr.meta {
                Meta::List(meta_list) => {
                    let tokens = &meta_list.tokens;
                    let lit: Lit = syn::parse2(tokens.clone())?;
                    match lit {
                        Lit::Str(lit_str) => {
                            return Ok(lit_str.value());
                        }
                        _ => {
                            return Err(MacroError::invalid_attribute(
                                attr.span(),
                                attr_name,
                                "non-string literal",
                                "string literal",
                            ));
                        }
                    }
                }
                _ => {
                    return Err(MacroError::invalid_attribute(
                        attr.span(),
                        attr_name,
                        "invalid syntax",
                        &format!("#[{attr_name}(\"value\")]"),
                    ));
                }
            }
        }
    }

    Err(MacroError::missing_attribute(
        Span::call_site(),
        attr_name,
        &format!("{attr_name} attribute"),
    ))
}

/// Extract format specification from #[component("format")] attribute
pub fn extract_component_attribute(attrs: &[Attribute]) -> MacroResult<String> {
    extract_string_attribute(attrs, "component")
}

/// Extract field specification from #[field("tag")] attribute
pub fn extract_field_attribute(attrs: &[Attribute]) -> MacroResult<String> {
    extract_string_attribute(attrs, "field")
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    /// Helper function to check if a specific attribute exists
    fn has_attribute(attrs: &[Attribute], attr_name: &str) -> bool {
        attrs.iter().any(|attr| attr.path().is_ident(attr_name))
    }

    /// Helper function to check if a serde attribute contains specific content
    fn has_serde_attribute_with_content(attrs: &[Attribute], content: &str) -> bool {
        attrs.iter().any(|attr| {
            if attr.path().is_ident("serde") {
                if let Ok(tokens) = attr.parse_args::<proc_macro2::TokenStream>() {
                    return tokens.to_string().contains(content);
                }
            }
            false
        })
    }

    #[test]
    fn test_extract_component_attribute() {
        let attrs: Vec<Attribute> = vec![parse_quote!(#[component("16x")])];

        let result = extract_component_attribute(&attrs);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "16x");
    }

    #[test]
    fn test_extract_field_attribute() {
        let attrs: Vec<Attribute> = vec![parse_quote!(#[field("20")])];

        let result = extract_field_attribute(&attrs);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "20");
    }

    #[test]
    fn test_has_attribute() {
        let attrs: Vec<Attribute> = vec![
            parse_quote!(#[component("16x")]),
            parse_quote!(#[serde(skip_serializing_if = "Option::is_none")]),
        ];

        assert!(has_attribute(&attrs, "component"));
        assert!(has_attribute(&attrs, "serde"));
        assert!(!has_attribute(&attrs, "field"));
    }

    #[test]
    fn test_has_serde_attribute_with_content() {
        let attrs: Vec<Attribute> =
            vec![parse_quote!(#[serde(skip_serializing_if = "Option::is_none")])];

        assert!(has_serde_attribute_with_content(
            &attrs,
            "skip_serializing_if"
        ));
        assert!(!has_serde_attribute_with_content(&attrs, "flatten"));
    }
}
