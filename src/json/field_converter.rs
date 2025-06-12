//! Field-specific JSON conversion utilities
//!
//! This module provides streamlined implementations for field conversion
//! with reduced duplication and improved maintainability.

use crate::errors::{ParseError, Result};
use crate::field_parser::SwiftFieldContainer;
use crate::mt_models::fields::beneficiary::Field59;
use crate::mt_models::fields::institutions::{
    Field52, Field53, Field54, Field55, Field56, Field57,
};
use crate::mt_models::fields::ordering_customer::Field50;
use serde_json::{Map, Value};

/// Trait for field types that can be converted to/from JSON
pub trait FieldJsonConverter {
    /// Extract field data for JSON serialization
    fn extract_to_json(&self) -> Result<Value>;

    /// Recreate field from JSON data
    fn recreate_from_json(tag: &str, value: &Value) -> Result<Self>
    where
        Self: Sized;
}

/// Helper function to create JSON serialization errors
fn json_serialize_error<T: std::fmt::Display>(field_type: &str, error: T) -> ParseError {
    ParseError::JsonError {
        message: format!("Failed to serialize {}: {}", field_type, error),
    }
}

/// Helper function to create JSON deserialization errors
fn json_deserialize_error<T: std::fmt::Display>(field_tag: &str, error: T) -> ParseError {
    ParseError::JsonError {
        message: format!("Failed to parse {}: {}", field_tag, error),
    }
}

/// Generic function to serialize enum variants
fn serialize_enum_variant<T: serde::Serialize>(variant: &T, field_type: &str) -> Result<Value> {
    serde_json::to_value(variant).map_err(|e| json_serialize_error(field_type, e))
}

/// Generic function to deserialize enum variants
fn deserialize_enum_variant<T: serde::de::DeserializeOwned>(
    value: &Value,
    field_tag: &str,
) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(|e| json_deserialize_error(field_tag, e))
}

/// Extract field data from a SwiftFieldContainer for JSON serialization
pub fn extract_field_data(field: &SwiftFieldContainer) -> Result<Value> {
    match field {
        // Fields with custom JSON conversion (enum variants)
        SwiftFieldContainer::Field50(f) => f.extract_to_json(),
        SwiftFieldContainer::Field52(f) => f.extract_to_json(),
        SwiftFieldContainer::Field53(f) => f.extract_to_json(),
        SwiftFieldContainer::Field54(f) => f.extract_to_json(),
        SwiftFieldContainer::Field55(f) => f.extract_to_json(),
        SwiftFieldContainer::Field56(f) => f.extract_to_json(),
        SwiftFieldContainer::Field57(f) => f.extract_to_json(),
        SwiftFieldContainer::Field59(f) => f.extract_to_json(),
        
        // Simple field types - serialize directly without enum wrapper
        SwiftFieldContainer::Field13C(f) => serialize_enum_variant(f, "Field13C"),
        SwiftFieldContainer::Field20(f) => serialize_enum_variant(f, "Field20"),
        SwiftFieldContainer::Field23B(f) => serialize_enum_variant(f, "Field23B"),
        SwiftFieldContainer::Field23E(f) => serialize_enum_variant(f, "Field23E"),
        SwiftFieldContainer::Field26T(f) => serialize_enum_variant(f, "Field26T"),
        SwiftFieldContainer::Field32A(f) => serialize_enum_variant(f, "Field32A"),
        SwiftFieldContainer::Field33B(f) => serialize_enum_variant(f, "Field33B"),
        SwiftFieldContainer::Field36(f) => serialize_enum_variant(f, "Field36"),
        SwiftFieldContainer::Field51A(f) => serialize_enum_variant(f, "Field51A"),
        SwiftFieldContainer::Field70(f) => serialize_enum_variant(f, "Field70"),
        SwiftFieldContainer::Field71A(f) => serialize_enum_variant(f, "Field71A"),
        SwiftFieldContainer::Field71F(f) => serialize_enum_variant(f, "Field71F"),
        SwiftFieldContainer::Field71G(f) => serialize_enum_variant(f, "Field71G"),
        SwiftFieldContainer::Field72(f) => serialize_enum_variant(f, "Field72"),
        SwiftFieldContainer::Field77B(f) => serialize_enum_variant(f, "Field77B"),

        // Unknown fields get special handling
        SwiftFieldContainer::Unknown(f) => {
            let mut map = Map::new();
            map.insert("tag".to_string(), Value::String(f.tag.clone()));
            map.insert("content".to_string(), Value::String(f.content.clone()));
            Ok(Value::Object(map))
        }
    }
}

/// Field recreation registry for dynamic dispatch
type FieldRecreator = fn(&str, &Value) -> Result<SwiftFieldContainer>;

/// Get the appropriate field recreator based on field tag prefix
fn get_field_recreator(tag: &str) -> Option<FieldRecreator> {
    let prefix = &tag[..tag.len().min(2)];
    match prefix {
        "50" => Some(|tag, value| {
            let field = Field50::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field50(field))
        }),
        "52" => Some(|tag, value| {
            let field = Field52::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field52(field))
        }),
        "53" => Some(|tag, value| {
            let field = Field53::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field53(field))
        }),
        "54" => Some(|tag, value| {
            let field = Field54::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field54(field))
        }),
        "55" => Some(|tag, value| {
            let field = Field55::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field55(field))
        }),
        "56" => Some(|tag, value| {
            let field = Field56::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field56(field))
        }),
        "57" => Some(|tag, value| {
            let field = Field57::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field57(field))
        }),
        "59" => Some(|tag, value| {
            let field = Field59::recreate_from_json(tag, value)?;
            Ok(SwiftFieldContainer::Field59(field))
        }),
        _ => None,
    }
}

/// Recreate a SwiftFieldContainer from tag and JSON value
pub fn recreate_field_container(tag: &str, value: &Value) -> Result<SwiftFieldContainer> {
    if let Some(recreator) = get_field_recreator(tag) {
        recreator(tag, value)
    } else {
        // Handle simple field types
        match tag {
            "13C" => {
                let field: crate::mt_models::fields::common::Field13C = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field13C(field))
            },
            "20" => {
                let field: crate::mt_models::fields::common::Field20 = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field20(field))
            },
            "23B" => {
                let field: crate::mt_models::fields::common::Field23B = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field23B(field))
            },
            "23E" => {
                let field: crate::mt_models::fields::common::Field23E = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field23E(field))
            },
            "26T" => {
                let field: crate::mt_models::fields::common::Field26T = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field26T(field))
            },
            "32A" => {
                let field: crate::mt_models::fields::common::Field32A = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field32A(field))
            },
            "33B" => {
                let field: crate::mt_models::fields::common::Field33B = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field33B(field))
            },
            "36" => {
                let field: crate::mt_models::fields::common::Field36 = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field36(field))
            },
            "51A" => {
                let field: crate::mt_models::fields::institutions::Field51A = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field51A(field))
            },
            "70" => {
                let field: crate::mt_models::fields::common::Field70 = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field70(field))
            },
            "71A" => {
                let field: crate::mt_models::fields::charges::Field71A = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field71A(field))
            },
            "71F" => {
                let field: crate::mt_models::fields::charges::Field71F = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field71F(field))
            },
            "71G" => {
                let field: crate::mt_models::fields::charges::Field71G = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field71G(field))
            },
            "72" => {
                let field: crate::mt_models::fields::common::Field72 = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field72(field))
            },
            "77B" => {
                let field: crate::mt_models::fields::common::Field77B = deserialize_enum_variant(value, tag)?;
                Ok(SwiftFieldContainer::Field77B(field))
            },
            _ => recreate_unknown_field_fallback(tag, value),
        }
    }
}

fn recreate_unknown_field_fallback(tag: &str, value: &Value) -> Result<SwiftFieldContainer> {
    Ok(SwiftFieldContainer::Unknown(
        crate::field_parser::UnknownField {
            tag: tag.to_string(),
            content: value.to_string(),
        },
    ))
}

/// Macro to reduce boilerplate for field variant implementations
macro_rules! impl_field_json_converter {
    ($field_type:ty, $field_name:literal, {
        $($variant:ident($variant_type:ty) => $variant_tag:literal),+ $(,)?
    }) => {
        impl FieldJsonConverter for $field_type {
            fn extract_to_json(&self) -> Result<Value> {
                match self {
                    $(
                        Self::$variant(f) => serialize_enum_variant(f, $field_name),
                    )+
                }
            }

            fn recreate_from_json(tag: &str, value: &Value) -> Result<Self> {
                match tag {
                    $(
                        $variant_tag => {
                            let field: $variant_type = deserialize_enum_variant(value, tag)?;
                            Ok(Self::$variant(field))
                        },
                    )+
                    _ => Err(ParseError::JsonError {
                        message: format!("Unknown {} variant: {}", $field_name, tag),
                    }),
                }
            }
        }
    };
}

// Apply the macro to reduce boilerplate
impl_field_json_converter!(Field50, "Field50", {
    A(crate::mt_models::fields::ordering_customer::Field50A) => "50A",
    F(crate::mt_models::fields::ordering_customer::Field50F) => "50F",
    K(crate::mt_models::fields::ordering_customer::Field50K) => "50K",
});

impl_field_json_converter!(Field52, "Field52", {
    A(crate::mt_models::fields::institutions::Field52A) => "52A",
    D(crate::mt_models::fields::institutions::Field52D) => "52D",
});

impl_field_json_converter!(Field53, "Field53", {
    A(crate::mt_models::fields::institutions::Field53A) => "53A",
    B(crate::mt_models::fields::institutions::Field53B) => "53B",
    D(crate::mt_models::fields::institutions::Field53D) => "53D",
});

impl_field_json_converter!(Field54, "Field54", {
    A(crate::mt_models::fields::institutions::Field54A) => "54A",
    B(crate::mt_models::fields::institutions::Field54B) => "54B",
    D(crate::mt_models::fields::institutions::Field54D) => "54D",
});

impl_field_json_converter!(Field55, "Field55", {
    A(crate::mt_models::fields::institutions::Field55A) => "55A",
    B(crate::mt_models::fields::institutions::Field55B) => "55B",
    D(crate::mt_models::fields::institutions::Field55D) => "55D",
});

impl_field_json_converter!(Field56, "Field56", {
    A(crate::mt_models::fields::institutions::Field56A) => "56A",
    C(crate::mt_models::fields::institutions::Field56C) => "56C",
    D(crate::mt_models::fields::institutions::Field56D) => "56D",
});

impl_field_json_converter!(Field57, "Field57", {
    A(crate::mt_models::fields::institutions::Field57A) => "57A",
    B(crate::mt_models::fields::institutions::Field57B) => "57B",
    C(crate::mt_models::fields::institutions::Field57C) => "57C",
    D(crate::mt_models::fields::institutions::Field57D) => "57D",
});

impl_field_json_converter!(Field59, "Field59", {
    A(crate::mt_models::fields::beneficiary::Field59A) => "59A",
    NoOption(crate::mt_models::fields::beneficiary::Field59Basic) => "59",
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mt_models::fields::ordering_customer::Field50K;

    #[test]
    fn test_field_recreator_registry() {
        assert!(get_field_recreator("50A").is_some());
        assert!(get_field_recreator("59").is_some());
        assert!(get_field_recreator("99Z").is_none());
    }

    #[test]
    fn test_error_helpers() {
        let error = json_serialize_error("TestField", "test error");
        assert!(matches!(error, ParseError::JsonError { .. }));

        let error = json_deserialize_error("50A", "invalid data");
        assert!(matches!(error, ParseError::JsonError { .. }));
    }

    #[test]
    fn test_generic_serialization() {
        let field = Field50K {
            name_and_address: vec!["John Doe".to_string()],
        };

        let result = serialize_enum_variant(&field, "Field50K");
        assert!(result.is_ok());
    }

    #[test]
    fn test_field_json_no_extra_nesting() {
        use crate::mt_models::fields::common::{Field20, Field23B, Field32A};
        use crate::mt_models::fields::charges::Field71A;
        use crate::field_parser::SwiftField;
        
        // Test Field20 - should have no extra nesting
        let field20 = Field20::parse("FT21001234567890").unwrap();
        let container = SwiftFieldContainer::Field20(field20);
        let json_value = extract_field_data(&container).unwrap();
        
        // Should have transaction_reference directly, not wrapped in Field20
        assert!(json_value.get("transaction_reference").is_some());
        assert!(json_value.get("Field20").is_none(), "Field20 should not have extra nesting layer");
        
        // Test Field23B - should have no extra nesting
        let field23b = Field23B::parse("CRED").unwrap();
        let container = SwiftFieldContainer::Field23B(field23b);
        let json_value = extract_field_data(&container).unwrap();
        
        // Should have bank_operation_code directly, not wrapped in Field23B
        assert!(json_value.get("bank_operation_code").is_some());
        assert!(json_value.get("Field23B").is_none(), "Field23B should not have extra nesting layer");
        
        // Test Field32A - should have no extra nesting  
        let field32a = Field32A::parse("240101USD1000,00").unwrap();
        let container = SwiftFieldContainer::Field32A(field32a);
        let json_value = extract_field_data(&container).unwrap();
        
        // Should have currency, amount, etc. directly, not wrapped in Field32A
        assert!(json_value.get("currency").is_some());
        assert!(json_value.get("amount").is_some());
        assert!(json_value.get("Field32A").is_none(), "Field32A should not have extra nesting layer");
        
        // Test Field71A - should have no extra nesting
        let field71a = Field71A::parse("OUR").unwrap();
        let container = SwiftFieldContainer::Field71A(field71a);
        let json_value = extract_field_data(&container).unwrap();
        
        // Should have details_of_charges directly, not wrapped in Field71A
        assert!(json_value.get("details_of_charges").is_some());
        assert!(json_value.get("Field71A").is_none(), "Field71A should not have extra nesting layer");
    }
}
