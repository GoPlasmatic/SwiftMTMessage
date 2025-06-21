use syn::{Attribute, FieldsNamed};

/// Component specification parsed from #[component] attribute
#[derive(Debug, Clone)]
pub struct ComponentSpec {
    /// SWIFT format specification (e.g., "6!n", "3!a", "15d")
    pub format: String,
    /// Whether this component is optional
    pub optional: bool,
    /// Validation rules to apply (e.g., ["date_format", "currency_code"])
    pub validation_rules: Vec<String>,
    /// Field name this component applies to
    pub field_name: String,
    /// Field type
    pub field_type: syn::Type,
}

/// Specification for a sequence field
#[derive(Debug, Clone)]
pub struct SequenceSpec {
    /// Sequence identifier (e.g., "A", "B")
    pub sequence_id: String,
    /// Whether this sequence is repetitive
    pub repetitive: bool,
    /// Field name this sequence applies to
    pub field_name: String,
    /// Field type (must be Vec<T> for repetitive sequences)
    pub field_type: syn::Type,
}

/// Field specification - either a component or a sequence
#[derive(Debug, Clone)]
pub enum FieldSpec {
    Component(ComponentSpec),
    Sequence(SequenceSpec),
}

/// Parse field specifications from struct fields
pub fn parse_field_specs(fields: &FieldsNamed) -> Result<Vec<FieldSpec>, String> {
    let mut specs = Vec::new();

    for field in &fields.named {
        let field_name = field
            .ident
            .as_ref()
            .ok_or("Field must have a name")?
            .to_string();

        // Check for #[sequence(...)] attribute first
        if let Some(sequence_attr) = find_sequence_attribute(&field.attrs) {
            let spec = parse_sequence_attribute(sequence_attr, field_name, field.ty.clone())?;
            specs.push(FieldSpec::Sequence(spec));
        }
        // Then check for #[component(...)] attribute
        else if let Some(component_attr) = find_component_attribute(&field.attrs) {
            let spec = parse_component_attribute(component_attr, field_name, field.ty.clone())?;
            specs.push(FieldSpec::Component(spec));
        }
        // Check for #[field(...)] attribute (new unified approach)
        else if let Some(field_attr) = find_field_attribute(&field.attrs) {
            let spec = parse_field_attribute(field_attr, field_name, field.ty.clone())?;
            specs.push(spec);
        }
    }

    if specs.is_empty() {
        return Err(
            "No field attributes found. SwiftMessage requires field specifications.".to_string(),
        );
    }

    Ok(specs)
}

/// Parse component specifications from struct fields (legacy support)
pub fn parse_component_specs(fields: &FieldsNamed) -> Result<Vec<ComponentSpec>, String> {
    let field_specs = parse_field_specs(fields)?;

    let components: Vec<ComponentSpec> = field_specs
        .into_iter()
        .filter_map(|spec| match spec {
            FieldSpec::Component(comp) => Some(comp),
            FieldSpec::Sequence(_) => None,
        })
        .collect();

    if components.is_empty() {
        return Err("No component specifications found.".to_string());
    }

    Ok(components)
}

/// Find the #[sequence(...)] attribute in a list of attributes
fn find_sequence_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("sequence"))
}

/// Find the #[field(...)] attribute in a list of attributes
fn find_field_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("field"))
}

/// Parse a #[sequence(...)] attribute
fn parse_sequence_attribute(
    attr: &Attribute,
    field_name: String,
    field_type: syn::Type,
) -> Result<SequenceSpec, String> {
    let mut sequence_id = None;
    let mut repetitive = false;

    if let syn::Meta::List(meta_list) = &attr.meta {
        let tokens_str = meta_list.tokens.to_string();

        // Parse sequence ID (first quoted string)
        if let Some(start) = tokens_str.find('"') {
            if let Some(end) = tokens_str[start + 1..].find('"') {
                sequence_id = Some(tokens_str[start + 1..start + 1 + end].to_string());
            }
        }

        // Check for repetitive flag
        if tokens_str.contains("repetitive") {
            repetitive = true;
        }
    }

    let sequence_id = sequence_id.ok_or("Sequence must specify an ID (e.g., \"A\", \"B\")")?;

    Ok(SequenceSpec {
        sequence_id,
        repetitive,
        field_name,
        field_type,
    })
}

/// Parse a #[field(...)] attribute (unified approach)
fn parse_field_attribute(
    attr: &Attribute,
    field_name: String,
    field_type: syn::Type,
) -> Result<FieldSpec, String> {
    if let syn::Meta::List(meta_list) = &attr.meta {
        let tokens_str = meta_list.tokens.to_string();

        // Check if this is a sequence specification
        if tokens_str.contains("repetitive") || tokens_str.contains("sequence") {
            // Parse as sequence
            let sequence_spec =
                parse_sequence_from_field_attr(&tokens_str, field_name, field_type)?;
            Ok(FieldSpec::Sequence(sequence_spec))
        } else {
            // Parse as component
            let component_spec =
                parse_component_from_field_attr(&tokens_str, field_name, field_type)?;
            Ok(FieldSpec::Component(component_spec))
        }
    } else {
        Err("Field attribute must have parameters".to_string())
    }
}

/// Parse sequence specification from #[field(...)] attribute
fn parse_sequence_from_field_attr(
    tokens_str: &str,
    field_name: String,
    field_type: syn::Type,
) -> Result<SequenceSpec, String> {
    let mut sequence_id = None;
    let mut repetitive = false;

    // Parse sequence ID (first quoted string)
    if let Some(start) = tokens_str.find('"') {
        if let Some(end) = tokens_str[start + 1..].find('"') {
            sequence_id = Some(tokens_str[start + 1..start + 1 + end].to_string());
        }
    }

    // Check for repetitive flag
    if tokens_str.contains("repetitive") {
        repetitive = true;
    }

    let sequence_id = sequence_id.unwrap_or_else(|| field_name.clone());

    Ok(SequenceSpec {
        sequence_id,
        repetitive,
        field_name,
        field_type,
    })
}

/// Parse component specification from #[field(...)] attribute
fn parse_component_from_field_attr(
    tokens_str: &str,
    field_name: String,
    field_type: syn::Type,
) -> Result<ComponentSpec, String> {
    let mut format = None;
    let mut optional = false;
    let mut validation_rules = Vec::new();

    // Parse format string (first quoted string)
    if let Some(start) = tokens_str.find('"') {
        if let Some(end) = tokens_str[start + 1..].find('"') {
            format = Some(tokens_str[start + 1..start + 1 + end].to_string());
        }
    }

    // Check for optional/mandatory flags
    if tokens_str.contains("optional") {
        optional = true;
    }
    // mandatory is the default, but we can be explicit

    // Parse validation rules
    if tokens_str.contains("validate") {
        validation_rules = parse_validation_rules_from_string(tokens_str)?;
    }

    let format = format.ok_or("Field must specify a format string or be a sequence")?;

    Ok(ComponentSpec {
        format,
        optional,
        validation_rules,
        field_name,
        field_type,
    })
}

/// Parse validation rules from the token string
fn parse_validation_rules_from_string(tokens_str: &str) -> Result<Vec<String>, String> {
    let mut rules = Vec::new();

    // Look for validate = [...]
    if let Some(validate_pos) = tokens_str.find("validate") {
        let after_validate = &tokens_str[validate_pos..];

        // Look for opening bracket
        if let Some(bracket_start) = after_validate.find('[') {
            if let Some(bracket_end) = after_validate.find(']') {
                let rules_content = &after_validate[bracket_start + 1..bracket_end];

                // Split by comma and clean up quotes
                for rule in rules_content.split(',') {
                    let clean_rule = rule.trim().trim_matches('"').trim();
                    if !clean_rule.is_empty() {
                        rules.push(clean_rule.to_string());
                    }
                }
            }
        } else {
            // Look for single quoted rule: validate = "rule"
            let after_equals = if let Some(eq_pos) = after_validate.find('=') {
                &after_validate[eq_pos + 1..]
            } else {
                after_validate
            };

            if let Some(quote_start) = after_equals.find('"') {
                if let Some(quote_end) = after_equals[quote_start + 1..].find('"') {
                    let rule = &after_equals[quote_start + 1..quote_start + 1 + quote_end];
                    rules.push(rule.to_string());
                }
            }
        }
    }

    Ok(rules)
}

/// Generate the combined format specification from all components
pub fn derive_format_spec(components: &[ComponentSpec]) -> String {
    components
        .iter()
        .map(|comp| {
            if comp.optional {
                format!("[{}]", comp.format)
            } else {
                comp.format.clone()
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Check if a field type is Option<T>
pub fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Extract the inner type from Option<T>
pub fn extract_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

/// Check if a field type is Vec<T>
pub fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// Extract the inner type from Vec<T>
pub fn extract_vec_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

/// Check if a field type is u32
pub fn is_u32_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "u32";
        }
    }
    false
}

/// Check if a field type is f64
pub fn is_f64_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "f64";
        }
    }
    false
}

/// Get the base type for a field, handling Option<T> and Vec<T> wrappers
pub fn get_base_type(ty: &syn::Type) -> &syn::Type {
    // First check if it's Option<T>
    if let Some(inner) = extract_option_inner_type(ty) {
        // Could be Option<Vec<T>> or Option<u32>, etc.
        return get_base_type(inner);
    }

    // Then check if it's Vec<T>
    if let Some(inner) = extract_vec_inner_type(ty) {
        return inner;
    }

    // Otherwise return the type as-is
    ty
}

/// Check if a field type is NaiveDate
pub fn is_naive_date_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "NaiveDate";
        }
    }
    false
}

/// Check if a field type is NaiveTime
pub fn is_naive_time_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "NaiveTime";
        }
    }
    false
}

/// Check if a field type is char
pub fn is_char_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "char";
        }
    }
    false
}

/// Check if a field type is i32
pub fn is_i32_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "i32";
        }
    }
    false
}

/// Check if a field type is u8
pub fn is_u8_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "u8";
        }
    }
    false
}

/// Check if a field type is bool
pub fn is_bool_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "bool";
        }
    }
    false
}

/// Check if a type is likely a SwiftMessage (custom struct, not a field type)
pub fn is_swift_message_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();
            // SwiftMessage types typically start with MT or are custom structs
            // They don't start with Generic or Field prefixes
            return type_name.starts_with("MT")
                || (!type_name.starts_with("Generic")
                    && !type_name.starts_with("Field")
                    && !is_primitive_type(&type_name));
        }
    }
    false
}

/// Check if a type name represents a primitive type
fn is_primitive_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "u8" | "u16"
            | "u32"
            | "u64"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "isize"
            | "f32"
            | "f64"
            | "bool"
            | "char"
            | "String"
            | "NaiveDate"
            | "NaiveTime"
            | "NaiveDateTime"
    )
}

/// Check if a Vec<T> contains SwiftMessage types
pub fn is_vec_of_swift_messages(ty: &syn::Type) -> bool {
    if let Some(inner_type) = extract_vec_inner_type(ty) {
        return is_swift_message_type(inner_type);
    }
    false
}

/// Find the #[component] attribute in a list of attributes
fn find_component_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("component"))
}

/// Parse a single #[component(...)] attribute using a simplified approach
fn parse_component_attribute(
    attr: &Attribute,
    field_name: String,
    field_type: syn::Type,
) -> Result<ComponentSpec, String> {
    let mut format = None;
    let mut optional = false;
    let mut validation_rules = Vec::new();

    // Simple parsing approach: parse the tokens manually
    if let syn::Meta::List(meta_list) = &attr.meta {
        let tokens_str = meta_list.tokens.to_string();

        // Parse format string (first quoted string)
        if let Some(start) = tokens_str.find('"') {
            if let Some(end) = tokens_str[start + 1..].find('"') {
                format = Some(tokens_str[start + 1..start + 1 + end].to_string());
            }
        }

        // Check for optional flag
        if tokens_str.contains("optional") {
            optional = true;
        }

        // Parse validation rules (look for validate = [...])
        if tokens_str.contains("validate") {
            validation_rules = parse_validation_rules_from_string(&tokens_str)?;
        }
    }

    let format = format.ok_or("Component must specify a format string")?;

    Ok(ComponentSpec {
        format,
        optional,
        validation_rules,
        field_name,
        field_type,
    })
}
