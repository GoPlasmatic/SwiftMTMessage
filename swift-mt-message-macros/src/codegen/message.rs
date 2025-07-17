//! Code generation for SwiftMessage derive macro

use crate::ast::{MessageDefinition, MessageField};
use crate::error::MacroResult;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate SwiftMessage implementation for a message definition
pub fn generate_swift_message_impl(definition: &MessageDefinition) -> MacroResult<TokenStream> {
    let name = &definition.name;
    let message_type = extract_message_type_from_name(&definition.name.to_string());
    let required_fields_impl = generate_required_fields_impl(&definition.fields)?;
    let optional_fields_impl = generate_optional_fields_impl(&definition.fields)?;
    let from_fields_impl = generate_from_fields_impl(&definition.fields)?;
    let to_fields_impl = generate_to_fields_impl(&definition.fields)?;
    let sample_impl = generate_sample_impl(&definition.fields)?;
    let sample_minimal_impl = generate_sample_minimal_impl(&definition.fields)?;
    let sample_full_impl = generate_sample_full_impl(&definition.fields)?;
    let validation_rules_impl = generate_validation_rules_impl(&definition)?;
    
    Ok(quote! {
        impl crate::SwiftMessageBody for #name {
            fn message_type() -> &'static str {
                #message_type
            }
            
            fn from_fields(fields: std::collections::HashMap<String, Vec<String>>) -> crate::SwiftResult<Self> {
                use crate::SwiftField;
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
                        let mut sample = Self::sample();
                        // Add STP-specific field configurations
                        sample
                    }
                    Some(MessageScenario::CoverPayment) => {
                        // Generate cover payment sample
                        let mut sample = Self::sample_full();
                        // Add cover payment specific fields
                        sample
                    }
                    _ => Self::sample(),
                }
            }
        }
        
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
        let field_type = &field.field_type;
        let tag = &field.tag;
        
        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                field_parsers.push(quote! {
                    #field_name: fields.get(#tag)
                        .map(|values| {
                            values.iter()
                                .map(|v| #field_type::parse(v))
                                .collect::<crate::SwiftResult<Vec<_>>>()
                        })
                        .transpose()?
                });
            } else {
                // Optional T
                field_parsers.push(quote! {
                    #field_name: fields.get(#tag)
                        .and_then(|values| values.first())
                        .map(|v| #field_type::parse(v))
                        .transpose()?
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_parsers.push(quote! {
                #field_name: fields.get(#tag)
                    .map(|values| {
                        values.iter()
                            .map(|v| #field_type::parse(v))
                            .collect::<crate::SwiftResult<Vec<_>>>()
                    })
                    .unwrap_or_else(|| Ok(Vec::new()))?
            });
        } else {
            // Required T
            field_parsers.push(quote! {
                #field_name: {
                    let values = fields.get(#tag)
                        .ok_or_else(|| crate::errors::ParseError::MissingRequiredField {
                            field_tag: #tag.to_string()
                        })?;
                    let value = values.first()
                        .ok_or_else(|| crate::errors::ParseError::InvalidFormat {
                            message: format!("Field {} has no values", #tag)
                        })?;
                    #field_type::parse(value)?
                }
            });
        }
    }
    
    Ok(quote! {
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
                // Optional T
                field_serializers.push(quote! {
                    if let Some(ref value) = self.#field_name {
                        fields.insert(#tag.to_string(), vec![value.to_swift_string()]);
                    }
                });
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
            // Required T
            field_serializers.push(quote! {
                fields.insert(#tag.to_string(), vec![self.#field_name.to_swift_string()]);
            });
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
        let field_type = &field.field_type;
        
        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                field_samples.push(quote! {
                    #field_name: Some(vec![#field_type::sample()])
                });
            } else {
                // Optional T
                field_samples.push(quote! {
                    #field_name: Some(#field_type::sample())
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_samples.push(quote! {
                #field_name: vec![#field_type::sample()]
            });
        } else {
            // Required T
            field_samples.push(quote! {
                #field_name: #field_type::sample()
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
        let field_type = &field.field_type;
        
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
                #field_name: vec![#field_type::sample()]
            });
        } else {
            // Required T
            field_samples.push(quote! {
                #field_name: #field_type::sample()
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
        let field_type = &field.field_type;
        
        if field.is_optional {
            if field.is_repetitive {
                // Optional Vec<T>
                field_samples.push(quote! {
                    #field_name: Some(vec![#field_type::sample(), #field_type::sample()])
                });
            } else {
                // Optional T
                field_samples.push(quote! {
                    #field_name: Some(#field_type::sample())
                });
            }
        } else if field.is_repetitive {
            // Required Vec<T>
            field_samples.push(quote! {
                #field_name: vec![#field_type::sample(), #field_type::sample()]
            });
        } else {
            // Required T
            field_samples.push(quote! {
                #field_name: #field_type::sample()
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
fn extract_message_type_from_name(name: &str) -> String {
    if name.starts_with("MT") {
        name[2..].to_string()
    } else {
        name.to_string()
    }
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
        let default_rules = r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#;
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
