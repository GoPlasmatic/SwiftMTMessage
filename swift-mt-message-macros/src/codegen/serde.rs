//! Serde integration for clean JSON serialization

use crate::error::MacroResult;
use crate::utils::types::{TypeCategory, categorize_type};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

/// Generate serde attributes for clean JSON serialization
/// Adds serde(rename = "field_tag") attributes to fields based on their #[field("tag")] attributes
pub fn generate_serde_attributes(input: &DeriveInput) -> MacroResult<TokenStream> {
    let mut modified_input = input.clone();

    // Process struct fields if it's a struct
    if let syn::Data::Struct(ref mut data_struct) = modified_input.data
        && let syn::Fields::Named(ref mut fields) = data_struct.fields
    {
        for field in &mut fields.named {
            // Look for #[field("tag")] attributes
            if let Some(field_tag) = extract_field_tag(&field.attrs) {
                // Handle sequence fields marked with "#"
                if field_tag == "#" {
                    // For sequence fields, add rename to "#" and skip_serializing_if for Vec types
                    let type_category = categorize_type(&field.ty);
                    if matches!(type_category, TypeCategory::Vec | TypeCategory::VecString) {
                        let rename_value = "#";
                        let serde_attr = syn::parse_quote! {
                            #[serde(rename = #rename_value, skip_serializing_if = "Vec::is_empty", default)]
                        };
                        field.attrs.push(serde_attr);
                    }
                    continue;
                }

                // Check if the field is an Option type
                let type_category = categorize_type(&field.ty);
                if matches!(
                    type_category,
                    TypeCategory::OptionString
                        | TypeCategory::OptionNaiveDate
                        | TypeCategory::OptionNaiveTime
                        | TypeCategory::OptionF64
                        | TypeCategory::OptionU32
                        | TypeCategory::OptionU8
                        | TypeCategory::OptionBool
                        | TypeCategory::OptionChar
                        | TypeCategory::OptionField
                        | TypeCategory::OptionVec
                ) {
                    // Add serde(rename = "tag", skip_serializing_if = "Option::is_none") attribute
                    let serde_attr = syn::parse_quote! {
                        #[serde(rename = #field_tag, skip_serializing_if = "Option::is_none")]
                    };
                    field.attrs.push(serde_attr);
                }
                // Check if the field is a Vec type
                else if matches!(type_category, TypeCategory::Vec | TypeCategory::VecString) {
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

    // Convert the modified input to TokenStream
    let mut tokens = TokenStream::new();
    modified_input.to_tokens(&mut tokens);
    Ok(tokens)
}

/// Extract field tag from #[field("tag")] attribute
fn extract_field_tag(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("field")
            && let syn::Meta::List(meta_list) = &attr.meta
            && let Ok(lit) = syn::parse2::<syn::LitStr>(meta_list.tokens.clone())
        {
            return Some(lit.value());
        }
    }
    None
}
