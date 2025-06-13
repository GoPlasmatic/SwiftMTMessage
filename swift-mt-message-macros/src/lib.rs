use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, GenericArgument, Meta, PathArguments, Type,
    parse_macro_input,
};

/// Derive macro for SwiftField trait implementation
#[proc_macro_derive(SwiftField, attributes(format, field_option))]
pub fn derive_swift_field(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    // Handle struct with named fields
                    let field_parsing = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        let _field_type = &field.ty;

                        // Look for #[format("...")] attribute
                        let format_spec = extract_format_attribute(&field.attrs);

                        match format_spec.as_deref() {
                            Some("16x") => quote! {
                                #field_name: content.trim().to_string()
                            },
                            Some("4*35x") => quote! {
                                #field_name: content.split('\n').map(|s| s.trim().to_string()).collect()
                            },
                            Some("BIC") => quote! {
                                #field_name: crate::common::BIC::new(content.trim().to_string())
                            },
                            Some("structured_party_identifier") => quote! {
                                #field_name: content.trim().to_string() // TODO: implement proper parsing
                            },
                            Some(spec) if spec.ends_with("x") => quote! {
                                #field_name: content.trim().to_string()
                            },
                            _ => quote! {
                                #field_name: content.trim().to_string()
                            }
                        }
                    });

                    let validation_logic = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        let format_spec = extract_format_attribute(&field.attrs);

                        match format_spec.as_deref() {
                            Some("16x") => quote! {
                                if self.#field_name.len() > 16 {
                                    errors.push(crate::ValidationError::LengthValidation {
                                        field_tag: stringify!(#name).to_string(),
                                        expected: "max 16 characters".to_string(),
                                        actual: self.#field_name.len(),
                                    });
                                }
                            },
                            Some("BIC") => quote! {
                                if !self.#field_name.validate() {
                                    errors.push(crate::ValidationError::FormatValidation {
                                        field_tag: stringify!(#name).to_string(),
                                        message: "Invalid BIC format".to_string(),
                                    });
                                }
                            },
                            _ => quote! {
                                // Basic validation
                                if self.#field_name.is_empty() {
                                    errors.push(crate::ValidationError::ValueValidation {
                                        field_tag: stringify!(#name).to_string(),
                                        message: "Field cannot be empty".to_string(),
                                    });
                                }
                            },
                        }
                    });

                    let format_spec = extract_format_attribute_from_struct(&input.attrs)
                        .unwrap_or_else(|| "custom".to_string());

                    // Extract field tag from struct name (e.g., "Field20" -> "20")
                    let field_tag = name
                        .to_string()
                        .strip_prefix("Field")
                        .unwrap_or(&name.to_string())
                        .to_uppercase();

                    // Generate to_swift_string implementation based on field structure
                    let to_swift_string_impl = if fields.named.len() == 1 {
                        // Single field - handle different types appropriately
                        let field = fields.named.first().unwrap();
                        let field_name = &field.ident;
                        let _field_type = &field.ty;
                        let format_spec = extract_format_attribute(&field.attrs);

                        match format_spec.as_deref() {
                            Some("BIC") => quote! {
                                fn to_swift_string(&self) -> String {
                                    format!(":{}:{}", #field_tag, self.#field_name.value)
                                }
                            },
                            Some("4*35x") => quote! {
                                fn to_swift_string(&self) -> String {
                                    format!(":{}:{}", #field_tag, self.#field_name.join("\n"))
                                }
                            },
                            _ => quote! {
                                fn to_swift_string(&self) -> String {
                                    format!(":{}:{}", #field_tag, self.#field_name)
                                }
                            },
                        }
                    } else {
                        // Multiple fields - need custom logic per field type
                        quote! {
                            fn to_swift_string(&self) -> String {
                                // Multi-field implementation - customize per field type
                                format!(":{}:{:?}", #field_tag, self)
                            }
                        }
                    };

                    let expanded = quote! {
                        impl crate::SwiftField for #name {
                            fn parse(value: &str) -> crate::Result<Self> {
                                let value = value.trim();

                                // Handle input that includes field tag prefix
                                let content = if value.starts_with(&format!(":{}:", #field_tag)) {
                                    &value[#field_tag.len() + 2..]  // Remove ":TAG:" prefix
                                } else if value.starts_with(&format!("{}:", #field_tag)) {
                                    &value[#field_tag.len() + 1..]  // Remove "TAG:" prefix
                                } else {
                                    value  // Use as-is if no prefix
                                };

                                Ok(Self {
                                    #(#field_parsing,)*
                                })
                            }

                            #to_swift_string_impl

                            fn validate(&self) -> crate::ValidationResult {
                                let mut errors = Vec::new();
                                #(#validation_logic)*

                                crate::ValidationResult {
                                    is_valid: errors.is_empty(),
                                    errors,
                                    warnings: Vec::new(),
                                }
                            }

                            fn format_spec() -> &'static str {
                                #format_spec
                            }
                        }
                    };

                    TokenStream::from(expanded)
                }
                _ => {
                    panic!("SwiftField can only be derived for structs with named fields");
                }
            }
        }
        Data::Enum(data) => {
            // Handle enum with field options
            let variants = &data.variants;

            let parse_arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let option_letter = extract_field_option_attribute(&variant.attrs)
                    .unwrap_or_else(|| variant_name.to_string());

                // Get the type inside the variant
                if let syn::Fields::Unnamed(fields) = &variant.fields {
                    if let Some(field) = fields.unnamed.first() {
                        let field_type = &field.ty;
                        quote! {
                            #option_letter => {
                                let inner = <#field_type>::parse(&value[1..])?;
                                Ok(#name::#variant_name(inner))
                            }
                        }
                    } else {
                        quote! {
                            #option_letter => Ok(#name::#variant_name)
                        }
                    }
                } else {
                    quote! {
                        #option_letter => Ok(#name::#variant_name)
                    }
                }
            });

            let to_string_arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let option_letter = extract_field_option_attribute(&variant.attrs)
                    .unwrap_or_else(|| variant_name.to_string());

                if let syn::Fields::Unnamed(fields) = &variant.fields {
                    if fields.unnamed.len() > 0 {
                        quote! {
                            #name::#variant_name(inner) => {
                                format!("{}{}", #option_letter, inner.to_swift_string())
                            }
                        }
                    } else {
                        quote! {
                            #name::#variant_name => #option_letter.to_string()
                        }
                    }
                } else {
                    quote! {
                        #name::#variant_name => #option_letter.to_string()
                    }
                }
            });

            let validate_arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;

                if let syn::Fields::Unnamed(fields) = &variant.fields {
                    if fields.unnamed.len() > 0 {
                        quote! {
                            #name::#variant_name(inner) => inner.validate()
                        }
                    } else {
                        quote! {
                            #name::#variant_name => crate::ValidationResult::valid()
                        }
                    }
                } else {
                    quote! {
                        #name::#variant_name => crate::ValidationResult::valid()
                    }
                }
            });

            let expanded = quote! {
                impl crate::SwiftField for #name {
                    fn parse(value: &str) -> crate::Result<Self> {
                        let option = value.chars().next().map(|c| c.to_string()).unwrap_or_default();

                        match option.as_str() {
                            #(#parse_arms)*
                            _ => Err(crate::ParseError::InvalidFieldFormat {
                                field_tag: stringify!(#name).to_string(),
                                message: format!("Unknown option: {:?}", option),
                            })
                        }
                    }

                    fn to_swift_string(&self) -> String {
                        match self {
                            #(#to_string_arms)*
                        }
                    }

                    fn validate(&self) -> crate::ValidationResult {
                        match self {
                            #(#validate_arms)*
                        }
                    }

                    fn format_spec() -> &'static str {
                        "option"
                    }
                }
            };

            TokenStream::from(expanded)
        }
        Data::Union(_) => {
            panic!("SwiftField cannot be derived for unions");
        }
    }
}

/// Attribute macro for adding serde rename attributes based on field tags
#[proc_macro_attribute]
pub fn swift_serde(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    match &mut input.data {
        Data::Struct(data) => {
            match &mut data.fields {
                Fields::Named(fields) => {
                    // Add serde rename attributes to each field based on their field tag
                    for field in &mut fields.named {
                        let field_tag = extract_field_attribute(&field.attrs)
                            .expect("All fields must have #[field(\"tag\")]");

                        // Check if serde rename already exists
                        let has_serde_rename = field.attrs.iter().any(|attr| {
                            if attr.path().is_ident("serde") {
                                if let Meta::List(meta_list) = &attr.meta {
                                    return meta_list.tokens.to_string().contains("rename");
                                }
                            }
                            false
                        });

                        // Add serde rename attribute if it doesn't exist
                        if !has_serde_rename {
                            let serde_rename: Attribute = syn::parse_quote! {
                                #[serde(rename = #field_tag)]
                            };
                            field.attrs.push(serde_rename);
                        }
                    }

                    TokenStream::from(quote! { #input })
                }
                _ => {
                    panic!("swift_serde can only be applied to structs with named fields");
                }
            }
        }
        _ => {
            panic!("swift_serde can only be applied to structs");
        }
    }
}

/// Derive macro for SwiftMessage trait implementation
#[proc_macro_derive(SwiftMessage, attributes(swift_message, field))]
pub fn derive_swift_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract message type from #[swift_message(mt = "103")]
    let message_type = extract_message_type_attribute(&input.attrs)
        .expect("SwiftMessage requires #[swift_message(mt = \"...\")]");

    match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    // Separate required and optional fields
                    let mut required_field_parsing = Vec::new();
                    let mut optional_field_parsing = Vec::new();
                    let mut required_field_serialization = Vec::new();
                    let mut optional_field_serialization = Vec::new();
                    let mut required_field_tags = Vec::new();
                    let mut optional_field_tags = Vec::new();

                    for field in &fields.named {
                        let field_name = &field.ident;
                        let field_type = &field.ty;
                        let field_tag = extract_field_attribute(&field.attrs)
                            .expect("All fields must have #[field(\"tag\")]");

                        if is_option_type(field_type) {
                            // Optional field
                            let inner_type = extract_option_inner_type(field_type)
                                .expect("Failed to extract inner type from Option");

                            optional_field_parsing.push(quote! {
                                #field_name: if let Some(field_value) = fields.get(#field_tag) {
                                    Some(<#inner_type as crate::SwiftField>::parse(field_value)?)
                                } else {
                                    None
                                }
                            });

                            optional_field_serialization.push(quote! {
                                if let Some(ref field_value) = self.#field_name {
                                    fields.insert(#field_tag.to_string(), crate::SwiftField::to_swift_string(field_value));
                                }
                            });

                            optional_field_tags.push(quote! { #field_tag });
                        } else {
                            // Required field
                            required_field_parsing.push(quote! {
                                #field_name: <#field_type as crate::SwiftField>::parse(fields.get(#field_tag)
                                    .ok_or_else(|| crate::ParseError::MissingRequiredField {
                                        field_tag: #field_tag.to_string(),
                                    })?)?
                            });

                            required_field_serialization.push(quote! {
                                fields.insert(#field_tag.to_string(), crate::SwiftField::to_swift_string(&self.#field_name));
                            });

                            required_field_tags.push(quote! { #field_tag });
                        }
                    }

                    // Combine all field parsing
                    let all_field_parsing = required_field_parsing
                        .into_iter()
                        .chain(optional_field_parsing.into_iter());

                    // Combine all field serialization
                    let all_field_serialization = required_field_serialization
                        .into_iter()
                        .chain(optional_field_serialization.into_iter());

                    let expanded = quote! {
                        impl crate::SwiftMessageBody for #name {
                            fn message_type() -> &'static str {
                                #message_type
                            }

                            fn from_fields(fields: std::collections::HashMap<String, String>) -> crate::Result<Self> {
                                Ok(Self {
                                    #(#all_field_parsing,)*
                                })
                            }

                            fn to_fields(&self) -> std::collections::HashMap<String, String> {
                                let mut fields = std::collections::HashMap::new();
                                #(#all_field_serialization)*
                                fields
                            }

                            fn required_fields() -> Vec<&'static str> {
                                vec![#(#required_field_tags),*]
                            }

                            fn optional_fields() -> Vec<&'static str> {
                                vec![#(#optional_field_tags),*]
                            }
                        }
                    };

                    TokenStream::from(expanded)
                }
                _ => {
                    panic!("SwiftMessage can only be derived for structs with named fields");
                }
            }
        }
        _ => {
            panic!("SwiftMessage can only be derived for structs");
        }
    }
}

// Helper functions to extract attributes and handle Option types

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Extract the inner type T from Option<T>
fn extract_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

fn extract_format_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("format") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Some(nested) = meta_list.tokens.to_string().strip_prefix('"') {
                    if let Some(value) = nested.strip_suffix('"') {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

fn extract_format_attribute_from_struct(attrs: &[Attribute]) -> Option<String> {
    extract_format_attribute(attrs)
}

fn extract_field_option_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("field_option") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Some(nested) = meta_list.tokens.to_string().strip_prefix('"') {
                    if let Some(value) = nested.strip_suffix('"') {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

fn extract_message_type_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("swift_message") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                // Parse: mt = "103"
                if let Some(eq_pos) = tokens.find('=') {
                    let value_part = tokens[eq_pos + 1..].trim();
                    if let Some(nested) = value_part.strip_prefix('"') {
                        if let Some(value) = nested.strip_suffix('"') {
                            return Some(value.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn extract_field_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("field") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Some(nested) = meta_list.tokens.to_string().strip_prefix('"') {
                    if let Some(value) = nested.strip_suffix('"') {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}
