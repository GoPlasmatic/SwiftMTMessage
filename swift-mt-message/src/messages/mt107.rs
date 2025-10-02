use crate::fields::*;
use serde::{Deserialize, Serialize};

// MT107: General Direct Debit Message
// Used for general direct debit instructions, similar to MT104 but with additional flexibility

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT107Transaction {
    // Transaction Reference
    #[serde(rename = "21")]
    pub field_21: Field21NoOption,

    // Instruction Code (optional)
    #[serde(rename = "23E", skip_serializing_if = "Option::is_none")]
    pub field_23e: Option<Field23E>,

    // Mandate Reference (optional)
    #[serde(rename = "21C", skip_serializing_if = "Option::is_none")]
    pub field_21c: Option<Field21C>,

    // Direct Debit Reference (optional)
    #[serde(rename = "21D", skip_serializing_if = "Option::is_none")]
    pub field_21d: Option<Field21D>,

    // Registration Reference (optional)
    #[serde(rename = "21E", skip_serializing_if = "Option::is_none")]
    pub field_21e: Option<Field21E>,

    // Transaction Amount
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Instructing Party (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50_instructing: Option<Field50InstructingParty>,

    // Creditor (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50_creditor: Option<Field50Creditor>,

    // Creditor's Bank (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Debtor's Bank (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57>,

    // Debtor (mandatory)
    #[serde(flatten)]
    pub field_59: Field59,

    // Remittance Information (optional)
    #[serde(rename = "70", skip_serializing_if = "Option::is_none")]
    pub field_70: Option<Field70>,

    // Transaction Type Code (optional)
    #[serde(rename = "26T", skip_serializing_if = "Option::is_none")]
    pub field_26t: Option<Field26T>,

    // Regulatory Reporting (optional)
    #[serde(rename = "77B", skip_serializing_if = "Option::is_none")]
    pub field_77b: Option<Field77B>,

    // Original Ordered Amount (optional)
    #[serde(rename = "33B", skip_serializing_if = "Option::is_none")]
    pub field_33b: Option<Field33B>,

    // Details of Charges (optional)
    #[serde(rename = "71A", skip_serializing_if = "Option::is_none")]
    pub field_71a: Option<Field71A>,

    // Sender's Charges (optional)
    #[serde(rename = "71F", skip_serializing_if = "Option::is_none")]
    pub field_71f: Option<Field71F>,

    // Receiver's Charges (optional)
    #[serde(rename = "71G", skip_serializing_if = "Option::is_none")]
    pub field_71g: Option<Field71G>,

    // Exchange Rate (optional)
    #[serde(rename = "36", skip_serializing_if = "Option::is_none")]
    pub field_36: Option<Field36>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT107 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Instruction Code (optional)
    #[serde(rename = "23E", skip_serializing_if = "Option::is_none")]
    pub field_23e: Option<Field23E>,

    // Registration Reference (optional)
    #[serde(rename = "21E", skip_serializing_if = "Option::is_none")]
    pub field_21e: Option<Field21E>,

    // Requested Execution Date
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Sending Institution (optional)
    #[serde(rename = "51A", skip_serializing_if = "Option::is_none")]
    pub field_51a: Option<Field51A>,

    // Instructing Party (message level, optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50_instructing: Option<Field50InstructingParty>,

    // Creditor (message level, optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_50_creditor: Option<Field50Creditor>,

    // Creditor's Bank (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52OrderingInstitution>,

    // Transaction Type Code (optional)
    #[serde(rename = "26T", skip_serializing_if = "Option::is_none")]
    pub field_26t: Option<Field26T>,

    // Regulatory Reporting (optional)
    #[serde(rename = "77B", skip_serializing_if = "Option::is_none")]
    pub field_77b: Option<Field77B>,

    // Details of Charges (optional)
    #[serde(rename = "71A", skip_serializing_if = "Option::is_none")]
    pub field_71a: Option<Field71A>,

    // Sender to Receiver Information (optional)
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub field_72: Option<Field72>,

    // Transaction Details (repeating sequence)
    #[serde(rename = "#")]
    pub transactions: Vec<MT107Transaction>,

    // Settlement Amount (Sequence C - mandatory)
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Sum of Amounts (Sequence C - optional)
    #[serde(rename = "19", skip_serializing_if = "Option::is_none")]
    pub field_19: Option<Field19>,

    // Sum of Sender's Charges (Sequence C - optional)
    #[serde(rename = "71F", skip_serializing_if = "Option::is_none")]
    pub field_71f: Option<Field71F>,

    // Sum of Receiver's Charges (Sequence C - optional)
    #[serde(rename = "71G", skip_serializing_if = "Option::is_none")]
    pub field_71g: Option<Field71G>,

    // Sender's Correspondent (Sequence C - optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_53: Option<Field53SenderCorrespondent>,
}

impl MT107 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "107");

        // Parse Sequence A - General Information
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_23e = parser.parse_optional_field::<Field23E>("23E")?;
        let field_21e = parser.parse_optional_field::<Field21E>("21E")?;
        let field_30 = parser.parse_field::<Field30>("30")?;
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;

        // Try to parse field 50 - could be instructing party (C/L) or creditor (A/K)
        // We'll need to detect the variant and determine which type
        let (field_50_instructing, field_50_creditor) = Self::parse_field_50(&mut parser)?;

        let field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
        let field_26t = parser.parse_optional_field::<Field26T>("26T")?;
        let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
        let field_71a = parser.parse_optional_field::<Field71A>("71A")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Parse Sequence B - Transaction Details (repeating)
        let mut transactions = Vec::new();
        parser = parser.with_duplicates(true);

        while parser.detect_field("21") {
            let txn_field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let txn_field_23e = parser.parse_optional_field::<Field23E>("23E")?;
            let txn_field_21c = parser.parse_optional_field::<Field21C>("21C")?;
            let txn_field_21d = parser.parse_optional_field::<Field21D>("21D")?;
            let txn_field_21e = parser.parse_optional_field::<Field21E>("21E")?;
            let txn_field_32b = parser.parse_field::<Field32B>("32B")?;

            let (txn_field_50_instructing, txn_field_50_creditor) = Self::parse_field_50(&mut parser)?;

            let txn_field_52 = parser.parse_optional_variant_field::<Field52OrderingInstitution>("52")?;
            let txn_field_57 = parser.parse_optional_variant_field::<Field57>("57")?;
            let txn_field_59 = parser.parse_variant_field::<Field59>("59")?;
            let txn_field_70 = parser.parse_optional_field::<Field70>("70")?;
            let txn_field_26t = parser.parse_optional_field::<Field26T>("26T")?;
            let txn_field_77b = parser.parse_optional_field::<Field77B>("77B")?;
            let txn_field_33b = parser.parse_optional_field::<Field33B>("33B")?;
            let txn_field_71a = parser.parse_optional_field::<Field71A>("71A")?;
            let txn_field_71f = parser.parse_optional_field::<Field71F>("71F")?;
            let txn_field_71g = parser.parse_optional_field::<Field71G>("71G")?;
            let txn_field_36 = parser.parse_optional_field::<Field36>("36")?;

            transactions.push(MT107Transaction {
                field_21: txn_field_21,
                field_23e: txn_field_23e,
                field_21c: txn_field_21c,
                field_21d: txn_field_21d,
                field_21e: txn_field_21e,
                field_32b: txn_field_32b,
                field_50_instructing: txn_field_50_instructing,
                field_50_creditor: txn_field_50_creditor,
                field_52: txn_field_52,
                field_57: txn_field_57,
                field_59: txn_field_59,
                field_70: txn_field_70,
                field_26t: txn_field_26t,
                field_77b: txn_field_77b,
                field_33b: txn_field_33b,
                field_71a: txn_field_71a,
                field_71f: txn_field_71f,
                field_71g: txn_field_71g,
                field_36: txn_field_36,
            });
        }

        // Parse Sequence C - Settlement Details
        // Note: duplicates remain enabled to allow parsing field 32B again
        let settlement_field_32b = parser.parse_field::<Field32B>("32B")?;
        let settlement_field_19 = parser.parse_optional_field::<Field19>("19")?;
        let settlement_field_71f = parser.parse_optional_field::<Field71F>("71F")?;
        let settlement_field_71g = parser.parse_optional_field::<Field71G>("71G")?;
        let settlement_field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;

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
            field_21e,
            field_30,
            field_51a,
            field_50_instructing,
            field_50_creditor,
            field_52,
            field_26t,
            field_77b,
            field_71a,
            field_72,
            transactions,
            field_32b: settlement_field_32b,
            field_19: settlement_field_19,
            field_71f: settlement_field_71f,
            field_71g: settlement_field_71g,
            field_53: settlement_field_53,
        })
    }

    /// Helper to parse field 50 which can be either instructing party (C/L) or creditor (A/K)
    fn parse_field_50(
        parser: &mut crate::message_parser::MessageParser,
    ) -> Result<(Option<Field50InstructingParty>, Option<Field50Creditor>), crate::errors::ParseError> {
        // Detect which variant of field 50 is present
        let remaining = parser.remaining();
        let trimmed = remaining.trim_start_matches(|c: char| c.is_whitespace());

        // Check for instructing party variants (C, L)
        if trimmed.starts_with(":50C:") {
            let field_50_instructing = parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            return Ok((field_50_instructing, None));
        }
        if trimmed.starts_with(":50L:") {
            let field_50_instructing = parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            return Ok((field_50_instructing, None));
        }

        // Check for creditor variants (A, K)
        if trimmed.starts_with(":50A:") || trimmed.starts_with(":50K:") {
            let field_50_creditor = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
            return Ok((None, field_50_creditor));
        }

        // No field 50 present
        Ok((None, None))
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
}

impl crate::traits::SwiftMessageBody for MT107 {
    fn message_type() -> &'static str {
        "107"
    }

    fn from_fields(
        fields: std::collections::HashMap<String, Vec<(String, usize)>>,
    ) -> Result<Self, crate::errors::ParseError> {
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
        let mut fields = std::collections::HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.to_swift_value()]);

        if let Some(ref field_23e) = self.field_23e {
            fields.insert("23E".to_string(), vec![field_23e.to_swift_value()]);
        }

        if let Some(ref field_21e) = self.field_21e {
            fields.insert("21E".to_string(), vec![field_21e.to_swift_value()]);
        }

        fields.insert("30".to_string(), vec![self.field_30.to_swift_value()]);

        if let Some(ref field_51a) = self.field_51a {
            fields.insert("51A".to_string(), vec![field_51a.to_swift_value()]);
        }

        if let Some(ref field_50) = self.field_50_instructing {
            if let Some(variant_tag) = field_50.get_variant_tag() {
                fields.insert(format!("50{}", variant_tag), vec![field_50.to_swift_value()]);
            }
        }

        if let Some(ref field_50) = self.field_50_creditor {
            if let Some(variant_tag) = field_50.get_variant_tag() {
                fields.insert(format!("50{}", variant_tag), vec![field_50.to_swift_value()]);
            }
        }

        if let Some(ref field_52) = self.field_52 {
            if let Some(variant_tag) = field_52.get_variant_tag() {
                fields.insert(format!("52{}", variant_tag), vec![field_52.to_swift_value()]);
            }
        }

        if let Some(ref field_26t) = self.field_26t {
            fields.insert("26T".to_string(), vec![field_26t.to_swift_value()]);
        }

        if let Some(ref field_77b) = self.field_77b {
            fields.insert("77B".to_string(), vec![field_77b.to_swift_value()]);
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

            if let Some(ref field_21c) = transaction.field_21c {
                fields.entry("21C".to_string()).or_default().push(field_21c.to_swift_value());
            }

            if let Some(ref field_21d) = transaction.field_21d {
                fields.entry("21D".to_string()).or_default().push(field_21d.to_swift_value());
            }

            if let Some(ref field_21e) = transaction.field_21e {
                fields.entry("21E".to_string()).or_default().push(field_21e.to_swift_value());
            }

            fields.entry("32B".to_string()).or_default().push(transaction.field_32b.to_swift_value());

            if let Some(ref field_50) = transaction.field_50_instructing {
                if let Some(variant_tag) = field_50.get_variant_tag() {
                    fields.entry(format!("50{}", variant_tag)).or_default().push(field_50.to_swift_value());
                }
            }

            if let Some(ref field_50) = transaction.field_50_creditor {
                if let Some(variant_tag) = field_50.get_variant_tag() {
                    fields.entry(format!("50{}", variant_tag)).or_default().push(field_50.to_swift_value());
                }
            }

            if let Some(ref field_52) = transaction.field_52 {
                if let Some(variant_tag) = field_52.get_variant_tag() {
                    fields.entry(format!("52{}", variant_tag)).or_default().push(field_52.to_swift_value());
                }
            }

            if let Some(ref field_57) = transaction.field_57 {
                if let Some(variant_tag) = field_57.get_variant_tag() {
                    fields.entry(format!("57{}", variant_tag)).or_default().push(field_57.to_swift_value());
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

            if let Some(ref field_26t) = transaction.field_26t {
                fields.entry("26T".to_string()).or_default().push(field_26t.to_swift_value());
            }

            if let Some(ref field_77b) = transaction.field_77b {
                fields.entry("77B".to_string()).or_default().push(field_77b.to_swift_value());
            }

            if let Some(ref field_33b) = transaction.field_33b {
                fields.entry("33B".to_string()).or_default().push(field_33b.to_swift_value());
            }

            if let Some(ref field_71a) = transaction.field_71a {
                fields.entry("71A".to_string()).or_default().push(field_71a.to_swift_value());
            }

            if let Some(ref field_71f) = transaction.field_71f {
                fields.entry("71F".to_string()).or_default().push(field_71f.to_swift_value());
            }

            if let Some(ref field_71g) = transaction.field_71g {
                fields.entry("71G".to_string()).or_default().push(field_71g.to_swift_value());
            }

            if let Some(ref field_36) = transaction.field_36 {
                fields.entry("36".to_string()).or_default().push(field_36.to_swift_value());
            }
        }

        // Add Sequence C fields
        fields.entry("32B".to_string()).or_default().push(self.field_32b.to_swift_value());

        if let Some(ref field_19) = self.field_19 {
            fields.insert("19".to_string(), vec![field_19.to_swift_value()]);
        }

        if let Some(ref field_71f) = self.field_71f {
            fields.insert("71F".to_string(), vec![field_71f.to_swift_value()]);
        }

        if let Some(ref field_71g) = self.field_71g {
            fields.insert("71G".to_string(), vec![field_71g.to_swift_value()]);
        }

        if let Some(ref field_53) = self.field_53 {
            if let Some(variant_tag) = field_53.get_variant_tag() {
                fields.insert(format!("53{}", variant_tag), vec![field_53.to_swift_value()]);
            }
        }

        fields
    }

    fn to_ordered_fields(&self) -> Vec<(String, String)> {
        use crate::traits::SwiftField;
        let mut ordered_fields = Vec::new();

        // Sequence A - General Information
        ordered_fields.push(("20".to_string(), self.field_20.to_swift_value()));

        if let Some(ref field_23e) = self.field_23e {
            ordered_fields.push(("23E".to_string(), field_23e.to_swift_value()));
        }

        if let Some(ref field_21e) = self.field_21e {
            ordered_fields.push(("21E".to_string(), field_21e.to_swift_value()));
        }

        ordered_fields.push(("30".to_string(), self.field_30.to_swift_value()));

        if let Some(ref field_51a) = self.field_51a {
            ordered_fields.push(("51A".to_string(), field_51a.to_swift_value()));
        }

        if let Some(ref field_50) = self.field_50_instructing {
            if let Some(variant_tag) = field_50.get_variant_tag() {
                ordered_fields.push((format!("50{}", variant_tag), field_50.to_swift_value()));
            }
        }

        if let Some(ref field_50) = self.field_50_creditor {
            if let Some(variant_tag) = field_50.get_variant_tag() {
                ordered_fields.push((format!("50{}", variant_tag), field_50.to_swift_value()));
            }
        }

        if let Some(ref field_52) = self.field_52 {
            if let Some(variant_tag) = field_52.get_variant_tag() {
                ordered_fields.push((format!("52{}", variant_tag), field_52.to_swift_value()));
            }
        }

        if let Some(ref field_26t) = self.field_26t {
            ordered_fields.push(("26T".to_string(), field_26t.to_swift_value()));
        }

        if let Some(ref field_77b) = self.field_77b {
            ordered_fields.push(("77B".to_string(), field_77b.to_swift_value()));
        }

        if let Some(ref field_71a) = self.field_71a {
            ordered_fields.push(("71A".to_string(), field_71a.to_swift_value()));
        }

        if let Some(ref field_72) = self.field_72 {
            ordered_fields.push(("72".to_string(), field_72.to_swift_value()));
        }

        // Sequence B - Transaction Details
        for transaction in &self.transactions {
            ordered_fields.push(("21".to_string(), transaction.field_21.to_swift_value()));

            if let Some(ref field_23e) = transaction.field_23e {
                ordered_fields.push(("23E".to_string(), field_23e.to_swift_value()));
            }

            if let Some(ref field_21c) = transaction.field_21c {
                ordered_fields.push(("21C".to_string(), field_21c.to_swift_value()));
            }

            if let Some(ref field_21d) = transaction.field_21d {
                ordered_fields.push(("21D".to_string(), field_21d.to_swift_value()));
            }

            if let Some(ref field_21e) = transaction.field_21e {
                ordered_fields.push(("21E".to_string(), field_21e.to_swift_value()));
            }

            ordered_fields.push(("32B".to_string(), transaction.field_32b.to_swift_value()));

            if let Some(ref field_50) = transaction.field_50_instructing {
                if let Some(variant_tag) = field_50.get_variant_tag() {
                    ordered_fields.push((format!("50{}", variant_tag), field_50.to_swift_value()));
                }
            }

            if let Some(ref field_50) = transaction.field_50_creditor {
                if let Some(variant_tag) = field_50.get_variant_tag() {
                    ordered_fields.push((format!("50{}", variant_tag), field_50.to_swift_value()));
                }
            }

            if let Some(ref field_52) = transaction.field_52 {
                if let Some(variant_tag) = field_52.get_variant_tag() {
                    ordered_fields.push((format!("52{}", variant_tag), field_52.to_swift_value()));
                }
            }

            if let Some(ref field_57) = transaction.field_57 {
                if let Some(variant_tag) = field_57.get_variant_tag() {
                    ordered_fields.push((format!("57{}", variant_tag), field_57.to_swift_value()));
                }
            }

            if let Some(variant_tag) = transaction.field_59.get_variant_tag() {
                ordered_fields.push((format!("59{}", variant_tag), transaction.field_59.to_swift_value()));
            } else {
                ordered_fields.push(("59".to_string(), transaction.field_59.to_swift_value()));
            }

            if let Some(ref field_70) = transaction.field_70 {
                ordered_fields.push(("70".to_string(), field_70.to_swift_value()));
            }

            if let Some(ref field_26t) = transaction.field_26t {
                ordered_fields.push(("26T".to_string(), field_26t.to_swift_value()));
            }

            if let Some(ref field_77b) = transaction.field_77b {
                ordered_fields.push(("77B".to_string(), field_77b.to_swift_value()));
            }

            if let Some(ref field_33b) = transaction.field_33b {
                ordered_fields.push(("33B".to_string(), field_33b.to_swift_value()));
            }

            if let Some(ref field_71a) = transaction.field_71a {
                ordered_fields.push(("71A".to_string(), field_71a.to_swift_value()));
            }

            if let Some(ref field_71f) = transaction.field_71f {
                ordered_fields.push(("71F".to_string(), field_71f.to_swift_value()));
            }

            if let Some(ref field_71g) = transaction.field_71g {
                ordered_fields.push(("71G".to_string(), field_71g.to_swift_value()));
            }

            if let Some(ref field_36) = transaction.field_36 {
                ordered_fields.push(("36".to_string(), field_36.to_swift_value()));
            }
        }

        // Sequence C - Settlement Details
        ordered_fields.push(("32B".to_string(), self.field_32b.to_swift_value()));

        if let Some(ref field_19) = self.field_19 {
            ordered_fields.push(("19".to_string(), field_19.to_swift_value()));
        }

        if let Some(ref field_71f) = self.field_71f {
            ordered_fields.push(("71F".to_string(), field_71f.to_swift_value()));
        }

        if let Some(ref field_71g) = self.field_71g {
            ordered_fields.push(("71G".to_string(), field_71g.to_swift_value()));
        }

        if let Some(ref field_53) = self.field_53 {
            if let Some(variant_tag) = field_53.get_variant_tag() {
                ordered_fields.push((format!("53{}", variant_tag), field_53.to_swift_value()));
            }
        }

        ordered_fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "30", "32B", "21", "59"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["23E", "21E", "51A", "50", "52", "26T", "77B", "71A", "72", "21C", "21D", "57", "70", "33B", "71F", "71G", "36", "19", "53"]
    }
}
