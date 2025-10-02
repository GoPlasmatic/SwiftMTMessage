use crate::fields::*;

// MT103: Single Customer Credit Transfer
// Used to convey funds transfer instructions between financial institutions where the ordering
// or beneficiary customer (or both) are non-financial institutions.
// This is the most common payment message in the SWIFT network.

// MT103 doesn't use the macro due to repeated field limitations
// We'll implement it manually following the same pattern

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT103 {
    // Mandatory Fields
    #[serde(rename = "20")]
    pub field_20: Field20,

    #[serde(rename = "23B")]
    pub field_23b: Field23B,

    #[serde(rename = "32A")]
    pub field_32a: Field32A,

    #[serde(flatten)]
    pub field_50: Field50OrderingCustomerAFK,

    #[serde(flatten)]
    pub field_59: Field59,

    #[serde(rename = "71A")]
    pub field_71a: Field71A,

    // Optional Fields
    #[serde(rename = "13C")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub field_13c: Vec<Field13C>,

    #[serde(rename = "23E")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub field_23e: Vec<Field23E>,

    #[serde(rename = "26T")]
    pub field_26t: Option<Field26T>,

    #[serde(rename = "33B")]
    pub field_33b: Option<Field33B>,

    #[serde(rename = "36")]
    pub field_36: Option<Field36>,

    #[serde(rename = "51A")]
    pub field_51a: Option<Field51A>,

    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    #[serde(flatten)]
    pub field_53: Option<Field53SenderCorrespondent>,

    #[serde(flatten)]
    pub field_54: Option<Field54ReceiverCorrespondent>,

    #[serde(flatten)]
    pub field_55: Option<Field55ThirdReimbursementInstitution>,

    #[serde(flatten)]
    pub field_56: Option<Field56Intermediary>,

    #[serde(flatten)]
    pub field_57: Option<Field57>,

    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    #[serde(rename = "71F")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub field_71f: Vec<Field71F>,

    #[serde(rename = "71G")]
    pub field_71g: Option<Field71G>,

    #[serde(rename = "72")]
    pub field_72: Option<Field72>,

    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    #[serde(rename = "77T")]
    pub field_77t: Option<Field77T>,
}

// Additional methods for MT103
impl MT103 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "103");

        // Parse mandatory fields in proper order
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_23b = parser.parse_field::<Field23B>("23B")?;
        let field_32a = parser.parse_field::<Field32A>("32A")?;
        let field_50 = parser.parse_variant_field::<Field50OrderingCustomerAFK>("50")?;

        // Parse optional fields that come before field 59
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        let field_54 = parser.parse_optional_variant_field::<Field54ReceiverCorrespondent>("54")?;
        let field_55 =
            parser.parse_optional_variant_field::<Field55ThirdReimbursementInstitution>("55")?;
        let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
        let field_57 = parser.parse_optional_variant_field::<Field57>("57")?;

        // Parse mandatory field 59 (after optional routing fields)
        let field_59 = parser.parse_variant_field::<Field59>("59")?;

        // Parse optional fields that come after field 59
        let field_70 = parser.parse_optional_field::<Field70>("70")?;

        // Parse mandatory field 71A
        let field_71a = parser.parse_field::<Field71A>("71A")?;

        // Parse remaining optional fields
        let field_26t = parser.parse_optional_field::<Field26T>("26T")?;
        let field_33b = parser.parse_optional_field::<Field33B>("33B")?;
        let field_36 = parser.parse_optional_field::<Field36>("36")?;
        let field_71g = parser.parse_optional_field::<Field71G>("71G")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;
        let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
        let field_77t = parser.parse_optional_field::<Field77T>("77T")?;

        // Parse repeated fields
        parser = parser.with_duplicates(true);

        let mut field_13c = Vec::new();
        while let Ok(field) = parser.parse_field::<Field13C>("13C") {
            field_13c.push(field);
        }

        let mut field_23e = Vec::new();
        while let Ok(field) = parser.parse_field::<Field23E>("23E") {
            field_23e.push(field);
        }

        let mut field_71f = Vec::new();
        while let Ok(field) = parser.parse_field::<Field71F>("71F") {
            field_71f.push(field);
        }

        parser = parser.with_duplicates(false);

        // Verify all content is consumed
        if !parser.is_complete() {
            return Err(crate::errors::ParseError::InvalidFormat {
                message: format!(
                    "Unparsed content remaining in message: {}",
                    parser.remaining()
                ),
            });
        }

        Ok(Self {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c,
            field_23e,
            field_26t,
            field_33b,
            field_36,
            field_51a,
            field_52,
            field_53,
            field_54,
            field_55,
            field_56,
            field_57,
            field_70,
            field_71f,
            field_71g,
            field_72,
            field_77b,
            field_77t,
        })
    }

    /// Validation rules for the message
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#
    }

    /// Parse from SWIFT MT text format
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        // If input starts with block headers, extract Block 4
        let block4 = if input.starts_with("{") {
            crate::parser::SwiftParser::extract_block(input, 4)?.ok_or_else(|| {
                crate::errors::ParseError::InvalidFormat {
                    message: "Block 4 not found".to_string(),
                }
            })?
        } else {
            // Assume input is already block 4 content
            input.to_string()
        };
        Self::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        use crate::traits::SwiftField;
        let mut result = String::new();

        // Add mandatory fields in order
        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        // Add optional field 13C (repeated)
        for field_13c in &self.field_13c {
            result.push_str(&field_13c.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_23b.to_swift_string());
        result.push_str("\r\n");

        // Add optional field 23E (repeated)
        for field_23e in &self.field_23e {
            result.push_str(&field_23e.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 26T
        if let Some(ref field_26t) = self.field_26t {
            result.push_str(&field_26t.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_32a.to_swift_string());
        result.push_str("\r\n");

        // Add optional field 33B
        if let Some(ref field_33b) = self.field_33b {
            result.push_str(&field_33b.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 36
        if let Some(ref field_36) = self.field_36 {
            result.push_str(&field_36.to_swift_string());
            result.push_str("\r\n");
        }

        // Add field 50 (variant)
        result.push_str(&self.field_50.to_swift_string());
        result.push_str("\r\n");

        // Add optional field 51A
        if let Some(ref field_51a) = self.field_51a {
            result.push_str(&field_51a.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 52 (variant)
        if let Some(ref field_52) = self.field_52 {
            result.push_str(&field_52.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 53 (variant)
        if let Some(ref field_53) = self.field_53 {
            result.push_str(&field_53.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 54 (variant)
        if let Some(ref field_54) = self.field_54 {
            result.push_str(&field_54.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 55 (variant)
        if let Some(ref field_55) = self.field_55 {
            result.push_str(&field_55.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 56 (variant)
        if let Some(ref field_56) = self.field_56 {
            result.push_str(&field_56.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 57 (variant)
        if let Some(ref field_57) = self.field_57 {
            result.push_str(&field_57.to_swift_string());
            result.push_str("\r\n");
        }

        // Add field 59 (variant)
        result.push_str(&self.field_59.to_swift_string());
        result.push_str("\r\n");

        // Add optional field 70
        if let Some(ref field_70) = self.field_70 {
            result.push_str(&field_70.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_71a.to_swift_string());
        result.push_str("\r\n");

        // Add optional field 71F (repeated)
        for field_71f in &self.field_71f {
            result.push_str(&field_71f.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 71G
        if let Some(ref field_71g) = self.field_71g {
            result.push_str(&field_71g.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 72
        if let Some(ref field_72) = self.field_72 {
            result.push_str(&field_72.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 77B
        if let Some(ref field_77b) = self.field_77b {
            result.push_str(&field_77b.to_swift_string());
            result.push_str("\r\n");
        }

        // Add optional field 77T
        if let Some(ref field_77t) = self.field_77t {
            result.push_str(&field_77t.to_swift_string());
            result.push_str("\r\n");
        }

        result.push('-');
        result
    }

    /// Check if this MT103 message contains reject codes
    pub fn has_reject_codes(&self) -> bool {
        // Check field 72 for reject codes like /REJT/
        if let Some(ref field_72) = self.field_72 {
            for line in &field_72.information {
                if line.contains("/REJT/") || line.contains("/RETN/") {
                    return true;
                }
            }
        }
        false
    }

    /// Check if this MT103 message contains return codes
    pub fn has_return_codes(&self) -> bool {
        // Check field 72 for return codes
        if let Some(ref field_72) = self.field_72 {
            for line in &field_72.information {
                if line.contains("/RETN/") {
                    return true;
                }
            }
        }
        false
    }

    /// Check if this MT103 message is STP compliant
    pub fn is_stp_compliant(&self) -> bool {
        // Check if this is an STP message (SPRI, SSTD, or SPAY)
        let bank_op_code = &self.field_23b.instruction_code;
        if !["SPRI", "SSTD", "SPAY"].contains(&bank_op_code.as_str()) {
            // Not an STP message type, so it's compliant by default
            return true;
        }

        // C3: If 23B is SPRI, field 23E may contain only SDVA, TELB, PHOB, INTC
        // If 23B is SSTD or SPAY, field 23E must not be used
        if bank_op_code == "SPRI" {
            let allowed_codes = ["SDVA", "TELB", "PHOB", "INTC"];
            for field_23e in &self.field_23e {
                if !allowed_codes.contains(&field_23e.instruction_code.as_str()) {
                    return false;
                }
            }
        } else if ["SSTD", "SPAY"].contains(&bank_op_code.as_str()) && !self.field_23e.is_empty() {
            return false;
        }

        // C10: If 23B is SPRI, field 56 is not allowed
        // If 23B is SSTD or SPAY, field 56 may be present but only option A or C
        if bank_op_code == "SPRI" && self.field_56.is_some() {
            return false;
        }

        // Additional STP validation rules could be added here
        // For now, return true if basic checks pass
        true
    }
}

impl crate::traits::SwiftMessageBody for MT103 {
    fn message_type() -> &'static str {
        "103"
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
            block4.push_str(&format!(":{}:{}
", tag, value));
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
        let mut fields = std::collections::HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.to_swift_value()]);
        fields.insert("23B".to_string(), vec![self.field_23b.to_swift_value()]);
        fields.insert("32A".to_string(), vec![self.field_32a.to_swift_value()]);

        // Handle variant fields
        if let Some(variant_tag) = self.field_50.get_variant_tag() {
            fields.insert(
                format!("50{}", variant_tag),
                vec![self.field_50.to_swift_value()],
            );
        } else {
            fields.insert("50".to_string(), vec![self.field_50.to_swift_value()]);
        }

        if let Some(variant_tag) = self.field_59.get_variant_tag() {
            fields.insert(
                format!("59{}", variant_tag),
                vec![self.field_59.to_swift_value()],
            );
        } else {
            fields.insert("59".to_string(), vec![self.field_59.to_swift_value()]);
        }

        fields.insert("71A".to_string(), vec![self.field_71a.to_swift_value()]);

        // Optional fields
        if !self.field_13c.is_empty() {
            fields.insert(
                "13C".to_string(),
                self.field_13c.iter().map(|f| f.to_swift_value()).collect(),
            );
        }

        if !self.field_23e.is_empty() {
            fields.insert(
                "23E".to_string(),
                self.field_23e.iter().map(|f| f.to_swift_value()).collect(),
            );
        }

        if let Some(ref field) = self.field_26t {
            fields.insert("26T".to_string(), vec![field.to_swift_value()]);
        }

        if let Some(ref field) = self.field_33b {
            fields.insert("33B".to_string(), vec![field.to_swift_value()]);
        }

        if let Some(ref field) = self.field_36 {
            fields.insert("36".to_string(), vec![field.to_swift_value()]);
        }

        if let Some(ref field) = self.field_51a {
            fields.insert("51A".to_string(), vec![field.to_swift_value()]);
        }

        // Optional variant fields
        if let Some(ref field) = self.field_52 {
            if let Some(variant_tag) = field.get_variant_tag() {
                fields.insert(format!("52{}", variant_tag), vec![field.to_swift_value()]);
            } else {
                fields.insert("52".to_string(), vec![field.to_swift_value()]);
            }
        }

        if let Some(ref field) = self.field_53 {
            if let Some(variant_tag) = field.get_variant_tag() {
                fields.insert(format!("53{}", variant_tag), vec![field.to_swift_value()]);
            } else {
                fields.insert("53".to_string(), vec![field.to_swift_value()]);
            }
        }

        if let Some(ref field) = self.field_54 {
            if let Some(variant_tag) = field.get_variant_tag() {
                fields.insert(format!("54{}", variant_tag), vec![field.to_swift_value()]);
            } else {
                fields.insert("54".to_string(), vec![field.to_swift_value()]);
            }
        }

        if let Some(ref field) = self.field_55 {
            if let Some(variant_tag) = field.get_variant_tag() {
                fields.insert(format!("55{}", variant_tag), vec![field.to_swift_value()]);
            } else {
                fields.insert("55".to_string(), vec![field.to_swift_value()]);
            }
        }

        if let Some(ref field) = self.field_56 {
            if let Some(variant_tag) = field.get_variant_tag() {
                fields.insert(format!("56{}", variant_tag), vec![field.to_swift_value()]);
            } else {
                fields.insert("56".to_string(), vec![field.to_swift_value()]);
            }
        }

        if let Some(ref field) = self.field_57 {
            if let Some(variant_tag) = field.get_variant_tag() {
                fields.insert(format!("57{}", variant_tag), vec![field.to_swift_value()]);
            } else {
                fields.insert("57".to_string(), vec![field.to_swift_value()]);
            }
        }

        if let Some(ref field) = self.field_70 {
            fields.insert("70".to_string(), vec![field.to_swift_value()]);
        }

        if !self.field_71f.is_empty() {
            fields.insert(
                "71F".to_string(),
                self.field_71f.iter().map(|f| f.to_swift_value()).collect(),
            );
        }

        if let Some(ref field) = self.field_71g {
            fields.insert("71G".to_string(), vec![field.to_swift_value()]);
        }

        if let Some(ref field) = self.field_72 {
            fields.insert("72".to_string(), vec![field.to_swift_value()]);
        }

        if let Some(ref field) = self.field_77b {
            fields.insert("77B".to_string(), vec![field.to_swift_value()]);
        }

        if let Some(ref field) = self.field_77t {
            fields.insert("77T".to_string(), vec![field.to_swift_value()]);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "23B", "32A", "50", "59", "71A"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec![
            "13C", "23E", "26T", "33B", "36", "51A", "52", "53", "54", "55", "56", "57", "70",
            "71F", "71G", "72", "77B", "77T",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt103_parse() {
        let mt103_text = r#":20:123456789012345
:23B:CRED
:32A:241201USD1000000,00
:50K:/12345678901234567890
JOHN DOE
123 MAIN STREET
NEW YORK, NY 10001
:59:/98765432109876543210
JANE SMITH
456 OAK AVENUE
LOS ANGELES, CA 90001
:71A:OUR
-"#;
        let result = MT103::parse_from_block4(mt103_text);
        assert!(result.is_ok());
        let mt103 = result.unwrap();
        assert_eq!(mt103.field_20.reference, "123456789012345");
        assert_eq!(mt103.field_23b.instruction_code, "CRED");
        assert_eq!(mt103.field_71a.code, "OUR");
    }

    #[test]
    fn test_mt103_stp_compliance() {
        let mt103_text = r#":20:123456789012345
:23B:SPRI
:32A:241201USD1000000,00
:50K:/12345678901234567890
JOHN DOE
123 MAIN STREET
NEW YORK, NY 10001
:59:/98765432109876543210
JANE SMITH
456 OAK AVENUE
LOS ANGELES, CA 90001
:71A:OUR
-"#;
        let result = MT103::parse_from_block4(mt103_text);
        assert!(result.is_ok());
        let mt103 = result.unwrap();

        // SPRI message without field 56 should be STP compliant
        assert!(mt103.is_stp_compliant());
    }
}
