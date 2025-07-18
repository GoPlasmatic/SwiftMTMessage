//! Code generation for SwiftField derive macro

use crate::ast::{EnumField, FieldDefinition, FieldKind, StructField};
use crate::codegen::type_generators::{generate_to_swift_string_for_component, generate_sample_for_component};
use crate::error::MacroResult;
use crate::format::generate_regex_parse_impl;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate SwiftField implementation for a field definition
pub fn generate_swift_field_impl(definition: &FieldDefinition) -> MacroResult<TokenStream> {
    let name = &definition.name;

    match &definition.kind {
        FieldKind::Struct(struct_field) => generate_struct_field_impl(name, struct_field),
        FieldKind::Enum(enum_field) => generate_enum_field_impl(name, enum_field),
    }
}

/// Generate SwiftField implementation for struct fields
fn generate_struct_field_impl(
    name: &syn::Ident,
    struct_field: &StructField,
) -> MacroResult<TokenStream> {
    let parse_impl = generate_struct_parse_impl(name, struct_field)?;
    let to_swift_string_impl = generate_struct_to_swift_string_impl(struct_field)?;
    let format_spec_impl = generate_struct_format_spec_impl(struct_field)?;
    let sample_impl = generate_struct_sample_impl(struct_field)?;

    Ok(quote! {
        impl crate::SwiftField for #name {
            fn parse(value: &str) -> crate::Result<Self> {
                #parse_impl
            }

            fn to_swift_string(&self) -> String {
                #to_swift_string_impl
            }

            fn format_spec() -> &'static str {
                #format_spec_impl
            }

            fn sample() -> Self {
                #sample_impl
            }

            fn sample_with_config(config: &crate::sample::FieldConfig) -> Self {
                // Use config for more sophisticated sample generation
                Self::sample()
            }
        }
    })
}

/// Generate SwiftField implementation for enum fields
fn generate_enum_field_impl(name: &syn::Ident, enum_field: &EnumField) -> MacroResult<TokenStream> {
    let parse_impl = generate_enum_parse_impl(enum_field)?;
    let to_swift_string_impl = generate_enum_to_swift_string_impl(enum_field)?;
    let format_spec_impl = generate_enum_format_spec_impl(enum_field)?;
    let sample_impl = generate_enum_sample_impl(enum_field)?;

    // Extract variant information for parse_with_variant
    let variant_idents: Vec<_> = enum_field.variants.iter().map(|v| &v.ident).collect();
    let variant_types: Vec<_> = enum_field.variants.iter().map(|v| &v.type_name).collect();

    Ok(quote! {
        impl crate::SwiftField for #name {
            fn parse(value: &str) -> crate::Result<Self> {
                #parse_impl
            }

            fn parse_with_variant(value: &str, variant: Option<&str>, field_tag: Option<&str>) -> crate::Result<Self> {
                // Try direct variant first if provided
                if let Some(variant_letter) = variant {
                    #(
                        if variant_letter == stringify!(#variant_idents) {
                            // When variant hint is provided, respect it strictly
                            return #variant_types::parse(value)
                                .map(|parsed| Self::#variant_idents(parsed))
                                .map_err(|_| crate::errors::ParseError::InvalidFormat {
                                    message: format!(
                                        "Failed to parse as variant '{}' for field '{}'",
                                        variant_letter,
                                        field_tag.unwrap_or("unknown")
                                    )
                                });
                        }
                    )*

                    // If variant hint doesn't match any known variant, return error
                    return Err(crate::errors::ParseError::InvalidFormat {
                        message: format!(
                            "Unknown variant '{}' for field '{}'",
                            variant_letter,
                            field_tag.unwrap_or("unknown")
                        )
                    });
                } else {
                    // No variant letter provided - try NoOption variant first if it exists
                    #(
                        if stringify!(#variant_idents) == "NoOption" {
                            if let Ok(parsed) = #variant_types::parse(value) {
                                return Ok(Self::#variant_idents(parsed));
                            }
                        }
                    )*

                    // If no variant hint and no NoOption, fall back to trying all variants
                    return Self::parse(value);
                }
            }

            fn to_swift_string(&self) -> String {
                #to_swift_string_impl
            }

            fn format_spec() -> &'static str {
                #format_spec_impl
            }

            fn sample() -> Self {
                #sample_impl
            }

            fn sample_with_config(config: &crate::sample::FieldConfig) -> Self {
                // Try each variant and return the first successful one
                #sample_impl
            }
        }
    })
}

/// Generate parse implementation for struct fields
fn generate_struct_parse_impl(
    name: &syn::Ident,
    struct_field: &StructField,
) -> MacroResult<TokenStream> {
    // Always use regex-based parsing for consistency
    generate_regex_parse_impl(name, struct_field)
}


/// Generate to_swift_string implementation for struct fields
fn generate_struct_to_swift_string_impl(struct_field: &StructField) -> MacroResult<TokenStream> {
    if struct_field.components.len() == 1 {
        let component = &struct_field.components[0];
        let conversion = generate_to_swift_string_for_component(component);
        Ok(conversion)
    } else {
        // For multi-component fields, concatenate all components
        let component_conversions: Vec<_> = struct_field.components
            .iter()
            .map(generate_to_swift_string_for_component)
            .collect();

        Ok(quote! {
            // Concatenate all component string representations
            vec![#(#component_conversions),*].join("")
        })
    }
}

/// Generate format_spec implementation for struct fields
fn generate_struct_format_spec_impl(struct_field: &StructField) -> MacroResult<TokenStream> {
    if struct_field.components.len() == 1 {
        let component = &struct_field.components[0];
        let pattern = &component.format.pattern;

        Ok(quote! { #pattern })
    } else {
        // For multi-component fields, return a generic format spec
        Ok(quote! { "multi" })
    }
}

/// Generate sample implementation for struct fields
fn generate_struct_sample_impl(struct_field: &StructField) -> MacroResult<TokenStream> {
    let field_samples: Vec<_> = struct_field.components
        .iter()
        .map(generate_sample_for_component)
        .collect();

    Ok(quote! {
        Self {
            #(#field_samples),*
        }
    })
}


/// Generate parse implementation for enum fields
fn generate_enum_parse_impl(enum_field: &EnumField) -> MacroResult<TokenStream> {
    let mut variant_attempts = Vec::new();

    for variant in &enum_field.variants {
        let variant_ident = &variant.ident;
        let type_name = &variant.type_name;

        variant_attempts.push(quote! {
            if let Ok(parsed) = #type_name::parse(value) {
                return Ok(Self::#variant_ident(parsed));
            }
        });
    }

    Ok(quote! {
        #(#variant_attempts)*

        Err(crate::errors::ParseError::InvalidFormat {
            message: format!("Unable to parse value '{}' as any variant", value)
        })
    })
}

/// Generate to_swift_string implementation for enum fields
fn generate_enum_to_swift_string_impl(enum_field: &EnumField) -> MacroResult<TokenStream> {
    let mut match_arms = Vec::new();

    for variant in &enum_field.variants {
        let variant_ident = &variant.ident;

        match_arms.push(quote! {
            Self::#variant_ident(inner) => inner.to_swift_string()
        });
    }

    Ok(quote! {
        match self {
            #(#match_arms),*
        }
    })
}

/// Generate format_spec implementation for enum fields  
fn generate_enum_format_spec_impl(_enum_field: &EnumField) -> MacroResult<TokenStream> {
    // For enum fields, we return a generic format spec
    Ok(quote! { "variant" })
}

/// Generate sample implementation for enum fields
fn generate_enum_sample_impl(enum_field: &EnumField) -> MacroResult<TokenStream> {
    if let Some(first_variant) = enum_field.variants.first() {
        let variant_ident = &first_variant.ident;
        let type_name = &first_variant.type_name;

        Ok(quote! {
            Self::#variant_ident(<#type_name as crate::SwiftField>::sample())
        })
    } else {
        Err(crate::error::MacroError::internal(
            proc_macro2::Span::call_site(),
            "Enum must have at least one variant",
        ))
    }
}
