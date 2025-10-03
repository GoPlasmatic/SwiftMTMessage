use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT112: Status of a Request for Stop Payment of a Cheque
// Used by the drawee bank to notify the drawer bank about actions taken
// in response to an MT111 stop payment request.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT112 {
    // Transaction Reference Number
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

    // Answers (mandatory) - status information
    #[serde(rename = "76")]
    pub field_76: Field76,
}

impl MT112 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "112");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21 = parser.parse_field::<Field21NoOption>("21")?;
        let field_30 = parser.parse_field::<Field30>("30")?;

        // Parse amount - can be 32A or 32B per spec
        let field_32 = parser.parse_variant_field::<Field32AB>("32")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52DrawerBank>("52")?;
        let field_59 = parser.parse_optional_field::<Field59NoOption>("59")?;

        // Parse mandatory field 76
        let field_76 = parser.parse_field::<Field76>("76")?;

        Ok(MT112 {
            field_20,
            field_21,
            field_30,
            field_32,
            field_52,
            field_59,
            field_76,
        })
    }

    /// Static validation rules for MT112
    pub fn validate() -> &'static str {
        r#"{"rules": [
            {"id": "F20", "description": "Field 20 must not start or end with '/', and must not contain '//'"},
            {"id": "F21", "description": "Field 21 must not start or end with '/', and must not contain '//'"},
            {"id": "F30", "description": "Field 30 must be a valid date in YYMMDD format"},
            {"id": "F32", "description": "Field 32 must contain valid currency and positive amount"},
            {"id": "F59", "description": "Field 59 must not include account number"},
            {"id": "F76", "description": "Field 76 is mandatory and must contain status information"}
        ]}"#
    }

    /// Validate the message instance according to MT112 rules
    pub fn validate_instance(&self) -> Result<(), crate::errors::ParseError> {
        // Validate Field 20 - must not start/end with '/' or contain '//'
        let reference = &self.field_20.reference;
        if reference.starts_with('/') || reference.ends_with('/') || reference.contains("//") {
            return Err(crate::errors::ParseError::InvalidFormat {
                message:
                    "MT112: Field 20 must not start or end with '/', and must not contain '//'"
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
                    "MT112: Field 21 must not start or end with '/', and must not contain '//'"
                        .to_string(),
            });
        }

        // Validate Field 76 is not empty
        if self.field_76.information.is_empty() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: "MT112: Field 76 must contain at least one line of status information"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// Implement the SwiftMessageBody trait for MT112
impl crate::traits::SwiftMessageBody for MT112 {
    fn message_type() -> &'static str {
        "112"
    }

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.field_21.to_swift_string());
        result.push_str("\r\n");

        result.push_str(&self.field_30.to_swift_string());
        result.push_str("\r\n");

        match &self.field_32 {
            Field32AB::A(f) => result.push_str(&f.to_swift_string()),
            Field32AB::B(f) => result.push_str(&f.to_swift_string()),
        }
        result.push_str("\r\n");

        if let Some(ref field) = self.field_52 {
            match field {
                Field52DrawerBank::A(f) => result.push_str(&f.to_swift_string()),
                Field52DrawerBank::D(f) => result.push_str(&f.to_swift_string()),
            }
            result.push_str("\r\n");
        }

        if let Some(ref field) = self.field_59 {
            result.push_str(&field.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_76.to_swift_string());
        result.push_str("\r\n");

        // Remove trailing \r\n
        if result.ends_with("\r\n") {
            result.truncate(result.len() - 2);
        }

        result
    }
}

// Type alias for clarity
pub type Field52DrawerBank = Field52OrderingInstitution; // Can be A or D
