//! Field 13C: Time Indication
//!
//! Time at which the transaction should be processed, with UTC offset indication.
//! Format: /8c/4!n1!x4!n (time/sign+offset/sign+offset)

use crate::errors::{FieldParseError, Result, ValidationError};
use crate::field_parser::{FormatRules, SwiftField};
use serde::{Deserialize, Serialize};

/// Field 13C: Time Indication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field13C {
    /// Time portion (format: HHMMSS+DD)
    pub time: String,
    /// UTC offset 1 (format: +HHMM or -HHMM)
    pub utc_offset1: String,
    /// UTC offset 2 (format: +HHMM or -HHMM)
    pub utc_offset2: String,
}

impl Field13C {
    /// Create a new Field13C with validation
    pub fn new(
        time: impl Into<String>,
        utc_offset1: impl Into<String>,
        utc_offset2: impl Into<String>,
    ) -> Result<Self> {
        let time = time.into();
        let utc_offset1 = utc_offset1.into();
        let utc_offset2 = utc_offset2.into();

        // Validate time format (8 characters: HHMMSS+DD)
        if time.len() != 8 {
            return Err(FieldParseError::invalid_format(
                "13C",
                "Time must be exactly 8 characters (HHMMSS+DD)",
            )
            .into());
        }

        // Parse time components
        let hours_str = &time[0..2];
        let minutes_str = &time[2..4];
        let seconds_str = &time[4..6];
        let remainder = &time[6..8];

        // Validate hours (00-23)
        let hours: u32 = hours_str
            .parse()
            .map_err(|_| FieldParseError::invalid_format("13C", "Invalid hours in time portion"))?;
        if hours > 23 {
            return Err(FieldParseError::invalid_format("13C", "Hours must be 00-23").into());
        }

        // Validate minutes (00-59)
        let minutes: u32 = minutes_str.parse().map_err(|_| {
            FieldParseError::invalid_format("13C", "Invalid minutes in time portion")
        })?;
        if minutes > 59 {
            return Err(FieldParseError::invalid_format("13C", "Minutes must be 00-59").into());
        }

        // Validate seconds (00-59)
        let seconds: u32 = seconds_str.parse().map_err(|_| {
            FieldParseError::invalid_format("13C", "Invalid seconds in time portion")
        })?;
        if seconds > 59 {
            return Err(FieldParseError::invalid_format("13C", "Seconds must be 00-59").into());
        }

        // Validate remainder (format: +DD or -DD or similar)
        if !remainder.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(FieldParseError::invalid_format(
                "13C",
                "Invalid characters in time remainder",
            )
            .into());
        }

        // Validate UTC offsets
        Self::validate_utc_offset(&utc_offset1, "UTC offset 1")?;
        Self::validate_utc_offset(&utc_offset2, "UTC offset 2")?;

        Ok(Field13C {
            time,
            utc_offset1: utc_offset1.to_string(),
            utc_offset2: utc_offset2.to_string(),
        })
    }

    /// Validate UTC offset format (+HHMM or -HHMM)
    fn validate_utc_offset(offset: &str, context: &str) -> Result<()> {
        if offset.len() != 5 {
            return Err(FieldParseError::invalid_format(
                "13C",
                &format!("{} must be exactly 5 characters (+HHMM or -HHMM)", context),
            )
            .into());
        }

        let sign = &offset[0..1];
        let hours_str = &offset[1..3];
        let minutes_str = &offset[3..5];

        // Validate sign
        if sign != "+" && sign != "-" {
            return Err(FieldParseError::invalid_format(
                "13C",
                &format!("{} must start with + or -", context),
            )
            .into());
        }

        // Validate hours (00-14 for UTC offset)
        let hours: u32 = hours_str.parse().map_err(|_| {
            FieldParseError::invalid_format("13C", &format!("Invalid hours in {}", context))
        })?;
        if hours > 14 {
            return Err(FieldParseError::invalid_format(
                "13C",
                &format!("Hours in {} must be 00-14", context),
            )
            .into());
        }

        // Validate minutes (00 or 30 for most timezones, 00-59 allowed)
        let minutes: u32 = minutes_str.parse().map_err(|_| {
            FieldParseError::invalid_format("13C", &format!("Invalid minutes in {}", context))
        })?;
        if minutes > 59 {
            return Err(FieldParseError::invalid_format(
                "13C",
                &format!("Minutes in {} must be 00-59", context),
            )
            .into());
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
}

impl SwiftField for Field13C {
    const TAG: &'static str = "13C";

    fn parse(content: &str) -> Result<Self> {
        let content = content.trim();

        if content.is_empty() {
            return Err(
                FieldParseError::missing_data("13C", "Field content cannot be empty").into(),
            );
        }

        // Expected format: /8c/4!n1!x4!n
        // Example: /153045+01/+0100/-0500
        if !content.starts_with('/') {
            return Err(FieldParseError::invalid_format("13C", "Field must start with /").into());
        }

        let parts: Vec<&str> = content[1..].split('/').collect(); // Remove leading '/' and split

        if parts.len() != 3 {
            return Err(FieldParseError::invalid_format(
                "13C",
                "Format must be /time/offset1/offset2 (3 parts separated by /)",
            )
            .into());
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

    fn validate(&self, rules: &FormatRules) -> std::result::Result<(), ValidationError> {
        let content = format!("/{}/{}/{}", self.time, self.utc_offset1, self.utc_offset2);
        rules.validate_field("13C", &content)
    }

    fn description() -> &'static str {
        "Time Indication - Time at which transaction should be processed with UTC offsets"
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
            &self.time[6..8],
            self.utc_offset1,
            self.utc_offset2
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_parser::FormatRules;
    use std::collections::HashMap;

    #[test]
    fn test_field13c_creation() {
        let field = Field13C::new("153045+1", "+0100", "-0500").unwrap();
        assert_eq!(field.time, "153045+1");
        assert_eq!(field.utc_offset1, "+0100");
        assert_eq!(field.utc_offset2, "-0500");
    }

    #[test]
    fn test_field13c_parse() {
        let field = Field13C::parse("/123456+0/+0000/+0200").unwrap();
        assert_eq!(field.time, "123456+0");
        assert_eq!(field.utc_offset1, "+0000");
        assert_eq!(field.utc_offset2, "+0200");
    }

    #[test]
    fn test_field13c_time_validation() {
        // Valid time
        assert!(Field13C::new("235959+D", "+0000", "+0000").is_ok());

        // Invalid hours
        let result = Field13C::new("245959+D", "+0000", "+0000");
        assert!(result.is_err());

        // Invalid minutes
        let result = Field13C::new("236059+D", "+0000", "+0000");
        assert!(result.is_err());

        // Invalid seconds
        let result = Field13C::new("235960+D", "+0000", "+0000");
        assert!(result.is_err());

        // Invalid length
        let result = Field13C::new("1234567", "+0000", "+0000");
        assert!(result.is_err());
    }

    #[test]
    fn test_field13c_utc_offset_validation() {
        // Valid offsets
        assert!(Field13C::new("120000+0", "+0530", "-1200").is_ok());

        // Invalid sign
        let result = Field13C::new("120000+0", "X0530", "+0000");
        assert!(result.is_err());

        // Invalid length
        let result = Field13C::new("120000+0", "+053", "+0000");
        assert!(result.is_err());

        // Invalid hours
        let result = Field13C::new("120000+0", "+1500", "+0000");
        assert!(result.is_err());

        // Invalid minutes
        let result = Field13C::new("120000+0", "+0560", "+0000");
        assert!(result.is_err());
    }

    #[test]
    fn test_field13c_invalid_format() {
        let result = Field13C::parse("/123456+0/+0000"); // Missing third part
        assert!(result.is_err());

        let result = Field13C::parse("123456+0/+0000/+0000"); // Missing leading /
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
        let rules = FormatRules {
            fields: HashMap::new(),
        };
        assert!(field.validate(&rules).is_ok());
    }

    #[test]
    fn test_field13c_display() {
        let field = Field13C::new("143725+D", "+0100", "-0800").unwrap();
        assert_eq!(format!("{}", field), "14:37:25+D +0100 -0800");
    }

    #[test]
    fn test_field13c_accessors() {
        let field = Field13C::new("235959+X", "+1200", "-0700").unwrap();
        assert_eq!(field.time(), "235959+X");
        assert_eq!(field.utc_offset1(), "+1200");
        assert_eq!(field.utc_offset2(), "-0700");
        assert_eq!(field.hours(), 23);
        assert_eq!(field.minutes(), 59);
        assert_eq!(field.seconds(), 59);
    }
}
