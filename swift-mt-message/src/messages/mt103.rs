use crate::fields::*;
use crate::parsing_utils::*;

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
    pub field_13c: Option<Vec<Field13C>>,

    #[serde(rename = "23E")]
    pub field_23e: Option<Vec<Field23E>>,

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
    pub field_57: Option<Field57AccountWithInstitution>,

    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    #[serde(rename = "71F")]
    pub field_71f: Option<Vec<Field71F>>,

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
    /// Validation rules for the message
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#
    }

    /// Parse from SWIFT MT text format
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = extract_block4(input)?;
        <Self as crate::traits::SwiftMessageBody>::parse_from_block4(&block4)
    }

    /// Convert to SWIFT MT text format
    pub fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Add mandatory fields in order
        append_field(&mut result, &self.field_20);
        append_vec_field(&mut result, &self.field_13c);
        append_field(&mut result, &self.field_23b);
        append_vec_field(&mut result, &self.field_23e);
        append_optional_field(&mut result, &self.field_26t);
        append_field(&mut result, &self.field_32a);
        append_optional_field(&mut result, &self.field_33b);
        append_optional_field(&mut result, &self.field_36);
        append_field(&mut result, &self.field_50);
        append_optional_field(&mut result, &self.field_51a);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_53);
        append_optional_field(&mut result, &self.field_54);
        append_optional_field(&mut result, &self.field_55);
        append_optional_field(&mut result, &self.field_56);
        append_optional_field(&mut result, &self.field_57);
        append_field(&mut result, &self.field_59);
        append_optional_field(&mut result, &self.field_70);
        append_field(&mut result, &self.field_71a);
        append_vec_field(&mut result, &self.field_71f);
        append_optional_field(&mut result, &self.field_71g);
        append_optional_field(&mut result, &self.field_72);
        append_optional_field(&mut result, &self.field_77b);
        append_optional_field(&mut result, &self.field_77t);

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
            if let Some(ref field_23e_vec) = self.field_23e {
                let allowed_codes = ["SDVA", "TELB", "PHOB", "INTC"];
                for field_23e in field_23e_vec {
                    if !allowed_codes.contains(&field_23e.instruction_code.as_str()) {
                        return false;
                    }
                }
            }
        } else if ["SSTD", "SPAY"].contains(&bank_op_code.as_str()) && self.field_23e.is_some() {
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

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
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
        let field_57 =
            parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;

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
        verify_parser_complete(&parser)?;

        Ok(Self {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c: if field_13c.is_empty() {
                None
            } else {
                Some(field_13c)
            },
            field_23e: if field_23e.is_empty() {
                None
            } else {
                Some(field_23e)
            },
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
            field_71f: if field_71f.is_empty() {
                None
            } else {
                Some(field_71f)
            },
            field_71g,
            field_72,
            field_77b,
            field_77t,
        })
    }

    fn to_mt_string(&self) -> String {
        // Call the existing public method implementation
        MT103::to_mt_string(self)
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
        let result = <MT103 as crate::traits::SwiftMessageBody>::parse_from_block4(mt103_text);
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
        let result = <MT103 as crate::traits::SwiftMessageBody>::parse_from_block4(mt103_text);
        assert!(result.is_ok());
        let mt103 = result.unwrap();

        // SPRI message without field 56 should be STP compliant
        assert!(mt103.is_stp_compliant());
    }
}
