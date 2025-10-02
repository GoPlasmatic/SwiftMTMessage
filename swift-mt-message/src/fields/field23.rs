use super::swift_utils::{parse_exact_length, parse_swift_chars, parse_uppercase};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 23: Further Identification**
///
/// ## Purpose
/// Provides additional identification information for financial transactions, particularly
/// in money market and deposit transactions. This field enables precise categorization
/// of transaction types, timing specifications, and reference information essential
/// for proper transaction processing and regulatory compliance.
///
/// ## Format Specification
/// - **Swift Format**: `3!a[2!n]11x`
/// - **Structure**: Function code + optional days + reference information
/// - **Total Length**: Maximum 16 characters
/// - **Components**: Mandatory function code, conditional days, mandatory reference
///
/// ## Business Context Applications
/// - **Money Market Transactions**: Deposit and call money operations
/// - **Treasury Operations**: Interest rate and liquidity management
/// - **Corporate Banking**: Commercial account management
/// - **Settlement Processing**: Transaction categorization and routing
///
/// ## Function Code Categories
/// ### Deposit Operations
/// - **DEPOSIT**: Standard deposit transactions
/// - **NOTICE**: Notice deposit with specific day requirements
/// - **CALL**: Call money transactions (immediate settlement)
/// - **CURRENT**: Current account operations
///
/// ### Commercial Operations
/// - **COMMERCIAL**: Commercial transaction identification
/// - **BASE**: Base rate reference transactions
/// - **PRIME**: Prime rate related operations
///
/// ## Network Validation Requirements
/// - **Function Code**: Must be valid 3-character alphabetic code
/// - **Days Field**: Only required/allowed for NOTICE function code
/// - **Days Range**: 1-99 days when specified for NOTICE transactions
/// - **Reference Format**: Must comply with 11x character set restrictions
/// - **Character Set**: Standard SWIFT character set compliance
///
/// ## Message Type Integration
/// - **MT 200**: Financial institution transfer (function classification)
/// - **MT 202**: General financial institution transfer (operation type)
/// - **MT 210**: Notice to receive (notice period specification)
/// - **Treasury Messages**: Various treasury operations requiring identification
///
/// ## Regional Considerations
/// - **Money Market Standards**: Regional money market convention compliance
/// - **Central Bank Requirements**: Regulatory classification requirements
/// - **Settlement Systems**: Local settlement system integration
/// - **Regulatory Reporting**: Transaction classification for reporting purposes
///
/// ## Validation Logic
/// ### Function Code Rules
/// - **NOTICE**: Requires days field (2!n format, 1-99)
/// - **Other Codes**: Days field must not be present
/// - **Reference**: Always required, 11x format compliance
/// - **Character Validation**: Uppercase alphabetic characters only
///
/// ### Processing Impact
/// - **Settlement Timing**: Function code affects settlement procedures
/// - **Interest Calculation**: Impacts interest computation methods
/// - **Regulatory Classification**: Determines reporting categories
/// - **Risk Assessment**: Influences risk management procedures
///
/// ## Error Prevention Guidelines
/// - **Function Code Verification**: Confirm valid function code selection
/// - **Days Field Logic**: Only include days for NOTICE transactions
/// - **Reference Accuracy**: Ensure reference information is correct
/// - **Format Compliance**: Verify character set and length requirements
///
/// ## Related Fields Integration
/// - **Field 20**: Transaction Reference (transaction context)
/// - **Field 30**: Value Date (timing coordination)
/// - **Field 32A**: Currency/Amount (transaction details)
/// - **Field 52A/D**: Ordering Institution (institutional context)
///
/// ## Compliance Framework
/// - **Money Market Regulations**: Compliance with money market standards
/// - **Central Bank Reporting**: Regulatory classification requirements
/// - **Audit Documentation**: Complete transaction categorization
/// - **Risk Management**: Transaction type classification for risk assessment
///
/// ## Best Practices
/// - **Accurate Classification**: Select appropriate function code
/// - **Notice Period Management**: Proper days specification for NOTICE
/// - **Reference Standards**: Consistent reference information format
/// - **Documentation**: Complete transaction categorization documentation
///
/// ## See Also
/// - Swift FIN User Handbook: Further Identification Field Specifications
/// - Money Market Standards: Function Code Classifications
/// - Treasury Operations Guide: Transaction Identification Best Practices
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field23 {
    /// Function code (3!a format: BASE, CALL, COMMERCIAL, CURRENT, DEPOSIT, NOTICE, PRIME)
    ///
    /// Determines transaction processing type and whether days field is required
    pub function_code: String,

    /// Optional days specification (2!n format, 1-99)
    ///
    /// Only present for NOTICE function code, specifies notice period in days
    pub days: Option<u32>,

    /// Reference information (11x format)
    ///
    /// Additional transaction identification or reference details
    pub reference: String,
}

impl SwiftField for Field23 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.len() < 4 {
            // Minimum: 3 char function code + 1 char reference
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 23 must be at least 4 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse function code (first 3 characters)
        let function_code = parse_exact_length(&input[0..3], 3, "Field 23 function code")?;
        parse_uppercase(&function_code, "Field 23 function code")?;

        // Check if days field is present (next 2 characters could be numeric)
        let (days, reference_start) = if input.len() >= 5 {
            let potential_days = &input[3..5];
            if potential_days.chars().all(|c| c.is_numeric()) {
                let days_value =
                    potential_days
                        .parse::<u32>()
                        .map_err(|_| ParseError::InvalidFormat {
                            message: "Invalid days value in Field 23".to_string(),
                        })?;

                // Validate days range (1-99)
                if days_value == 0 || days_value > 99 {
                    return Err(ParseError::InvalidFormat {
                        message: format!(
                            "Field 23 days must be between 1 and 99, found {}",
                            days_value
                        ),
                    });
                }

                // NOTICE function code requires days field
                if function_code != "NOT" && function_code != "NOTICE" {
                    return Err(ParseError::InvalidFormat {
                        message: format!(
                            "Days field only allowed for NOTICE function code, found {}",
                            function_code
                        ),
                    });
                }

                (Some(days_value), 5)
            } else {
                (None, 3)
            }
        } else {
            (None, 3)
        };

        // Parse reference (remaining characters, max 11)
        let reference = if input.len() > reference_start {
            let ref_str = &input[reference_start..];
            if ref_str.len() > 11 {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 23 reference must be at most 11 characters, found {}",
                        ref_str.len()
                    ),
                });
            }
            parse_swift_chars(ref_str, "Field 23 reference")?;
            ref_str.to_string()
        } else {
            return Err(ParseError::InvalidFormat {
                message: "Field 23 reference is required".to_string(),
            });
        };

        Ok(Field23 {
            function_code,
            days,
            reference,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":23:");
        result.push_str(&self.function_code);
        if let Some(days) = self.days {
            result.push_str(&format!("{:02}", days));
        }
        result.push_str(&self.reference);
        result
    }
}

///   **Field 23B: Bank Operation Code**
///
/// ## Purpose
/// Specifies the bank operation code for payment instructions, determining the service
/// level and processing type for customer credit transfers. This field is crucial for
/// STP (Straight Through Processing) and affects how the payment is processed through
/// the payment chain.
///
/// ## Format
/// - **Swift Format**: `4!c`
/// - **Description**: Exactly 4 uppercase alphabetic characters
/// - **Valid Codes**: CRED, CRTS, SPAY, SPRI, SSTD
///
/// ## Code Definitions
/// - **CRED**: Creditor transfer - Standard credit transfer
/// - **CRTS**: Credit transfer with time criticality
/// - **SPAY**: Priority payment - High priority processing
/// - **SPRI**: Priority payment with immediate processing
/// - **SSTD**: Standard transfer - Normal processing priority
///
/// ## Network Validation Rules
/// - **Format**: Must be exactly 4 alphabetic characters
/// - **Case**: Must be uppercase
/// - **Validity**: Must be one of the defined codes
///
/// ## STP Impact
/// - Determines processing priority and routing
/// - Affects cut-off times and settlement windows
/// - Influences fee structures and service levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field23B {
    /// Bank operation code indicating service level and processing type
    ///
    /// Format: 4!c - Exactly 4 alphabetic characters
    /// Valid codes: CRED, CRTS, SPAY, SPRI, SSTD
    pub instruction_code: String,
}

impl SwiftField for Field23B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Must be exactly 4 characters
        let instruction_code = parse_exact_length(input, 4, "Field 23B instruction code")?;

        // Must be uppercase alphabetic
        parse_uppercase(&instruction_code, "Field 23B instruction code")?;

        // Validate against known codes
        // Common codes: CRED (Customer Transfer), CRTS (Credit Transfer System),
        // SPAY (STP), SPRI (Priority), SSTD (Standard), URGP (Urgent Payment),
        // SDVA (Same Day Value), TELB (Telecommunication Bulk)
        const VALID_CODES: &[&str] = &[
            "CRED", "CRTS", "SPAY", "SPRI", "SSTD", "URGP", "SDVA", "TELB", "PHON", "PHOB", "PHOI",
            "TELE", "REPA", "CORT", "INTC", "HOLD",
        ];
        if !VALID_CODES.contains(&instruction_code.as_str()) {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 23B instruction code must be one of {:?}, found {}",
                    VALID_CODES, instruction_code
                ),
            });
        }

        Ok(Field23B { instruction_code })
    }

    fn to_swift_string(&self) -> String {
        format!(":23B:{}", self.instruction_code)
    }
}

///   **Field 23E: Instruction Code**
///
/// ## Purpose
/// Provides additional instruction codes for payment processing, enabling specific
/// handling instructions and regulatory compliance indicators. Each instruction consists
/// of a 4-character code optionally followed by additional information.
///
/// ## Format
/// - **Swift Format**: `4!c[/35x]`
/// - **Structure**: Instruction code + optional additional information
/// - **Multiple Instructions**: Can contain multiple instruction codes
///
/// ## Common Instruction Codes
/// ### Regulatory Instructions
/// - **CHQB**: Cheque settlement instruction
/// - **CORT**: Corporate trade settlement
/// - **HOLD**: Hold for compliance review
/// - **PHON**: Phone verification completed
/// - **REPA**: Regulatory reporting required
/// - **SDVA**: Same day value adjustment
/// - **TELB**: Telephone initiated transfer
/// - **URGP**: Urgent payment instruction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field23E {
    /// Instruction code (4!c format)
    ///
    /// Specifies the type of instruction for processing
    pub instruction_code: String,

    /// Optional additional information (up to 35 characters)
    ///
    /// Provides supplementary details for the instruction code
    pub additional_info: Option<String>,
}

impl SwiftField for Field23E {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.len() < 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 23E must be at least 4 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse instruction code (first 4 characters)
        let instruction_code = parse_exact_length(&input[0..4], 4, "Field 23E instruction code")?;
        parse_uppercase(&instruction_code, "Field 23E instruction code")?;

        // Check for optional additional information after slash
        let additional_info = if input.len() > 4 {
            if !input[4..].starts_with('/') {
                return Err(ParseError::InvalidFormat {
                    message: "Field 23E additional information must start with '/'".to_string(),
                });
            }

            let info = &input[5..];
            if info.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field 23E additional information must be at most 35 characters, found {}",
                        info.len()
                    ),
                });
            }

            parse_swift_chars(info, "Field 23E additional information")?;
            Some(info.to_string())
        } else {
            None
        };

        Ok(Field23E {
            instruction_code,
            additional_info,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::from(":23E:");
        result.push_str(&self.instruction_code);
        if let Some(ref info) = self.additional_info {
            result.push('/');
            result.push_str(info);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field23() {
        // Test with days field (NOTICE)
        let field = Field23::parse("NOT02REFERENCE1").unwrap();
        assert_eq!(field.function_code, "NOT");
        assert_eq!(field.days, Some(2));
        assert_eq!(field.reference, "REFERENCE1");

        // Test without days field
        let field = Field23::parse("BASREFERENCE").unwrap();
        assert_eq!(field.function_code, "BAS");
        assert_eq!(field.days, None);
        assert_eq!(field.reference, "REFERENCE");

        // Test to_swift_string
        let field = Field23 {
            function_code: "NOT".to_string(),
            days: Some(15),
            reference: "REF123".to_string(),
        };
        assert_eq!(field.to_swift_string(), ":23:NOT15REF123");
    }

    #[test]
    fn test_field23b() {
        // Test valid codes
        let field = Field23B::parse("SSTD").unwrap();
        assert_eq!(field.instruction_code, "SSTD");

        let field = Field23B::parse("SPRI").unwrap();
        assert_eq!(field.instruction_code, "SPRI");

        // Test invalid length
        assert!(Field23B::parse("SST").is_err());
        assert!(Field23B::parse("SSTDD").is_err());

        // Test invalid code
        assert!(Field23B::parse("XXXX").is_err());

        // Test lowercase (should fail)
        assert!(Field23B::parse("sstd").is_err());
    }

    #[test]
    fn test_field23e() {
        // Test with additional info
        let field = Field23E::parse("CHQB/CHECK NUMBER 12345").unwrap();
        assert_eq!(field.instruction_code, "CHQB");
        assert_eq!(
            field.additional_info,
            Some("CHECK NUMBER 12345".to_string())
        );

        // Test without additional info
        let field = Field23E::parse("URGP").unwrap();
        assert_eq!(field.instruction_code, "URGP");
        assert_eq!(field.additional_info, None);

        // Test to_swift_string
        let field = Field23E {
            instruction_code: "HOLD".to_string(),
            additional_info: Some("COMPLIANCE REVIEW".to_string()),
        };
        assert_eq!(field.to_swift_string(), ":23E:HOLD/COMPLIANCE REVIEW");

        // Test max length additional info
        let long_info = "A".repeat(35);
        let input = format!("CODE/{}", long_info);
        assert!(Field23E::parse(&input).is_ok());

        // Test too long additional info
        let too_long = "A".repeat(36);
        let input = format!("CODE/{}", too_long);
        assert!(Field23E::parse(&input).is_err());
    }
}
