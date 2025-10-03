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
    #[serde(rename = "23E")]
    pub field_23e: Option<Field23E>,

    // Mandate Reference (optional)
    #[serde(rename = "21C")]
    pub field_21c: Option<Field21C>,

    // Direct Debit Reference (optional)
    #[serde(rename = "21D")]
    pub field_21d: Option<Field21D>,

    // Registration Reference (optional)
    #[serde(rename = "21E")]
    pub field_21e: Option<Field21E>,

    // Currency/Amount
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Instructing Party (Transaction level)
    #[serde(flatten)]
    pub instructing_party_tx: Option<Field50InstructingParty>,

    // Creditor (Transaction level)
    #[serde(flatten)]
    pub creditor_tx: Option<Field50Creditor>,

    // Creditor's Bank
    #[serde(flatten)]
    pub field_52: Option<Field52CreditorBank>,

    // Debtor's Bank
    #[serde(flatten)]
    pub field_57: Option<Field57DebtorBank>,

    // Debtor
    #[serde(flatten)]
    pub field_59: Field59Debtor,

    // Remittance Information
    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    // Transaction Type Code
    #[serde(rename = "26T")]
    pub field_26t: Option<Field26T>,

    // Regulatory Reporting
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    // Original Ordered Amount
    #[serde(rename = "33B")]
    pub field_33b: Option<Field33B>,

    // Details of Charges
    #[serde(rename = "71A")]
    pub field_71a: Option<Field71A>,

    // Sender's Charges
    #[serde(rename = "71F")]
    pub field_71f: Option<Field71F>,

    // Receiver's Charges
    #[serde(rename = "71G")]
    pub field_71g: Option<Field71G>,

    // Exchange Rate
    #[serde(rename = "36")]
    pub field_36: Option<Field36>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT104 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Customer Specified Reference
    #[serde(rename = "21R")]
    pub field_21r: Option<Field21R>,

    // Instruction Code
    #[serde(rename = "23E")]
    pub field_23e: Option<Field23E>,

    // Registration Reference
    #[serde(rename = "21E")]
    pub field_21e: Option<Field21E>,

    // Requested Execution Date
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Sending Institution
    #[serde(rename = "51A")]
    pub field_51a: Option<Field51A>,

    // Instructing Party
    #[serde(flatten)]
    pub instructing_party: Option<Field50InstructingParty>,

    // Creditor
    #[serde(flatten)]
    pub creditor: Option<Field50Creditor>,

    // Creditor's Bank
    #[serde(flatten)]
    pub field_52: Option<Field52CreditorBank>,

    // Transaction Type Code
    #[serde(rename = "26T")]
    pub field_26t: Option<Field26T>,

    // Regulatory Reporting
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    // Details of Charges
    #[serde(rename = "71A")]
    pub field_71a: Option<Field71A>,

    // Sender to Receiver Information
    #[serde(rename = "72")]
    pub field_72: Option<Field72>,

    // Transaction Details (repeating sequence)
    #[serde(rename = "#")]
    pub transactions: Vec<MT104Transaction>,

    // Currency and Settlement Amount (Sequence C)
    #[serde(rename = "32B")]
    pub field_32b: Option<Field32B>,

    // Sum of Amounts
    #[serde(rename = "19")]
    pub field_19: Option<Field19>,

    // Sum of Sender's Charges
    #[serde(rename = "71F")]
    pub field_71f: Option<Field71F>,

    // Sum of Receiver's Charges
    #[serde(rename = "71G")]
    pub field_71g: Option<Field71G>,

    // Sender's Correspondent
    #[serde(flatten)]
    pub field_53: Option<Field53SenderCorrespondent>,
}

impl MT104 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "104");

        // Parse Sequence A fields
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21r = parser.parse_optional_field::<Field21R>("21R")?;
        let field_23e = parser.parse_optional_field::<Field23E>("23E")?;
        let field_21e = parser.parse_optional_field::<Field21E>("21E")?;
        let field_30 = parser.parse_field::<Field30>("30")?;
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;

        // Parse optional ordering parties - check variant to determine field type
        let mut instructing_party = None;
        let mut creditor = None;

        // Check if field 50 exists and determine its type based on the variant
        if let Some(variant) = parser.peek_field_variant("50") {
            match variant.as_str() {
                "C" | "L" => {
                    // These variants are for Field50InstructingParty
                    instructing_party =
                        parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
                }
                "A" | "K" => {
                    // These variants are for Field50Creditor
                    creditor = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                }
                _ => {
                    // Unknown variant, try both
                    if let Ok(ip) =
                        parser.parse_optional_variant_field::<Field50InstructingParty>("50")
                    {
                        instructing_party = ip;
                    } else {
                        creditor = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                    }
                }
            }
        }

        let field_52 = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
        let field_26t = parser.parse_optional_field::<Field26T>("26T")?;
        let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
        let field_71a = parser.parse_optional_field::<Field71A>("71A")?;
        let field_72 = parser.parse_optional_field::<Field72>("72")?;

        // Parse Sequence B (transactions) - enable duplicates
        let mut transactions = Vec::new();
        parser = parser.with_duplicates(true);

        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let field_23e_tx = parser.parse_optional_field::<Field23E>("23E")?;
            let field_21c = parser.parse_optional_field::<Field21C>("21C")?;
            let field_21d = parser.parse_optional_field::<Field21D>("21D")?;
            let field_21e_tx = parser.parse_optional_field::<Field21E>("21E")?;
            let field_32b = parser.parse_field::<Field32B>("32B")?;

            // Transaction-level optional parties - check variant to determine field type
            let mut instructing_party_tx = None;
            let mut creditor_tx = None;

            // Check if field 50 exists and determine its type based on the variant
            if let Some(variant) = parser.peek_field_variant("50") {
                match variant.as_str() {
                    "C" | "L" => {
                        // These variants are for Field50InstructingParty
                        instructing_party_tx =
                            parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
                    }
                    "A" | "K" => {
                        // These variants are for Field50Creditor
                        creditor_tx =
                            parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                    }
                    _ => {
                        // Unknown variant, try both
                        if let Ok(ip) =
                            parser.parse_optional_variant_field::<Field50InstructingParty>("50")
                        {
                            instructing_party_tx = ip;
                        } else {
                            creditor_tx =
                                parser.parse_optional_variant_field::<Field50Creditor>("50")?;
                        }
                    }
                }
            }

            let field_52_tx = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
            let field_57 = parser.parse_optional_variant_field::<Field57DebtorBank>("57")?;
            let field_59 = parser.parse_variant_field::<Field59Debtor>("59")?;
            let field_70 = parser.parse_optional_field::<Field70>("70")?;
            let field_26t_tx = parser.parse_optional_field::<Field26T>("26T")?;
            let field_77b_tx = parser.parse_optional_field::<Field77B>("77B")?;
            let field_33b = parser.parse_optional_field::<Field33B>("33B")?;
            let field_71a_tx = parser.parse_optional_field::<Field71A>("71A")?;
            let field_71f = parser.parse_optional_field::<Field71F>("71F")?;
            let field_71g = parser.parse_optional_field::<Field71G>("71G")?;
            let field_36 = parser.parse_optional_field::<Field36>("36")?;

            transactions.push(MT104Transaction {
                field_21,
                field_23e: field_23e_tx,
                field_21c,
                field_21d,
                field_21e: field_21e_tx,
                field_32b,
                instructing_party_tx,
                creditor_tx,
                field_52: field_52_tx,
                field_57,
                field_59,
                field_70,
                field_26t: field_26t_tx,
                field_77b: field_77b_tx,
                field_33b,
                field_71a: field_71a_tx,
                field_71f,
                field_71g,
                field_36,
            });
        }

        // Parse Sequence C (optional settlement details)
        let field_32b = parser.parse_optional_field::<Field32B>("32B")?;
        let field_19 = parser.parse_optional_field::<Field19>("19")?;
        let field_71f = parser.parse_optional_field::<Field71F>("71F")?;
        let field_71g = parser.parse_optional_field::<Field71G>("71G")?;
        let field_53 = parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;

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
            field_21r,
            field_23e,
            field_21e,
            field_30,
            field_51a,
            instructing_party,
            creditor,
            field_52,
            field_26t,
            field_77b,
            field_71a,
            field_72,
            transactions,
            field_32b,
            field_19,
            field_71f,
            field_71g,
            field_53,
        })
    }

    /// Validation rules for the message
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#
    }

    /// Parse from generic SWIFT input
    pub fn parse(input: &str) -> Result<Self, crate::errors::ParseError> {
        let block4 = if input.contains("{4:") {
            // Extract block 4 content
            if let Some(start) = input.find("{4:") {
                if let Some(end) = input[start..].find("-}") {
                    input[start + 3..start + end].to_string()
                } else {
                    return Err(crate::errors::ParseError::InvalidFormat {
                        message: "Block 4 not properly terminated".to_string(),
                    });
                }
            } else {
                return Err(crate::errors::ParseError::InvalidFormat {
                    message: "Block 4 not found".to_string(),
                });
            }
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

        // Sequence A fields
        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field_21r) = self.field_21r {
            result.push_str(&field_21r.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_23e) = self.field_23e {
            result.push_str(&field_23e.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_21e) = self.field_21e {
            result.push_str(&field_21e.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_30.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field_51a) = self.field_51a {
            result.push_str(&field_51a.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref instructing_party) = self.instructing_party {
            result.push_str(&instructing_party.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref creditor) = self.creditor {
            result.push_str(&creditor.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_52) = self.field_52 {
            result.push_str(&field_52.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_26t) = self.field_26t {
            result.push_str(&field_26t.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_77b) = self.field_77b {
            result.push_str(&field_77b.to_swift_string());
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

        // Sequence B (transactions)
        for transaction in &self.transactions {
            result.push_str(&transaction.field_21.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field_23e) = transaction.field_23e {
                result.push_str(&field_23e.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_21c) = transaction.field_21c {
                result.push_str(&field_21c.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_21d) = transaction.field_21d {
                result.push_str(&field_21d.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_21e) = transaction.field_21e {
                result.push_str(&field_21e.to_swift_string());
                result.push_str("\r\n");
            }

            result.push_str(&transaction.field_32b.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref instructing_party_tx) = transaction.instructing_party_tx {
                result.push_str(&instructing_party_tx.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref creditor_tx) = transaction.creditor_tx {
                result.push_str(&creditor_tx.to_swift_string());
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

            if let Some(ref field_26t) = transaction.field_26t {
                result.push_str(&field_26t.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_77b) = transaction.field_77b {
                result.push_str(&field_77b.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_33b) = transaction.field_33b {
                result.push_str(&field_33b.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_71a) = transaction.field_71a {
                result.push_str(&field_71a.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_71f) = transaction.field_71f {
                result.push_str(&field_71f.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_71g) = transaction.field_71g {
                result.push_str(&field_71g.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_36) = transaction.field_36 {
                result.push_str(&field_36.to_swift_string());
                result.push_str("\r\n");
            }
        }

        // Sequence C (optional settlement)
        if let Some(ref field_32b) = self.field_32b {
            result.push_str(&field_32b.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_19) = self.field_19 {
            result.push_str(&field_19.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_71f) = self.field_71f {
            result.push_str(&field_71f.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_71g) = self.field_71g {
            result.push_str(&field_71g.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_53) = self.field_53 {
            result.push_str(&field_53.to_swift_string());
            result.push_str("\r\n");
        }

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

    fn to_ordered_fields(&self) -> Vec<(String, String)> {
        use crate::traits::SwiftField;
        let mut ordered_fields = Vec::new();

        // Add Sequence A fields
        ordered_fields.push(("20".to_string(), self.field_20.to_swift_value()));

        if let Some(ref field_21r) = self.field_21r {
            ordered_fields.push(("21R".to_string(), field_21r.to_swift_value()));
        }

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

        if let Some(ref instructing_party) = self.instructing_party {
            if let Some(tag) = instructing_party.get_variant_tag() {
                ordered_fields.push((format!("50{}", tag), instructing_party.to_swift_value()));
            } else {
                ordered_fields.push(("50".to_string(), instructing_party.to_swift_value()));
            }
        }

        if let Some(ref creditor) = self.creditor {
            if let Some(tag) = creditor.get_variant_tag() {
                ordered_fields.push((format!("50{}", tag), creditor.to_swift_value()));
            } else {
                ordered_fields.push(("50".to_string(), creditor.to_swift_value()));
            }
        }

        if let Some(ref field_52) = self.field_52 {
            if let Some(tag) = field_52.get_variant_tag() {
                ordered_fields.push((format!("52{}", tag), field_52.to_swift_value()));
            } else {
                ordered_fields.push(("52".to_string(), field_52.to_swift_value()));
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

        // Add Sequence B (transactions)
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

            if let Some(ref instructing_party_tx) = transaction.instructing_party_tx {
                if let Some(tag) = instructing_party_tx.get_variant_tag() {
                    ordered_fields
                        .push((format!("50{}", tag), instructing_party_tx.to_swift_value()));
                } else {
                    ordered_fields.push(("50".to_string(), instructing_party_tx.to_swift_value()));
                }
            }

            if let Some(ref creditor_tx) = transaction.creditor_tx {
                if let Some(tag) = creditor_tx.get_variant_tag() {
                    ordered_fields.push((format!("50{}", tag), creditor_tx.to_swift_value()));
                } else {
                    ordered_fields.push(("50".to_string(), creditor_tx.to_swift_value()));
                }
            }

            if let Some(ref field_52) = transaction.field_52 {
                if let Some(tag) = field_52.get_variant_tag() {
                    ordered_fields.push((format!("52{}", tag), field_52.to_swift_value()));
                } else {
                    ordered_fields.push(("52".to_string(), field_52.to_swift_value()));
                }
            }

            if let Some(ref field_57) = transaction.field_57 {
                if let Some(tag) = field_57.get_variant_tag() {
                    ordered_fields.push((format!("57{}", tag), field_57.to_swift_value()));
                } else {
                    ordered_fields.push(("57".to_string(), field_57.to_swift_value()));
                }
            }

            if let Some(tag) = transaction.field_59.get_variant_tag() {
                ordered_fields.push((format!("59{}", tag), transaction.field_59.to_swift_value()));
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

        // Add Sequence C (optional settlement)
        if let Some(ref field_32b) = self.field_32b {
            ordered_fields.push(("32B".to_string(), field_32b.to_swift_value()));
        }

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
            if let Some(tag) = field_53.get_variant_tag() {
                ordered_fields.push((format!("53{}", tag), field_53.to_swift_value()));
            } else {
                ordered_fields.push(("53".to_string(), field_53.to_swift_value()));
            }
        }

        ordered_fields
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Convert ordered fields to HashMap - this preserves duplicates through Vec
        let mut fields = std::collections::HashMap::new();

        for (tag, value) in self.to_ordered_fields() {
            fields.entry(tag).or_insert_with(Vec::new).push(value);
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "30", "21", "32B", "59"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec![
            "21R", "23E", "21E", "51A", "50", "52", "26T", "77B", "71A", "72", "21C", "21D", "70",
            "33B", "71F", "71G", "36", "57", "32B", "19", "53",
        ]
    }
}
