//! Type-specific code generators for field conversion and serialization
//!
//! This module provides specialized code generators for different field types,
//! eliminating code duplication and improving maintainability.

use crate::ast::Component;
use crate::utils::types::{TypeCategory, TypeMatcher, categorize_type, matchers};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

/// Generate to_swift_string code for a specific component based on its type
pub fn generate_to_swift_string_for_component(component: &Component) -> TokenStream {
    let field_name = &component.name;
    let field_type = &component.field_type;

    // Handle special format patterns that require delimiters or specific formatting
    match component.format.pattern.as_str() {
        "/8c/" => {
            // For /8c/ format, wrap the value in slashes
            quote! {
                format!("/{}/", self.#field_name)
            }
        }
        // Handle [1!a] format for negative indicator (Field37H)
        "[1!a]" if matches!(categorize_type(field_type), TypeCategory::OptionBool) => {
            quote! {
                self.#field_name.map(|b| if b { "N".to_string() } else { String::new() }).unwrap_or_default()
            }
        }
        // Handle 2!n format for two-digit numbers with leading zeros
        "2!n"
            if matches!(
                categorize_type(field_type),
                TypeCategory::U8 | TypeCategory::OptionU8
            ) =>
        {
            if matches!(categorize_type(field_type), TypeCategory::OptionU8) {
                quote! {
                    self.#field_name.map(|n| format!("{:02}", n)).unwrap_or_default()
                }
            } else {
                quote! {
                    format!("{:02}", self.#field_name)
                }
            }
        }
        // Handle optional prefix patterns like [/34x], [/2n], [/5n]
        pattern if pattern.starts_with("[/") && pattern.ends_with("]") => {
            if matchers::option(matchers::string()).matches(field_type) {
                quote! {
                    self.#field_name.as_ref()
                        .map(|value| format!("/{}", value))
                        .unwrap_or_default()
                }
            } else if matchers::option(matchers::u32()).matches(field_type)
                || matchers::option(matchers::u8()).matches(field_type)
            {
                quote! {
                    self.#field_name
                        .map(|value| format!("/{}", value))
                        .unwrap_or_default()
                }
            } else {
                generate_to_swift_string_for_type(field_name, field_type)
            }
        }
        // Handle multi-line with line numbering like 4*(1!n/33x)
        pattern if pattern.contains("*(1!n/") => {
            if matchers::vec(matchers::string()).matches(field_type) {
                quote! {
                    self.#field_name.iter()
                        .enumerate()
                        .map(|(i, line)| format!("{}/{}", i + 1, line))
                        .collect::<Vec<_>>()
                        .join("\n")
                }
            } else {
                generate_to_swift_string_for_type(field_name, field_type)
            }
        }
        // Handle amount format (15d) - always show 2 decimal places with comma separator
        "15d" => {
            if matchers::f64().matches(field_type) {
                quote! {
                    format!("{:.2}", self.#field_name).replace('.', ",")
                }
            } else {
                generate_to_swift_string_for_type(field_name, field_type)
            }
        }
        _ => generate_to_swift_string_for_type(field_name, field_type),
    }
}

/// Generate to_swift_string code for a field with a specific type
pub fn generate_to_swift_string_for_type(
    field_name: &syn::Ident,
    field_type: &Type,
) -> TokenStream {
    // Use TypeCategory for efficient categorization with caching
    match categorize_type(field_type) {
        TypeCategory::NaiveDate => quote! {
            self.#field_name.format("%y%m%d").to_string()
        },
        TypeCategory::NaiveTime => quote! {
            self.#field_name.format("%H%M").to_string()
        },
        TypeCategory::Char => quote! {
            self.#field_name.to_string()
        },
        TypeCategory::F64 => quote! {
            // For exchange rates, preserve original precision by using the format that removes trailing zeros
            format!("{}", self.#field_name).replace('.', ",")
        },
        TypeCategory::U32 | TypeCategory::U8 => quote! {
            self.#field_name.to_string()
        },
        TypeCategory::Bool => quote! {
            if self.#field_name { "true".to_string() } else { "false".to_string() }
        },
        TypeCategory::OptionNaiveDate => quote! {
            self.#field_name.as_ref()
                .map(|d| d.format("%y%m%d").to_string())
                .unwrap_or_default()
        },
        TypeCategory::OptionString => quote! {
            self.#field_name.as_ref().unwrap_or(&String::new()).clone()
        },
        TypeCategory::OptionU32 | TypeCategory::OptionU8 => quote! {
            self.#field_name.map(|n| n.to_string()).unwrap_or_default()
        },
        TypeCategory::OptionBool => quote! {
            self.#field_name.map(|b| if b { "true".to_string() } else { "false".to_string() }).unwrap_or_default()
        },
        TypeCategory::OptionChar => quote! {
            self.#field_name.map(|c| c.to_string()).unwrap_or_default()
        },
        TypeCategory::VecString => quote! {
            self.#field_name.join("\n")
        },
        TypeCategory::Vec => quote! {
            self.#field_name.iter()
                .map(|item| item.to_swift_string())
                .collect::<Vec<_>>()
                .join("")
        },
        TypeCategory::OptionField => quote! {
            self.#field_name.as_ref()
                .map(|item| item.to_swift_string())
                .unwrap_or_default()
        },
        TypeCategory::Field => quote! {
            self.#field_name.to_swift_string()
        },
        TypeCategory::String => quote! {
            self.#field_name.clone()
        },
        _ => {
            // Default to calling to_swift_string for unknown types
            quote! {
                self.#field_name.to_swift_string()
            }
        }
    }
}
