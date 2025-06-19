use proc_macro::TokenStream;
use syn::{Attribute, Data, DeriveInput, Fields, Meta, parse_macro_input};
use quote::quote;

use crate::utils::extract_field_attribute;

/// Attribute macro for adding serde rename attributes based on field tags
pub fn swift_serde_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    match &mut input.data {
        Data::Struct(data) => {
            match &mut data.fields {
                Fields::Named(fields) => {
                    // Add serde rename attributes to each field based on their field tag
                    for field in &mut fields.named {
                        let field_tag = extract_field_attribute(&field.attrs)
                            .expect("All fields must have #[field(\"tag\")]");

                        // Check if serde rename already exists
                        let has_serde_rename = field.attrs.iter().any(|attr| {
                            if attr.path().is_ident("serde") {
                                if let Meta::List(meta_list) = &attr.meta {
                                    return meta_list.tokens.to_string().contains("rename");
                                }
                            }
                            false
                        });

                        // Add serde rename attribute if it doesn't exist
                        if !has_serde_rename {
                            let serde_rename: Attribute = syn::parse_quote! {
                                #[serde(rename = #field_tag)]
                            };
                            field.attrs.push(serde_rename);
                        }
                    }

                    TokenStream::from(quote! { #input })
                }
                _ => {
                    panic!("swift_serde can only be applied to structs with named fields");
                }
            }
        }
        _ => {
            panic!("swift_serde can only be applied to structs");
        }
    }
} 