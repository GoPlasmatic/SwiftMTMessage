//! Type checking utilities for procedural macro generation
//!
//! This module provides centralized type checking functions to avoid code duplication
//! across the macro implementation. These functions help identify common Rust types
//! and their generic variants (`Option<T>`, `Vec<T>`, etc.).
//!
//! The module uses a trait-based type matching system for flexibility and extensibility,
//! building on the cached TypeCategory system for performance.

use quote::ToTokens;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use syn::{GenericArgument, PathArguments, Type};

/// Cached type categories for fast lookup without re-analyzing types
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TypeCategory {
    // Basic types
    String,
    NaiveDate,
    NaiveTime,
    F64,
    U32,
    U8,
    Bool,
    Char,

    // Optional types
    OptionString,
    OptionNaiveDate,
    OptionNaiveTime,
    OptionF64,
    OptionU32,
    OptionU8,
    OptionBool,
    OptionChar,
    OptionField,

    // Vector types
    Vec,
    VecString,
    OptionVec,

    // Field types
    Field,

    // Unknown/Other
    Unknown,
}

/// Global cache for type analysis results with thread-safe access
static TYPE_CACHE: OnceLock<Mutex<HashMap<String, TypeCategory>>> = OnceLock::new();

/// Get or initialize the type cache
fn get_type_cache() -> &'static Mutex<HashMap<String, TypeCategory>> {
    TYPE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Categorize a type and cache the result for performance
pub fn categorize_type(ty: &Type) -> TypeCategory {
    let type_string = ty.to_token_stream().to_string().replace(" ", "");

    // Check cache first
    if let Ok(cache) = get_type_cache().lock() {
        if let Some(category) = cache.get(&type_string) {
            return category.clone();
        }
    }

    // Analyze the type
    let category = analyze_type_category(ty);

    // Cache the result
    if let Ok(mut cache) = get_type_cache().lock() {
        cache.insert(type_string, category.clone());
    }

    category
}

/// Analyze the type category without caching
fn analyze_type_category(ty: &Type) -> TypeCategory {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = &segment.ident.to_string();

            match ident.as_str() {
                "String" => return TypeCategory::String,
                "NaiveDate" => return TypeCategory::NaiveDate,
                "NaiveTime" => return TypeCategory::NaiveTime,
                "f64" => return TypeCategory::F64,
                "u32" => return TypeCategory::U32,
                "u8" => return TypeCategory::U8,
                "bool" => return TypeCategory::Bool,
                "char" => return TypeCategory::Char,
                "Vec" => {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                            if matches!(analyze_type_category(inner_ty), TypeCategory::String) {
                                return TypeCategory::VecString;
                            }
                        }
                    }
                    return TypeCategory::Vec;
                }
                "Option" => {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                            match analyze_type_category(inner_ty) {
                                TypeCategory::String => return TypeCategory::OptionString,
                                TypeCategory::NaiveDate => return TypeCategory::OptionNaiveDate,
                                TypeCategory::NaiveTime => return TypeCategory::OptionNaiveTime,
                                TypeCategory::F64 => return TypeCategory::OptionF64,
                                TypeCategory::U32 => return TypeCategory::OptionU32,
                                TypeCategory::U8 => return TypeCategory::OptionU8,
                                TypeCategory::Bool => return TypeCategory::OptionBool,
                                TypeCategory::Char => return TypeCategory::OptionChar,
                                TypeCategory::Vec | TypeCategory::VecString => {
                                    return TypeCategory::OptionVec
                                }
                                TypeCategory::Field => return TypeCategory::OptionField,
                                _ => return TypeCategory::OptionField, // Assume unknown Option<T> is a field
                            }
                        }
                    }
                }
                _ => {
                    // Check if it's a field type (not a basic type)
                    if !matches!(
                        ident.as_str(),
                        "String"
                            | "NaiveDate"
                            | "NaiveTime"
                            | "f64"
                            | "u32"
                            | "u8"
                            | "bool"
                            | "char"
                            | "Vec"
                            | "Option"
                    ) {
                        return TypeCategory::Field;
                    }
                }
            }
        }
    }

    TypeCategory::Unknown
}

// Note: Old is_*_type functions have been replaced by TypeCategory enum and matchers
// Use categorize_type() and the matchers module for type checking instead

/// Extract inner type from Option<T>, Vec<T>, or return the type as-is
pub fn extract_inner_type(ty: &Type, is_optional: bool, is_repetitive: bool) -> Type {
    if is_optional || is_repetitive {
        if let Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.last() {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return inner_type.clone();
                    }
                }
            }
        }
    }
    ty.clone()
}

/// Extract the inner type from Option<T> or Vec<T>
#[allow(dead_code)]
pub fn extract_generic_inner_type(ty: &Type) -> Option<Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    return Some(inner_type.clone());
                }
            }
        }
    }
    None
}

/// Extract the inner type T from Option<Vec<T>>
pub fn extract_option_vec_inner_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(Type::Path(vec_type_path))) =
                        args.args.first()
                    {
                        if let Some(vec_segment) = vec_type_path.path.segments.last() {
                            if vec_segment.ident == "Vec" {
                                if let PathArguments::AngleBracketed(vec_args) =
                                    &vec_segment.arguments
                                {
                                    if let Some(GenericArgument::Type(inner_type)) =
                                        vec_args.args.first()
                                    {
                                        return inner_type.clone();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ty.clone()
}

// ===== Trait-based Type Matching System =====

/// Trait for matching types with composable patterns
pub trait TypeMatcher: std::fmt::Debug {
    /// Check if this matcher matches the given type
    fn matches(&self, ty: &Type) -> bool;

    /// Get a human-readable description of what this matcher matches
    fn description(&self) -> String;
}

/// Matcher for basic types like String, u32, etc.
#[derive(Debug, Clone)]
pub struct BasicTypeMatcher {
    type_name: &'static str,
    category: TypeCategory,
}

impl BasicTypeMatcher {
    pub const fn new(type_name: &'static str, category: TypeCategory) -> Self {
        Self {
            type_name,
            category,
        }
    }
}

impl TypeMatcher for BasicTypeMatcher {
    fn matches(&self, ty: &Type) -> bool {
        matches!(categorize_type(ty), ref cat if cat == &self.category)
    }

    fn description(&self) -> String {
        format!("Type::{}", self.type_name)
    }
}

/// Matcher for Option<T> types
#[derive(Debug)]
pub struct OptionTypeMatcher<T: TypeMatcher> {
    inner: T,
}

impl<T: TypeMatcher> OptionTypeMatcher<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: TypeMatcher> TypeMatcher for OptionTypeMatcher<T> {
    fn matches(&self, ty: &Type) -> bool {
        if let Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Option" {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                            return self.inner.matches(inner_ty);
                        }
                    }
                }
            }
        }
        false
    }

    fn description(&self) -> String {
        format!("Option<{}>", self.inner.description())
    }
}

/// Matcher for Vec<T> types
#[derive(Debug)]
pub struct VecTypeMatcher<T: TypeMatcher> {
    inner: T,
}

impl<T: TypeMatcher> VecTypeMatcher<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: TypeMatcher> TypeMatcher for VecTypeMatcher<T> {
    fn matches(&self, ty: &Type) -> bool {
        if let Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Vec" {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                            return self.inner.matches(inner_ty);
                        }
                    }
                }
            }
        }
        false
    }

    fn description(&self) -> String {
        format!("Vec<{}>", self.inner.description())
    }
}

/// Convenience functions for creating matchers
pub mod matchers {
    use super::*;

    /// Create a matcher for String type
    pub fn string() -> BasicTypeMatcher {
        BasicTypeMatcher::new("String", TypeCategory::String)
    }

    /// Create a matcher for f64 type
    pub fn f64() -> BasicTypeMatcher {
        BasicTypeMatcher::new("f64", TypeCategory::F64)
    }

    /// Create a matcher for u32 type
    pub fn u32() -> BasicTypeMatcher {
        BasicTypeMatcher::new("u32", TypeCategory::U32)
    }

    /// Create a matcher for u8 type
    pub fn u8() -> BasicTypeMatcher {
        BasicTypeMatcher::new("u8", TypeCategory::U8)
    }

    /// Create a matcher for Option<T>
    pub fn option<T: TypeMatcher>(inner: T) -> OptionTypeMatcher<T> {
        OptionTypeMatcher::new(inner)
    }

    /// Create a matcher for Vec<T>
    pub fn vec<T: TypeMatcher>(inner: T) -> VecTypeMatcher<T> {
        VecTypeMatcher::new(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use syn::parse_quote;

    #[test]
    fn test_basic_type_detection() {
        let string_type: Type = parse_quote!(String);
        let option_string_type: Type = parse_quote!(Option<String>);
        let vec_string_type: Type = parse_quote!(Vec<String>);
        let naive_date_type: Type = parse_quote!(NaiveDate);
        let u32_type: Type = parse_quote!(u32);

        assert_eq!(categorize_type(&string_type), TypeCategory::String);
        assert!(matches!(
            categorize_type(&option_string_type),
            TypeCategory::OptionString
        ));
        assert!(matches!(
            categorize_type(&vec_string_type),
            TypeCategory::VecString
        ));
        assert_eq!(categorize_type(&naive_date_type), TypeCategory::NaiveDate);
        assert_eq!(categorize_type(&u32_type), TypeCategory::U32);
    }

    #[test]
    fn test_option_type_variants() {
        let option_string: Type = parse_quote!(Option<String>);
        let option_u32: Type = parse_quote!(Option<u32>);
        let option_naive_date: Type = parse_quote!(Option<NaiveDate>);

        assert_eq!(categorize_type(&option_string), TypeCategory::OptionString);
        assert_eq!(categorize_type(&option_u32), TypeCategory::OptionU32);
        assert_eq!(
            categorize_type(&option_naive_date),
            TypeCategory::OptionNaiveDate
        );
    }

    #[test]
    fn test_field_type_detection() {
        let field_type: Type = parse_quote!(Field20);
        let string_type: Type = parse_quote!(String);
        let option_field_type: Type = parse_quote!(Option<Field20>);

        assert_eq!(categorize_type(&field_type), TypeCategory::Field);
        assert_eq!(categorize_type(&string_type), TypeCategory::String);
        assert_eq!(
            categorize_type(&option_field_type),
            TypeCategory::OptionField
        );
    }

    #[test]
    fn test_inner_type_extraction() {
        let option_string: Type = parse_quote!(Option<String>);
        let vec_string: Type = parse_quote!(Vec<String>);
        let string_type: Type = parse_quote!(String);

        let inner_from_option = extract_inner_type(&option_string, true, false);
        let inner_from_vec = extract_inner_type(&vec_string, false, true);
        let inner_from_basic = extract_inner_type(&string_type, false, false);

        assert_eq!(categorize_type(&inner_from_option), TypeCategory::String);
        assert_eq!(categorize_type(&inner_from_vec), TypeCategory::String);
        assert_eq!(categorize_type(&inner_from_basic), TypeCategory::String);
    }

    #[test]
    fn test_type_categorization_caching() {
        let string_type: Type = parse_quote!(String);
        let option_string_type: Type = parse_quote!(Option<String>);
        let field_type: Type = parse_quote!(Field20);

        // Test direct categorization
        assert_eq!(categorize_type(&string_type), TypeCategory::String);
        assert_eq!(
            categorize_type(&option_string_type),
            TypeCategory::OptionString
        );
        assert_eq!(categorize_type(&field_type), TypeCategory::Field);

        // Test that caching improves performance for repeated calls
        let start = Instant::now();
        for _ in 0..1000 {
            categorize_type(&string_type);
            categorize_type(&option_string_type);
            categorize_type(&field_type);
        }
        let cached_duration = start.elapsed();

        // Performance should be very fast with caching
        // This is mainly a smoke test to ensure caching is functional
        assert!(
            cached_duration.as_millis() < 100,
            "Caching should provide fast repeated access"
        );
    }

    #[test]
    fn test_category_matches() {
        let string_type: Type = parse_quote!(String);
        let option_string_type: Type = parse_quote!(Option<String>);
        let vec_string_type: Type = parse_quote!(Vec<String>);
        let option_vec_type: Type = parse_quote!(Option<Vec<String>>);

        // Test that the new categorization system provides correct results
        assert_eq!(categorize_type(&string_type), TypeCategory::String);
        assert_eq!(
            categorize_type(&option_string_type),
            TypeCategory::OptionString
        );
        assert_eq!(categorize_type(&vec_string_type), TypeCategory::VecString);
        assert_eq!(categorize_type(&option_vec_type), TypeCategory::OptionVec);
    }

    #[test]
    fn test_trait_based_matchers() {
        use matchers::*;

        let string_type: Type = parse_quote!(String);
        let option_string_type: Type = parse_quote!(Option<String>);
        let vec_string_type: Type = parse_quote!(Vec<String>);
        let option_vec_string_type: Type = parse_quote!(Option<Vec<String>>);
        let field_type: Type = parse_quote!(Field20);
        let option_field_type: Type = parse_quote!(Option<Field20>);

        // Test basic matchers
        assert!(string().matches(&string_type));
        assert!(!string().matches(&option_string_type));

        // Test Option matcher
        assert!(option(string()).matches(&option_string_type));
        assert!(!option(string()).matches(&string_type));

        // Test Vec matcher
        assert!(vec(string()).matches(&vec_string_type));
        assert!(!vec(string()).matches(&string_type));

        // Test nested matchers
        assert!(option(vec(string())).matches(&option_vec_string_type));

        // Test field type categorization
        assert_eq!(categorize_type(&field_type), TypeCategory::Field);
        assert_eq!(
            categorize_type(&option_field_type),
            TypeCategory::OptionField
        );
    }

    #[test]
    fn test_matcher_descriptions() {
        use matchers::*;

        assert_eq!(string().description(), "Type::String");
        assert_eq!(option(string()).description(), "Option<Type::String>");
        assert_eq!(vec(string()).description(), "Vec<Type::String>");
        assert_eq!(
            option(vec(string())).description(),
            "Option<Vec<Type::String>>"
        );
    }
}
