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

    // F/X Deal Reference
    #[serde(rename = "21F")]
    pub field_21f: Option<Field21F>,

    // Instruction Code
    #[serde(rename = "23E")]
    pub field_23e: Option<Vec<Field23E>>,

    // Currency/Amount
    #[serde(rename = "32B")]
    pub field_32b: Field32B,

    // Instructing Party (Transaction level)
    #[serde(flatten)]
    pub instructing_party_tx: Option<Field50InstructingParty>,

    // Ordering Customer (Transaction level)
    #[serde(flatten)]
    pub ordering_customer_tx: Option<Field50OrderingCustomerFGH>,

    // Account Servicing Institution
    #[serde(flatten)]
    pub field_52: Option<Field52AccountServicingInstitution>,

    // Intermediary
    #[serde(flatten)]
    pub field_56: Option<Field56Intermediary>,

    // Account With Institution
    #[serde(flatten)]
    pub field_57: Option<Field57AccountWithInstitution>,

    // Beneficiary Customer
    #[serde(flatten)]
    pub field_59: Field59,

    // Remittance Information
    #[serde(rename = "70")]
    pub field_70: Option<Field70>,

    // Regulatory Reporting
    #[serde(rename = "77B")]
    pub field_77b: Option<Field77B>,

    // Currency/Original Amount
    #[serde(rename = "33B")]
    pub field_33b: Option<Field33B>,

    // Details of Charges
    #[serde(rename = "71A")]
    pub field_71a: Field71A,

    // Charges Account
    #[serde(rename = "25A")]
    pub field_25a: Option<Field25A>,

    // Exchange Rate
    #[serde(rename = "36")]
    pub field_36: Option<Field36>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT101 {
    // Sender's Reference
    #[serde(rename = "20")]
    pub field_20: Field20,

    // Customer Specified Reference
    #[serde(rename = "21R")]
    pub field_21r: Option<Field21R>,

    // Message Index/Total
    #[serde(rename = "28D")]
    pub field_28d: Field28D,

    // Instructing Party
    #[serde(flatten)]
    pub instructing_party: Option<Field50InstructingParty>,

    // Ordering Customer
    #[serde(flatten)]
    pub ordering_customer: Option<Field50OrderingCustomerFGH>,

    // Account Servicing Institution (Seq A)
    #[serde(flatten)]
    pub field_52a: Option<Field52AccountServicingInstitution>,

    // Sending Institution
    #[serde(rename = "51A")]
    pub field_51a: Option<Field51A>,

    // Requested Execution Date
    #[serde(rename = "30")]
    pub field_30: Field30,

    // Account Identification
    #[serde(rename = "25")]
    pub field_25: Option<Field25NoOption>,

    // Transaction Details (repeating sequence)
    #[serde(rename = "#")]
    pub transactions: Vec<MT101Transaction>,
}

impl MT101 {
    /// Parse message from Block 4 content
    pub fn parse_from_block4(block4: &str) -> Result<Self, crate::errors::ParseError> {
        let mut parser = crate::message_parser::MessageParser::new(block4, "101");

        // Parse mandatory and optional fields in sequence A
        let field_20 = parser.parse_field::<Field20>("20")?;
        let field_21r = parser.parse_optional_field::<Field21R>("21R")?;
        let field_28d = parser.parse_field::<Field28D>("28D")?;

        // Parse optional ordering customer and instructing party (can appear in either order)
        let instructing_party =
            parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
        let ordering_customer =
            parser.parse_optional_variant_field::<Field50OrderingCustomerFGH>("50")?;

        let field_52a =
            parser.parse_optional_variant_field::<Field52AccountServicingInstitution>("52")?;
        let field_51a = parser.parse_optional_field::<Field51A>("51A")?;
        let field_30 = parser.parse_field::<Field30>("30")?;
        let field_25 = parser.parse_optional_field::<Field25NoOption>("25")?;

        // Parse transactions - this is a repeating sequence B
        let mut transactions = Vec::new();

        // Enable duplicates for repeating fields
        parser = parser.with_duplicates(true);

        // Parse each transaction - they start with field 21
        while parser.detect_field("21") {
            let field_21 = parser.parse_field::<Field21NoOption>("21")?;
            let field_21f = parser.parse_optional_field::<Field21F>("21F")?;

            // Field 23E can appear multiple times
            let field_23e = if parser.detect_field("23E") {
                let mut codes = Vec::new();
                while let Ok(field) = parser.parse_field::<Field23E>("23E") {
                    codes.push(field);
                }
                if !codes.is_empty() { Some(codes) } else { None }
            } else {
                None
            };

            let field_32b = parser.parse_field::<Field32B>("32B")?;

            // Transaction-level optional ordering parties
            let instructing_party_tx =
                parser.parse_optional_variant_field::<Field50InstructingParty>("50")?;
            let ordering_customer_tx =
                parser.parse_optional_variant_field::<Field50OrderingCustomerFGH>("50")?;

            let field_52 =
                parser.parse_optional_variant_field::<Field52AccountServicingInstitution>("52")?;
            let field_56 = parser.parse_optional_variant_field::<Field56Intermediary>("56")?;
            let field_57 =
                parser.parse_optional_variant_field::<Field57AccountWithInstitution>("57")?;
            let field_59 = parser.parse_variant_field::<Field59>("59")?;
            let field_70 = parser.parse_optional_field::<Field70>("70")?;
            let field_77b = parser.parse_optional_field::<Field77B>("77B")?;
            let field_33b = parser.parse_optional_field::<Field33B>("33B")?;
            let field_71a = parser.parse_field::<Field71A>("71A")?; // Mandatory
            let field_25a = parser.parse_optional_field::<Field25A>("25A")?;
            let field_36 = parser.parse_optional_field::<Field36>("36")?;

            transactions.push(MT101Transaction {
                field_21,
                field_21f,
                field_23e,
                field_32b,
                instructing_party_tx,
                ordering_customer_tx,
                field_52,
                field_56,
                field_57,
                field_59,
                field_70,
                field_77b,
                field_33b,
                field_71a,
                field_25a,
                field_36,
            });
        }

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
            field_28d,
            instructing_party,
            ordering_customer,
            field_52a,
            field_51a,
            field_30,
            field_25,
            transactions,
        })
    }

    /// Validation rules for the message
    pub fn validate() -> &'static str {
        r#"{"rules": [{"id": "BASIC", "description": "Basic validation", "condition": true}]}"#
    }

    /// Parse from generic SWIFT input (tries to detect blocks)
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

        // Add mandatory fields in sequence A
        result.push_str(&self.field_20.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field_21r) = self.field_21r {
            result.push_str(&field_21r.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_28d.to_swift_string());
        result.push_str("\r\n");

        // Optional ordering parties
        if let Some(ref instructing_party) = self.instructing_party {
            result.push_str(&instructing_party.to_swift_string());
            result.push_str("\r\n");
        }
        if let Some(ref ordering_customer) = self.ordering_customer {
            result.push_str(&ordering_customer.to_swift_string());
            result.push_str("\r\n");
        }

        if let Some(ref field_52a) = self.field_52a {
            result.push_str(&field_52a.to_swift_string());
            result.push_str("\r\n");
        }
        if let Some(ref field_51a) = self.field_51a {
            result.push_str(&field_51a.to_swift_string());
            result.push_str("\r\n");
        }

        result.push_str(&self.field_30.to_swift_string());
        result.push_str("\r\n");

        if let Some(ref field_25) = self.field_25 {
            result.push_str(&field_25.to_swift_string());
            result.push_str("\r\n");
        }

        // Add transactions (sequence B)
        for transaction in &self.transactions {
            result.push_str(&transaction.field_21.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field_21f) = transaction.field_21f {
                result.push_str(&field_21f.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_23e_vec) = transaction.field_23e {
                for field_23e in field_23e_vec {
                    result.push_str(&field_23e.to_swift_string());
                    result.push_str("\r\n");
                }
            }

            result.push_str(&transaction.field_32b.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref instructing_party_tx) = transaction.instructing_party_tx {
                result.push_str(&instructing_party_tx.to_swift_string());
                result.push_str("\r\n");
            }
            if let Some(ref ordering_customer_tx) = transaction.ordering_customer_tx {
                result.push_str(&ordering_customer_tx.to_swift_string());
                result.push_str("\r\n");
            }

            if let Some(ref field_52) = transaction.field_52 {
                result.push_str(&field_52.to_swift_string());
                result.push_str("\r\n");
            }
            if let Some(ref field_56) = transaction.field_56 {
                result.push_str(&field_56.to_swift_string());
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
            if let Some(ref field_77b) = transaction.field_77b {
                result.push_str(&field_77b.to_swift_string());
                result.push_str("\r\n");
            }
            if let Some(ref field_33b) = transaction.field_33b {
                result.push_str(&field_33b.to_swift_string());
                result.push_str("\r\n");
            }

            // Field 71A is mandatory
            result.push_str(&transaction.field_71a.to_swift_string());
            result.push_str("\r\n");

            if let Some(ref field_25a) = transaction.field_25a {
                result.push_str(&field_25a.to_swift_string());
                result.push_str("\r\n");
            }
            if let Some(ref field_36) = transaction.field_36 {
                result.push_str(&field_36.to_swift_string());
                result.push_str("\r\n");
            }
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

        // Add header fields in sequence A
        ordered_fields.push(("20".to_string(), self.field_20.to_swift_value()));

        if let Some(ref field_21r) = self.field_21r {
            ordered_fields.push(("21R".to_string(), field_21r.to_swift_value()));
        }

        ordered_fields.push(("28D".to_string(), self.field_28d.to_swift_value()));

        // Add optional ordering parties
        if let Some(ref instructing_party) = self.instructing_party {
            if let Some(variant_tag) = instructing_party.get_variant_tag() {
                ordered_fields.push((
                    format!("50{}", variant_tag),
                    instructing_party.to_swift_value(),
                ));
            } else {
                ordered_fields.push(("50".to_string(), instructing_party.to_swift_value()));
            }
        }
        if let Some(ref ordering_customer) = self.ordering_customer {
            if let Some(variant_tag) = ordering_customer.get_variant_tag() {
                ordered_fields.push((
                    format!("50{}", variant_tag),
                    ordering_customer.to_swift_value(),
                ));
            } else {
                ordered_fields.push(("50".to_string(), ordering_customer.to_swift_value()));
            }
        }

        if let Some(ref field_52a) = self.field_52a {
            if let Some(variant_tag) = field_52a.get_variant_tag() {
                ordered_fields.push((format!("52{}", variant_tag), field_52a.to_swift_value()));
            } else {
                ordered_fields.push(("52".to_string(), field_52a.to_swift_value()));
            }
        }
        if let Some(ref field_51a) = self.field_51a {
            ordered_fields.push(("51A".to_string(), field_51a.to_swift_value()));
        }

        ordered_fields.push(("30".to_string(), self.field_30.to_swift_value()));

        if let Some(ref field_25) = self.field_25 {
            ordered_fields.push(("25".to_string(), field_25.to_swift_value()));
        }

        // Add transaction fields in sequence B
        for transaction in &self.transactions {
            ordered_fields.push(("21".to_string(), transaction.field_21.to_swift_value()));

            if let Some(ref field_21f) = transaction.field_21f {
                ordered_fields.push(("21F".to_string(), field_21f.to_swift_value()));
            }

            if let Some(ref field_23e_vec) = transaction.field_23e {
                for field_23e in field_23e_vec {
                    ordered_fields.push(("23E".to_string(), field_23e.to_swift_value()));
                }
            }

            ordered_fields.push(("32B".to_string(), transaction.field_32b.to_swift_value()));

            // Transaction-level ordering parties
            if let Some(ref instructing_party_tx) = transaction.instructing_party_tx {
                if let Some(variant_tag) = instructing_party_tx.get_variant_tag() {
                    ordered_fields.push((
                        format!("50{}", variant_tag),
                        instructing_party_tx.to_swift_value(),
                    ));
                } else {
                    ordered_fields.push(("50".to_string(), instructing_party_tx.to_swift_value()));
                }
            }
            if let Some(ref ordering_customer_tx) = transaction.ordering_customer_tx {
                if let Some(variant_tag) = ordering_customer_tx.get_variant_tag() {
                    ordered_fields.push((
                        format!("50{}", variant_tag),
                        ordering_customer_tx.to_swift_value(),
                    ));
                } else {
                    ordered_fields.push(("50".to_string(), ordering_customer_tx.to_swift_value()));
                }
            }

            if let Some(ref field_52) = transaction.field_52 {
                if let Some(variant_tag) = field_52.get_variant_tag() {
                    ordered_fields.push((format!("52{}", variant_tag), field_52.to_swift_value()));
                } else {
                    ordered_fields.push(("52".to_string(), field_52.to_swift_value()));
                }
            }
            if let Some(ref field_56) = transaction.field_56 {
                if let Some(variant_tag) = field_56.get_variant_tag() {
                    ordered_fields.push((format!("56{}", variant_tag), field_56.to_swift_value()));
                } else {
                    ordered_fields.push(("56".to_string(), field_56.to_swift_value()));
                }
            }
            if let Some(ref field_57) = transaction.field_57 {
                if let Some(variant_tag) = field_57.get_variant_tag() {
                    ordered_fields.push((format!("57{}", variant_tag), field_57.to_swift_value()));
                } else {
                    ordered_fields.push(("57".to_string(), field_57.to_swift_value()));
                }
            }

            // Field 59 with variant
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
            if let Some(ref field_77b) = transaction.field_77b {
                ordered_fields.push(("77B".to_string(), field_77b.to_swift_value()));
            }
            if let Some(ref field_33b) = transaction.field_33b {
                ordered_fields.push(("33B".to_string(), field_33b.to_swift_value()));
            }

            // Field 71A is mandatory
            ordered_fields.push(("71A".to_string(), transaction.field_71a.to_swift_value()));

            if let Some(ref field_25a) = transaction.field_25a {
                ordered_fields.push(("25A".to_string(), field_25a.to_swift_value()));
            }
            if let Some(ref field_36) = transaction.field_36 {
                ordered_fields.push(("36".to_string(), field_36.to_swift_value()));
            }
        }

        ordered_fields
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        use crate::traits::SwiftField;
        let mut fields = std::collections::HashMap::new();

        fields.insert("20".to_string(), vec![self.field_20.to_swift_value()]);
        if let Some(ref field_21r) = self.field_21r {
            fields.insert("21R".to_string(), vec![field_21r.to_swift_value()]);
        }
        fields.insert("28D".to_string(), vec![self.field_28d.to_swift_value()]);

        // Optional ordering parties
        if let Some(ref instructing_party) = self.instructing_party {
            if let Some(variant_tag) = instructing_party.get_variant_tag() {
                fields.insert(
                    format!("50{}", variant_tag),
                    vec![instructing_party.to_swift_value()],
                );
            } else {
                fields.insert("50".to_string(), vec![instructing_party.to_swift_value()]);
            }
        }
        if let Some(ref ordering_customer) = self.ordering_customer {
            if let Some(variant_tag) = ordering_customer.get_variant_tag() {
                fields.insert(
                    format!("50{}", variant_tag),
                    vec![ordering_customer.to_swift_value()],
                );
            } else {
                fields.insert("50".to_string(), vec![ordering_customer.to_swift_value()]);
            }
        }

        if let Some(ref field_52a) = self.field_52a {
            if let Some(variant_tag) = field_52a.get_variant_tag() {
                fields.insert(
                    format!("52{}", variant_tag),
                    vec![field_52a.to_swift_value()],
                );
            } else {
                fields.insert("52".to_string(), vec![field_52a.to_swift_value()]);
            }
        }
        if let Some(ref field_51a) = self.field_51a {
            fields.insert("51A".to_string(), vec![field_51a.to_swift_value()]);
        }
        fields.insert("30".to_string(), vec![self.field_30.to_swift_value()]);
        if let Some(ref field_25) = self.field_25 {
            fields.insert("25".to_string(), vec![field_25.to_swift_value()]);
        }

        // Transaction fields can repeat
        for transaction in &self.transactions {
            fields
                .entry("21".to_string())
                .or_default()
                .push(transaction.field_21.to_swift_value());

            if let Some(ref field_21f) = transaction.field_21f {
                fields
                    .entry("21F".to_string())
                    .or_default()
                    .push(field_21f.to_swift_value());
            }

            if let Some(ref field_23e_vec) = transaction.field_23e {
                for field_23e in field_23e_vec {
                    fields
                        .entry("23E".to_string())
                        .or_default()
                        .push(field_23e.to_swift_value());
                }
            }

            fields
                .entry("32B".to_string())
                .or_default()
                .push(transaction.field_32b.to_swift_value());

            // Handle all other transaction fields...
            // (Same pattern as to_ordered_fields but appending to existing vectors)
        }

        fields
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "28D", "30", "21", "32B", "59", "71A"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec![
            "21R", "50", "52", "51A", "25", "21F", "23E", "56", "57", "70", "77B", "33B", "25A",
            "36",
        ]
    }
}
