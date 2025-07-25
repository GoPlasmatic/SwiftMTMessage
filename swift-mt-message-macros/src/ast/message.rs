//! AST structures and parsing for message definitions

use crate::error::{MacroError, MacroResult};
use crate::utils::attributes::extract_field_attribute;
use crate::utils::types::{
    categorize_type, extract_inner_type, extract_option_vec_inner_type, TypeCategory,
};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta, Type};

/// Sequence configuration for messages with multiple sequences
#[derive(Debug, Clone)]
pub struct SequenceConfig {
    /// Field that marks the start of sequence B (usually "21")
    pub sequence_b_marker: String,
    /// Fields that belong exclusively to sequence C (if any)
    pub sequence_c_fields: Vec<String>,
    /// Whether sequence C exists for this message type
    pub has_sequence_c: bool,
}

/// Parsed message structure information
///
/// Represents a complete SWIFT message definition parsed from a Rust struct
/// that uses the `#[derive(SwiftMessage)]` macro. Contains all fields and
/// metadata needed to generate the SwiftMessageBody trait implementation.
///
/// ## Example
/// For a message like:
/// ```logic
/// #[derive(SwiftMessage)]
/// struct MT103 {
///     #[field("20")]
///     field_20: Field20,
///     #[field("32A")]  
///     field_32a: Field32A,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MessageDefinition {
    /// The message struct name (e.g., `MT103`, `MT202`)
    pub name: Ident,
    /// List of message fields with their SWIFT tags and types
    pub fields: Vec<MessageField>,
    /// Validation rules constant name (e.g., "MT103_VALIDATION_RULES")
    pub validation_rules_const: Option<String>,
    /// Whether this message has multiple sequences (like MT104)
    pub has_sequences: bool,
    /// Sequence configuration (if has_sequences is true)
    pub sequence_config: Option<SequenceConfig>,
}

/// Message field definition
///
/// Represents a single field within a SWIFT message, extracted from a struct field
/// with a `#[field("tag")]` attribute. Contains the mapping between Rust field
/// names and SWIFT field tags, along with type information for proper parsing.
///
/// ## Example
/// For a field definition like:
/// ```logic
/// #[field("32A")]
/// field_32a: Field32A,
/// ```
/// This creates a MessageField with:
/// - name: `field_32a`
/// - tag: `"32A"`
/// - inner_type: `Field32A`
#[derive(Debug, Clone)]
pub struct MessageField {
    /// Field name in the struct (e.g., `field_20`, `field_32a`)
    pub name: Ident,
    /// Field type (e.g., `Field20`, `Option<Field50>`, `Vec<Field71A>`)
    #[allow(dead_code)]
    pub field_type: Type,
    /// Inner field type (extracted from Option<T> or Vec<T>)
    pub inner_type: Type,
    /// SWIFT field tag (e.g., "20", "32A", "71A")
    pub tag: String,
    /// Whether the field is optional (wrapped in Option<T>)
    pub is_optional: bool,
    /// Whether the field is repetitive (wrapped in Vec<T>)
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

        // Extract sequence configuration from attributes
        let (has_sequences, sequence_config) =
            extract_sequence_config(&name.to_string(), &input.attrs)?;

        Ok(MessageDefinition {
            name,
            fields,
            validation_rules_const,
            has_sequences,
            sequence_config,
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

        // Determine if field is optional or repetitive using TypeCategory
        let type_category = categorize_type(&field_type);
        let is_option_vec = matches!(type_category, TypeCategory::OptionVec);
        let is_optional = matches!(
            type_category,
            TypeCategory::OptionString
                | TypeCategory::OptionNaiveDate
                | TypeCategory::OptionNaiveTime
                | TypeCategory::OptionF64
                | TypeCategory::OptionU32
                | TypeCategory::OptionU8
                | TypeCategory::OptionBool
                | TypeCategory::OptionChar
                | TypeCategory::OptionField
                | TypeCategory::OptionVec
        );
        let is_repetitive = matches!(
            type_category,
            TypeCategory::Vec | TypeCategory::VecString | TypeCategory::OptionVec
        );

        // Extract inner type
        let inner_type = if is_option_vec {
            // For Option<Vec<T>>, we need to extract T from Option<Vec<T>>
            extract_option_vec_inner_type(&field_type)
        } else {
            extract_inner_type(&field_type, is_optional, is_repetitive)
        };

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

/// Extract sequence configuration from message name and attributes
fn extract_sequence_config(
    message_name: &str,
    attrs: &[Attribute],
) -> MacroResult<(bool, Option<SequenceConfig>)> {
    // First check if there's an explicit #[sequences(...)] attribute
    for attr in attrs {
        if attr.path().is_ident("sequences") {
            // Parse custom sequence configuration
            // For now, we'll use defaults
            return Ok((true, Some(get_default_sequence_config(message_name))));
        }
    }

    // Check known multi-sequence messages
    match message_name {
        "MT101" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "21".to_string(),
                sequence_c_fields: vec![],
                has_sequence_c: false,
            }),
        )),
        "MT104" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "21".to_string(),
                sequence_c_fields: vec![
                    "32B".to_string(),
                    "19".to_string(),
                    "71F".to_string(),
                    "71G".to_string(),
                    "53".to_string(),
                ],
                has_sequence_c: true,
            }),
        )),
        "MT107" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "21".to_string(),
                sequence_c_fields: vec![],
                has_sequence_c: false,
            }),
        )),
        "MT110" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "21".to_string(),
                sequence_c_fields: vec![],
                has_sequence_c: false,
            }),
        )),
        "MT210" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "21".to_string(),
                sequence_c_fields: vec![],
                has_sequence_c: false,
            }),
        )),
        "MT920" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "12".to_string(),
                sequence_c_fields: vec![],
                has_sequence_c: false,
            }),
        )),
        "MT935" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "23".to_string(),
                sequence_c_fields: vec![],
                has_sequence_c: false,
            }),
        )),
        "MT940" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "61".to_string(),
                sequence_c_fields: vec!["62".to_string(), "64".to_string(), "65".to_string(), "86".to_string()],
                has_sequence_c: true,
            }),
        )),
        "MT942" => Ok((
            true,
            Some(SequenceConfig {
                sequence_b_marker: "61".to_string(),
                sequence_c_fields: vec![],
                has_sequence_c: false,
            }),
        )),
        _ => Ok((false, None)),
    }
}

/// Get default sequence configuration for known message types
fn get_default_sequence_config(message_name: &str) -> SequenceConfig {
    match message_name {
        "MT104" => SequenceConfig {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec![
                "32B".to_string(),
                "19".to_string(),
                "71F".to_string(),
                "71G".to_string(),
                "53".to_string(),
            ],
            has_sequence_c: true,
        },
        "MT210" => SequenceConfig {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
        "MT920" => SequenceConfig {
            sequence_b_marker: "12".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
        "MT935" => SequenceConfig {
            sequence_b_marker: "23".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
        "MT940" => SequenceConfig {
            sequence_b_marker: "61".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
        "MT942" => SequenceConfig {
            sequence_b_marker: "61".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
        _ => SequenceConfig {
            sequence_b_marker: "21".to_string(),
            sequence_c_fields: vec![],
            has_sequence_c: false,
        },
    }
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
