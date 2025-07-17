//! Validation rules for SWIFT fields and messages

use crate::error::MacroResult;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Lit, Meta};

/// Validation rule types
#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// Length validation
    Length { min: Option<usize>, max: Option<usize> },
    /// Pattern validation (regex)
    Pattern(String),
    /// Custom validation function
    Custom(String),
    /// BIC validation
    Bic,
    /// Currency validation
    Currency,
    /// Amount validation
    Amount,
}

/// Parse validation rules from attributes
pub fn parse_validation_rules(attrs: &[Attribute]) -> MacroResult<Vec<ValidationRule>> {
    let mut rules = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("validation_rules") {
            match &attr.meta {
                Meta::List(meta_list) => {
                    // Parse nested validation rules
                    // Format: #[validation_rules(length(min=1, max=35), pattern("^[A-Z]+$"))]
                    rules.extend(parse_validation_rule_list(meta_list)?);
                }
                Meta::Path(_) => {
                    // Simple validation rule
                    rules.push(ValidationRule::Custom("default".to_string()));
                }
                Meta::NameValue(_) => {
                    // Name-value validation rule
                    // Format: #[validation_rules = "bic"]
                    rules.push(parse_simple_validation_rule(attr)?);
                }
            }
        }
    }
    
    Ok(rules)
}

/// Generate validation code for rules
pub fn generate_validation_code(rules: &[ValidationRule], value_expr: &TokenStream) -> TokenStream {
    if rules.is_empty() {
        return quote! {};
    }
    
    let validation_checks = rules.iter().map(|rule| {
        match rule {
            ValidationRule::Length { min, max } => {
                let min_check = min.map(|min_val| quote! {
                    if #value_expr.len() < #min_val {
                        return Err(crate::errors::ParseError::ValidationFailed {
                            errors: vec![crate::errors::ValidationError::LengthValidation {
                                field_tag: "field".to_string(),
                                expected: format!("minimum {} characters", #min_val),
                                actual: #value_expr.len(),
                            }]
                        });
                    }
                });
                
                let max_check = max.map(|max_val| quote! {
                    if #value_expr.len() > #max_val {
                        return Err(crate::errors::ParseError::ValidationFailed {
                            errors: vec![crate::errors::ValidationError::LengthValidation {
                                field_tag: "field".to_string(),
                                expected: format!("maximum {} characters", #max_val),
                                actual: #value_expr.len(),
                            }]
                        });
                    }
                });
                
                quote! {
                    #min_check
                    #max_check
                }
            }
            
            ValidationRule::Pattern(pattern) => {
                quote! {
                    if !regex::Regex::new(#pattern).unwrap().is_match(&#value_expr) {
                        return Err(crate::errors::ParseError::ValidationFailed {
                            errors: vec![crate::errors::ValidationError::PatternValidation {
                                field_tag: "field".to_string(),
                                message: format!("Value '{}' does not match pattern '{}'", #value_expr, #pattern),
                            }]
                        });
                    }
                }
            }
            
            ValidationRule::Bic => {
                quote! {
                    if !crate::validation::is_valid_bic(&#value_expr) {
                        return Err(crate::errors::ParseError::ValidationFailed {
                            errors: vec![crate::errors::ValidationError::ValueValidation {
                                field_tag: "field".to_string(),
                                message: format!("Invalid BIC code: '{}'", #value_expr),
                            }]
                        });
                    }
                }
            }
            
            ValidationRule::Currency => {
                quote! {
                    if !crate::validation::is_valid_currency(&#value_expr) {
                        return Err(crate::errors::ParseError::ValidationFailed {
                            errors: vec![crate::errors::ValidationError::ValueValidation {
                                field_tag: "field".to_string(),
                                message: format!("Invalid currency code: '{}'", #value_expr),
                            }]
                        });
                    }
                }
            }
            
            ValidationRule::Amount => {
                quote! {
                    if let Err(e) = #value_expr.parse::<f64>() {
                        return Err(crate::errors::ParseError::ValidationFailed {
                            errors: vec![crate::errors::ValidationError::ValueValidation {
                                field_tag: "field".to_string(),
                                message: format!("Invalid amount format: '{}' - {}", #value_expr, e),
                            }]
                        });
                    }
                }
            }
            
            ValidationRule::Custom(func_name) => {
                let func_ident = syn::Ident::new(func_name, proc_macro2::Span::call_site());
                quote! {
                    if let Err(e) = #func_ident(&#value_expr) {
                        return Err(crate::errors::ParseError::ValidationFailed {
                            errors: vec![crate::errors::ValidationError::ValueValidation {
                                field_tag: "field".to_string(),
                                message: format!("Custom validation failed: {}", e),
                            }]
                        });
                    }
                }
            }
        }
    });
    
    quote! {
        // Apply validation rules
        #(#validation_checks)*
    }
}

fn parse_validation_rule_list(_meta_list: &syn::MetaList) -> MacroResult<Vec<ValidationRule>> {
    // For now, return empty - this would parse complex nested rules
    Ok(Vec::new())
}

fn parse_simple_validation_rule(attr: &Attribute) -> MacroResult<ValidationRule> {
    match &attr.meta {
        Meta::NameValue(name_value) => {
            if let syn::Expr::Lit(expr_lit) = &name_value.value {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                    let rule_type = lit_str.value();
                    match rule_type.as_str() {
                        "bic" => Ok(ValidationRule::Bic),
                        "currency" => Ok(ValidationRule::Currency),
                        "amount" => Ok(ValidationRule::Amount),
                        _ => Ok(ValidationRule::Custom(rule_type)),
                    }
                } else {
                    Err(crate::error::MacroError::invalid_attribute(
                        attr.span(),
                        "validation_rules",
                        "non-string value",
                        "string literal"
                    ))
                }
            } else {
                Err(crate::error::MacroError::invalid_attribute(
                    attr.span(),
                    "validation_rules",
                    "complex expression",
                    "string literal"
                ))
            }
        }
        _ => Ok(ValidationRule::Custom("default".to_string()))
    }
}