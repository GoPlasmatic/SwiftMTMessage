use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// Field 13C: Time Indication
///
/// Format: /8c/4!n1!x4!n (time/sign+offset/sign+offset)
///
/// Time at which the transaction should be processed, with UTC offset indication.
/// Example: /153045+1/+0100/-0500
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field13C {
    /// Time portion (format: HHMMSS+DD, 8 characters)
    pub time: String,

    /// First UTC offset (format: +HHMM or -HHMM)
    pub utc_offset1: String,

    /// Second UTC offset (format: +HHMM or -HHMM)
    pub utc_offset2: String,
}

impl SwiftField for Field13C {
    fn parse(value: &str) -> crate::Result<Self> {
        let value = value.trim();

        if value.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13C".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        // Handle input that includes field tag prefix (e.g., ":13C:/153045+1/+0100/-0500")
        let content = if value.starts_with(":13C:") {
            &value[5..] // Remove ":13C:" prefix
        } else if value.starts_with("13C:") {
            &value[4..] // Remove "13C:" prefix
        } else {
            value // Use as-is if no prefix
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
    /// Create a new Field13C with validation
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

    /// Validate UTC offset format (+HHMM or -HHMM)
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

    /// Get the time portion
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Get the first UTC offset
    pub fn utc_offset1(&self) -> &str {
        &self.utc_offset1
    }

    /// Get the second UTC offset
    pub fn utc_offset2(&self) -> &str {
        &self.utc_offset2
    }

    /// Parse hours from time
    pub fn hours(&self) -> u32 {
        self.time[0..2].parse().unwrap_or(0)
    }

    /// Parse minutes from time
    pub fn minutes(&self) -> u32 {
        self.time[2..4].parse().unwrap_or(0)
    }

    /// Parse seconds from time
    pub fn seconds(&self) -> u32 {
        self.time[4..6].parse().unwrap_or(0)
    }

    /// Get the time remainder (last 2 characters)
    pub fn time_remainder(&self) -> &str {
        &self.time[6..8]
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
}
