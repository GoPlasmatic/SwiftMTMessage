use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT104: Direct Debit and Request for Debit Transfer Message
// Used for direct debit instructions where the creditor instructs its bank
// to collect funds from one or more debtors.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT104Transaction {
    // Transaction Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Instruction Code (optional)
    #[serde(rename = "23E", skip_serializing_if = "Option::is_none")]
    pub field_23e: Option<Field23E>,

    // Transaction Amount
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Creditor (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50: Option<Field50Creditor>,

    // Instructing Party (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Account With Institution (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57>,

    // Debtor
    #[serde(flatten)]
    pub field_59: Field59,

    // Remittance Information
    #[serde(rename = "70", skip_serializing_if = "Option::is_none")]
    pub field_70: Option<Field70>,

    // Details of Charges (optional, mutually exclusive with message-level 71A per Rule C3)
    #[serde(rename = "71A", skip_serializing_if = "Option::is_none")]
    pub field_71a: Option<Field71A>,

    // Sender to Receiver Information
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    // Regulatory Reporting (optional, mutually exclusive with message-level 77B per Rule C3)
    #[serde(rename = "77B", skip_serializing_if = "Option::is_none")]
    pub field_77b: Option<Field77B>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT104 {
    // Message Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Instruction Code (optional)
    #[serde(rename = "23E", skip_serializing_if = "Option::is_none")]
    pub field_23e: Option<Field23E>,

    // Requested Execution Date
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Total Amount (summary)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Creditor (message level)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50: Option<Field50Creditor>,

    // Ordering Institution
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Sender's Correspondent
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53: Option<Field53SenderCorrespondent>,

    // Details of Charges (optional, mutually exclusive with transaction-level 71A per Rule C3)
    #[serde(rename = "71A", skip_serializing_if = "Option::is_none")]
    pub field_71a: Option<Field71A>,

    // Sender to Receiver Information
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    // Transaction Details (repeating sequence)
    #[serde(rename = "#")]
    pub transactions: Vec<MT104Transaction>,

    // Sum of Amounts (Sequence C - optional, used when settlement amount differs from sum of transaction amounts)
    #[serde(rename = "19", skip_serializing_if = "Option::is_none")]
    pub field_19: Option<Field19>,

    // Regulatory Reporting (Sequence A - optional at message level)
    #[serde(rename = "77B", skip_serializing_if = "Option::is_none")]
    pub field_77b: Option<Field77B>,
}

impl MT104 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "104");

        // Parse mandatory fields
        eprintln!("DEBUG MT104: Starting message-level parsing");
        let field_20 = parser.parse_field::<Field20>("20")?;
        eprintln!("DEBUG MT104: After field 20, position = {}", parser.position());

        let field_23e = parser.parse_optional_field::<Field23E>("23E")?;
        eprintln!("DEBUG MT104: After field 23E, position = {}", parser.position());

        let field_30 = parser.parse_field::<Field30>("30")?;
        eprintln!("DEBUG MT104: After field 30, position = {}, remaining: {:?}", parser.position(), parser.remaining().chars().take(100).collect::<String>());

        // Parse optional fields before transactions
        let field_50 = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
        eprintln!("DEBUG MT104: After msg field 50, position = {}, found: {}", parser.position(), field_50.is_some());

        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        eprintln!("DEBUG MT104: After msg field 52, position = {}, found: {}", parser.position(), field_52.is_some());

        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;
        eprintln!("DEBUG MT104: After msg field 53, position = {}, found: {}", parser.position(), field_53.is_some());

        let field_71a = parser.parse_optional_field::<Field71A>("71A")?;
        eprintln!("DEBUG MT104: After msg field 71A, position = {}, found: {}", parser.position(), field_71a.is_some());

        let field_72 = parser.parse_optional_field::<Field72>("72")?;
        eprintln!("DEBUG MT104: After msg field 72, position = {}, found: {}", parser.position(), field_72.is_some());

        // Parse transactions - enable duplicates for repeating fields
        let mut transactions = Vec::new();
        parser = parser.with_duplicates(true);

        eprintln!("DEBUG MT104: Starting transaction parsing loop at position {}", parser.position());

        // Parse each transaction - they start with field 21
        let mut txn_count = 0;
        while parser.detect_field("21") {
            txn_count += 1;
            eprintln!("DEBUG MT104: === Transaction {} detected at position {} ===", txn_count, parser.position());
            eprintln!("DEBUG MT104: Remaining content (first 150 chars): {:?}", parser.remaining().chars().take(150).collect::<String>());

            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            eprintln!("DEBUG MT104: After field 21, position = {}", parser.position());

            let field_23e = parser.parse_optional_field::<Field23E>("23E")?;
            eprintln!("DEBUG MT104: After field 23E, position = {}", parser.position());

            let field_32b = parser.parse_field::<Field32B>("32B")?;
            eprintln!("DEBUG MT104: After field 32B, position = {}", parser.position());

            // Parse optional transaction fields
            let field_50 = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
            eprintln!("DEBUG MT104: After field 50, position = {}", parser.position());

            let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
            eprintln!("DEBUG MT104: After field 52, position = {}", parser.position());

            let field_57 = parser.parse_optional_variant_field::<Field57>("57")?;
            eprintln!("DEBUG MT104: After field 57, position = {}", parser.position());

            let field_59 = parser.parse_variant_field::<Field59>("59")?;
            eprintln!("DEBUG MT104: After field 59, position = {}", parser.position());

            let field_70 = parser.parse_optional_field::<Field70>("70")?;
            eprintln!("DEBUG MT104: After field 70, position = {}", parser.position());

            let field_71a = parser.parse_optional_field::<Field71A>("71A")?;
            eprintln!("DEBUG MT104: After field 71A, position = {}", parser.position());

            let field_72 = parser.parse_optional_field::<Field72>("72")?;
            eprintln!("DEBUG MT104: After field 72, position = {}", parser.position());

            let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
            eprintln!("DEBUG MT104: After field 77B, position = {}", parser.position());
            eprintln!("DEBUG MT104: Transaction {} complete. Remaining (first 80 chars): {:?}", txn_count, parser.remaining().chars().take(80).collect::<String>());

            transactions.push(MT104Transaction {
                field_21,
                field_23e,
                field_32b,
                field_50,
                field_52,
                field_57,
                field_59,
                field_70,
                field_71a,
                field_72,
                field_77b,
            });
        }

        eprintln!("DEBUG MT104: Transaction loop exited. Parsed {} transactions. Current position = {}", txn_count, parser.position());
        eprintln!("DEBUG MT104: Checking for next field 21: {}", parser.detect_field("21"));
        eprintln!("DEBUG MT104: Remaining content (first 200 chars): {:?}", parser.remaining().chars().take(200).collect::<String>());

        // Parse the summary amount field at the end (Sequence C)
        let field_32b = parser.parse_field::<Field32B>("32B")?;

        // Parse optional Sequence C fields
        let field_19 = parser.parse_optional_field::<Field19>("19")?;
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
            field_23e,
            field_30,
            field_32b,
            field_50,
            field_52,
            field_53,
            field_71a,
            field_72,
            transactions,
            field_19,
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

        if let Some(ref field_23e) = self.field_23e {
            result.push_str(&field_23e.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_30.to_swift_string());
        result.push_str("\r\n");

        // Add optional header fields
        if let Some(ref field_50) = self.field_50 {
            result.push_str(&field_50.to_swift_string());
            result.push_str("\r\n");
        }
        if let Some(ref field_52) = self.field_52 {
            result.push_str(&field_52.to_swift_string());
            result.push_str("\r\n");
        }
        if let Some(ref field_53) = self.field_53 {
            result.push_str(&field_53.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_71a) = self.field_71a {
            result.push_str(&field_71a.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_72) = self.field_72 {
            result.push_str(&field_72.to_swift_string());
            result.push_str("\r\n");
        }

        // Add transactions
        for transaction in &self.transactions {
            result.push_str(&transaction.field_21.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field_23e) = transaction.field_23e {
                result.push_str(&field_23e.to_swift_string());
                result.push_str("\r\n");
            }

            result.push_str(&transaction.field_32b.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field_50) = transaction.field_50 {
                result.push_str(&field_50.to_swift_string());
                result.push_str("\r\n");
            }
            if let Some(ref field_52) = transaction.field_52 {
                result.push_str(&field_52.to_swift_string());
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
            if let Some(ref field_72) = transaction.field_72 {
                result.push_str(&field_72.to_swift_string());
                result.push_str("\r\n");
            }
        }

        // Add summary amount
        result.push_str(&self.field_32b.to_swift_string());
        result.push_str("\r\n");

        result.push('-');
        result
    }
}

impl crate::traits::SwiftMessageBody for MT104 {
    fn message_type() -> &'static str {
        "104"
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

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        use crate::traits::SwiftField;
        let mut fields = std::collections::HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.to_swift_value()]);

        if let Some(ref field_23e) = self.field_23e {
            fields.insert("23E".to_string(), vec![field_23e.to_swift_value()]);
        }

        fields.insert("30".to_string(), vec![self.field_30.to_swift_value()]);

        if let Some(ref field_50) = self.field_50 {
            if let Some(variant_tag) = field_50.get_variant_tag() {
                fields.insert(format!("50{}", variant_tag), vec![field_50.to_swift_value()]);
            } else {
                fields.insert("50".to_string(), vec![field_50.to_swift_value()]);
            }
        }

        if let Some(ref field_52) = self.field_52 {
            if let Some(variant_tag) = field_52.get_variant_tag() {
                fields.insert(format!("52{}", variant_tag), vec![field_52.to_swift_value()]);
            } else {
                fields.insert("52".to_string(), vec![field_52.to_swift_value()]);
            }
        }

        if let Some(ref field_53) = self.field_53 {
            if let Some(variant_tag) = field_53.get_variant_tag() {
                fields.insert(format!("53{}", variant_tag), vec![field_53.to_swift_value()]);
            } else {
                fields.insert("53".to_string(), vec![field_53.to_swift_value()]);
            }
        }

        if let Some(ref field_71a) = self.field_71a {
            fields.insert("71A".to_string(), vec![field_71a.to_swift_value()]);
        }

        if let Some(ref field_72) = self.field_72 {
            fields.insert("72".to_string(), vec![field_72.to_swift_value()]);
        }

        // Add transaction fields
        for transaction in &self.transactions {
            fields.entry("21".to_string()).or_default().push(transaction.field_21.to_swift_value());

            if let Some(ref field_23e) = transaction.field_23e {
                fields.entry("23E".to_string()).or_default().push(field_23e.to_swift_value());
            }

            fields.entry("32B".to_string()).or_default().push(transaction.field_32b.to_swift_value());

            if let Some(ref field_50) = transaction.field_50 {
                if let Some(variant_tag) = field_50.get_variant_tag() {
                    fields.entry(format!("50{}", variant_tag)).or_default().push(field_50.to_swift_value());
                } else {
                    fields.entry("50".to_string()).or_default().push(field_50.to_swift_value());
                }
            }

            if let Some(ref field_52) = transaction.field_52 {
                if let Some(variant_tag) = field_52.get_variant_tag() {
                    fields.entry(format!("52{}", variant_tag)).or_default().push(field_52.to_swift_value());
                } else {
                    fields.entry("52".to_string()).or_default().push(field_52.to_swift_value());
                }
            }

            if let Some(ref field_57) = transaction.field_57 {
                if let Some(variant_tag) = field_57.get_variant_tag() {
                    fields.entry(format!("57{}", variant_tag)).or_default().push(field_57.to_swift_value());
                } else {
                    fields.entry("57".to_string()).or_default().push(field_57.to_swift_value());
                }
            }

            if let Some(variant_tag) = transaction.field_59.get_variant_tag() {
                fields.entry(format!("59{}", variant_tag)).or_default().push(transaction.field_59.to_swift_value());
            } else {
                fields.entry("59".to_string()).or_default().push(transaction.field_59.to_swift_value());
            }

            if let Some(ref field_70) = transaction.field_70 {
                fields.entry("70".to_string()).or_default().push(field_70.to_swift_value());
            }

            if let Some(ref field_72) = transaction.field_72 {
                fields.entry("72".to_string()).or_default().push(field_72.to_swift_value());
            }
        }

        // Add summary amount (Sequence C)
        fields.entry("32B".to_string()).or_default().push(self.field_32b.to_swift_value());

        // Add optional Sequence C fields
        if let Some(ref field_19) = self.field_19 {
            fields.insert("19".to_string(), vec![field_19.to_swift_value()]);
        }

        if let Some(ref field_77b) = self.field_77b {
            fields.insert("77B".to_string(), vec![field_77b.to_swift_value()]);
        }

        fields
    }

    fn to_ordered_fields(&self) -> Vec<(String, String)> {
        use crate::traits::SwiftField;
        let mut ordered_fields = Vec::new();

        // Add header fields in correct order
        ordered_fields.push(("20".to_string(), self.field_20.to_swift_value()));

        if let Some(ref field_23e) = self.field_23e {
            ordered_fields.push(("23E".to_string(), field_23e.to_swift_value()));
        }

        ordered_fields.push(("30".to_string(), self.field_30.to_swift_value()));

        // Add optional message-level fields
        if let Some(ref field_50) = self.field_50 {
            if let Some(variant_tag) = field_50.get_variant_tag() {
                ordered_fields.push((format!("50{}", variant_tag), field_50.to_swift_value()));
            } else {
                ordered_fields.push(("50".to_string(), field_50.to_swift_value()));
            }
        }

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

        if let Some(ref field_71a) = self.field_71a {
            ordered_fields.push(("71A".to_string(), field_71a.to_swift_value()));
        }

        if let Some(ref field_72) = self.field_72 {
            ordered_fields.push(("72".to_string(), field_72.to_swift_value()));
        }

        // Add transaction fields in order
        for transaction in &self.transactions {
            ordered_fields.push(("21".to_string(), transaction.field_21.to_swift_value()));

            if let Some(ref field_23e) = transaction.field_23e {
                ordered_fields.push(("23E".to_string(), field_23e.to_swift_value()));
            }

            ordered_fields.push(("32B".to_string(), transaction.field_32b.to_swift_value()));

            if let Some(ref field_50) = transaction.field_50 {
                if let Some(variant_tag) = field_50.get_variant_tag() {
                    ordered_fields.push((format!("50{}", variant_tag), field_50.to_swift_value()));
                } else {
                    ordered_fields.push(("50".to_string(), field_50.to_swift_value()));
                }
            }

            if let Some(ref field_52) = transaction.field_52 {
                if let Some(variant_tag) = field_52.get_variant_tag() {
                    ordered_fields.push((format!("52{}", variant_tag), field_52.to_swift_value()));
                } else {
                    ordered_fields.push(("52".to_string(), field_52.to_swift_value()));
                }
            }

            if let Some(ref field_57) = transaction.field_57 {
                if let Some(variant_tag) = field_57.get_variant_tag() {
                    ordered_fields.push((format!("57{}", variant_tag), field_57.to_swift_value()));
                } else {
                    ordered_fields.push(("57".to_string(), field_57.to_swift_value()));
                }
            }

            // Add field 59 with variant
            if let Some(variant_tag) = transaction.field_59.get_variant_tag() {
                ordered_fields.push((format!("59{}", variant_tag), transaction.field_59.to_swift_value()));
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

        // Add summary amount field at the end (Sequence C)
        ordered_fields.push(("32B".to_string(), self.field_32b.to_swift_value()));

        // Add optional Sequence C fields
        if let Some(ref field_19) = self.field_19 {
            ordered_fields.push(("19".to_string(), field_19.to_swift_value()));
        }

        if let Some(ref field_77b) = self.field_77b {
            ordered_fields.push(("77B".to_string(), field_77b.to_swift_value()));
        }

        ordered_fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "30", "32B", "71A", "21", "59"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["23E", "50", "52", "53", "57", "70", "72"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt104_parse() {
        let mt104_text = r#":20:DDREF001
:30:241225
:71A:SHA
:21:TRANS001
:32B:USD10000,00
:59:/DE89370400440532013000
DEBTOR NAME
DEBTOR ADDRESS
:32B:USD10000,00
-"#;
        let result = MT104::parse_from_block4(mt104_text);
        assert!(result.is_ok());
        let mt104 = result.unwrap();
        assert_eq!(mt104.field_20.reference, "DDREF001");
        assert_eq!(mt104.transactions.len(), 1);
        assert_eq!(mt104.transactions[0].field_21.reference, "TRANS001");
    }
}