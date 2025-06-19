use crate::fields::*;
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT920: Request Message
///
/// ## Overview
/// MT920 is used by a financial institution to request specific types of statements
/// or reports from another financial institution. This message enables automated
/// request processing for account statements, balance reports, and transaction
/// reports, facilitating efficient cash management and reconciliation processes.
///
/// ## Message Type Specification
/// **Message Type**: `920`  
/// **Category**: Cash Management and Customer Status (Category 9)  
/// **Usage**: Request Message  
/// **Processing**: Statement and report request processing  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ## Message Structure
/// The MT920 message consists of mandatory and optional fields organized in a specific sequence:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 12**: Message Requested (type of statement/report requested)
/// - **Field 25**: Account Identification (account for which statement is requested)
///
/// ### Optional Fields (Enhanced Processing)
/// ```text
/// Field 34F   - Debit or Debit/Credit Floor Limit (for MT942 requests)
/// Field 34F   - Credit Floor Limit Indicator (additional limit for MT942)
/// ```
///
/// ## Conditional Rules
/// - **C1**: If Field 12 = '942', Field 34F for debit or debit/credit must be present
/// - **C2**: When both Field 34F fields are present:
///   - First 34F must have sign 'D' (debit)
///   - Second 34F must have sign 'C' (credit)
/// - **C3**: Currency code must be same across all Field 34F entries in a message
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Statement requests**: Requesting MT940 (customer statement) or MT950 (statement message)
/// - **Balance reports**: Requesting MT941 (balance report)
/// - **Interim reports**: Requesting MT942 (interim transaction report)
/// - **Automated reporting**: Scheduled statement and report generation
/// - **Cash management**: Regular balance and transaction monitoring
/// - **Reconciliation**: Obtaining statements for reconciliation purposes
///
/// ### Industry Sectors
/// - **Corporate Banking**: Customer statement requests
/// - **Cash Management**: Automated balance monitoring
/// - **Treasury Operations**: Regular reporting requirements
/// - **Correspondent Banking**: Inter-bank statement requests
/// - **Financial Institutions**: Regulatory reporting needs
///
/// ## Usage Constraints and Guidelines
///
/// ### Message Types Supported
/// - **940**: Customer Statement Message
/// - **941**: Balance Report
/// - **942**: Interim Transaction Report
/// - **950**: Statement Message
///
/// ### When to Use MT920
/// - **✅ Automated requests**: For scheduled statement generation
/// - **✅ On-demand reporting**: For immediate statement needs
/// - **✅ Threshold monitoring**: For balance-based reporting (MT942)
/// - **✅ Reconciliation support**: For periodic reconciliation processes
///
/// ## Field Specifications and Business Rules
///
/// ### Field 20 - Transaction Reference Number
/// - **Format**: `16x` (up to 16 alphanumeric characters)
/// - **Rule**: No leading/trailing slash, no '//' sequences
/// - **Purpose**: Unique identification for this request message
///
/// ### Field 12 - Message Requested
/// - **Format**: `3!n` (exactly 3 numeric characters)
/// - **Values**: 940, 941, 942, 950
/// - **Purpose**: Specifies the type of statement or report requested
///
/// ### Field 25 - Account Identification
/// - **Format**: `35x` (up to 35 alphanumeric characters)
/// - **Content**: IBAN or other account identifier
/// - **Purpose**: Identifies the account for which the statement is requested
///
/// ### Field 34F - Floor Limit (Optional, Conditional)
/// - **Format**: `3!a[1!a]15d` (currency + optional sign + amount)
/// - **Sign**: 'D' for debit, 'C' for credit
/// - **Usage**: Required for MT942 requests, optional for others
/// - **Business Rule**: Amount must include comma for decimals
///
/// ## Processing and Compliance
///
/// ### Network Validated Rules (SWIFT Standards)
/// - **T20**: Transaction reference format validation
/// - **T12**: Message type validation (must be 940, 941, 942, or 950)
/// - **T25**: Account identification format validation
/// - **T34**: Floor limit format validation (if present)
///
/// ### Business Rule Validations
/// - Transaction reference should be unique per sender per business day
/// - Message requested must be valid SWIFT MT message type
/// - Account identification must be valid and accessible
/// - Floor limits must have valid currency and amount format
/// - Currency consistency across all 34F fields in the message
/// - Proper sign indicators for debit/credit limits
///
/// ## Examples
/// ```text
/// Basic MT920 request for customer statement:
/// :20:REQ240315001234
/// :12:940
/// :25:GB33BUKB20201555555555
///
/// MT920 request for interim report with floor limits:
/// :20:REQ240315001235
/// :12:942
/// :25:DE89370400440532013000
/// :34F:EURD10000,00
/// :34F:EURC5000,00
/// ```
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "920")]
pub struct MT920 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific request message.
    /// Used throughout the request lifecycle for tracking, correlation with
    /// response messages, and audit purposes.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT920 messages  
    /// **Business Rule**: No leading/trailing slash, no '//' sequences  
    /// **Example**: "REQ240315001234"
    #[field("20")]
    pub field_20: Field20,

    /// **Message Requested** - Field 12
    ///
    /// Specifies the type of SWIFT message being requested. This determines
    /// the format and content of the response message that will be generated.
    ///
    /// **Format**: Exactly 3 numeric characters  
    /// **Usage**: Mandatory in all MT920 messages  
    /// **Valid Values**: 940, 941, 942, 950  
    /// **Purpose**: Determines response message type and format
    #[field("12")]
    pub field_12: Field12,

    /// **Account Identification** - Field 25
    ///
    /// Identifies the specific account for which the statement or report
    /// is being requested. Must be a valid account identifier that the
    /// receiver can process and generate reports for.
    ///
    /// **Format**: Up to 35 alphanumeric characters  
    /// **Usage**: Mandatory in all MT920 messages  
    /// **Content**: IBAN or other account identifier  
    /// **Example**: "GB33BUKB20201555555555" (IBAN format)
    #[field("25")]
    pub field_25: Field25,

    /// **Debit or Debit/Credit Floor Limit** - Field 34F (Optional, Conditional C1)
    ///
    /// Specifies the floor limit for debit transactions or combined debit/credit
    /// transactions when requesting MT942 interim transaction reports. Transactions
    /// above this limit will be included in the report.
    ///
    /// **Format**: Currency (3 chars) + Optional sign ('D') + Amount  
    /// **Usage**: Conditional C1 - Required if Field 12 = '942'  
    /// **Sign**: 'D' indicates debit limit  
    /// **Business Rule**: Currency must match across all 34F fields
    #[field("34F_DEBIT")]
    pub field_34f_debit: Option<Field34F>,

    /// **Credit Floor Limit Indicator** - Field 34F (Optional, Conditional C2)
    ///
    /// Specifies the floor limit for credit transactions when requesting MT942
    /// interim transaction reports. Used in conjunction with debit floor limit
    /// to provide comprehensive transaction filtering.
    ///
    /// **Format**: Currency (3 chars) + Sign ('C') + Amount  
    /// **Usage**: Conditional C2 - When both 34F fields present  
    /// **Sign**: 'C' indicates credit limit  
    /// **Sequence**: Must be second 34F field if both are present
    #[field("34F_CREDIT")]
    pub field_34f_credit: Option<Field34F>,
}

impl MT920 {
    /// Creates a new MT920 with minimal required fields
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_12` - Message requested (940, 941, 942, or 950)
    /// * `field_25` - Account identification
    ///
    /// # Returns
    /// A new MT920 instance with only mandatory fields populated
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::messages::MT920;
    /// # use swift_mt_message::fields::*;
    /// let field_20 = Field20::new("REQ240315001234".to_string());
    /// let field_12 = Field12::new("940").unwrap();
    /// let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
    ///
    /// let mt920 = MT920::new(field_20, field_12, field_25);
    /// ```
    pub fn new(field_20: Field20, field_12: Field12, field_25: Field25) -> Self {
        Self {
            field_20,
            field_12,
            field_25,
            field_34f_debit: None,
            field_34f_credit: None,
        }
    }

    /// Creates a new MT920 with debit floor limit (for MT942 requests)
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_12` - Message requested (should be "942")
    /// * `field_25` - Account identification
    /// * `field_34f_debit` - Debit floor limit
    ///
    /// # Returns
    /// A new MT920 instance with debit floor limit (satisfies C1 rule for MT942)
    pub fn new_with_debit_limit(
        field_20: Field20,
        field_12: Field12,
        field_25: Field25,
        field_34f_debit: Field34F,
    ) -> Self {
        Self {
            field_20,
            field_12,
            field_25,
            field_34f_debit: Some(field_34f_debit),
            field_34f_credit: None,
        }
    }

    /// Creates a new MT920 with both debit and credit floor limits
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_12` - Message requested (should be "942")
    /// * `field_25` - Account identification
    /// * `field_34f_debit` - Debit floor limit (must have sign 'D')
    /// * `field_34f_credit` - Credit floor limit (must have sign 'C')
    ///
    /// # Returns
    /// A new MT920 instance with both floor limits (satisfies C1 and C2 rules)
    pub fn new_with_both_limits(
        field_20: Field20,
        field_12: Field12,
        field_25: Field25,
        field_34f_debit: Field34F,
        field_34f_credit: Field34F,
    ) -> Self {
        Self {
            field_20,
            field_12,
            field_25,
            field_34f_debit: Some(field_34f_debit),
            field_34f_credit: Some(field_34f_credit),
        }
    }

    /// Creates a new MT920 with all fields
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_12` - Message requested
    /// * `field_25` - Account identification
    /// * `field_34f_debit` - Debit floor limit (optional)
    /// * `field_34f_credit` - Credit floor limit (optional)
    ///
    /// # Returns
    /// A new MT920 instance with all fields populated
    ///
    /// # Note
    /// Caller must ensure conditional rule compliance
    pub fn new_complete(
        field_20: Field20,
        field_12: Field12,
        field_25: Field25,
        field_34f_debit: Option<Field34F>,
        field_34f_credit: Option<Field34F>,
    ) -> Self {
        Self {
            field_20,
            field_12,
            field_25,
            field_34f_debit,
            field_34f_credit,
        }
    }

    // Accessor methods for key fields

    /// Returns the transaction reference number
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Returns the requested message type
    pub fn message_requested(&self) -> &str {
        &self.field_12.message_type
    }

    /// Returns the account identification
    pub fn account_identification(&self) -> &str {
        self.field_25.authorisation()
    }

    /// Returns the debit floor limit if present
    pub fn debit_floor_limit(&self) -> Option<&Field34F> {
        self.field_34f_debit.as_ref()
    }

    /// Returns the credit floor limit if present
    pub fn credit_floor_limit(&self) -> Option<&Field34F> {
        self.field_34f_credit.as_ref()
    }

    // Business logic methods

    /// Checks if this is a customer statement request (MT940)
    ///
    /// # Returns
    /// `true` if requesting MT940, `false` otherwise
    pub fn is_customer_statement_request(&self) -> bool {
        self.field_12.is_customer_statement_request()
    }

    /// Checks if this is a balance report request (MT941)
    ///
    /// # Returns
    /// `true` if requesting MT941, `false` otherwise
    pub fn is_balance_report_request(&self) -> bool {
        self.field_12.is_balance_report_request()
    }

    /// Checks if this is an interim transaction report request (MT942)
    ///
    /// # Returns
    /// `true` if requesting MT942, `false` otherwise
    pub fn is_interim_report_request(&self) -> bool {
        self.field_12.is_interim_report_request()
    }

    /// Checks if this is a statement message request (MT950)
    ///
    /// # Returns
    /// `true` if requesting MT950, `false` otherwise
    pub fn is_statement_message_request(&self) -> bool {
        self.message_requested() == "950"
    }

    /// Checks if debit floor limit is present
    ///
    /// # Returns
    /// `true` if Field 34F for debit is present
    pub fn has_debit_floor_limit(&self) -> bool {
        self.field_34f_debit.is_some()
    }

    /// Checks if credit floor limit is present
    ///
    /// # Returns
    /// `true` if Field 34F for credit is present
    pub fn has_credit_floor_limit(&self) -> bool {
        self.field_34f_credit.is_some()
    }

    /// Checks if both floor limits are present
    ///
    /// # Returns
    /// `true` if both debit and credit floor limits are present
    pub fn has_both_floor_limits(&self) -> bool {
        self.has_debit_floor_limit() && self.has_credit_floor_limit()
    }

    /// Validates the C1 conditional rule for MT942 requests
    ///
    /// # Returns
    /// `true` if C1 rule is satisfied, `false` otherwise
    pub fn validate_rule_c1(&self) -> bool {
        if self.is_interim_report_request() {
            // For MT942, Field 34F for debit must be present
            self.has_debit_floor_limit()
        } else {
            // For other message types, C1 doesn't apply
            true
        }
    }

    /// Validates the C2 conditional rule for floor limit signs
    ///
    /// # Returns
    /// `true` if C2 rule is satisfied, `false` otherwise
    pub fn validate_rule_c2(&self) -> bool {
        match (&self.field_34f_debit, &self.field_34f_credit) {
            (Some(debit), Some(credit)) => {
                // When both present, first must be 'D', second must be 'C'
                debit.sign_indicator() == Some('D') && credit.sign_indicator() == Some('C')
            }
            _ => true, // Rule doesn't apply when not both present
        }
    }

    /// Validates the C3 conditional rule for currency consistency
    ///
    /// # Returns
    /// `true` if C3 rule is satisfied, `false` otherwise
    pub fn validate_rule_c3(&self) -> bool {
        match (&self.field_34f_debit, &self.field_34f_credit) {
            (Some(debit), Some(credit)) => {
                // Currency must be same across all 34F fields
                debit.currency() == credit.currency()
            }
            _ => true, // Rule doesn't apply when not both present
        }
    }

    /// Validates the overall message structure and business rules
    ///
    /// # Returns
    /// `true` if all validation rules pass, `false` otherwise
    pub fn validate_structure(&self) -> bool {
        // Basic structural validation
        let basic_valid = !self.transaction_reference().is_empty()
            && !self.message_requested().is_empty()
            && !self.account_identification().is_empty();

        // Conditional rule validation
        let c1_valid = self.validate_rule_c1();
        let c2_valid = self.validate_rule_c2();
        let c3_valid = self.validate_rule_c3();

        // Additional business rule validation
        let business_valid = !self.transaction_reference().starts_with('/')
            && !self.transaction_reference().ends_with('/')
            && !self.transaction_reference().contains("//");

        basic_valid && c1_valid && c2_valid && c3_valid && business_valid
    }

    /// Gets the currency code from floor limits (if any)
    ///
    /// # Returns
    /// Currency code if floor limits are present, None otherwise
    pub fn floor_limit_currency(&self) -> Option<&str> {
        if let Some(debit) = &self.field_34f_debit {
            Some(debit.currency())
        } else if let Some(credit) = &self.field_34f_credit {
            Some(credit.currency())
        } else {
            None
        }
    }

    /// Gets the debit floor limit amount
    ///
    /// # Returns
    /// Debit floor limit amount if present, None otherwise
    pub fn debit_floor_amount(&self) -> Option<f64> {
        self.field_34f_debit.as_ref().map(|f| f.amount())
    }

    /// Gets the credit floor limit amount
    ///
    /// # Returns
    /// Credit floor limit amount if present, None otherwise
    pub fn credit_floor_amount(&self) -> Option<f64> {
        self.field_34f_credit.as_ref().map(|f| f.amount())
    }

    /// Returns a formatted description of the request
    ///
    /// # Returns
    /// A human-readable description of the MT920 request
    pub fn get_request_description(&self) -> String {
        let message_type = self.field_12.get_description();
        let account = self.account_identification();

        let limits = if self.has_both_floor_limits() {
            format!(
                " with debit limit {} {:.2} and credit limit {} {:.2}",
                self.debit_floor_limit().unwrap().currency(),
                self.debit_floor_amount().unwrap(),
                self.credit_floor_limit().unwrap().currency(),
                self.credit_floor_amount().unwrap()
            )
        } else if self.has_debit_floor_limit() {
            format!(
                " with debit limit {} {:.2}",
                self.debit_floor_limit().unwrap().currency(),
                self.debit_floor_amount().unwrap()
            )
        } else {
            String::new()
        };

        format!(
            "Request for {} for account {}{}",
            message_type, account, limits
        )
    }

    /// Gets a summary of all floor limits present
    ///
    /// # Returns
    /// Vector of strings describing floor limits
    pub fn get_floor_limits_summary(&self) -> Vec<String> {
        let mut summary = Vec::new();

        if let Some(debit) = &self.field_34f_debit {
            summary.push(format!(
                "Debit floor limit: {} {:.2}",
                debit.currency(),
                debit.amount()
            ));
        }

        if let Some(credit) = &self.field_34f_credit {
            summary.push(format!(
                "Credit floor limit: {} {:.2}",
                credit.currency(),
                credit.amount()
            ));
        }

        summary
    }

    /// Checks if this request requires threshold-based reporting
    ///
    /// # Returns
    /// `true` if any floor limits are specified
    pub fn requires_threshold_reporting(&self) -> bool {
        self.has_debit_floor_limit() || self.has_credit_floor_limit()
    }

    /// Gets the request type category
    ///
    /// # Returns
    /// String describing the category of request
    pub fn get_request_category(&self) -> &'static str {
        match self.message_requested() {
            "940" => "Customer Statement",
            "941" => "Balance Report",
            "942" => "Interim Transaction Report",
            "950" => "Statement Message",
            _ => "Unknown Request Type",
        }
    }

    /// Checks if this is a valid MT942 request with proper floor limits
    ///
    /// # Returns
    /// `true` if MT942 request with valid floor limit configuration
    pub fn is_valid_mt942_request(&self) -> bool {
        if !self.is_interim_report_request() {
            return false;
        }

        // Must have debit floor limit for MT942
        if !self.has_debit_floor_limit() {
            return false;
        }

        // If both limits present, validate signs and currency
        if self.has_both_floor_limits() {
            self.validate_rule_c2() && self.validate_rule_c3()
        } else {
            true
        }
    }

    /// Gets validation errors for the message structure
    ///
    /// # Returns
    /// Vector of validation error descriptions
    pub fn get_validation_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if !self.validate_rule_c1() {
            errors
                .push("C1 rule violation: MT942 requests must have debit floor limit".to_string());
        }

        if !self.validate_rule_c2() {
            errors.push(
                "C2 rule violation: First 34F must have sign 'D', second must have sign 'C'"
                    .to_string(),
            );
        }

        if !self.validate_rule_c3() {
            errors
                .push("C3 rule violation: Currency must be same across all 34F fields".to_string());
        }

        if self.transaction_reference().contains("//") {
            errors.push("Transaction reference contains invalid '//' sequence".to_string());
        }

        if self.transaction_reference().starts_with('/')
            || self.transaction_reference().ends_with('/')
        {
            errors.push("Transaction reference has invalid leading or trailing slash".to_string());
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;

    #[test]
    fn test_mt920_creation_basic() {
        let field_20 = Field20::new("REQ240315001234".to_string());
        let field_12 = Field12::new("940").unwrap();
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());

        let mt920 = MT920::new(field_20, field_12, field_25);

        assert_eq!(mt920.transaction_reference(), "REQ240315001234");
        assert_eq!(mt920.message_requested(), "940");
        assert_eq!(mt920.account_identification(), "GB33BUKB20201555555555");
        assert!(mt920.is_customer_statement_request());
        assert!(!mt920.has_debit_floor_limit());
        assert!(!mt920.has_credit_floor_limit());
        assert!(mt920.validate_structure());
    }

    #[test]
    fn test_mt920_message_type() {
        assert_eq!(MT920::message_type(), "920");
    }

    #[test]
    fn test_mt920_with_debit_limit() {
        let field_20 = Field20::new("REQ240315001235".to_string());
        let field_12 = Field12::new("942").unwrap();
        let field_25 = Field25::new("DE89370400440532013000".to_string());
        let field_34f_debit = Field34F::new("EUR", Some('D'), 10000.00).unwrap();

        let mt920 = MT920::new_with_debit_limit(field_20, field_12, field_25, field_34f_debit);

        assert_eq!(mt920.message_requested(), "942");
        assert!(mt920.is_interim_report_request());
        assert!(mt920.has_debit_floor_limit());
        assert!(!mt920.has_credit_floor_limit());
        assert_eq!(mt920.debit_floor_amount(), Some(10000.00));
        assert_eq!(mt920.floor_limit_currency(), Some("EUR"));
        assert!(mt920.validate_rule_c1());
        assert!(mt920.validate_structure());
    }

    #[test]
    fn test_mt920_with_both_limits() {
        let field_20 = Field20::new("REQ240315001236".to_string());
        let field_12 = Field12::new("942").unwrap();
        let field_25 = Field25::new("US1234567890123456".to_string());
        let field_34f_debit = Field34F::new("USD", Some('D'), 15000.00).unwrap();
        let field_34f_credit = Field34F::new("USD", Some('C'), 5000.00).unwrap();

        let mt920 = MT920::new_with_both_limits(
            field_20,
            field_12,
            field_25,
            field_34f_debit,
            field_34f_credit,
        );

        assert!(mt920.has_both_floor_limits());
        assert_eq!(mt920.debit_floor_amount(), Some(15000.00));
        assert_eq!(mt920.credit_floor_amount(), Some(5000.00));
        assert!(mt920.validate_rule_c1());
        assert!(mt920.validate_rule_c2());
        assert!(mt920.validate_rule_c3());
        assert!(mt920.is_valid_mt942_request());
        assert!(mt920.validate_structure());
    }

    #[test]
    fn test_mt920_c1_rule_validation() {
        let field_20 = Field20::new("REQ240315001237".to_string());
        let field_25 = Field25::new("CH1234567890123456".to_string());

        // Test MT942 without debit limit (should fail C1)
        let field_12_942 = Field12::new("942").unwrap();
        let mt920_942_no_limit = MT920::new(field_20.clone(), field_12_942, field_25.clone());
        assert!(!mt920_942_no_limit.validate_rule_c1());
        assert!(!mt920_942_no_limit.validate_structure());

        // Test MT940 without limit (should pass C1 - rule doesn't apply)
        let field_12_940 = Field12::new("940").unwrap();
        let mt920_940_no_limit = MT920::new(field_20, field_12_940, field_25);
        assert!(mt920_940_no_limit.validate_rule_c1());
        assert!(mt920_940_no_limit.validate_structure());
    }

    #[test]
    fn test_mt920_c2_rule_validation() {
        let field_20 = Field20::new("REQ240315001238".to_string());
        let field_12 = Field12::new("942").unwrap();
        let field_25 = Field25::new("JP1234567890123456".to_string());

        // Test with correct signs (should pass C2)
        let field_34f_debit = Field34F::new("JPY", Some('D'), 1000000.00).unwrap();
        let field_34f_credit = Field34F::new("JPY", Some('C'), 500000.00).unwrap();
        let mt920_correct = MT920::new_with_both_limits(
            field_20.clone(),
            field_12.clone(),
            field_25.clone(),
            field_34f_debit,
            field_34f_credit,
        );
        assert!(mt920_correct.validate_rule_c2());

        // Test with incorrect signs (should fail C2)
        let field_34f_wrong1 = Field34F::new("JPY", Some('C'), 1000000.00).unwrap(); // Wrong sign for first
        let field_34f_wrong2 = Field34F::new("JPY", Some('D'), 500000.00).unwrap(); // Wrong sign for second
        let mt920_wrong = MT920::new_with_both_limits(
            field_20,
            field_12,
            field_25,
            field_34f_wrong1,
            field_34f_wrong2,
        );
        assert!(!mt920_wrong.validate_rule_c2());
        assert!(!mt920_wrong.validate_structure());
    }

    #[test]
    fn test_mt920_c3_rule_validation() {
        let field_20 = Field20::new("REQ240315001239".to_string());
        let field_12 = Field12::new("942").unwrap();
        let field_25 = Field25::new("CA1234567890123456".to_string());

        // Test with same currency (should pass C3)
        let field_34f_debit = Field34F::new("CAD", Some('D'), 20000.00).unwrap();
        let field_34f_credit = Field34F::new("CAD", Some('C'), 10000.00).unwrap();
        let mt920_same_currency = MT920::new_with_both_limits(
            field_20.clone(),
            field_12.clone(),
            field_25.clone(),
            field_34f_debit,
            field_34f_credit,
        );
        assert!(mt920_same_currency.validate_rule_c3());

        // Test with different currencies (should fail C3)
        let field_34f_debit_usd = Field34F::new("USD", Some('D'), 20000.00).unwrap();
        let field_34f_credit_eur = Field34F::new("EUR", Some('C'), 10000.00).unwrap();
        let mt920_diff_currency = MT920::new_with_both_limits(
            field_20,
            field_12,
            field_25,
            field_34f_debit_usd,
            field_34f_credit_eur,
        );
        assert!(!mt920_diff_currency.validate_rule_c3());
        assert!(!mt920_diff_currency.validate_structure());
    }

    #[test]
    fn test_mt920_request_types() {
        let field_20 = Field20::new("REQ240315001240".to_string());
        let field_25 = Field25::new("GB1234567890123456".to_string());

        // Test different request types
        let test_cases = [
            ("940", "Customer Statement"),
            ("941", "Balance Report"),
            ("942", "Interim Transaction Report"),
            ("950", "Statement Message"),
        ];

        for (msg_type, expected_category) in test_cases.iter() {
            let field_12 = Field12::new(msg_type).unwrap();
            let mt920 = MT920::new(field_20.clone(), field_12, field_25.clone());

            assert_eq!(mt920.message_requested(), *msg_type);
            assert_eq!(mt920.get_request_category(), *expected_category);

            match *msg_type {
                "940" => assert!(mt920.is_customer_statement_request()),
                "941" => assert!(mt920.is_balance_report_request()),
                "942" => assert!(mt920.is_interim_report_request()),
                "950" => assert!(mt920.is_statement_message_request()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_mt920_business_logic() {
        let field_20 = Field20::new("REQ240315001241".to_string());
        let field_12 = Field12::new("942").unwrap();
        let field_25 = Field25::new("AU1234567890123456".to_string());
        let field_34f_debit = Field34F::new("AUD", Some('D'), 50000.00).unwrap();
        let field_34f_credit = Field34F::new("AUD", Some('C'), 25000.00).unwrap();

        let mt920 = MT920::new_with_both_limits(
            field_20,
            field_12,
            field_25,
            field_34f_debit,
            field_34f_credit,
        );

        // Test business logic methods
        assert!(mt920.requires_threshold_reporting());
        assert!(mt920.is_valid_mt942_request());

        let description = mt920.get_request_description();
        assert!(description.contains("Interim Transaction Report"));
        assert!(description.contains("AU1234567890123456"));
        assert!(description.contains("debit limit AUD 50000.00"));
        assert!(description.contains("credit limit AUD 25000.00"));

        let limits_summary = mt920.get_floor_limits_summary();
        assert_eq!(limits_summary.len(), 2);
        assert!(limits_summary[0].contains("Debit floor limit: AUD 50000.00"));
        assert!(limits_summary[1].contains("Credit floor limit: AUD 25000.00"));
    }

    #[test]
    fn test_mt920_validation_errors() {
        let field_20 = Field20::new("REQ//240315001242".to_string()); // Invalid reference
        let field_12 = Field12::new("942").unwrap();
        let field_25 = Field25::new("NZ1234567890123456".to_string());

        // Create MT942 without required debit limit
        let mt920_invalid = MT920::new(field_20, field_12, field_25);

        let errors = mt920_invalid.get_validation_errors();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.contains("C1 rule violation")));
        assert!(errors.iter().any(|e| e.contains("invalid '//' sequence")));
    }

    #[test]
    fn test_mt920_floor_limit_access() {
        let field_20 = Field20::new("REQ240315001243".to_string());
        let field_12 = Field12::new("942").unwrap();
        let field_25 = Field25::new("SG1234567890123456".to_string());
        let field_34f_debit = Field34F::new("SGD", Some('D'), 75000.00).unwrap();

        let mt920 = MT920::new_with_debit_limit(field_20, field_12, field_25, field_34f_debit);

        assert_eq!(mt920.debit_floor_amount(), Some(75000.00));
        assert_eq!(mt920.credit_floor_amount(), None);
        assert_eq!(mt920.floor_limit_currency(), Some("SGD"));
        assert!(mt920.debit_floor_limit().is_some());
        assert!(mt920.credit_floor_limit().is_none());
    }
}
