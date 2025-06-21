use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Meta, Type, parse_macro_input, parse_quote};

/// Attribute macro implementation that automatically adds serde attributes based on field configurations
pub fn serde_swift_fields_impl(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    match &mut input.data {
        Data::Struct(data) => {
            if let Fields::Named(fields) = &mut data.fields {
                for field in &mut fields.named {
                    if let Some((field_tag, is_optional_config)) =
                        extract_field_metadata_from_attrs(&field.attrs)
                    {
                        // Check if serde attributes already exist
                        let has_serde_attrs =
                            field.attrs.iter().any(|attr| attr.path().is_ident("serde"));

                        // Add serde attributes if they don't exist
                        if !has_serde_attrs {
                            // Check if the field type is actually Option<T>
                            let is_option_type_actual = is_option_type(&field.ty);

                            if is_optional_config && is_option_type_actual {
                                // Optional field with Option<T> type: add rename and skip_serializing_if
                                let serde_attr: Attribute = parse_quote! {
                                    #[serde(rename = #field_tag, skip_serializing_if = "Option::is_none")]
                                };
                                field.attrs.push(serde_attr);
                            } else {
                                // Mandatory field or non-Option type: add only rename
                                let serde_attr: Attribute = parse_quote! {
                                    #[serde(rename = #field_tag)]
                                };
                                field.attrs.push(serde_attr);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(
                &input,
                "serde_swift_fields can only be applied to structs",
            )
            .to_compile_error()
            .into();
        }
    }

    TokenStream::from(quote! { #input })
}

/// Extract field tag and optional flag from #[field("tag", mandatory/optional)] attributes
fn extract_field_metadata_from_attrs(attrs: &[Attribute]) -> Option<(String, bool)> {
    for attr in attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("field") {
                let tokens = meta_list.tokens.to_string();

                // Extract field tag (first quoted string)
                if let Some(tag_start) = tokens.find('"') {
                    if let Some(tag_end) = tokens[tag_start + 1..].find('"') {
                        let field_tag = tokens[tag_start + 1..tag_start + 1 + tag_end].to_string();

                        // Determine if field is optional
                        let is_optional =
                            tokens.contains("optional") || !tokens.contains("mandatory");

                        return Some((field_tag, is_optional));
                    }
                }
            }
        }
    }
    None
}

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}
