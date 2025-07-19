//! Serde attribute management utilities
//!
//! This module provides functions for automatically adding appropriate serde attributes
//! to structs and enums for clean JSON serialization without enum wrappers.

use crate::error::MacroError;
use crate::utils::types::{is_option_type, is_vec_type};
use syn::DeriveInput;

/// Add serde attributes to optional and vector fields, and enum flattening
pub fn add_serde_attributes_to_optional_fields(input: &mut DeriveInput) -> Result<(), MacroError> {
    // Handle enum types - add flattening and content-based serialization
    if let syn::Data::Enum(_) = input.data {
        // Check if it already has serde attributes for enums
        let has_enum_serde_attr = input.attrs.iter().any(|attr| {
            if attr.path().is_ident("serde") {
                if let Ok(tokens) = attr.parse_args::<proc_macro2::TokenStream>() {
                    let tokens_str = tokens.to_string();
                    return tokens_str.contains("tag")
                        || tokens_str.contains("content")
                        || tokens_str.contains("untagged");
                }
            }
            false
        });

        if !has_enum_serde_attr {
            // Add serde(untagged) to enum for cleaner JSON without variant wrappers
            let enum_serde_attr = syn::parse_quote! {
                #[serde(untagged)]
            };
            input.attrs.push(enum_serde_attr);
        }
        return Ok(());
    }

    // Handle struct types
    if let syn::Data::Struct(ref mut data_struct) = input.data {
        if let syn::Fields::Named(ref mut fields) = data_struct.fields {
            for field in &mut fields.named {
                // Check if it already has a serde skip_serializing_if attribute
                let has_skip_attr = field.attrs.iter().any(|attr| {
                    if attr.path().is_ident("serde") {
                        if let Ok(tokens) = attr.parse_args::<proc_macro2::TokenStream>() {
                            return tokens.to_string().contains("skip_serializing_if");
                        }
                    }
                    false
                });

                // Skip if attribute already exists
                if has_skip_attr {
                    continue;
                }

                // Check if field type is Option<T>
                if is_option_type(&field.ty) {
                    let skip_attr = syn::parse_quote! {
                        #[serde(skip_serializing_if = "Option::is_none")]
                    };
                    field.attrs.push(skip_attr);
                }
                // Check if field type is Vec<T>
                else if is_vec_type(&field.ty) {
                    let skip_attr = syn::parse_quote! {
                        #[serde(skip_serializing_if = "Vec::is_empty")]
                    };
                    field.attrs.push(skip_attr);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_add_serde_attributes_to_optional_fields() {
        let mut input: DeriveInput = parse_quote! {
            struct TestStruct {
                required_field: String,
                optional_field: Option<String>,
                vec_field: Vec<String>,
            }
        };

        add_serde_attributes_to_optional_fields(&mut input).unwrap();

        // Check that serde attributes were added to optional and vec fields
        if let syn::Data::Struct(data_struct) = &input.data {
            if let syn::Fields::Named(fields) = &data_struct.fields {
                let fields: Vec<_> = fields.named.iter().collect();

                // Required field should have no serde attributes
                assert!(fields[0].attrs.is_empty());

                // Optional field should have skip_serializing_if attribute
                assert!(fields[1].attrs.iter().any(|attr| {
                    attr.path().is_ident("serde")
                        && attr
                            .parse_args::<proc_macro2::TokenStream>()
                            .unwrap()
                            .to_string()
                            .contains("skip_serializing_if")
                }));

                // Vec field should have skip_serializing_if attribute
                assert!(fields[2].attrs.iter().any(|attr| {
                    attr.path().is_ident("serde")
                        && attr
                            .parse_args::<proc_macro2::TokenStream>()
                            .unwrap()
                            .to_string()
                            .contains("skip_serializing_if")
                }));
            }
        }
    }

    #[test]
    fn test_add_serde_attributes_to_enum() {
        let mut input: DeriveInput = parse_quote! {
            enum TestEnum {
                A(String),
                B(u32),
            }
        };

        add_serde_attributes_to_optional_fields(&mut input).unwrap();

        // Check that untagged attribute was added
        assert!(input.attrs.iter().any(|attr| {
            attr.path().is_ident("serde")
                && attr
                    .parse_args::<proc_macro2::TokenStream>()
                    .unwrap()
                    .to_string()
                    .contains("untagged")
        }));
    }
}
