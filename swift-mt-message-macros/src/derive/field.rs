use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

use crate::format::FormatSpecParser;
use crate::utils::*;

/// Derive macro for SwiftField trait implementation
pub fn derive_swift_field_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Check for enhanced format specification - support more patterns
    if let Some(format_spec) = extract_format_attribute_from_struct(&input.attrs) {
        if format_spec == "6!n3!a15d"
            || format_spec == "3!n3!a15d"
            || format_spec == "3!a[1!a]15d"
            || format_spec == "12d"
            || format_spec == "6!n"
            || format_spec == "35x"
            || format_spec == "3!n"
            || format_spec == "16x"
            || format_spec == "4!c"
            || format_spec == "6!n4!n1!x4!n"
            || format_spec == "3!c"
            || format_spec == "1!a/1!a/35x"
            || format_spec == "1!a[N]12d"
            || format_spec == "5n[/2n]"
            || format_spec == "5n[/5n]"
            || format_spec == "3!a[2!n]11x"
        {
            // Support all enhanced patterns
            if let Ok(format_parser) = FormatSpecParser::parse(&format_spec) {
                // Use enhanced component-based parsing for supported patterns
                return generate_enhanced_swift_field(&input, &format_parser);
            }
        }
    }

    // Fallback to original implementation for all other cases
    generate_original_swift_field(input)
}

/// Generate enhanced SwiftField implementation for complex component-based fields
pub fn generate_enhanced_swift_field(
    input: &DeriveInput,
    format_parser: &FormatSpecParser,
) -> TokenStream {
    let name = &input.ident;

    // Extract field tag from struct name
    let field_tag = name
        .to_string()
        .strip_prefix("Field")
        .unwrap_or(&name.to_string())
        .to_uppercase();

    let format_spec = &format_parser.spec;

    // Generate field-specific parsing based on common patterns
    let parsing_impl = generate_parsing_implementation(&field_tag);
    let serialization_impl = generate_serialization_implementation(&field_tag);

    let expanded = quote! {
        impl crate::SwiftField for #name {
            fn parse(value: &str) -> crate::Result<Self> {
                #parsing_impl
            }

            fn to_swift_string(&self) -> String {
                #serialization_impl
            }

            fn validate(&self) -> crate::ValidationResult {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();
                
                // Convert to Swift string to get the field content
                let swift_string = self.to_swift_string();
                
                // Extract content after the field tag (e.g., ":20:CONTENT" -> "CONTENT")
                let content = if let Some(colon_pos) = swift_string.rfind(':') {
                    &swift_string[colon_pos + 1..]
                } else {
                    &swift_string
                };
                
                // Validate length based on format specification
                match #format_spec {
                    "16x" => {
                        if content.len() > 16 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "maximum 16 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    "4!c" => {
                        if content.len() != 4 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "exactly 4 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    "3!c" => {
                        if content.len() != 3 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "exactly 3 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    "3!n" => {
                        if content.len() != 3 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "exactly 3 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                        if !content.chars().all(|c| c.is_ascii_digit()) {
                            errors.push(crate::ValidationError::FormatValidation {
                                field_tag: #field_tag.to_string(),
                                message: "must contain only digits".to_string(),
                            });
                        }
                    },
                    "35x" => {
                        if content.len() > 35 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "maximum 35 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    _ => {
                        // For other formats, just basic validation
                        if content.is_empty() {
                            warnings.push(format!("Field {} is empty", #field_tag));
                        }
                    }
                }
                
                crate::ValidationResult {
                    is_valid: errors.is_empty(),
                    errors,
                    warnings,
                }
            }

            fn format_spec() -> &'static str {
                #format_spec
            }
        }
    };

    TokenStream::from(expanded)
}

/// Original SwiftField implementation for backward compatibility
pub fn generate_original_swift_field(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    
    // Extract field tag from struct name
    let field_tag = name
        .to_string()
        .strip_prefix("Field")
        .unwrap_or(&name.to_string())
        .to_uppercase();

    // Extract format from attributes
    let format_spec = extract_format_attribute_from_struct(&input.attrs)
        .unwrap_or("original".to_string());

    // Generate field-specific parsing based on common patterns
    let parsing_impl = generate_parsing_implementation(&field_tag);
    let serialization_impl = generate_serialization_implementation(&field_tag);

    let expanded = quote! {
        impl crate::SwiftField for #name {
            fn parse(value: &str) -> crate::Result<Self> {
                #parsing_impl
            }

            fn to_swift_string(&self) -> String {
                #serialization_impl
            }

            fn validate(&self) -> crate::ValidationResult {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();
                
                // Convert to Swift string to get the field content
                let swift_string = self.to_swift_string();
                
                // Extract content after the field tag (e.g., ":20:CONTENT" -> "CONTENT")
                let content = if let Some(colon_pos) = swift_string.rfind(':') {
                    &swift_string[colon_pos + 1..]
                } else {
                    &swift_string
                };
                
                // Validate length based on format specification
                match #format_spec {
                    "16x" => {
                        if content.len() > 16 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "maximum 16 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    "4!c" => {
                        if content.len() != 4 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "exactly 4 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    "3!c" => {
                        if content.len() != 3 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "exactly 3 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    "3!n" => {
                        if content.len() != 3 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "exactly 3 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                        if !content.chars().all(|c| c.is_ascii_digit()) {
                            errors.push(crate::ValidationError::FormatValidation {
                                field_tag: #field_tag.to_string(),
                                message: "must contain only digits".to_string(),
                            });
                        }
                    },
                    "35x" => {
                        if content.len() > 35 {
                            errors.push(crate::ValidationError::LengthValidation {
                                field_tag: #field_tag.to_string(),
                                expected: "maximum 35 characters".to_string(),
                                actual: content.len(),
                            });
                        }
                    },
                    _ => {
                        // For other formats, just basic validation
                        if content.is_empty() {
                            warnings.push(format!("Field {} is empty", #field_tag));
                        }
                    }
                }
                
                crate::ValidationResult {
                    is_valid: errors.is_empty(),
                    errors,
                    warnings,
                }
            }

            fn format_spec() -> &'static str {
                #format_spec
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate field-specific parsing implementation
fn generate_parsing_implementation(field_tag: &str) -> proc_macro2::TokenStream {
    match field_tag {
        "12" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":12:") {
                &value[4..]
            } else if value.starts_with("12:") {
                &value[3..]
            } else {
                value
            };

            Self::new(content).map_err(|e| e.into())
        },
        "20" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":20:") {
                &value[4..]
            } else if value.starts_with("20:") {
                &value[3..]
            } else {
                value
            };

            Ok(Self::new(content.to_string()))
        },
        "21" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":21:") {
                &value[4..]
            } else if value.starts_with("21:") {
                &value[3..]
            } else {
                value
            };

            Ok(Self::new(content.to_string()))
        },
        "23B" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":23B:") {
                &value[5..]
            } else if value.starts_with("23B:") {
                &value[4..]
            } else {
                value
            };

            Self::new(content).map_err(|e| e.into())
        },
        "26T" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":26T:") {
                &value[5..]
            } else if value.starts_with("26T:") {
                &value[4..]
            } else {
                value
            };

            Self::new(content).map_err(|e| e.into())
        },
        "32A" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":32A:") {
                &value[5..]
            } else if value.starts_with("32A:") {
                &value[4..]
            } else {
                value
            };

            // Parse format: YYMMDDCCCAMOUNT
            if content.len() < 9 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "32A".to_string(),
                    message: "Field 32A content too short".to_string(),
                }.into());
            }

            // Extract date (first 6 characters: YYMMDD)
            let date_str = &content[0..6];
            let year = 2000 + date_str[0..2].parse::<i32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid year in date".to_string(),
            })?;
            let month = date_str[2..4].parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid month in date".to_string(),
            })?;
            let day = date_str[4..6].parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid day in date".to_string(),
            })?;

            let value_date = chrono::NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| crate::ParseError::InvalidFieldFormat {
                    field_tag: "32A".to_string(),
                    message: "Invalid date".to_string(),
                })?;

            // Extract currency (next 3 characters)
            let currency = content[6..9].to_string().to_uppercase();

            // Extract amount (remaining characters)
            let amount_str = &content[9..];
            let raw_amount = amount_str.to_string();
            
            let amount = amount_str.replace(',', ".").parse::<f64>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid amount format".to_string(),
            })?;

            Ok(Self {
                value_date,
                currency,
                amount,
                raw_amount,
            })
        },
        "36" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":36:") {
                &value[4..]
            } else if value.starts_with("36:") {
                &value[3..]
            } else {
                value
            };

            Self::from_raw(content).map_err(|e| e.into())
        },
        "37H" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":37H:") {
                &value[5..]
            } else if value.starts_with("37H:") {
                &value[4..]
            } else {
                value
            };

            // Parse format: [Rate Indicator][N][Rate]
            if content.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "37H".to_string(),
                    message: "Field 37H content cannot be empty".to_string(),
                }.into());
            }

            let rate_indicator = content.chars().next().unwrap().to_ascii_uppercase();
            let mut remaining = &content[1..];
            
            // Check for negative indicator 'N'
            let is_negative = if remaining.starts_with('N') || remaining.starts_with('n') {
                remaining = &remaining[1..];
                true
            } else {
                false
            };

            // Parse the rate
            let _rate = remaining.replace(',', ".").parse::<f64>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "37H".to_string(),
                message: "Invalid rate format".to_string(),
            })?;

            // Use the original remaining part as raw_rate to preserve formatting
            Self::from_raw(rate_indicator, is_negative, remaining).map_err(|e| e.into())
        },
        "71A" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":71A:") {
                &value[5..]
            } else if value.starts_with("71A:") {
                &value[4..]
            } else {
                value
            };

            Self::new(content).map_err(|e| e.into())
        },
        "77T" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":77T:") {
                &value[5..]
            } else if value.starts_with("77T:") {
                &value[4..]
            } else {
                value
            };

            // Parse format: [Type][Format]/[Identifier] 
            if content.len() < 4 || !content.contains('/') {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "77T".to_string(),
                    message: "Field 77T must contain envelope type, format, and identifier separated by '/'".to_string(),
                }.into());
            }

            let slash_pos = content.find('/').unwrap();
            if slash_pos < 2 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "77T".to_string(),
                    message: "Field 77T must have type and format before '/'".to_string(),
                }.into());
            }

            let envelope_type = content[0..1].to_string();
            let envelope_format = content[1..slash_pos].to_string();
            let envelope_identifier = content[slash_pos + 1..].to_string();

            Self::new(envelope_type, envelope_format, envelope_identifier).map_err(|e| e.into())
        },
        "90C" | "90D" => quote! {
            let value = value.trim();
            let field_tag = #field_tag;
            let tag_prefix1 = format!(":{}:", field_tag);
            let tag_prefix2 = format!("{}:", field_tag);
            
            let content = if value.starts_with(&tag_prefix1) {
                &value[tag_prefix1.len()..]
            } else if value.starts_with(&tag_prefix2) {
                &value[tag_prefix2.len()..]
            } else {
                value
            };

            // Parse format: 3!n3!a15d (e.g., "035USD987654,32")
            if content.len() < 9 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: field_tag.to_string(),
                    message: format!("Field {} content too short", field_tag),
                }.into());
            }

            // Extract entry count (first 3 characters: digits)
            let entry_count = content[0..3].to_string();
            
            // Validate entry count is numeric
            if !entry_count.chars().all(|c| c.is_ascii_digit()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: field_tag.to_string(),
                    message: "Entry count must be numeric".to_string(),
                }.into());
            }

            // Extract currency (next 3 characters)
            let currency = content[3..6].to_string().to_uppercase();

            // Validate currency format
            if !currency.chars().all(|c| c.is_ascii_alphabetic()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: field_tag.to_string(),
                    message: "Currency must be alphabetic".to_string(),
                }.into());
            }

            // Extract amount (remaining characters)
            let amount_str = &content[6..];
            let raw_amount = amount_str.to_string();
            
            let amount = amount_str.replace(',', ".").parse::<f64>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: field_tag.to_string(),
                message: "Invalid amount format".to_string(),
            })?;

            Ok(Self {
                entry_count,
                currency,
                amount,
                raw_amount,
            })
        },
        "13D" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":13D:") {
                &value[5..]
            } else if value.starts_with("13D:") {
                &value[4..]
            } else {
                value
            };

            // Parse format: YYMMDDhhmm±hhmm (15 characters)
            if content.len() != 15 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "13D".to_string(),
                    message: "Field 13D must be exactly 15 characters (YYMMDDhhmm±hhmm)".to_string(),
                }.into());
            }

            // Extract date (first 6 characters: YYMMDD)
            let date_str = &content[0..6];
            let year = 2000 + date_str[0..2].parse::<i32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Invalid year in date".to_string(),
            })?;
            let month = date_str[2..4].parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Invalid month in date".to_string(),
            })?;
            let day = date_str[4..6].parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Invalid day in date".to_string(),
            })?;

            let date = chrono::NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| crate::ParseError::InvalidFieldFormat {
                    field_tag: "13D".to_string(),
                    message: "Invalid date".to_string(),
                })?;

            // Extract time (next 4 characters: hhmm)
            let time_str = &content[6..10];
            let hour = time_str[0..2].parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Invalid hour".to_string(),
            })?;
            let minute = time_str[2..4].parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Invalid minute".to_string(),
            })?;

            let time = chrono::NaiveTime::from_hms_opt(hour, minute, 0)
                .ok_or_else(|| crate::ParseError::InvalidFieldFormat {
                    field_tag: "13D".to_string(),
                    message: "Invalid time".to_string(),
                })?;

            // Extract offset sign (character 11)
            let offset_sign = content.chars().nth(10).unwrap();
            if offset_sign != '+' && offset_sign != '-' {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "13D".to_string(),
                    message: "Offset sign must be '+' or '-'".to_string(),
                }.into());
            }

            // Extract offset time (last 4 characters: hhmm)
            let offset_str = &content[11..15];
            let offset_hours = offset_str[0..2].parse::<i32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Invalid offset hours".to_string(),
            })?;
            let offset_minutes = offset_str[2..4].parse::<i32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Invalid offset minutes".to_string(),
            })?;

            let offset_seconds = (offset_hours * 3600 + offset_minutes * 60) * if offset_sign == '+' { 1 } else { -1 };

            Ok(Self {
                date,
                time,
                offset_sign,
                offset_seconds,
            })
        },
        "23" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":23:") {
                &value[4..]
            } else if value.starts_with("23:") {
                &value[3..]
            } else {
                value
            };

            if content.len() < 4 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23".to_string(),
                    message: "Field23 must be at least 4 characters long".to_string(),
                }.into());
            }

            // Extract function code (first 4 characters)
            let function_code = &content[0..4];

            // Check if this is a NOTI function with days
            if function_code == "NOTI" && content.len() >= 6 {
                let potential_days = &content[4..6];
                if potential_days.chars().all(|c| c.is_ascii_digit()) {
                    let days: u8 = potential_days.parse().map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: "23".to_string(),
                        message: "Invalid days format".to_string(),
                    })?;
                    let reference = if content.len() > 6 { &content[6..] } else { "" };
                    return Ok(Self {
                        function_code: function_code.to_string(),
                        days: Some(days),
                        reference: reference.to_string(),
                    });
                }
            }

            // Handle other function codes or NOTI without explicit days
            let reference = if content.len() > 4 { &content[4..] } else { "" };

            Ok(Self {
                function_code: function_code.to_string(),
                days: None,
                reference: reference.to_string(),
            })
        },
        "28" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":28:") {
                &value[4..]
            } else if value.starts_with("28:") {
                &value[3..]
            } else {
                value
            };

            // Parse statement number and optional sequence number
            if let Some(slash_pos) = content.find('/') {
                // Has sequence number
                let statement_str = &content[..slash_pos];
                let sequence_str = &content[slash_pos + 1..];

                if statement_str.is_empty() {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "28".to_string(),
                        message: "Statement number cannot be empty".to_string(),
                    }.into());
                }

                if sequence_str.is_empty() {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "28".to_string(),
                        message: "Sequence number cannot be empty after slash".to_string(),
                    }.into());
                }

                let statement_number = statement_str.parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "28".to_string(),
                    message: "Invalid statement number format".to_string(),
                })?;

                let sequence_number = sequence_str.parse::<u8>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "28".to_string(),
                    message: "Invalid sequence number format".to_string(),
                })?;

                Ok(Self {
                    statement_number,
                    sequence_number: Some(sequence_number),
                })
            } else {
                // Statement number only
                let statement_number = content.parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "28".to_string(),
                    message: "Invalid statement number format".to_string(),
                })?;

                Ok(Self {
                    statement_number,
                    sequence_number: None,
                })
            }
        },
        "28C" => quote! {
            let value = value.trim();
            let content = if value.starts_with(":28C:") {
                &value[5..]
            } else if value.starts_with("28C:") {
                &value[4..]
            } else {
                value
            };

            // Parse statement number and optional sequence number (5-digit)
            if let Some(slash_pos) = content.find('/') {
                // Has sequence number
                let statement_str = &content[..slash_pos];
                let sequence_str = &content[slash_pos + 1..];

                if statement_str.is_empty() {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "28C".to_string(),
                        message: "Statement number cannot be empty".to_string(),
                    }.into());
                }

                if sequence_str.is_empty() {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "28C".to_string(),
                        message: "Sequence number cannot be empty after slash".to_string(),
                    }.into());
                }

                let statement_number = statement_str.parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "28C".to_string(),
                    message: "Invalid statement number format".to_string(),
                })?;

                let sequence_number = sequence_str.parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "28C".to_string(),
                    message: "Invalid sequence number format".to_string(),
                })?;

                Ok(Self {
                    statement_number,
                    sequence_number: Some(sequence_number),
                })
            } else {
                // Statement number only
                let statement_number = content.parse::<u32>().map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "28C".to_string(),
                    message: "Invalid statement number format".to_string(),
                })?;

                Ok(Self {
                    statement_number,
                    sequence_number: None,
                })
            }
        },
        _ => quote! {
            // Generic parsing for other fields
            let value = value.trim();
            let tag = #field_tag.to_lowercase();
            let prefix1 = format!(":{}:", tag);
            let prefix2 = format!("{}:", tag);
            
            let content = if value.starts_with(&prefix1) {
                &value[prefix1.len()..]
            } else if value.starts_with(&prefix2) {
                &value[prefix2.len()..]
            } else {
                value
            };

            Err(crate::ParseError::InvalidFieldFormat {
                field_tag: #field_tag.to_string(),
                message: format!("Parsing not implemented for field {}", #field_tag),
            }.into())
        },
    }
}

/// Generate field-specific serialization implementation
fn generate_serialization_implementation(field_tag: &str) -> proc_macro2::TokenStream {
    match field_tag {
        "12" => quote! {
            format!(":12:{}", self.message_type)
        },
        "20" => quote! {
            format!(":20:{}", self.transaction_reference)
        },
        "21" => quote! {
            format!(":21:{}", self.related_reference)
        },
        "23B" => quote! {
            format!(":23B:{}", self.bank_operation_code)
        },
        "26T" => quote! {
            format!(":26T:{}", self.transaction_type_code)
        },
        "32A" => quote! {
            {
                use chrono::Datelike;
                let date_str = format!("{:02}{:02}{:02}", 
                    self.value_date.year() % 100,
                    self.value_date.month(),
                    self.value_date.day());
                format!(":32A:{}{}{}", date_str, self.currency, self.raw_amount)
            }
        },
        "36" => quote! {
            format!(":36:{}", self.raw_rate)
        },
        "37H" => quote! {
            let neg_indicator = if self.is_negative { "N" } else { "" };
            format!(":37H:{}{}{}", self.rate_indicator, neg_indicator, self.raw_rate)
        },
        "71A" => quote! {
            format!(":71A:{}", self.details_of_charges)
        },
        "77T" => quote! {
            format!(":77T:{}{}/{}", self.envelope_type, self.envelope_format, self.envelope_identifier)
        },
        "90C" | "90D" => quote! {
            format!(":{}:{}{}{}", #field_tag, self.entry_count, self.currency, self.raw_amount)
        },
        "13D" => quote! {
            use chrono::{Datelike, Timelike};
            let date_str = format!("{:02}{:02}{:02}", 
                self.date.year() % 100,
                self.date.month(),
                self.date.day());
            let time_str = format!("{:02}{:02}", self.time.hour(), self.time.minute());
            let offset_hours = self.offset_seconds.abs() / 3600;
            let offset_minutes = (self.offset_seconds.abs() % 3600) / 60;
            let offset_sign = if self.offset_seconds >= 0 { '+' } else { '-' };
            let offset_str = format!("{}{:02}{:02}", offset_sign, offset_hours, offset_minutes);
            format!(":13D:{}{}{}", date_str, time_str, offset_str)
        },
        "23" => quote! {
            match self.days {
                Some(days) => format!(":23:{}{:02}{}", self.function_code, days, self.reference),
                None => format!(":23:{}{}", self.function_code, self.reference),
            }
        },
        "28" => quote! {
            match self.sequence_number {
                Some(seq) => format!(":28:{:05}/{:02}", self.statement_number, seq),
                None => format!(":28:{:05}", self.statement_number),
            }
        },
        "28C" => quote! {
            match self.sequence_number {
                Some(seq) => format!(":28C:{:05}/{:05}", self.statement_number, seq),
                None => format!(":28C:{:05}", self.statement_number),
            }
        },
        _ => quote! {
            // Generic serialization - this needs to be implemented per field
            format!(":{}:NOT_IMPLEMENTED", #field_tag)
        },
    }
} 