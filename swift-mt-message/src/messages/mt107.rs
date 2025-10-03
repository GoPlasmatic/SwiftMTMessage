use crate::fields::*;
use crate::parsing_utils::*;
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
    pub instructing_party_tx: Option<Field50InstructingParty>,

    // Creditor (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub creditor_tx: Option<Field50Creditor>,

    // Creditor's Bank (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52CreditorBank>,

    // Debtor's Bank (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_57: Option<Field57DebtorBank>,

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
    pub instructing_party: Option<Field50InstructingParty>,

    // Creditor (message level, optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub creditor: Option<Field50Creditor>,

    // Creditor's Bank (optional)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field_52: Option<Field52CreditorBank>,

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
        let (instructing_party, creditor) = Self::parse_field_50(&mut parser)?;

        let field_52 = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
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

            let (instructing_party_tx, creditor_tx) = Self::parse_field_50(&mut parser)?;

            let txn_field_52 = parser.parse_optional_variant_field::<Field52CreditorBank>("52")?;
            let txn_field_57 = parser.parse_optional_variant_field::<Field57DebtorBank>("57")?;
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
                instructing_party_tx,
                creditor_tx,
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
        let settlement_field_53 =
            parser.parse_optional_variant_field::<Field53SenderCorrespondent>("53")?;

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
            instructing_party,
            creditor,
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
    ) -> Result<(Option<Field50InstructingParty>, Option<Field50Creditor>), crate::errors::ParseError>
    {
        // Detect which variant of field 50 is present
        let remaining = parser.remaining();
        let trimmed = remaining.trim_start_matches(|c: char| c.is_whitespace());

        // Check for instructing party variants (C, L)
        if trimmed.starts_with(":50C:") {
            let instructing_party =
                parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            return Ok((instructing_party, None));
        }
        if trimmed.starts_with(":50L:") {
            let instructing_party =
                parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            return Ok((instructing_party, None));
        }

        // Check for creditor variants (A, K)
        if trimmed.starts_with(":50A:") || trimmed.starts_with(":50K:") {
            let creditor = parser.parse_optional_variant_field::<Field50Creditor>("50")?;
            return Ok((None, creditor));
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

    fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        Self::parse_from_block4(block4)
    }

    fn to_mt_string(&self) -> String {
        let mut result = String::new();

        // Sequence A - General Information
        append_field(&mut result, &self.field_20);
        append_optional_field(&mut result, &self.field_23e);
        append_optional_field(&mut result, &self.field_21e);
        append_field(&mut result, &self.field_30);
        append_optional_field(&mut result, &self.field_51a);
        append_optional_field(&mut result, &self.instructing_party);
        append_optional_field(&mut result, &self.creditor);
        append_optional_field(&mut result, &self.field_52);
        append_optional_field(&mut result, &self.field_26t);
        append_optional_field(&mut result, &self.field_77b);
        append_optional_field(&mut result, &self.field_71a);
        append_optional_field(&mut result, &self.field_72);

        // Sequence B - Transaction Details
        for txn in &self.transactions {
            append_field(&mut result, &txn.field_21);
            append_optional_field(&mut result, &txn.field_23e);
            append_optional_field(&mut result, &txn.field_21c);
            append_optional_field(&mut result, &txn.field_21d);
            append_optional_field(&mut result, &txn.field_21e);
            append_field(&mut result, &txn.field_32b);
            append_optional_field(&mut result, &txn.instructing_party_tx);
            append_optional_field(&mut result, &txn.creditor_tx);
            append_optional_field(&mut result, &txn.field_52);
            append_optional_field(&mut result, &txn.field_57);
            append_field(&mut result, &txn.field_59);
            append_optional_field(&mut result, &txn.field_70);
            append_optional_field(&mut result, &txn.field_26t);
            append_optional_field(&mut result, &txn.field_77b);
            append_optional_field(&mut result, &txn.field_33b);
            append_optional_field(&mut result, &txn.field_71a);
            append_optional_field(&mut result, &txn.field_71f);
            append_optional_field(&mut result, &txn.field_71g);
            append_optional_field(&mut result, &txn.field_36);
        }

        // Sequence C - Settlement Details
        append_field(&mut result, &self.field_32b);
        append_optional_field(&mut result, &self.field_19);
        append_optional_field(&mut result, &self.field_71f);
        append_optional_field(&mut result, &self.field_71g);
        append_optional_field(&mut result, &self.field_53);

        finalize_mt_string(result, false)
    }
}
