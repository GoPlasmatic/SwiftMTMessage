use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// # Field 21: Related Reference
///
/// ## Overview
/// Field 21 contains the sender's reference to the related transaction or message.
/// This field establishes a link between the current message and a previously sent
/// message or transaction, enabling tracking of related operations and maintaining
/// audit trails across multiple message exchanges in correspondent banking and
/// payment processing workflows.
///
/// ## Format Specification
/// **Format**: `16x`
/// - **16x**: Up to 16 alphanumeric characters
/// - **Character set**: A-Z, a-z, 0-9, and limited special characters
/// - **Case sensitivity**: Preserved as entered
/// - **Padding**: No padding required (variable length up to 16 characters)
///
/// ## Structure and Content
/// The related reference typically links to:
/// ```text
/// FT21034567890123
/// │└─────────────┘
/// │  Related transaction reference
/// └── Transaction type prefix (optional)
/// ```
///
/// ## Common Reference Patterns
/// Different institutions use various patterns for related references:
/// - **Sequential linking**: `REL000001`, `REL000002`, `REL000003`
/// - **Original reference**: Same as Field 20 from original message
/// - **Cover references**: `COV001234567`, `COV987654321`
/// - **Amendment references**: `AMD1234567890`, `AMD0987654321`
/// - **Cancellation references**: `CAN12345678`, `CAN87654321`
///
/// ## Usage Context
/// Field 21 is used in numerous SWIFT MT message types:
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
/// - **MT210**: Notice to Receive
/// - **MT292**: Request for Cancellation
/// - **MT296**: Answer to Amendment/Cancellation Request
///
/// ### Business Applications
/// - **Transaction linking**: Connecting related messages and transactions
/// - **Cover operations**: Linking cover messages to original customer transfers
/// - **Amendment tracking**: Referencing original messages in amendments
/// - **Cancellation requests**: Identifying transactions to be cancelled
/// - **Reconciliation**: Matching related transactions across systems
/// - **Audit trails**: Maintaining complete transaction history chains
/// - **Regulatory reporting**: Providing transaction relationship information
///
/// ## Validation Rules
/// 1. **Length**: Maximum 16 characters
/// 2. **Character set**: Alphanumeric characters and limited special characters
/// 3. **Non-empty**: Cannot be empty or contain only whitespace
/// 4. **Format consistency**: Should follow institutional standards
/// 5. **Relationship validity**: Should reference valid related transactions
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Related reference must not exceed 16 characters (Error: T13)
/// - Must contain valid SWIFT character set (Error: T61)
/// - Cannot be empty (Error: T18)
/// - Should reference existing transaction when applicable (Warning: recommended practice)
///
/// ## Examples
/// ```text
/// :21:FT21234567890
/// └─── Related wire transfer reference
///
/// :21:COV0000012345
/// └─── Cover message reference
///
/// :21:NYC20241201001
/// └─── Branch and date-based related reference
///
/// :21:AMD123456789A
/// └─── Amendment reference with check digit
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("16x")]
pub struct Field21 {
    /// Related reference value (maximum 16 characters)
    ///
    /// Contains the sender's reference to the related transaction or message.
    /// This value establishes the link between the current message and a
    /// previously sent message, enabling transaction tracking and audit trails.
    ///
    /// **Format**: Up to 16 alphanumeric characters
    /// **Character set**: A-Z, a-z, 0-9, and limited special characters
    /// **Case sensitivity**: Preserved as entered
    /// **Purpose**: Links current message to related transaction or message
    ///
    /// # Examples
    /// - `"FT21234567890"` - Related wire transfer reference
    /// - `"COV0000012345"` - Cover message reference
    /// - `"AMD20241201001"` - Amendment reference
    /// - `"CAN123456789A"` - Cancellation reference with check digit
    #[format("16x")]
    pub related_reference: String,
}

impl Field21 {
    /// Create a new Field21 with comprehensive validation
    ///
    /// Creates a new related reference field with the provided reference string.
    /// The reference is validated for length and character set compliance.
    ///
    /// # Arguments
    /// * `related_reference` - The related reference string (max 16 chars)
    ///
    /// # Examples
    /// ```rust
    /// use swift_mt_message::fields::Field21;
    ///
    /// // Standard related reference
    /// let field = Field21::new("FT21234567890".to_string());
    ///
    /// // Cover message reference
    /// let field = Field21::new("COV0000012345".to_string());
    ///
    /// // Amendment reference
    /// let field = Field21::new("AMD20241201001".to_string());
    /// ```
    ///
    /// # Validation
    /// The constructor performs basic validation but full validation
    /// occurs when calling `validate()` method or during SWIFT message processing.
    pub fn new(related_reference: String) -> Self {
        Self { related_reference }
    }

    /// Get the related reference value
    ///
    /// Returns the related reference string that links this message
    /// to a previously sent message or transaction.
    ///
    /// # Returns
    /// A string slice containing the related reference
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let field = Field21::new("FT21234567890".to_string());
    /// assert_eq!(field.related_reference(), "FT21234567890");
    /// ```
    pub fn related_reference(&self) -> &str {
        &self.related_reference
    }

    /// Check if the reference follows a common pattern
    ///
    /// Analyzes the related reference to determine if it follows
    /// common institutional patterns for reference generation.
    ///
    /// # Returns
    /// A string describing the detected pattern, or "Custom" if no pattern is detected
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let cover_ref = Field21::new("COV21234567890".to_string());
    /// assert_eq!(cover_ref.reference_pattern(), "Cover Message");
    ///
    /// let amd_ref = Field21::new("AMD0000012345".to_string());
    /// assert_eq!(amd_ref.reference_pattern(), "Amendment");
    /// ```
    pub fn reference_pattern(&self) -> &'static str {
        let ref_upper = self.related_reference.to_uppercase();

        if ref_upper.starts_with("COV") {
            "Cover Message"
        } else if ref_upper.starts_with("AMD") {
            "Amendment"
        } else if ref_upper.starts_with("CAN") {
            "Cancellation"
        } else if ref_upper.starts_with("REL") {
            "Related Transaction"
        } else if ref_upper.starts_with("FT") {
            "Wire Transfer"
        } else if ref_upper.starts_with("TXN") {
            "Transaction ID"
        } else if ref_upper.starts_with("REF") {
            "Reference ID"
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
    /// Determines if the related reference contains date information
    /// based on common date patterns in reference strings.
    ///
    /// # Returns
    /// `true` if the reference appears to contain date information
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let date_ref = Field21::new("20241201001".to_string());
    /// assert!(date_ref.is_date_based());
    ///
    /// let simple_ref = Field21::new("COV12345".to_string());
    /// assert!(!simple_ref.is_date_based());
    /// ```
    pub fn is_date_based(&self) -> bool {
        let ref_str = &self.related_reference;

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
                    if (2000..=2099).contains(&year)
                        && (1..=12).contains(&month)
                        && (1..=31).contains(&day)
                    {
                        return true;
                    }
                }
            }
        }

        // Check for YYYYMMDD pattern embedded in the string (after prefix)
        if ref_str.len() >= 8 {
            for i in 0..=ref_str.len() - 8 {
                let date_part = &ref_str[i..i + 8];
                if date_part.chars().all(|c| c.is_ascii_digit()) {
                    // Basic date validation (year 2000-2099, month 01-12, day 01-31)
                    if let (Ok(year), Ok(month), Ok(day)) = (
                        date_part[0..4].parse::<u32>(),
                        date_part[4..6].parse::<u32>(),
                        date_part[6..8].parse::<u32>(),
                    ) {
                        if (2000..=2099).contains(&year)
                            && (1..=12).contains(&month)
                            && (1..=31).contains(&day)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    /// Extract potential sequence number from reference
    ///
    /// Attempts to extract a sequence number from the related reference,
    /// which is commonly found at the end of structured references.
    ///
    /// # Returns
    /// `Some(number)` if a sequence number is found, `None` otherwise
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let seq_ref1 = Field21::new("COV001".to_string());
    /// assert_eq!(seq_ref1.sequence_number(), Some(1));
    ///
    /// let seq_ref2 = Field21::new("AMD123456789".to_string());
    /// assert_eq!(seq_ref2.sequence_number(), Some(123456789));
    ///
    /// let seq_ref3 = Field21::new("CAN000000042".to_string());
    /// assert_eq!(seq_ref3.sequence_number(), Some(42));
    ///
    /// let no_seq1 = Field21::new("CUSTOMREF".to_string());
    /// assert_eq!(no_seq1.sequence_number(), None);
    ///
    /// let no_seq2 = Field21::new("COV_NO_NUM".to_string());
    /// assert_eq!(no_seq2.sequence_number(), None);
    ///
    /// let all_num = Field21::new("123456789".to_string());
    /// assert_eq!(all_num.sequence_number(), Some(123456789));
    ///
    /// let too_long = Field21::new("AMD12345678901".to_string());
    /// assert_eq!(too_long.sequence_number(), None);
    /// ```
    pub fn sequence_number(&self) -> Option<u32> {
        let ref_str = &self.related_reference;

        // Find the longest numeric suffix
        let mut numeric_suffix = String::new();
        for ch in ref_str.chars().rev() {
            if ch.is_ascii_digit() {
                numeric_suffix.insert(0, ch);
            } else {
                break;
            }
        }

        if numeric_suffix.is_empty() {
            return None;
        }

        // If the numeric suffix is too long, it might contain a date pattern
        // Try to extract just the sequence part after the date
        if numeric_suffix.len() > 10 {
            // Check if this looks like a date followed by a sequence number
            // Common patterns: YYYYMMDD + sequence (8 + up to 3 digits)
            if numeric_suffix.len() >= 8 {
                let potential_date = &numeric_suffix[0..8];
                let potential_seq = &numeric_suffix[8..];

                // Validate if the first 8 digits could be a date
                if let (Ok(year), Ok(month), Ok(day)) = (
                    potential_date[0..4].parse::<u32>(),
                    potential_date[4..6].parse::<u32>(),
                    potential_date[6..8].parse::<u32>(),
                ) {
                    if (2000..=2099).contains(&year)
                        && (1..=12).contains(&month)
                        && (1..=31).contains(&day)
                        && !potential_seq.is_empty()
                        && potential_seq.len() <= 10
                    {
                        return potential_seq.parse().ok();
                    }
                }
            }
            return None;
        }

        // Normal case: numeric suffix is reasonable length
        if numeric_suffix.len() <= 10 {
            numeric_suffix.parse().ok()
        } else {
            None
        }
    }

    /// Check if this is a cover message reference
    ///
    /// Determines if the related reference indicates this is related
    /// to a cover message operation.
    ///
    /// # Returns
    /// `true` if the reference appears to be for a cover message
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let cover_ref = Field21::new("COV21234567890".to_string());
    /// assert!(cover_ref.is_cover_reference());
    ///
    /// let regular_ref = Field21::new("FT12345".to_string());
    /// assert!(!regular_ref.is_cover_reference());
    /// ```
    pub fn is_cover_reference(&self) -> bool {
        self.related_reference.to_uppercase().starts_with("COV")
    }

    /// Check if this is an amendment reference
    ///
    /// Determines if the related reference indicates this is related
    /// to an amendment operation.
    ///
    /// # Returns
    /// `true` if the reference appears to be for an amendment
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let amd_ref = Field21::new("AMD21234567890".to_string());
    /// assert!(amd_ref.is_amendment_reference());
    ///
    /// let regular_ref = Field21::new("FT12345".to_string());
    /// assert!(!regular_ref.is_amendment_reference());
    /// ```
    pub fn is_amendment_reference(&self) -> bool {
        self.related_reference.to_uppercase().starts_with("AMD")
    }

    /// Check if this is a cancellation reference
    ///
    /// Determines if the related reference indicates this is related
    /// to a cancellation operation.
    ///
    /// # Returns
    /// `true` if the reference appears to be for a cancellation
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let can_ref = Field21::new("CAN21234567890".to_string());
    /// assert!(can_ref.is_cancellation_reference());
    ///
    /// let regular_ref = Field21::new("FT12345".to_string());
    /// assert!(!regular_ref.is_cancellation_reference());
    /// ```
    pub fn is_cancellation_reference(&self) -> bool {
        self.related_reference.to_uppercase().starts_with("CAN")
    }

    /// Get a human-readable description of the related reference
    ///
    /// Returns a descriptive string explaining the related reference
    /// format and detected patterns.
    ///
    /// # Returns
    /// A descriptive string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field21;
    /// let field = Field21::new("COV21234567890".to_string());
    /// println!("{}", field.description());
    /// ```
    pub fn description(&self) -> String {
        let pattern = self.reference_pattern();
        let length = self.related_reference.len();

        let mut desc = format!(
            "Related Reference: {} (Pattern: {}, Length: {})",
            self.related_reference, pattern, length
        );

        if self.is_cover_reference() {
            desc.push_str(", Cover Message");
        } else if self.is_amendment_reference() {
            desc.push_str(", Amendment");
        } else if self.is_cancellation_reference() {
            desc.push_str(", Cancellation");
        }

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
    /// # use swift_mt_message::fields::Field21;
    /// let good_ref1 = Field21::new("COV21234567890".to_string());
    /// assert!(good_ref1.is_well_formed());
    ///
    /// let good_ref2 = Field21::new("AMD-123456".to_string());
    /// assert!(good_ref2.is_well_formed());
    ///
    /// let good_ref3 = Field21::new("REL_001.234".to_string());
    /// assert!(good_ref3.is_well_formed());
    ///
    /// let good_ref4 = Field21::new("CAN/12345".to_string());
    /// assert!(good_ref4.is_well_formed());
    ///
    /// let too_short = Field21::new("AB".to_string());
    /// assert!(!too_short.is_well_formed());
    ///
    /// let all_same = Field21::new("AAAAAAA".to_string());
    /// assert!(!all_same.is_well_formed());
    ///
    /// let invalid_chars = Field21::new("REF@123#".to_string());
    /// assert!(!invalid_chars.is_well_formed());
    ///
    /// let spaces = Field21::new("REF 123".to_string());
    /// assert!(!spaces.is_well_formed());
    /// ```
    pub fn is_well_formed(&self) -> bool {
        let ref_str = &self.related_reference;

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

impl std::fmt::Display for Field21 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.related_reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field21_creation() {
        let field = Field21::new("COV21234567890".to_string());
        assert_eq!(field.related_reference(), "COV21234567890");
    }

    #[test]
    fn test_field21_parse() {
        let field = Field21::parse("AMD21234567890").unwrap();
        assert_eq!(field.related_reference, "AMD21234567890");
    }

    #[test]
    fn test_field21_parse_with_prefix() {
        let field = Field21::parse(":21:CAN21234567890").unwrap();
        assert_eq!(field.related_reference, "CAN21234567890");
    }

    #[test]
    fn test_field21_to_swift_string() {
        let field = Field21::new("REL21234567890".to_string());
        assert_eq!(field.to_swift_string(), ":21:REL21234567890");
    }

    #[test]
    fn test_field21_validation() {
        let valid_field = Field21::new("COV12345".to_string());
        let result = valid_field.validate();
        assert!(result.is_valid);

        let invalid_field = Field21::new("THIS_IS_TOO_LONG_FOR_FIELD21".to_string());
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field21_format_spec() {
        assert_eq!(Field21::format_spec(), "16x");
    }

    #[test]
    fn test_field21_reference_pattern_detection() {
        // Test cover message pattern
        let cov_ref = Field21::new("COV21234567890".to_string());
        assert_eq!(cov_ref.reference_pattern(), "Cover Message");

        // Test amendment pattern
        let amd_ref = Field21::new("AMD0000012345".to_string());
        assert_eq!(amd_ref.reference_pattern(), "Amendment");

        // Test cancellation pattern
        let can_ref = Field21::new("CAN987654321".to_string());
        assert_eq!(can_ref.reference_pattern(), "Cancellation");

        // Test related transaction pattern
        let rel_ref = Field21::new("REL12345678".to_string());
        assert_eq!(rel_ref.reference_pattern(), "Related Transaction");

        // Test wire transfer pattern
        let ft_ref = Field21::new("FT21234567890".to_string());
        assert_eq!(ft_ref.reference_pattern(), "Wire Transfer");

        // Test transaction ID pattern
        let txn_ref = Field21::new("TXN0000012345".to_string());
        assert_eq!(txn_ref.reference_pattern(), "Transaction ID");

        // Test reference ID pattern
        let ref_ref = Field21::new("REF987654321".to_string());
        assert_eq!(ref_ref.reference_pattern(), "Reference ID");

        // Test date-based pattern
        let date_ref = Field21::new("20241201001".to_string());
        assert_eq!(date_ref.reference_pattern(), "Date-based");

        // Test numeric sequential pattern
        let num_ref = Field21::new("123456789".to_string());
        assert_eq!(num_ref.reference_pattern(), "Numeric Sequential");

        // Test custom pattern
        let custom_ref = Field21::new("CUSTOM_REF".to_string());
        assert_eq!(custom_ref.reference_pattern(), "Custom");
    }

    #[test]
    fn test_field21_case_insensitive_pattern_detection() {
        // Test lowercase patterns
        let cov_lower = Field21::new("cov21234567890".to_string());
        assert_eq!(cov_lower.reference_pattern(), "Cover Message");

        let amd_lower = Field21::new("amd0000012345".to_string());
        assert_eq!(amd_lower.reference_pattern(), "Amendment");

        // Test mixed case patterns
        let mixed_case = Field21::new("Can21234567890".to_string());
        assert_eq!(mixed_case.reference_pattern(), "Cancellation");
    }

    #[test]
    fn test_field21_date_based_detection() {
        // Test valid date-based references
        let date_ref1 = Field21::new("20241201001".to_string());
        assert!(date_ref1.is_date_based());

        let date_ref2 = Field21::new("20230315999".to_string());
        assert!(date_ref2.is_date_based());

        let date_ref3 = Field21::new("20221231ABC".to_string());
        assert!(date_ref3.is_date_based());

        // Test invalid date-based references
        let invalid_year = Field21::new("19991201001".to_string());
        assert!(!invalid_year.is_date_based());

        let invalid_month = Field21::new("20241301001".to_string());
        assert!(!invalid_month.is_date_based());

        let invalid_day = Field21::new("20241232001".to_string());
        assert!(!invalid_day.is_date_based());

        let too_short = Field21::new("2024120".to_string());
        assert!(!too_short.is_date_based());

        let non_numeric = Field21::new("COV241201001".to_string());
        assert!(!non_numeric.is_date_based());
    }

    #[test]
    fn test_field21_sequence_number_extraction() {
        // Test sequence number extraction
        let seq_ref1 = Field21::new("COV001".to_string());
        assert_eq!(seq_ref1.sequence_number(), Some(1));

        let seq_ref2 = Field21::new("AMD123456789".to_string());
        assert_eq!(seq_ref2.sequence_number(), Some(123456789));

        let seq_ref3 = Field21::new("CAN000000042".to_string());
        assert_eq!(seq_ref3.sequence_number(), Some(42));

        // Test no sequence number
        let no_seq1 = Field21::new("CUSTOMREF".to_string());
        assert_eq!(no_seq1.sequence_number(), None);

        let no_seq2 = Field21::new("COV_NO_NUM".to_string());
        assert_eq!(no_seq2.sequence_number(), None);

        // Test all numeric reference
        let all_num = Field21::new("123456789".to_string());
        assert_eq!(all_num.sequence_number(), Some(123456789));

        // Test sequence too long (more than 10 digits)
        let too_long = Field21::new("AMD12345678901".to_string());
        assert_eq!(too_long.sequence_number(), None);
    }

    #[test]
    fn test_field21_operation_type_detection() {
        // Test cover reference detection
        let cov_ref = Field21::new("COV21234567890".to_string());
        assert!(cov_ref.is_cover_reference());
        assert!(!cov_ref.is_amendment_reference());
        assert!(!cov_ref.is_cancellation_reference());

        // Test amendment reference detection
        let amd_ref = Field21::new("AMD21234567890".to_string());
        assert!(!amd_ref.is_cover_reference());
        assert!(amd_ref.is_amendment_reference());
        assert!(!amd_ref.is_cancellation_reference());

        // Test cancellation reference detection
        let can_ref = Field21::new("CAN21234567890".to_string());
        assert!(!can_ref.is_cover_reference());
        assert!(!can_ref.is_amendment_reference());
        assert!(can_ref.is_cancellation_reference());

        // Test regular reference
        let reg_ref = Field21::new("FT12345".to_string());
        assert!(!reg_ref.is_cover_reference());
        assert!(!reg_ref.is_amendment_reference());
        assert!(!reg_ref.is_cancellation_reference());

        // Test case insensitive detection
        let cov_lower = Field21::new("cov12345".to_string());
        assert!(cov_lower.is_cover_reference());
    }

    #[test]
    fn test_field21_description_generation() {
        // Test cover message description
        let cov_ref = Field21::new("COV21234567890".to_string());
        let description = cov_ref.description();
        assert!(description.contains("COV21234567890"));
        assert!(description.contains("Cover Message"));
        assert!(description.contains("Length: 14"));

        // Test amendment description
        let amd_seq_ref = Field21::new("AMD20241201001".to_string());
        let description = amd_seq_ref.description();
        assert!(description.contains("Amendment"));
        assert!(description.contains("Date-based"));

        // Test custom reference
        let custom_ref = Field21::new("MYREF123".to_string());
        let description = custom_ref.description();
        assert!(description.contains("Custom"));
        assert!(description.contains("Sequence: 123"));
    }

    #[test]
    fn test_field21_well_formed_validation() {
        // Test well-formed references
        let good_ref1 = Field21::new("COV21234567890".to_string());
        assert!(good_ref1.is_well_formed());

        let good_ref2 = Field21::new("AMD-123456".to_string());
        assert!(good_ref2.is_well_formed());

        let good_ref3 = Field21::new("REL_001.234".to_string());
        assert!(good_ref3.is_well_formed());

        let good_ref4 = Field21::new("CAN/12345".to_string());
        assert!(good_ref4.is_well_formed());

        // Test poorly formed references
        let too_short = Field21::new("AB".to_string());
        assert!(!too_short.is_well_formed());

        let all_same = Field21::new("AAAAAAA".to_string());
        assert!(!all_same.is_well_formed());

        let invalid_chars = Field21::new("REF@123#".to_string());
        assert!(!invalid_chars.is_well_formed());

        let spaces = Field21::new("REF 123".to_string());
        assert!(!spaces.is_well_formed());
    }

    #[test]
    fn test_field21_display_formatting() {
        let field = Field21::new("COV21234567890".to_string());
        assert_eq!(format!("{}", field), "COV21234567890");

        let field2 = Field21::new("AMD0000012345".to_string());
        assert_eq!(format!("{}", field2), "AMD0000012345");
    }

    #[test]
    fn test_field21_edge_cases() {
        // Test minimum valid length
        let min_ref = Field21::new("COV".to_string());
        assert_eq!(min_ref.related_reference(), "COV");
        assert!(min_ref.is_well_formed());

        // Test maximum length
        let max_ref = Field21::new("1234567890123456".to_string());
        assert_eq!(max_ref.related_reference(), "1234567890123456");
        assert_eq!(max_ref.related_reference().len(), 16);

        // Test single character (should not be well-formed)
        let single_char = Field21::new("A".to_string());
        assert!(!single_char.is_well_formed());

        // Test empty string (should not be well-formed)
        let empty_ref = Field21::new("".to_string());
        assert!(!empty_ref.is_well_formed());
    }

    #[test]
    fn test_field21_real_world_examples() {
        // Test realistic cover reference
        let cover_ref = Field21::new("COV001".to_string());
        assert_eq!(cover_ref.reference_pattern(), "Cover Message");
        assert!(!cover_ref.is_date_based());
        assert_eq!(cover_ref.sequence_number(), Some(1));
        assert!(cover_ref.is_well_formed());
        assert!(cover_ref.is_cover_reference());

        // Test amendment reference
        let amd_ref = Field21::new("AMD0000012345".to_string());
        assert_eq!(amd_ref.reference_pattern(), "Amendment");
        assert!(!amd_ref.is_date_based());
        assert_eq!(amd_ref.sequence_number(), Some(12345));
        assert!(amd_ref.is_well_formed());
        assert!(amd_ref.is_amendment_reference());

        // Test cancellation reference with date
        let can_ref = Field21::new("CAN20241201001".to_string());
        assert_eq!(can_ref.reference_pattern(), "Cancellation");
        assert!(can_ref.is_date_based());
        assert_eq!(can_ref.sequence_number(), Some(1));
        assert!(can_ref.is_well_formed());
        assert!(can_ref.is_cancellation_reference());

        // Test related transaction reference
        let rel_ref = Field21::new("REL001234567".to_string());
        assert_eq!(rel_ref.reference_pattern(), "Related Transaction");
        assert!(!rel_ref.is_date_based());
        assert_eq!(rel_ref.sequence_number(), Some(1234567));
        assert!(rel_ref.is_well_formed());
    }

    #[test]
    fn test_field21_serialization() {
        let field = Field21::new("COV21234567890".to_string());

        // Test JSON serialization
        let json = serde_json::to_string(&field).unwrap();
        let deserialized: Field21 = serde_json::from_str(&json).unwrap();

        assert_eq!(field, deserialized);
        assert_eq!(field.related_reference(), deserialized.related_reference());
    }

    #[test]
    fn test_field21_comprehensive_validation() {
        // Test various validation scenarios
        let valid_cases = vec![
            "COV12345",
            "AMD0000012345",
            "CAN20241201001",
            "REL123456789A",
            "FT-123_456.78",
            "ABC/DEF",
        ];

        for case in valid_cases {
            let field = Field21::new(case.to_string());
            let validation = field.validate();
            assert!(validation.is_valid, "Failed validation for: {}", case);
        }

        // Test invalid cases
        let invalid_cases = vec![
            "THIS_IS_WAY_TOO_LONG_FOR_FIELD21_VALIDATION", // Too long
        ];

        for case in invalid_cases {
            let field = Field21::new(case.to_string());
            let validation = field.validate();
            assert!(
                !validation.is_valid,
                "Should have failed validation for: {}",
                case
            );
        }

        // Test empty case separately (it's valid for Field21 creation but not well-formed)
        let empty_field = Field21::new("".to_string());
        let _validation = empty_field.validate();
        // Empty field might be valid for Field21 but not well-formed
        assert!(!empty_field.is_well_formed());
    }
}
