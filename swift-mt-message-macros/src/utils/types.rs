//! Type checking utilities for procedural macro generation
//!
//! This module provides centralized type checking functions to avoid code duplication
//! across the macro implementation. These functions help identify common Rust types
//! and their generic variants (Option<T>, Vec<T>, etc.).

use syn::{GenericArgument, PathArguments, Type};

/// Check if a type is Option<T>
pub fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Check if a type is Vec<T>
pub fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// Check if a type is String
pub fn is_string_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        // Check if the path ends with String (handles both String and std::string::String)
        let path_str = quote::quote!(#type_path).to_string();
        // Remove spaces and check various forms
        let normalized = path_str.replace(" ", "");
        return normalized == "String"
            || normalized == "std::string::String"
            || normalized.ends_with("::String")
            || path_str == "String";
    }
    false
}

/// Check if a type is NaiveDate
pub fn is_naive_date_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "NaiveDate";
        }
    }
    false
}

/// Check if a type is NaiveTime
pub fn is_naive_time_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "NaiveTime";
        }
    }
    false
}

/// Check if a type is f64
pub fn is_f64_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "f64";
        }
    }
    false
}

/// Check if a type is u32
pub fn is_u32_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "u32";
        }
    }
    false
}

/// Check if a type is u8
pub fn is_u8_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "u8";
        }
    }
    false
}

/// Check if a type is bool
pub fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "bool";
        }
    }
    false
}

/// Check if a type is char
pub fn is_char_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "char";
        }
    }
    false
}

/// Check if a type is Option<String>
pub fn is_option_string_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_string_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Option<NaiveDate>
pub fn is_option_naive_date_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_naive_date_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Option<u32>
pub fn is_option_u32_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_u32_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Option<u8>
pub fn is_option_u8_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_u8_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Option<bool>
pub fn is_option_bool_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_bool_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Option<char>
pub fn is_option_char_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_char_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Vec<String>
pub fn is_vec_string_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Vec" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_string_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Option<SomeField> (not basic types)
pub fn is_option_field_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        // Check if it's not basic types (suggesting it's a Field type)
                        return !is_string_type(inner_ty)
                            && !is_naive_date_type(inner_ty)
                            && !is_u32_type(inner_ty)
                            && !is_u8_type(inner_ty)
                            && !is_f64_type(inner_ty)
                            && !is_char_type(inner_ty)
                            && !is_naive_time_type(inner_ty)
                            && !is_bool_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is a Field type (not basic types)
pub fn is_field_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = &segment.ident;
            // Check if it's not basic types (suggesting it's a Field type)
            return ident != "String"
                && ident != "NaiveDate"
                && ident != "u32"
                && ident != "u8"
                && ident != "f64"
                && ident != "char"
                && ident != "NaiveTime"
                && ident != "bool"
                && ident != "Vec"
                && ident != "Option";
        }
    }
    false
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_basic_type_detection() {
        let string_type: Type = parse_quote!(String);
        let option_string_type: Type = parse_quote!(Option<String>);
        let vec_string_type: Type = parse_quote!(Vec<String>);
        let naive_date_type: Type = parse_quote!(NaiveDate);
        let u32_type: Type = parse_quote!(u32);

        assert!(is_string_type(&string_type));
        assert!(is_option_type(&option_string_type));
        assert!(is_vec_type(&vec_string_type));
        assert!(is_naive_date_type(&naive_date_type));
        assert!(is_u32_type(&u32_type));
    }

    #[test]
    fn test_option_type_variants() {
        let option_string: Type = parse_quote!(Option<String>);
        let option_u32: Type = parse_quote!(Option<u32>);
        let option_naive_date: Type = parse_quote!(Option<NaiveDate>);

        assert!(is_option_string_type(&option_string));
        assert!(is_option_u32_type(&option_u32));
        assert!(is_option_naive_date_type(&option_naive_date));
    }

    #[test]
    fn test_field_type_detection() {
        let field_type: Type = parse_quote!(Field20);
        let string_type: Type = parse_quote!(String);
        let option_field_type: Type = parse_quote!(Option<Field20>);

        assert!(is_field_type(&field_type));
        assert!(!is_field_type(&string_type));
        assert!(is_option_field_type(&option_field_type));
    }

    #[test]
    fn test_inner_type_extraction() {
        let option_string: Type = parse_quote!(Option<String>);
        let vec_string: Type = parse_quote!(Vec<String>);
        let string_type: Type = parse_quote!(String);

        let inner_from_option = extract_inner_type(&option_string, true, false);
        let inner_from_vec = extract_inner_type(&vec_string, false, true);
        let inner_from_basic = extract_inner_type(&string_type, false, false);

        assert!(is_string_type(&inner_from_option));
        assert!(is_string_type(&inner_from_vec));
        assert!(is_string_type(&inner_from_basic));
    }
}
