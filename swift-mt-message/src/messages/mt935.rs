use crate::fields::{Field20, Field23, Field25, Field30, Field37H, Field72};
use crate::{SwiftField, SwiftMessageBody, SwiftResult};
use serde::{Deserialize, Serialize};

/// # MT935: Rate Change Advice
///
/// ## Overview
/// MT935 is used by a financial institution to advise another financial institution
/// of a change in interest rates. This message is critical for managing interest
/// rate exposure, updating pricing models, and ensuring accurate interest calculations
/// across correspondent banking relationships and customer accounts.
///
/// ## Message Type Specification
/// **Message Type**: `935`  
/// **Category**: Cash Management and Customer Status (Category 9)  
/// **Usage**: Rate Change Advice  
/// **Processing**: Interest rate management and notification  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ## Message Structure
/// The MT935 message consists of mandatory fields and repeating sequences for multiple rate changes:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 30**: Effective Date of New Rate (when rate becomes effective)
/// - **Field 37H**: New Interest Rate (rate value and indicator)
///
/// ### Conditional Fields (Sequence Context)
/// ```text
/// Field 23    - Further Identification (for general rate changes)
/// Field 25    - Account Identification (for account-specific rates)
/// Field 72    - Sender to Receiver Information (additional details)
/// ```
///
/// ## Conditional Rules
/// - **C1**: The repeating sequence of fields 23/25/30/37H must occur at least once and at most 10 times
/// - **C2**: Either Field 23 or Field 25 must be present in each sequence, but not both
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Interest rate updates**: Notifying changes in deposit or lending rates
/// - **Base rate changes**: Communicating central bank rate adjustments
/// - **Account-specific rates**: Updating rates for specific customer accounts
/// - **Product rate changes**: Modifying rates for specific banking products
/// - **Regulatory compliance**: Meeting rate disclosure requirements
/// - **Risk management**: Coordinating rate changes across institutions
///
/// ### Industry Sectors
/// - **Commercial Banking**: Customer account rate management
/// - **Correspondent Banking**: Inter-bank rate coordination
/// - **Central Banking**: Policy rate communication
/// - **Investment Banking**: Securities lending rate updates
/// - **Treasury Operations**: Funding rate adjustments
/// - **Corporate Banking**: Large customer rate negotiations
///
/// ## Usage Constraints and Guidelines
///
/// ### Rate Change Types
/// - **BASE**: Base rate changes (reference rates)
/// - **CALL**: Call money rates
/// - **COMMERCIAL**: Commercial lending rates
/// - **CURRENT**: Current account rates
/// - **DEPOSIT**: Deposit rates
/// - **NOTICE**: Notice deposit rates (with days specification)
/// - **PRIME**: Prime lending rates
///
/// ### When to Use MT935
/// - **✅ Rate announcements**: For official rate change notifications
/// - **✅ Bulk updates**: For multiple account or product rate changes
/// - **✅ Scheduled changes**: For pre-announced rate adjustments
/// - **✅ Emergency changes**: For urgent rate modifications
///
/// ## Field Specifications and Business Rules
///
/// ### Field 20 - Transaction Reference Number
/// - **Format**: `16x` (up to 16 alphanumeric characters)
/// - **Rule**: No leading/trailing slash, no '//' sequences
/// - **Purpose**: Unique identification for this rate change advice
///
/// ### Field 23 - Further Identification (Conditional C2)
/// - **Format**: `3!a[2!n]11x` (function code + optional days + identifier)
/// - **Function Codes**: BASE, CALL, COMMERCIAL, CURRENT, DEPOSIT, NOTICE, PRIME
/// - **Days**: Used only with NOTICE function (e.g., "07" for 7-day notice)
/// - **Purpose**: Identifies the type of rate being changed
///
/// ### Field 25 - Account Identification (Conditional C2)
/// - **Format**: `35x` (up to 35 alphanumeric characters)
/// - **Purpose**: Identifies specific account when rate applies to individual account
/// - **Alternative**: Used instead of Field 23 for account-specific rates
///
/// ### Field 30 - Effective Date of New Rate
/// - **Format**: `6!n` (YYMMDD format)
/// - **Rule**: Must be a valid calendar date
/// - **Purpose**: Specifies when the new rate becomes effective
///
/// ### Field 37H - New Interest Rate
/// - **Format**: `1!a[N]12d` (indicator + optional 'N' + rate)
/// - **Indicator**: 'C' for credit rate, 'D' for debit rate
/// - **Rate**: Up to 12 digits with comma as decimal separator
/// - **Rule**: Must include at least one digit before comma
/// - **Special**: Must not include sign if rate is zero
///
/// ### Field 72 - Sender to Receiver Information (Optional)
/// - **Format**: `6*35x` (up to 6 lines of 35 characters each)
/// - **Content**: Can include structured text or narrative
/// - **Codes**: Supports `/code/` + `//narrative` formats for bilateral use
///
/// ## Processing and Compliance
///
/// ### Network Validated Rules (SWIFT Standards)
/// - **T20**: Transaction reference format validation
/// - **T23**: Further identification format validation (if present)
/// - **T25**: Account identification format validation (if present)
/// - **T30**: Effective date format validation
/// - **T37**: Interest rate format validation
/// - **T72**: Sender to receiver information format validation (if present)
///
/// ### Business Rule Validations
/// - Transaction reference should be unique per sender per business day
/// - Effective date must be valid and reasonable (not too far in past/future)
/// - Interest rate must be within reasonable bounds for the currency/product
/// - Either Field 23 or Field 25 must be present in each sequence (C2 rule)
/// - Maximum 10 rate change sequences per message (C1 rule)
/// - Function codes must be valid for Field 23
/// - Days specification only valid with NOTICE function
///
/// ## Examples
/// ```text
/// Basic MT935 for base rate change:
/// :20:RATE240315001234
/// :23:BASE
/// :30:240316
/// :37H:D3,25
///
/// MT935 for account-specific rate with notice:
/// :20:RATE240315001235
/// :25:GB33BUKB20201555555555
/// :30:240320
/// :37H:C2,50
/// :72:/NOTICE/7DAYS
/// //RATE CHANGE EFFECTIVE 20/03/24
///
/// MT935 for multiple rate changes:
/// :20:RATE240315001236
/// :23:DEPOSIT
/// :30:240316
/// :37H:C2,75
/// :23:CURRENT
/// :30:240316
/// :37H:D0,50
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MT935 {
    /// **Transaction Reference Number** - Field 20 (Mandatory)
    /// Unique sender's reference for this rate change advice
    pub field_20: Field20,

    /// **Rate Change Sequences** (Mandatory, 1-10 occurrences)
    /// Each sequence represents one rate change
    pub rate_changes: Vec<RateChangeSequence>,

    /// **Sender to Receiver Information** - Field 72 (Optional)
    /// Additional information about the rate changes
    pub field_72: Option<Field72>,
}

/// # Rate Change Sequence
///
/// Represents a single rate change within an MT935 message.
/// Each sequence must have either Field 23 (rate type) OR Field 25 (account), but not both.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RateChangeSequence {
    /// **Further Identification** - Field 23 (Conditional C2)
    /// Identifies the type of rate being changed (BASE, CALL, COMMERCIAL, etc.)
    pub field_23: Option<Field23>,

    /// **Account Identification** - Field 25 (Conditional C2)
    /// Identifies specific account for account-specific rate changes
    pub field_25: Option<Field25>,

    /// **Effective Date of New Rate** - Field 30 (Mandatory)
    /// When the new rate becomes effective (YYMMDD format)
    pub field_30: Field30,

    /// **New Interest Rate** - Field 37H (Mandatory)
    /// The new interest rate value with C/D indicator
    pub field_37h: Field37H,
}

impl RateChangeSequence {
    /// Create a new rate change sequence with rate type identification
    pub fn new_with_rate_type(field_23: Field23, field_30: Field30, field_37h: Field37H) -> Self {
        Self {
            field_23: Some(field_23),
            field_25: None,
            field_30,
            field_37h,
        }
    }

    /// Create a new rate change sequence with account identification
    pub fn new_with_account(field_25: Field25, field_30: Field30, field_37h: Field37H) -> Self {
        Self {
            field_23: None,
            field_25: Some(field_25),
            field_30,
            field_37h,
        }
    }

    /// Check if this sequence satisfies conditional rule C2
    /// (either Field 23 OR Field 25 must be present, but not both)
    pub fn validate_rule_c2(&self) -> bool {
        match (&self.field_23, &self.field_25) {
            (Some(_), None) => true, // Has field 23, no field 25
            (None, Some(_)) => true, // Has field 25, no field 23
            _ => false,              // Both or neither present
        }
    }

    /// Get the effective date as a string
    pub fn effective_date_string(&self) -> String {
        self.field_30.to_swift_string()
    }

    /// Check if this is a credit rate
    pub fn is_credit_rate(&self) -> bool {
        self.field_37h.is_credit_rate()
    }

    /// Check if this is a debit rate
    pub fn is_debit_rate(&self) -> bool {
        self.field_37h.is_debit_rate()
    }
}

impl MT935 {
    /// Create a new MT935 with a single rate change
    pub fn new_single(field_20: Field20, rate_change: RateChangeSequence) -> Self {
        Self {
            field_20,
            rate_changes: vec![rate_change],
            field_72: None,
        }
    }

    /// Create a new MT935 with multiple rate changes
    pub fn new_multiple(field_20: Field20, rate_changes: Vec<RateChangeSequence>) -> Self {
        Self {
            field_20,
            rate_changes,
            field_72: None,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        &self.field_20.transaction_reference
    }

    /// Get the number of rate change sequences
    pub fn sequence_count(&self) -> usize {
        self.rate_changes.len()
    }

    /// Validate conditional rule C1 (1-10 sequences)
    pub fn validate_rule_c1(&self) -> bool {
        let count = self.rate_changes.len();
        (1..=10).contains(&count)
    }

    /// Validate conditional rule C2 for all sequences
    pub fn validate_rule_c2(&self) -> bool {
        self.rate_changes.iter().all(|seq| seq.validate_rule_c2())
    }

    /// Validate the overall message structure
    pub fn validate_structure(&self) -> bool {
        self.validate_rule_c1() && self.validate_rule_c2()
    }

    /// Add a rate change sequence
    pub fn add_rate_change(&mut self, rate_change: RateChangeSequence) {
        if self.rate_changes.len() < 10 {
            self.rate_changes.push(rate_change);
        }
    }

    /// Set additional information
    pub fn set_additional_info(&mut self, field_72: Field72) {
        self.field_72 = Some(field_72);
    }
}

impl SwiftMessageBody for MT935 {
    fn message_type() -> &'static str {
        "935"
    }

    fn from_fields(_fields: std::collections::HashMap<String, Vec<String>>) -> SwiftResult<Self> {
        // For now, return a basic implementation error
        // This would need proper field parsing implementation
        Err(crate::errors::ParseError::InvalidFormat {
            message: "MT935 field parsing not yet implemented".to_string(),
        })
    }

    fn to_fields(&self) -> std::collections::HashMap<String, Vec<String>> {
        // Basic implementation - would need proper field serialization
        std::collections::HashMap::new()
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["20", "30", "37H"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["23", "25", "72"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::*;

    #[test]
    fn test_mt935_message_type() {
        assert_eq!(MT935::message_type(), "935");
    }

    #[test]
    fn test_rate_change_sequence_with_rate_type() {
        let field_23 = Field23::new("BASE", None, "REF001").unwrap();
        let field_30 = Field30::new("240315");
        let field_37h = Field37H::new('D', false, 3.25).unwrap();

        let sequence = RateChangeSequence::new_with_rate_type(field_23, field_30, field_37h);

        assert!(sequence.field_23.is_some());
        assert!(sequence.field_25.is_none());
        assert!(sequence.validate_rule_c2());
        assert!(sequence.is_debit_rate());
        assert!(!sequence.is_credit_rate());
    }

    #[test]
    fn test_rate_change_sequence_with_account() {
        let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
        let field_30 = Field30::new("240315");
        let field_37h = Field37H::new('C', false, 2.50).unwrap();

        let sequence = RateChangeSequence::new_with_account(field_25, field_30, field_37h);

        assert!(sequence.field_23.is_none());
        assert!(sequence.field_25.is_some());
        assert!(sequence.validate_rule_c2());
        assert!(sequence.is_credit_rate());
        assert!(!sequence.is_debit_rate());
    }

    #[test]
    fn test_mt935_single_sequence() {
        let field_20 = Field20::new("RATE240315001234".to_string());
        let field_23 = Field23::new("BASE", None, "REF001").unwrap();
        let field_30 = Field30::new("240315");
        let field_37h = Field37H::new('D', false, 3.25).unwrap();

        let sequence = RateChangeSequence::new_with_rate_type(field_23, field_30, field_37h);
        let mt935 = MT935::new_single(field_20, sequence);

        assert_eq!(mt935.transaction_reference(), "RATE240315001234");
        assert_eq!(mt935.sequence_count(), 1);
        assert!(mt935.validate_structure());
    }

    #[test]
    fn test_mt935_multiple_sequences() {
        let field_20 = Field20::new("RATE240315001235".to_string());

        let field_23_1 = Field23::new("DEPO", None, "DEP001").unwrap();
        let field_30_1 = Field30::new("240316");
        let field_37h_1 = Field37H::new('C', false, 2.75).unwrap();
        let sequence1 = RateChangeSequence::new_with_rate_type(field_23_1, field_30_1, field_37h_1);

        let field_23_2 = Field23::new("CURR", None, "CUR001").unwrap();
        let field_30_2 = Field30::new("240316");
        let field_37h_2 = Field37H::new('D', false, 0.50).unwrap();
        let sequence2 = RateChangeSequence::new_with_rate_type(field_23_2, field_30_2, field_37h_2);

        let mt935 = MT935::new_multiple(field_20, vec![sequence1, sequence2]);

        assert_eq!(mt935.sequence_count(), 2);
        assert!(mt935.validate_structure());
    }

    #[test]
    fn test_mt935_validation_rules() {
        let field_20 = Field20::new("RATE240315001236".to_string());

        // Test empty sequences (should fail C1)
        let mt935_empty = MT935::new_multiple(field_20.clone(), vec![]);
        assert!(!mt935_empty.validate_rule_c1());

        // Test too many sequences (should fail C1)
        let field_23 = Field23::new("BASE", None, "REF001").unwrap();
        let field_30 = Field30::new("240315");
        let field_37h = Field37H::new('D', false, 3.25).unwrap();
        let sequence = RateChangeSequence::new_with_rate_type(field_23, field_30, field_37h);

        let many_sequences = vec![sequence; 11]; // 11 sequences (too many)
        let mt935_many = MT935::new_multiple(field_20, many_sequences);
        assert!(!mt935_many.validate_rule_c1());
    }
}
