use syn::{Attribute, Meta};

pub fn extract_format_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("format") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Some(nested) = meta_list.tokens.to_string().strip_prefix('"') {
                    if let Some(value) = nested.strip_suffix('"') {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn extract_format_attribute_from_struct(attrs: &[Attribute]) -> Option<String> {
    extract_format_attribute(attrs)
}

pub fn extract_field_option_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("field_option") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Some(nested) = meta_list.tokens.to_string().strip_prefix('"') {
                    if let Some(value) = nested.strip_suffix('"') {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn extract_message_type_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("swift_message") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                // Parse: mt = "103"
                if let Some(eq_pos) = tokens.find('=') {
                    let value_part = tokens[eq_pos + 1..].trim();
                    if let Some(nested) = value_part.strip_prefix('"') {
                        if let Some(value) = nested.strip_suffix('"') {
                            return Some(value.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn extract_field_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("field") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Some(nested) = meta_list.tokens.to_string().strip_prefix('"') {
                    if let Some(value) = nested.strip_suffix('"') {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
} 