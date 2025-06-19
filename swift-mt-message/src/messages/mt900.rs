use crate::fields::*;
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT900: Confirmation of Debit
///
/// ## Overview
/// MT900 is used by a financial institution to confirm to another financial institution
/// that a debit has been made to the sender's account held with the receiver, or that
/// the sender's account held with a third party has been debited. This message serves
/// as official confirmation of debit transactions and facilitates reconciliation between
/// financial institutions.
///
/// ## Message Type Specification
/// **Message Type**: `900`  
/// **Category**: Cash Management and Customer Status (Category 9)  
/// **Usage**: Confirmation of Debit  
/// **Processing**: Account confirmation and reconciliation  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ## Message Structure
/// The MT900 message consists of mandatory and optional fields organized in a specific sequence:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 21**: Related Reference (reference to original transaction)
/// - **Field 25a**: Account Identification (account that was debited)
/// - **Field 32A**: Value Date/Currency/Amount (debit details)
///
/// ### Optional Fields (Enhanced Processing)
/// ```text
/// Field 13D   - Date/Time Indication (when debit occurred)
/// Field 52a   - Ordering Institution (institution that ordered the debit)
/// Field 72    - Sender to Receiver Information (additional details)
/// ```
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Account reconciliation**: Confirming debit transactions between institutions
/// - **Correspondent banking**: Nostro/vostro account debit confirmations
/// - **Settlement confirmation**: Confirming settlement debits
/// - **Liquidity management**: Account balance change notifications
/// - **Audit trail**: Creating audit records for debit transactions
/// - **Exception handling**: Confirming problem resolution debits
///
/// ### Industry Sectors
/// - **Correspondent Banking**: Inter-bank account management
/// - **Central Banking**: Central bank and commercial bank confirmations
/// - **Clearing Houses**: Settlement confirmation processes
/// - **Commercial Banking**: Customer account debit confirmations
/// - **Investment Banking**: Securities settlement confirmations
/// - **Corporate Banking**: Large transaction confirmations
///
/// ## Usage Constraints and Guidelines
///
/// ### When NOT to Use MT900
/// - **🚫 Frequent statements**: Do not use if statements for the account are frequently transmitted
/// - **🚫 Routine confirmations**: Avoid for high-volume routine transactions
/// - **🚫 Real-time processing**: Not suitable for real-time transaction processing
///
/// ### When TO Use MT900
/// - **✅ Ad-hoc confirmations**: For specific, non-routine debit confirmations
/// - **✅ Large amounts**: For significant debit transactions requiring confirmation
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
/// - **Rule**: Refers to original transaction that triggered this debit
/// - **Purpose**: Links confirmation to original transaction
///
/// ### Field 25a - Account Identification
/// - **Option P**: `35x` (account number or BIC)
/// - **Purpose**: Identifies the account that was debited
/// - **Content**: May include account number or BIC code
///
/// ### Field 32A - Value Date, Currency, Amount
/// - **Format**: `6!n3!a15d` (date + currency + amount)
/// - **Amount format**: Uses comma as decimal separator (e.g., USD1234,56)
/// - **Purpose**: Specifies when, in what currency, and for what amount the debit occurred
///
/// ### Field 13D - Date/Time Indication (Optional)
/// - **Format**: `6!n4!n1!x4!n` (YYMMDDhhmm±hhmm)
/// - **Example**: `2403151430+0530` (March 15, 2024, 14:30 UTC+5:30)
/// - **Purpose**: Precise timing of when the debit was processed
///
/// ### Field 52a - Ordering Institution (Optional)
/// - **Option A**: BIC code format
/// - **Option D**: Name and address with optional clearing codes
/// - **Purpose**: Institution that ordered the debit transaction
///
/// ### Field 72 - Sender to Receiver Information (Optional)
/// - **Format**: `6*35x` (up to 6 lines of 35 characters each)
/// - **Content**: Structured or narrative information
/// - **Codes**: May include `/EXCH/`, `/ERI/`, or bilateral codes prefixed by `//`
/// - **Restriction**: Should not contain booking instructions
///
/// ## Processing and Compliance
///
/// ### Network Validated Rules (SWIFT Standards)
/// - **T20**: Transaction reference format validation
/// - **T21**: Related reference format validation  
/// - **T25**: Account identification format validation
/// - **T32**: Value date, currency, and amount format validation
/// - **T13**: Date/time indication format validation (if present)
/// - **T52**: Ordering institution format validation (if present)
/// - **T72**: Sender to receiver information format validation (if present)
///
/// ### Business Rule Validations
/// - Transaction reference should be unique per sender per business day
/// - Related reference should link to a valid original transaction
/// - Account identification must be valid and accessible
/// - Value date should be valid business day for the currency
/// - Amount must be positive (debits are positive amounts in MT900)
/// - Currency code must be valid ISO 4217
///
/// ## Examples
/// ```text
/// Basic MT900 confirmation:
/// :20:TXN240315001234
/// :21:ORIG240314567890
/// :25P:GB33BUKB20201555555555
/// :32A:240315USD10000,00
///
/// MT900 with timing and ordering institution:
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
#[swift_message(mt = "900")]
pub struct MT900 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific debit confirmation.
    /// Used throughout the confirmation lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT900 messages  
    /// **Business Rule**: No leading/trailing slash, no '//' sequences  
    /// **Example**: "TXN240315001234"
    #[field("20")]
    pub field_20: Field20,

    /// **Related Reference** - Field 21
    ///
    /// Reference to the original transaction or message that resulted in this debit.
    /// Critical for linking the confirmation back to the initiating transaction
    /// and maintaining complete audit trails.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT900 messages  
    /// **Relationship**: Links to original transaction reference  
    /// **Example**: "ORIG240314567890"
    #[field("21")]
    pub field_21: Field21,

    /// **Account Identification** - Field 25a
    ///
    /// Identifies the specific account that has been debited. This account
    /// is typically held by the sender with the receiver, or with a third party
    /// as specified in the original transaction.
    ///
    /// **Option P**: Account number (up to 35 characters) or BIC  
    /// **Usage**: Mandatory in all MT900 messages  
    /// **Content**: May include IBAN, account number, or BIC code  
    /// **Example**: "GB33BUKB20201555555555" (IBAN format)
    #[field("25")]
    pub field_25: Field25,

    /// **Value Date, Currency, Amount** - Field 32A
    ///
    /// Core debit details specifying when the debit was effective, in what currency,
    /// and for what amount. The value date indicates when the debit actually
    /// took effect on the account.
    ///
    /// **Components**: Date (YYMMDD) + Currency (3 chars) + Amount (decimal)  
    /// **Business Rule**: Amount uses comma as decimal separator  
    /// **Settlement**: Indicates actual debit effective date  
    /// **Usage**: Mandatory in all MT900 messages
    #[field("32A")]
    pub field_32a: Field32A,

    /// **Date/Time Indication** - Field 13D (Optional)
    ///
    /// Provides precise timing information for when the debit was processed,
    /// including UTC offset for accurate time coordination across time zones.
    ///
    /// **Format**: YYMMDDhhmm±hhmm (date + time + UTC offset)  
    /// **Usage**: Optional, used for precise timing requirements  
    /// **Example**: "2403151430+0530" (March 15, 2024, 14:30 UTC+5:30)  
    /// **Business Value**: Enables precise audit trail and timing coordination
    #[field("13D")]
    pub field_13d: Option<Field13D>,

    /// **Ordering Institution** - Field 52a (Optional)
    ///
    /// Identifies the financial institution that ordered or initiated the
    /// transaction that resulted in this debit. May include additional
    /// clearing or routing information.
    ///
    /// **Option A**: BIC code format  
    /// **Option D**: Name and address with optional national clearing codes  
    /// **Usage**: Optional, provides transaction origination context  
    /// **Clearing Codes**: May include codes prefixed with "//" for domestic routing
    #[field("52")]
    pub field_52a: Option<GenericBicField>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    ///
    /// Free-format field for additional information about the debit transaction.
    /// May contain structured codes, exchange rate information, or narrative
    /// details relevant to the debit confirmation.
    ///
    /// **Format**: Up to 6 lines of 35 characters each  
    /// **Usage**: Optional, provides additional context  
    /// **Content**: Structured codes (/EXCH/, /ERI/) or narrative information  
    /// **Restriction**: Must not contain booking instructions
    #[field("72")]
    pub field_72: Option<Field72>,
}

impl MT900 {
    /// Creates a new MT900 with minimal required fields
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_21` - Related reference
    /// * `field_25` - Account identification
    /// * `field_32a` - Value date, currency, and amount
    ///
    /// # Returns
    /// A new MT900 instance with only mandatory fields populated
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::messages::MT900;
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
    ///
    /// let mt900 = MT900::new(field_20, field_21, field_25, field_32a);
    /// ```
    pub fn new(
        field_20: Field20,
        field_21: Field21,
        field_25: Field25,
        field_32a: Field32A,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_25,
            field_32a,
            field_13d: None,
            field_52a: None,
            field_72: None,
        }
    }

    /// Creates a new MT900 with all fields
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_21` - Related reference
    /// * `field_25` - Account identification
    /// * `field_32a` - Value date, currency, and amount
    /// * `field_13d` - Date/time indication (optional)
    /// * `field_52a` - Ordering institution (optional)
    /// * `field_72` - Sender to receiver information (optional)
    ///
    /// # Returns
    /// A new MT900 instance with all fields populated
    pub fn new_complete(
        field_20: Field20,
        field_21: Field21,
        field_25: Field25,
        field_32a: Field32A,
        field_13d: Option<Field13D>,
        field_52a: Option<GenericBicField>,
        field_72: Option<Field72>,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_25,
            field_32a,
            field_13d,
            field_52a,
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

    /// Returns the debit amount as decimal
    pub fn debit_amount(&self) -> f64 {
        self.field_32a.amount_decimal()
    }

    /// Returns the date/time indication if present
    pub fn date_time_indication(&self) -> Option<&Field13D> {
        self.field_13d.as_ref()
    }

    /// Returns the ordering institution BIC if present
    pub fn ordering_institution_bic(&self) -> Option<&str> {
        self.field_52a.as_ref().map(|f| f.bic())
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

    /// Checks if this confirmation identifies the ordering institution
    ///
    /// # Returns
    /// `true` if Field 52a is present, indicating known ordering institution
    pub fn has_ordering_institution(&self) -> bool {
        self.field_52a.is_some()
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

    /// Checks if this is a back-dated debit confirmation
    ///
    /// # Returns
    /// `true` if the value date is in the past
    pub fn is_back_dated_debit(&self) -> bool {
        self.field_32a.is_back_dated()
    }

    /// Returns a formatted description of the debit confirmation
    ///
    /// # Returns
    /// A human-readable description of the MT900 confirmation
    pub fn get_confirmation_description(&self) -> String {
        let timing = if self.has_precise_timing() {
            " with precise timing"
        } else {
            ""
        };

        let institution = if self.has_ordering_institution() {
            " from identified institution"
        } else {
            ""
        };

        format!(
            "Debit confirmation for {} {:.2} on {}{}{}",
            self.currency_code(),
            self.debit_amount(),
            self.value_date().format("%Y-%m-%d"),
            timing,
            institution
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
            && self.debit_amount() > 0.0
            && !self.currency_code().is_empty();

        // Additional business rule validation
        let business_valid = !self.transaction_reference().starts_with('/')
            && !self.transaction_reference().ends_with('/')
            && !self.transaction_reference().contains("//")
            && !self.related_reference().starts_with('/')
            && !self.related_reference().ends_with('/')
            && !self.related_reference().contains("//");

        basic_valid && business_valid
    }

    /// Checks if this confirmation is for a high-value transaction
    ///
    /// # Arguments
    /// * `threshold` - The amount threshold to consider "high-value"
    ///
    /// # Returns
    /// `true` if the debit amount exceeds the threshold
    pub fn is_high_value_transaction(&self, threshold: f64) -> bool {
        self.debit_amount() > threshold
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

        if self.has_ordering_institution() {
            if let Some(bic) = self.ordering_institution_bic() {
                summary.push(format!("Ordering institution: {}", bic));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;
    use chrono::NaiveDate;

    #[test]
    fn test_mt900_creation() {
        let field_20 = Field20::new("TXN240315001234".to_string());
        let field_21 = Field21::new("ORIG240314567890".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            10000.00,
        );

        let mt900 = MT900::new(field_20, field_21, field_25, field_32a);

        assert_eq!(mt900.transaction_reference(), "TXN240315001234");
        assert_eq!(mt900.related_reference(), "ORIG240314567890");
        assert_eq!(mt900.account_identification(), "GB33BUKB20201555555555");
        assert_eq!(mt900.currency_code(), "USD");
        assert_eq!(mt900.debit_amount(), 10000.00);
        assert_eq!(
            mt900.value_date(),
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()
        );
    }

    #[test]
    fn test_mt900_message_type() {
        assert_eq!(MT900::message_type(), "900");
    }

    #[test]
    fn test_mt900_with_optional_fields() {
        let field_20 = Field20::new("TXN240315001235".to_string());
        let field_21 = Field21::new("ORIG240314567891".to_string());
        let field_25 = Field25::new("CH1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "EUR".to_string(),
            5000.00,
        );

        // Add optional fields
        let field_13d = Some(Field13D::new("240315", "1430", "+", "0100").unwrap());
        let field_52a = Some(GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap());
        let field_72 = Some(Field72::new(vec!["/EXCH/SPOT/1.0850".to_string()]).unwrap());

        let mt900 = MT900::new_complete(
            field_20, field_21, field_25, field_32a, field_13d, field_52a, field_72,
        );

        assert!(mt900.has_precise_timing());
        assert!(mt900.has_ordering_institution());
        assert!(mt900.has_additional_information());
        assert!(mt900.has_exchange_rate_info());
        assert_eq!(mt900.ordering_institution_bic(), Some("DEUTDEFFXXX"));
    }

    #[test]
    fn test_mt900_validation() {
        let field_20 = Field20::new("TXN240315001234".to_string());
        let field_21 = Field21::new("ORIG240314567890".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            10000.00,
        );

        let mt900 = MT900::new(field_20, field_21, field_25, field_32a);
        assert!(mt900.validate_structure());

        // Test invalid reference with slashes
        let field_20_invalid = Field20::new("/TXN240315001234/".to_string());
        let mt900_invalid = MT900::new(
            field_20_invalid,
            mt900.field_21.clone(),
            mt900.field_25.clone(),
            mt900.field_32a.clone(),
        );
        assert!(!mt900_invalid.validate_structure());
    }

    #[test]
    fn test_mt900_business_logic() {
        let field_20 = Field20::new("TXN240315001234".to_string());
        let field_21 = Field21::new("ORIG240314567890".to_string());
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            50000.00,
        );

        let mt900 = MT900::new(field_20, field_21, field_25, field_32a);

        // Test high-value transaction detection
        assert!(mt900.is_high_value_transaction(10000.00));
        assert!(!mt900.is_high_value_transaction(100000.00));

        // Test description generation
        let description = mt900.get_confirmation_description();
        assert!(description.contains("Debit confirmation"));
        assert!(description.contains("USD"));
        assert!(description.contains("50000.00"));
        assert!(description.contains("2024-03-15"));
    }

    #[test]
    fn test_mt900_eri_codes() {
        let field_20 = Field20::new("TXN240315001236".to_string());
        let field_21 = Field21::new("ORIG240314567892".to_string());
        let field_25 = Field25::new("US1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            25000.00,
        );

        let field_72 = Some(
            Field72::new(vec![
                "/ERI/BILATERAL001".to_string(),
                "/EXCH/FORWARD/1.0950".to_string(),
            ])
            .unwrap(),
        );

        let mt900 = MT900::new_complete(
            field_20, field_21, field_25, field_32a, None, None, field_72,
        );

        assert!(mt900.has_eri_codes());
        assert!(mt900.has_exchange_rate_info());

        let instructions = mt900.get_processing_instructions();
        assert_eq!(instructions.len(), 2);
        assert!(instructions[0].contains("/ERI/"));
        assert!(instructions[1].contains("/EXCH/"));
    }

    #[test]
    fn test_mt900_optional_information_summary() {
        let field_20 = Field20::new("TXN240315001237".to_string());
        let field_21 = Field21::new("ORIG240314567893".to_string());
        let field_25 = Field25::new("DE1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "EUR".to_string(),
            15000.00,
        );

        let field_13d = Some(Field13D::new("240315", "0930", "-", "0500").unwrap());
        let field_52a = Some(GenericBicField::new(None, None, "CHASUS33XXX").unwrap());
        let field_72 = Some(
            Field72::new(vec![
                "/ERI/MULTILATERAL".to_string(),
                "SETTLEMENT CONFIRMATION".to_string(),
            ])
            .unwrap(),
        );

        let mt900 = MT900::new_complete(
            field_20, field_21, field_25, field_32a, field_13d, field_52a, field_72,
        );

        let summary = mt900.get_optional_information_summary();
        assert!(summary.len() >= 3); // Should have timing, institution, and additional info
        assert!(summary.iter().any(|s| s.contains("Precise timing")));
        assert!(summary.iter().any(|s| s.contains("Ordering institution")));
        assert!(summary.iter().any(|s| s.contains("Additional information")));
        assert!(summary.iter().any(|s| s.contains("ERI codes")));
    }

    #[test]
    fn test_mt900_date_time_formatting() {
        let field_20 = Field20::new("TXN240315001238".to_string());
        let field_21 = Field21::new("ORIG240314567894".to_string());
        let field_25 = Field25::new("JP1234567890123456".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "JPY".to_string(),
            1000000.00,
        );

        let field_13d = Some(Field13D::new("240315", "2145", "+", "0900").unwrap());

        let mt900 = MT900::new_complete(
            field_20, field_21, field_25, field_32a, field_13d, None, None,
        );

        let formatted_datetime = mt900.get_formatted_datetime();
        assert!(formatted_datetime.is_some());
        let datetime_str = formatted_datetime.unwrap();
        assert!(datetime_str.contains("24/03/15"));
        assert!(datetime_str.contains("21:45"));
        assert!(datetime_str.contains("+09:00"));
    }
}
