use super::swift_utils::{parse_exact_length, parse_swift_chars, parse_uppercase};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 23: Further Identification**
///
/// Transaction categorization for money market and deposit operations.
///
/// **Format:** `3!a[2!n]11x` (function code + optional days + reference)
/// **Function codes:** BASE, CALL, COMMERCIAL, CURRENT, DEPOSIT, NOTICE, PRIME
/// **Days field:** Required only for NOTICE (1-99)
///
/// **Example:**
/// ```text
/// :23:NOT15REF123
/// :23:BASREFERENCE
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field23 {
    /// Function code (3 chars, uppercase)
    pub function_code: String,
    /// Days (1-99, only for NOTICE)
    pub days: Option<u32>,
    /// Reference (max 11 chars)
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

/// **Field 23B: Bank Operation Code**
///
/// Service level and processing type for payment instructions.
/// Affects STP routing, priority, and settlement timing.
///
/// **Format:** `4!c` (exactly 4 uppercase chars)
/// **Common codes:** CRED, CRTS, SPAY, SPRI, SSTD, URGP
///
/// **Example:**
/// ```text
/// :23B:SSTD
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field23B {
    /// Bank operation code (4 chars, uppercase)
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

/// **Field 23E: Instruction Code**
///
/// Additional processing instructions and compliance indicators.
///
/// **Format:** `4!c[/35x]` (code + optional details)
/// **Common codes:** CHQB, CORT, HOLD, PHON, REPA, SDVA, TELB, URGP
///
/// **Example:**
/// ```text
/// :23E:HOLD/COMPLIANCE REVIEW
/// :23E:URGP
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field23E {
    /// Instruction code (4 chars, uppercase)
    pub instruction_code: String,
    /// Additional info (max 35 chars)
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
