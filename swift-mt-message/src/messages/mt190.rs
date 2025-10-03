use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT190: Advice of Charges, Interest and Other Adjustments
// Used to advise charges, interest and other adjustments that have been
// debited or credited to an account.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT190 {
    // Transaction Reference Number
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Related Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Account Identification
    #[serde(rename = "25")]
    pub field_25: Field25NoOption,

    // Value Date, Currency Code, Amount (can be 32C or 32D for MT190)
    #[serde(flatten)]
    pub field_32: Field32AmountCD,

    // Ordering Institution (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Details of Charges
    #[serde(rename = "71B")]
    pub field_71b: Field71B,

    // Sender to Receiver Information (optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,
}

impl MT190 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "190");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_25 = parser.parse_field::<Field25NoOption>("25")?;

        // Parse amount - can be 32C or 32D for credit/debit adjustments
        let field_32 = parser.parse_variant_field::<Field32AmountCD>("32")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;

        // Parse mandatory field 71B
        let field_71b = parser.parse_field::<Field71B>("71B")?;

        // Parse optional field 72
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        Ok(MT190 {
            field_20,
            field_21,
            field_25,
            field_32,
            field_52,
            field_71b,
            field_72,
        })
    }

    /// Static validation rules for MT190
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F25", "description": "Field 25 must contain valid account identification"},
            {"id": "F32", "description": "Field 32C or 32D must contain valid currency and amount"},
            {"id": "F71B", "description": "Field 71B must contain at least one line of charge details"}
        ]}"#
    }

    /// Validate the message instance according to MT190 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT190: Field 20 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let related_ref = &self.field_21.reference;
        if related_ref.starts_with('/') || related_ref.ends_with('/') || related_ref.contains("//")
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT190: Field 21 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 71B has content
        if self.field_71b.details.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT190: Field 71B must contain at least one line of charge details"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT190
impl crate::traits::SwiftMessageBody for MT190 {
    fn message_type() -> &'static str {
        "190"
    }

    fn from_fields(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
    ) -> crate::SwiftResult<Self> {
        // Collect all fields with their positions
        let mut all_fields: Vec<(String, String, usize)> = Vec::new();
        for (tag, values) in fields {
            for (value, position) in values {
                all_fields.push((tag.clone(), value, position));
            }
        }

        // Sort by position to preserve field order
        all_fields.sort_by_key(|(_, _, pos)| *pos);

        // Reconstruct block4 in the correct order
        let mut block4 = String::new();
        for (tag, value, _) in all_fields {
            block4.push_str(&format!(":{}:{}\n", tag, value));
        }
        Self::parse_from_block4(&block4)
    }

    fn from_fields_with_config(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
        _config: &crate::errors::ParserConfig,
    ) -> std::result::Result<crate::errors::ParseResult<Self>, crate::errors::ParseError> {
        match Self::from_fields(fields) {
            Ok(msg) => Ok(crate::errors::ParseResult::Success(msg)),
            Err(e) => Err(e),
        }
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        use crate::traits::SwiftField;
        use chrono::Datelike;
        let mut fields = std::collections::HashMap::new();

        // Add mandatory fields
        fields.insert("20".to_string(), vec![self.field_20.reference.clone()]);
        fields.insert("21".to_string(), vec![self.field_21.reference.clone()]);
        fields.insert("25".to_string(), vec![self.field_25.authorisation.clone()]);

        // Add amount field (32C or 32D)
        match &self.field_32 {
            Field32AmountCD::C(field_32c) => {
                fields.insert(
                    "32C".to_string(),
                    vec![format!(
                        "{:02}{:02}{:02}{}{}",
                        field_32c.value_date.year() % 100,
                        field_32c.value_date.month(),
                        field_32c.value_date.day(),
                        field_32c.currency,
                        field_32c.amount.to_string().replace('.', ",")
                    )],
                );
            }
            Field32AmountCD::D(field_32d) => {
                fields.insert(
                    "32D".to_string(),
                    vec![format!(
                        "{:02}{:02}{:02}{}{}",
                        field_32d.value_date.year() % 100,
                        field_32d.value_date.month(),
                        field_32d.value_date.day(),
                        field_32d.currency,
                        field_32d.amount.to_string().replace('.', ",")
                    )],
                );
            }
        }

        // Add optional fields
        if let Some(ref field_52) = self.field_52 {
            match field_52 {
                Field52OrderingInstitution::A(f) => {
                    fields.insert("52A".to_string(), vec![f.to_swift_value()]);
                }
                Field52OrderingInstitution::D(f) => {
                    fields.insert("52D".to_string(), vec![f.to_swift_value()]);
                }
            }
        }

        // Add mandatory field 71B
        fields.insert("71B".to_string(), vec![self.field_71b.details.join("\n")]);

        // Add optional field 72
        if let Some(ref field_72) = self.field_72 {
            fields.insert("72".to_string(), vec![field_72.information.join("\n")]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "25", "32", "71B"] // Note: 32 can be 32C or 32D
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["52", "72"]
    }
}
