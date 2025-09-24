//! Code generation for SwiftMessage derive macro

use crate::ast::{MessageDefinition, MessageField};
use crate::error::MacroResult;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

/// Extract base tag by removing index suffix (e.g., "50#1" -> "50")
fn extract_base_tag(tag: &str) -> &str {
    if let Some(index_pos) = tag.find('#') {
        &tag[..index_pos]
    } else {
        tag
    }
}

/// Generate SwiftMessage implementation for a message definition
pub fn generate_swift_message_impl(definition: &MessageDefinition) -> MacroResult<TokenStream> {
    let name = &definition.name;
    let name_str = definition.name.to_string();
    let message_type = extract_message_type_from_name(&name_str);
    let required_fields_impl = generate_required_fields_impl(&definition.fields)?;
    let optional_fields_impl = generate_optional_fields_impl(&definition.fields)?;
    let from_fields_impl = if definition.has_sequences {
        generate_from_fields_with_sequences_impl(definition)?
    } else {
        generate_from_fields_impl(&definition.fields)?
    };
    let from_fields_with_config_impl = if definition.has_sequences {
        generate_from_fields_with_sequences_and_config_impl(definition)?
    } else {
        generate_from_fields_with_config_impl(&definition.fields)?
    };
    let to_fields_impl = generate_to_fields_impl(&definition.fields)?;
    let to_ordered_fields_impl = if definition.has_sequences {
        generate_to_ordered_fields_with_sequences_impl(definition)?
    } else {
        // Generate custom implementation for nested messages with enum fields
        generate_to_ordered_fields_for_nested_messages(&definition.fields)?
    };
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

            fn from_fields_with_config(
                fields: std::collections::HashMap<String, Vec<(String, usize)>>,
                config: &crate::errors::ParserConfig
            ) -> std::result::Result<crate::errors::ParseResult<Self>, crate::errors::ParseError> {
                use crate::SwiftField;
                use crate::parser::FieldConsumptionTracker;

                #from_fields_with_config_impl
            }

            fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
                use crate::SwiftField;
                #to_fields_impl
            }

            #to_ordered_fields_impl

            fn required_fields() -> Vec<&'static str> {
                #required_fields_impl
            }

            fn optional_fields() -> Vec<&'static str> {
                #optional_fields_impl
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

/// Generate from_fields implementation for messages with sequences
fn generate_from_fields_with_sequences_impl(
    definition: &MessageDefinition,
) -> MacroResult<TokenStream> {
    let fields = &definition.fields;
    let sequence_config = definition.sequence_config.as_ref().unwrap();

    // Separate fields by sequence
    let mut seq_a_fields: Vec<&MessageField> = Vec::new();
    let mut seq_b_field: Option<&MessageField> = None;
    let mut seq_c_message_fields: Vec<&MessageField> = Vec::new();

    for field in fields {
        if field.tag == "#" {
            seq_b_field = Some(field);
        } else if sequence_config.sequence_c_fields.contains(&field.tag) {
            seq_c_message_fields.push(field);
        } else {
            seq_a_fields.push(field);
        }
    }

    // Generate field parsers for each sequence
    let mut field_parsers = Vec::new();

    // First, split the fields into sequences
    let seq_b_marker = &sequence_config.sequence_b_marker;
    let seq_c_fields = &sequence_config.sequence_c_fields;
    let has_seq_c = sequence_config.has_sequence_c;

    field_parsers.push(quote! {
        // Split fields into sequences
        let sequence_config = crate::parser::SequenceConfig {
            sequence_b_marker: #seq_b_marker.to_string(),
            sequence_c_fields: vec![#(#seq_c_fields.to_string()),*],
            has_sequence_c: #has_seq_c,
        };
        let parsed_sequences = crate::parser::split_into_sequences(&fields, &sequence_config)?;
    });

    // Parse sequence A fields
    for field in &seq_a_fields {
        let parser = generate_sequence_field_parser(field, quote! { parsed_sequences.sequence_a })?;
        field_parsers.push(parser);
    }

    // Parse sequence B (transactions)
    if let Some(seq_b) = seq_b_field {
        let field_name = &seq_b.name;
        let inner_type = &seq_b.inner_type;
        field_parsers.push(quote! {
            let #field_name = {
                // Parse sequence B transactions
                crate::parser::parse_sequences::<#inner_type>(&parsed_sequences.sequence_b, &mut tracker)?
            };
        });
    }

    // Parse sequence C fields
    for field in &seq_c_message_fields {
        let parser = generate_sequence_field_parser(field, quote! { parsed_sequences.sequence_c })?;
        field_parsers.push(parser);
    }

    // Build the struct fields
    let mut struct_fields = Vec::new();
    for field in fields {
        let field_name = &field.name;
        struct_fields.push(quote! { #field_name });
    }

    Ok(quote! {
        let mut tracker = FieldConsumptionTracker::new();

        #(#field_parsers)*

        Ok(Self {
            #(#struct_fields),*
        })
    })
}

/// Generate parser for a field in a specific sequence
fn generate_sequence_field_parser(
    field: &MessageField,
    sequence_name: TokenStream,
) -> MacroResult<TokenStream> {
    let field_name = &field.name;
    let inner_type = &field.inner_type;
    let tag = &field.tag;

    if field.is_optional && field.is_repetitive {
        // Get variant constraints for numbered fields
        let variant_constraints = &field.variant_constraints;
        let variant_constraints_tokens = if let Some(constraints) = variant_constraints {
            let constraint_strings: Vec<_> = constraints.iter().map(|s| s.as_str()).collect();
            quote! { Some(vec![#(#constraint_strings),*]) }
        } else {
            quote! { None::<Vec<&str>> }
        };

        // Handle Option<Vec<T>>
        Ok(quote! {
            let #field_name = {
                let base_tag = crate::extract_base_tag(#tag);

                // Always use the field's valid variants
                let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());

                // Find all matching fields
                let mut results = Vec::new();

                // Try to find fields with tracker
                while let Some((value, variant_tag, pos)) =
                    crate::parser::find_field_with_variant_sequential_constrained(&#sequence_name, base_tag, &mut tracker, valid_variants_slice)
                {
                    let parsed = #inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(base_tag))
                        .map_err(|e| {
                            let line_num = pos >> 16;
                            crate::errors::ParseError::FieldParsingFailed {
                                field_tag: #tag.to_string(),
                                field_type: stringify!(#inner_type).to_string(),
                                position: line_num,
                                original_error: e.to_string(),
                            }
                        })?;
                    results.push(parsed);
                    tracker.mark_consumed(base_tag, pos);
                }

                if results.is_empty() {
                    None
                } else {
                    Some(results)
                }
            };
        })
    } else if field.is_optional {
        // Get variant constraints for numbered fields
        let variant_constraints = &field.variant_constraints;
        let variant_constraints_tokens = if let Some(constraints) = variant_constraints {
            let constraint_strings: Vec<_> = constraints.iter().map(|s| s.as_str()).collect();
            quote! { Some(vec![#(#constraint_strings),*]) }
        } else {
            quote! { None::<Vec<&str>> }
        };

        Ok(quote! {
            let #field_name = {
                // Always use sequential consumption for fields
                let base_tag = crate::extract_base_tag(#tag);
                let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());
                let field_result = crate::parser::find_field_with_variant_sequential_constrained(&#sequence_name, base_tag, &mut tracker, valid_variants_slice);

                if let Some((value, variant_tag, pos)) = field_result {
                    let parse_base_tag = crate::extract_base_tag(#tag);
                    Some(#inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(parse_base_tag))
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
                    // Fallback for sequence parsing where fields might have variant tags
                    let mut found = None;
                    let fallback_base_tag = crate::extract_base_tag(#tag);
                    let fallback_valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                    let fallback_valid_variants_vec: Option<Vec<&str>> = fallback_valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let fallback_valid_variants_slice = fallback_valid_variants_vec.as_ref().map(|v| v.as_slice());

                    // If we have valid variants, try each one
                    if let Some(variants) = fallback_valid_variants_slice {
                        for variant in variants {
                            let variant_tag = format!("{}{}", fallback_base_tag, variant);

                            if let Some(values) = #sequence_name.get(&variant_tag) {
                                if let Some((value, pos)) = values.first() {
                                    found = Some((value.clone(), Some(variant.to_string()), *pos));
                                    break;
                                }
                            }
                        }
                    }

                    // If no variants found, try the base tag directly
                    if found.is_none() {
                        if let Some(values) = #sequence_name.get(fallback_base_tag) {
                            if let Some((value, pos)) = values.first() {
                                found = Some((value.clone(), None, *pos));
                            }
                        }
                    }

                    if let Some((value, variant_tag, pos)) = found {
                        Some(#inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(fallback_base_tag))
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
            };
        })
    } else if field.is_repetitive {
        // Handle Vec<T> (required repetitive)
        Ok(quote! {
            let #field_name = {
                let base_tag = crate::extract_base_tag(#tag);
                let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());

                // Find all matching fields
                let mut results = Vec::new();

                // Try to find fields with tracker
                while let Some((value, variant_tag, pos)) = crate::parser::find_field_with_variant_sequential_constrained(&#sequence_name, base_tag, &mut tracker, valid_variants_slice) {
                    let parsed = #inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(base_tag))
                        .map_err(|e| {
                            let line_num = pos >> 16;
                            crate::errors::ParseError::FieldParsingFailed {
                                field_tag: #tag.to_string(),
                                field_type: stringify!(#inner_type).to_string(),
                                position: line_num,
                                original_error: e.to_string(),
                            }
                        })?;
                    results.push(parsed);
                    tracker.mark_consumed(base_tag, pos);
                }

                if results.is_empty() {
                    return Err(crate::errors::ParseError::MissingRequiredField {
                        field_tag: #tag.to_string(),
                        field_name: stringify!(#field_name).to_string(),
                        message_type: Self::message_type().to_string(),
                        position_in_block4: None,
                    });
                }

                results
            };
        })
    } else {
        Ok(quote! {
            let #field_name = {
                // Use variant-based routing for all fields
                let base_tag = crate::extract_base_tag(#tag);
                let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());

                // Find field with matching variant for this specific type
                let field_result = crate::parser::find_field_with_variant_sequential_constrained(&#sequence_name, base_tag, &mut tracker, valid_variants_slice);

                let (value, variant_tag, pos) = if let Some(result) = field_result {
                    result
                } else {
                    // Fallback for sequence parsing where fields might have variant tags
                    let mut found = None;

                    // Define variables needed for fallback logic
                    let fallback_base_tag = crate::extract_base_tag(#tag);
                    let fallback_valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                    let fallback_valid_variants_vec: Option<Vec<&str>> = fallback_valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let fallback_valid_variants_slice = fallback_valid_variants_vec.as_ref().map(|v| v.as_slice());

                    // If we have valid variants, try each one
                    if let Some(variants) = fallback_valid_variants_slice {
                        for variant in variants {
                            let variant_tag = format!("{}{}", fallback_base_tag, variant);

                            if let Some(values) = #sequence_name.get(&variant_tag) {
                                if let Some((value, pos)) = values.first() {
                                    found = Some((value.clone(), Some(variant.to_string()), *pos));
                                    break;
                                }
                            }
                        }
                    }

                    // If no variants found, try the base tag directly
                    if found.is_none() {
                        if let Some(values) = #sequence_name.get(fallback_base_tag) {
                            if let Some((value, pos)) = values.first() {
                                found = Some((value.clone(), None, *pos));
                            }
                        }
                    }

                    found.ok_or_else(|| crate::errors::ParseError::MissingRequiredField {
                        field_tag: #tag.to_string(),
                        field_name: stringify!(#field_name).to_string(),
                        message_type: Self::message_type().to_string(),
                        position_in_block4: None,
                    })?
                };

                let parse_base_tag = crate::extract_base_tag(#tag);
                #inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(parse_base_tag))
                    .map_err(|e| {
                        let line_num = pos >> 16;
                        crate::errors::ParseError::FieldParsingFailed {
                            field_tag: #tag.to_string(),
                            field_type: stringify!(#inner_type).to_string(),
                            position: line_num,
                            original_error: e.to_string(),
                        }
                    })?
            };
        })
    }
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
                        // Parse sequence fields by identifying boundaries
                        crate::parser::parse_sequences::<#inner_type>(&fields, &mut tracker)?
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
                    #field_name: {
                        let base_tag = crate::extract_base_tag(#tag);
                        fields.get(base_tag)
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
                    }
                });
            } else {
                // Get variant constraints for numbered fields
                let variant_constraints = &field.variant_constraints;
                let variant_constraints_tokens = if let Some(constraints) = variant_constraints {
                    let constraint_strings = constraints.iter().map(|s| s.as_str());
                    quote! { Some(vec![#(#constraint_strings),*]) }
                } else {
                    quote! { None }
                };

                // Optional T - use sequential consumption with enhanced error context
                field_parsers.push(quote! {
                    #field_name: {
                        // For numbered fields (e.g., "50#1", "50#2"), use variant-based routing
                        let field_result = if #tag.contains('#') {
                            // For numbered fields, we never look for the exact numbered tag in MT parsing
                            // Instead, we use variant-based routing to determine which field to populate
                            let base_tag = crate::extract_base_tag(#tag);

                            // Use the variant constraints extracted at compile time for this specific numbered field
                            let valid_variants = #variant_constraints_tokens;

                            // Pass the field tag to help with routing strategy
                            crate::parser::find_field_with_variant_sequential_numbered(&fields, base_tag, &mut tracker, valid_variants, #tag)
                        } else {
                            // Non-numbered fields use normal variant logic
                            let base_tag = crate::extract_base_tag(#tag);
                            let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                            let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());
                            crate::parser::find_field_with_variant_sequential_constrained(&fields, base_tag, &mut tracker, valid_variants_slice)
                        };

                        if let Some((value, variant_tag, pos)) = field_result {
                            let parse_base_tag = crate::extract_base_tag(#tag);
                            Some(#inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(parse_base_tag))
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
                            // Fallback for sequence parsing where fields might have variant tags
                            let mut found = None;
                            let fallback_base_tag = crate::extract_base_tag(#tag);
                            let fallback_valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                            let fallback_valid_variants_vec: Option<Vec<&str>> = fallback_valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let fallback_valid_variants_slice = fallback_valid_variants_vec.as_ref().map(|v| v.as_slice());

                            // If we have valid variants, try each one
                            if let Some(variants) = fallback_valid_variants_slice {
                                for variant in variants {
                                    let variant_tag = format!("{}{}", fallback_base_tag, variant);

                                    if let Some(values) = fields.get(&variant_tag) {
                                        if let Some((value, pos)) = values.first() {
                                            found = Some((value.clone(), Some(variant.to_string()), *pos));
                                            break;
                                        }
                                    }
                                }
                            }

                            // Debug: Check if there are any fields with this base tag but different variants
                            #[cfg(debug_assertions)]
                            {
                                for (field_tag, _) in fields.iter() {
                                    if field_tag.starts_with(fallback_base_tag) && field_tag != fallback_base_tag {
                                        if let Some(variants) = fallback_valid_variants_slice {
                                            let field_variant = &field_tag[fallback_base_tag.len()..];
                                            if !variants.contains(&field_variant) {
                                                eprintln!("DEBUG: Field {} exists but variant {} not in valid variants {:?} for {}",
                                                    field_tag, field_variant, variants, #tag);
                                            }
                                        }
                                    }
                                }
                            }

                            // If no variants found, try the base tag directly
                            if found.is_none() {
                                if let Some(values) = fields.get(fallback_base_tag) {
                                    if let Some((value, pos)) = values.first() {
                                        found = Some((value.clone(), None, *pos));
                                    }
                                }
                            }

                            if let Some((value, variant_tag, pos)) = found {
                                Some(#inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(fallback_base_tag))
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
                    }
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T> - consume all values for this tag with enhanced error context
            field_parsers.push(quote! {
                #field_name: {
                    let base_tag = crate::extract_base_tag(#tag);
                    fields.get(base_tag)
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
                }
            });
        } else {
            // Required T - use sequential consumption with enhanced error context
            field_parsers.push(quote! {
                #field_name: {
                    // For numbered fields (e.g., "50#1", "50#2"), use variant-based routing
                    let field_result = if #tag.contains('#') {
                        // For numbered fields, we never look for the exact numbered tag in MT parsing
                        // Instead, we use variant-based routing to determine which field to populate
                        let base_tag = crate::extract_base_tag(#tag);
                        let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                        let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());

                        // Find field with matching variant for this specific type
                        crate::parser::find_field_with_variant_sequential_constrained(&fields, base_tag, &mut tracker, valid_variants_slice)
                    } else {
                        // Non-numbered fields use normal variant logic
                        let base_tag = crate::extract_base_tag(#tag);
                        let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                        let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());
                        crate::parser::find_field_with_variant_sequential_constrained(&fields, base_tag, &mut tracker, valid_variants_slice)
                    };

                    let (value, variant_tag, pos) = if let Some(result) = field_result {
                        result
                    } else {
                        // Fallback for sequence parsing where fields might have variant tags
                        // This handles the case where we're parsing from pre-collected sequence fields
                        let mut found = None;
                        let fallback_base_tag = crate::extract_base_tag(#tag);
                        let fallback_valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                        let fallback_valid_variants_vec: Option<Vec<&str>> = fallback_valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let fallback_valid_variants_slice = fallback_valid_variants_vec.as_ref().map(|v| v.as_slice());

                        // If we have valid variants, try each one
                        if let Some(variants) = fallback_valid_variants_slice {
                            for variant in variants {
                                let variant_tag = format!("{}{}", fallback_base_tag, variant);
                                if let Some(values) = fields.get(&variant_tag) {
                                    if let Some((value, pos)) = values.first() {
                                        found = Some((value.clone(), Some(variant.to_string()), *pos));
                                        break;
                                    }
                                }
                            }
                        }

                        // If no variants found, try the base tag directly
                        if found.is_none() {
                            if let Some(values) = fields.get(fallback_base_tag) {
                                if let Some((value, pos)) = values.first() {
                                    found = Some((value.clone(), None, *pos));
                                }
                            }
                        }

                        found.ok_or_else(|| crate::errors::ParseError::MissingRequiredField {
                            field_tag: #tag.to_string(),
                            field_name: stringify!(#field_name).to_string(),
                            message_type: Self::message_type().to_string(),
                            position_in_block4: None,
                        })?
                    };

                    #inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(crate::extract_base_tag(#tag)))
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

/// Generate from_fields implementation with error collection
fn generate_from_fields_with_config_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let mut field_parsers = Vec::new();
    let mut field_names = Vec::new();
    let mut field_errors = Vec::new();

    for field in fields {
        let field_name = &field.name;
        let inner_type = &field.inner_type;
        let tag = &field.tag;
        field_names.push(field_name);

        // Special handling for sequence fields marked with "#"
        if tag == "#" {
            if field.is_repetitive {
                // This is a sequence field (like transactions in MT101)
                field_parsers.push(quote! {
                    let #field_name = if config.fail_fast {
                        // Fail-fast mode: use ? operator
                        Some(crate::parser::parse_sequences::<#inner_type>(&fields, &mut tracker)?)
                    } else {
                        // Error collection mode
                        match crate::parser::parse_sequences::<#inner_type>(&fields, &mut tracker) {
                            Ok(val) => Some(val),
                            Err(e) => {
                                errors.push(e);
                                None
                            }
                        }
                    };
                });
                field_errors.push(quote! {
                    if #field_name.is_none() && !field_is_optional(stringify!(#field_name)) {
                        has_critical_errors = true;
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
                // Optional Vec<T>
                field_parsers.push(quote! {
                    let #field_name = {
                        let base_tag = crate::extract_base_tag(#tag);
                        fields.get(base_tag)
                            .map(|values| {
                                let mut results = Vec::new();
                                let mut has_error = false;

                                for (idx, (v, pos)) in values.iter().enumerate() {
                                    match #inner_type::parse(v) {
                                        Ok(parsed) => results.push(parsed),
                                        Err(e) => {
                                            let line_num = *pos >> 16;
                                            let error = crate::errors::ParseError::FieldParsingFailed {
                                                field_tag: #tag.to_string(),
                                                field_type: stringify!(#inner_type).to_string(),
                                                position: line_num,
                                                original_error: format!("Item {}: {}", idx, e),
                                            };
                                            if config.fail_fast {
                                                return Err(error);
                                            } else {
                                                errors.push(error);
                                                has_error = true;
                                            }
                                        }
                                    }
                                }

                                if has_error && results.is_empty() {
                                    Ok(Vec::new())  // Return empty vec instead of error for optional fields
                                } else {
                                    Ok(results)
                                }
                            })
                            .transpose()
                            .ok()
                            .flatten()
                    };
                });
            } else {
                // Optional T
                field_parsers.push(quote! {
                    let #field_name = {
                        let base_tag = crate::extract_base_tag(#tag);
                        let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                        let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());

                        let field_result = crate::parser::find_field_with_variant_sequential_constrained(&fields, base_tag, &mut tracker, valid_variants_slice);

                        if let Some((value, variant_tag, pos)) = field_result {
                            match #inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(base_tag)) {
                                Ok(parsed) => Some(parsed),
                                Err(e) => {
                                    let line_num = pos >> 16;
                                    let error = crate::errors::ParseError::FieldParsingFailed {
                                        field_tag: #tag.to_string(),
                                        field_type: stringify!(#inner_type).to_string(),
                                        position: line_num,
                                        original_error: e.to_string(),
                                    };
                                    if config.fail_fast {
                                        return Err(error);
                                    } else {
                                        errors.push(error);
                                        None
                                    }
                                }
                            }
                        } else {
                            None
                        }
                    };
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_parsers.push(quote! {
                let #field_name = {
                    let base_tag = crate::extract_base_tag(#tag);
                    let field_values = fields.get(base_tag);

                    if let Some(values) = field_values {
                        let mut results = Vec::new();
                        let mut has_error = false;

                        for (idx, (v, pos)) in values.iter().enumerate() {
                            match #inner_type::parse(v) {
                                Ok(parsed) => results.push(parsed),
                                Err(e) => {
                                    let line_num = *pos >> 16;
                                    let error = crate::errors::ParseError::FieldParsingFailed {
                                        field_tag: #tag.to_string(),
                                        field_type: stringify!(#inner_type).to_string(),
                                        position: line_num,
                                        original_error: format!("Item {}: {}", idx, e),
                                    };
                                    if config.fail_fast {
                                        return Err(error);
                                    } else {
                                        errors.push(error);
                                        has_error = true;
                                    }
                                }
                            }
                        }

                        if has_error && results.is_empty() {
                            None
                        } else {
                            Some(results)
                        }
                    } else {
                        None
                    }
                };
            });
            field_errors.push(quote! {
                if #field_name.is_none() {
                    let error = crate::errors::ParseError::MissingRequiredField {
                        field_tag: #tag.to_string(),
                        field_name: stringify!(#field_name).to_string(),
                        message_type: Self::message_type().to_string(),
                        position_in_block4: None,
                    };
                    errors.push(error);
                    has_critical_errors = true;
                }
            });
        } else {
            // Required T
            field_parsers.push(quote! {
                let #field_name = {
                    let base_tag = crate::extract_base_tag(#tag);
                    let valid_variants = <#inner_type as crate::SwiftField>::valid_variants();
                    let valid_variants_vec: Option<Vec<&str>> = valid_variants.as_ref().map(|v| v.iter().map(|s| *s).collect());
                    let valid_variants_slice = valid_variants_vec.as_ref().map(|v| v.as_slice());

                    let field_result = crate::parser::find_field_with_variant_sequential_constrained(&fields, base_tag, &mut tracker, valid_variants_slice);

                    if let Some((value, variant_tag, pos)) = field_result {
                        match #inner_type::parse_with_variant(&value, variant_tag.as_deref(), Some(base_tag)) {
                            Ok(parsed) => Some(parsed),
                            Err(e) => {
                                let line_num = pos >> 16;
                                let error = crate::errors::ParseError::FieldParsingFailed {
                                    field_tag: #tag.to_string(),
                                    field_type: stringify!(#inner_type).to_string(),
                                    position: line_num,
                                    original_error: e.to_string(),
                                };
                                if config.fail_fast {
                                    return Err(error);
                                } else {
                                    errors.push(error);
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                };
            });
            field_errors.push(quote! {
                if #field_name.is_none() {
                    let error = crate::errors::ParseError::MissingRequiredField {
                        field_tag: #tag.to_string(),
                        field_name: stringify!(#field_name).to_string(),
                        message_type: Self::message_type().to_string(),
                        position_in_block4: None,
                    };
                    errors.push(error);
                    has_critical_errors = true;
                }
            });
        }
    }

    // Generate struct construction with proper unwrapping
    let mut struct_fields = Vec::new();
    for field in fields {
        let field_name = &field.name;
        if field.is_optional {
            struct_fields.push(quote! { #field_name });
        } else if field.is_repetitive {
            struct_fields.push(quote! { #field_name: #field_name.unwrap_or_default() });
        } else {
            // For required fields, we need to check if we can provide a default
            struct_fields.push(quote! {
                #field_name: #field_name.ok_or_else(|| {
                    crate::errors::ParseError::MissingRequiredField {
                        field_tag: stringify!(#field_name).to_string(),
                        field_name: stringify!(#field_name).to_string(),
                        message_type: Self::message_type().to_string(),
                        position_in_block4: None,
                    }
                })?
            });
        }
    }

    Ok(quote! {
        let mut tracker = FieldConsumptionTracker::new();
        let mut errors = Vec::new();
        let mut has_critical_errors = false;

        // Helper function to check if a field is optional
        let optional_fields = <Self as crate::SwiftMessageBody>::optional_fields();
        let field_is_optional = |field_name: &str| -> bool {
            optional_fields.contains(&field_name)
        };

        #(#field_parsers)*

        // Check for missing required fields
        #(#field_errors)*

        // Return result based on errors collected
        if !errors.is_empty() {
            if config.fail_fast {
                // In fail-fast mode, return the first error
                Err(errors.into_iter().next().unwrap())
            } else if has_critical_errors {
                // Critical errors mean we can't construct the message
                Ok(crate::errors::ParseResult::Failure(errors))
            } else {
                // Try to construct with what we have
                match (|| -> crate::SwiftResult<Self> {
                    Ok(Self {
                        #(#struct_fields),*
                    })
                })() {
                    Ok(msg) => Ok(crate::errors::ParseResult::PartialSuccess(msg, errors)),
                    Err(_) => Ok(crate::errors::ParseResult::Failure(errors))
                }
            }
        } else {
            // Try to construct the message
            match (|| -> crate::SwiftResult<Self> {
                Ok(Self {
                    #(#struct_fields),*
                })
            })() {
                Ok(msg) => Ok(crate::errors::ParseResult::Success(msg)),
                Err(e) => {
                    if config.fail_fast {
                        Err(e)
                    } else {
                        Ok(crate::errors::ParseResult::Failure(vec![e]))
                    }
                }
            }
        }
    })
}

/// Generate from_fields implementation with sequences and error collection
fn generate_from_fields_with_sequences_and_config_impl(
    _definition: &MessageDefinition,
) -> MacroResult<TokenStream> {
    // For now, use the default implementation that delegates to from_fields
    // A proper implementation would handle sequences with error collection
    Ok(quote! {
        // Use default implementation for sequences
        if config.fail_fast {
            match Self::from_fields(fields) {
                Ok(msg) => Ok(crate::errors::ParseResult::Success(msg)),
                Err(e) => Err(e),
            }
        } else {
            // For non-fail-fast mode with sequences, fall back to fail-fast for now
            // TODO: Implement proper error collection for sequences
            match Self::from_fields(fields) {
                Ok(msg) => Ok(crate::errors::ParseResult::Success(msg)),
                Err(e) => Err(e),
            }
        }
    })
}

/// Generate to_fields implementation
fn generate_to_fields_impl(fields: &[MessageField]) -> MacroResult<TokenStream> {
    let mut field_serializers = Vec::new();

    // Track fields that might appear in transactions
    let transaction_field_tags = vec![
        "32B", "33B", "36", "71A", "71F", "71G", "21", "23E", "50", "52", "57", "59", "70", "26T",
        "77B", "21C", "21D", "21E", "21F",
    ];
    let transaction_fields_set: std::collections::HashSet<&str> =
        transaction_field_tags.into_iter().collect();

    for field in fields {
        let field_name = &field.name;
        let tag = &field.tag;

        // Special handling for sequence fields marked with "#"
        if tag == "#" {
            if field.is_repetitive {
                // This is a sequence field (like transactions in MT101/MT104)
                // Each item in the Vec needs to be serialized to its component fields
                field_serializers.push(quote! {
                    // Serialize each transaction/sequence item
                    for item in &self.#field_name {
                        let item_fields = item.to_fields();
                        // Merge the item's fields into the main field map
                        for (item_tag, item_values) in item_fields {
                            fields.entry(item_tag).or_insert_with(Vec::new).extend(item_values);
                        }
                    }
                });
            }
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
                            // Keep the full tag to avoid overwriting
                            fields.insert(#tag.to_string(), serialized_values);
                        }
                    }
                });
            } else {
                // Optional T - check if it's an enum field that needs variant handling
                let field_type = &field.inner_type;
                let base_tag = extract_base_tag(tag);
                if is_enum_field_type(field_type) {
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            let base_tag = crate::get_field_tag_for_mt(#tag);
                            let field_tag_with_variant = crate::get_field_tag_with_variant(&base_tag, value);
                            fields.insert(field_tag_with_variant, vec![value.to_swift_string()]);
                        }
                    });
                } else if transaction_fields_set.contains(base_tag) {
                    // Use extend for fields that might appear in transactions
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            let mt_tag = crate::get_field_tag_for_mt(#tag);
                            fields.entry(mt_tag).or_insert_with(Vec::new).push(value.to_swift_string());
                        }
                    });
                } else {
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            // For regular fields, use the tag as-is
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
                // Keep the full tag to avoid overwriting
                fields.insert(#tag.to_string(), serialized_values);
            });
        } else {
            // Required T - check if it's an enum field that needs variant handling
            let field_type = &field.inner_type;
            if tag.contains('#') && is_enum_field_type(field_type) {
                // For numbered enum fields (e.g., 50#1, 50#2), preserve the numbered tag in JSON
                // but include variant information for proper MT serialization
                field_serializers.push(quote! {
                    fields.insert(#tag.to_string(), vec![self.#field_name.to_swift_string()]);
                });
            } else if tag.contains('#') {
                // For numbered non-enum fields, preserve the numbered tag in JSON
                field_serializers.push(quote! {
                    fields.insert(#tag.to_string(), vec![self.#field_name.to_swift_string()]);
                });
            } else if is_enum_field_type(field_type) {
                field_serializers.push(quote! {
                    let base_tag = crate::get_field_tag_for_mt(#tag);
                    let field_tag_with_variant = crate::get_field_tag_with_variant(&base_tag, &self.#field_name);
                    fields.insert(field_tag_with_variant, vec![self.#field_name.to_swift_string()]);
                });
            } else {
                // Check if this field might appear in both sequence A/C and transactions
                let base_tag = extract_base_tag(tag);
                if transaction_fields_set.contains(base_tag) {
                    // Use extend for fields that might appear in transactions
                    field_serializers.push(quote! {
                        let mt_tag = crate::get_field_tag_for_mt(#tag);
                        fields.entry(mt_tag).or_insert_with(Vec::new).push(self.#field_name.to_swift_string());
                    });
                } else {
                    field_serializers.push(quote! {
                        // Keep the full tag to avoid overwriting
                        fields.insert(#tag.to_string(), vec![self.#field_name.to_swift_string()]);
                    });
                }
            }
        }
    }

    Ok(quote! {
        let mut fields = std::collections::HashMap::new();
        #(#field_serializers)*
        fields
    })
}

/// Generate to_ordered_fields implementation for multi-sequence messages
fn generate_to_ordered_fields_with_sequences_impl(
    definition: &MessageDefinition,
) -> MacroResult<TokenStream> {
    let message_name = &definition.name.to_string();

    // Get sequence configuration
    let _sequence_config = definition.sequence_config.as_ref().ok_or_else(|| {
        crate::error::MacroError::internal(
            proc_macro2::Span::call_site(),
            "Expected sequence configuration for multi-sequence message",
        )
    })?;

    // Identify which fields belong to sequence A (main sequence) vs sequence B (transactions)
    let mut seq_a_fields = Vec::new();
    let mut transaction_field_name = None;
    let mut transaction_field_type = None;

    for field in &definition.fields {
        if field.tag == "#" {
            transaction_field_name = Some(&field.name);
            transaction_field_type = Some(&field.inner_type);
        } else {
            seq_a_fields.push(&field.tag);
        }
    }

    let _transaction_field = transaction_field_name.ok_or_else(|| {
        crate::error::MacroError::internal(
            proc_macro2::Span::call_site(),
            "Multi-sequence message must have a # field for transactions",
        )
    })?;

    let transaction_type = transaction_field_type.ok_or_else(|| {
        crate::error::MacroError::internal(
            proc_macro2::Span::call_site(),
            "Transaction field must have a type",
        )
    })?;

    // For MT101/MT104, we need to know which fields belong to transactions
    // This is determined by the transaction type structure
    let _transaction_fields = match message_name.as_str() {
        "MT101" => vec![
            "21", "21F", "23E", "32B", "50", "52", "56", "57", "59", "70", "77B", "33B", "71A",
            "25A", "36",
        ],
        "MT104" => vec![
            "21", "23E", "21C", "21D", "21E", "32B", "50", "52", "57", "59", "70", "26T", "77B",
            "33B", "71A", "71F", "71G", "36",
        ],
        "MT107" => vec![
            "21", "23E", "21C", "21D", "21E", "32B", "33B", "50", "52", "56", "57", "59", "70",
            "72", "77B", "71A", "36",
        ],
        "MT110" => vec!["21", "30", "32", "50", "52", "59"],
        "MT210" => vec!["21", "32B", "50", "52", "56"],
        "MT920" => vec!["12", "25", "34F"],
        "MT935" => vec!["23", "25", "30", "37H"],
        "MT940" => vec!["61", "86"],
        "MT942" => vec!["61", "86"],
        _ => vec![],
    };

    // Sequence C fields that appear after all transactions
    let sequence_c_fields = match message_name.as_str() {
        "MT104" => vec!["32B", "19", "71F", "71G", "53"],
        "MT935" => vec!["72"], // Field 72 appears after rate changes
        "MT940" => vec!["62", "64", "65", "86"], // Closing balance and final fields
        "MT942" => vec!["62", "64", "65", "86"], // Closing balance and final fields
        _ => vec![],
    };

    // For MT104, we need to generate field serializers directly from struct fields
    // to maintain proper sequence ordering
    let mut seq_a_field_serializers = Vec::new();
    let mut seq_c_field_serializers = Vec::new();
    let transaction_field = transaction_field_name.unwrap();

    // Collect and sort sequence A fields by numeric order
    let mut seq_a_fields_sorted: Vec<(&MessageField, u32)> = Vec::new();
    let mut seq_c_fields_sorted: Vec<(&MessageField, u32)> = Vec::new();

    for field in &definition.fields {
        if field.tag == "#" {
            continue; // Skip transaction field
        }

        let base_tag = extract_base_tag(&field.tag);
        let num = base_tag
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .fold(0u32, |acc, c| acc * 10 + (c as u32 - '0' as u32));

        if sequence_c_fields.contains(&base_tag) {
            seq_c_fields_sorted.push((field, num));
        } else {
            seq_a_fields_sorted.push((field, num));
        }
    }

    // Sort by numeric order
    seq_a_fields_sorted.sort_by_key(|(_, num)| *num);
    seq_c_fields_sorted.sort_by_key(|(_, num)| *num);

    // Generate serializers for sorted sequence A fields
    for (field, _) in seq_a_fields_sorted {
        let field_name = &field.name;
        let tag = &field.tag;
        let base_tag = extract_base_tag(tag);

        // This field is already in sequence A (not in sequence C)
        let field_type = &field.inner_type;
        if is_enum_field_type(field_type) {
            // Enum field - needs variant handling
            if field.is_optional {
                seq_a_field_serializers.push(quote! {
                    if let Some(ref value) = self.#field_name {
                        let field_tag = crate::get_field_tag_with_variant(#base_tag, value);
                        ordered_fields.push((field_tag, value.to_swift_string()));
                    }
                });
            } else {
                seq_a_field_serializers.push(quote! {
                    let field_tag = crate::get_field_tag_with_variant(#base_tag, &self.#field_name);
                    ordered_fields.push((field_tag, self.#field_name.to_swift_string()));
                });
            }
        } else {
            // Non-enum field - use base tag directly
            if field.is_optional {
                if field.is_repetitive {
                    // Optional Vec<T>
                    seq_a_field_serializers.push(quote! {
                        if let Some(ref values) = self.#field_name {
                            for value in values {
                                ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                            }
                        }
                    });
                } else {
                    // Optional T
                    seq_a_field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                        }
                    });
                }
            } else if field.is_repetitive {
                // Required Vec<T>
                seq_a_field_serializers.push(quote! {
                    for value in &self.#field_name {
                        ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                    }
                });
            } else {
                // Required T
                seq_a_field_serializers.push(quote! {
                    ordered_fields.push((#base_tag.to_string(), self.#field_name.to_swift_string()));
                });
            }
        }
    }

    // Generate serializers for sorted sequence C fields
    for (field, _) in seq_c_fields_sorted {
        let field_name = &field.name;
        let tag = &field.tag;
        let base_tag = extract_base_tag(tag);
        let field_type = &field.inner_type;

        // Check if this is an enum field that needs variant handling
        if is_enum_field_type(field_type) {
            // Enum field - needs variant handling
            if field.is_optional {
                if field.is_repetitive {
                    // Optional Vec<T> with enum
                    seq_c_field_serializers.push(quote! {
                        if let Some(ref values) = self.#field_name {
                            for value in values {
                                let field_tag = crate::get_field_tag_with_variant(#base_tag, value);
                                ordered_fields.push((field_tag, value.to_swift_string()));
                            }
                        }
                    });
                } else {
                    // Optional T with enum
                    seq_c_field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            let field_tag = crate::get_field_tag_with_variant(#base_tag, value);
                            ordered_fields.push((field_tag, value.to_swift_string()));
                        }
                    });
                }
            } else if field.is_repetitive {
                // Required Vec<T> with enum
                seq_c_field_serializers.push(quote! {
                    for value in &self.#field_name {
                        let field_tag = crate::get_field_tag_with_variant(#base_tag, value);
                        ordered_fields.push((field_tag, value.to_swift_string()));
                    }
                });
            } else {
                // Required T with enum
                seq_c_field_serializers.push(quote! {
                    let field_tag = crate::get_field_tag_with_variant(#base_tag, &self.#field_name);
                    ordered_fields.push((field_tag, self.#field_name.to_swift_string()));
                });
            }
        } else {
            // Non-enum field - use base tag directly
            if field.is_optional {
                if field.is_repetitive {
                    // Optional Vec<T>
                    seq_c_field_serializers.push(quote! {
                        if let Some(ref values) = self.#field_name {
                            for value in values {
                                ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                            }
                        }
                    });
                } else {
                    // Optional T
                    seq_c_field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                        }
                    });
                }
            } else if field.is_repetitive {
                // Required Vec<T>
                seq_c_field_serializers.push(quote! {
                    for value in &self.#field_name {
                        ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                    }
                });
            } else {
                // Required T
                seq_c_field_serializers.push(quote! {
                    ordered_fields.push((#base_tag.to_string(), self.#field_name.to_swift_string()));
                });
            }
        }
    }

    Ok(quote! {
        fn to_ordered_fields(&self) -> Vec<(String, String)> {
            use crate::SwiftField;
            let mut ordered_fields = Vec::new();

            // Serialize sequence A fields in order
            #(#seq_a_field_serializers)*

            // Serialize transaction fields
            for transaction in &self.#transaction_field {
                let tx_fields = <#transaction_type as crate::SwiftMessageBody>::to_ordered_fields(transaction);
                ordered_fields.extend(tx_fields);
            }

            // Serialize sequence C fields
            #(#seq_c_field_serializers)*

            ordered_fields
        }
    })
}

/// Extract message type from struct name (e.g., MT103 -> "103")
fn extract_message_type_from_name(name: &str) -> &str {
    name.strip_prefix("MT").unwrap_or(name)
}

/// Generate to_ordered_fields implementation for nested messages with enum fields
fn generate_to_ordered_fields_for_nested_messages(
    fields: &[MessageField],
) -> MacroResult<TokenStream> {
    // Check if any fields are numbered enum fields
    let has_numbered_enum_fields = fields
        .iter()
        .any(|f| f.tag.contains('#') && is_enum_field_type(&f.inner_type));

    if !has_numbered_enum_fields {
        // Use default implementation if no numbered enum fields
        return Ok(quote! {});
    }

    // Generate custom implementation that handles enum variants
    // We need to properly handle numbered enum fields by preserving their variants
    let mut field_serializers = Vec::new();

    for field in fields {
        let field_name = &field.name;
        let tag = &field.tag;
        let base_tag = extract_base_tag(tag);
        let field_type = &field.inner_type;

        // Check if this is an enum field that needs variant handling
        let is_enum = is_enum_field_type(field_type);
        let is_numbered = tag.contains('#');

        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                if is_numbered && is_enum {
                    field_serializers.push(quote! {
                        if let Some(ref values) = self.#field_name {
                            for value in values {
                                let field_tag = crate::get_field_tag_with_variant(#base_tag, value);
                                ordered_fields.push((field_tag, value.to_swift_string()));
                            }
                        }
                    });
                } else if is_enum {
                    field_serializers.push(quote! {
                        if let Some(ref values) = self.#field_name {
                            for value in values {
                                let field_tag = crate::get_field_tag_with_variant(#tag, value);
                                ordered_fields.push((field_tag, value.to_swift_string()));
                            }
                        }
                    });
                } else {
                    field_serializers.push(quote! {
                        if let Some(ref values) = self.#field_name {
                            for value in values {
                                ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                            }
                        }
                    });
                }
            } else {
                // Optional T
                if is_numbered && is_enum {
                    // Numbered enum field - need to extract variant and use base tag with variant
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            let field_tag = crate::get_field_tag_with_variant(#base_tag, value);
                            ordered_fields.push((field_tag, value.to_swift_string()));
                        }
                    });
                } else if is_enum {
                    // Regular enum field - need variant
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            let field_tag = crate::get_field_tag_with_variant(#tag, value);
                            ordered_fields.push((field_tag, value.to_swift_string()));
                        }
                    });
                } else {
                    // Non-enum field
                    field_serializers.push(quote! {
                        if let Some(ref value) = self.#field_name {
                            ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                        }
                    });
                }
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            if is_numbered && is_enum {
                field_serializers.push(quote! {
                    for value in &self.#field_name {
                        let field_tag = crate::get_field_tag_with_variant(#base_tag, value);
                        ordered_fields.push((field_tag, value.to_swift_string()));
                    }
                });
            } else if is_enum {
                field_serializers.push(quote! {
                    for value in &self.#field_name {
                        let field_tag = crate::get_field_tag_with_variant(#tag, value);
                        ordered_fields.push((field_tag, value.to_swift_string()));
                    }
                });
            } else {
                field_serializers.push(quote! {
                    for value in &self.#field_name {
                        ordered_fields.push((#base_tag.to_string(), value.to_swift_string()));
                    }
                });
            }
        } else {
            // Required T
            if is_numbered && is_enum {
                // Required numbered enum field
                field_serializers.push(quote! {
                    let field_tag = crate::get_field_tag_with_variant(#base_tag, &self.#field_name);
                    ordered_fields.push((field_tag, self.#field_name.to_swift_string()));
                });
            } else if is_enum {
                // Required regular enum field
                field_serializers.push(quote! {
                    let field_tag = crate::get_field_tag_with_variant(#tag, &self.#field_name);
                    ordered_fields.push((field_tag, self.#field_name.to_swift_string()));
                });
            } else {
                // Required non-enum field
                field_serializers.push(quote! {
                    ordered_fields.push((#base_tag.to_string(), self.#field_name.to_swift_string()));
                });
            }
        }
    }

    Ok(quote! {
        fn to_ordered_fields(&self) -> Vec<(String, String)> {
            use crate::SwiftField;
            let mut ordered_fields = Vec::new();

            #(#field_serializers)*

            // Sort by field tag number for proper ordering
            ordered_fields.sort_by(|(tag_a, _), (tag_b, _)| {
                let num_a = tag_a.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .fold(0u32, |acc, c| acc * 10 + (c as u32 - '0' as u32));
                let num_b = tag_b.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .fold(0u32, |acc, c| acc * 10 + (c as u32 - '0' as u32));
                num_a.cmp(&num_b).then_with(|| tag_a.cmp(tag_b))
            });

            ordered_fields
        }
    })
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
                pub fn validate() -> &'static str {
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
                pub fn validate() -> &'static str {
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
        serde_json::from_str::<Self>(value)
            .map_err(|e| crate::errors::ParseError::SerializationError {
                message: format!("Failed to parse message from JSON: {}", e),
            })
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
                    type_name.contains("AccountIdentification") ||
                    type_name.contains("AccountServicing") ||
                    type_name.contains("ThirdReimbursement") ||
                    type_name.contains("DebtorBank") ||
                    type_name.contains("DebtInstitution") ||
                    type_name.contains("DrawerBank") ||
                    type_name.contains("OrderingCustomer") ||
                    type_name.contains("OrderingInstitution") ||
                    type_name.ends_with("AFK") ||
                    type_name.ends_with("NCF") ||
                    type_name.ends_with("FGH") ||
                    type_name.ends_with("AD") ||
                    // Explicit list of known enum fields (have A/F/K/D/etc variants)
                    // This replaces the problematic digit-based check
                    matches!(type_name.as_str(),
                        "Field11" | "Field25" | "Field32" | "Field50" | "Field52" | "Field53" | "Field54" |
                        "Field55" | "Field56" | "Field57" | "Field58" | "Field59" | "Field60" | "Field62"
                    )
                )
        } else {
            false
        }
    } else {
        false
    }
}
