//! AST structures and parsing for message definitions

use crate::error::{MacroError, MacroResult};
use crate::utils::types::{is_option_type, is_vec_type, extract_inner_type};
use crate::utils::attributes::extract_field_attribute;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta, Type};

/// Parsed message structure information
#[derive(Debug, Clone)]
pub struct MessageDefinition {
    /// The message struct name (e.g., MT103)
    pub name: Ident,
    /// List of message fields
    pub fields: Vec<MessageField>,
    /// Validation rules constant name (e.g., "MT103_VALIDATION_RULES")
    pub validation_rules_const: Option<String>,
    /// Span for error reporting
    #[allow(dead_code)]
    pub span: Span,
}

/// Message field definition
#[derive(Debug, Clone)]
pub struct MessageField {
    /// Field name in the struct
    pub name: Ident,
    /// Field type
    #[allow(dead_code)]
    pub field_type: Type,
    /// Inner field type (extracted from Option<T> or Vec<T>)
    pub inner_type: Type,
    /// SWIFT field tag (e.g., "20", "23B")
    pub tag: String,
    /// Whether the field is optional
    pub is_optional: bool,
    /// Whether the field is repetitive (Vec<T>)
    pub is_repetitive: bool,
    /// Span for error reporting
    #[allow(dead_code)]
    pub span: Span,
}

impl MessageDefinition {
    /// Parse message definition from derive input
    pub fn parse(input: &DeriveInput) -> MacroResult<Self> {
        let name = input.ident.clone();
        let span = input.ident.span();

        let fields = match &input.data {
            syn::Data::Struct(data_struct) => MessageField::parse_all(&data_struct.fields)?,
            syn::Data::Enum(_) => {
                return Err(MacroError::unsupported_type(
                    span,
                    "enum",
                    "SwiftMessage can only be derived for structs",
                ));
            }
            syn::Data::Union(_) => {
                return Err(MacroError::unsupported_type(
                    span,
                    "union",
                    "SwiftMessage can only be derived for structs",
                ));
            }
        };

        // Extract validation rules from attributes
        let validation_rules_const = extract_validation_rules_attribute(&input.attrs)?;

        Ok(MessageDefinition {
            name,
            fields,
            validation_rules_const,
            span,
        })
    }
}

impl MessageField {
    /// Parse all message fields from struct fields
    fn parse_all(fields: &Fields) -> MacroResult<Vec<Self>> {
        match fields {
            Fields::Named(named_fields) => {
                let mut message_fields = Vec::new();

                for field in &named_fields.named {
                    let message_field = MessageField::parse(field)?;
                    message_fields.push(message_field);
                }

                Ok(message_fields)
            }
            Fields::Unnamed(_) => Err(MacroError::unsupported_type(
                Span::call_site(),
                "tuple struct",
                "SwiftMessage requires named fields",
            )),
            Fields::Unit => Err(MacroError::unsupported_type(
                Span::call_site(),
                "unit struct",
                "SwiftMessage requires fields",
            )),
        }
    }

    /// Parse a single message field
    fn parse(field: &Field) -> MacroResult<Self> {
        let name = field
            .ident
            .clone()
            .ok_or_else(|| MacroError::internal(field.span(), "Field must have a name"))?;

        let field_type = field.ty.clone();
        let span = field.span();

        // Extract field tag from #[field("tag")] attribute
        let tag = extract_field_attribute(&field.attrs)?;

        // Determine if field is optional or repetitive
        let is_optional = is_option_type(&field_type);
        let is_repetitive = is_vec_type(&field_type);

        // Extract inner type
        let inner_type = extract_inner_type(&field_type, is_optional, is_repetitive);

        Ok(MessageField {
            name,
            field_type,
            inner_type,
            tag,
            is_optional,
            is_repetitive,
            span,
        })
    }
}



/// Extract validation rules constant name from attributes
fn extract_validation_rules_attribute(attrs: &[Attribute]) -> MacroResult<Option<String>> {
    for attr in attrs {
        if attr.path().is_ident("validation_rules") {
            if let Meta::List(meta_list) = &attr.meta {
                // Parse #[validation_rules(CONSTANT_NAME)]
                if let Ok(Lit::Str(lit_str)) = meta_list.parse_args::<Lit>() {
                    return Ok(Some(lit_str.value()));
                }
                // Try parsing as identifier for #[validation_rules(CONSTANT_NAME)]
                if let Ok(ident) = meta_list.parse_args::<Ident>() {
                    return Ok(Some(ident.to_string()));
                }
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_parse_simple_message() {
        let input: DeriveInput = syn::parse2(quote! {
            struct MT103 {
                #[field("20")]
                transaction_reference: Field20,
                #[field("23B")]
                bank_operation_code: Field23B,
            }
        })
        .unwrap();

        let definition = MessageDefinition::parse(&input).unwrap();
        assert_eq!(definition.name.to_string(), "MT103");
        assert_eq!(definition.fields.len(), 2);
        assert_eq!(definition.fields[0].tag, "20");
        assert_eq!(definition.fields[1].tag, "23B");
    }

    #[test]
    fn test_parse_optional_field() {
        let input: DeriveInput = syn::parse2(quote! {
            struct TestMessage {
                #[field("50")]
                ordering_customer: Option<Field50>,
            }
        })
        .unwrap();

        let definition = MessageDefinition::parse(&input).unwrap();
        assert_eq!(definition.fields.len(), 1);
        assert!(definition.fields[0].is_optional);
        assert!(!definition.fields[0].is_repetitive);
    }

    #[test]
    fn test_parse_repetitive_field() {
        let input: DeriveInput = syn::parse2(quote! {
            struct TestMessage {
                #[field("61")]
                statement_lines: Vec<Field61>,
            }
        })
        .unwrap();

        let definition = MessageDefinition::parse(&input).unwrap();
        assert_eq!(definition.fields.len(), 1);
        assert!(!definition.fields[0].is_optional);
        assert!(definition.fields[0].is_repetitive);
    }
}
