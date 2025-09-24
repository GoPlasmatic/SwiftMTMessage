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
            // Look for #[field("tag")] or #[field("tag", name = "...")] attributes
            if let Some((field_tag, semantic_name)) = extract_field_tag_with_name(&field.attrs) {
                // Determine the rename value: use semantic name if provided, otherwise use field tag
                let rename_value = semantic_name.as_ref().unwrap_or(&field_tag);

                // Handle sequence fields marked with "#"
                if field_tag == "#" {
                    // For sequence fields, add rename to "#" and skip_serializing_if for Vec types
                    let type_category = categorize_type(&field.ty);
                    if matches!(type_category, TypeCategory::Vec | TypeCategory::VecString) {
                        let serde_attr = syn::parse_quote! {
                            #[serde(rename = "#", skip_serializing_if = "Vec::is_empty", default)]
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
                    // Add serde(rename = "...", skip_serializing_if = "Option::is_none") attribute
                    let serde_attr = syn::parse_quote! {
                        #[serde(rename = #rename_value, skip_serializing_if = "Option::is_none")]
                    };
                    field.attrs.push(serde_attr);
                }
                // Check if the field is a Vec type
                else if matches!(type_category, TypeCategory::Vec | TypeCategory::VecString) {
                    // Add serde(rename = "...", skip_serializing_if = "Vec::is_empty") attribute
                    let serde_attr = syn::parse_quote! {
                        #[serde(rename = #rename_value, skip_serializing_if = "Vec::is_empty")]
                    };
                    field.attrs.push(serde_attr);
                } else {
                    // Add serde(rename = "...") attribute for other fields
                    let serde_rename_attr = syn::parse_quote! {
                        #[serde(rename = #rename_value)]
                    };
                    field.attrs.push(serde_rename_attr);
                }
            } else if has_component_attribute(&field.attrs) {
                // This is a field struct with #[component(...)] attributes
                let type_category = categorize_type(&field.ty);

                // Check if this is an f64 field with "15d" or "17d" format (amount fields)
                let is_amount_field = matches!(type_category, TypeCategory::F64)
                    && is_decimal_format_component(&field.attrs);

                if is_amount_field {
                    // Add custom amount serializer for f64 fields with decimal format
                    let serde_attr = syn::parse_quote! {
                        #[serde(with = "crate::serde_helpers::amount_serializer")]
                    };
                    field.attrs.push(serde_attr);
                } else if matches!(type_category, TypeCategory::OptionF64)
                    && is_decimal_format_component(&field.attrs)
                {
                    // Add custom amount serializer for optional f64 fields
                    let serde_attr = syn::parse_quote! {
                        #[serde(with = "crate::serde_helpers::optional_amount_serializer", skip_serializing_if = "Option::is_none")]
                    };
                    field.attrs.push(serde_attr);
                } else if matches!(
                    type_category,
                    TypeCategory::OptionString
                        | TypeCategory::OptionNaiveDate
                        | TypeCategory::OptionNaiveTime
                        | TypeCategory::OptionU32
                        | TypeCategory::OptionU8
                        | TypeCategory::OptionBool
                        | TypeCategory::OptionChar
                        | TypeCategory::OptionField
                        | TypeCategory::OptionVec
                ) {
                    // Add serde(skip_serializing_if = "Option::is_none") attribute
                    let serde_attr = syn::parse_quote! {
                        #[serde(skip_serializing_if = "Option::is_none")]
                    };
                    field.attrs.push(serde_attr);
                }
                // Check if the field is a Vec type
                else if matches!(type_category, TypeCategory::Vec | TypeCategory::VecString) {
                    // Add serde(skip_serializing_if = "Vec::is_empty") attribute
                    let serde_attr = syn::parse_quote! {
                        #[serde(skip_serializing_if = "Vec::is_empty")]
                    };
                    field.attrs.push(serde_attr);
                }
            }
        }
    }

    // Convert the modified input to TokenStream
    let mut tokens = TokenStream::new();
    modified_input.to_tokens(&mut tokens);
    Ok(tokens)
}

/// Extract field tag and optional name from #[field("tag")] or #[field("tag", name = "...")] attribute
fn extract_field_tag_with_name(attrs: &[syn::Attribute]) -> Option<(String, Option<String>)> {
    use crate::utils::attributes::extract_field_attribute_with_name;

    extract_field_attribute_with_name(attrs).ok()
}

/// Check if field has a #[component(...)] attribute
fn has_component_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("component"))
}

/// Check if field has a decimal format component attribute (15d, 17d, etc.)
fn is_decimal_format_component(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("component")
            && let syn::Meta::List(meta_list) = &attr.meta
            && let Ok(lit) = syn::parse2::<syn::LitStr>(meta_list.tokens.clone())
        {
            let value = lit.value();
            // Check for decimal formats like "15d", "17d", etc.
            return value.ends_with('d')
                && value[..value.len() - 1]
                    .chars()
                    .all(|c| c.is_ascii_digit() || c == '!');
        }
    }
    false
}
