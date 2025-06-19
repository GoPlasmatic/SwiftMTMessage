#[derive(Debug, Clone)]
pub struct ComponentSpec {
    pub key: String,
    pub length: usize,
    pub format_type: String,
    pub start_pos: usize,
}

/// Enhanced format specification parser for complex patterns like "6!n3!a15d"
pub struct FormatSpecParser {
    pub spec: String,
    pub components: Vec<ComponentSpec>,
}

impl FormatSpecParser {
    pub fn parse(format_spec: &str) -> Result<Self, String> {
        let mut components = Vec::new();
        let mut current_pos = 0;
        let mut chars = format_spec.char_indices().peekable();

        while let Some((_, ch)) = chars.next() {
            if ch == '[' {
                // Parse optional component [1!a]
                let mut optional_component = String::new();
                while let Some((_, next_ch)) = chars.next() {
                    if next_ch == ']' {
                        break;
                    }
                    optional_component.push(next_ch);
                }

                let component_key = format!("[{}]", optional_component);
                let parsed_length = 1; // Optional single char

                components.push(ComponentSpec {
                    key: component_key,
                    length: parsed_length,
                    format_type: "optional".to_string(),
                    start_pos: current_pos,
                });

                current_pos += parsed_length; // Optional component takes 1 position when present
            } else if ch.is_ascii_digit() {
                // Parse length specifier
                let mut length = String::new();
                length.push(ch);

                // Collect remaining digits
                while let Some((_, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        length.push(*next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Parse format specifier (!n, !a, d, etc.)
                let format_type = if chars.peek().map(|(_, c)| *c) == Some('!') {
                    chars.next(); // consume '!'
                    if let Some((_, type_ch)) = chars.next() {
                        format!("!{}", type_ch)
                    } else {
                        return Err("Expected format type after !".to_string());
                    }
                } else if let Some((_, type_ch)) = chars.next() {
                    type_ch.to_string()
                } else {
                    return Err("Expected format type".to_string());
                };

                let component_key = format!("{}{}", length, format_type);
                let parsed_length: usize = length.parse().unwrap_or(0);

                // Update position for next component (except for variable length components like 15d)
                let is_variable_length = format_type.as_str() == "d";

                components.push(ComponentSpec {
                    key: component_key,
                    length: parsed_length,
                    format_type,
                    start_pos: current_pos,
                });

                if !is_variable_length {
                    current_pos += parsed_length;
                }
            }
        }

        Ok(FormatSpecParser {
            spec: format_spec.to_string(),
            components,
        })
    }
} 