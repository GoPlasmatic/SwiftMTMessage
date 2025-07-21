//! Code generation for SwiftField derive macro

use crate::ast::{EnumField, FieldDefinition, FieldKind, StructField};
use crate::codegen::helpers::{
    generate_optional_prefix_field, generate_account_bic_field, 
    generate_numbered_lines_field
};
use crate::codegen::type_generators::{
    generate_sample_for_component, generate_to_swift_string_for_component,
};
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
                // Note: SWIFT field validation is now handled at the component level
                // to allow for proper field-specific validation rules
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
    let parse_impl = generate_enum_parse_impl(name, enum_field)?;
    let to_swift_string_impl = generate_enum_to_swift_string_impl(enum_field)?;
    let format_spec_impl = generate_enum_format_spec_impl(enum_field)?;
    let sample_impl = generate_enum_sample_impl(enum_field)?;

    // Extract variant information for parse_with_variant
    let variant_idents: Vec<_> = enum_field.variants.iter().map(|v| &v.ident).collect();
    let variant_types: Vec<_> = enum_field.variants.iter().map(|v| &v.type_name).collect();

    Ok(quote! {
        impl crate::SwiftField for #name {
            fn parse(value: &str) -> crate::Result<Self> {
                // Note: SWIFT field validation is now handled at the component level
                // to allow for proper field-specific validation rules
                #parse_impl
            }

            fn parse_with_variant(value: &str, variant: Option<&str>, field_tag: Option<&str>) -> crate::Result<Self> {
                let field_tag = field_tag.unwrap_or("unknown");

                // Try direct variant first if provided
                if let Some(variant_letter) = variant {
                    #(
                        if variant_letter == stringify!(#variant_idents) {
                            return #variant_types::parse(value)
                                .map(|parsed| Self::#variant_idents(parsed))
                                .map_err(|e| crate::errors::ParseError::InvalidFieldFormat {
                                    field_tag: field_tag.to_string(),
                                    component_name: format!("variant_{}", variant_letter),
                                    value: value.to_string(),
                                    format_spec: #variant_types::format_spec().to_string(),
                                    position: None,
                                    inner_error: e.to_string(),
                                });
                        }
                    )*

                    // Unknown variant
                    return Err(crate::errors::ParseError::InvalidFieldFormat {
                        field_tag: field_tag.to_string(),
                        component_name: "variant".to_string(),
                        value: variant_letter.to_string(),
                        format_spec: "Valid variant letter".to_string(),
                        position: None,
                        inner_error: format!("Unknown variant '{}' for field {}", variant_letter, field_tag),
                    });
                } else {
                    // Try NoOption variant first if it exists
                    #(
                        if stringify!(#variant_idents) == "NoOption" {
                            if let Ok(parsed) = #variant_types::parse(value) {
                                return Ok(Self::#variant_idents(parsed));
                            }
                        }
                    )*

                    // Try all variants and collect errors
                    let mut errors = Vec::new();
                    #(
                        match #variant_types::parse(value) {
                            Ok(parsed) => return Ok(Self::#variant_idents(parsed)),
                            Err(e) => errors.push(format!("{}: {}", stringify!(#variant_idents), e)),
                        }
                    )*

                    // All variants failed
                    Err(crate::errors::ParseError::InvalidFieldFormat {
                        field_tag: field_tag.to_string(),
                        component_name: "any_variant".to_string(),
                        value: value.to_string(),
                        format_spec: "One of the valid variants".to_string(),
                        position: None,
                        inner_error: format!("All variants failed: {}", errors.join("; ")),
                    })
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
        // Handle multi-component fields with smart formatting
        generate_multi_component_to_swift_string(struct_field)
    }
}

/// Generate to_swift_string for multi-component fields with smart formatting
fn generate_multi_component_to_swift_string(
    struct_field: &StructField,
) -> MacroResult<TokenStream> {
    // Check for specific patterns that need custom handling
    let patterns: Vec<&str> = struct_field
        .components
        .iter()
        .map(|c| c.format.pattern.as_str())
        .collect();

    // Check for specific patterns that need custom handling

    // Pattern: [/34x] + 4*35x (Field50K style - optional string + vec string)
    if patterns.len() == 2 &&
       patterns[0].starts_with("[/") && patterns[0].ends_with("]") &&
       patterns[0].contains("x") && // Ensure it's a text field
       patterns[1].contains("*") && patterns[1].contains("x")
    {
        let first_component = &struct_field.components[0];
        let second_component = &struct_field.components[1];
        let first_field = &first_component.name;
        let second_field = &second_component.name;

        return Ok(generate_optional_prefix_field(
            first_field,
            second_field,
            '/',
            "\n",
            true,  // first is optional
            false, // second is not optional (it's a vec)
            true,  // second is vec
        ));
    }

    // Pattern: 4!c + [/30x] (Field23E style - string + optional string)
    if patterns.len() == 2
        && !patterns[0].starts_with("[")
        && !patterns[0].ends_with("]")
        && patterns[1].starts_with("[/")
        && patterns[1].ends_with("]")
        && patterns[1].contains("x")
    {
        let first_component = &struct_field.components[0];
        let second_component = &struct_field.components[1];
        let first_field = &first_component.name;
        let second_field = &second_component.name;

        return Ok(generate_optional_prefix_field(
            first_field,
            second_field,
            '/',
            "",    // no separator between components
            false, // first is not optional
            true,  // second is optional
            false, // second is not vec
        ));
    }

    // Pattern: [/34x] + BIC (Field59A style - optional string + BIC string)
    if patterns.len() == 2 &&
       patterns[0].starts_with("[/") && patterns[0].ends_with("]") &&
       patterns[0].contains("x") && // Ensure it's a text field
       patterns[1].contains("!a") && patterns[1].contains("!c")
    {
        let first_component = &struct_field.components[0];
        let second_component = &struct_field.components[1];
        let first_field = &first_component.name;
        let second_field = &second_component.name;

        return Ok(generate_account_bic_field(first_field, second_field));
    }

    // Pattern: [/1!a][/34x] + [35x] (Field53B/Field57B style - optional party identifier + optional location)
    if patterns.len() == 2 && patterns[0] == "[/1!a][/34x]" && patterns[1] == "[35x]" {
        let first_component = &struct_field.components[0];
        let second_component = &struct_field.components[1];
        let first_field = &first_component.name;
        let second_field = &second_component.name;

        // This pattern is simpler - just two optional fields concatenated
        return Ok(quote! {
            {
                let capacity = self.#first_field.as_ref().map(|s| s.len()).unwrap_or(0)
                    + self.#second_field.as_ref().map(|s| s.len() + 1).unwrap_or(0);
                let mut result = String::with_capacity(capacity);

                if let Some(ref party_id) = self.#first_field {
                    result.push_str(party_id);
                }

                if let Some(ref location) = self.#second_field {
                    if !result.is_empty() {
                        result.push_str("\n");
                    }
                    result.push_str(location);
                }

                result
            }
        });
    }

    // Pattern: [/34x] + 4*(1!n/33x) (Field59F style - optional string + vec string with line numbering)
    if patterns.len() == 2 &&
       patterns[0].starts_with("[/") && patterns[0].ends_with("]") &&
       patterns[0].contains("x") && // Ensure it's a text field
       patterns[1] == "4*(1!n/33x)"
    {
        let first_component = &struct_field.components[0];
        let second_component = &struct_field.components[1];
        let first_field = &first_component.name;
        let second_field = &second_component.name;

        return Ok(generate_numbered_lines_field(first_field, second_field));
    }

    // Default: use original concatenation logic for multi-component fields
    let component_conversions: Vec<_> = struct_field
        .components
        .iter()
        .map(generate_to_swift_string_for_component)
        .collect();

    Ok(quote! {
        // Concatenate all component string representations
        vec![#(#component_conversions),*].join("")
    })
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
    let field_samples: Vec<_> = struct_field
        .components
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
fn generate_enum_parse_impl(name: &syn::Ident, enum_field: &EnumField) -> MacroResult<TokenStream> {
    let mut variant_attempts = Vec::new();
    let variant_names: Vec<String> = enum_field.variants.iter()
        .map(|v| v.ident.to_string())
        .collect();
    let variants_list = variant_names.join(", ");

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

        Err(crate::errors::ParseError::InvalidFieldFormat {
            field_tag: stringify!(#name).to_string(),
            component_name: "variant".to_string(),
            value: value.to_string(),
            format_spec: format!("One of the following variants: {}", #variants_list),
            position: None,
            inner_error: "Unable to parse value as any variant".to_string(),
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
