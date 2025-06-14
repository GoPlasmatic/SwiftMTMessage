use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 13C: Time Indication
///
/// ## Overview
/// Field 13C specifies time indication(s) related to the processing of payment instructions
/// in SWIFT MT messages. This field is used to indicate specific timing requirements or
/// constraints for transaction processing, particularly in time-sensitive payment scenarios.
///
/// ## Format Specification
/// **Format**: `/8c/4!n1!x4!n`
/// - **8c**: Time portion (8 characters: HHMMSS + 2 additional characters)
/// - **4!n1!x4!n**: UTC offset (4 digits + sign + 4 digits, format: ±HHMM)
/// - **4!n1!x4!n**: Second UTC offset (4 digits + sign + 4 digits, format: ±HHMM)
///
/// ## Structure
/// ```text
/// /HHMMSS+D/±HHMM/±HHMM
/// │└─────┘│ │    │ │
/// │  Time │ │    │ └── Second UTC offset
/// │       │ │    └──── First UTC offset  
/// │       │ └───────── Time remainder/indicator
/// │       └─────────── Time (HHMMSS)
/// └─────────────────── Field delimiter
/// ```
///
/// ## Time Codes and Indicators
/// The time portion consists of:
/// - **HH**: Hours (00-23)
/// - **MM**: Minutes (00-59)
/// - **SS**: Seconds (00-59)
/// - **+D**: Additional indicator (varies by usage context)
///
/// Common time indicators include:
/// - `+0`, `+1`, `+2`, etc.: Numeric indicators
/// - `+D`: Day indicator
/// - `+X`: Special processing indicator
///
/// ## UTC Offset Format
/// UTC offsets must follow the format `±HHMM`:
/// - **±**: Plus (+) or minus (-) sign
/// - **HH**: Hours offset from UTC (00-14)
/// - **MM**: Minutes offset (00-59, typically 00 or 30)
///
/// ## Usage Context
/// Field 13C is commonly used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
///
/// ### Business Applications
/// - **Cut-off times**: Specify latest processing time
/// - **Value dating**: Indicate when funds should be available
/// - **Time zone coordination**: Handle cross-border payments
/// - **STP processing**: Ensure straight-through processing timing
///
/// ## Examples
/// ```text
/// :13C:/CLSTIME/153045+1/+0100/-0500
/// └─── CLS Bank cut-off time at 15:30:45+1, CET (+0100), EST (-0500)
///
/// :13C:/RNCTIME/090000+0/+0000/+0900  
/// └─── TARGET receive time at 09:00:00, UTC (+0000), JST (+0900)
///
/// :13C:/SNDTIME/235959+D/+0200/-0800
/// └─── Send time at 23:59:59+D, CEST (+0200), PST (-0800)
/// ```
///
/// ## Validation Rules
/// 1. **Time format**: Must be exactly 8 characters (HHMMSS + 2 chars)
/// 2. **Time values**: Hours (00-23), Minutes (00-59), Seconds (00-59)
/// 3. **UTC offsets**: Must be ±HHMM format with valid ranges
/// 4. **Structure**: Must have exactly 3 parts separated by '/'
/// 5. **Leading slash**: Field content must start with '/'
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Time indication must be a valid time expressed as HHMM (Error: T38)
/// - Sign must be either "+" or "-" (Error: T15)
/// - Time offset hours must be 00-13, minutes 00-59 (Error: T16)
/// - Format must comply with SWIFT field specifications
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field13C {
    /// Time portion (8 characters: HHMMSS + 2 additional characters)
    ///
    /// Format: HHMMSS+D where:
    /// - HH: Hours (00-23)
    /// - MM: Minutes (00-59)
    /// - SS: Seconds (00-59)
    /// - +D: Additional indicator (context-dependent)
    ///
    /// Examples: "153045+1", "090000+0", "235959+D"
    pub time: String,

    /// First UTC offset (format: ±HHMM)
    ///
    /// Represents the UTC offset for the first timezone reference.
    /// Format: ±HHMM where:
    /// - ±: Plus (+) or minus (-) sign
    /// - HH: Hours offset from UTC (00-14)
    /// - MM: Minutes offset (00-59, typically 00 or 30)
    ///
    /// Examples: "+0100" (CET), "-0500" (EST), "+0000" (UTC)
    pub utc_offset1: String,

    /// Second UTC offset (format: ±HHMM)
    ///
    /// Represents the UTC offset for the second timezone reference.
    /// Used for cross-timezone coordination or dual time indication.
    /// Same format as utc_offset1.
    ///
    /// Examples: "-0800" (PST), "+0900" (JST), "+0530" (IST)
    pub utc_offset2: String,
}

impl SwiftField for Field13C {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":13C:") {
            stripped // Remove ":13C:" prefix
        } else if let Some(stripped) = value.strip_prefix("13C:") {
            stripped // Remove "13C:" prefix
        } else {
            value
        };

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Field content cannot be empty after removing tag".to_string(),
            });
        }

        // Expected format: /8c/4!n1!x4!n
        // Example: /153045+1/+0100/-0500
        if !content.starts_with('/') {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Field must start with /".to_string(),
            });
        }

        let parts: Vec<&str> = content[1..].split('/').collect(); // Remove leading '/' and split

        if parts.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Format must be /time/offset1/offset2 (3 parts separated by /)"
                    .to_string(),
            });
        }

        let time = parts[0].to_string();
        let utc_offset1 = parts[1].to_string();
        let utc_offset2 = parts[2].to_string();

        Self::new(time, utc_offset1, utc_offset2)
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":13C:/{}/{}/{}",
            self.time, self.utc_offset1, self.utc_offset2
        )
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate time format (8 characters: HHMMSS+DD)
        if self.time.len() != 8 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "13C".to_string(),
                expected: "8 characters".to_string(),
                actual: self.time.len(),
            });
        } else {
            // Parse time components
            let hours_str = &self.time[0..2];
            let minutes_str = &self.time[2..4];
            let seconds_str = &self.time[4..6];
            let remainder = &self.time[6..8];

            // Validate hours (00-23)
            if let Ok(hours) = hours_str.parse::<u32>() {
                if hours > 23 {
                    errors.push(ValidationError::ValueValidation {
                        field_tag: "13C".to_string(),
                        message: "Hours must be 00-23".to_string(),
                    });
                }
            } else {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "13C".to_string(),
                    message: "Invalid hours in time portion".to_string(),
                });
            }

            // Validate minutes (00-59)
            if let Ok(minutes) = minutes_str.parse::<u32>() {
                if minutes > 59 {
                    errors.push(ValidationError::ValueValidation {
                        field_tag: "13C".to_string(),
                        message: "Minutes must be 00-59".to_string(),
                    });
                }
            } else {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "13C".to_string(),
                    message: "Invalid minutes in time portion".to_string(),
                });
            }

            // Validate seconds (00-59)
            if let Ok(seconds) = seconds_str.parse::<u32>() {
                if seconds > 59 {
                    errors.push(ValidationError::ValueValidation {
                        field_tag: "13C".to_string(),
                        message: "Seconds must be 00-59".to_string(),
                    });
                }
            } else {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "13C".to_string(),
                    message: "Invalid seconds in time portion".to_string(),
                });
            }

            // Validate remainder (format: +DD or -DD or similar)
            if !remainder.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "13C".to_string(),
                    message: "Invalid characters in time remainder".to_string(),
                });
            }
        }

        // Validate UTC offsets
        if let Err(e) = Self::validate_utc_offset(&self.utc_offset1, "UTC offset 1") {
            errors.push(ValidationError::FormatValidation {
                field_tag: "13C".to_string(),
                message: format!("UTC offset 1 validation failed: {}", e),
            });
        }

        if let Err(e) = Self::validate_utc_offset(&self.utc_offset2, "UTC offset 2") {
            errors.push(ValidationError::FormatValidation {
                field_tag: "13C".to_string(),
                message: format!("UTC offset 2 validation failed: {}", e),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "/8c/4!n1!x4!n"
    }
}

impl Field13C {
    /// Create a new Field13C with comprehensive validation
    ///
    /// # Arguments
    /// * `time` - Time portion (8 characters: HHMMSS + 2 additional chars)
    /// * `utc_offset1` - First UTC offset (±HHMM format)
    /// * `utc_offset2` - Second UTC offset (±HHMM format)
    ///
    /// # Examples
    /// ```rust
    /// use swift_mt_message::fields::Field13C;
    ///
    /// // CLS Bank cut-off time
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    ///
    /// // TARGET processing time
    /// let field = Field13C::new("090000+0", "+0000", "+0900").unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns `ParseError` if:
    /// - Time is not exactly 8 characters
    /// - Hours, minutes, or seconds are out of valid range
    /// - UTC offsets are not in ±HHMM format
    /// - UTC offset values are out of valid range
    pub fn new(
        time: impl Into<String>,
        utc_offset1: impl Into<String>,
        utc_offset2: impl Into<String>,
    ) -> crate::Result<Self> {
        let time = time.into().trim().to_string();
        let utc_offset1 = utc_offset1.into().trim().to_string();
        let utc_offset2 = utc_offset2.into().trim().to_string();

        // Validate time format (8 characters: HHMMSS+DD)
        if time.len() != 8 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Time must be exactly 8 characters (HHMMSS+DD)".to_string(),
            });
        }

        // Parse time components
        let hours_str = &time[0..2];
        let minutes_str = &time[2..4];
        let seconds_str = &time[4..6];
        let remainder = &time[6..8];

        // Validate hours (00-23)
        let hours: u32 = hours_str
            .parse()
            .map_err(|_| crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Invalid hours in time portion".to_string(),
            })?;
        if hours > 23 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Hours must be 00-23".to_string(),
            });
        }

        // Validate minutes (00-59)
        let minutes: u32 =
            minutes_str
                .parse()
                .map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "13C".to_string(),
                    message: "Invalid minutes in time portion".to_string(),
                })?;
        if minutes > 59 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Minutes must be 00-59".to_string(),
            });
        }

        // Validate seconds (00-59)
        let seconds: u32 =
            seconds_str
                .parse()
                .map_err(|_| crate::ParseError::InvalidFieldFormat {
                    field_tag: "13C".to_string(),
                    message: "Invalid seconds in time portion".to_string(),
                })?;
        if seconds > 59 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Seconds must be 00-59".to_string(),
            });
        }

        // Validate remainder (format: +DD or -DD or similar)
        if !remainder.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Invalid characters in time remainder".to_string(),
            });
        }

        // Validate UTC offsets
        Self::validate_utc_offset(&utc_offset1, "UTC offset 1").map_err(|msg| {
            crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: format!("UTC offset 1 validation failed: {}", msg),
            }
        })?;

        Self::validate_utc_offset(&utc_offset2, "UTC offset 2").map_err(|msg| {
            crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: format!("UTC offset 2 validation failed: {}", msg),
            }
        })?;

        Ok(Field13C {
            time,
            utc_offset1,
            utc_offset2,
        })
    }

    /// Validate UTC offset format according to SWIFT standards
    ///
    /// Validates that the UTC offset follows the ±HHMM format with realistic values:
    /// - Sign must be + or -
    /// - Hours must be 00-14 (covers all real-world timezones)
    /// - Minutes must be 00-59 (typically 00, 15, 30, or 45)
    ///
    /// # Arguments
    /// * `offset` - The UTC offset string to validate
    /// * `context` - Description for error messages
    ///
    /// # Returns
    /// `Ok(())` if valid, `Err(String)` with error description if invalid
    fn validate_utc_offset(offset: &str, context: &str) -> Result<(), String> {
        if offset.len() != 5 {
            return Err(format!(
                "{} must be exactly 5 characters (+HHMM or -HHMM)",
                context
            ));
        }

        let sign = &offset[0..1];
        let hours_str = &offset[1..3];
        let minutes_str = &offset[3..5];

        // Validate sign
        if sign != "+" && sign != "-" {
            return Err(format!("{} must start with + or -", context));
        }

        // Validate hours (00-14 for UTC offset)
        let hours: u32 = hours_str
            .parse()
            .map_err(|_| format!("Invalid hours in {}", context))?;
        if hours > 14 {
            return Err(format!("Hours in {} must be 00-14", context));
        }

        // Validate minutes (00 or 30 for most timezones, 00-59 allowed)
        let minutes: u32 = minutes_str
            .parse()
            .map_err(|_| format!("Invalid minutes in {}", context))?;
        if minutes > 59 {
            return Err(format!("Minutes in {} must be 00-59", context));
        }

        Ok(())
    }

    /// Get the complete time portion
    ///
    /// Returns the full 8-character time string including the time indicator.
    ///
    /// # Returns
    /// The time string in format HHMMSS+D
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// assert_eq!(field.time(), "153045+1");
    /// ```
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Get the first UTC offset
    ///
    /// Returns the first UTC offset in ±HHMM format.
    ///
    /// # Returns
    /// The first UTC offset string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// assert_eq!(field.utc_offset1(), "+0100");
    /// ```
    pub fn utc_offset1(&self) -> &str {
        &self.utc_offset1
    }

    /// Get the second UTC offset
    ///
    /// Returns the second UTC offset in ±HHMM format.
    ///
    /// # Returns
    /// The second UTC offset string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// assert_eq!(field.utc_offset2(), "-0500");
    /// ```
    pub fn utc_offset2(&self) -> &str {
        &self.utc_offset2
    }

    /// Extract hours from the time portion
    ///
    /// Parses and returns the hours component (00-23) from the time string.
    ///
    /// # Returns
    /// Hours as u32, or 0 if parsing fails
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// assert_eq!(field.hours(), 15);
    /// ```
    pub fn hours(&self) -> u32 {
        self.time[0..2].parse().unwrap_or(0)
    }

    /// Extract minutes from the time portion
    ///
    /// Parses and returns the minutes component (00-59) from the time string.
    ///
    /// # Returns
    /// Minutes as u32, or 0 if parsing fails
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// assert_eq!(field.minutes(), 30);
    /// ```
    pub fn minutes(&self) -> u32 {
        self.time[2..4].parse().unwrap_or(0)
    }

    /// Extract seconds from the time portion
    ///
    /// Parses and returns the seconds component (00-59) from the time string.
    ///
    /// # Returns
    /// Seconds as u32, or 0 if parsing fails
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// assert_eq!(field.seconds(), 45);
    /// ```
    pub fn seconds(&self) -> u32 {
        self.time[4..6].parse().unwrap_or(0)
    }

    /// Get the time remainder/indicator
    ///
    /// Returns the last 2 characters of the time string, which typically
    /// contain additional time indicators or processing codes.
    ///
    /// # Returns
    /// The time remainder string (2 characters)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// assert_eq!(field.time_remainder(), "+1");
    /// ```
    pub fn time_remainder(&self) -> &str {
        &self.time[6..8]
    }

    /// Check if this is a CLS Bank time indication
    ///
    /// Determines if this field represents a CLS Bank cut-off time
    /// based on common patterns and indicators.
    ///
    /// # Returns
    /// `true` if this appears to be a CLS time indication
    pub fn is_cls_time(&self) -> bool {
        // CLS times often use specific indicators
        matches!(self.time_remainder(), "+1" | "+0" | "+C")
    }

    /// Check if this is a TARGET system time indication
    ///
    /// Determines if this field represents a TARGET (Trans-European Automated
    /// Real-time Gross Settlement Express Transfer) system time indication.
    ///
    /// # Returns
    /// `true` if this appears to be a TARGET time indication
    pub fn is_target_time(&self) -> bool {
        // TARGET times often use +0 indicator and CET timezone
        self.time_remainder() == "+0"
            && (self.utc_offset1 == "+0100" || self.utc_offset1 == "+0200")
    }

    /// Get a human-readable description of the time indication
    ///
    /// Returns a descriptive string explaining what this time indication
    /// represents based on common SWIFT usage patterns.
    ///
    /// # Returns
    /// A descriptive string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field13C;
    /// let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
    /// println!("{}", field.description());
    /// ```
    pub fn description(&self) -> String {
        let time_desc = if self.is_target_time() {
            "TARGET system time"
        } else if self.is_cls_time() {
            "CLS Bank cut-off time"
        } else {
            "Time indication"
        };

        format!(
            "{} at {:02}:{:02}:{:02}{} (UTC{}/UTC{})",
            time_desc,
            self.hours(),
            self.minutes(),
            self.seconds(),
            self.time_remainder(),
            self.utc_offset1,
            self.utc_offset2
        )
    }
}

impl std::fmt::Display for Field13C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}{} {} {}",
            self.hours(),
            self.minutes(),
            self.seconds(),
            self.time_remainder(),
            self.utc_offset1,
            self.utc_offset2
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field13c_creation() {
        let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
        assert_eq!(field.time(), "153045+1");
        assert_eq!(field.utc_offset1(), "+0100");
        assert_eq!(field.utc_offset2(), "-0500");
    }

    #[test]
    fn test_field13c_parse() {
        let field = Field13C::parse("/123456+0/+0000/+0200").unwrap();
        assert_eq!(field.time, "123456+0");
        assert_eq!(field.utc_offset1, "+0000");
        assert_eq!(field.utc_offset2, "+0200");
    }

    #[test]
    fn test_field13c_parse_with_prefix() {
        let field = Field13C::parse(":13C:/235959+D/+0000/+0530").unwrap();
        assert_eq!(field.time, "235959+D");
        assert_eq!(field.utc_offset1, "+0000");
        assert_eq!(field.utc_offset2, "+0530");

        let field = Field13C::parse("13C:/090000+X/-0300/+0900").unwrap();
        assert_eq!(field.time, "090000+X");
        assert_eq!(field.utc_offset1, "-0300");
        assert_eq!(field.utc_offset2, "+0900");
    }

    #[test]
    fn test_field13c_case_normalization() {
        // Time field is not case-normalized, it's kept as-is
        let field = Field13C::new("120000+d", "+0100", "-0500").unwrap();
        assert_eq!(field.time, "120000+d");
    }

    #[test]
    fn test_field13c_invalid_time_code() {
        let result = Field13C::new("1234567", "+0100", "-0500"); // Too short
        assert!(result.is_err());

        let result = Field13C::new("123456789", "+0100", "-0500"); // Too long
        assert!(result.is_err());

        let result = Field13C::new("245959+D", "+0100", "-0500"); // Invalid hours
        assert!(result.is_err());

        let result = Field13C::new("236059+D", "+0100", "-0500"); // Invalid minutes
        assert!(result.is_err());

        let result = Field13C::new("235960+D", "+0100", "-0500"); // Invalid seconds
        assert!(result.is_err());
    }

    #[test]
    fn test_field13c_invalid_utc_offset() {
        let result = Field13C::new("120000+0", "0100", "-0500"); // Missing sign
        assert!(result.is_err());

        let result = Field13C::new("120000+0", "+25000", "-0500"); // Too long
        assert!(result.is_err());

        let result = Field13C::new("120000+0", "+1500", "-0500"); // Invalid hours
        assert!(result.is_err());

        let result = Field13C::new("120000+0", "+0160", "-0500"); // Invalid minutes
        assert!(result.is_err());
    }

    #[test]
    fn test_field13c_invalid_format() {
        let result = Field13C::parse("123456+0/+0100/-0500"); // Missing leading /
        assert!(result.is_err());

        let result = Field13C::parse("/123456+0/+0100"); // Missing second offset
        assert!(result.is_err());

        let result = Field13C::parse("/123456+0/+0100/-0500/extra"); // Too many parts
        assert!(result.is_err());
    }

    #[test]
    fn test_field13c_to_swift_string() {
        let field = Field13C::new("143725+2", "+0100", "-0800").unwrap();
        assert_eq!(field.to_swift_string(), ":13C:/143725+2/+0100/-0800");
    }

    #[test]
    fn test_field13c_validation() {
        let field = Field13C::new("090000+0", "+0000", "+0000").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let invalid_field = Field13C {
            time: "1234567".to_string(), // Invalid length
            utc_offset1: "+0100".to_string(),
            utc_offset2: "-0500".to_string(),
        };
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field13c_format_spec() {
        assert_eq!(Field13C::format_spec(), "/8c/4!n1!x4!n");
    }

    #[test]
    fn test_field13c_display() {
        let field = Field13C::new("143725+D", "+0100", "-0800").unwrap();
        assert_eq!(format!("{}", field), "14:37:25+D +0100 -0800");
    }

    #[test]
    fn test_field13c_descriptions() {
        let field = Field13C::new("235959+X", "+1200", "-0700").unwrap();
        assert_eq!(field.time(), "235959+X");
        assert_eq!(field.utc_offset1(), "+1200");
        assert_eq!(field.utc_offset2(), "-0700");
        assert_eq!(field.hours(), 23);
        assert_eq!(field.minutes(), 59);
        assert_eq!(field.seconds(), 59);
        assert_eq!(field.time_remainder(), "+X");
    }

    #[test]
    fn test_field13c_is_valid_time_code() {
        // Test time parsing methods
        let field = Field13C::new("143725+2", "+0100", "-0500").unwrap();
        assert_eq!(field.hours(), 14);
        assert_eq!(field.minutes(), 37);
        assert_eq!(field.seconds(), 25);
        assert_eq!(field.time_remainder(), "+2");
    }

    #[test]
    fn test_field13c_cls_time_detection() {
        // Test CLS Bank time detection
        let cls_field1 = Field13C::new("153045+1", "+0100", "-0500").unwrap();
        assert!(cls_field1.is_cls_time());

        let cls_field2 = Field13C::new("090000+0", "+0000", "+0900").unwrap();
        assert!(cls_field2.is_cls_time());

        let cls_field3 = Field13C::new("120000+C", "+0200", "-0800").unwrap();
        assert!(cls_field3.is_cls_time());

        let non_cls_field = Field13C::new("143725+D", "+0100", "-0800").unwrap();
        assert!(!non_cls_field.is_cls_time());
    }

    #[test]
    fn test_field13c_target_time_detection() {
        // Test TARGET system time detection
        let target_field1 = Field13C::new("090000+0", "+0100", "+0900").unwrap();
        assert!(target_field1.is_target_time());

        let target_field2 = Field13C::new("160000+0", "+0200", "-0500").unwrap();
        assert!(target_field2.is_target_time());

        let non_target_field1 = Field13C::new("090000+1", "+0100", "+0900").unwrap();
        assert!(!non_target_field1.is_target_time());

        let non_target_field2 = Field13C::new("090000+0", "+0000", "+0900").unwrap();
        assert!(!non_target_field2.is_target_time());
    }

    #[test]
    fn test_field13c_description_generation() {
        // Test CLS Bank description
        let cls_field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
        let description = cls_field.description();
        assert!(description.contains("CLS Bank cut-off time"));
        assert!(description.contains("15:30:45+1"));
        assert!(description.contains("UTC+0100"));
        assert!(description.contains("UTC-0500"));

        // Test TARGET system description
        let target_field = Field13C::new("090000+0", "+0100", "+0900").unwrap();
        let description = target_field.description();
        assert!(description.contains("TARGET system time"));
        assert!(description.contains("09:00:00+0"));

        // Test generic time indication
        let generic_field = Field13C::new("143725+D", "+0200", "-0800").unwrap();
        let description = generic_field.description();
        assert!(description.contains("Time indication"));
        assert!(description.contains("14:37:25+D"));
    }

    #[test]
    fn test_field13c_real_world_examples() {
        // CLS Bank cut-off time example
        let cls_example = Field13C::new("153045+1", "+0100", "-0500").unwrap();
        assert_eq!(cls_example.to_swift_string(), ":13C:/153045+1/+0100/-0500");
        assert!(cls_example.is_cls_time());
        assert!(!cls_example.is_target_time());

        // TARGET system time example
        let target_example = Field13C::new("090000+0", "+0100", "+0900").unwrap();
        assert_eq!(
            target_example.to_swift_string(),
            ":13C:/090000+0/+0100/+0900"
        );
        assert!(target_example.is_target_time());
        assert!(target_example.is_cls_time()); // +0 is also a CLS indicator

        // Generic processing time
        let generic_example = Field13C::new("235959+D", "+0200", "-0800").unwrap();
        assert_eq!(
            generic_example.to_swift_string(),
            ":13C:/235959+D/+0200/-0800"
        );
        assert!(!generic_example.is_cls_time());
        assert!(!generic_example.is_target_time());
    }

    #[test]
    fn test_field13c_edge_cases() {
        // Test midnight
        let midnight = Field13C::new("000000+0", "+0000", "+0000").unwrap();
        assert_eq!(midnight.hours(), 0);
        assert_eq!(midnight.minutes(), 0);
        assert_eq!(midnight.seconds(), 0);

        // Test end of day
        let end_of_day = Field13C::new("235959+X", "+1400", "-1200").unwrap();
        assert_eq!(end_of_day.hours(), 23);
        assert_eq!(end_of_day.minutes(), 59);
        assert_eq!(end_of_day.seconds(), 59);

        // Test extreme timezone offsets
        let extreme_positive = Field13C::new("120000+Z", "+1400", "+1200").unwrap();
        assert_eq!(extreme_positive.utc_offset1(), "+1400");

        let extreme_negative = Field13C::new("120000+A", "-1200", "-1100").unwrap();
        assert_eq!(extreme_negative.utc_offset1(), "-1200");
    }

    #[test]
    fn test_field13c_serialization() {
        let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&field).unwrap();
        let deserialized: Field13C = serde_json::from_str(&json).unwrap();

        assert_eq!(field, deserialized);
        assert_eq!(field.time(), deserialized.time());
        assert_eq!(field.utc_offset1(), deserialized.utc_offset1());
        assert_eq!(field.utc_offset2(), deserialized.utc_offset2());
    }
}
