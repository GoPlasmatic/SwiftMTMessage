use crate::fields::*;
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT910: Confirmation of Credit
///
/// ## Overview
/// MT910 is used by a financial institution to confirm to another financial institution
/// that a credit has been made to the sender's account held with the receiver, or that
/// the sender's account held with a third party has been credited. This message serves
/// as official confirmation of credit transactions and facilitates reconciliation between
/// financial institutions.
///
/// ## Message Type Specification
/// **Message Type**: `910`  
/// **Category**: Cash Management and Customer Status (Category 9)  
/// **Usage**: Confirmation of Credit  
/// **Processing**: Account confirmation and reconciliation  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ## Message Structure
/// The MT910 message consists of mandatory and optional fields organized in a specific sequence:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 21**: Related Reference (reference to original transaction)
/// - **Field 25a**: Account Identification (account that was credited)
/// - **Field 32A**: Value Date/Currency/Amount (credit details)
///
/// ### Optional Fields (Enhanced Processing)
/// ```text
/// Field 13D   - Date/Time Indication (when credit occurred)
/// Field 50a   - Ordering Customer (customer who originated transaction)
/// Field 52a   - Ordering Institution (institution that ordered the credit)
/// Field 56a   - Intermediary (institution sender received funds from)
/// Field 72    - Sender to Receiver Information (additional details)
/// ```
///
/// ## Conditional Rules
/// - **C1**: Either Field 50a or Field 52a must be present (not both)
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Account reconciliation**: Confirming credit transactions between institutions
/// - **Correspondent banking**: Nostro/vostro account credit confirmations
/// - **Settlement confirmation**: Confirming settlement credits
/// - **Liquidity management**: Account balance change notifications
/// - **Audit trail**: Creating audit records for credit transactions
/// - **Exception handling**: Confirming problem resolution credits
///
/// ### Industry Sectors
/// - **Correspondent Banking**: Inter-bank account management
/// - **Central Banking**: Central bank and commercial bank confirmations
/// - **Clearing Houses**: Settlement confirmation processes
/// - **Commercial Banking**: Customer account credit confirmations
/// - **Investment Banking**: Securities settlement confirmations
/// - **Corporate Banking**: Large transaction confirmations
///
/// ## Usage Constraints and Guidelines
///
/// ### When NOT to Use MT910
/// - **🚫 Frequent statements**: Do not use if statements for the account are frequently transmitted
/// - **🚫 Routine confirmations**: Avoid for high-volume routine transactions
/// - **🚫 Real-time processing**: Not suitable for real-time transaction processing
///
/// ### When TO Use MT910
/// - **✅ Ad-hoc confirmations**: For specific, non-routine credit confirmations
/// - **✅ Large amounts**: For significant credit transactions requiring confirmation
/// - **✅ Exception cases**: For problem resolution or special handling
/// - **✅ Audit requirements**: When audit trail documentation is required
///
/// ## Field Specifications and Business Rules
///
/// ### Field 20 - Transaction Reference Number
/// - **Format**: `16x` (up to 16 alphanumeric characters)
/// - **Rule**: No leading/trailing slash, no '//' sequences
/// - **Purpose**: Unique identification for this confirmation message
///
/// ### Field 21 - Related Reference  
/// - **Format**: `16x` (up to 16 alphanumeric characters)
/// - **Rule**: Copy unchanged from original inward MT103/202
/// - **Purpose**: Links confirmation to original transaction
///
/// ### Field 25a - Account Identification
/// - **Option P**: `35x` (account number or BIC)
/// - **Purpose**: Identifies the account that was credited
/// - **Content**: May include account number or BIC code
///
/// ### Field 32A - Value Date, Currency, Amount
/// - **Format**: `6!n3!a15d` (date + currency + amount)
/// - **Amount format**: Uses comma as decimal separator (e.g., USD1234,56)
/// - **Purpose**: Specifies when, in what currency, and for what amount the credit occurred
///
/// ### Field 13D - Date/Time Indication (Optional)
/// - **Format**: `6!n4!n1!x4!n` (YYMMDDhhmm±hhmm)
/// - **Example**: `2403151430+0530` (March 15, 2024, 14:30 UTC+5:30)
/// - **Purpose**: Precise timing of when the credit was processed
///
/// ### Field 50a - Ordering Customer (Conditional C1)
/// - **Options**: A (BIC), F (Party ID+Name/Address), K (Name/Address)
/// - **Purpose**: Originator of transaction that triggered credit
/// - **Rule**: Either Field 50a OR Field 52a must be present, not both
///
/// ### Field 52a - Ordering Institution (Conditional C1)
/// - **Options**: A (BIC), D (Name/Address with clearing codes)
/// - **Purpose**: Financial institution of ordering customer
/// - **Rule**: Either Field 50a OR Field 52a must be present, not both
///
/// ### Field 56a - Intermediary (Optional)
/// - **Options**: A (BIC), D (Name/Address with clearing codes)
/// - **Purpose**: Financial institution sender received funds from
/// - **Usage**: For complex routing scenarios
///
/// ### Field 72 - Sender to Receiver Information (Optional)
/// - **Format**: `6*35x` (up to 6 lines of 35 characters each)
/// - **Content**: Narrative only, may use bilateral/ERI/EXCH codes
/// - **Restriction**: Must not include instructions - only information
///
/// ## Processing and Compliance
///
/// ### Network Validated Rules (SWIFT Standards)
/// - **T20**: Transaction reference format validation
/// - **T21**: Related reference format validation  
/// - **T25**: Account identification format validation
/// - **T32**: Value date, currency, and amount format validation
/// - **T13**: Date/time indication format validation (if present)
/// - **T50**: Ordering customer format validation (if present)
/// - **T52**: Ordering institution format validation (if present)
/// - **T56**: Intermediary format validation (if present)
/// - **T72**: Sender to receiver information format validation (if present)
///
/// ### Business Rule Validations
/// - Transaction reference should be unique per sender per business day
/// - Related reference should link to a valid original transaction
/// - Account identification must be valid and accessible
/// - Value date should be valid business day for the currency
/// - Amount must be positive (credits are positive amounts in MT910)
/// - Currency code must be valid ISO 4217
/// - Either Field 50a or Field 52a must be present (C1 rule)
///
/// ## Examples
/// ```text
/// Basic MT910 confirmation with ordering customer:
/// :20:TXN240315001234
/// :21:ORIG240314567890
/// :25P:GB33BUKB20201555555555
/// :32A:240315USD10000,00
/// :50K:ACME CORPORATION
/// NEW YORK NY US
///
/// MT910 with ordering institution and timing:
/// :20:TXN240315001235  
/// :21:ORIG240314567891
/// :25P:CH1234567890123456
/// :13D:2403151430+0100
/// :32A:240315EUR5000,00
/// :52A:DEUTDEFFXXX
/// :72:/EXCH/SPOT/1.0850
/// ```
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "910")]
pub struct MT910 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific credit confirmation.
    /// Used throughout the confirmation lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT910 messages  
    /// **Business Rule**: No leading/trailing slash, no '//' sequences  
    /// **Example**: "TXN240315001234"
    #[field("20")]
    pub field_20: Field20,

    /// **Related Reference** - Field 21
    ///
    /// Reference to the original transaction or message that resulted in this credit.
    /// Should be copied unchanged from the original inward MT103/202 that triggered
    /// this credit confirmation.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT910 messages  
    /// **Relationship**: Links to original transaction reference  
    /// **Rule**: Copy unchanged from original message
    /// **Example**: "ORIG240314567890"
    #[field("21")]
    pub field_21: Field21,

    /// **Account Identification** - Field 25a
    ///
    /// Identifies the specific account that has been credited. This account
    /// is typically held by the sender with the receiver, or with a third party
    /// as specified in the original transaction.
    ///
    /// **Option P**: Account number (up to 35 characters) or BIC  
    /// **Usage**: Mandatory in all MT910 messages  
    /// **Content**: May include IBAN, account number, or BIC code  
    /// **Example**: "GB33BUKB20201555555555" (IBAN format)
    #[field("25")]
    pub field_25: Field25,

    /// **Value Date, Currency, Amount** - Field 32A
    ///
    /// Core credit details specifying when the credit was effective, in what currency,
    /// and for what amount. The value date indicates when the credit actually
    /// took effect on the account.
    ///
    /// **Components**: Date (YYMMDD) + Currency (3 chars) + Amount (decimal)  
    /// **Business Rule**: Amount uses comma as decimal separator  
    /// **Settlement**: Indicates actual credit effective date  
    /// **Usage**: Mandatory in all MT910 messages
    #[field("32A")]
    pub field_32a: Field32A,

    /// **Date/Time Indication** - Field 13D (Optional)
    ///
    /// Provides precise timing information for when the credit was processed,
    /// including UTC offset for accurate time coordination across time zones.
    ///
    /// **Format**: YYMMDDhhmm±hhmm (date + time + UTC offset)  
    /// **Usage**: Optional, used for precise timing requirements  
    /// **Example**: "2403151430+0530" (March 15, 2024, 14:30 UTC+5:30)  
    /// **Business Value**: Enables precise audit trail and timing coordination
    #[field("13D")]
    pub field_13d: Option<Field13D>,

    /// **Ordering Customer** - Field 50a (Conditional C1)
    ///
    /// Identifies the customer who originated the transaction that resulted in this credit.
    /// This field provides customer-level traceability for the credit transaction.
    ///
    /// **Options**: A (BIC), F (Party ID+Name/Address), K (Name/Address)  
    /// **Usage**: Conditional C1 - Either Field 50a OR Field 52a must be present  
    /// **Purpose**: Originator of transaction that triggered credit  
    /// **Traceability**: Links credit to ultimate ordering customer
    #[field("50")]
    pub field_50a: Option<Field50>,

    /// **Ordering Institution** - Field 52a (Conditional C1)
    ///
    /// Identifies the financial institution of the ordering customer or the institution
    /// that ordered the transaction resulting in this credit. Alternative to Field 50a
    /// when institutional-level identification is more appropriate.
    ///
    /// **Options**: A (BIC), D (Name/Address with clearing codes)  
    /// **Usage**: Conditional C1 - Either Field 50a OR Field 52a must be present  
    /// **Purpose**: Financial institution of ordering customer  
    /// **Clearing Codes**: May include codes prefixed with "//" for domestic routing
    #[field("52")]
    pub field_52a: Option<GenericBicField>,

    /// **Intermediary** - Field 56a (Optional)
    ///
    /// Identifies the financial institution from which the sender received the funds
    /// that resulted in this credit. Used to document the routing chain and source
    /// of funds for audit and reconciliation purposes.
    ///
    /// **Options**: A (BIC), D (Name/Address with clearing codes)  
    /// **Usage**: Optional, provides routing context  
    /// **Purpose**: Financial institution sender received funds from  
    /// **Routing**: Documents fund flow for complex routing scenarios
    #[field("56")]
    pub field_56a: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    ///
    /// Free-format field for additional information about the credit transaction.
    /// Must contain narrative information only and may include structured codes
    /// for bilateral use or exchange rate information.
    ///
    /// **Format**: Up to 6 lines of 35 characters each  
    /// **Usage**: Optional, provides additional context  
    /// **Content**: Narrative only, may use bilateral/ERI/EXCH codes  
    /// **Restriction**: Must not include instructions - only information
    #[field("72")]
    pub field_72: Option<Field72>,
}

impl MT910 {
    /// Creates a new MT910 with minimal required fields and ordering customer
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_21` - Related reference
    /// * `field_25` - Account identification
    /// * `field_32a` - Value date, currency, and amount
    /// * `field_50a` - Ordering customer
    ///
    /// # Returns
    /// A new MT910 instance with ordering customer (satisfies C1 rule)
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::messages::MT910;
    /// # use swift_mt_message::fields::*;
    /// # use chrono::NaiveDate;
    /// let field_20 = Field20::new("TXN240315001234".to_string());
    /// let field_21 = Field21::new("ORIG240314567890".to_string());
    /// let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
    /// let field_32a = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     10000.00
    /// );
    /// let field_50a = Field50::K(Field50K::new(vec!["ACME CORP".to_string()]).unwrap());
    ///
    /// let mt910 = MT910::new_with_customer(field_20, field_21, field_25, field_32a, field_50a);
    /// ```
    pub fn new_with_customer(
        field_20: Field20,
        field_21: Field21,
        field_25: Field25,
        field_32a: Field32A,
        field_50a: Field50,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_25,
            field_32a,
            field_13d: None,
            field_50a: Some(field_50a),
            field_52a: None,
            field_56a: None,
            field_72: None,
        }
    }

    /// Creates a new MT910 with minimal required fields and ordering institution
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_21` - Related reference
    /// * `field_25` - Account identification
    /// * `field_32a` - Value date, currency, and amount
    /// * `field_52a` - Ordering institution
    ///
    /// # Returns
    /// A new MT910 instance with ordering institution (satisfies C1 rule)
    pub fn new_with_institution(
        field_20: Field20,
        field_21: Field21,
        field_25: Field25,
        field_32a: Field32A,
        field_52a: GenericBicField,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_25,
            field_32a,
            field_13d: None,
            field_50a: None,
            field_52a: Some(field_52a),
            field_56a: None,
            field_72: None,
        }
    }

    /// Creates a new MT910 with all fields
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_21` - Related reference
    /// * `field_25` - Account identification
    /// * `field_32a` - Value date, currency, and amount
    /// * `field_13d` - Date/time indication (optional)
    /// * `field_50a` - Ordering customer (conditional C1)
    /// * `field_52a` - Ordering institution (conditional C1)
    /// * `field_56a` - Intermediary (optional)
    /// * `field_72` - Sender to receiver information (optional)
    ///
    /// # Returns
    /// A new MT910 instance with all fields populated
    ///
    /// # Note
    /// Caller must ensure C1 rule compliance (either field_50a or field_52a, not both)
    pub fn new_complete(
        field_20: Field20,
        field_21: Field21,
        field_25: Field25,
        field_32a: Field32A,
        field_13d: Option<Field13D>,
        field_50a: Option<Field50>,
        field_52a: Option<GenericBicField>,
        field_56a: Option<GenericBicField>,
        field_72: Option<Field72>,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_25,
            field_32a,
            field_13d,
            field_50a,
            field_52a,
            field_56a,
            field_72,
        }
    }

    // Accessor methods for key fields

    /// Returns the transaction reference number
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Returns the related reference
    pub fn related_reference(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Returns the account identification
    pub fn account_identification(&self) -> &str {
        self.field_25.authorisation()
    }

    /// Returns the value date as NaiveDate
    pub fn value_date(&self) -> chrono::NaiveDate {
        self.field_32a.value_date
    }

    /// Returns the currency code
    pub fn currency_code(&self) -> &str {
        self.field_32a.currency_code()
    }

    /// Returns the credit amount as decimal
    pub fn credit_amount(&self) -> f64 {
        self.field_32a.amount_decimal()
    }

    /// Returns the date/time indication if present
    pub fn date_time_indication(&self) -> Option<&Field13D> {
        self.field_13d.as_ref()
    }

    /// Returns the ordering customer if present
    pub fn ordering_customer(&self) -> Option<&Field50> {
        self.field_50a.as_ref()
    }

    /// Returns the ordering institution BIC if present
    pub fn ordering_institution_bic(&self) -> Option<&str> {
        self.field_52a.as_ref().map(|f| f.bic())
    }

    /// Returns the intermediary BIC if present
    pub fn intermediary_bic(&self) -> Option<&str> {
        self.field_56a.as_ref().map(|f| f.bic())
    }

    /// Returns the sender to receiver information if present
    pub fn sender_to_receiver_info(&self) -> Option<&Field72> {
        self.field_72.as_ref()
    }

    // Business logic methods

    /// Checks if this confirmation includes precise timing information
    ///
    /// # Returns
    /// `true` if Field 13D is present, indicating precise timing
    pub fn has_precise_timing(&self) -> bool {
        self.field_13d.is_some()
    }

    /// Checks if this confirmation identifies the ordering customer
    ///
    /// # Returns
    /// `true` if Field 50a is present, indicating customer-level identification
    pub fn has_ordering_customer(&self) -> bool {
        self.field_50a.is_some()
    }

    /// Checks if this confirmation identifies the ordering institution
    ///
    /// # Returns
    /// `true` if Field 52a is present, indicating institutional-level identification
    pub fn has_ordering_institution(&self) -> bool {
        self.field_52a.is_some()
    }

    /// Checks if this confirmation includes intermediary information
    ///
    /// # Returns
    /// `true` if Field 56a is present, indicating intermediary involvement
    pub fn has_intermediary(&self) -> bool {
        self.field_56a.is_some()
    }

    /// Checks if this confirmation includes additional information
    ///
    /// # Returns
    /// `true` if Field 72 is present, indicating additional details
    pub fn has_additional_information(&self) -> bool {
        self.field_72.is_some()
    }

    /// Gets the formatted date/time if precise timing is available
    ///
    /// # Returns
    /// Formatted date/time string or None if not available
    pub fn get_formatted_datetime(&self) -> Option<String> {
        self.field_13d.as_ref().map(|f| f.get_formatted_datetime())
    }

    /// Validates the C1 conditional rule
    ///
    /// # Returns
    /// `true` if exactly one of Field 50a or Field 52a is present, `false` otherwise
    pub fn validate_rule_c1(&self) -> bool {
        match (&self.field_50a, &self.field_52a) {
            (Some(_), None) => true, // Field 50a present, Field 52a absent
            (None, Some(_)) => true, // Field 52a present, Field 50a absent
            _ => false,              // Both present or both absent
        }
    }

    /// Checks if this is a back-dated credit confirmation
    ///
    /// # Returns
    /// `true` if the value date is in the past
    pub fn is_back_dated_credit(&self) -> bool {
        self.field_32a.is_back_dated()
    }

    /// Returns a formatted description of the credit confirmation
    ///
    /// # Returns
    /// A human-readable description of the MT910 confirmation
    pub fn get_confirmation_description(&self) -> String {
        let timing = if self.has_precise_timing() {
            " with precise timing"
        } else {
            ""
        };

        let originator = if self.has_ordering_customer() {
            " from customer"
        } else if self.has_ordering_institution() {
            " from institution"
        } else {
            ""
        };

        let intermediary = if self.has_intermediary() {
            " via intermediary"
        } else {
            ""
        };

        format!(
            "Credit confirmation for {} {:.2} on {}{}{}{}",
            self.currency_code(),
            self.credit_amount(),
            self.value_date().format("%Y-%m-%d"),
            timing,
            originator,
            intermediary
        )
    }

    /// Gets processing instructions from field 72 if present
    ///
    /// # Returns
    /// Vector of processing instruction lines
    pub fn get_processing_instructions(&self) -> Vec<String> {
        if let Some(field_72) = &self.field_72 {
            field_72.information.clone()
        } else {
            Vec::new()
        }
    }

    /// Checks if the confirmation contains exchange rate information
    ///
    /// # Returns
    /// `true` if Field 72 contains exchange rate codes
    pub fn has_exchange_rate_info(&self) -> bool {
        if let Some(field_72) = &self.field_72 {
            field_72.information.iter().any(|line| {
                line.contains("/EXCH/") || line.contains("/RATE/") || line.contains("/FX/")
            })
        } else {
            false
        }
    }

    /// Checks if the confirmation contains ERI (Exchange Rate Information) codes
    ///
    /// # Returns
    /// `true` if Field 72 contains ERI codes
    pub fn has_eri_codes(&self) -> bool {
        if let Some(field_72) = &self.field_72 {
            field_72
                .information
                .iter()
                .any(|line| line.contains("/ERI/"))
        } else {
            false
        }
    }

    /// Validates the overall message structure and business rules
    ///
    /// # Returns
    /// `true` if all validation rules pass, `false` otherwise
    pub fn validate_structure(&self) -> bool {
        // Basic structural validation
        let basic_valid = !self.transaction_reference().is_empty()
            && !self.related_reference().is_empty()
            && !self.account_identification().is_empty()
            && self.credit_amount() > 0.0
            && !self.currency_code().is_empty();

        // C1 rule validation
        let c1_valid = self.validate_rule_c1();

        // Additional business rule validation
        let business_valid = !self.transaction_reference().starts_with('/')
            && !self.transaction_reference().ends_with('/')
            && !self.transaction_reference().contains("//")
            && !self.related_reference().starts_with('/')
            && !self.related_reference().ends_with('/')
            && !self.related_reference().contains("//");

        basic_valid && c1_valid && business_valid
    }

    /// Checks if this confirmation is for a high-value transaction
    ///
    /// # Arguments
    /// * `threshold` - The amount threshold to consider "high-value"
    ///
    /// # Returns
    /// `true` if the credit amount exceeds the threshold
    pub fn is_high_value_transaction(&self, threshold: f64) -> bool {
        self.credit_amount() > threshold
    }

    /// Gets a summary of all optional information present
    ///
    /// # Returns
    /// Vector of strings describing what optional information is present
    pub fn get_optional_information_summary(&self) -> Vec<String> {
        let mut summary = Vec::new();

        if self.has_precise_timing() {
            if let Some(datetime) = self.get_formatted_datetime() {
                summary.push(format!("Precise timing: {}", datetime));
            }
        }

        if self.has_ordering_customer() {
            summary.push("Ordering customer identified".to_string());
        } else if self.has_ordering_institution() {
            if let Some(bic) = self.ordering_institution_bic() {
                summary.push(format!("Ordering institution: {}", bic));
            }
        }

        if self.has_intermediary() {
            if let Some(bic) = self.intermediary_bic() {
                summary.push(format!("Intermediary institution: {}", bic));
            }
        }

        if self.has_additional_information() {
            summary.push("Additional information provided".to_string());

            if self.has_exchange_rate_info() {
                summary.push("Contains exchange rate information".to_string());
            }

            if self.has_eri_codes() {
                summary.push("Contains ERI codes".to_string());
            }
        }

        summary
    }

    /// Gets the originator type description
    ///
    /// # Returns
    /// String describing the type of originator identification
    pub fn get_originator_type(&self) -> &'static str {
        if self.has_ordering_customer() {
            "Customer"
        } else if self.has_ordering_institution() {
            "Institution"
        } else {
            "Unknown (validation error)"
        }
    }

    /// Gets the complete routing chain information
    ///
    /// # Returns
    /// Vector of tuples containing (role, identifier) for all routing parties
    pub fn get_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        if let Some(customer) = &self.field_50a {
            match customer {
                Field50::A(bic_field) => {
                    chain.push(("Ordering Customer (BIC)", bic_field.bic().to_string()));
                }
                Field50::F(_) => {
                    chain.push((
                        "Ordering Customer (Party ID)",
                        "Party ID+Name/Address".to_string(),
                    ));
                }
                Field50::K(_) => {
                    chain.push(("Ordering Customer (Name)", "Name/Address".to_string()));
                }
            }
        }

        if let Some(institution) = &self.field_52a {
            chain.push(("Ordering Institution", institution.bic().to_string()));
        }

        if let Some(intermediary) = &self.field_56a {
            chain.push(("Intermediary", intermediary.bic().to_string()));
        }

        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;
    use chrono::NaiveDate;

    #[test]
    fn test_mt910_creation_with_customer() {
        let field_20 = Field20::new("TXN240315001234".to_string());
        let field_21 = Field21::new("ORIG240314567890".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            10000.00,
        );
        let field_50a = Field50::K(Field50K::new(vec!["ACME CORPORATION".to_string()]).unwrap());

        let mt910 = MT910::new_with_customer(field_20, field_21, field_25, field_32a, field_50a);

        assert_eq!(mt910.transaction_reference(), "TXN240315001234");
        assert_eq!(mt910.related_reference(), "ORIG240314567890");
        assert_eq!(mt910.account_identification(), "GB33BUKB20201555555555");
        assert_eq!(mt910.currency_code(), "USD");
        assert_eq!(mt910.credit_amount(), 10000.00);
        assert_eq!(
            mt910.value_date(),
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()
        );
        assert!(mt910.has_ordering_customer());
        assert!(!mt910.has_ordering_institution());
        assert!(mt910.validate_rule_c1());
    }

    #[test]
    fn test_mt910_creation_with_institution() {
        let field_20 = Field20::new("TXN240315001235".to_string());
        let field_21 = Field21::new("ORIG240314567891".to_string());
        let field_25 = Field25::new("CH1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "EUR".to_string(),
            5000.00,
        );
        let field_52a = GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap();

        let mt910 = MT910::new_with_institution(field_20, field_21, field_25, field_32a, field_52a);

        assert_eq!(mt910.transaction_reference(), "TXN240315001235");
        assert_eq!(mt910.related_reference(), "ORIG240314567891");
        assert_eq!(mt910.ordering_institution_bic(), Some("DEUTDEFFXXX"));
        assert!(!mt910.has_ordering_customer());
        assert!(mt910.has_ordering_institution());
        assert!(mt910.validate_rule_c1());
    }

    #[test]
    fn test_mt910_message_type() {
        assert_eq!(MT910::message_type(), "910");
    }

    #[test]
    fn test_mt910_with_all_optional_fields() {
        let field_20 = Field20::new("TXN240315001236".to_string());
        let field_21 = Field21::new("ORIG240314567892".to_string());
        let field_25 = Field25::new("US1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            25000.00,
        );

        let field_13d = Some(Field13D::new("240315", "1430", "+", "0500").unwrap());
        let field_52a = Some(GenericBicField::new(None, None, "CHASUS33XXX").unwrap());
        let field_56a = Some(GenericBicField::new(None, None, "CITIUS33XXX").unwrap());
        let field_72 = Some(
            Field72::new(vec![
                "/ERI/BILATERAL001".to_string(),
                "/EXCH/SPOT/1.0850".to_string(),
            ])
            .unwrap(),
        );

        let mt910 = MT910::new_complete(
            field_20, field_21, field_25, field_32a, field_13d, None, field_52a, field_56a,
            field_72,
        );

        assert!(mt910.has_precise_timing());
        assert!(mt910.has_ordering_institution());
        assert!(mt910.has_intermediary());
        assert!(mt910.has_additional_information());
        assert!(mt910.has_eri_codes());
        assert!(mt910.has_exchange_rate_info());
        assert_eq!(mt910.ordering_institution_bic(), Some("CHASUS33XXX"));
        assert_eq!(mt910.intermediary_bic(), Some("CITIUS33XXX"));
    }

    #[test]
    fn test_mt910_c1_rule_validation() {
        let field_20 = Field20::new("TXN240315001237".to_string());
        let field_21 = Field21::new("ORIG240314567893".to_string());
        let field_25 = Field25::new("DE1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "EUR".to_string(),
            15000.00,
        );

        // Test with both fields present (should fail C1)
        let field_50a = Some(Field50::K(
            Field50K::new(vec!["CUSTOMER".to_string()]).unwrap(),
        ));
        let field_52a = Some(GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap());
        let mt910_both = MT910::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_25.clone(),
            field_32a.clone(),
            None,
            field_50a,
            field_52a,
            None,
            None,
        );
        assert!(!mt910_both.validate_rule_c1());
        assert!(!mt910_both.validate_structure());

        // Test with neither field present (should fail C1)
        let mt910_neither = MT910::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_25.clone(),
            field_32a.clone(),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(!mt910_neither.validate_rule_c1());
        assert!(!mt910_neither.validate_structure());

        // Test with only customer (should pass C1)
        let field_50a = Some(Field50::K(
            Field50K::new(vec!["CUSTOMER".to_string()]).unwrap(),
        ));
        let mt910_customer = MT910::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_25.clone(),
            field_32a.clone(),
            None,
            field_50a,
            None,
            None,
            None,
        );
        assert!(mt910_customer.validate_rule_c1());
        assert!(mt910_customer.validate_structure());

        // Test with only institution (should pass C1)
        let field_52a = Some(GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap());
        let mt910_institution = MT910::new_complete(
            field_20, field_21, field_25, field_32a, None, None, field_52a, None, None,
        );
        assert!(mt910_institution.validate_rule_c1());
        assert!(mt910_institution.validate_structure());
    }

    #[test]
    fn test_mt910_business_logic() {
        let field_20 = Field20::new("TXN240315001238".to_string());
        let field_21 = Field21::new("ORIG240314567894".to_string());
        let field_25 = Field25::new("JP1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            2500000.00,
        );
        let field_50a = Field50::K(Field50K::new(vec!["BIG CORPORATION".to_string()]).unwrap());

        let mt910 = MT910::new_with_customer(field_20, field_21, field_25, field_32a, field_50a);

        // Test high-value transaction detection
        assert!(mt910.is_high_value_transaction(1000000.00));
        assert!(!mt910.is_high_value_transaction(5000000.00));

        // Test description generation
        let description = mt910.get_confirmation_description();
        assert!(description.contains("Credit confirmation"));
        assert!(description.contains("USD"));
        assert!(description.contains("2500000.00"));
        assert!(description.contains("2024-03-15"));
        assert!(description.contains("from customer"));

        // Test originator type
        assert_eq!(mt910.get_originator_type(), "Customer");
    }

    #[test]
    fn test_mt910_routing_chain() {
        let field_20 = Field20::new("TXN240315001239".to_string());
        let field_21 = Field21::new("ORIG240314567895".to_string());
        let field_25 = Field25::new("GB1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "GBP".to_string(),
            50000.00,
        );

        let field_52a = Some(GenericBicField::new(None, None, "BARCGB22XXX").unwrap());
        let field_56a = Some(GenericBicField::new(None, None, "CITIUS33XXX").unwrap());

        let mt910 = MT910::new_complete(
            field_20, field_21, field_25, field_32a, None, None, field_52a, field_56a, None,
        );

        let routing_chain = mt910.get_routing_chain();
        assert_eq!(routing_chain.len(), 2);
        assert_eq!(routing_chain[0].0, "Ordering Institution");
        assert_eq!(routing_chain[0].1, "BARCGB22XXX");
        assert_eq!(routing_chain[1].0, "Intermediary");
        assert_eq!(routing_chain[1].1, "CITIUS33XXX");
    }

    #[test]
    fn test_mt910_optional_information_summary() {
        let field_20 = Field20::new("TXN240315001240".to_string());
        let field_21 = Field21::new("ORIG240314567896".to_string());
        let field_25 = Field25::new("CA1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "CAD".to_string(),
            75000.00,
        );

        let field_13d = Some(Field13D::new("240315", "0930", "-", "0500").unwrap());
        let field_52a = Some(GenericBicField::new(None, None, "TDOMCATTXXX").unwrap());
        let field_72 = Some(
            Field72::new(vec![
                "/ERI/MULTILATERAL".to_string(),
                "LARGE VALUE TRANSFER".to_string(),
            ])
            .unwrap(),
        );

        let mt910 = MT910::new_complete(
            field_20, field_21, field_25, field_32a, field_13d, None, field_52a, None, field_72,
        );

        let summary = mt910.get_optional_information_summary();
        assert!(summary.len() >= 3); // Should have timing, institution, and additional info
        assert!(summary.iter().any(|s| s.contains("Precise timing")));
        assert!(summary.iter().any(|s| s.contains("Ordering institution")));
        assert!(summary.iter().any(|s| s.contains("Additional information")));
        assert!(summary.iter().any(|s| s.contains("ERI codes")));
    }

    #[test]
    fn test_mt910_date_time_formatting() {
        let field_20 = Field20::new("TXN240315001241".to_string());
        let field_21 = Field21::new("ORIG240314567897".to_string());
        let field_25 = Field25::new("AU1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "AUD".to_string(),
            100000.00,
        );

        let field_13d = Some(Field13D::new("240315", "1645", "+", "1000").unwrap());
        let field_50a = Field50::K(Field50K::new(vec!["AUSSIE CORP".to_string()]).unwrap());

        let mt910 = MT910::new_complete(
            field_20,
            field_21,
            field_25,
            field_32a,
            field_13d,
            Some(field_50a),
            None,
            None,
            None,
        );

        let formatted_datetime = mt910.get_formatted_datetime();
        assert!(formatted_datetime.is_some());
        let datetime_str = formatted_datetime.unwrap();
        assert!(datetime_str.contains("24/03/15"));
        assert!(datetime_str.contains("16:45"));
        assert!(datetime_str.contains("+10:00"));
    }
}
