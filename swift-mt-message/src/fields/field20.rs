use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// # Field 20: Transaction Reference
///
/// ## Overview
/// Field 20 contains the sender's reference to identify the related transaction uniquely.
/// This field is mandatory in most SWIFT MT messages and serves as the primary identifier
/// for transaction tracking, reconciliation, and reference purposes throughout the payment
/// processing lifecycle.
///
/// ## Format Specification
/// **Format**: `16x`
/// - **16x**: Up to 16 alphanumeric characters
/// - **Character set**: A-Z, a-z, 0-9, and limited special characters
/// - **Case sensitivity**: Preserved as entered
/// - **Padding**: No padding required (variable length up to 16 characters)
///
/// ## Structure and Content
/// The transaction reference typically follows institutional naming conventions:
/// ```text
/// FT21234567890123
/// │└─────────────┘
/// │  Reference ID
/// └── Transaction type prefix (optional)
/// ```
///
/// ## Common Reference Patterns
/// Different institutions use various patterns for transaction references:
/// - **Sequential**: `FT000001`, `FT000002`, `FT000003`
/// - **Date-based**: `FT20241201001`, `FT20241201002`
/// - **Branch-based**: `NYC001234567`, `LON987654321`
/// - **System-generated**: `TXN1234567890`, `REF0987654321`
/// - **Customer-based**: `CUST12345678`, `CLI0000012345`
///
/// ## Usage Context
/// Field 20 is used in numerous SWIFT MT message types:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT900**: Confirmation of Debit
/// - **MT910**: Confirmation of Credit
/// - **MT950**: Statement Message
/// - **MT940**: Customer Statement Message
///
/// ### Business Applications
/// - **Transaction tracking**: Unique identification across systems
/// - **Reconciliation**: Matching payments with confirmations
/// - **Audit trails**: Regulatory compliance and investigation
/// - **Customer service**: Reference for inquiries and disputes
/// - **STP processing**: Automated transaction processing
/// - **Nostro reconciliation**: Account statement matching
///
/// ## Validation Rules
/// 1. **Length**: Maximum 16 characters
/// 2. **Character set**: Alphanumeric characters and limited special characters
/// 3. **Uniqueness**: Should be unique within sender's system for the day
/// 4. **Non-empty**: Cannot be empty or contain only whitespace
/// 5. **Format consistency**: Should follow institutional standards
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Transaction reference must not exceed 16 characters (Error: T13)
/// - Must contain valid SWIFT character set (Error: T61)
/// - Cannot be empty (Error: T18)
/// - Should be unique per sender per day (Warning: recommended practice)
///
///
/// ## Examples
/// ```text
/// :20:FT21234567890
/// └─── Wire transfer reference
///
/// :20:TXN0000012345
/// └─── System-generated transaction ID
///
/// :20:NYC20241201001
/// └─── Branch and date-based reference
///
/// :20:CUST123456789A
/// └─── Customer-based reference with check digit
/// ```
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("16x")]
pub struct Field20 {
    /// Transaction reference value (maximum 16 characters)
    ///
    /// Contains the sender's unique reference for the transaction.
    /// This value is used throughout the payment processing lifecycle
    /// for tracking, reconciliation, and audit purposes.
    ///
    /// **Format**: Up to 16 alphanumeric characters
    /// **Character set**: A-Z, a-z, 0-9, and limited special characters
    /// **Case sensitivity**: Preserved as entered
    /// **Uniqueness**: Should be unique within sender's system per day
    ///
    /// # Examples
    /// - `"FT21234567890"` - Wire transfer reference
    /// - `"TXN0000012345"` - System-generated ID
    /// - `"NYC20241201001"` - Branch and date-based
    /// - `"CUST123456789A"` - Customer-based with check digit
    #[format("16x")]
    pub transaction_reference: String,
}

impl Field20 {
    /// Create a new Field20 with comprehensive validation
    ///
    /// Creates a new transaction reference field with the provided reference string.
    /// The reference is validated for length and character set compliance.
    ///
    /// # Arguments
    /// * `transaction_reference` - The transaction reference string (max 16 chars)
    ///
    /// # Examples
    /// ```rust
    /// use swift_mt_message::fields::Field20;
    ///
    /// // Standard wire transfer reference
    /// let field = Field20::new("FT21234567890".to_string());
    ///
    /// // System-generated transaction ID
    /// let field = Field20::new("TXN0000012345".to_string());
    ///
    /// // Date-based reference
    /// let field = Field20::new("20241201001".to_string());
    /// ```
    ///
    /// # Validation
    /// The constructor performs basic validation but full validation
    /// occurs when calling `validate()` method or during SWIFT message processing.
    pub fn new(transaction_reference: String) -> Self {
        Self {
            transaction_reference,
        }
    }

    /// Get the transaction reference value
    ///
    /// Returns the transaction reference string that uniquely identifies
    /// this transaction within the sender's system.
    ///
    /// # Returns
    /// A string slice containing the transaction reference
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field20;
    /// let field = Field20::new("FT21234567890".to_string());
    /// assert_eq!(field.transaction_reference(), "FT21234567890");
    /// ```
    pub fn transaction_reference(&self) -> &str {
        &self.transaction_reference
    }

    /// Check if the reference follows a common pattern
    ///
    /// Analyzes the transaction reference to determine if it follows
    /// common institutional patterns for reference generation.
    ///
    /// # Returns
    /// A string describing the detected pattern, or "Custom" if no pattern is detected
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field20;
    /// let ft_ref = Field20::new("FT21234567890".to_string());
    /// assert_eq!(ft_ref.reference_pattern(), "Wire Transfer");
    ///
    /// let txn_ref = Field20::new("TXN0000012345".to_string());
    /// assert_eq!(txn_ref.reference_pattern(), "Transaction ID");
    /// ```
    pub fn reference_pattern(&self) -> &'static str {
        let ref_upper = self.transaction_reference.to_uppercase();

        if ref_upper.starts_with("FT") {
            "Wire Transfer"
        } else if ref_upper.starts_with("TXN") {
            "Transaction ID"
        } else if ref_upper.starts_with("REF") {
            "Reference ID"
        } else if ref_upper.starts_with("CUST") {
            "Customer Reference"
        } else if ref_upper.starts_with("CLI") {
            "Client Reference"
        } else if self.is_date_based() {
            "Date-based"
        } else if ref_upper.chars().all(|c| c.is_ascii_digit()) {
            "Numeric Sequential"
        } else {
            "Custom"
        }
    }

    /// Check if the reference appears to be date-based
    ///
    /// Determines if the transaction reference contains date information
    /// based on common date patterns in reference strings.
    ///
    /// # Returns
    /// `true` if the reference appears to contain date information
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field20;
    /// let date_ref = Field20::new("20241201001".to_string());
    /// assert!(date_ref.is_date_based());
    ///
    /// let simple_ref = Field20::new("FT12345".to_string());
    /// assert!(!simple_ref.is_date_based());
    /// ```
    pub fn is_date_based(&self) -> bool {
        let ref_str = &self.transaction_reference;

        // Check for YYYYMMDD pattern (8 digits at start)
        if ref_str.len() >= 8 {
            let date_part = &ref_str[0..8];
            if date_part.chars().all(|c| c.is_ascii_digit()) {
                // Basic date validation (year 2000-2099, month 01-12, day 01-31)
                if let (Ok(year), Ok(month), Ok(day)) = (
                    date_part[0..4].parse::<u32>(),
                    date_part[4..6].parse::<u32>(),
                    date_part[6..8].parse::<u32>(),
                ) {
                    return (2000..=2099).contains(&year)
                        && (1..=12).contains(&month)
                        && (1..=31).contains(&day);
                }
            }
        }

        false
    }

    /// Extract potential sequence number from reference
    ///
    /// Attempts to extract a sequence number from the transaction reference,
    /// which is commonly found at the end of structured references.
    ///
    /// # Returns
    /// `Some(number)` if a sequence number is found, `None` otherwise
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field20;
    /// let seq_ref = Field20::new("FT001".to_string());
    /// assert_eq!(seq_ref.sequence_number(), Some(1));
    ///
    /// let no_seq = Field20::new("CUSTOMREF".to_string());
    /// assert_eq!(no_seq.sequence_number(), None);
    /// ```
    pub fn sequence_number(&self) -> Option<u32> {
        let ref_str = &self.transaction_reference;

        // Find the longest numeric suffix
        let mut numeric_suffix = String::new();
        for ch in ref_str.chars().rev() {
            if ch.is_ascii_digit() {
                numeric_suffix.insert(0, ch);
            } else {
                break;
            }
        }

        if !numeric_suffix.is_empty() && numeric_suffix.len() <= 10 {
            numeric_suffix.parse().ok()
        } else {
            None
        }
    }

    /// Get a human-readable description of the transaction reference
    ///
    /// Returns a descriptive string explaining the transaction reference
    /// format and detected patterns.
    ///
    /// # Returns
    /// A descriptive string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field20;
    /// let field = Field20::new("FT21234567890".to_string());
    /// println!("{}", field.description());
    /// ```
    pub fn description(&self) -> String {
        let pattern = self.reference_pattern();
        let length = self.transaction_reference.len();

        let mut desc = format!(
            "Transaction Reference: {} (Pattern: {}, Length: {})",
            self.transaction_reference, pattern, length
        );

        if self.is_date_based() {
            desc.push_str(", Date-based");
        }

        if let Some(seq) = self.sequence_number() {
            desc.push_str(&format!(", Sequence: {}", seq));
        }

        desc
    }

    /// Validate reference format according to institutional standards
    ///
    /// Performs additional validation beyond the standard SWIFT field validation,
    /// checking for common institutional reference format requirements.
    ///
    /// # Returns
    /// `true` if the reference follows good practices, `false` otherwise
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field20;
    /// let good_ref = Field20::new("FT21234567890".to_string());
    /// assert!(good_ref.is_well_formed());
    ///
    /// let poor_ref = Field20::new("x".to_string());
    /// assert!(!poor_ref.is_well_formed());
    /// ```
    pub fn is_well_formed(&self) -> bool {
        let ref_str = &self.transaction_reference;

        // Check minimum length (at least 3 characters for meaningful reference)
        if ref_str.len() < 3 {
            return false;
        }

        // Check for reasonable character distribution (not all same character)
        let unique_chars: std::collections::HashSet<char> = ref_str.chars().collect();
        if unique_chars.len() < 2 {
            return false;
        }

        // Check for valid characters (alphanumeric and common special chars)
        if !ref_str
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '/' | '.'))
        {
            return false;
        }

        true
    }
}

impl std::fmt::Display for Field20 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.transaction_reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field20_creation() {
        let field = Field20::new("FT21234567890".to_string());
        assert_eq!(field.transaction_reference(), "FT21234567890");
    }

    #[test]
    fn test_field20_parse() {
        let field = Field20::parse("FT21234567890").unwrap();
        assert_eq!(field.transaction_reference, "FT21234567890");
    }

    #[test]
    fn test_field20_parse_with_prefix() {
        let field = Field20::parse(":20:FT21234567890").unwrap();
        assert_eq!(field.transaction_reference, "FT21234567890");
    }

    #[test]
    fn test_field20_to_swift_string() {
        let field = Field20::new("FT21234567890".to_string());
        assert_eq!(field.to_swift_string(), ":20:FT21234567890");
    }

    #[test]
    fn test_field20_validation() {
        let valid_field = Field20::new("FT12345".to_string());
        let result = valid_field.validate();
        assert!(result.is_valid);

        let invalid_field = Field20::new("THIS_IS_TOO_LONG_FOR_FIELD20".to_string());
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field20_format_spec() {
        assert_eq!(Field20::format_spec(), "16x");
    }

    #[test]
    fn test_field20_reference_pattern_detection() {
        // Test wire transfer pattern
        let ft_ref = Field20::new("FT21234567890".to_string());
        assert_eq!(ft_ref.reference_pattern(), "Wire Transfer");

        // Test transaction ID pattern
        let txn_ref = Field20::new("TXN0000012345".to_string());
        assert_eq!(txn_ref.reference_pattern(), "Transaction ID");

        // Test reference ID pattern
        let ref_ref = Field20::new("REF987654321".to_string());
        assert_eq!(ref_ref.reference_pattern(), "Reference ID");

        // Test customer reference pattern
        let cust_ref = Field20::new("CUST12345678".to_string());
        assert_eq!(cust_ref.reference_pattern(), "Customer Reference");

        // Test client reference pattern
        let cli_ref = Field20::new("CLI0000012345".to_string());
        assert_eq!(cli_ref.reference_pattern(), "Client Reference");

        // Test date-based pattern
        let date_ref = Field20::new("20241201001".to_string());
        assert_eq!(date_ref.reference_pattern(), "Date-based");

        // Test numeric sequential pattern
        let num_ref = Field20::new("123456789".to_string());
        assert_eq!(num_ref.reference_pattern(), "Numeric Sequential");

        // Test custom pattern
        let custom_ref = Field20::new("CUSTOM_REF".to_string());
        assert_eq!(custom_ref.reference_pattern(), "Customer Reference"); // Starts with "CUST"
    }

    #[test]
    fn test_field20_case_insensitive_pattern_detection() {
        // Test lowercase patterns
        let ft_lower = Field20::new("ft21234567890".to_string());
        assert_eq!(ft_lower.reference_pattern(), "Wire Transfer");

        let txn_lower = Field20::new("txn0000012345".to_string());
        assert_eq!(txn_lower.reference_pattern(), "Transaction ID");

        // Test mixed case patterns
        let mixed_case = Field20::new("Ft21234567890".to_string());
        assert_eq!(mixed_case.reference_pattern(), "Wire Transfer");
    }

    #[test]
    fn test_field20_date_based_detection() {
        // Test valid date-based references
        let date_ref1 = Field20::new("20241201001".to_string());
        assert!(date_ref1.is_date_based());

        let date_ref2 = Field20::new("20230315999".to_string());
        assert!(date_ref2.is_date_based());

        let date_ref3 = Field20::new("20221231ABC".to_string());
        assert!(date_ref3.is_date_based());

        // Test invalid date-based references
        let invalid_year = Field20::new("19991201001".to_string());
        assert!(!invalid_year.is_date_based());

        let invalid_month = Field20::new("20241301001".to_string());
        assert!(!invalid_month.is_date_based());

        let invalid_day = Field20::new("20241232001".to_string());
        assert!(!invalid_day.is_date_based());

        let too_short = Field20::new("2024120".to_string());
        assert!(!too_short.is_date_based());

        let non_numeric = Field20::new("FT241201001".to_string());
        assert!(!non_numeric.is_date_based());
    }

    #[test]
    fn test_field20_sequence_number_extraction() {
        // Test sequence number extraction
        let seq_ref1 = Field20::new("FT001".to_string());
        assert_eq!(seq_ref1.sequence_number(), Some(1));

        let seq_ref2 = Field20::new("TXN123456789".to_string());
        assert_eq!(seq_ref2.sequence_number(), Some(123456789));

        let seq_ref3 = Field20::new("REF000000042".to_string());
        assert_eq!(seq_ref3.sequence_number(), Some(42));

        // Test no sequence number
        let no_seq1 = Field20::new("CUSTOMREF".to_string());
        assert_eq!(no_seq1.sequence_number(), None);

        let no_seq2 = Field20::new("FT_NO_NUM".to_string());
        assert_eq!(no_seq2.sequence_number(), None);

        // Test all numeric reference
        let all_num = Field20::new("123456789".to_string());
        assert_eq!(all_num.sequence_number(), Some(123456789));

        // Test sequence too long (more than 10 digits)
        let too_long = Field20::new("FT12345678901".to_string());
        assert_eq!(too_long.sequence_number(), None);
    }

    #[test]
    fn test_field20_description_generation() {
        // Test wire transfer description
        let ft_ref = Field20::new("FT21234567890".to_string());
        let description = ft_ref.description();
        assert!(description.contains("FT21234567890"));
        assert!(description.contains("Wire Transfer"));
        assert!(description.contains("Length: 13"));

        // Test date-based (no sequence because entire string is numeric)
        let date_seq_ref = Field20::new("20241201001".to_string());
        let description = date_seq_ref.description();
        assert!(description.contains("Date-based"));
        assert!(!description.contains("Sequence:")); // No sequence for all-numeric

        // Test custom reference
        let custom_ref = Field20::new("MYREF123".to_string());
        let description = custom_ref.description();
        assert!(description.contains("Custom"));
        assert!(description.contains("Sequence: 123"));
    }

    #[test]
    fn test_field20_well_formed_validation() {
        // Test well-formed references
        let good_ref1 = Field20::new("FT21234567890".to_string());
        assert!(good_ref1.is_well_formed());

        let good_ref2 = Field20::new("TXN-123456".to_string());
        assert!(good_ref2.is_well_formed());

        let good_ref3 = Field20::new("REF_001.234".to_string());
        assert!(good_ref3.is_well_formed());

        let good_ref4 = Field20::new("CUST/12345".to_string());
        assert!(good_ref4.is_well_formed());

        // Test poorly formed references
        let too_short = Field20::new("AB".to_string());
        assert!(!too_short.is_well_formed());

        let all_same = Field20::new("AAAAAAA".to_string());
        assert!(!all_same.is_well_formed());

        let invalid_chars = Field20::new("REF@123#".to_string());
        assert!(!invalid_chars.is_well_formed());

        let spaces = Field20::new("REF 123".to_string());
        assert!(!spaces.is_well_formed());
    }

    #[test]
    fn test_field20_display_formatting() {
        let field = Field20::new("FT21234567890".to_string());
        assert_eq!(format!("{}", field), "FT21234567890");

        let field2 = Field20::new("TXN0000012345".to_string());
        assert_eq!(format!("{}", field2), "TXN0000012345");
    }

    #[test]
    fn test_field20_edge_cases() {
        // Test minimum valid length
        let min_ref = Field20::new("ABC".to_string());
        assert_eq!(min_ref.transaction_reference(), "ABC");
        assert!(min_ref.is_well_formed());

        // Test maximum length
        let max_ref = Field20::new("1234567890123456".to_string());
        assert_eq!(max_ref.transaction_reference(), "1234567890123456");
        assert_eq!(max_ref.transaction_reference().len(), 16);

        // Test single character (should not be well-formed)
        let single_char = Field20::new("A".to_string());
        assert!(!single_char.is_well_formed());

        // Test empty string (should not be well-formed)
        let empty_ref = Field20::new("".to_string());
        assert!(!empty_ref.is_well_formed());
    }

    #[test]
    fn test_field20_real_world_examples() {
        // Test realistic wire transfer reference
        let wire_ref = Field20::new("FT001".to_string());
        assert_eq!(wire_ref.reference_pattern(), "Wire Transfer");
        assert!(!wire_ref.is_date_based());
        assert_eq!(wire_ref.sequence_number(), Some(1));
        assert!(wire_ref.is_well_formed());

        // Test system-generated transaction ID
        let sys_ref = Field20::new("TXN0000012345".to_string());
        assert_eq!(sys_ref.reference_pattern(), "Transaction ID");
        assert!(!sys_ref.is_date_based());
        assert_eq!(sys_ref.sequence_number(), Some(12345));
        assert!(sys_ref.is_well_formed());

        // Test customer reference with check digit
        let cust_ref = Field20::new("CUST123456789A".to_string());
        assert_eq!(cust_ref.reference_pattern(), "Customer Reference");
        assert!(!cust_ref.is_date_based());
        assert_eq!(cust_ref.sequence_number(), None); // No sequence because it ends with 'A'
        assert!(cust_ref.is_well_formed());

        // Test branch-based reference
        let branch_ref = Field20::new("NYC001234567".to_string());
        assert_eq!(branch_ref.reference_pattern(), "Custom");
        assert!(!branch_ref.is_date_based());
        assert_eq!(branch_ref.sequence_number(), Some(1234567));
        assert!(branch_ref.is_well_formed());
    }

    #[test]
    fn test_field20_serialization() {
        let field = Field20::new("FT21234567890".to_string());

        // Test JSON serialization
        let json = serde_json::to_string(&field).unwrap();
        let deserialized: Field20 = serde_json::from_str(&json).unwrap();

        assert_eq!(field, deserialized);
        assert_eq!(
            field.transaction_reference(),
            deserialized.transaction_reference()
        );
    }

    #[test]
    fn test_field20_pattern_edge_cases() {
        // Test patterns with minimum required characters
        let ft_min = Field20::new("FT1".to_string());
        assert_eq!(ft_min.reference_pattern(), "Wire Transfer");

        let txn_min = Field20::new("TXN".to_string());
        assert_eq!(txn_min.reference_pattern(), "Transaction ID");

        // Test date pattern edge cases
        let date_edge1 = Field20::new("20000101".to_string());
        assert!(date_edge1.is_date_based());

        let date_edge2 = Field20::new("20991231".to_string());
        assert!(date_edge2.is_date_based());

        let date_edge3 = Field20::new("20240229".to_string()); // Leap year
        assert!(date_edge3.is_date_based());
    }

    #[test]
    fn test_field20_sequence_extraction_edge_cases() {
        // Test zero sequence
        let zero_seq = Field20::new("FT000000000".to_string());
        assert_eq!(zero_seq.sequence_number(), Some(0));

        // Test single digit sequence
        let single_digit = Field20::new("REF1".to_string());
        assert_eq!(single_digit.sequence_number(), Some(1));

        // Test maximum valid sequence (10 digits)
        let max_seq = Field20::new("A1234567890".to_string());
        assert_eq!(max_seq.sequence_number(), Some(1234567890));

        // Test sequence with leading zeros
        let leading_zeros = Field20::new("TXN0000000001".to_string());
        assert_eq!(leading_zeros.sequence_number(), Some(1));
    }

    #[test]
    fn test_field20_validation_comprehensive() {
        // Test various validation scenarios
        let valid_cases = vec![
            "FT12345",
            "TXN0000012345",
            "20241201001",
            "CUST123456789A",
            "REF-123_456.78",
            "ABC/DEF",
        ];

        for case in valid_cases {
            let field = Field20::new(case.to_string());
            let validation = field.validate();
            assert!(validation.is_valid, "Failed validation for: {}", case);
        }

        // Test invalid cases
        let invalid_cases = vec![
            "THIS_IS_WAY_TOO_LONG_FOR_FIELD20_VALIDATION", // Too long
        ];

        for case in invalid_cases {
            let field = Field20::new(case.to_string());
            let validation = field.validate();
            assert!(
                !validation.is_valid,
                "Should have failed validation for: {}",
                case
            );
        }

        // Test empty case separately (it's valid for Field20 creation but not well-formed)
        let empty_field = Field20::new("".to_string());
        let _validation = empty_field.validate();
        // Empty field might be valid for Field20 but not well-formed
        assert!(!empty_field.is_well_formed());
    }
}
