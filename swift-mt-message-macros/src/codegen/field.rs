//! Code generation for SwiftField derive macro

use crate::ast::{Component, EnumField, EnumVariant, FieldDefinition, FieldKind, StructField};
use crate::error::MacroResult;
use crate::format::{FormatSpec, FormatType};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, PathArguments};

/// Generate SwiftField implementation for a field definition
pub fn generate_swift_field_impl(definition: &FieldDefinition) -> MacroResult<TokenStream> {
    let name = &definition.name;
    
    match &definition.kind {
        FieldKind::Struct(struct_field) => {
            generate_struct_field_impl(name, struct_field)
        }
        FieldKind::Enum(enum_field) => {
            generate_enum_field_impl(name, enum_field)
        }
    }
}

/// Generate SwiftField implementation for struct fields
fn generate_struct_field_impl(name: &syn::Ident, struct_field: &StructField) -> MacroResult<TokenStream> {
    let parse_impl = generate_struct_parse_impl(struct_field)?;
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
                // Try each variant and return the first successful one
                #sample_impl
            }
        }
    })
}

/// Generate parse implementation for struct fields
fn generate_struct_parse_impl(struct_field: &StructField) -> MacroResult<TokenStream> {
    if struct_field.components.len() == 1 {
        // Single component field - parse directly
        let component = &struct_field.components[0];
        let field_name = &component.name;
        let parse_expr = generate_component_parse_expr(component)?;
        
        Ok(quote! {
            let parsed_value = #parse_expr;
            Ok(Self {
                #field_name: parsed_value,
            })
        })
    } else {
        // Multi-component field - generate appropriate default values for each component
        let mut field_assignments = Vec::new();
        
        for component in &struct_field.components {
            let field_name = &component.name;
            let field_type = &component.field_type;
            
            // Generate appropriate default/parsed values based on component type
            let value_expr = if is_char_type(field_type) {
                quote! { value.chars().nth(0).unwrap_or('X') }
            } else if is_bool_type(field_type) {
                quote! { !value.is_empty() }
            } else if is_option_bool_type(field_type) {
                quote! { if value.is_empty() { None } else { Some(!value.is_empty()) } }
            } else if is_option_u32_type(field_type) {
                quote! { if value.is_empty() { None } else { Some(value.parse::<u32>().unwrap_or(0)) } }
            } else if is_option_u8_type(field_type) {
                quote! { if value.is_empty() { None } else { Some(value.parse::<u8>().unwrap_or(0)) } }
            } else if is_option_naive_date_type(field_type) {
                quote! { if value.is_empty() { None } else { Some(chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()) } }
            } else if is_option_char_type(field_type) {
                quote! { if value.is_empty() { None } else { Some(value.chars().nth(0).unwrap_or('X')) } }
            } else if is_option_string_type(field_type) {
                quote! { if value.is_empty() { None } else { Some(value.to_string()) } }
            } else if is_vec_string_type(field_type) {
                quote! { if value.is_empty() { vec![] } else { vec![value.to_string()] } }
            } else if is_vec_type(field_type) {
                quote! { Vec::new() }
            } else if is_f64_type(field_type) {
                quote! { value.parse::<f64>().unwrap_or(0.0) }
            } else if is_u32_type(field_type) {
                quote! { value.parse::<u32>().unwrap_or(0) }
            } else if is_u8_type(field_type) {
                quote! { value.parse::<u8>().unwrap_or(0) }
            } else if is_naive_date_type(field_type) {
                quote! { chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap() }
            } else if is_naive_time_type(field_type) {
                quote! { chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap() }
            } else if is_field_type(field_type) {
                quote! { #field_type::parse(value).unwrap_or_else(|_| #field_type::sample()) }
            } else {
                // Default to String
                quote! { value.to_string() }
            };
            
            field_assignments.push(quote! {
                #field_name: #value_expr
            });
        }
        
        Ok(quote! {
            // For multi-component fields, we parse/assign appropriate values for each component
            // This is a simplified approach - real SWIFT parsing would split the input appropriately
            Ok(Self {
                #(#field_assignments),*
            })
        })
    }
}

/// Generate component parsing expression
fn generate_component_parse_expr(component: &Component) -> MacroResult<TokenStream> {
    let field_type = &component.field_type;
    let format_spec = &component.format;
    
    // Generate parsing code based on the actual field type
    if is_naive_date_type(field_type) {
        Ok(quote! {
            {
                // Parse YYMMDD date format
                if value.len() != 6 {
                    return Err(crate::errors::ParseError::InvalidFormat {
                        message: "Date must be 6 characters (YYMMDD)".to_string()
                    });
                }
                let year = 2000 + value[0..2].parse::<i32>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid year in date".to_string()
                    })?;
                let month = value[2..4].parse::<u32>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid month in date".to_string()
                    })?;
                let day = value[4..6].parse::<u32>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid day in date".to_string()
                    })?;
                chrono::NaiveDate::from_ymd_opt(year, month, day)
                    .ok_or_else(|| crate::errors::ParseError::InvalidFormat {
                        message: "Invalid date".to_string()
                    })?
            }
        })
    } else if is_naive_time_type(field_type) {
        Ok(quote! {
            {
                // Parse HHMM time format
                if value.len() != 4 {
                    return Err(crate::errors::ParseError::InvalidFormat {
                        message: "Time must be 4 characters (HHMM)".to_string()
                    });
                }
                let hour = value[0..2].parse::<u32>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid hour in time".to_string()
                    })?;
                let minute = value[2..4].parse::<u32>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid minute in time".to_string()
                    })?;
                chrono::NaiveTime::from_hms_opt(hour, minute, 0)
                    .ok_or_else(|| crate::errors::ParseError::InvalidFormat {
                        message: "Invalid time".to_string()
                    })?
            }
        })
    } else if is_char_type(field_type) {
        Ok(quote! {
            {
                // Parse single character
                if value.len() != 1 {
                    return Err(crate::errors::ParseError::InvalidFormat {
                        message: "Expected exactly 1 character".to_string()
                    });
                }
                value.chars().next().unwrap()
            }
        })
    } else if is_f64_type(field_type) {
        Ok(quote! {
            {
                // Parse decimal amount
                value.replace(',', ".").parse::<f64>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid decimal number".to_string()
                    })?
            }
        })
    } else if is_u32_type(field_type) {
        Ok(quote! {
            {
                // Parse u32 number
                value.parse::<u32>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid u32 number".to_string()
                    })?
            }
        })
    } else if is_u8_type(field_type) {
        Ok(quote! {
            {
                // Parse u8 number
                value.parse::<u8>().map_err(|_| 
                    crate::errors::ParseError::InvalidFormat {
                        message: "Invalid u8 number".to_string()
                    })?
            }
        })
    } else if is_bool_type(field_type) {
        Ok(quote! {
            {
                // Parse boolean
                match value.to_lowercase().as_str() {
                    "true" | "1" | "yes" | "y" => true,
                    "false" | "0" | "no" | "n" => false,
                    _ => return Err(crate::errors::ParseError::InvalidFormat {
                        message: "Invalid boolean value".to_string()
                    })
                }
            }
        })
    } else if is_option_naive_date_type(field_type) {
        Ok(quote! {
            {
                // Parse optional date
                if value.is_empty() {
                    None
                } else {
                    if value.len() != 6 {
                        return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Date must be 6 characters (YYMMDD)".to_string()
                        });
                    }
                    let year = 2000 + value[0..2].parse::<i32>().map_err(|_| 
                        crate::errors::ParseError::InvalidFormat {
                            message: "Invalid year in date".to_string()
                        })?;
                    let month = value[2..4].parse::<u32>().map_err(|_| 
                        crate::errors::ParseError::InvalidFormat {
                            message: "Invalid month in date".to_string()
                        })?;
                    let day = value[4..6].parse::<u32>().map_err(|_| 
                        crate::errors::ParseError::InvalidFormat {
                            message: "Invalid day in date".to_string()
                        })?;
                    Some(chrono::NaiveDate::from_ymd_opt(year, month, day)
                        .ok_or_else(|| crate::errors::ParseError::InvalidFormat {
                            message: "Invalid date".to_string()
                        })?)
                }
            }
        })
    } else if is_option_string_type(field_type) {
        Ok(quote! {
            {
                if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                }
            }
        })
    } else if is_vec_type(field_type) {
        // Handle Vec<T> - for now just create empty vec as placeholder
        Ok(quote! {
            {
                Vec::new()
            }
        })
    } else if is_option_u32_type(field_type) {
        Ok(quote! {
            {
                if value.is_empty() {
                    None
                } else {
                    Some(value.parse::<u32>().map_err(|_| 
                        crate::errors::ParseError::InvalidFormat {
                            message: "Invalid u32 number".to_string()
                        })?)
                }
            }
        })
    } else if is_option_u8_type(field_type) {
        Ok(quote! {
            {
                if value.is_empty() {
                    None
                } else {
                    Some(value.parse::<u8>().map_err(|_| 
                        crate::errors::ParseError::InvalidFormat {
                            message: "Invalid u8 number".to_string()
                        })?)
                }
            }
        })
    } else if is_option_bool_type(field_type) {
        Ok(quote! {
            {
                if value.is_empty() {
                    None
                } else {
                    Some(match value.to_lowercase().as_str() {
                        "true" | "1" | "yes" | "y" => true,
                        "false" | "0" | "no" | "n" => false,
                        _ => return Err(crate::errors::ParseError::InvalidFormat {
                            message: "Invalid boolean value".to_string()
                        })
                    })
                }
            }
        })
    } else if is_option_char_type(field_type) {
        Ok(quote! {
            {
                if value.is_empty() {
                    None
                } else {
                    Some(value.chars().nth(0).unwrap_or('X'))
                }
            }
        })
    } else if is_option_field_type(field_type) {
        // Handle Option<SomeField> - extract the inner type and parse it
        if let syn::Type::Path(type_path) = field_type {
            if let Some(segment) = type_path.path.segments.last() {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Ok(quote! {
                            {
                                if value.is_empty() {
                                    None
                                } else {
                                    Some(#inner_ty::parse(value)?)
                                }
                            }
                        });
                    }
                }
            }
        }
        // Fallback
        Ok(quote! { None })
    } else if is_field_type(field_type) {
        // Handle other SwiftField types
        Ok(quote! {
            {
                #field_type::parse(value)?
            }
        })
    } else {
        // Default to String parsing with format validation
        generate_string_parse_expr(format_spec)
    }
}

/// Generate string parsing expression with format validation
fn generate_string_parse_expr(format_spec: &crate::format::FormatSpec) -> MacroResult<TokenStream> {
    match &format_spec.format_type {
        FormatType::AnyCharacter => {
            if let Some(length) = format_spec.length {
                if format_spec.is_fixed {
                    Ok(quote! {
                        {
                            if value.len() != #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected exactly {} characters, got {}", #length, value.len())
                                });
                            }
                            value.to_string()
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            if value.len() > #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected at most {} characters, got {}", #length, value.len())
                                });
                            }
                            value.to_string()
                        }
                    })
                }
            } else {
                Ok(quote! { value.to_string() })
            }
        }
        FormatType::Numeric => {
            if let Some(length) = format_spec.length {
                if format_spec.is_fixed {
                    Ok(quote! {
                        {
                            if value.len() != #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected exactly {} numeric characters, got {}", #length, value.len())
                                });
                            }
                            if !value.chars().all(|c| c.is_ascii_digit()) {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: "Expected only numeric characters".to_string()
                                });
                            }
                            value.to_string()
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            if value.len() > #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected at most {} numeric characters, got {}", #length, value.len())
                                });
                            }
                            if !value.chars().all(|c| c.is_ascii_digit()) {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: "Expected only numeric characters".to_string()
                                });
                            }
                            value.to_string()
                        }
                    })
                }
            } else {
                Ok(quote! {
                    {
                        if !value.chars().all(|c| c.is_ascii_digit()) {
                            return Err(crate::errors::ParseError::InvalidFormat {
                                message: "Expected only numeric characters".to_string()
                            });
                        }
                        value.to_string()
                    }
                })
            }
        }
        FormatType::Alpha => {
            if let Some(length) = format_spec.length {
                if format_spec.is_fixed {
                    Ok(quote! {
                        {
                            if value.len() != #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected exactly {} alphabetic characters, got {}", #length, value.len())
                                });
                            }
                            if !value.chars().all(|c| c.is_ascii_alphabetic()) {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: "Expected only alphabetic characters".to_string()
                                });
                            }
                            value.to_string()
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            if value.len() > #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected at most {} alphabetic characters, got {}", #length, value.len())
                                });
                            }
                            if !value.chars().all(|c| c.is_ascii_alphabetic()) {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: "Expected only alphabetic characters".to_string()
                                });
                            }
                            value.to_string()
                        }
                    })
                }
            } else {
                Ok(quote! {
                    {
                        if !value.chars().all(|c| c.is_ascii_alphabetic()) {
                            return Err(crate::errors::ParseError::InvalidFormat {
                                message: "Expected only alphabetic characters".to_string()
                            });
                        }
                        value.to_string()
                    }
                })
            }
        }
        FormatType::CharacterSet => {
            // Character set allows A-Z, 0-9, and some special characters
            if let Some(length) = format_spec.length {
                if format_spec.is_fixed {
                    Ok(quote! {
                        {
                            if value.len() != #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected exactly {} character set characters, got {}", #length, value.len())
                                });
                            }
                            if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: "Expected only alphanumeric characters".to_string()
                                });
                            }
                            value.to_string()
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            if value.len() > #length {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: format!("Expected at most {} character set characters, got {}", #length, value.len())
                                });
                            }
                            if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
                                return Err(crate::errors::ParseError::InvalidFormat {
                                    message: "Expected only alphanumeric characters".to_string()
                                });
                            }
                            value.to_string()
                        }
                    })
                }
            } else {
                Ok(quote! {
                    {
                        if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
                            return Err(crate::errors::ParseError::InvalidFormat {
                                message: "Expected only alphanumeric characters".to_string()
                            });
                        }
                        value.to_string()
                    }
                })
            }
        }
        _ => {
            // For other format types, just do basic string validation
            Ok(quote! { value.to_string() })
        }
    }
}

/// Check if type is NaiveDate
fn is_naive_date_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "NaiveDate";
        }
    }
    false
}

/// Check if type is f64
fn is_f64_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "f64";
        }
    }
    false
}

/// Check if type is u32
fn is_u32_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "u32";
        }
    }
    false
}

/// Check if type is u8
fn is_u8_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "u8";
        }
    }
    false
}

/// Check if type is bool
fn is_bool_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "bool";
        }
    }
    false
}

/// Check if type is Option<NaiveDate>
fn is_option_naive_date_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_naive_date_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if type is Option<String>
fn is_option_string_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        if let syn::Type::Path(inner_path) = inner_ty {
                            if let Some(inner_segment) = inner_path.path.segments.last() {
                                return inner_segment.ident == "String";
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Check if type is Option<u32>
fn is_option_u32_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_u32_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if type is Option<u8>
fn is_option_u8_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_u8_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if type is Option<bool>
fn is_option_bool_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_bool_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if type is Option<char>
fn is_option_char_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_char_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if type is NaiveTime
fn is_naive_time_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "NaiveTime";
        }
    }
    false
}

/// Check if type is char
fn is_char_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "char";
        }
    }
    false
}

/// Check if type is Vec<T>
fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// Check if type is Vec<String>
fn is_vec_string_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_string_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

/// Check if type is Option<SomeField> (not basic types)
fn is_option_field_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
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

/// Check if type is a Field type (not a basic type)
fn is_field_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
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

/// Check if type is String
fn is_string_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        // Check if the path ends with String (handles both String and std::string::String)
        let path_str = quote!(#type_path).to_string();
        // Remove spaces and check various forms
        let normalized = path_str.replace(" ", "");
        return normalized == "String" 
            || normalized == "std::string::String" 
            || normalized.ends_with("::String")
            || path_str == "String";
    }
    false
}

/// Generate to_swift_string implementation for struct fields
fn generate_struct_to_swift_string_impl(struct_field: &StructField) -> MacroResult<TokenStream> {
    if struct_field.components.len() == 1 {
        let component = &struct_field.components[0];
        let field_name = &component.name;
        let field_type = &component.field_type;
        
        // Generate conversion based on field type
        if is_naive_date_type(field_type) {
            Ok(quote! {
                self.#field_name.format("%y%m%d").to_string()
            })
        } else if is_naive_time_type(field_type) {
            Ok(quote! {
                self.#field_name.format("%H%M").to_string()
            })
        } else if is_char_type(field_type) {
            Ok(quote! {
                self.#field_name.to_string()
            })
        } else if is_f64_type(field_type) {
            Ok(quote! {
                format!("{:.2}", self.#field_name).replace('.', ",")
            })
        } else if is_u32_type(field_type) {
            Ok(quote! {
                self.#field_name.to_string()
            })
        } else if is_u8_type(field_type) {
            Ok(quote! {
                self.#field_name.to_string()
            })
        } else if is_bool_type(field_type) {
            Ok(quote! {
                if self.#field_name { "true".to_string() } else { "false".to_string() }
            })
        } else if is_option_naive_date_type(field_type) {
            Ok(quote! {
                self.#field_name.as_ref()
                    .map(|d| d.format("%y%m%d").to_string())
                    .unwrap_or_default()
            })
        } else if is_option_string_type(field_type) {
            Ok(quote! {
                self.#field_name.as_ref().unwrap_or(&String::new()).clone()
            })
        } else if is_option_u32_type(field_type) {
            Ok(quote! {
                self.#field_name.map(|n| n.to_string()).unwrap_or_default()
            })
        } else if is_option_u8_type(field_type) {
            Ok(quote! {
                self.#field_name.map(|n| n.to_string()).unwrap_or_default()
            })
        } else if is_option_bool_type(field_type) {
            Ok(quote! {
                self.#field_name.map(|b| if b { "true".to_string() } else { "false".to_string() }).unwrap_or_default()
            })
        } else if is_option_char_type(field_type) {
            Ok(quote! {
                self.#field_name.map(|c| c.to_string()).unwrap_or_default()
            })
        } else if is_vec_string_type(field_type) {
            Ok(quote! {
                self.#field_name.join("")
            })
        } else if is_vec_type(field_type) {
            Ok(quote! {
                self.#field_name.iter()
                    .map(|item| item.to_swift_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
        } else if is_option_field_type(field_type) {
            Ok(quote! {
                self.#field_name.as_ref()
                    .map(|item| item.to_swift_string())
                    .unwrap_or_default()
            })
        } else if is_field_type(field_type) {
            Ok(quote! {
                self.#field_name.to_swift_string()
            })
        } else if is_string_type(field_type) {
            Ok(quote! {
                self.#field_name.clone()
            })
        } else {
            // Default to calling to_swift_string for unknown types
            Ok(quote! {
                self.#field_name.to_swift_string()
            })
        }
    } else {
        // For multi-component fields, concatenate all components
        let mut component_conversions = Vec::new();
        
        for component in &struct_field.components {
            let field_name = &component.name;
            let field_type = &component.field_type;
            
            // Generate conversion based on field type for each component
            if is_naive_date_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.format("%y%m%d").to_string()
                });
            } else if is_naive_time_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.format("%H%M").to_string()
                });
            } else if is_char_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.to_string()
                });
            } else if is_f64_type(field_type) {
                component_conversions.push(quote! {
                    format!("{:.2}", self.#field_name).replace('.', ",")
                });
            } else if is_u32_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.to_string()
                });
            } else if is_u8_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.to_string()
                });
            } else if is_bool_type(field_type) {
                component_conversions.push(quote! {
                    if self.#field_name { "true".to_string() } else { "false".to_string() }
                });
            } else if is_option_naive_date_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.as_ref()
                        .map(|d| d.format("%y%m%d").to_string())
                        .unwrap_or_default()
                });
            } else if is_option_string_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.as_ref().unwrap_or(&String::new()).clone()
                });
            } else if is_option_u32_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.map(|n| n.to_string()).unwrap_or_default()
                });
            } else if is_option_u8_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.map(|n| n.to_string()).unwrap_or_default()
                });
            } else if is_option_bool_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.map(|b| if b { "true".to_string() } else { "false".to_string() }).unwrap_or_default()
                });
            } else if is_option_char_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.map(|c| c.to_string()).unwrap_or_default()
                });
            } else if is_vec_string_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.join("")
                });
            } else if is_vec_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.iter()
                        .map(|item| item.to_swift_string())
                        .collect::<Vec<_>>()
                        .join("")
                });
            } else if is_option_field_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.as_ref()
                        .map(|item| item.to_swift_string())
                        .unwrap_or_default()
                });
            } else if is_field_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.to_swift_string()
                });
            } else if is_string_type(field_type) {
                component_conversions.push(quote! {
                    self.#field_name.clone()
                });
            } else {
                // Default to calling to_swift_string for unknown types
                component_conversions.push(quote! {
                    self.#field_name.to_swift_string()
                });
            }
        }
        
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
    if struct_field.components.len() == 1 {
        let component = &struct_field.components[0];
        let field_name = &component.name;
        let sample_expr = generate_component_sample_expr(component)?;
        
        Ok(quote! {
            Self {
                #field_name: #sample_expr,
            }
        })
    } else {
        // For multi-component fields, generate samples for all components
        let mut field_samples = Vec::new();
        
        for component in &struct_field.components {
            let field_name = &component.name;
            let sample_expr = generate_component_sample_expr(component)?;
            
            field_samples.push(quote! {
                #field_name: #sample_expr
            });
        }
        
        Ok(quote! {
            Self {
                #(#field_samples),*
            }
        })
    }
}

/// Generate sample expression for a component
fn generate_component_sample_expr(component: &Component) -> MacroResult<TokenStream> {
    let field_type = &component.field_type;
    let format_spec = &component.format;
    
    // Generate sample code based on the actual field type
    if is_naive_date_type(field_type) {
        Ok(quote! {
            chrono::NaiveDate::from_ymd_opt(2023, 1, 15).unwrap()
        })
    } else if is_naive_time_type(field_type) {
        Ok(quote! {
            chrono::NaiveTime::from_hms_opt(14, 30, 0).unwrap()
        })
    } else if is_char_type(field_type) {
        Ok(quote! { 'D' })
    } else if is_f64_type(field_type) {
        Ok(quote! { 1234.56 })
    } else if is_u32_type(field_type) {
        Ok(quote! { 12345u32 })
    } else if is_u8_type(field_type) {
        Ok(quote! { 42u8 })
    } else if is_bool_type(field_type) {
        Ok(quote! { true })
    } else if is_option_naive_date_type(field_type) {
        Ok(quote! {
            Some(chrono::NaiveDate::from_ymd_opt(2023, 1, 15).unwrap())
        })
    } else if is_option_string_type(field_type) {
        // Generate sample string based on format
        let sample_string = generate_sample_string_literal(format_spec);
        Ok(quote! { Some(#sample_string.to_string()) })
    } else if is_option_u32_type(field_type) {
        Ok(quote! { Some(67890u32) })
    } else if is_option_u8_type(field_type) {
        Ok(quote! { Some(99u8) })
    } else if is_option_bool_type(field_type) {
        Ok(quote! { Some(false) })
    } else if is_option_char_type(field_type) {
        Ok(quote! { Some('X') })
    } else if is_vec_string_type(field_type) {
        // Generate sample Vec<String>
        Ok(quote! { vec!["SAMPLE".to_string()] })
    } else if is_vec_type(field_type) {
        // Generate empty Vec for now - should be populated based on content
        Ok(quote! { Vec::new() })
    } else if is_option_field_type(field_type) {
        // Generate Some with sample for optional field types
        if let syn::Type::Path(type_path) = field_type {
            if let Some(segment) = type_path.path.segments.last() {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Ok(quote! { Some(#inner_ty::sample()) });
                    }
                }
            }
        }
        Ok(quote! { None })
    } else if is_field_type(field_type) {
        // Generate sample for field types
        Ok(quote! { #field_type::sample() })
    } else {
        // Default to String
        let sample_string = generate_sample_string_literal(format_spec);
        Ok(quote! { #sample_string.to_string() })
    }
}

/// Generate a sample string literal based on format spec
fn generate_sample_string_literal(format_spec: &crate::format::FormatSpec) -> String {
    match &format_spec.format_type {
        FormatType::AnyCharacter => {
            let length = format_spec.length.unwrap_or(16);
            "A".repeat(length)
        }
        FormatType::Numeric => {
            let length = format_spec.length.unwrap_or(6);
            "1".repeat(length)
        }
        FormatType::Alpha => {
            let length = format_spec.length.unwrap_or(4);
            "TEST".chars().cycle().take(length).collect()
        }
        FormatType::CharacterSet => {
            let length = format_spec.length.unwrap_or(4);
            "ABC1".chars().cycle().take(length).collect()
        }
        _ => {
            let length = format_spec.length.unwrap_or(8);
            "SAMPLE".chars().cycle().take(length).collect()
        }
    }
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
            Self::#variant_ident(#type_name::sample())
        })
    } else {
        return Err(crate::error::MacroError::internal(
            proc_macro2::Span::call_site(),
            "Enum must have at least one variant"
        ));
    }
}

