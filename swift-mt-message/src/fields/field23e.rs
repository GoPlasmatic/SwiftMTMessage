use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Valid instruction codes for Field 23E
pub const VALID_INSTRUCTION_CODES: &[&str] = &[
    "CHQB", "HOLD", "INTC", "PHOB", "PHOI", "PHON", "REPA", "SDVA", "TELB", "TELE", "TELI",
];

/// # Field 23E: Instruction Code
///
/// ## Overview
/// Field 23E contains instruction codes that specify how the transaction should be processed
/// by the receiving financial institution. These codes provide additional processing instructions
/// beyond the basic operation code in Field 23B, enabling more granular control over payment
/// handling, timing, and communication requirements.
///
/// ## Format Specification
/// **Format**: `4!c[/30x]`
/// - **4!c**: Exactly 4 alphanumeric characters (instruction code)
/// - **[/30x]**: Optional additional information (up to 30 characters after slash)
/// - **Character set**: A-Z, 0-9 for instruction code; printable ASCII for additional info
/// - **Case handling**: Instruction codes normalized to uppercase
/// - **Validation**: Must use recognized instruction codes
///
/// ## Standard Instruction Codes
/// The SWIFT-recognized instruction codes and their meanings:
///
/// ### Payment Method Instructions
/// - **CHQB**: Pay by cheque/banker's draft - Physical payment instrument required
/// - **REPA**: Reimbursement payment - Payment for reimbursement purposes
///
/// ### Communication Instructions
/// - **PHOB**: Phone ordering customer before payment - Contact beneficiary before processing
/// - **PHOI**: Phone intermediary bank before payment - Contact intermediary institution
/// - **PHON**: Phone all parties before payment - Contact all relevant parties
/// - **TELB**: Telex beneficiary before payment - Send telex to beneficiary
/// - **TELE**: Telex all parties before payment - Send telex to all parties
/// - **TELI**: Telex intermediary bank before payment - Send telex to intermediary
///
/// ### Processing Instructions
/// - **HOLD**: Hold payment until further notice - Suspend processing pending instructions
/// - **INTC**: Intracompany payment - Internal company transfer
/// - **SDVA**: Same day value - Ensure same-day value dating
///
/// ## Usage Context
/// Field 23E is used in conjunction with Field 23B in various MT message types:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its Own Account
///
/// ### Business Applications
/// - **Payment timing**: Control when payments are processed (SDVA, HOLD)
/// - **Communication protocols**: Specify required notifications (PHOB, TELB, etc.)
/// - **Payment methods**: Indicate specific payment instruments (CHQB)
/// - **Risk management**: Enable additional verification steps (PHON, HOLD)
/// - **Compliance**: Support regulatory and internal control requirements
/// - **Customer service**: Ensure proper communication with beneficiaries
///
/// ## Business Rules and Restrictions
/// Field 23E usage is restricted based on the operation code in Field 23B:
///
/// ### Field 23B = SPRI (Special Priority)
/// Only the following 23E codes are permitted:
/// - **SDVA**: Same day value
/// - **TELB**: Telex beneficiary before payment
/// - **PHOB**: Phone ordering customer before payment
/// - **INTC**: Intracompany payment
///
/// ### Field 23B = SSTD or SPAY
/// Field 23E **must not** be present when 23B contains these codes.
///
/// ### Other 23B Values
/// Any valid instruction code may be used with other operation codes.
///
/// ## Validation Rules
/// 1. **Instruction code**: Must be exactly 4 alphanumeric characters
/// 2. **Valid codes**: Must be from the recognized instruction code list
/// 3. **Additional info**: Optional, max 30 characters if present
/// 4. **Character set**: Printable ASCII characters only
/// 5. **Business rules**: Must comply with Field 23B restrictions
/// 6. **Format**: Additional info must follow slash separator
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Instruction code must be exactly 4 characters (Error: T26)
/// - Must be a recognized instruction code (Error: T18)
/// - Additional information max 30 characters (Error: T13)
/// - Must comply with Field 23B business rules (Error: T40)
/// - Character set validation (Error: T61)
///
///
/// ## Examples
/// ```text
/// :23E:CHQB
/// └─── Pay by cheque/banker's draft
///
/// :23E:HOLD/COMPLIANCE CHECK
/// └─── Hold payment with additional information
///
/// :23E:PHOB/CALL BEFORE 5PM
/// └─── Phone beneficiary with specific timing
///
/// :23E:SDVA
/// └─── Same day value dating required
/// ```
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field23E {
    /// Instruction code (exactly 4 alphanumeric characters)
    ///
    /// Specifies the processing instruction for the transaction.
    /// Must be one of the recognized SWIFT instruction codes.
    ///
    /// **Format**: Exactly 4 alphanumeric characters (A-Z, 0-9)
    /// **Case handling**: Automatically normalized to uppercase
    /// **Validation**: Must be from the standard instruction code list
    ///
    /// # Standard Codes
    /// - `"CHQB"` - Pay by cheque/banker's draft
    /// - `"HOLD"` - Hold payment until further notice
    /// - `"INTC"` - Intracompany payment
    /// - `"PHOB"` - Phone ordering customer before payment
    /// - `"PHOI"` - Phone intermediary bank before payment
    /// - `"PHON"` - Phone all parties before payment
    /// - `"REPA"` - Reimbursement payment
    /// - `"SDVA"` - Same day value
    /// - `"TELB"` - Telex beneficiary before payment
    /// - `"TELE"` - Telex all parties before payment
    /// - `"TELI"` - Telex intermediary bank before payment
    pub instruction_code: String,

    /// Additional information (optional, up to 30 characters)
    ///
    /// Provides supplementary details about the instruction code.
    /// This field is optional and should only be used when additional
    /// clarification or specific details are required.
    ///
    /// **Format**: Up to 30 printable ASCII characters
    /// **Separator**: Must be preceded by "/" in SWIFT format
    /// **Validation**: Cannot be empty if specified
    ///
    /// # Examples
    /// - `"COMPLIANCE CHECK"` - For HOLD instructions
    /// - `"CALL BEFORE 5PM"` - For phone instructions
    /// - `"WEEKLY PAYMENT"` - For REPA instructions
    /// - `"URGENT PROCESSING"` - For time-sensitive instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl SwiftField for Field23E {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":23E:") {
            stripped // Remove ":23E:" prefix
        } else if let Some(stripped) = value.strip_prefix("23E:") {
            stripped // Remove "23E:" prefix
        } else {
            value
        };

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Field content cannot be empty after removing tag".to_string(),
            });
        }

        // Check if there's additional info (indicated by /)
        if let Some(slash_pos) = content.find('/') {
            let instruction_code = &content[..slash_pos];
            let additional_info = &content[slash_pos + 1..];

            Self::new(instruction_code, Some(additional_info.to_string()))
        } else {
            Self::new(content, None)
        }
    }

    fn to_swift_string(&self) -> String {
        match &self.additional_info {
            Some(info) => format!(":23E:{}/{}", self.instruction_code, info),
            None => format!(":23E:{}", self.instruction_code),
        }
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate instruction code length
        if self.instruction_code.len() != 4 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "23E".to_string(),
                expected: "4 characters".to_string(),
                actual: self.instruction_code.len(),
            });
        }

        // Validate instruction code characters (alphanumeric)
        if !self
            .instruction_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "23E".to_string(),
                message: "Instruction code must contain only alphanumeric characters".to_string(),
            });
        }

        // Validate against known instruction codes
        if !VALID_INSTRUCTION_CODES.contains(&self.instruction_code.as_str()) {
            errors.push(ValidationError::ValueValidation {
                field_tag: "23E".to_string(),
                message: format!("Invalid instruction code: {}", self.instruction_code),
            });
        }

        // Validate additional info if present
        if let Some(ref info) = self.additional_info {
            if info.len() > 30 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "23E".to_string(),
                    expected: "max 30 characters".to_string(),
                    actual: info.len(),
                });
            }

            if info.is_empty() {
                errors.push(ValidationError::ValueValidation {
                    field_tag: "23E".to_string(),
                    message: "Additional information cannot be empty if specified".to_string(),
                });
            }

            // Validate characters (printable ASCII)
            if !info.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "23E".to_string(),
                    message: "Additional information contains invalid characters".to_string(),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "4!c[/30x]"
    }
}

impl Field23E {
    /// Create a new Field23E with validation
    pub fn new(
        instruction_code: impl Into<String>,
        additional_info: Option<String>,
    ) -> crate::Result<Self> {
        let code = instruction_code.into().trim().to_uppercase();

        if code.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Instruction code cannot be empty".to_string(),
            });
        }

        if code.len() != 4 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Instruction code must be exactly 4 characters".to_string(),
            });
        }

        // Validate characters (alphanumeric)
        if !code.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: "Instruction code must contain only alphanumeric characters".to_string(),
            });
        }

        // Validate against known instruction codes
        if !VALID_INSTRUCTION_CODES.contains(&code.as_str()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "23E".to_string(),
                message: format!("Invalid instruction code: {}", code),
            });
        }

        // Validate additional info if present
        if let Some(ref info) = additional_info {
            if info.len() > 30 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Additional information too long (max 30 characters)".to_string(),
                });
            }

            if info.is_empty() {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Additional information cannot be empty if specified".to_string(),
                });
            }

            // Validate characters (printable ASCII)
            if !info.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Additional information contains invalid characters".to_string(),
                });
            }
        }

        Ok(Field23E {
            instruction_code: code,
            additional_info,
        })
    }

    /// Get the instruction code
    ///
    /// Returns the 4-character instruction code that specifies
    /// how the transaction should be processed.
    ///
    /// # Returns
    /// A string slice containing the instruction code in uppercase
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let field = Field23E::new("CHQB", None).unwrap();
    /// assert_eq!(field.code(), "CHQB");
    /// ```
    pub fn code(&self) -> &str {
        &self.instruction_code
    }

    /// Get the additional information
    ///
    /// Returns the optional additional information that provides
    /// supplementary details about the instruction.
    ///
    /// # Returns
    /// An optional string slice containing the additional information
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let field = Field23E::new("HOLD", Some("COMPLIANCE CHECK".to_string())).unwrap();
    /// assert_eq!(field.additional_info(), Some("COMPLIANCE CHECK"));
    /// ```
    pub fn additional_info(&self) -> Option<&str> {
        self.additional_info.as_deref()
    }

    /// Check if this is a valid instruction code
    ///
    /// Determines if the instruction code is recognized by SWIFT standards.
    ///
    /// # Returns
    /// `true` if the instruction code is valid
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let field = Field23E::new("CHQB", None).unwrap();
    /// assert!(field.is_valid_code());
    /// ```
    pub fn is_valid_code(&self) -> bool {
        VALID_INSTRUCTION_CODES.contains(&self.instruction_code.as_str())
    }

    /// Check if this is a communication instruction
    ///
    /// Determines if the instruction code requires communication
    /// with parties before processing the payment.
    ///
    /// # Returns
    /// `true` if the instruction requires communication
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let phone_instruction = Field23E::new("PHOB", None).unwrap();
    /// assert!(phone_instruction.is_communication_instruction());
    ///
    /// let payment_instruction = Field23E::new("CHQB", None).unwrap();
    /// assert!(!payment_instruction.is_communication_instruction());
    /// ```
    pub fn is_communication_instruction(&self) -> bool {
        matches!(
            self.instruction_code.as_str(),
            "PHOB" | "PHOI" | "PHON" | "TELB" | "TELE" | "TELI"
        )
    }

    /// Check if this is a timing instruction
    ///
    /// Determines if the instruction code affects the timing
    /// or scheduling of the payment processing.
    ///
    /// # Returns
    /// `true` if the instruction affects timing
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let timing_instruction = Field23E::new("SDVA", None).unwrap();
    /// assert!(timing_instruction.is_timing_instruction());
    ///
    /// let hold_instruction = Field23E::new("HOLD", None).unwrap();
    /// assert!(hold_instruction.is_timing_instruction());
    /// ```
    pub fn is_timing_instruction(&self) -> bool {
        matches!(self.instruction_code.as_str(), "SDVA" | "HOLD")
    }

    /// Check if this is a payment method instruction
    ///
    /// Determines if the instruction code specifies a particular
    /// payment method or instrument.
    ///
    /// # Returns
    /// `true` if the instruction specifies a payment method
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let method_instruction = Field23E::new("CHQB", None).unwrap();
    /// assert!(method_instruction.is_payment_method_instruction());
    ///
    /// let reimbursement = Field23E::new("REPA", None).unwrap();
    /// assert!(reimbursement.is_payment_method_instruction());
    /// ```
    pub fn is_payment_method_instruction(&self) -> bool {
        matches!(self.instruction_code.as_str(), "CHQB" | "REPA")
    }

    /// Check if this instruction requires manual intervention
    ///
    /// Determines if the instruction code typically requires
    /// manual processing or intervention by bank staff.
    ///
    /// # Returns
    /// `true` if manual intervention is likely required
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let manual_instruction = Field23E::new("HOLD", None).unwrap();
    /// assert!(manual_instruction.requires_manual_intervention());
    ///
    /// let auto_instruction = Field23E::new("SDVA", None).unwrap();
    /// assert!(!auto_instruction.requires_manual_intervention());
    /// ```
    pub fn requires_manual_intervention(&self) -> bool {
        matches!(
            self.instruction_code.as_str(),
            "HOLD" | "PHOB" | "PHOI" | "PHON" | "TELB" | "TELE" | "TELI" | "CHQB"
        )
    }

    /// Get the processing priority impact
    ///
    /// Returns how this instruction affects processing priority.
    /// Positive values increase priority, negative values decrease it.
    ///
    /// # Returns
    /// Priority adjustment (-2 to +2)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let urgent = Field23E::new("SDVA", None).unwrap();
    /// assert_eq!(urgent.priority_impact(), 2);
    ///
    /// let hold = Field23E::new("HOLD", None).unwrap();
    /// assert_eq!(hold.priority_impact(), -2);
    /// ```
    pub fn priority_impact(&self) -> i8 {
        match self.instruction_code.as_str() {
            "SDVA" => 2,                             // Same day value increases priority
            "PHON" | "TELE" => 1, // Communication to all parties increases priority
            "INTC" => 0,          // Intracompany has neutral impact
            "PHOB" | "PHOI" | "TELB" | "TELI" => -1, // Specific communication decreases priority
            "CHQB" | "REPA" => -1, // Physical instruments decrease priority
            "HOLD" => -2,         // Hold significantly decreases priority
            _ => 0,               // Unknown codes have neutral impact
        }
    }

    /// Get human-readable description of the instruction code
    ///
    /// Returns a descriptive string explaining what this instruction code
    /// represents and its typical usage in payment processing.
    ///
    /// # Returns
    /// A descriptive string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let field = Field23E::new("CHQB", None).unwrap();
    /// println!("{}", field.description());
    /// ```
    pub fn description(&self) -> &'static str {
        match self.instruction_code.as_str() {
            "CHQB" => "Pay by cheque/banker's draft - Physical payment instrument required",
            "HOLD" => "Hold payment until further notice - Suspend processing pending instructions",
            "INTC" => "Intracompany payment - Internal company transfer between related entities",
            "PHOB" => {
                "Phone ordering customer before payment - Contact beneficiary before processing"
            }
            "PHOI" => "Phone intermediary bank before payment - Contact intermediary institution",
            "PHON" => {
                "Phone all parties before payment - Contact all relevant parties for verification"
            }
            "REPA" => "Reimbursement payment - Payment for reimbursement or expense purposes",
            "SDVA" => "Same day value - Ensure same-day value dating for the payment",
            "TELB" => "Telex beneficiary before payment - Send telex notification to beneficiary",
            "TELE" => "Telex all parties before payment - Send telex notifications to all parties",
            "TELI" => {
                "Telex intermediary bank before payment - Send telex to intermediary institution"
            }
            _ => "Unknown instruction code - Non-standard or institution-specific instruction",
        }
    }

    /// Get the instruction category
    ///
    /// Returns the category that this instruction code belongs to,
    /// helping to group related instructions.
    ///
    /// # Returns
    /// Instruction category as a string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let field = Field23E::new("PHOB", None).unwrap();
    /// assert_eq!(field.instruction_category(), "Communication");
    /// ```
    pub fn instruction_category(&self) -> &'static str {
        match self.instruction_code.as_str() {
            "PHOB" | "PHOI" | "PHON" | "TELB" | "TELE" | "TELI" => "Communication",
            "SDVA" | "HOLD" => "Timing",
            "CHQB" | "REPA" => "Payment Method",
            "INTC" => "Internal Transfer",
            _ => "Other",
        }
    }

    /// Check if additional information is recommended
    ///
    /// Determines if this instruction code typically benefits from
    /// additional information to clarify processing requirements.
    ///
    /// # Returns
    /// `true` if additional information is recommended
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let hold = Field23E::new("HOLD", None).unwrap();
    /// assert!(hold.recommends_additional_info());
    ///
    /// let same_day = Field23E::new("SDVA", None).unwrap();
    /// assert!(!same_day.recommends_additional_info());
    /// ```
    pub fn recommends_additional_info(&self) -> bool {
        matches!(
            self.instruction_code.as_str(),
            "HOLD" | "PHOB" | "PHOI" | "PHON" | "TELB" | "TELE" | "TELI" | "REPA"
        )
    }

    /// Validate against Field 23B business rules
    pub fn validate_with_field_23b(&self, field_23b_code: &str) -> crate::Result<()> {
        match field_23b_code {
            "SPRI" => {
                // If 23B = SPRI, 23E can only contain SDVA, TELB, PHOB, INTC
                if !["SDVA", "TELB", "PHOB", "INTC"].contains(&self.instruction_code.as_str()) {
                    return Err(crate::ParseError::InvalidFieldFormat {
                        field_tag: "23E".to_string(),
                        message: format!(
                            "When Field 23B is SPRI, Field 23E can only be SDVA, TELB, PHOB, or INTC. Got: {}",
                            self.instruction_code
                        ),
                    });
                }
            }
            "SSTD" | "SPAY" => {
                // If 23B = SSTD/SPAY, 23E must not be used
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "23E".to_string(),
                    message: "Field 23E must not be present when Field 23B is SSTD or SPAY"
                        .to_string(),
                });
            }
            _ => {
                // For other 23B values, 23E can contain any valid instruction code
            }
        }

        Ok(())
    }

    /// Get comprehensive instruction details
    ///
    /// Returns a detailed description including the instruction code,
    /// category, description, and additional information if present.
    ///
    /// # Returns
    /// Formatted string with comprehensive details
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field23E;
    /// let field = Field23E::new("HOLD", Some("COMPLIANCE CHECK".to_string())).unwrap();
    /// println!("{}", field.comprehensive_description());
    /// ```
    pub fn comprehensive_description(&self) -> String {
        let base = format!(
            "{} ({}): {}",
            self.instruction_code,
            self.instruction_category(),
            self.description()
        );

        if let Some(ref info) = self.additional_info {
            format!("{} - Additional Info: {}", base, info)
        } else {
            base
        }
    }
}

impl std::fmt::Display for Field23E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.additional_info {
            Some(info) => write!(f, "{}/{}", self.instruction_code, info),
            None => write!(f, "{}", self.instruction_code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field23e_creation_simple() {
        let field = Field23E::new("CHQB", None).unwrap();
        assert_eq!(field.instruction_code, "CHQB");
        assert_eq!(field.additional_info, None);
        assert_eq!(field.code(), "CHQB");
    }

    #[test]
    fn test_field23e_creation_with_info() {
        let field = Field23E::new("HOLD", Some("COMPLIANCE CHECK".to_string())).unwrap();
        assert_eq!(field.instruction_code, "HOLD");
        assert_eq!(field.additional_info, Some("COMPLIANCE CHECK".to_string()));
        assert_eq!(field.additional_info(), Some("COMPLIANCE CHECK"));
    }

    #[test]
    fn test_field23e_parse_simple() {
        let field = Field23E::parse("INTC").unwrap();
        assert_eq!(field.instruction_code, "INTC");
        assert_eq!(field.additional_info, None);
    }

    #[test]
    fn test_field23e_parse_with_info() {
        let field = Field23E::parse("REPA/WEEKLY PAYMENT").unwrap();
        assert_eq!(field.instruction_code, "REPA");
        assert_eq!(field.additional_info, Some("WEEKLY PAYMENT".to_string()));
    }

    #[test]
    fn test_field23e_parse_with_tag_prefix() {
        let field = Field23E::parse(":23E:HOLD/INFO").unwrap();
        assert_eq!(field.instruction_code, "HOLD");
        assert_eq!(field.additional_info, Some("INFO".to_string()));

        let field = Field23E::parse("23E:SDVA").unwrap();
        assert_eq!(field.instruction_code, "SDVA");
        assert_eq!(field.additional_info, None);
    }

    #[test]
    fn test_field23e_case_normalization() {
        let field = Field23E::new("phob", None).unwrap();
        assert_eq!(field.instruction_code, "PHOB");
    }

    #[test]
    fn test_field23e_invalid_code() {
        let result = Field23E::new("INVL", None); // Invalid code
        assert!(result.is_err());

        let result = Field23E::new("ABC", None); // Too short
        assert!(result.is_err());

        let result = Field23E::new("ABCDE", None); // Too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field23e_invalid_additional_info() {
        let result = Field23E::new("HOLD", Some("A".repeat(31))); // Too long
        assert!(result.is_err());

        let result = Field23E::new("HOLD", Some("".to_string())); // Empty
        assert!(result.is_err());
    }

    #[test]
    fn test_field23e_business_rules() {
        // SPRI with valid codes
        let field = Field23E::new("SDVA", None).unwrap();
        assert!(field.validate_with_field_23b("SPRI").is_ok());

        let field = Field23E::new("TELB", None).unwrap();
        assert!(field.validate_with_field_23b("SPRI").is_ok());

        // SPRI with invalid code
        let field = Field23E::new("CHQB", None).unwrap();
        assert!(field.validate_with_field_23b("SPRI").is_err());

        // SSTD/SPAY should not allow 23E
        let field = Field23E::new("HOLD", None).unwrap();
        assert!(field.validate_with_field_23b("SSTD").is_err());
        assert!(field.validate_with_field_23b("SPAY").is_err());

        // Other 23B values allow any valid code
        let field = Field23E::new("CHQB", None).unwrap();
        assert!(field.validate_with_field_23b("CRED").is_ok());
    }

    #[test]
    fn test_field23e_to_swift_string() {
        let field = Field23E::new("TELI", None).unwrap();
        assert_eq!(field.to_swift_string(), ":23E:TELI");

        let field = Field23E::new("PHON", Some("CALL BEFORE".to_string())).unwrap();
        assert_eq!(field.to_swift_string(), ":23E:PHON/CALL BEFORE");
    }

    #[test]
    fn test_field23e_validation() {
        let field = Field23E::new("TELE", None).unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let invalid_field = Field23E {
            instruction_code: "INVALID".to_string(),
            additional_info: None,
        };
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field23e_format_spec() {
        assert_eq!(Field23E::format_spec(), "4!c[/30x]");
    }

    #[test]
    fn test_field23e_display() {
        let field = Field23E::new("PHOI", None).unwrap();
        assert_eq!(format!("{}", field), "PHOI");

        let field = Field23E::new("SDVA", Some("SAME DAY".to_string())).unwrap();
        assert_eq!(format!("{}", field), "SDVA/SAME DAY");
    }

    #[test]
    fn test_field23e_descriptions() {
        let field = Field23E::new("CHQB", None).unwrap();
        assert_eq!(
            field.description(),
            "Pay by cheque/banker's draft - Physical payment instrument required"
        );

        let field = Field23E::new("HOLD", None).unwrap();
        assert_eq!(
            field.description(),
            "Hold payment until further notice - Suspend processing pending instructions"
        );

        let field = Field23E::new("SDVA", None).unwrap();
        assert_eq!(
            field.description(),
            "Same day value - Ensure same-day value dating for the payment"
        );
    }

    #[test]
    fn test_field23e_is_valid_code() {
        let field = Field23E::new("CHQB", None).unwrap();
        assert!(field.is_valid_code());

        let field = Field23E {
            instruction_code: "XXXX".to_string(),
            additional_info: None,
        };
        assert!(!field.is_valid_code());
    }

    #[test]
    fn test_field23e_communication_instructions() {
        let communication_codes = ["PHOB", "PHOI", "PHON", "TELB", "TELE", "TELI"];
        for code in communication_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                field.is_communication_instruction(),
                "Code {} should be communication instruction",
                code
            );
            assert!(
                !field.is_timing_instruction(),
                "Code {} should not be timing instruction",
                code
            );
            assert!(
                !field.is_payment_method_instruction(),
                "Code {} should not be payment method instruction",
                code
            );
        }

        let non_communication_codes = ["CHQB", "HOLD", "INTC", "REPA", "SDVA"];
        for code in non_communication_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                !field.is_communication_instruction(),
                "Code {} should not be communication instruction",
                code
            );
        }
    }

    #[test]
    fn test_field23e_timing_instructions() {
        let timing_codes = ["SDVA", "HOLD"];
        for code in timing_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                field.is_timing_instruction(),
                "Code {} should be timing instruction",
                code
            );
            assert!(
                !field.is_communication_instruction(),
                "Code {} should not be communication instruction",
                code
            );
        }

        let non_timing_codes = ["CHQB", "PHOB", "INTC", "REPA"];
        for code in non_timing_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                !field.is_timing_instruction(),
                "Code {} should not be timing instruction",
                code
            );
        }
    }

    #[test]
    fn test_field23e_payment_method_instructions() {
        let payment_method_codes = ["CHQB", "REPA"];
        for code in payment_method_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                field.is_payment_method_instruction(),
                "Code {} should be payment method instruction",
                code
            );
            assert!(
                !field.is_communication_instruction(),
                "Code {} should not be communication instruction",
                code
            );
            assert!(
                !field.is_timing_instruction(),
                "Code {} should not be timing instruction",
                code
            );
        }

        let non_payment_method_codes = ["HOLD", "PHOB", "INTC", "SDVA"];
        for code in non_payment_method_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                !field.is_payment_method_instruction(),
                "Code {} should not be payment method instruction",
                code
            );
        }
    }

    #[test]
    fn test_field23e_manual_intervention_requirements() {
        let manual_codes = [
            "HOLD", "PHOB", "PHOI", "PHON", "TELB", "TELE", "TELI", "CHQB",
        ];
        for code in manual_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                field.requires_manual_intervention(),
                "Code {} should require manual intervention",
                code
            );
        }

        let automatic_codes = ["SDVA", "INTC", "REPA"];
        for code in automatic_codes {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                !field.requires_manual_intervention(),
                "Code {} should not require manual intervention",
                code
            );
        }
    }

    #[test]
    fn test_field23e_priority_impact() {
        let test_cases = [
            ("SDVA", 2),  // Same day value increases priority
            ("PHON", 1),  // Communication to all parties increases priority
            ("TELE", 1),  // Communication to all parties increases priority
            ("INTC", 0),  // Intracompany has neutral impact
            ("PHOB", -1), // Specific communication decreases priority
            ("PHOI", -1), // Specific communication decreases priority
            ("TELB", -1), // Specific communication decreases priority
            ("TELI", -1), // Specific communication decreases priority
            ("CHQB", -1), // Physical instruments decrease priority
            ("REPA", -1), // Physical instruments decrease priority
            ("HOLD", -2), // Hold significantly decreases priority
        ];

        for (code, expected_impact) in test_cases {
            let field = Field23E::new(code, None).unwrap();
            assert_eq!(
                field.priority_impact(),
                expected_impact,
                "Priority impact mismatch for code {}",
                code
            );
        }
    }

    #[test]
    fn test_field23e_instruction_categories() {
        let test_cases = [
            ("PHOB", "Communication"),
            ("PHOI", "Communication"),
            ("PHON", "Communication"),
            ("TELB", "Communication"),
            ("TELE", "Communication"),
            ("TELI", "Communication"),
            ("SDVA", "Timing"),
            ("HOLD", "Timing"),
            ("CHQB", "Payment Method"),
            ("REPA", "Payment Method"),
            ("INTC", "Internal Transfer"),
        ];

        for (code, expected_category) in test_cases {
            let field = Field23E::new(code, None).unwrap();
            assert_eq!(
                field.instruction_category(),
                expected_category,
                "Category mismatch for code {}",
                code
            );
        }
    }

    #[test]
    fn test_field23e_additional_info_recommendations() {
        let recommends_info = [
            "HOLD", "PHOB", "PHOI", "PHON", "TELB", "TELE", "TELI", "REPA",
        ];
        for code in recommends_info {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                field.recommends_additional_info(),
                "Code {} should recommend additional info",
                code
            );
        }

        let no_info_needed = ["SDVA", "INTC", "CHQB"];
        for code in no_info_needed {
            let field = Field23E::new(code, None).unwrap();
            assert!(
                !field.recommends_additional_info(),
                "Code {} should not recommend additional info",
                code
            );
        }
    }

    #[test]
    fn test_field23e_comprehensive_description() {
        // Test without additional info
        let field = Field23E::new("CHQB", None).unwrap();
        let desc = field.comprehensive_description();
        assert!(desc.contains("CHQB"));
        assert!(desc.contains("Payment Method"));
        assert!(desc.contains("Physical payment instrument required"));

        // Test with additional info
        let field = Field23E::new("HOLD", Some("COMPLIANCE CHECK".to_string())).unwrap();
        let desc = field.comprehensive_description();
        assert!(desc.contains("HOLD"));
        assert!(desc.contains("Timing"));
        assert!(desc.contains("Suspend processing pending instructions"));
        assert!(desc.contains("Additional Info: COMPLIANCE CHECK"));
    }

    #[test]
    fn test_field23e_enhanced_descriptions() {
        let test_cases = [
            (
                "CHQB",
                "Pay by cheque/banker's draft - Physical payment instrument required",
            ),
            (
                "HOLD",
                "Hold payment until further notice - Suspend processing pending instructions",
            ),
            (
                "INTC",
                "Intracompany payment - Internal company transfer between related entities",
            ),
            (
                "PHOB",
                "Phone ordering customer before payment - Contact beneficiary before processing",
            ),
            (
                "PHOI",
                "Phone intermediary bank before payment - Contact intermediary institution",
            ),
            (
                "PHON",
                "Phone all parties before payment - Contact all relevant parties for verification",
            ),
            (
                "REPA",
                "Reimbursement payment - Payment for reimbursement or expense purposes",
            ),
            (
                "SDVA",
                "Same day value - Ensure same-day value dating for the payment",
            ),
            (
                "TELB",
                "Telex beneficiary before payment - Send telex notification to beneficiary",
            ),
            (
                "TELE",
                "Telex all parties before payment - Send telex notifications to all parties",
            ),
            (
                "TELI",
                "Telex intermediary bank before payment - Send telex to intermediary institution",
            ),
        ];

        for (code, expected_desc) in test_cases {
            let field = Field23E::new(code, None).unwrap();
            assert_eq!(
                field.description(),
                expected_desc,
                "Description mismatch for code {}",
                code
            );
        }
    }

    #[test]
    fn test_field23e_business_logic_combinations() {
        // Test SDVA: timing instruction, high priority
        let sdva = Field23E::new("SDVA", None).unwrap();
        assert!(sdva.is_timing_instruction());
        assert_eq!(sdva.priority_impact(), 2);
        assert!(!sdva.requires_manual_intervention());
        assert!(!sdva.recommends_additional_info());
        assert_eq!(sdva.instruction_category(), "Timing");

        // Test HOLD: timing instruction, low priority, manual intervention
        let hold = Field23E::new("HOLD", None).unwrap();
        assert!(hold.is_timing_instruction());
        assert_eq!(hold.priority_impact(), -2);
        assert!(hold.requires_manual_intervention());
        assert!(hold.recommends_additional_info());
        assert_eq!(hold.instruction_category(), "Timing");

        // Test PHON: communication instruction, slight priority increase
        let phon = Field23E::new("PHON", None).unwrap();
        assert!(phon.is_communication_instruction());
        assert_eq!(phon.priority_impact(), 1);
        assert!(phon.requires_manual_intervention());
        assert!(phon.recommends_additional_info());
        assert_eq!(phon.instruction_category(), "Communication");

        // Test CHQB: payment method, low priority, manual intervention
        let chqb = Field23E::new("CHQB", None).unwrap();
        assert!(chqb.is_payment_method_instruction());
        assert_eq!(chqb.priority_impact(), -1);
        assert!(chqb.requires_manual_intervention());
        assert!(!chqb.recommends_additional_info());
        assert_eq!(chqb.instruction_category(), "Payment Method");
    }

    #[test]
    fn test_field23e_serialization_with_enhanced_fields() {
        let field = Field23E::new("HOLD", Some("COMPLIANCE CHECK".to_string())).unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&field).unwrap();
        let deserialized: Field23E = serde_json::from_str(&json).unwrap();

        assert_eq!(field, deserialized);
        assert_eq!(field.code(), deserialized.code());
        assert_eq!(field.additional_info(), deserialized.additional_info());
        assert_eq!(
            field.instruction_category(),
            deserialized.instruction_category()
        );
        assert_eq!(field.priority_impact(), deserialized.priority_impact());
    }

    #[test]
    fn test_field23e_real_world_scenarios() {
        // Scenario 1: High-value payment requiring verification
        let high_value = Field23E::new("PHON", Some("CALL ALL PARTIES".to_string())).unwrap();
        assert!(high_value.is_communication_instruction());
        assert!(high_value.requires_manual_intervention());
        assert_eq!(high_value.priority_impact(), 1);

        // Scenario 2: Same-day urgent payment
        let urgent = Field23E::new("SDVA", None).unwrap();
        assert!(urgent.is_timing_instruction());
        assert!(!urgent.requires_manual_intervention());
        assert_eq!(urgent.priority_impact(), 2);

        // Scenario 3: Compliance hold with details
        let compliance = Field23E::new("HOLD", Some("AML REVIEW REQUIRED".to_string())).unwrap();
        assert!(compliance.is_timing_instruction());
        assert!(compliance.requires_manual_intervention());
        assert_eq!(compliance.priority_impact(), -2);
        assert!(compliance.recommends_additional_info());

        // Scenario 4: Internal company transfer
        let internal = Field23E::new("INTC", None).unwrap();
        assert_eq!(internal.instruction_category(), "Internal Transfer");
        assert!(!internal.requires_manual_intervention());
        assert_eq!(internal.priority_impact(), 0);
    }

    #[test]
    fn test_field23e_edge_cases_enhanced() {
        // Test all instruction categories are covered
        let all_codes = [
            "CHQB", "HOLD", "INTC", "PHOB", "PHOI", "PHON", "REPA", "SDVA", "TELB", "TELE", "TELI",
        ];
        for code in all_codes {
            let field = Field23E::new(code, None).unwrap();

            // Every code should have a category
            assert!(!field.instruction_category().is_empty());

            // Every code should have a description
            assert!(!field.description().is_empty());

            // Priority impact should be in valid range
            assert!(field.priority_impact() >= -2 && field.priority_impact() <= 2);

            // Priority impact should be reasonable
            assert!(field.priority_impact() >= -2 && field.priority_impact() <= 2);
        }
    }

    #[test]
    fn test_field23e_comprehensive_validation() {
        // Test all valid instruction codes
        for &code in VALID_INSTRUCTION_CODES {
            let field = Field23E::new(code, None).unwrap();
            let validation = field.validate();
            assert!(validation.is_valid, "Code {} should be valid", code);
            assert!(field.is_valid_code());

            // Test with additional info
            let field_with_info = Field23E::new(code, Some("TEST INFO".to_string())).unwrap();
            let validation_with_info = field_with_info.validate();
            assert!(
                validation_with_info.is_valid,
                "Code {} with info should be valid",
                code
            );
        }
    }
}
