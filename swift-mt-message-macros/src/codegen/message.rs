//! Code generation for SwiftMessage derive macro

use crate::ast::{MessageDefinition, MessageField};
use crate::error::MacroResult;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

/// Generate SwiftMessage implementation for a message definition
pub fn generate_swift_message_impl(definition: &MessageDefinition) -> MacroResult<TokenStream> {
    let name = &definition.name;
    let name_str = definition.name.to_string();
    let message_type = extract_message_type_from_name(&name_str);
    let required_fields_impl = generate_required_fields_impl(&definition.fields)?;
    let optional_fields_impl = generate_optional_fields_impl(&definition.fields)?;
    let from_fields_impl = generate_from_fields_impl(&definition.fields)?;
    let to_fields_impl = generate_to_fields_impl(&definition.fields)?;
    let sample_impl = generate_sample_impl(&definition.fields)?;
    let sample_minimal_impl = generate_sample_minimal_impl(&definition.fields)?;
    let sample_full_impl = generate_sample_full_impl(&definition.fields)?;
    let validation_rules_impl = generate_validation_rules_impl(definition)?;

    // Generate SwiftField implementation only for nested message structures
    let swift_field_impl = if is_nested_message_structure(&definition.name.to_string()) {
        generate_swift_field_impl_for_message(definition)?
    } else {
        quote! {}
    };

    Ok(quote! {
        impl crate::SwiftMessageBody for #name {
            fn message_type() -> &'static str {
                #message_type
            }

            fn from_fields(fields: std::collections::HashMap<String, Vec<(String, usize)>>) -> crate::SwiftResult<Self> {
                use crate::SwiftField;
                use crate::parser::FieldConsumptionTracker;

                #from_fields_impl
            }

            fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
                use crate::SwiftField;
                #to_fields_impl
            }

            fn required_fields() -> Vec<&'static str> {
                #required_fields_impl
            }

            fn optional_fields() -> Vec<&'static str> {
                #optional_fields_impl
            }

            fn sample() -> Self {
                use crate::SwiftField;
                #sample_impl
            }

            fn sample_minimal() -> Self {
                use crate::SwiftField;
                #sample_minimal_impl
            }

            fn sample_full() -> Self {
                use crate::SwiftField;
                #sample_full_impl
            }

            fn sample_with_config(config: &crate::sample::MessageConfig) -> Self {
                use crate::sample::MessageScenario;
                use crate::SwiftField;

                match config.scenario {
                    Some(MessageScenario::Minimal) => Self::sample_minimal(),
                    Some(MessageScenario::Full) => Self::sample_full(),
                    Some(MessageScenario::StpCompliant) => {
                        // Generate STP-compliant sample
                        let mut sample = <Self as crate::SwiftMessageBody>::sample();
                        // Add STP-specific field configurations
                        sample
                    }
                    Some(MessageScenario::CoverPayment) => {
                        // Generate cover payment sample
                        let mut sample = Self::sample_full();
                        // Add cover payment specific fields
                        sample
                    }
                    _ => <Self as crate::SwiftMessageBody>::sample(),
                }
            }
        }

        // Generate SwiftField implementation for message structures
        #swift_field_impl

        // Only generate validation_rules if not already manually defined
        // For now, skip for MT940 which has manual implementation
        #validation_rules_impl
    })
}

/// Generate required_fields implementation
fn generate_required_fields_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let required_tags: Vec<_> = fields
        .iter()
        .filter(|field| !field.is_optional)
        .map(|field| &field.tag)
        .collect();

    Ok(quote! {
        vec![#(#required_tags),*]
    })
}

/// Generate optional_fields implementation
fn generate_optional_fields_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let optional_tags: Vec<_> = fields
        .iter()
        .filter(|field| field.is_optional)
        .map(|field| &field.tag)
        .collect();

    Ok(quote! {
        vec![#(#optional_tags),*]
    })
}

/// Generate from_fields implementation
fn generate_from_fields_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let mut field_parsers = Vec::new();

    for field in fields {
        let field_name = &field.name;
        let inner_type = &field.inner_type;
        let tag = &field.tag;

        // Special handling for sequence fields marked with "#"
        if tag == "#" {
            if field.is_repetitive {
                // This is a sequence field (like transactions in MT101)
                field_parsers.push(quote! {
                    #field_name: {
                        // Parse sequence B fields
                        // This will be handled by the message-specific logic
                        // For now, return empty vec
                        Vec::new()
                    }
                });
            } else {
                return Err(crate::error::MacroError::internal(
                    proc_macro2::Span::call_site(),
                    "Field with tag '#' must be repetitive (Vec<T>)",
                ));
            }
            continue;
        }

        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T> - consume all values for this tag with enhanced error context
                field_parsers.push(quote! {
                    #field_name: fields.get(#tag)
                        .map(|values| {
                            values.iter()
                                .enumerate()
                                .map(|(idx, (v, pos))| {
                                    #inner_type::parse(v)
                                        .map_err(|e| {
                                            let line_num = *pos >> 16;
                                            crate::errors::ParseError::FieldParsingFailed {
                                                field_tag: #tag.to_string(),
                                                field_type: stringify!(#inner_type).to_string(),
                                                position: line_num,
                                                original_error: format!("Item {}: {}", idx, e),
                                            }
                                        })
                                })
                                .collect::<crate::SwiftResult<Vec<_>>>()
                        })
                        .transpose()?
                });
            } else {
                // Optional T - use sequential consumption with enhanced error context
                field_parsers.push(quote! {
                    #field_name: {
                        if let Some((value, variant_tag, pos)) =
                            crate::parser::find_field_with_variant_sequential(&fields, #tag, &mut tracker) {
                            Some(#inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(#tag))
                                .map_err(|e| {
                                    let line_num = pos >> 16;
                                    crate::errors::ParseError::FieldParsingFailed {
                                        field_tag: #tag.to_string(),
                                        field_type: stringify!(#inner_type).to_string(),
                                        position: line_num,
                                        original_error: e.to_string(),
                                    }
                                })?)
                        } else {
                            None
                        }
                    }
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T> - consume all values for this tag with enhanced error context
            field_parsers.push(quote! {
                #field_name: fields.get(#tag)
                    .map(|values| {
                        values.iter()
                            .enumerate()
                            .map(|(idx, (v, pos))| {
                                #inner_type::parse(v)
                                    .map_err(|e| {
                                        let line_num = *pos >> 16;
                                        crate::errors::ParseError::FieldParsingFailed {
                                            field_tag: #tag.to_string(),
                                            field_type: stringify!(#inner_type).to_string(),
                                            position: line_num,
                                            original_error: format!("Item {}: {}", idx, e),
                                        }
                                    })
                            })
                            .collect::<crate::SwiftResult<Vec<_>>>()
                    })
                    .unwrap_or_else(|| Ok(Vec::new()))?
            });
        } else {
            // Required T - use sequential consumption with enhanced error context
            field_parsers.push(quote! {
                #field_name: {
                    let (value, variant_tag, pos) =
                        crate::parser::find_field_with_variant_sequential(&fields, #tag, &mut tracker)
                            .ok_or_else(|| crate::errors::ParseError::MissingRequiredField {
                                field_tag: #tag.to_string(),
                                field_name: stringify!(#field_name).to_string(),
                                message_type: Self::message_type().to_string(),
                                position_in_block4: None,
                            })?;

                    #inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(#tag))
                        .map_err(|e| {
                            let line_num = pos >> 16;
                            crate::errors::ParseError::FieldParsingFailed {
                                field_tag: #tag.to_string(),
                                field_type: stringify!(#inner_type).to_string(),
                                position: line_num,
                                original_error: e.to_string(),
                            }
                        })?
                }
            });
        }
    }

    Ok(quote! {
        let mut tracker = FieldConsumptionTracker::new();

        Ok(Self {
            #(#field_parsers),*
        })
    })
}

/// Generate to_fields implementation
fn generate_to_fields_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let mut field_serializers = Vec::new();

    for field in fields {
        let field_name = &field.name;
        let tag = &field.tag;

        // Skip sequence fields marked with "#"
        if tag == "#" {
            continue;
        }

        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                field_serializers.push(quote! {
                    if let Some(ref values) = self.#field_name {
                        let serialized_values: Vec<String> = values.iter()
                            .map(|v| v.to_swift_string())
                            .collect();
                        if !serialized_values.is_empty() {
                            fields.insert(#tag.to_string(), serialized_values);
                        }
                    }
                });
            } else {
                // Optional T - check if it's an enum field that needs variant handling
                let field_type = &field.inner_type;
                if is_enum_field_type(field_type) {
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            let field_tag_with_variant = crate::get_field_tag_with_variant(#tag, value);
                            fields.insert(field_tag_with_variant, vec![value.to_swift_string()]);
                        }
                    });
                } else {
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            fields.insert(#tag.to_string(), vec![value.to_swift_string()]);
                        }
                    });
                }
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_serializers.push(quote! {
                let serialized_values: Vec<String> = self.#field_name.iter()
                    .map(|v| v.to_swift_string())
                    .collect();
                fields.insert(#tag.to_string(), serialized_values);
            });
        } else {
            // Required T - check if it's an enum field that needs variant handling
            let field_type = &field.inner_type;
            if is_enum_field_type(field_type) {
                field_serializers.push(quote! {
                    let field_tag_with_variant = crate::get_field_tag_with_variant(#tag, &self.#field_name);
                    fields.insert(field_tag_with_variant, vec![self.#field_name.to_swift_string()]);
                });
            } else {
                field_serializers.push(quote! {
                    fields.insert(#tag.to_string(), vec![self.#field_name.to_swift_string()]);
                });
            }
        }
    }

    Ok(quote! {
        let mut fields = std::collections::HashMap::new();
        #(#field_serializers)*
        fields
    })
}

/// Generate sample implementation
fn generate_sample_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let mut field_samples = Vec::new();

    for field in fields {
        let field_name = &field.name;
        let inner_type = &field.inner_type;

        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                field_samples.push(quote! {
                    #field_name: Some(vec![<#inner_type as crate::SwiftField>::sample()])
                });
            } else {
                // Optional T
                field_samples.push(quote! {
                    #field_name: Some(<#inner_type as crate::SwiftField>::sample())
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_samples.push(quote! {
                #field_name: vec![<#inner_type as crate::SwiftField>::sample()]
            });
        } else {
            // Required T
            field_samples.push(quote! {
                #field_name: <#inner_type as crate::SwiftField>::sample()
            });
        }
    }

    Ok(quote! {
        Self {
            #(#field_samples),*
        }
    })
}

/// Generate sample_minimal implementation (only required fields)
fn generate_sample_minimal_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let mut field_samples = Vec::new();

    for field in fields {
        let field_name = &field.name;
        let inner_type = &field.inner_type;

        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                field_samples.push(quote! {
                    #field_name: None
                });
            } else {
                // Optional T
                field_samples.push(quote! {
                    #field_name: None
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_samples.push(quote! {
                #field_name: vec![<#inner_type as crate::SwiftField>::sample()]
            });
        } else {
            // Required T
            field_samples.push(quote! {
                #field_name: <#inner_type as crate::SwiftField>::sample()
            });
        }
    }

    Ok(quote! {
        Self {
            #(#field_samples),*
        }
    })
}

/// Generate sample_full implementation (all fields)
fn generate_sample_full_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let mut field_samples = Vec::new();

    for field in fields {
        let field_name = &field.name;
        let inner_type = &field.inner_type;

        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                field_samples.push(quote! {
                    #field_name: Some(vec![<#inner_type as crate::SwiftField>::sample(), <#inner_type as crate::SwiftField>::sample()])
                });
            } else {
                // Optional T
                field_samples.push(quote! {
                    #field_name: Some(<#inner_type as crate::SwiftField>::sample())
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_samples.push(quote! {
                #field_name: vec![<#inner_type as crate::SwiftField>::sample(), <#inner_type as crate::SwiftField>::sample()]
            });
        } else {
            // Required T
            field_samples.push(quote! {
                #field_name: <#inner_type as crate::SwiftField>::sample()
            });
        }
    }

    Ok(quote! {
        Self {
            #(#field_samples),*
        }
    })
}

/// Extract message type from struct name (e.g., MT103 -> "103")
fn extract_message_type_from_name(name: &str) -> &str {
    name.strip_prefix("MT").unwrap_or(name)
}

/// Check if a message structure is nested (used as a field in other messages)
fn is_nested_message_structure(name: &str) -> bool {
    // Nested message structures have specific suffixes
    name.ends_with("Transaction")
        || name.ends_with("StatementLine")
        || name.ends_with("Block")
        || name.ends_with("Cheque")
        || name.ends_with("Sequence")
        || name.ends_with("RateChange")
}

/// Generate validation_rules function implementation
fn generate_validation_rules_impl(definition: &MessageDefinition) -> MacroResult<TokenStream> {
    let name = &definition.name;

    if let Some(validation_rules_const) = &definition.validation_rules_const {
        let const_ident = syn::Ident::new(validation_rules_const, proc_macro2::Span::call_site());
        Ok(quote! {
            impl #name {
                /// Get validation rules for this message type
                pub fn validation_rules() -> &'static str {
                    #const_ident
                }
            }
        })
    } else {
        // Generate default validation rules if none specified
        let default_rules =
            r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#;
        Ok(quote! {
            impl #name {
                /// Get validation rules for this message type
                pub fn validation_rules() -> &'static str {
                    #default_rules
                }
            }
        })
    }
}

/// Generate SwiftField implementation for message structures
fn generate_swift_field_impl_for_message(
    definition: &MessageDefinition,
) -> MacroResult<TokenStream> {
    let name = &definition.name;
    let message_parse_impl = generate_message_parse_impl(&definition.fields)?;
    let message_to_swift_string_impl = generate_message_to_swift_string_impl(&definition.fields)?;
    let message_format_spec_impl = generate_message_format_spec_impl(definition)?;

    Ok(quote! {
        impl crate::SwiftField for #name {
            fn parse(value: &str) -> crate::Result<Self> {
                #message_parse_impl
            }

            fn to_swift_string(&self) -> String {
                #message_to_swift_string_impl
            }

            fn format_spec() -> &'static str {
                #message_format_spec_impl
            }

            fn sample() -> Self {
                // Delegate to SwiftMessageBody's sample method to avoid conflicts
                <Self as crate::SwiftMessageBody>::sample()
            }

            fn sample_with_config(_config: &crate::sample::FieldConfig) -> Self {
                // For messages, use the message body sample
                <Self as crate::SwiftMessageBody>::sample()
            }
        }
    })
}

/// Generate parse implementation for messages
fn generate_message_parse_impl(_fields: &[MessageField]) -> MacroResult<TokenStream> {
    // For message parsing, we expect a JSON-like field map format
    // This is a simplified implementation - in reality, you'd want proper field parsing
    Ok(quote! {
        // Parse message from field map format (simplified)
        // In practice, this would parse from a proper field map structure
        use serde_json;

        // Try to parse as JSON first
        if let Ok(parsed) = serde_json::from_str::<Self>(value) {
            return Ok(parsed);
        }

        // Fallback: create a sample instance
        Ok(<Self as crate::SwiftMessageBody>::sample())
    })
}

/// Generate to_swift_string implementation for messages
fn generate_message_to_swift_string_impl(_fields: &[MessageField]) -> MacroResult<TokenStream> {
    Ok(quote! {
        // Convert message to field map format
        use crate::SwiftMessageBody;
        let fields_map = self.to_fields();

        // Create a simple field representation
        let mut result = String::new();
        for (tag, values) in fields_map {
            result.push_str(&format!("{}:", tag));
            for value in values {
                result.push_str(&value);
                result.push('|');
            }
            result.push('\n');
        }

        result
    })
}

/// Generate format_spec implementation for messages
fn generate_message_format_spec_impl(definition: &MessageDefinition) -> MacroResult<TokenStream> {
    let name_str = definition.name.to_string();
    let message_type = extract_message_type_from_name(&name_str);
    Ok(quote! {
        #message_type
    })
}

/// Check if a type is an enum field type that needs variant handling
fn is_enum_field_type(field_type: &Type) -> bool {
    if let Type::Path(type_path) = field_type {
        if let Some(last_segment) = type_path.path.segments.last() {
            let type_name = last_segment.ident.to_string();
            // Check if this is a Field enum type
            type_name.starts_with("Field")
                && (
                    // Specific enum patterns
                    type_name.contains("Ordering") || 
             type_name.contains("Creditor") || 
             type_name.contains("Debtor") ||
             type_name.contains("Beneficiary") ||
             type_name.contains("Instructing") ||
             type_name.contains("Party") ||
             type_name.contains("SenderCorrespondent") ||
             type_name.contains("ReceiverCorrespondent") ||
             type_name.contains("Intermediary") ||
             type_name.contains("AccountWithInstitution") ||
             type_name.contains("DebtorBank") ||
             type_name.ends_with("AFK") ||
             type_name.ends_with("NCF") ||
             type_name.ends_with("FGH") ||
             // Simple field enum patterns like Field50, Field52, Field58, Field59, etc.
             // These are just "Field" + number with no suffix
             (type_name.len() <= 8 && type_name.chars().skip(5).all(|c| c.is_ascii_digit()))
                )
        } else {
            false
        }
    } else {
        false
    }
}
