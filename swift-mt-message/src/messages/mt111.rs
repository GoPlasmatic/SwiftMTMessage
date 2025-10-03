use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT111: Request for Stop Payment of a Cheque
// Sent by the drawer bank (or its agent) to the drawee bank to request
// stop payment of a cheque.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT111 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Cheque Number
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Date of Issue
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Amount (can be 32A or 32B per SWIFT spec)
    #[serde(flatten)]
    pub field_32: Field32AB,

    // Drawer Bank (optional) - can be A, B, or D
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52DrawerBank>,

    // Payee (optional) - name and address only
    #[serde(rename = "59", skip_serializing_if = "Option::is_none")]
    pub field_59: Option<Field59NoOption>,

    // Queries (optional)
    #[serde(rename = "75", skip_serializing_if = "Option::is_none")]
    pub field_75: Option<Field75>,
}

impl MT111 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "111");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_30 = parser.parse_field::<Field30>("30")?;

        // Parse amount - can be 32A or 32B per spec
        let field_32 = parser.parse_variant_field::<Field32AB>("32")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52DrawerBank>("52")?;
        let field_59 = parser.parse_optional_field::<Field59NoOption>("59")?;
        let field_75 = parser.parse_optional_field::<Field75>("75")?;

        Ok(MT111 {
            field_20,
            field_21,
            field_30,
            field_32,
            field_52,
            field_59,
            field_75,
        })
    }

    /// Static validation rules for MT111
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F30", "description": "Field 30 must be a valid date in YYMMDD format"},
            {"id": "F32", "description": "Field 32 must contain valid currency and positive amount"},
            {"id": "F59", "description": "Field 59 must not include account number"}
        ]}"#
    }

    /// Validate the message instance according to MT111 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT111: Field 20 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 21 - same rules as Field 20
        let cheque_number = &self.field_21.reference;
        if cheque_number.starts_with('/')
            || cheque_number.ends_with('/')
            || cheque_number.contains("//")
        {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT111: Field 21 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Amount validation is handled by Field32A/B parse methods

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT111
impl crate::traits::SwiftMessageBody for MT111 {
    fn message_type() -> &'static str {
        "111"
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
        fields.insert(
            "30".to_string(),
            vec![format!(
                "{:02}{:02}{:02}",
                self.field_30.execution_date.year() % 100,
                self.field_30.execution_date.month(),
                self.field_30.execution_date.day()
            )],
        );

        // Add amount field (32A or 32B)
        match &self.field_32 {
            Field32AB::A(field_32a) => {
                fields.insert(
                    "32A".to_string(),
                    vec![format!(
                        "{:02}{:02}{:02}{}{}",
                        field_32a.value_date.year() % 100,
                        field_32a.value_date.month(),
                        field_32a.value_date.day(),
                        field_32a.currency,
                        field_32a.amount.to_string().replace('.', ",")
                    )],
                );
            }
            Field32AB::B(field_32b) => {
                fields.insert(
                    "32B".to_string(),
                    vec![format!(
                        "{}{}",
                        field_32b.currency,
                        field_32b.amount.to_string().replace('.', ",")
                    )],
                );
            }
        }

        // Add optional fields
        if let Some(ref field_52) = self.field_52 {
            match field_52 {
                Field52DrawerBank::A(f) => {
                    fields.insert("52A".to_string(), vec![f.to_swift_value()]);
                }
                Field52DrawerBank::D(f) => {
                    fields.insert("52D".to_string(), vec![f.to_swift_value()]);
                }
            }
        }

        if let Some(ref field_59) = self.field_59 {
            let mut value = String::new();
            if let Some(ref account) = field_59.account {
                value.push_str(account);
                value.push('\n');
            }
            value.push_str(&field_59.name_and_address.join("\n"));
            fields.insert("59".to_string(), vec![value]);
        }

        if let Some(ref field_75) = self.field_75 {
            fields.insert("75".to_string(), vec![field_75.information.join("\n")]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "21", "30", "32"] // Note: 32 can be 32A or 32B
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["52", "59", "75"]
    }
}

// Type alias for clarity
pub type Field52DrawerBank = Field52OrderingInstitution; // Can be A or D
