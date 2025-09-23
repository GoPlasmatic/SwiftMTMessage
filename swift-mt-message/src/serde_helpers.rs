//! Custom serialization helpers for SWIFT MT fields
//!
//! This module provides custom serde implementations to ensure consistent
//! JSON serialization behavior across the library, particularly for numeric fields.

/// Custom serializer/deserializer for f64 amount fields
///
/// This module ensures consistent serialization of f64 amounts:
/// - Whole numbers are serialized without decimal points (e.g., 1000 not 1000.0)
/// - Fractional numbers preserve their decimal precision
/// - Deserialization accepts both integer and float formats for compatibility
pub mod amount_serializer {
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serialize f64 amounts, using integer format for whole numbers
    pub fn serialize<S>(amount: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Check if the amount is a whole number
        if amount.fract() == 0.0 && amount.is_finite() {
            // For whole numbers, serialize as integer to avoid ".0" suffix
            // This handles amounts up to i64::MAX (about 9 quintillion)
            if *amount >= i64::MIN as f64 && *amount <= i64::MAX as f64 {
                serializer.serialize_i64(*amount as i64)
            } else {
                // For very large whole numbers, fall back to f64
                serializer.serialize_f64(*amount)
            }
        } else {
            // For fractional amounts, preserve the decimal
            serializer.serialize_f64(*amount)
        }
    }

    /// Deserialize amounts, accepting both integer and float formats
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Helper enum to accept both integer and float JSON values
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum AmountValue {
            Integer(i64),
            Float(f64),
        }

        // Accept either format and convert to f64
        match AmountValue::deserialize(deserializer)? {
            AmountValue::Integer(i) => Ok(i as f64),
            AmountValue::Float(f) => Ok(f),
        }
    }
}

/// Custom serializer for optional f64 amount fields
pub mod optional_amount_serializer {
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serialize optional f64 amounts
    pub fn serialize<S>(amount: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match amount {
            Some(val) => super::amount_serializer::serialize(val, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Deserialize optional amounts
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OptionalAmount {
            Some(#[serde(deserialize_with = "super::amount_serializer::deserialize")] f64),
            None,
        }

        match OptionalAmount::deserialize(deserializer)? {
            OptionalAmount::Some(val) => Ok(Some(val)),
            OptionalAmount::None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestAmount {
        #[serde(with = "amount_serializer")]
        amount: f64,
    }

    #[test]
    fn test_whole_number_serialization() {
        let test = TestAmount { amount: 1000.0 };
        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, r#"{"amount":1000}"#);
    }

    #[test]
    fn test_fractional_serialization() {
        let test = TestAmount { amount: 1234.56 };
        let json = serde_json::to_string(&test).unwrap();
        assert_eq!(json, r#"{"amount":1234.56}"#);
    }

    #[test]
    fn test_deserialize_integer() {
        let json = r#"{"amount":1000}"#;
        let test: TestAmount = serde_json::from_str(json).unwrap();
        assert_eq!(test.amount, 1000.0);
    }

    #[test]
    fn test_deserialize_float() {
        let json = r#"{"amount":1000.0}"#;
        let test: TestAmount = serde_json::from_str(json).unwrap();
        assert_eq!(test.amount, 1000.0);
    }

    #[test]
    fn test_round_trip_whole() {
        let original = TestAmount { amount: 60000000.0 };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TestAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
        assert_eq!(json, r#"{"amount":60000000}"#);
    }

    #[test]
    fn test_round_trip_fractional() {
        let original = TestAmount { amount: 123.45 };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TestAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
