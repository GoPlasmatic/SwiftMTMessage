//! Helper functions for code generation to reduce duplication
//!
//! This module contains reusable code generation helpers that are used across
//! different field patterns to eliminate code duplication in the macro system.

use proc_macro2::TokenStream;
use quote::quote;
use syn;

/// Generate code for fields with optional prefix pattern
///
/// Handles patterns like:
/// - [/34x] + 4*35x (Field50K, Field53B, Field57B, Field58B)
/// - 4!c + [/30x] (Field23E)
/// - [/1!a][/34x] + BIC (Field53A, Field57A)
pub fn generate_optional_prefix_field(
    first_field: &syn::Ident,
    second_field: &syn::Ident,
    prefix: char,
    separator: &str,
    first_is_optional: bool,
    second_is_optional: bool,
    second_is_vec: bool,
) -> TokenStream {
    let capacity_calc = if first_is_optional {
        quote! {
            self.#first_field.as_ref().map(|s| s.len() + 1).unwrap_or(0)
        }
    } else {
        quote! { self.#first_field.len() }
    };

    let second_capacity = if second_is_vec {
        quote! { self.#second_field.iter().map(|s| s.len() + 1).sum::<usize>() }
    } else if second_is_optional {
        quote! { self.#second_field.as_ref().map(|s| s.len() + 1).unwrap_or(0) }
    } else {
        quote! { self.#second_field.len() }
    };

    let add_first = if first_is_optional {
        quote! {
            if let Some(ref value) = self.#first_field {
                result.push(#prefix);
                result.push_str(value);
            }
        }
    } else {
        quote! {
            result.push_str(&self.#first_field);
        }
    };

    let add_separator = if !separator.is_empty() {
        quote! {
            if !result.is_empty() {
                result.push_str(#separator);
            }
        }
    } else {
        quote! {}
    };

    let add_second = if second_is_vec {
        quote! {
            if !self.#second_field.is_empty() {
                #add_separator
                result.push_str(&self.#second_field.join("\n"));
            }
        }
    } else if second_is_optional {
        quote! {
            if let Some(ref value) = self.#second_field {
                #add_separator
                result.push(#prefix);
                result.push_str(value);
            }
        }
    } else {
        quote! {
            #add_separator
            result.push_str(&self.#second_field);
        }
    };

    quote! {
        {
            let capacity = #capacity_calc + #second_capacity;
            let mut result = String::with_capacity(capacity);

            #add_first
            #add_second

            result
        }
    }
}

/// Generate code for account/BIC pattern fields
///
/// Handles patterns like:
/// - [/34x] + BIC (Field56A, Field57A, Field59A)
pub fn generate_account_bic_field(
    account_field: &syn::Ident,
    bic_field: &syn::Ident,
) -> TokenStream {
    quote! {
        {
            let capacity = self.#account_field.as_ref().map(|s| s.len() + 2).unwrap_or(0)
                + self.#bic_field.len();
            let mut result = String::with_capacity(capacity);

            if let Some(ref account) = self.#account_field {
                result.push('/');
                result.push_str(account);
                result.push_str("\n");
            }

            result.push_str(&self.#bic_field);
            result
        }
    }
}

/// Generate code for Field59F pattern with line numbering
///
/// Handles pattern: [/34x] + 4*(1!n/33x) where lines are numbered
pub fn generate_numbered_lines_field(
    party_field: &syn::Ident,
    lines_field: &syn::Ident,
) -> TokenStream {
    quote! {
        {
            let capacity = self.#party_field.as_ref().map(|s| s.len() + 1).unwrap_or(0)
                + self.#lines_field.iter().enumerate()
                    .map(|(i, s)| s.len() + 3 + (i + 1).to_string().len())
                    .sum::<usize>();
            let mut result = String::with_capacity(capacity);

            if let Some(ref party_id) = self.#party_field {
                result.push('/');
                result.push_str(party_id);
            }

            use std::fmt::Write;
            for (i, line) in self.#lines_field.iter().enumerate() {
                if !result.is_empty() || i > 0 {
                    result.push_str("\n");
                }
                write!(&mut result, "{}/{}", i + 1, line).unwrap();
            }

            result
        }
    }
}
