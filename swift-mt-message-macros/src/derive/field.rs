use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::component::parser::{
    ComponentSpec, derive_format_spec, extract_option_inner_type, get_base_type, is_bool_type,
    is_char_type, is_f64_type, is_i32_type, is_naive_date_type, is_naive_time_type, is_option_type,
    is_u8_type, is_u32_type, is_vec_type, parse_component_specs,
};

/// Derive macro for SwiftField trait implementation
pub fn derive_swift_field_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract field tag from struct name (Field32A -> 32A)
    let field_tag = name
        .to_string()
        .strip_prefix("Field")
        .unwrap_or(&name.to_string())
        .to_uppercase();

    // Parse component specifications from struct fields
    let components = match extract_components_from_struct(&input) {
        Ok(components) => components,
        Err(e) => {
            return syn::Error::new_spanned(&input, e).to_compile_error().into();
        }
    };

    // Generate the SwiftField implementation
    generate_swift_field_impl(&input, &field_tag, &components)
}

/// Extract component specifications from struct fields
fn extract_components_from_struct(input: &DeriveInput) -> Result<Vec<ComponentSpec>, String> {
    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => parse_component_specs(fields_named),
            _ => Err("SwiftField can only be derived for structs with named fields".to_string()),
        },
        _ => Err("SwiftField can only be derived for structs".to_string()),
    }
}

/// Generate SwiftField implementation using component specifications
fn generate_swift_field_impl(
    input: &DeriveInput,
    field_tag: &str,
    components: &[ComponentSpec],
) -> TokenStream {
    let name = &input.ident;

    // Derive the combined format specification
    let format_spec = derive_format_spec(components);

    // Generate parsing logic from components
    let parse_logic = generate_parse_logic(components, field_tag);

    // Generate formatting logic from components
    let format_logic = generate_format_logic(components);

    // Generate validation logic from components
    let validation_logic = generate_validation_logic(components, field_tag);

    // Generate from_raw method
    let from_raw_logic = generate_from_raw_logic(components, field_tag);

    let expanded = quote! {
        impl crate::SwiftField for #name {
            fn parse(value: &str) -> crate::Result<Self> {
                let value = value.trim();

                // Extract field content using generic regex pattern to remove field tags
                let content = Self::remove_field_tag_prefix(value);

                #parse_logic
            }

            fn to_swift_string(&self) -> String {
                let content = #format_logic;
                // For generic fields, just return the content without field tag prefix
                // The actual field tag will be added at the message level
                if #field_tag.starts_with("GENERIC") || #field_tag.len() > 10 {
                    content
                } else {
                    format!(":{}:{}", #field_tag, content)
                }
            }

            fn validate(&self) -> crate::ValidationResult {
                let mut errors = Vec::new();
                let mut warnings = Vec::new();

                #validation_logic

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

        impl #name {
            /// Parse from raw string content
            pub fn from_raw(content: &str) -> Result<Self, crate::ParseError> {
                #from_raw_logic
            }

            /// Remove field tag prefix using generic regex pattern
            /// Handles patterns like ":50K:", "50K:", ":20:", "32A:", etc.
            fn remove_field_tag_prefix(value: &str) -> &str {
                // Use lazy_static for regex compilation performance
                use std::sync::OnceLock;
                static FIELD_TAG_REGEX: OnceLock<regex::Regex> = OnceLock::new();

                let regex = FIELD_TAG_REGEX.get_or_init(|| {
                    // Pattern matches: optional colon + field identifier + mandatory colon
                    // Field identifier: 1-3 digits optionally followed by 1-2 letters
                    regex::Regex::new(r"^:?([0-9]{1,3}[A-Z]{0,2}):").unwrap()
                });

                if let Some(captures) = regex.find(value) {
                    &value[captures.end()..]
                } else {
                    value
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let content = #format_logic;
                write!(f, "{}", content)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generate parsing logic from component specifications
fn generate_parse_logic(components: &[ComponentSpec], field_tag: &str) -> proc_macro2::TokenStream {
    if components.len() == 1 {
        // Single component case - use the original logic
        let comp = &components[0];
        let field_name = syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site());
        let field_parsing = generate_component_parsing(comp, field_tag);

        quote! {
            Ok(Self {
                #field_name: #field_parsing
            })
        }
    } else {
        // Multiple components case - use positional parsing (same as from_raw_logic)
        let mut parsing_statements = Vec::new();
        let mut position = 0;

        for comp in components {
            let field_name = syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site());
            let is_optional = is_option_type(&comp.field_type) || comp.optional;

            let component_size = get_component_fixed_size(&comp.format);

            let parsing_logic = if component_size > 0 {
                // Fixed size component
                let end_pos = position + component_size;
                let base_parsing =
                    generate_positional_component_parsing(comp, position, end_pos, field_tag);

                if is_optional {
                    quote! {
                        let #field_name = if content.len() > #position {
                            Some(#base_parsing)
                        } else {
                            None
                        };
                    }
                } else {
                    quote! {
                        let #field_name = #base_parsing;
                    }
                }
            } else {
                // Variable size component (takes remaining content)
                let base_parsing = generate_remaining_component_parsing(comp, position, field_tag);

                if is_optional {
                    quote! {
                        let #field_name = if content.len() > #position {
                            let remaining = &content[#position..];
                            if !remaining.is_empty() {
                                Some(#base_parsing)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                    }
                } else {
                    quote! {
                        let #field_name = {
                            let remaining = &content[#position..];
                            #base_parsing
                        };
                    }
                }
            };

            parsing_statements.push(parsing_logic);
            // Only advance position for fixed size components
            if component_size > 0 {
                position += component_size;
            }
        }

        let field_names: Vec<_> = components
            .iter()
            .map(|comp| syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site()))
            .collect();

        quote! {
            #( #parsing_statements )*

            Ok(Self {
                #( #field_names, )*
            })
        }
    }
}

/// Generate parsing logic for a single component
fn generate_component_parsing(comp: &ComponentSpec, field_tag: &str) -> proc_macro2::TokenStream {
    let is_optional = is_option_type(&comp.field_type) || comp.optional;
    let base_parsing = generate_base_component_parsing(comp, field_tag);

    if is_optional {
        // For optional fields, we need to handle the case where the content might be empty or missing
        quote! {
            {
                if content.is_empty() {
                    None
                } else {
                    Some(#base_parsing)
                }
            }
        }
    } else {
        base_parsing
    }
}

/// Generate base parsing logic for a component (without Option wrapper)
fn generate_base_component_parsing(
    comp: &ComponentSpec,
    field_tag: &str,
) -> proc_macro2::TokenStream {
    // First, determine the actual type we're parsing to
    let base_type = get_base_type(&comp.field_type);
    let is_vec = is_vec_type(&comp.field_type)
        || (is_option_type(&comp.field_type)
            && is_vec_type(
                extract_option_inner_type(&comp.field_type).unwrap_or(&comp.field_type),
            ));
    let is_u32 = is_u32_type(base_type);
    let is_f64 = is_f64_type(base_type);
    let is_naive_date = is_naive_date_type(base_type);
    let is_naive_time = is_naive_time_type(base_type);
    let is_char = is_char_type(base_type);
    let is_i32 = is_i32_type(base_type);
    let is_u8 = is_u8_type(base_type);
    let is_bool = is_bool_type(base_type);
    let is_custom_fromstr = is_custom_fromstr_type(base_type);

    // Handle Vec<T> types
    if is_vec {
        return generate_vec_parsing(comp, field_tag);
    }

    // Handle NaiveDate types
    if is_naive_date {
        return generate_naive_date_parsing(field_tag);
    }

    // Handle NaiveTime types
    if is_naive_time {
        return generate_naive_time_parsing(field_tag);
    }

    // Handle char types
    if is_char {
        return generate_char_parsing(field_tag);
    }

    // Handle u32 types
    if is_u32 {
        return generate_u32_parsing(field_tag);
    }

    // Handle i32 types
    if is_i32 {
        return generate_i32_parsing(field_tag);
    }

    // Handle u8 types
    if is_u8 {
        return generate_u8_parsing(field_tag);
    }

    // Handle f64 types
    if is_f64 {
        return generate_f64_parsing(field_tag);
    }

    // Handle bool types
    if is_bool {
        return generate_bool_parsing(field_tag);
    }

    // Handle custom FromStr types (like BIC)
    if is_custom_fromstr {
        return generate_custom_fromstr_parsing(field_tag, base_type);
    }

    // Original format-based parsing for other types
    match comp.format.as_str() {
        "6!n" => generate_date_parsing(field_tag),
        "3!a" => generate_currency_parsing(field_tag),
        "15d" | "12d" => generate_amount_parsing(field_tag),
        "4!c" | "35x" | "16x" | "30x" | "34x" | "11x" | "2!a" | "4!a" | "1!a" | "2!n" | "5n"
        | "8c" => generate_text_parsing(),
        "4!n" => generate_numeric_parsing(field_tag),
        "lines" => generate_lines_parsing(),
        "[+/-]4!n" => generate_utc_offset_parsing(),
        _ => {
            // Generic text parsing for unknown formats
            generate_text_parsing()
        }
    }
}

/// Generate date parsing logic (6!n format)
fn generate_date_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            if content.len() < 6 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Date field too short".to_string(),
                });
            }

            let date_str = &content[0..6];
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
    }
}

/// Generate currency parsing logic (3!a format)
fn generate_currency_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            if content.len() < 3 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Currency code too short".to_string(),
                });
            }
            content[0..3].to_string()
        }
    }
}

/// Generate amount parsing logic (15d, 12d formats)
fn generate_amount_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let amount_str = if content.len() > 3 { &content[3..] } else { content };
            let normalized_amount = amount_str.replace(',', ".");
            normalized_amount.parse::<f64>().map_err(|_| {
                crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Invalid amount format".to_string(),
                }
            })?
        }
    }
}

/// Generate text parsing logic (various alphanumeric formats)
fn generate_text_parsing() -> proc_macro2::TokenStream {
    quote! {
        content.to_string()
    }
}

/// Generate numeric parsing logic (4!n format)
fn generate_numeric_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            if content.len() < 4 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Numeric field too short".to_string(),
                });
            }
            content[0..4].to_string()
        }
    }
}

/// Generate lines parsing logic (multiline fields)
fn generate_lines_parsing() -> proc_macro2::TokenStream {
    quote! {
        content.lines().map(|line| line.to_string()).collect()
    }
}

/// Generate UTC offset parsing logic ([+/-]4!n format)
fn generate_utc_offset_parsing() -> proc_macro2::TokenStream {
    quote! {
        content.to_string()
    }
}

/// Generate Vec<T> parsing logic
fn generate_vec_parsing(comp: &ComponentSpec, _field_tag: &str) -> proc_macro2::TokenStream {
    match comp.format.as_str() {
        "lines" => {
            // Multiline text -> Vec<String>
            quote! {
                content.lines().map(|line| line.to_string()).collect()
            }
        }
        _ => {
            // For other formats, split by delimiter (e.g., comma, space, or newline)
            quote! {
                {
                    if content.is_empty() {
                        Vec::new()
                    } else {
                        // Try different delimiters - newline first, then comma, then space
                        let items: Vec<&str> = if content.contains('\n') {
                            content.lines().collect()
                        } else if content.contains(',') {
                            content.split(',').collect()
                        } else {
                            content.split_whitespace().collect()
                        };

                        items.iter()
                            .map(|item| item.trim().to_string())
                            .filter(|item| !item.is_empty())
                            .collect()
                    }
                }
            }
        }
    }
}

/// Generate u32 parsing logic
fn generate_u32_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let num_str = content.trim();
            num_str.parse::<u32>().map_err(|_| {
                crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: format!("Invalid u32 format: {}", num_str),
                }
            })?
        }
    }
}

/// Generate f64 parsing logic
fn generate_f64_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let num_str = content.trim();
            // Handle SWIFT decimal format (comma as decimal separator)
            let normalized_num = num_str.replace(',', ".");
            normalized_num.parse::<f64>().map_err(|_| {
                crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: format!("Invalid f64 format: {}", num_str),
                }
            })?
        }
    }
}

/// Generate NaiveDate parsing logic
fn generate_naive_date_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let date_str = content.trim();
            if date_str.len() < 6 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Date field too short".to_string(),
                });
            }

            // Handle different date formats
            if date_str.len() == 6 && date_str.chars().all(|c| c.is_ascii_digit()) {
                // YYMMDD format
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
            } else if date_str.len() == 8 && date_str.chars().all(|c| c.is_ascii_digit()) {
                // YYYYMMDD format
                let year = &date_str[0..4];
                let month = &date_str[4..6];
                let day = &date_str[6..8];

                chrono::NaiveDate::parse_from_str(
                    &format!("{}-{}-{}", year, month, day),
                    "%Y-%m-%d"
                ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Invalid date".to_string(),
                    })?
            } else {
                // Try parsing ISO format (YYYY-MM-DD)
                chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                        message: "Invalid date format".to_string(),
                    })?
            }
        }
    }
}

/// Generate NaiveTime parsing logic
fn generate_naive_time_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let time_str = content.trim();

            // Handle different time formats
            if time_str.len() == 4 && time_str.chars().all(|c| c.is_ascii_digit()) {
                // HHMM format
                let hour = &time_str[0..2];
                let minute = &time_str[2..4];

                chrono::NaiveTime::parse_from_str(
                    &format!("{}:{}", hour, minute),
                    "%H:%M"
                ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Invalid time".to_string(),
                })?
            } else if time_str.len() == 6 && time_str.chars().all(|c| c.is_ascii_digit()) {
                // HHMMSS format
                let hour = &time_str[0..2];
                let minute = &time_str[2..4];
                let second = &time_str[4..6];

                chrono::NaiveTime::parse_from_str(
                    &format!("{}:{}:{}", hour, minute, second),
                    "%H:%M:%S"
                ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: "Invalid time".to_string(),
                })?
            } else {
                // Try parsing standard formats
                chrono::NaiveTime::parse_from_str(time_str, "%H:%M:%S")
                    .or_else(|_| chrono::NaiveTime::parse_from_str(time_str, "%H:%M"))
                    .map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid time format".to_string(),
                    })?
            }
        }
    }
}

/// Generate char parsing logic
fn generate_char_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let char_str = content.trim();
            if char_str.len() != 1 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: format!("Expected single character, got: {}", char_str),
                });
            }
            char_str.chars().next().unwrap()
        }
    }
}

/// Generate i32 parsing logic
fn generate_i32_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let num_str = content.trim();
            num_str.parse::<i32>().map_err(|_| {
                crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: format!("Invalid i32 format: {}", num_str),
                }
            })?
        }
    }
}

/// Generate u8 parsing logic
fn generate_u8_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let num_str = content.trim();
            num_str.parse::<u8>().map_err(|_| {
                crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: format!("Invalid u8 format: {}", num_str),
                }
            })?
        }
    }
}

/// Generate bool parsing logic
fn generate_bool_parsing(field_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let bool_str = content.trim().to_lowercase();
            match bool_str.as_str() {
                "true" | "t" | "yes" | "y" | "1" => true,
                "false" | "f" | "no" | "n" | "0" => false,
                _ => {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid boolean format: {}", bool_str),
                    });
                }
            }
        }
    }
}

/// Generate formatting logic from component specifications
fn generate_format_logic(components: &[ComponentSpec]) -> proc_macro2::TokenStream {
    let format_parts = components.iter().map(|comp| {
        let field_name = syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site());
        let is_optional = is_option_type(&comp.field_type) || comp.optional;
        let base_type = get_base_type(&comp.field_type);
        let is_vec = is_vec_type(&comp.field_type)
            || (is_optional
                && is_vec_type(
                    extract_option_inner_type(&comp.field_type).unwrap_or(&comp.field_type),
                ));
        let is_u32 = is_u32_type(base_type);
        let is_f64 = is_f64_type(base_type);
        let is_naive_date = is_naive_date_type(base_type);
        let is_naive_time = is_naive_time_type(base_type);
        let is_char = is_char_type(base_type);
        let is_i32 = is_i32_type(base_type);
        let is_u8 = is_u8_type(base_type);
        let is_bool = is_bool_type(base_type);
        let is_custom_fromstr = is_custom_fromstr_type(base_type);

        let base_format = if is_vec {
            match comp.format.as_str() {
                "lines" => quote! { val.join("\n") },
                _ => quote! { val.join(",") }, // Default to comma-separated for other Vec formats
            }
        } else if is_naive_date {
            // Format NaiveDate based on the component format
            match comp.format.as_str() {
                "6!n" => quote! { val.format("%y%m%d").to_string() },
                "8!n" => quote! { val.format("%Y%m%d").to_string() },
                _ => quote! { val.format("%Y-%m-%d").to_string() },
            }
        } else if is_naive_time {
            // Format NaiveTime based on the component format
            match comp.format.as_str() {
                "4!n" => quote! { val.format("%H%M").to_string() },
                "6!n" => quote! { val.format("%H%M%S").to_string() },
                _ => quote! { val.format("%H:%M:%S").to_string() },
            }
        } else if is_char || is_u32 || is_i32 || is_u8 {
            quote! { val.to_string() }
        } else if is_f64 {
            // Format f64 with SWIFT decimal format (comma as decimal separator)
            quote! { val.to_string().replace('.', ",") }
        } else if is_bool {
            // Format boolean values
            quote! { if *val { "1" } else { "0" }.to_string() }
        } else if is_custom_fromstr {
            // Format custom FromStr types (like BIC)
            quote! { val.to_string() }
        } else {
            // Original format logic based on format specification
            match comp.format.as_str() {
                "6!n" => quote! { val.format("%y%m%d").to_string() },
                "15d" | "12d" => quote! { val.to_string().replace('.', ",") },
                "lines" => quote! { val.join("\n") },
                _ => quote! { val.to_string() },
            }
        };

        if is_optional {
            quote! {
                if let Some(ref val) = self.#field_name {
                    #base_format
                } else {
                    String::new()
                }
            }
        } else {
            quote! {
                {
                    let val = &self.#field_name;
                    #base_format
                }
            }
        }
    });

    if components.len() == 1 {
        let single_part = format_parts.into_iter().next().unwrap();
        quote! { #single_part }
    } else {
        quote! {
            {
                let parts: Vec<String> = vec![#( #format_parts ),*];
                parts.into_iter().filter(|p| !p.is_empty()).collect::<Vec<_>>().join("")
            }
        }
    }
}

/// Generate validation logic from component specifications
fn generate_validation_logic(
    components: &[ComponentSpec],
    field_tag: &str,
) -> proc_macro2::TokenStream {
    let validations = components.iter().map(|comp| {
        let field_name = syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site());
        let rules = &comp.validation_rules;
        let is_optional = is_option_type(&comp.field_type) || comp.optional;

        if rules.is_empty() {
            return quote! {};
        }

        let rule_checks = rules.iter().map(|rule| {
            let base_type = get_base_type(&comp.field_type);
            let is_u32 = is_u32_type(base_type);
            let is_f64 = is_f64_type(base_type);
            let is_i32 = is_i32_type(base_type);
            let is_u8 = is_u8_type(base_type);

            match rule.as_str() {
                "positive_amount" => {
                    if is_u32 || is_u8 {
                        // u32 is always positive, but we can check for zero
                        if is_optional {
                            quote! {
                                if let Some(ref val) = self.#field_name {
                                    if *val == 0 {
                                        warnings.push("Amount is zero".to_string());
                                    }
                                }
                            }
                        } else {
                            quote! {
                                if self.#field_name == 0 {
                                                                    warnings.push("Amount is zero".to_string());
                                }
                            }
                        }
                    } else if is_i32 {
                        // i32 can be negative, so we need to check for positive values
                        if is_optional {
                            quote! {
                                if let Some(ref val) = self.#field_name {
                                    if *val <= 0 {
                                        errors.push(crate::ValidationError::BusinessRuleValidation {
                                            rule_name: "positive_amount".to_string(),
                                            message: format!("Amount must be positive: {}", val),
                                        });
                                    }
                                }
                            }
                        } else {
                            quote! {
                                if self.#field_name <= 0 {
                                    errors.push(crate::ValidationError::BusinessRuleValidation {
                                        rule_name: "positive_amount".to_string(),
                                        message: format!("Amount must be positive: {}", self.#field_name),
                                    });
                                }
                            }
                        }
                    } else if is_f64 {
                        if is_optional {
                            quote! {
                                if let Some(ref val) = self.#field_name {
                                    if *val <= 0.0 {
                                        errors.push(crate::ValidationError::BusinessRuleValidation {
                                            rule_name: "positive_amount".to_string(),
                                            message: format!("Amount must be positive: {}", val),
                                        });
                                    }
                                }
                            }
                        } else {
                            quote! {
                                if self.#field_name <= 0.0 {
                                    errors.push(crate::ValidationError::BusinessRuleValidation {
                                        rule_name: "positive_amount".to_string(),
                                        message: format!("Amount must be positive: {}", self.#field_name),
                                    });
                                }
                            }
                        }
                    } else {
                        // Default behavior for other types
                        if is_optional {
                            quote! {
                                if let Some(ref val) = self.#field_name {
                                    if *val <= 0.0 {
                                        errors.push(crate::ValidationError::BusinessRuleValidation {
                                            rule_name: "positive_amount".to_string(),
                                            message: format!("Amount must be positive: {}", val),
                                        });
                                    }
                                }
                            }
                        } else {
                            quote! {
                                if self.#field_name <= 0.0 {
                                    errors.push(crate::ValidationError::BusinessRuleValidation {
                                        rule_name: "positive_amount".to_string(),
                                        message: format!("Amount must be positive: {}", self.#field_name),
                                    });
                                }
                            }
                        }
                    }
                },
                "currency_code" => {
                    if is_optional {
                        quote! {
                            if let Some(ref val) = self.#field_name {
                                if val.len() != 3 || !val.chars().all(|c| c.is_ascii_alphabetic()) {
                                    errors.push(crate::ValidationError::FormatValidation {
                                        field_tag: #field_tag.to_string(),
                                        message: format!("Invalid currency code: {}", val),
                                    });
                                }
                            }
                        }
                    } else {
                        quote! {
                            if self.#field_name.len() != 3 || !self.#field_name.chars().all(|c| c.is_ascii_alphabetic()) {
                                errors.push(crate::ValidationError::FormatValidation {
                                    field_tag: #field_tag.to_string(),
                                    message: format!("Invalid currency code: {}", self.#field_name),
                                });
                            }
                        }
                    }
                },
                _ => quote! {
                    // Additional validation rules would be implemented here
                },
            }
        });

        quote! {
            #( #rule_checks )*
        }
    });

    quote! {
        #( #validations )*
    }
}

/// Generate from_raw method logic
fn generate_from_raw_logic(
    components: &[ComponentSpec],
    field_tag: &str,
) -> proc_macro2::TokenStream {
    if components.len() == 1 {
        // Single component case
        let comp = &components[0];
        let field_name = syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site());
        let field_parsing = generate_component_parsing(comp, field_tag);

        quote! {
            Ok(Self {
                #field_name: #field_parsing
            })
        }
    } else {
        // Multiple components case - parse sequentially based on format specifications
        let mut parsing_statements = Vec::new();
        let mut position = 0;

        for comp in components {
            let field_name = syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site());
            let is_optional = is_option_type(&comp.field_type) || comp.optional;

            let component_size = get_component_fixed_size(&comp.format);

            let parsing_logic = if component_size > 0 {
                // Fixed size component
                let end_pos = position + component_size;
                let base_parsing =
                    generate_positional_component_parsing(comp, position, end_pos, field_tag);

                if is_optional {
                    quote! {
                        let #field_name = if content.len() > #position {
                            Some(#base_parsing)
                        } else {
                            None
                        };
                    }
                } else {
                    quote! {
                        let #field_name = #base_parsing;
                    }
                }
            } else {
                // Variable size component (takes remaining content)
                let base_parsing = generate_remaining_component_parsing(comp, position, field_tag);

                if is_optional {
                    quote! {
                        let #field_name = if content.len() > #position {
                            let remaining = &content[#position..];
                            if !remaining.is_empty() {
                                Some(#base_parsing)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                    }
                } else {
                    quote! {
                        let #field_name = {
                            let remaining = &content[#position..];
                            #base_parsing
                        };
                    }
                }
            };

            parsing_statements.push(parsing_logic);
            position += component_size;
        }

        let field_names: Vec<_> = components
            .iter()
            .map(|comp| syn::Ident::new(&comp.field_name, proc_macro2::Span::call_site()))
            .collect();

        quote! {
            #( #parsing_statements )*

            Ok(Self {
                #( #field_names, )*
            })
        }
    }
}

/// Get fixed size for a component format, or 0 if variable size
fn get_component_fixed_size(format: &str) -> usize {
    match format {
        "6!n" => 6,                                 // Date: YYMMDD
        "3!a" => 3,                                 // Currency: USD, EUR, etc.
        "3!n" => 3,                                 // Numeric: 3 digits
        "15d" | "12d" => 0,                         // Decimal amounts (variable size)
        "4!a" => 4,                                 // Code: 4 letters
        "4!c" => 4,                                 // Code: 4 alphanumeric
        "4!n" => 4,                                 // Numeric: 4 digits
        "1!a" => 1,                                 // Single letter
        "2!a" => 2,                                 // Two letters
        "2!n" => 2,                                 // Two digits
        "8c" => 8,                                  // 8 characters
        "4!a2!a2!c[3!c]" => 0,                      // BIC code (8 or 11 characters, variable)
        "35x" | "16x" | "30x" | "34x" | "11x" => 0, // Variable size text
        "5n" => 0,                                  // Variable size numeric
        "lines" => 0,                               // Variable size multiline
        "[+/-]4!n" => 5,                            // Sign + 4 digits
        _ => 0,                                     // Unknown format, treat as variable
    }
}

/// Generate parsing logic for a component at a specific position
fn generate_positional_component_parsing(
    comp: &ComponentSpec,
    start_pos: usize,
    end_pos: usize,
    field_tag: &str,
) -> proc_macro2::TokenStream {
    let base_type = get_base_type(&comp.field_type);
    let is_u32 = is_u32_type(base_type);
    let is_f64 = is_f64_type(base_type);
    let is_naive_date = is_naive_date_type(base_type);
    let is_naive_time = is_naive_time_type(base_type);
    let is_char = is_char_type(base_type);
    let is_i32 = is_i32_type(base_type);
    let is_u8 = is_u8_type(base_type);
    let is_bool = is_bool_type(base_type);
    let is_custom_fromstr = is_custom_fromstr_type(base_type);

    // Handle type-specific parsing
    if is_custom_fromstr {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for custom type field".to_string(),
                    });
                }
                let type_str = &content[#start_pos..#end_pos];
                type_str.trim().parse().map_err(|e: String| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Failed to parse {}: {}", stringify!(#base_type), e),
                    }
                })?
            }
        };
    }

    if is_naive_date {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for date field".to_string(),
                    });
                }
                let date_str = &content[#start_pos..#end_pos];

                // For Field11S, we expect YYMMDD format
                if date_str.len() == 6 && date_str.chars().all(|c| c.is_ascii_digit()) {
                    let year = format!("20{}", &date_str[0..2]);
                    let month = &date_str[2..4];
                    let day = &date_str[4..6];

                    let formatted_date = format!("{}-{}-{}", year, month, day);
                    chrono::NaiveDate::parse_from_str(&formatted_date, "%Y-%m-%d")
                        .map_err(|_| crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: format!("Invalid date: {}", formatted_date),
                        })?
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid date format: {}", date_str),
                    });
                }
            }
        };
    }

    if is_naive_time {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for time field".to_string(),
                    });
                }
                let time_str = &content[#start_pos..#end_pos];

                if time_str.len() == 4 && time_str.chars().all(|c| c.is_ascii_digit()) {
                    let hour = &time_str[0..2];
                    let minute = &time_str[2..4];

                    chrono::NaiveTime::parse_from_str(
                        &format!("{}:{}", hour, minute),
                        "%H:%M"
                    ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid time".to_string(),
                    })?
                } else if time_str.len() == 6 && time_str.chars().all(|c| c.is_ascii_digit()) {
                    let hour = &time_str[0..2];
                    let minute = &time_str[2..4];
                    let second = &time_str[4..6];

                    chrono::NaiveTime::parse_from_str(
                        &format!("{}:{}:{}", hour, minute, second),
                        "%H:%M:%S"
                    ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid time".to_string(),
                    })?
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid time format".to_string(),
                    });
                }
            }
        };
    }

    if is_char {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for char field".to_string(),
                    });
                }
                let char_str = &content[#start_pos..#end_pos];
                if char_str.len() != 1 {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Expected single character, got: {}", char_str),
                    });
                }
                char_str.chars().next().unwrap()
            }
        };
    }

    if is_u32 {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for u32 field".to_string(),
                    });
                }
                let num_str = &content[#start_pos..#end_pos];
                num_str.trim().parse::<u32>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid u32 format: {}", num_str),
                    }
                })?
            }
        };
    }

    if is_i32 {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for i32 field".to_string(),
                    });
                }
                let num_str = &content[#start_pos..#end_pos];
                num_str.trim().parse::<i32>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid i32 format: {}", num_str),
                    }
                })?
            }
        };
    }

    if is_u8 {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for u8 field".to_string(),
                    });
                }
                let num_str = &content[#start_pos..#end_pos];
                num_str.trim().parse::<u8>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid u8 format: {}", num_str),
                    }
                })?
            }
        };
    }

    if is_bool {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for bool field".to_string(),
                    });
                }
                let bool_str = &content[#start_pos..#end_pos];
                let bool_val = bool_str.trim().to_lowercase();
                match bool_val.as_str() {
                    "true" | "t" | "yes" | "y" | "1" => true,
                    "false" | "f" | "no" | "n" | "0" => false,
                    _ => {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: format!("Invalid boolean format: {}", bool_str),
                        });
                    }
                }
            }
        };
    }

    if is_f64 {
        return quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for f64 field".to_string(),
                    });
                }
                let num_str = &content[#start_pos..#end_pos];
                let normalized_num = num_str.trim().replace(',', ".");
                normalized_num.parse::<f64>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid f64 format: {}", num_str),
                    }
                })?
            }
        };
    }

    // Original format-based parsing
    match comp.format.as_str() {
        "3!a" => quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short for currency field".to_string(),
                    });
                }
                content[#start_pos..#end_pos].to_string()
            }
        },
        _ => quote! {
            {
                if content.len() < #end_pos {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Content too short".to_string(),
                    });
                }
                content[#start_pos..#end_pos].to_string()
            }
        },
    }
}

/// Check if type is a custom type that implements FromStr (like BIC)
fn is_custom_fromstr_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            matches!(segment.ident.to_string().as_str(), "BIC")
        } else {
            false
        }
    } else {
        false
    }
}

/// Generate parsing logic for custom FromStr types
fn generate_custom_fromstr_parsing(
    field_tag: &str,
    base_type: &syn::Type,
) -> proc_macro2::TokenStream {
    quote! {
        {
            content.trim().parse().map_err(|e: String| {
                crate::ParseError::InvalidFieldFormat {
                    field_tag: #field_tag.to_string(),
                    message: format!("Failed to parse {}: {}", stringify!(#base_type), e),
                }
            })?
        }
    }
}

/// Generate parsing logic for a component that takes remaining content
fn generate_remaining_component_parsing(
    comp: &ComponentSpec,
    _start_pos: usize,
    field_tag: &str,
) -> proc_macro2::TokenStream {
    let base_type = get_base_type(&comp.field_type);
    let is_vec = is_vec_type(&comp.field_type)
        || (is_option_type(&comp.field_type)
            && is_vec_type(
                extract_option_inner_type(&comp.field_type).unwrap_or(&comp.field_type),
            ));
    let is_u32 = is_u32_type(base_type);
    let is_f64 = is_f64_type(base_type);
    let is_naive_date = is_naive_date_type(base_type);
    let is_naive_time = is_naive_time_type(base_type);
    let is_char = is_char_type(base_type);
    let is_i32 = is_i32_type(base_type);
    let is_u8 = is_u8_type(base_type);
    let is_bool = is_bool_type(base_type);
    let is_custom_fromstr = is_custom_fromstr_type(base_type);

    // Handle type-specific parsing
    if is_custom_fromstr {
        return quote! {
            {
                let type_str = remaining.trim();
                type_str.parse().map_err(|e: String| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Failed to parse {}: {}", stringify!(#base_type), e),
                    }
                })?
            }
        };
    }

    if is_vec {
        return match comp.format.as_str() {
            "lines" => quote! {
                remaining.lines().map(|line| line.to_string()).collect()
            },
            _ => quote! {
                {
                    if remaining.is_empty() {
                        Vec::new()
                    } else {
                        // Try different delimiters - newline first, then comma, then space
                        let items: Vec<&str> = if remaining.contains('\n') {
                            remaining.lines().collect()
                        } else if remaining.contains(',') {
                            remaining.split(',').collect()
                        } else {
                            remaining.split_whitespace().collect()
                        };

                        items.iter()
                            .map(|item| item.trim().to_string())
                            .filter(|item| !item.is_empty())
                            .collect()
                    }
                }
            },
        };
    }

    if is_naive_date {
        return quote! {
            {
                let date_str = remaining.trim();
                if date_str.len() == 6 && date_str.chars().all(|c| c.is_ascii_digit()) {
                    let year = format!("20{}", &date_str[0..2]);
                    let month = &date_str[2..4];
                    let day = &date_str[4..6];

                    let formatted_date = format!("{}-{}-{}", year, month, day);
                    chrono::NaiveDate::parse_from_str(&formatted_date, "%Y-%m-%d")
                        .map_err(|_| crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: format!("Invalid date: {}", formatted_date),
                        })?
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid date format: {}", date_str),
                    });
                }
            }
        };
    }

    if is_naive_time {
        return quote! {
            {
                let time_str = remaining.trim();
                if time_str.len() == 4 && time_str.chars().all(|c| c.is_ascii_digit()) {
                    let hour = &time_str[0..2];
                    let minute = &time_str[2..4];

                    chrono::NaiveTime::parse_from_str(
                        &format!("{}:{}", hour, minute),
                        "%H:%M"
                    ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid time".to_string(),
                    })?
                } else if time_str.len() == 6 && time_str.chars().all(|c| c.is_ascii_digit()) {
                    let hour = &time_str[0..2];
                    let minute = &time_str[2..4];
                    let second = &time_str[4..6];

                    chrono::NaiveTime::parse_from_str(
                        &format!("{}:{}:{}", hour, minute, second),
                        "%H:%M:%S"
                    ).map_err(|_| crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid time".to_string(),
                    })?
                } else {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid time format".to_string(),
                    });
                }
            }
        };
    }

    if is_char {
        return quote! {
            {
                let char_str = remaining.trim();
                if char_str.len() != 1 {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Expected single character, got: {}", char_str),
                    });
                }
                char_str.chars().next().unwrap()
            }
        };
    }

    if is_u32 {
        return quote! {
            {
                let num_str = remaining.trim();
                num_str.parse::<u32>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid u32 format: {}", num_str),
                    }
                })?
            }
        };
    }

    if is_i32 {
        return quote! {
            {
                let num_str = remaining.trim();
                num_str.parse::<i32>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid i32 format: {}", num_str),
                    }
                })?
            }
        };
    }

    if is_u8 {
        return quote! {
            {
                let num_str = remaining.trim();
                num_str.parse::<u8>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid u8 format: {}", num_str),
                    }
                })?
            }
        };
    }

    if is_bool {
        return quote! {
            {
                let bool_str = remaining.trim().to_lowercase();
                match bool_str.as_str() {
                    "true" | "t" | "yes" | "y" | "1" => true,
                    "false" | "f" | "no" | "n" | "0" => false,
                    _ => {
                        return Err(crate::ParseError::InvalidFieldFormat {
                            field_tag: #field_tag.to_string(),
                            message: format!("Invalid boolean format: {}", bool_str),
                        });
                    }
                }
            }
        };
    }

    if is_f64 {
        return quote! {
            {
                let num_str = remaining.trim();
                let normalized_num = num_str.replace(',', ".");
                normalized_num.parse::<f64>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: format!("Invalid f64 format: {}", num_str),
                    }
                })?
            }
        };
    }

    // Original format-based parsing
    match comp.format.as_str() {
        "15d" | "12d" => quote! {
            {
                let amount_str = &remaining;
                let normalized_amount = amount_str.replace(',', ".");
                normalized_amount.parse::<f64>().map_err(|_| {
                    crate::ParseError::InvalidFieldFormat {
                        field_tag: #field_tag.to_string(),
                        message: "Invalid amount format".to_string(),
                    }
                })?
            }
        },
        "lines" => quote! {
            remaining.lines().map(|line| line.to_string()).collect()
        },
        _ => quote! {
            remaining.to_string()
        },
    }
}
