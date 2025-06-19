use quote::quote;

/// Component parser types for different SWIFT field components
#[derive(Debug, Clone)]
pub enum ComponentParser {
    SwiftDate,          // 6!n - YYMMDD format
    Currency,           // 3!a - ISO 4217 currency codes
    Amount,             // 15d - Decimal amounts with comma separator
    DecimalAmount,      // 12d - Decimal amounts up to 12 digits
    AlphaCode,          // 4!c - Alphanumeric codes
    AlphaCode3,         // 3!c - 3-character alphanumeric codes
    Alphanumeric,       // 16x - Alphanumeric strings
    Text35,             // 35x - 35-character text strings
    Bic,                // BIC codes
    SingleChar,         // 1!a - Single alphabetic character
    OptionalSingleChar, // [1!a] - Optional single alphabetic character
    EntryCount,         // 3!n - 3-digit entry count parser
    Numeric4,           // 4!n - 4-digit numeric pattern
    SingleSign,         // 1!x - Single sign character (+/-)
    EnvelopeContent,    // 1!a/1!a/35x - Envelope content pattern (Field77T)
    InterestRate,       // 1!a[N]12d - Interest rate pattern (Field37H)
    // New patterns for Phase 1
    StatementSequence,  // 5n[/2n] - Statement number with optional sequence (Field28)
    StatementSequence5, // 5n[/5n] - Statement number with optional 5-digit sequence (Field28C)
    DateTimeOffset,     // 6!n4!n1!x4!n - Date, time, sign, offset (Field13D)
    FunctionCodeRef,    // 3!a[2!n]11x - Function code with optional days and reference (Field23)
}

impl ComponentParser {
    pub fn from_pattern(pattern: &str) -> Option<Self> {
        match pattern {
            "6!n" => Some(ComponentParser::SwiftDate),
            "3!a" => Some(ComponentParser::Currency),
            "15d" => Some(ComponentParser::Amount),
            "4!c" => Some(ComponentParser::AlphaCode),
            "3!c" => Some(ComponentParser::AlphaCode3),
            "16x" => Some(ComponentParser::Alphanumeric),
            "1!a" => Some(ComponentParser::SingleChar),
            "BIC" => Some(ComponentParser::Bic),
            "3!n" => Some(ComponentParser::EntryCount), // New pattern
            _ => None,
        }
    }

    pub fn from_parser_name(parser: &str) -> Option<Self> {
        match parser {
            "swift_date" => Some(ComponentParser::SwiftDate),
            "currency" => Some(ComponentParser::Currency),
            "amount" => Some(ComponentParser::Amount),
            "alpha_code" => Some(ComponentParser::AlphaCode),
            "alphanumeric" => Some(ComponentParser::Alphanumeric),
            "single_char" => Some(ComponentParser::SingleChar),
            "bic" => Some(ComponentParser::Bic),
            "entry_count" => Some(ComponentParser::EntryCount), // New parser
            _ => None,
        }
    }

    pub fn generate_parse_logic(
        &self,
        field_name: &syn::Ident,
        start_pos: usize,
        field_tag: &str,
    ) -> proc_macro2::TokenStream {
        match self {
            ComponentParser::SwiftDate => quote! {
                #field_name: {
                    let date_str = &content[#start_pos..#start_pos + 6];
                    if !date_str.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Date must be 6 digits".to_string(),
                        });
                    }

                    let year = format!("20{}", &date_str[0..2]);
                    let month = &date_str[2..4];
                    let day = &date_str[4..6];

                    chrono::NaiveDate::parse_from_str(
                        &format!("{}-{}-{}", year, month, day),
                        "%Y-%m-%d"
                    ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid date".to_string(),
                    })?
                }
            },
            ComponentParser::Currency => quote! {
                #field_name: content[#start_pos..#start_pos + 3].to_string()
            },
            ComponentParser::Amount => quote! {
                #field_name: {
                    let amount_str = &content[#start_pos..];

                    // Parse SWIFT decimal format (comma as decimal separator)
                    let normalized_amount = amount_str.replace(',', ".");
                    normalized_amount.parse::<f64>().map_err(|_| {
                        crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Invalid amount format".to_string(),
                        }
                    })?
                }
            },
            ComponentParser::DecimalAmount => quote! {
                #field_name: {
                    let rate_str = &content[#start_pos..];

                    // Validate length for 12d pattern (max 12 digits)
                    if rate_str.len() > 12 {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: format!("Exchange rate too long: {} characters (max 12)", rate_str.len()),
                        });
                    }

                    // Check for invalid characters
                    if !rate_str.chars().all(|c| c.is_ascii_digit() || c == ',' || c == '.') {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Exchange rate contains invalid characters".to_string(),
                        });
                    }

                    // Parse SWIFT decimal format (comma as decimal separator)
                    let normalized_rate = rate_str.replace(',', ".");
                    let value = normalized_rate.parse::<f64>().map_err(|_| {
                        crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Invalid exchange rate format".to_string(),
                        }
                    })?;

                    // Validate positive value
                    if value <= 0.0 {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: format!("Exchange rate must be positive, got: {}", value),
                        });
                    }

                    value
                }
            },

            ComponentParser::EntryCount => quote! {
                #field_name: {
                    let entry_count_str = &content[#start_pos..#start_pos + 3];
                    // For Field12 message type, we keep it as String
                    entry_count_str.to_string()
                }
            },
            ComponentParser::OptionalSingleChar => quote! {
                #field_name: {
                    if content.len() > #start_pos {
                        let ch = content.chars().nth(#start_pos).unwrap();
                        if ch.is_alphabetic() && (ch == 'D' || ch == 'C') {
                            Some(ch)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            },
            ComponentParser::Text35 => quote! {
                #field_name: content[#start_pos..].to_string()
            },
            ComponentParser::AlphaCode => quote! {
                #field_name: content[#start_pos..].to_string()
            },
            ComponentParser::AlphaCode3 => quote! {
                #field_name: {
                    let code_str = &content[#start_pos..#start_pos + 3];
                    if !code_str.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "3-character alphanumeric code must contain only alphanumeric characters".to_string(),
                        });
                    }
                    code_str.to_uppercase()
                }
            },
            ComponentParser::Alphanumeric => quote! {
                #field_name: content[#start_pos..].to_string()
            },
            ComponentParser::Bic => quote! {
                #field_name: content[#start_pos..].to_string()
            },
            ComponentParser::SingleChar => quote! {
                #field_name: content[#start_pos..].to_string()
            },
            ComponentParser::Numeric4 => quote! {
                #field_name: {
                    let numeric_str = &content[#start_pos..#start_pos + 4];
                    if !numeric_str.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "4-digit numeric field must contain only digits".to_string(),
                        });
                    }
                    numeric_str.to_string()
                }
            },
            ComponentParser::SingleSign => quote! {
                #field_name: {
                    let sign_char = &content[#start_pos..#start_pos + 1];
                    if sign_char != "+" && sign_char != "-" {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Sign must be '+' or '-'".to_string(),
                        });
                    }
                    sign_char.to_string()
                }
            },
            ComponentParser::EnvelopeContent => quote! {
                // Special handling for envelope content pattern 1!a/1!a/35x
                envelope_type: {
                    let envelope_type = content.chars().nth(0)
                        .ok_or_else(|| crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Missing envelope type".to_string(),
                        })?.to_string().to_uppercase();

                    if !envelope_type.chars().all(|c| c.is_ascii_alphabetic()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Envelope type must be alphabetic".to_string(),
                        });
                    }
                    envelope_type
                },
                envelope_format: {
                    let envelope_format = content.chars().nth(1)
                        .ok_or_else(|| crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Missing envelope format".to_string(),
                        })?.to_string().to_uppercase();

                    if !envelope_format.chars().all(|c| c.is_ascii_alphabetic()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Envelope format must be alphabetic".to_string(),
                        });
                    }
                    envelope_format
                },
                envelope_identifier: {
                    if content.len() < 4 || content.chars().nth(2) != Some('/') {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Missing separator '/' after envelope codes".to_string(),
                        });
                    }

                    let identifier = content[3..].to_string();
                    if identifier.is_empty() {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Envelope identifier cannot be empty".to_string(),
                        });
                    }

                    if identifier.len() > 35 {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Envelope identifier cannot exceed 35 characters".to_string(),
                        });
                    }

                    identifier
                }
            },
            ComponentParser::InterestRate => quote! {
                // Interest rate parsing handled specially - this shouldn't be called
                #field_name: content.to_string()
            },
            ComponentParser::StatementSequence => quote! {
                #field_name: {
                    let statement_sequence_str = &content[#start_pos..#start_pos + 5];
                    if !statement_sequence_str.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Statement sequence must be numeric".to_string(),
                        });
                    }
                    statement_sequence_str.parse::<u32>().map_err(|_| {
                        crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Invalid statement sequence format".to_string(),
                        }
                    })?
                }
            },
            ComponentParser::StatementSequence5 => quote! {
                #field_name: {
                    let statement_sequence_str = &content[#start_pos..#start_pos + 5];
                    if !statement_sequence_str.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Statement sequence must be numeric".to_string(),
                        });
                    }
                    statement_sequence_str.parse::<u32>().map_err(|_| {
                        crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Invalid statement sequence format".to_string(),
                        }
                    })?
                }
            },
            ComponentParser::DateTimeOffset => quote! {
                #field_name: {
                    let date_str = &content[#start_pos..#start_pos + 6];
                    if !date_str.chars().all(|c| c.is_ascii_digit()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Date must be 6 digits".to_string(),
                        });
                    }

                    let year = format!("20{}", &date_str[0..2]);
                    let month = &date_str[2..4];
                    let day = &date_str[4..6];

                    chrono::NaiveDate::parse_from_str(
                        &format!("{}-{}-{}", year, month, day),
                        "%Y-%m-%d"
                    ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid date".to_string(),
                    })?
                }
            },
            ComponentParser::FunctionCodeRef => quote! {
                #field_name: {
                    let function_code_ref_str = &content[#start_pos..#start_pos + 3];
                    if !function_code_ref_str.chars().all(|c| c.is_ascii_alphanumeric()) {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: "Function code reference must be alphanumeric".to_string(),
                        });
                    }
                    function_code_ref_str.to_string()
                }
            },
        }
    }

    pub fn generate_serialize_logic(&self, field_name: &syn::Ident) -> proc_macro2::TokenStream {
        match self {
            ComponentParser::SwiftDate => quote! {
                format!("{:02}{:02}{:02}", self.#field_name.year() % 100, self.#field_name.month(), self.#field_name.day())
            },
            ComponentParser::Currency => quote! {
                self.#field_name.clone()
            },
            ComponentParser::Amount => quote! {
                format!("{:.2}", self.#field_name).replace('.', ",")
            },
            ComponentParser::DecimalAmount => quote! {
                self.raw_rate.clone()
            },
            ComponentParser::EntryCount => quote! {
                format!("{:03}", self.#field_name)
            },
            ComponentParser::OptionalSingleChar => quote! {
                self.#field_name.map(|c| c.to_string()).unwrap_or_else(|| String::new())
            },
            ComponentParser::AlphaCode => quote! {
                self.#field_name.clone()
            },
            ComponentParser::AlphaCode3 => quote! {
                self.#field_name.clone()
            },
            ComponentParser::Alphanumeric => quote! {
                self.#field_name.clone()
            },
            ComponentParser::Bic => quote! {
                self.#field_name.clone()
            },
            ComponentParser::SingleChar => quote! {
                self.#field_name.to_string()
            },
            ComponentParser::Text35 => quote! {
                self.#field_name.clone()
            },
            ComponentParser::Numeric4 => quote! {
                self.#field_name.clone()
            },
            ComponentParser::SingleSign => quote! {
                self.#field_name.clone()
            },
            ComponentParser::EnvelopeContent => quote! {
                format!("{}{}/{}", self.envelope_type, self.envelope_format, self.envelope_identifier)
            },
            ComponentParser::InterestRate => quote! {
                {
                    let negative_part = if self.is_negative { "N" } else { "" };
                    format!("{}{}{}", self.rate_indicator, negative_part, self.raw_rate)
                }
            },
            ComponentParser::StatementSequence => quote! {
                format!("{:05}", self.#field_name)
            },
            ComponentParser::StatementSequence5 => quote! {
                format!("{:05}", self.#field_name)
            },
            ComponentParser::DateTimeOffset => quote! {
                format!("{:02}{:02}{:02}", self.#field_name.year() % 100, self.#field_name.month(), self.#field_name.day())
            },
            ComponentParser::FunctionCodeRef => quote! {
                self.#field_name.clone()
            },
        }
    }

    pub fn component_length(&self) -> usize {
        match self {
            ComponentParser::SwiftDate => 6,
            ComponentParser::Currency => 3,
            ComponentParser::Amount => 0, // Variable length, takes rest of string
            ComponentParser::DecimalAmount => 0, // Variable length, takes rest of string
            ComponentParser::AlphaCode => 4,
            ComponentParser::AlphaCode3 => 3,
            ComponentParser::Alphanumeric => 16,
            ComponentParser::SingleChar => 1,
            ComponentParser::OptionalSingleChar => 1, // Optional single char
            ComponentParser::Bic => 11,               // Max BIC length
            ComponentParser::EntryCount => 3,         // Fixed 3-digit entry count
            ComponentParser::Text35 => 35,            // Up to 35 characters
            ComponentParser::Numeric4 => 4,           // Fixed 4-digit numeric
            ComponentParser::SingleSign => 1,         // Single sign character
            ComponentParser::EnvelopeContent => 0,    // Variable length envelope content
            ComponentParser::InterestRate => 0,       // Variable length interest rate
            ComponentParser::StatementSequence => 5,   // Fixed 5-digit statement sequence
            ComponentParser::StatementSequence5 => 5,  // Fixed 5-digit statement sequence
            ComponentParser::DateTimeOffset => 6,      // Fixed 6-digit date
            ComponentParser::FunctionCodeRef => 3,     // Fixed 3-character function code reference
        }
    }
} 