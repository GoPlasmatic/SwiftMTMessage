//! AST structures and parsing for message definitions

use crate::error::{MacroError, MacroResult};
use crate::utils::attributes::extract_field_attribute_with_name;
use crate::utils::types::{
    TypeCategory, categorize_type, extract_inner_type, extract_option_vec_inner_type,
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
/// Or with a semantic name:
/// ```logic
/// #[field("34F", name = "floor_limit_debit")]
/// floor_limit_debit: Field34F,
/// ```
#[derive(Debug, Clone)]
pub struct MessageField {
    /// Field name in the struct (e.g., `field_20`, `floor_limit_debit`)
    pub name: Ident,
    /// Inner field type (extracted from Option<T> or Vec<T>)
    pub inner_type: Type,
    /// SWIFT field tag (e.g., "20", "32A", "34F")
    pub tag: String,
    /// Semantic name for JSON serialization (e.g., "floor_limit_debit")
    /// If provided, this will be used for serde rename attribute
    pub semantic_name: Option<String>,
    /// Whether the field is optional (wrapped in Option<T>)
    pub is_optional: bool,
    /// Whether the field is repetitive (wrapped in Vec<T>)
    pub is_repetitive: bool,
    /// Valid variants for numbered field tags (e.g., ["A", "F", "K"] for Field50)
    /// Only populated for fields with numbered tags like "50#1", "50#2"
    pub variant_constraints: Option<Vec<String>>,
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

        // Extract field tag and optional name from #[field("tag")] or #[field("tag", name = "...")] attribute
        let (tag, semantic_name) = extract_field_attribute_with_name(&field.attrs)?;

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

        // Extract variant constraints for numbered field tags
        let variant_constraints = if tag.contains('#') {
            extract_variant_constraints_from_type(&inner_type)
        } else {
            None
        };

        Ok(MessageField {
            name,
            inner_type,
            tag,
            semantic_name,
            is_optional,
            is_repetitive,
            variant_constraints,
        })
    }
}

/// Extract validation rules constant name from attributes
fn extract_validation_rules_attribute(attrs: &[Attribute]) -> MacroResult<Option<String>> {
    for attr in attrs {
        if attr.path().is_ident("validation_rules")
            && let Meta::List(meta_list) = &attr.meta
        {
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
                sequence_c_fields: vec![
                    "62".to_string(),
                    "64".to_string(),
                    "65".to_string(),
                    "86".to_string(),
                ],
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

/// Extract variant constraints from field type for numbered field tags
///
/// This function maps field types to their valid SWIFT variants based on the
/// SWIFT MT standards. For example, Field50InstructingParty accepts only
/// variants "C" and "L", while Field50Creditor accepts "A", "F", and "K".
fn extract_variant_constraints_from_type(inner_type: &Type) -> Option<Vec<String>> {
    if let Type::Path(type_path) = inner_type {
        if let Some(last_segment) = type_path.path.segments.last() {
            let type_name = last_segment.ident.to_string();

            // Map field types to their valid variants
            match type_name.as_str() {
                // Field 50 variants
                "Field50InstructingParty" => Some(vec!["C".to_string(), "L".to_string()]),
                "Field50Creditor" => Some(vec!["A".to_string(), "F".to_string(), "K".to_string()]),
                "Field50OrderingCustomer" => Some(vec!["A".to_string(), "F".to_string(), "K".to_string()]),
                "Field50OrderingCustomerAFK" => Some(vec!["A".to_string(), "F".to_string(), "K".to_string()]),

                // Field 52 variants
                "Field52CreditorBank" => Some(vec!["A".to_string(), "B".to_string(), "D".to_string()]),
                "Field52OrderingInstitution" => Some(vec!["A".to_string(), "B".to_string(), "D".to_string()]),

                // Field 53 variants
                "Field53SenderCorrespondent" => Some(vec!["A".to_string(), "B".to_string(), "D".to_string()]),

                // Field 54 variants
                "Field54ReceiverCorrespondent" => Some(vec!["A".to_string(), "B".to_string(), "D".to_string()]),

                // Field 56 variants
                "Field56Intermediary" => Some(vec!["A".to_string(), "C".to_string(), "D".to_string()]),

                // Field 57 variants
                "Field57DebtorBank" => Some(vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]),
                "Field57AccountWithInstitution" => Some(vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]),

                // Field 58 variants
                "Field58Beneficiary" => Some(vec!["A".to_string(), "D".to_string()]),

                // Field 59 variants
                "Field59BeneficiaryCustomer" => Some(vec!["A".to_string(), "F".to_string()]),
                "Field59Debtor" => Some(vec!["A".to_string(), "F".to_string()]),

                // Add more field type mappings as needed
                _ => {
                    // For unknown types, try to infer from naming patterns
                    if type_name.contains("InstructingParty") || type_name.contains("Creditor") {
                        // Most instructing party and creditor fields use A/F/K or C/L variants
                        if type_name.contains("Instructing") {
                            Some(vec!["C".to_string(), "L".to_string()])
                        } else {
                            Some(vec!["A".to_string(), "F".to_string(), "K".to_string()])
                        }
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    } else {
        None
    }
}
