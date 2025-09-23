//! Slash handler utilities for macro code generation
//!
//! This module contains utilities used by the macros to generate code that handles slash prefixes.
//! The actual runtime implementation is in the swift-mt-message crate.

use quote::quote;

/// Types of slash prefixes in SWIFT patterns (compile-time version)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashPrefixType {
    /// No slash prefix
    None,
    /// Optional slash prefix: [/34x] - slash required when value present
    Optional,
    /// Required slash prefix: /34x - always has slash
    Required,
    /// Double slash prefix: //16x - double slash prefix
    Double,
    /// Wrapped in slashes: /8c/ - both leading and trailing
    Wrapper,
    /// Optional with numeric constraint: [/5n]
    OptionalNumeric(usize),
}

/// Detect slash type from a pattern (compile-time)
pub fn get_slash_type(pattern: &str) -> SlashPrefixType {
    // Handle wrapped pattern (e.g., /8c/)
    if pattern.starts_with('/') && pattern.ends_with('/') && pattern.len() > 2 {
        return SlashPrefixType::Wrapper;
    }

    // Handle optional patterns [...]
    if pattern.starts_with('[') && pattern.ends_with(']') {
        let inner = &pattern[1..pattern.len() - 1];

        // Double slash optional [//16x]
        if inner.starts_with("//") {
            return SlashPrefixType::Double;
        }

        // Single slash optional [/34x]
        if let Some(stripped) = inner.strip_prefix('/') {
            // Check if numeric pattern like [/5n]
            if let Some(n) = extract_numeric_length(stripped) {
                return SlashPrefixType::OptionalNumeric(n);
            }
            return SlashPrefixType::Optional;
        }
    }

    // Handle required slash patterns
    if pattern.starts_with('/') {
        if pattern.starts_with("//") {
            return SlashPrefixType::Double;
        }
        return SlashPrefixType::Required;
    }

    SlashPrefixType::None
}

/// Extract numeric length from a pattern like "5n" or "34x"
fn extract_numeric_length(pattern: &str) -> Option<usize> {
    // Look for patterns like "5n", "2n", etc.
    if pattern.len() >= 2
        && pattern.ends_with('n')
        && let Ok(n) = pattern[..pattern.len() - 1].parse::<usize>()
    {
        return Some(n);
    }
    None
}

/// Generate slash type expression for use in quote! macros
pub fn generate_slash_type_expr(slash_type: SlashPrefixType) -> proc_macro2::TokenStream {
    match slash_type {
        SlashPrefixType::None => quote! { crate::slash_handler::SlashPrefixType::None },
        SlashPrefixType::Optional => quote! { crate::slash_handler::SlashPrefixType::Optional },
        SlashPrefixType::Required => quote! { crate::slash_handler::SlashPrefixType::Required },
        SlashPrefixType::Double => quote! { crate::slash_handler::SlashPrefixType::Double },
        SlashPrefixType::Wrapper => quote! { crate::slash_handler::SlashPrefixType::Wrapper },
        SlashPrefixType::OptionalNumeric(n) => {
            quote! { crate::slash_handler::SlashPrefixType::OptionalNumeric(#n) }
        }
    }
}
