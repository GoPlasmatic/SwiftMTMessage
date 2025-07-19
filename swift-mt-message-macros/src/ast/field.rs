//! AST structures and parsing for field definitions

use crate::error::{MacroError, MacroResult};
use crate::format::FormatSpec;
use crate::utils::attributes::extract_component_attribute;
use crate::utils::types::{is_option_type, is_vec_type};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{DeriveInput, Field, Fields, FieldsNamed, Ident, Type};

/// Parsed field structure information
/// 
/// Represents a complete field definition parsed from a Rust struct or enum
/// that uses the `#[derive(SwiftField)]` macro. This structure contains all
/// the information needed to generate the SwiftField trait implementation.
#[derive(Debug, Clone)]
pub struct FieldDefinition {
    /// The struct or enum name (e.g., `Field20`, `Field50`)
    pub name: Ident,
    /// Field structure type (struct or enum)
    pub kind: FieldKind,
    /// Span for error reporting
    #[allow(dead_code)]
    pub span: Span,
}

/// Field structure type
#[derive(Debug, Clone)]
pub enum FieldKind {
    /// Simple struct with components
    Struct(StructField),
    /// Enum with variants
    Enum(EnumField),
}

/// Struct field definition
#[derive(Debug, Clone)]
pub struct StructField {
    /// List of component fields
    pub components: Vec<Component>,
}

/// Enum field definition  
#[derive(Debug, Clone)]
pub struct EnumField {
    /// List of enum variants
    pub variants: Vec<EnumVariant>,
}

/// Enum variant information
#[derive(Debug, Clone)]
pub struct EnumVariant {
    /// Variant identifier (e.g., "A", "F", "K")
    pub ident: Ident,
    /// The type this variant wraps
    pub type_name: Type,
    /// Span for error reporting
    #[allow(dead_code)]
    pub span: Span,
}

/// Component field within a struct
/// 
/// Represents a single component of a SWIFT field, extracted from a struct field
/// with a `#[component("format")]` attribute. Components define the individual
/// parts that make up a complete SWIFT field.
/// 
/// ## Example
/// For a field like `Field32A`, this would represent each component:
/// ```ignore
/// struct Field32A {
///     #[component("6!n")]   // Component: date
///     date: String,
///     #[component("3!a")]   // Component: currency  
///     currency: String,
///     #[component("15d")]   // Component: amount
///     amount: f64,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Component {
    /// Field name (e.g., `date`, `currency`, `amount`)
    pub name: Ident,
    /// Field type (e.g., `String`, `f64`, `Option<String>`)
    pub field_type: Type,
    /// SWIFT format specification (e.g., "6!n", "3!a", "15d")
    pub format: FormatSpec,
    /// Whether the field is optional (Option<T>)
    pub is_optional: bool,
    /// Whether the field is repetitive (Vec<T>)
    pub is_repetitive: bool,
    /// Span for error reporting
    #[allow(dead_code)]
    pub span: Span,
}

impl FieldDefinition {
    /// Parse field definition from derive input
    pub fn parse(input: &DeriveInput) -> MacroResult<Self> {
        let name = input.ident.clone();
        let span = input.ident.span();

        let kind = match &input.data {
            syn::Data::Struct(data_struct) => {
                let struct_field = StructField::parse(&data_struct.fields)?;
                FieldKind::Struct(struct_field)
            }
            syn::Data::Enum(data_enum) => {
                let enum_field = EnumField::parse(data_enum)?;
                FieldKind::Enum(enum_field)
            }
            syn::Data::Union(_) => {
                return Err(MacroError::unsupported_type(
                    span,
                    "union",
                    "SwiftField can only be derived for structs and enums",
                ));
            }
        };

        Ok(FieldDefinition { name, kind, span })
    }
}

impl StructField {
    /// Parse struct field from syn::Fields
    fn parse(fields: &Fields) -> MacroResult<Self> {
        match fields {
            Fields::Named(named_fields) => {
                let components = Component::parse_all(named_fields)?;
                Ok(StructField { components })
            }
            Fields::Unnamed(_) => Err(MacroError::unsupported_type(
                Span::call_site(),
                "tuple struct",
                "SwiftField requires named fields",
            )),
            Fields::Unit => Err(MacroError::unsupported_type(
                Span::call_site(),
                "unit struct",
                "SwiftField requires fields with components",
            )),
        }
    }
}

impl EnumField {
    /// Parse enum field from syn::DataEnum
    fn parse(data_enum: &syn::DataEnum) -> MacroResult<Self> {
        let mut variants = Vec::new();

        for variant in &data_enum.variants {
            let enum_variant = EnumVariant::parse(variant)?;
            variants.push(enum_variant);
        }

        if variants.is_empty() {
            return Err(MacroError::invalid_attribute(
                Span::call_site(),
                "enum",
                "empty",
                "at least one variant",
            ));
        }

        Ok(EnumField { variants })
    }
}

impl EnumVariant {
    /// Parse enum variant from syn::Variant
    fn parse(variant: &syn::Variant) -> MacroResult<Self> {
        let ident = variant.ident.clone();
        let span = variant.ident.span();

        // Extract the wrapped type from the variant
        let type_name = match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                fields.unnamed.first().unwrap().ty.clone()
            }
            Fields::Named(_) => {
                return Err(MacroError::unsupported_type(
                    span,
                    "named fields in enum variant",
                    "enum variants must wrap a single type: A(TypeA)",
                ));
            }
            Fields::Unit => {
                return Err(MacroError::unsupported_type(
                    span,
                    "unit enum variant",
                    "enum variants must wrap a single type: A(TypeA)",
                ));
            }
            Fields::Unnamed(_) => {
                return Err(MacroError::unsupported_type(
                    span,
                    "multiple fields in enum variant",
                    "enum variants must wrap a single type: A(TypeA)",
                ));
            }
        };

        Ok(EnumVariant {
            ident,
            type_name,
            span,
        })
    }
}

impl Component {
    /// Parse all components from named fields
    fn parse_all(fields: &FieldsNamed) -> MacroResult<Vec<Self>> {
        let mut components = Vec::new();

        for field in &fields.named {
            let component = Component::parse(field)?;
            components.push(component);
        }

        Ok(components)
    }

    /// Parse a single component from a field
    fn parse(field: &Field) -> MacroResult<Self> {
        let name = field
            .ident
            .clone()
            .ok_or_else(|| MacroError::internal(field.span(), "Field must have a name"))?;

        let field_type = field.ty.clone();
        let span = field.span();

        // Extract format specification from #[component("format")] attribute
        let format_spec = extract_component_attribute(&field.attrs)?;
        let format = FormatSpec::parse(&format_spec)?;

        // Determine if field is optional or repetitive
        let is_optional = is_option_type(&field_type);
        let is_repetitive = is_vec_type(&field_type);

        Ok(Component {
            name,
            field_type,
            format,
            is_optional,
            is_repetitive,
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_parse_simple_struct() {
        let input: DeriveInput = syn::parse2(quote! {
            struct Field20 {
                #[component("16x")]
                reference: String,
            }
        })
        .unwrap();

        let definition = FieldDefinition::parse(&input).unwrap();
        assert_eq!(definition.name, "Field20");

        if let FieldKind::Struct(struct_field) = definition.kind {
            assert_eq!(struct_field.components.len(), 1);
            assert_eq!(struct_field.components[0].name, "reference");
        } else {
            panic!("Expected struct field");
        }
    }
}
