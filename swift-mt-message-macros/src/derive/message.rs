use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input, Data, Fields, Type, Attribute, Lit, Meta, PathArguments, GenericArgument};
use quote::quote;

use crate::utils::*;

/// Derive macro for SwiftMessage trait implementation
pub fn derive_swift_message_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract message type from #[swift_message(mt = "103")]
    let message_type = extract_message_type_attribute(&input.attrs)
        .expect("SwiftMessage requires #[swift_message(mt = \"...\")]");

    // Parse struct fields to extract field information
    let (required_fields, optional_fields, field_mappings) = extract_field_info(&input);

    // Generate required fields list
    let required_fields_list = required_fields.iter().map(|field| quote! { #field });
    
    // Generate optional fields list  
    let optional_fields_list = optional_fields.iter().map(|field| quote! { #field });

    // Generate from_fields parsing logic
    let from_fields_impl = generate_from_fields_logic(&field_mappings);

    // Generate to_fields serialization logic
    let to_fields_impl = generate_to_fields_logic(&field_mappings);

    let expanded = quote! {
        impl crate::SwiftMessageBody for #name {
            fn message_type() -> &'static str {
                #message_type
            }

            fn from_fields(fields: std::collections::HashMap<String, Vec<String>>) -> crate::Result<Self> {
                #from_fields_impl
            }

            fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
                #to_fields_impl
            }

            fn required_fields() -> Vec<&'static str> {
                vec![ #( #required_fields_list ),* ]
            }

            fn optional_fields() -> Vec<&'static str> {
                vec![ #( #optional_fields_list ),* ]
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extract field information from struct
fn extract_field_info(input: &DeriveInput) -> (Vec<String>, Vec<String>, Vec<FieldMapping>) {
    let mut required_fields = Vec::new();
    let mut optional_fields = Vec::new();
    let mut field_mappings = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                
                // Extract field tag from #[field("XX")] attribute
                if let Some(field_tag) = extract_field_tag(&field.attrs) {
                    let is_optional = is_option_type(field_type);
                    
                    if is_optional {
                        optional_fields.push(field_tag.clone());
                    } else {
                        required_fields.push(field_tag.clone());
                    }
                    
                    field_mappings.push(FieldMapping {
                        field_name: field_name.clone(),
                        field_tag,
                        field_type: field_type.clone(),
                        is_optional,
                    });
                }
            }
        }
    }

    (required_fields, optional_fields, field_mappings)
}

/// Extract field tag from #[field("XX")] attribute
fn extract_field_tag(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("field") {
                if let Ok(lit) = meta_list.parse_args::<Lit>() {
                    if let Lit::Str(lit_str) = lit {
                        return Some(lit_str.value());
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
            if segment.ident == "Option" {
                return true;
            }
        }
    }
    false
}

/// Field mapping information
struct FieldMapping {
    field_name: syn::Ident,
    field_tag: String,
    field_type: Type,
    is_optional: bool,
}

/// Generate from_fields parsing logic
fn generate_from_fields_logic(field_mappings: &[FieldMapping]) -> proc_macro2::TokenStream {
    let field_parsing = field_mappings.iter().map(|mapping| {
        let field_name = &mapping.field_name;
        let field_tag = &mapping.field_tag;
        
        if mapping.is_optional {
            // Check if it's a Vec type inside Option
            if is_vec_inside_option(&mapping.field_type) {
                quote! {
                    let #field_name = if let Some(field_values) = fields.get(#field_tag) {
                        if !field_values.is_empty() {
                            let mut parsed_values = Vec::new();
                            for value in field_values {
                                parsed_values.push(crate::SwiftField::parse(value)?);
                            }
                            Some(parsed_values)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                }
            } else {
                quote! {
                    let #field_name = if let Some(field_values) = fields.get(#field_tag) {
                        if let Some(first_value) = field_values.first() {
                            Some(crate::SwiftField::parse(first_value)?)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                }
            }
        } else {
            // Check if it's a direct Vec type (not wrapped in Option)
            if is_vec_type(&mapping.field_type) {
                quote! {
                    let #field_name = if let Some(field_values) = fields.get(#field_tag) {
                        let mut parsed_values = Vec::new();
                        for value in field_values {
                            parsed_values.push(crate::SwiftField::parse(value)?);
                        }
                        parsed_values
                    } else {
                        Vec::new() // Default to empty Vec for required Vec fields
                    };
                }
            } else {
                quote! {
                    let #field_name = fields.get(#field_tag)
                        .and_then(|values| values.first())
                        .ok_or_else(|| crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: format!("Required field {} is missing", #field_tag),
                        })
                        .and_then(|value| crate::SwiftField::parse(value))?;
                }
            }
        }
    });

    let field_names = field_mappings.iter().map(|mapping| &mapping.field_name);

    quote! {
        use crate::SwiftField;
        
        #( #field_parsing )*
        
        Ok(Self {
            #( #field_names ),*
        })
    }
}

/// Generate to_fields serialization logic
fn generate_to_fields_logic(field_mappings: &[FieldMapping]) -> proc_macro2::TokenStream {
    let field_serialization = field_mappings.iter().map(|mapping| {
        let field_name = &mapping.field_name;
        let field_tag = &mapping.field_tag;
        
        if mapping.is_optional {
            // Check if it's a Vec type inside Option
            if is_vec_inside_option(&mapping.field_type) {
                quote! {
                    if let Some(ref field_values) = self.#field_name {
                        let mut serialized_values = Vec::new();
                        for field_value in field_values {
                            serialized_values.push(field_value.to_swift_string());
                        }
                        fields.insert(#field_tag.to_string(), serialized_values);
                    }
                }
            } else {
                quote! {
                    if let Some(ref field_value) = self.#field_name {
                        fields.insert(#field_tag.to_string(), vec![field_value.to_swift_string()]);
                    }
                }
            }
        } else {
            // Check if it's a direct Vec type (not wrapped in Option)
            if is_vec_type(&mapping.field_type) {
                quote! {
                    if !self.#field_name.is_empty() {
                        let mut serialized_values = Vec::new();
                        for field_value in &self.#field_name {
                            serialized_values.push(field_value.to_swift_string());
                        }
                        fields.insert(#field_tag.to_string(), serialized_values);
                    }
                }
            } else {
                quote! {
                    fields.insert(#field_tag.to_string(), vec![self.#field_name.to_swift_string()]);
                }
            }
        }
    });

    quote! {
        use crate::SwiftField;
        
        let mut fields = std::collections::HashMap::new();
        #( #field_serialization )*
        fields
    }
}

/// Check if a type is Vec<T>
fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// Check if a type is Option<Vec<T>>
fn is_vec_inside_option(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        if let Type::Path(inner_path) = inner_type {
                            if let Some(inner_segment) = inner_path.path.segments.last() {
                                return inner_segment.ident == "Vec";
                            }
                        }
                    }
                }
            }
        }
    }
    false
} 