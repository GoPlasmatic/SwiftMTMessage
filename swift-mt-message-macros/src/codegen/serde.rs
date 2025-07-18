//! Serde integration for clean JSON serialization

use crate::error::MacroResult;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

/// Generate serde attributes for clean JSON serialization
/// Adds serde(rename = "field_tag") attributes to fields based on their #[field("tag")] attributes
pub fn generate_serde_attributes(input: &DeriveInput) -> MacroResult<TokenStream> {
    let mut modified_input = input.clone();

    // Process struct fields if it's a struct
    if let syn::Data::Struct(ref mut data_struct) = modified_input.data {
        if let syn::Fields::Named(ref mut fields) = data_struct.fields {
            for field in &mut fields.named {
                // Look for #[field("tag")] attributes
                if let Some(field_tag) = extract_field_tag(&field.attrs) {
                    // Check if the field is an Option type
                    if is_option_type(&field.ty) {
                        // Add serde(rename = "tag", skip_serializing_if = "Option::is_none") attribute
                        let serde_attr = syn::parse_quote! {
                            #[serde(rename = #field_tag, skip_serializing_if = "Option::is_none")]
                        };
                        field.attrs.push(serde_attr);
                    }
                    // Check if the field is a Vec type
                    else if is_vec_type(&field.ty) {
                        // Add serde(rename = "tag", skip_serializing_if = "Vec::is_empty") attribute
                        let serde_attr = syn::parse_quote! {
                            #[serde(rename = #field_tag, skip_serializing_if = "Vec::is_empty")]
                        };
                        field.attrs.push(serde_attr);
                    } else {
                        // Add serde(rename = "tag") attribute for other fields
                        let serde_rename_attr = syn::parse_quote! {
                            #[serde(rename = #field_tag)]
                        };
                        field.attrs.push(serde_rename_attr);
                    }
                }
            }
        }
    }

    // Convert the modified input to TokenStream
    let mut tokens = TokenStream::new();
    modified_input.to_tokens(&mut tokens);
    Ok(tokens)
}

/// Extract field tag from #[field("tag")] attribute
fn extract_field_tag(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("field") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                if let Ok(lit) = syn::parse2::<syn::LitStr>(meta_list.tokens.clone()) {
                    return Some(lit.value());
                }
            }
        }
    }
    None
}

/// Check if a type is Option<T>
fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Check if a type is Vec<T>
fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}
