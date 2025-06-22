use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Meta, Type, parse_macro_input};

use crate::component::parser::{
    ComponentSpec, FieldSpec, is_swift_message_type, is_vec_of_swift_messages, parse_field_specs,
};
use crate::utils::attributes::extract_message_type_attribute;

/// Derive macro for SwiftMessage trait implementation
pub fn derive_swift_message_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract message type from attribute first, then fall back to struct name
    let message_type = extract_message_type_attribute(&input.attrs)
        .unwrap_or_else(|| extract_message_type_from_name(&name.to_string()));

    // Extract validation rules if present
    let validation_rules = extract_validation_rules_attribute(&input.attrs);

    // Parse struct fields to extract field information including sequences
    let (required_fields, optional_fields, field_mappings, sequence_mappings) =
        extract_field_info(&input);

    // Generate required fields list
    let required_fields_list = required_fields.iter().map(|field| quote! { #field });

    // Generate optional fields list
    let optional_fields_list = optional_fields.iter().map(|field| quote! { #field });

    // Generate from_fields parsing logic
    let from_fields_impl = generate_from_fields_logic(&field_mappings, &sequence_mappings);

    // Generate to_fields serialization logic
    let to_fields_impl = generate_to_fields_logic(&field_mappings, &sequence_mappings);

    // Generate validation rules implementation if present
    let validation_impl = if let Some(rules_const) = validation_rules {
        let rules_ident = syn::Ident::new(&rules_const, proc_macro2::Span::call_site());
        quote! {
            impl #name {
                /// Get the validation rules for this message type
                pub fn validation_rules() -> &'static str {
                    #rules_ident
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl crate::SwiftMessageBody for #name {
            fn message_type() -> &'static str {
                #message_type
            }

            fn from_fields(fields: std::collections::HashMap<String, Vec<String>>) -> crate::SwiftResult<Self> {
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

        #validation_impl
    };

    TokenStream::from(expanded)
}

/// Extract field information from struct including sequences
fn extract_field_info(
    input: &DeriveInput,
) -> (
    Vec<String>,
    Vec<String>,
    Vec<FieldMapping>,
    Vec<SequenceMapping>,
) {
    let mut required_fields = Vec::new();
    let mut optional_fields = Vec::new();
    let mut field_mappings = Vec::new();
    let mut sequence_mappings = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            // Use the enhanced parser to get field specifications
            match parse_field_specs(fields_named) {
                Ok(field_specs) => {
                    for spec in field_specs {
                        match spec {
                            FieldSpec::Component(component_spec) => {
                                let field_name = syn::Ident::new(
                                    &component_spec.field_name,
                                    proc_macro2::Span::call_site(),
                                );

                                // Find the original field in the struct to get its attributes
                                let field_tag = if let Some(field) =
                                    fields_named.named.iter().find(|f| {
                                        f.ident.as_ref().map(|id| id.to_string())
                                            == Some(component_spec.field_name.clone())
                                    }) {
                                    // Try to extract field tag from field attributes first
                                    if let Some((tag_from_attr, _)) =
                                        extract_field_metadata(&field.attrs)
                                    {
                                        tag_from_attr
                                    } else {
                                        extract_field_tag_from_type(
                                            &component_spec.field_type,
                                            &component_spec.field_name,
                                        )
                                    }
                                } else {
                                    extract_field_tag_from_type(
                                        &component_spec.field_type,
                                        &component_spec.field_name,
                                    )
                                };

                                let is_optional = component_spec.optional
                                    || is_option_type(&component_spec.field_type);

                                if is_optional {
                                    optional_fields.push(field_tag.clone());
                                } else {
                                    required_fields.push(field_tag.clone());
                                }

                                let _metadata = FieldMetadata::from_component_spec(&component_spec);
                                field_mappings.push(FieldMapping {
                                    field_name,
                                    field_tag,
                                    field_type: component_spec.field_type.clone(),
                                    is_optional,
                                });
                            }
                            FieldSpec::Sequence(sequence_spec) => {
                                let field_name = syn::Ident::new(
                                    &sequence_spec.field_name,
                                    proc_macro2::Span::call_site(),
                                );
                                let sequence_tag = sequence_spec.sequence_id.clone();

                                // For sequences, the tag is based on the sequence ID
                                if sequence_spec.repetitive {
                                    // Repetitive sequences are typically optional (min = 0 allowed)
                                    optional_fields.push(sequence_tag.clone());
                                } else {
                                    required_fields.push(sequence_tag.clone());
                                }

                                sequence_mappings.push(SequenceMapping {
                                    field_name,
                                    sequence_tag,
                                    field_type: sequence_spec.field_type.clone(),
                                    repetitive: sequence_spec.repetitive,
                                });
                            }
                        }
                    }
                }
                Err(_) => {
                    // Fallback to legacy field extraction if parsing fails
                    for field in &fields_named.named {
                        let field_name = field.ident.as_ref().unwrap();
                        let field_type = &field.ty;

                        // Extract field tag and metadata from #[field("XX", ...)] attribute
                        if let Some((field_tag, metadata)) = extract_field_metadata(&field.attrs) {
                            let is_optional = is_option_type(field_type) || !metadata.mandatory;

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
        }
    }

    (
        required_fields,
        optional_fields,
        field_mappings,
        sequence_mappings,
    )
}

/// Extract field tag from field type and name
fn extract_field_tag_from_type(_field_type: &Type, field_name: &str) -> String {
    // Try to extract from field name patterns like field_20, field_21F, etc.
    if let Some(tag_part) = field_name.strip_prefix("field_") {
        return tag_part.to_uppercase();
    }

    // Fallback to field name
    field_name.to_uppercase()
}

/// Field mapping information
struct FieldMapping {
    field_name: syn::Ident,
    field_tag: String,
    field_type: Type,
    is_optional: bool,
}

/// Sequence mapping information
struct SequenceMapping {
    field_name: syn::Ident,
    sequence_tag: String,
    field_type: Type,
    repetitive: bool,
}

/// Field metadata from attributes
#[derive(Debug, Default)]
struct FieldMetadata {
    mandatory: bool,
    format: Option<String>,
    rules: Vec<String>,
    options: Vec<String>,
}

impl FieldMetadata {
    fn from_component_spec(spec: &ComponentSpec) -> Self {
        Self {
            mandatory: !spec.optional,
            format: Some(spec.format.clone()),
            rules: spec.validation_rules.clone(),
            options: Vec::new(),
        }
    }
}

/// Generate from_fields parsing logic including sequences
fn generate_from_fields_logic(
    field_mappings: &[FieldMapping],
    sequence_mappings: &[SequenceMapping],
) -> proc_macro2::TokenStream {
    let field_parsing = field_mappings.iter().map(|mapping| {
        let field_name = &mapping.field_name;
        let field_tag = &mapping.field_tag;

        if mapping.is_optional {
            // Check if it's a Vec type inside Option
            if is_vec_inside_option(&mapping.field_type) {
                if is_option_vec_of_swift_messages(&mapping.field_type) {
                    // Handle Option<Vec<SwiftMessage>> types
                    quote! {
                        let #field_name = if let Some(field_values) = fields.get(#field_tag) {
                            if !field_values.is_empty() {
                                let mut parsed_values = Vec::new();
                                for value in field_values {
                                    // Parse each SwiftMessage from its field representation
                                    let message_fields = crate::parser::parse_swift_message_from_string(value)?;
                                    parsed_values.push(crate::SwiftMessageBody::from_fields(message_fields)?);
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
                    // Handle Option<Vec<SwiftField>> types
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
                }
            } else if is_option_swift_message(&mapping.field_type) {
                // Handle Option<SwiftMessage> types (single instance)
                quote! {
                    let #field_name = if let Some(field_values) = fields.get(#field_tag) {
                        if let Some(first_value) = field_values.first() {
                            // Parse SwiftMessage from its field representation
                            let message_fields = crate::parser::parse_swift_message_from_string(first_value)?;
                            Some(crate::SwiftMessageBody::from_fields(message_fields)?)
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
                if is_vec_of_swift_messages(&mapping.field_type) {
                    // Handle Vec<SwiftMessage> types
                    quote! {
                        let #field_name = if let Some(field_values) = fields.get(#field_tag) {
                            let mut parsed_values = Vec::new();
                            for value in field_values {
                                // Parse each SwiftMessage from its field representation
                                let message_fields = crate::parser::parse_swift_message_from_string(value)?;
                                parsed_values.push(crate::SwiftMessageBody::from_fields(message_fields)?);
                            }
                            parsed_values
                        } else {
                            Vec::new() // Default to empty Vec for required Vec fields
                        };
                    }
                } else {
                    // Handle Vec<SwiftField> types
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
                }
            } else {
                quote! {
                    let #field_name = fields.get(#field_tag)
                        .and_then(|values| values.first())
                        .ok_or_else(|| crate::errors::ParseError::MissingRequiredField {
                            field_tag: #field_tag.to_string(),
                        })
                        .and_then(|value| crate::SwiftField::parse(value))?;
                }
            }
        }
    });

    // Generate sequence parsing logic
    let sequence_parsing = sequence_mappings.iter().map(|mapping| {
        let field_name = &mapping.field_name;
        let sequence_tag = &mapping.sequence_tag;

        if mapping.repetitive {
            // For repetitive sequences, parse multiple instances
            if is_vec_of_swift_messages(&mapping.field_type) {
                quote! {
                    let #field_name = if let Some(sequence_values) = fields.get(#sequence_tag) {
                        let mut parsed_sequences = Vec::new();
                        for value in sequence_values {
                            // Parse each SwiftMessage from its field representation
                            let message_fields = crate::parser::parse_swift_message_from_string(value)?;
                            parsed_sequences.push(crate::SwiftMessageBody::from_fields(message_fields)?);
                        }
                        parsed_sequences
                    } else {
                        Vec::new()
                    };
                }
            } else {
                quote! {
                    let #field_name = if let Some(sequence_values) = fields.get(#sequence_tag) {
                        let mut parsed_sequences = Vec::new();
                        for value in sequence_values {
                            parsed_sequences.push(crate::SwiftField::parse(value)?);
                        }
                        parsed_sequences
                    } else {
                        Vec::new()
                    };
                }
            }
        } else {
            // For non-repetitive sequences, parse as single instance
            quote! {
                let #field_name = if let Some(sequence_values) = fields.get(#sequence_tag) {
                    if let Some(first_value) = sequence_values.first() {
                        crate::SwiftField::parse(first_value)?
                    } else {
                        return Err(crate::errors::ParseError::MissingRequiredField {
                            field_tag: #sequence_tag.to_string(),
                        });
                    }
                } else {
                    return Err(crate::errors::ParseError::MissingRequiredField {
                        field_tag: #sequence_tag.to_string(),
                    });
                };
            }
        }
    });

    let field_names = field_mappings.iter().map(|mapping| &mapping.field_name);
    let sequence_names = sequence_mappings.iter().map(|mapping| &mapping.field_name);

    quote! {
        use crate::SwiftField;

        #( #field_parsing )*
        #( #sequence_parsing )*

        Ok(Self {
            #( #field_names, )*
            #( #sequence_names, )*
        })
    }
}

/// Generate to_fields serialization logic including sequences
fn generate_to_fields_logic(
    field_mappings: &[FieldMapping],
    sequence_mappings: &[SequenceMapping],
) -> proc_macro2::TokenStream {
    let field_serialization = field_mappings.iter().map(|mapping| {
        let field_name = &mapping.field_name;
        let field_tag = &mapping.field_tag;

        if mapping.is_optional {
            // Check if it's a Vec type inside Option
            if is_vec_inside_option(&mapping.field_type) {
                if is_option_vec_of_swift_messages(&mapping.field_type) {
                    // Handle Option<Vec<SwiftMessage>> types
                    quote! {
                        if let Some(ref field_values) = self.#field_name {
                            let mut serialized_values = Vec::new();
                            for message_value in field_values {
                                // Serialize each SwiftMessage to its field representation
                                let message_fields = message_value.to_fields();
                                let serialized_message = crate::parser::serialize_swift_message_to_string(&message_fields);
                                serialized_values.push(serialized_message);
                            }
                            fields.insert(#field_tag.to_string(), serialized_values);
                        }
                    }
                } else {
                    // Handle Option<Vec<SwiftField>> types
                    quote! {
                        if let Some(ref field_values) = self.#field_name {
                            let mut serialized_values = Vec::new();
                            for field_value in field_values {
                                serialized_values.push(field_value.to_swift_string());
                            }
                            fields.insert(#field_tag.to_string(), serialized_values);
                        }
                    }
                }
            } else if is_option_swift_message(&mapping.field_type) {
                // Handle Option<SwiftMessage> types (single instance)
                quote! {
                    if let Some(ref message_value) = self.#field_name {
                        // Serialize SwiftMessage to its field representation
                        let message_fields = message_value.to_fields();
                        let serialized_message = crate::parser::serialize_swift_message_to_string(&message_fields);
                        fields.insert(#field_tag.to_string(), vec![serialized_message]);
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
                if is_vec_of_swift_messages(&mapping.field_type) {
                    // Handle Vec<SwiftMessage> types
                    quote! {
                        if !self.#field_name.is_empty() {
                            let mut serialized_values = Vec::new();
                            for message_value in &self.#field_name {
                                // Serialize each SwiftMessage to its field representation
                                let message_fields = message_value.to_fields();
                                let serialized_message = crate::parser::serialize_swift_message_to_string(&message_fields);
                                serialized_values.push(serialized_message);
                            }
                            fields.insert(#field_tag.to_string(), serialized_values);
                        }
                    }
                } else {
                    // Handle Vec<SwiftField> types
                    quote! {
                        if !self.#field_name.is_empty() {
                            let mut serialized_values = Vec::new();
                            for field_value in &self.#field_name {
                                serialized_values.push(field_value.to_swift_string());
                            }
                            fields.insert(#field_tag.to_string(), serialized_values);
                        }
                    }
                }
            } else {
                quote! {
                    fields.insert(#field_tag.to_string(), vec![self.#field_name.to_swift_string()]);
                }
            }
        }
    });

    // Generate sequence serialization logic
    let sequence_serialization = sequence_mappings.iter().map(|mapping| {
        let field_name = &mapping.field_name;
        let sequence_tag = &mapping.sequence_tag;

        if mapping.repetitive {
            if is_vec_of_swift_messages(&mapping.field_type) {
                quote! {
                    if !self.#field_name.is_empty() {
                        let mut serialized_sequences = Vec::new();
                        for sequence_value in &self.#field_name {
                            // Serialize each SwiftMessage to its field representation
                            let message_fields = sequence_value.to_fields();
                            let serialized_message = crate::parser::serialize_swift_message_to_string(&message_fields);
                            serialized_sequences.push(serialized_message);
                        }
                        fields.insert(#sequence_tag.to_string(), serialized_sequences);
                    }
                }
            } else {
                quote! {
                    if !self.#field_name.is_empty() {
                        let mut serialized_sequences = Vec::new();
                        for sequence_value in &self.#field_name {
                            serialized_sequences.push(sequence_value.to_swift_string());
                        }
                        fields.insert(#sequence_tag.to_string(), serialized_sequences);
                    }
                }
            }
        } else {
            quote! {
                fields.insert(#sequence_tag.to_string(), vec![self.#field_name.to_swift_string()]);
            }
        }
    });

    quote! {
        use crate::SwiftField;

        let mut fields = std::collections::HashMap::new();
        #( #field_serialization )*
        #( #sequence_serialization )*
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
                    if let Some(syn::GenericArgument::Type(Type::Path(inner_path))) =
                        args.args.first()
                    {
                        if let Some(inner_segment) = inner_path.path.segments.last() {
                            return inner_segment.ident == "Vec";
                        }
                    }
                }
            }
        }
    }
    false
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

/// Check if Option<Vec<T>> contains SwiftMessage types
fn is_option_vec_of_swift_messages(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return is_vec_of_swift_messages(inner_type);
                    }
                }
            }
        }
    }
    false
}

/// Check if a type is Option<SwiftMessage> (single instance, not Vec)
fn is_option_swift_message(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return is_swift_message_type(inner_type);
                    }
                }
            }
        }
    }
    false
}

/// Extract field metadata from #[field("XX", mandatory/optional, format = "...", rules = [...])] attribute
fn extract_field_metadata(attrs: &[Attribute]) -> Option<(String, FieldMetadata)> {
    for attr in attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("field") {
                // Parse the tokens to extract field metadata
                let tokens = meta_list.tokens.clone();
                let token_str = tokens.to_string();

                // Simple parsing - extract field tag (first quoted string)
                if let Some(tag_start) = token_str.find('"') {
                    if let Some(tag_end) = token_str[tag_start + 1..].find('"') {
                        let field_tag =
                            token_str[tag_start + 1..tag_start + 1 + tag_end].to_string();

                        let mut metadata = FieldMetadata {
                            mandatory: token_str.contains("mandatory")
                                || !token_str.contains("optional"),
                            ..Default::default()
                        };

                        // Extract format if present
                        if let Some(format_start) = token_str.find("format = \"") {
                            let format_content_start = format_start + 10; // length of "format = \""
                            if let Some(format_end) = token_str[format_content_start..].find('"') {
                                metadata.format = Some(
                                    token_str
                                        [format_content_start..format_content_start + format_end]
                                        .to_string(),
                                );
                            }
                        }

                        // Extract rules if present
                        if let Some(rules_start) = token_str.find("rules = [") {
                            let rules_content_start = rules_start + 9; // length of "rules = ["
                            if let Some(rules_end) = token_str[rules_content_start..].find(']') {
                                let rules_content = &token_str
                                    [rules_content_start..rules_content_start + rules_end];
                                // Simple parsing: split by comma and extract quoted strings
                                for rule in rules_content.split(',') {
                                    let rule = rule.trim();
                                    if rule.starts_with('"') && rule.ends_with('"') {
                                        metadata.rules.push(rule[1..rule.len() - 1].to_string());
                                    }
                                }
                            }
                        }

                        // Extract options if present
                        if let Some(options_start) = token_str.find("options = [") {
                            let options_content_start = options_start + 11; // length of "options = ["
                            if let Some(options_end) = token_str[options_content_start..].find(']')
                            {
                                let options_content = &token_str
                                    [options_content_start..options_content_start + options_end];
                                // Simple parsing: split by comma and extract quoted strings
                                for option in options_content.split(',') {
                                    let option = option.trim();
                                    if option.starts_with('"') && option.ends_with('"') {
                                        metadata
                                            .options
                                            .push(option[1..option.len() - 1].to_string());
                                    }
                                }
                            }
                        }

                        return Some((field_tag, metadata));
                    }
                }
            }
        }
    }
    None
}

/// Extract message type from struct name (e.g., "MT103" -> "103")
fn extract_message_type_from_name(name: &str) -> String {
    if name.starts_with("MT") && name.len() > 2 {
        name[2..].to_string()
    } else {
        // Fallback - use the full name if it doesn't match MT pattern
        name.to_string()
    }
}

/// Extract validation rules from #[validation_rules(...)] attribute
fn extract_validation_rules_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if let Meta::List(meta_list) = &attr.meta {
            if meta_list.path.is_ident("validation_rules") {
                let tokens = meta_list.tokens.to_string();
                // Extract the identifier (should be a const name like MT103_VALIDATION_RULES)
                return Some(tokens.trim().to_string());
            }
        }
    }
    None
}
