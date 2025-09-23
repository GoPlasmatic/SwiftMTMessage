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
    let pattern = &component.format.pattern;

    // Check if this pattern has a slash prefix
    use crate::slash_handler_utils;
    let slash_type = slash_handler_utils::get_slash_type(pattern);
    let has_slash = !matches!(slash_type, slash_handler_utils::SlashPrefixType::None);

    // Handle special format patterns that require delimiters or specific formatting
    match pattern.as_str() {
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
        // Use slash handler for patterns with slash prefixes
        _ if has_slash => {
            let slash_type_expr = slash_handler_utils::generate_slash_type_expr(slash_type);

            if matchers::option(matchers::string()).matches(field_type) {
                quote! {
                    self.#field_name.as_ref()
                        .map(|value| {
                            use crate::slash_handler::SlashPrefixHandler;
                            crate::slash_handler::SwiftSlashHandler::serialize_with_slash(value, #slash_type_expr)
                        })
                        .unwrap_or_default()
                }
            } else if matchers::option(matchers::u32()).matches(field_type)
                || matchers::option(matchers::u8()).matches(field_type)
            {
                quote! {
                    self.#field_name
                        .map(|value| {
                            use crate::slash_handler::SlashPrefixHandler;
                            crate::slash_handler::SwiftSlashHandler::serialize_with_slash(&value.to_string(), #slash_type_expr)
                        })
                        .unwrap_or_default()
                }
            } else if matchers::string().matches(field_type) {
                // Required string field with slash prefix
                let base_value = quote! { self.#field_name.as_str() };
                quote! {
                    {
                        use crate::slash_handler::SlashPrefixHandler;
                        crate::slash_handler::SwiftSlashHandler::serialize_with_slash(
                            #base_value,
                            #slash_type_expr
                        )
                    }
                }
            } else {
                // Other types - convert to string first
                let base_value = generate_to_swift_string_for_type(field_name, field_type);
                quote! {
                    {
                        use crate::slash_handler::SlashPrefixHandler;
                        crate::slash_handler::SwiftSlashHandler::serialize_with_slash(
                            &(#base_value),
                            #slash_type_expr
                        )
                    }
                }
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
        // Handle amount format (15d) - format with appropriate precision
        "15d" => {
            if matchers::f64().matches(field_type) {
                quote! {
                    {
                        // Format the amount preserving EXACT precision
                        // SWIFT amounts use comma as decimal separator
                        // Use Rust's default f64 Display which preserves the exact value
                        let value = self.#field_name;
                        // Use format! with no precision specifier to get exact representation
                        let formatted = format!("{}", value);
                        formatted.replace('.', ",")
                    }
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
            // Preserve EXACT precision by using Rust's default f64 Display
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
