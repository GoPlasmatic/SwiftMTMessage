use syn::{Attribute, Meta};

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
