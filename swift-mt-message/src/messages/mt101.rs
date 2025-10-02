use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT101: Request for Transfer
// Used to instruct the account servicing institution to debit an account held by
// the sender and to arrange for the payment to the beneficiary(ies).
// Contains one or more transfer instructions.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT101Transaction {
    // Transaction Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Instructed Amount
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Account With Institution (Beneficiary Bank)
    #[serde(flatten)]
    pub field_57: Option<Field57>,

    // Beneficiary Customer
    #[serde(flatten)]
    pub field_59: Field59,

    // Remittance Information
    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    // Details of Charges
    #[serde(rename = "71A")]
    pub field_71a: Option<Field71A>,

    // Sender to Receiver Information
    #[serde(rename = "72")]
    pub field_72: Option<Field72>,

    // Instruction Code
    #[serde(rename = "23E")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub field_23e: Vec<Field23E>,

    // Exchange Rate
    #[serde(rename = "36")]
    pub field_36: Option<Field36>,

    // Regulatory Reporting
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT101 {
    // Message Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Message Index/Total
    #[serde(rename = "28D")]
    pub field_28d: Field28D,

    // Requested Execution Date
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Ordering Customer - REQUIRED field in SWIFT spec, must come before transactions
    #[serde(flatten)]
    pub field_50: Field50OrderingCustomerAFK,

    // Ordering Institution
    #[serde(flatten)]
    pub field_52: Option<Field52OrderingInstitution>,

    // Sender's Correspondent
    #[serde(flatten)]
    pub field_53: Option<Field53SenderCorrespondent>,

    // Intermediary Institution
    #[serde(flatten)]
    pub field_56: Option<Field56Intermediary>,

    // Transaction Details (repeating sequence)
    #[serde(rename = "#")]
    pub transactions: Vec<MT101Transaction>,

    // Instruction for Next Agent
    #[serde(rename = "72")]
    pub field_72: Option<Field72>,

    // Regulatory Reporting
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,
}

impl MT101 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "101");

        // Parse mandatory fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_28d = parser.parse_field::<Field28D>("28D")?;
        let field_30 = parser.parse_field::<Field30>("30")?;

        // Parse ordering customer (can be A, F, or K variant)
        let field_50 = parser.parse_variant_field::<Field50OrderingCustomerAFK>("50")?;

        // Parse optional fields
        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;

        // Parse transactions - this is a repeating sequence
        let mut transactions = Vec::new();

        // Enable duplicates for repeating fields
        parser = parser.with_duplicates(true);

        // Parse each transaction - they start with field 21
        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;

            // Parse optional transaction-level fields in correct order
            // Field 23E can appear multiple times
            let mut field_23e = Vec::new();
            while let Ok(field) = parser.parse_field::<Field23E>("23E") {
                field_23e.push(field);
            }

            // Field 36 is optional
            let field_36 = parser.parse_optional_field::<Field36>("36")?;

            // Field 32B is mandatory
            let field_32b = parser.parse_field::<Field32B>("32B")?;

            // Parse remaining optional transaction fields
            let field_57 = parser.parse_optional_variant_field::<Field57>("57")?;
            let field_59 = parser.parse_variant_field::<Field59>("59")?;
            let field_70 = parser.parse_optional_field::<Field70>("70")?;
            let field_71a = parser.parse_optional_field::<Field71A>("71A")?;

            // Transaction-level field 72
            let trans_field_72 = if parser.detect_field("72") && !parser.detect_field("21") {
                parser.parse_optional_field::<Field72>("72")?
            } else {
                None
            };

            let field_77b = parser.parse_optional_field::<Field77B>("77B")?;

            transactions.push(MT101Transaction {
                field_21,
                field_32b,
                field_57,
                field_59,
                field_70,
                field_71a,
                field_72: trans_field_72,
                field_23e,
                field_36,
                field_77b,
            });
        }

        // Parse message-level optional fields
        let field_72 = parser.parse_optional_field::<Field72>("72")?;
        let field_77b = parser.parse_optional_field::<Field77B>("77B")?;

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
            field_28d,
            field_30,
            field_50,
            field_52,
            field_53,
            field_56,
            transactions,
            field_72,
            field_77b,
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

        // Add mandatory fields
        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");
        result.push_str(&self.field_28d.to_swift_string());
        result.push_str("\r\n");
        result.push_str(&self.field_30.to_swift_string());
        result.push_str("\r\n");
        result.push_str(&self.field_50.to_swift_string());
        result.push_str("\r\n");

        // Add optional header fields
        if let Some(ref field_52) = self.field_52 {
            result.push_str(&field_52.to_swift_string());
            result.push_str("\r\n");
        }
        if let Some(ref field_53) = self.field_53 {
            result.push_str(&field_53.to_swift_string());
            result.push_str("\r\n");
        }
        if let Some(ref field_56) = self.field_56 {
            result.push_str(&field_56.to_swift_string());
            result.push_str("\r\n");
        }

        // Add transactions
        for transaction in &self.transactions {
            result.push_str(&transaction.field_21.to_swift_string());
            result.push_str("\r\n");
            result.push_str(&transaction.field_32b.to_swift_string());
            result.push_str("\r\n");

            for field_23e in &transaction.field_23e {
                result.push_str(&field_23e.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_36) = transaction.field_36 {
                result.push_str(&field_36.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_57) = transaction.field_57 {
                result.push_str(&field_57.to_swift_string());
                result.push_str("\r\n");
            }

            result.push_str(&transaction.field_59.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field_70) = transaction.field_70 {
                result.push_str(&field_70.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_71a) = transaction.field_71a {
                result.push_str(&field_71a.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_72) = transaction.field_72 {
                result.push_str(&field_72.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_77b) = transaction.field_77b {
                result.push_str(&field_77b.to_swift_string());
                result.push_str("\r\n");
            }
        }

        // Add message-level optional fields
        if let Some(ref field_72) = self.field_72 {
            result.push_str(&field_72.to_swift_string());
            result.push_str("\r\n");
        }
        if let Some(ref field_77b) = self.field_77b {
            result.push_str(&field_77b.to_swift_string());
            result.push_str("\r\n");
        }

        result.push('-');
        result
    }
}

impl crate::traits::SwiftMessageBody for MT101 {
    fn message_type() -> &'static str {
        "101"
    }

    fn from_fields(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
    ) -> crate::SwiftResult<Self> {
        // Flatten all fields with their positions into a single vec
        let mut all_fields: Vec<(&String, &String, usize)> = Vec::new();
        for (tag, values) in fields.iter() {
            for (value, pos) in values {
                all_fields.push((tag, value, *pos));
            }
        }

        // Sort by position to maintain the original sequence order
        all_fields.sort_by_key(|(_, _, pos)| *pos);

        // Build block4 string in the correct order
        let mut block4_parts = Vec::new();
        for (tag, value, _) in all_fields {
            block4_parts.push(format!(":{}:{}", tag, value));
        }

        let block4 = block4_parts.join("\n") + "\n-";
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

    fn to_ordered_fields(&self) -> Vec<(String, String)> {
        use crate::traits::SwiftField;
        let mut ordered_fields = Vec::new();

        // Add header fields in correct order
        ordered_fields.push(("20".to_string(), self.field_20.to_swift_value()));
        ordered_fields.push(("28D".to_string(), self.field_28d.to_swift_value()));
        ordered_fields.push(("30".to_string(), self.field_30.to_swift_value()));

        // Add field 50 with variant
        if let Some(variant_tag) = self.field_50.get_variant_tag() {
            ordered_fields.push((format!("50{}", variant_tag), self.field_50.to_swift_value()));
        } else {
            ordered_fields.push(("50".to_string(), self.field_50.to_swift_value()));
        }

        // Add optional fields
        if let Some(ref field_52) = self.field_52 {
            if let Some(variant_tag) = field_52.get_variant_tag() {
                ordered_fields.push((format!("52{}", variant_tag), field_52.to_swift_value()));
            } else {
                ordered_fields.push(("52".to_string(), field_52.to_swift_value()));
            }
        }

        if let Some(ref field_53) = self.field_53 {
            if let Some(variant_tag) = field_53.get_variant_tag() {
                ordered_fields.push((format!("53{}", variant_tag), field_53.to_swift_value()));
            } else {
                ordered_fields.push(("53".to_string(), field_53.to_swift_value()));
            }
        }

        if let Some(ref field_56) = self.field_56 {
            if let Some(variant_tag) = field_56.get_variant_tag() {
                ordered_fields.push((format!("56{}", variant_tag), field_56.to_swift_value()));
            } else {
                ordered_fields.push(("56".to_string(), field_56.to_swift_value()));
            }
        }

        // Add transaction fields in order
        for transaction in &self.transactions {
            ordered_fields.push(("21".to_string(), transaction.field_21.to_swift_value()));

            for field_23e in &transaction.field_23e {
                ordered_fields.push(("23E".to_string(), field_23e.to_swift_value()));
            }

            if let Some(ref field_36) = transaction.field_36 {
                ordered_fields.push(("36".to_string(), field_36.to_swift_value()));
            }

            ordered_fields.push(("32B".to_string(), transaction.field_32b.to_swift_value()));

            if let Some(ref field_57) = transaction.field_57 {
                if let Some(variant_tag) = field_57.get_variant_tag() {
                    ordered_fields.push((format!("57{}", variant_tag), field_57.to_swift_value()));
                } else {
                    ordered_fields.push(("57".to_string(), field_57.to_swift_value()));
                }
            }

            // Add field 59 with variant
            if let Some(variant_tag) = transaction.field_59.get_variant_tag() {
                ordered_fields.push((
                    format!("59{}", variant_tag),
                    transaction.field_59.to_swift_value(),
                ));
            } else {
                ordered_fields.push(("59".to_string(), transaction.field_59.to_swift_value()));
            }

            if let Some(ref field_70) = transaction.field_70 {
                ordered_fields.push(("70".to_string(), field_70.to_swift_value()));
            }

            if let Some(ref field_71a) = transaction.field_71a {
                ordered_fields.push(("71A".to_string(), field_71a.to_swift_value()));
            }

            if let Some(ref field_72) = transaction.field_72 {
                ordered_fields.push(("72".to_string(), field_72.to_swift_value()));
            }

            if let Some(ref field_77b) = transaction.field_77b {
                ordered_fields.push(("77B".to_string(), field_77b.to_swift_value()));
            }
        }

        // Add message-level optional fields
        if let Some(ref field_72) = self.field_72 {
            ordered_fields.push(("72".to_string(), field_72.to_swift_value()));
        }

        if let Some(ref field_77b) = self.field_77b {
            ordered_fields.push(("77B".to_string(), field_77b.to_swift_value()));
        }

        ordered_fields
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        use crate::traits::SwiftField;
        let mut fields = std::collections::HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.to_swift_value()]);
        fields.insert("28D".to_string(), vec![self.field_28d.to_swift_value()]);
        fields.insert("30".to_string(), vec![self.field_30.to_swift_value()]);

        // Add field 50 with variant
        if let Some(variant_tag) = self.field_50.get_variant_tag() {
            fields.insert(
                format!("50{}", variant_tag),
                vec![self.field_50.to_swift_value()],
            );
        } else {
            fields.insert("50".to_string(), vec![self.field_50.to_swift_value()]);
        }

        // Add optional fields
        if let Some(ref field_52) = self.field_52 {
            if let Some(variant_tag) = field_52.get_variant_tag() {
                fields.insert(
                    format!("52{}", variant_tag),
                    vec![field_52.to_swift_value()],
                );
            } else {
                fields.insert("52".to_string(), vec![field_52.to_swift_value()]);
            }
        }

        if let Some(ref field_53) = self.field_53 {
            if let Some(variant_tag) = field_53.get_variant_tag() {
                fields.insert(
                    format!("53{}", variant_tag),
                    vec![field_53.to_swift_value()],
                );
            } else {
                fields.insert("53".to_string(), vec![field_53.to_swift_value()]);
            }
        }

        if let Some(ref field_56) = self.field_56 {
            if let Some(variant_tag) = field_56.get_variant_tag() {
                fields.insert(
                    format!("56{}", variant_tag),
                    vec![field_56.to_swift_value()],
                );
            } else {
                fields.insert("56".to_string(), vec![field_56.to_swift_value()]);
            }
        }

        // Add transaction fields
        for transaction in &self.transactions {
            // Transaction fields can repeat, so we need to handle them carefully
            fields
                .entry("21".to_string())
                .or_default()
                .push(transaction.field_21.to_swift_value());
            fields
                .entry("32B".to_string())
                .or_default()
                .push(transaction.field_32b.to_swift_value());

            for field_23e in &transaction.field_23e {
                fields
                    .entry("23E".to_string())
                    .or_default()
                    .push(field_23e.to_swift_value());
            }

            if let Some(ref field_36) = transaction.field_36 {
                fields
                    .entry("36".to_string())
                    .or_default()
                    .push(field_36.to_swift_value());
            }

            if let Some(ref field_57) = transaction.field_57 {
                if let Some(variant_tag) = field_57.get_variant_tag() {
                    fields
                        .entry(format!("57{}", variant_tag))
                        .or_default()
                        .push(field_57.to_swift_value());
                } else {
                    fields
                        .entry("57".to_string())
                        .or_default()
                        .push(field_57.to_swift_value());
                }
            }

            // Add field 59 with variant
            if let Some(variant_tag) = transaction.field_59.get_variant_tag() {
                fields
                    .entry(format!("59{}", variant_tag))
                    .or_default()
                    .push(transaction.field_59.to_swift_value());
            } else {
                fields
                    .entry("59".to_string())
                    .or_default()
                    .push(transaction.field_59.to_swift_value());
            }

            if let Some(ref field_70) = transaction.field_70 {
                fields
                    .entry("70".to_string())
                    .or_default()
                    .push(field_70.to_swift_value());
            }

            if let Some(ref field_71a) = transaction.field_71a {
                fields
                    .entry("71A".to_string())
                    .or_default()
                    .push(field_71a.to_swift_value());
            }

            if let Some(ref field_72) = transaction.field_72 {
                fields
                    .entry("72".to_string())
                    .or_default()
                    .push(field_72.to_swift_value());
            }

            if let Some(ref field_77b) = transaction.field_77b {
                fields
                    .entry("77B".to_string())
                    .or_default()
                    .push(field_77b.to_swift_value());
            }
        }

        // Add message-level optional fields
        if let Some(ref field_72) = self.field_72 {
            fields
                .entry("72".to_string())
                .or_default()
                .push(field_72.to_swift_value());
        }

        if let Some(ref field_77b) = self.field_77b {
            fields
                .entry("77B".to_string())
                .or_default()
                .push(field_77b.to_swift_value());
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "28D", "30", "50", "21", "32B", "59"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec![
            "23E", "36", "52", "53", "56", "57", "70", "71A", "72", "77B",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt101_parse() {
        let mt101_text = r#":20:PAYREF001
:28D:1/1
:30:241220
:50F:/1234567890
ORDERING CUSTOMER NAME
ADDRESS LINE 1
ADDRESS LINE 2
:21:TRANS001
:32B:USD10000,00
:59:/DE89370400440532013000
BENEFICIARY NAME
BENEFICIARY ADDRESS
:70:PAYMENT FOR SERVICES
:71A:SHA
-"#;
        let result = MT101::parse_from_block4(mt101_text);
        assert!(result.is_ok());
        let mt101 = result.unwrap();
        assert_eq!(mt101.field_20.reference, "PAYREF001");
        assert_eq!(mt101.transactions.len(), 1);
        assert_eq!(mt101.transactions[0].field_21.reference, "TRANS001");
    }
}
