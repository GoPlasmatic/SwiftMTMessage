use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// # Field 13D: Date/Time Indication
///
/// Used in MT900, MT910, MT941, MT942 to indicate the date and time with UTC offset.
///
/// ## Format
/// `6!n4!n1!x4!n` (YYMMDDhhmm±hhmm)
///
/// Where:
/// - `YYMMDD`: Date (year, month, day)
/// - `hhmm`: Time (hours, minutes in 24-hour format)
/// - `±`: Sign (+ or -)
/// - `hhmm`: UTC offset (hours, minutes)
///
/// ## Example Usage
/// ```rust
/// # use swift_mt_message::fields::Field13D;
/// let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
/// assert_eq!(field.to_swift_string(), "2403151430+0530");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Field13D {
    /// Date in YYMMDD format
    pub date: String,
    /// Time in hhmm format (24-hour)
    pub time: String,
    /// UTC offset sign (+ or -)
    pub offset_sign: String,
    /// UTC offset in hhmm format
    pub offset_time: String,
}

impl Field13D {
    /// Creates a new Field13D with validation
    ///
    /// # Arguments
    /// * `date` - Date in YYMMDD format
    /// * `time` - Time in hhmm format (24-hour)
    /// * `offset_sign` - UTC offset sign ("+" or "-")
    /// * `offset_time` - UTC offset in hhmm format
    ///
    /// # Returns
    /// * `Ok(Field13D)` if all components are valid
    /// * `Err(String)` if validation fails
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field13D;
    /// let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
    /// assert_eq!(field.date, "240315");
    /// assert_eq!(field.time, "1430");
    /// ```
    pub fn new(
        date: &str,
        time: &str,
        offset_sign: &str,
        offset_time: &str,
    ) -> Result<Self, String> {
        // Validate date (YYMMDD)
        if date.len() != 6 || !date.chars().all(|c| c.is_ascii_digit()) {
            return Err("Date must be exactly 6 digits in YYMMDD format".to_string());
        }

        let month = &date[2..4];
        let day = &date[4..6];

        // Basic month validation
        let month_num: u8 = month.parse().map_err(|_| "Invalid month")?;
        if !(1..=12).contains(&month_num) {
            return Err("Month must be between 01 and 12".to_string());
        }

        // Basic day validation
        let day_num: u8 = day.parse().map_err(|_| "Invalid day")?;
        if !(1..=31).contains(&day_num) {
            return Err("Day must be between 01 and 31".to_string());
        }

        // Validate time (hhmm)
        if time.len() != 4 || !time.chars().all(|c| c.is_ascii_digit()) {
            return Err("Time must be exactly 4 digits in hhmm format".to_string());
        }

        let hour = &time[0..2];
        let minute = &time[2..4];

        let hour_num: u8 = hour.parse().map_err(|_| "Invalid hour")?;
        if hour_num > 23 {
            return Err("Hour must be between 00 and 23".to_string());
        }

        let minute_num: u8 = minute.parse().map_err(|_| "Invalid minute")?;
        if minute_num > 59 {
            return Err("Minute must be between 00 and 59".to_string());
        }

        // Validate offset sign
        if offset_sign != "+" && offset_sign != "-" {
            return Err("Offset sign must be '+' or '-'".to_string());
        }

        // Validate offset time (hhmm)
        if offset_time.len() != 4 || !offset_time.chars().all(|c| c.is_ascii_digit()) {
            return Err("Offset time must be exactly 4 digits in hhmm format".to_string());
        }

        let offset_hour = &offset_time[0..2];
        let offset_minute = &offset_time[2..4];

        let offset_hour_num: u8 = offset_hour.parse().map_err(|_| "Invalid offset hour")?;
        if offset_hour_num > 14 {
            return Err("Offset hour must be between 00 and 14".to_string());
        }

        let offset_minute_num: u8 = offset_minute.parse().map_err(|_| "Invalid offset minute")?;
        if offset_minute_num > 59 {
            return Err("Offset minute must be between 00 and 59".to_string());
        }

        // Validate total offset is within ±14:00
        let total_offset_minutes = offset_hour_num as i32 * 60 + offset_minute_num as i32;
        if total_offset_minutes > 14 * 60 {
            return Err("UTC offset cannot exceed ±14:00".to_string());
        }

        Ok(Field13D {
            date: date.to_string(),
            time: time.to_string(),
            offset_sign: offset_sign.to_string(),
            offset_time: offset_time.to_string(),
        })
    }

    /// Parses Field13D from a SWIFT message string
    ///
    /// # Arguments
    /// * `input` - The input string to parse (14 characters: YYMMDDhhmm±hhmm)
    ///
    /// # Returns
    /// * `Ok(Field13D)` if parsing succeeds
    /// * `Err(String)` if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field13D;
    /// let field = Field13D::parse("2403151430+0530").unwrap();
    /// assert_eq!(field.date, "240315");
    /// assert_eq!(field.time, "1430");
    /// assert_eq!(field.offset_sign, "+");
    /// assert_eq!(field.offset_time, "0530");
    /// ```
    pub fn parse(input: &str) -> Result<Self, String> {
        let cleaned = input
            .trim()
            .strip_prefix(":13D:")
            .or_else(|| input.strip_prefix("13D:"))
            .unwrap_or(input);

        if cleaned.len() != 15 {
            return Err("Field13D must be exactly 15 characters (YYMMDDhhmm±hhmm)".to_string());
        }

        let date = &cleaned[0..6];
        let time = &cleaned[6..10];
        let offset_sign = &cleaned[10..11];
        let offset_time = &cleaned[11..15];

        Self::new(date, time, offset_sign, offset_time)
    }

    /// Converts the field to its SWIFT string representation
    ///
    /// # Returns
    /// The field formatted for SWIFT messages
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field13D;
    /// let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
    /// assert_eq!(field.to_swift_string(), "2403151430+0530");
    /// ```
    pub fn to_swift_string(&self) -> String {
        format!(
            "{}{}{}{}",
            self.date, self.time, self.offset_sign, self.offset_time
        )
    }

    /// Returns the SWIFT field format specification
    ///
    /// # Returns
    /// The format specification string
    pub fn format_spec() -> &'static str {
        "6!n4!n1!x4!n"
    }

    /// Gets the date component as a formatted string
    ///
    /// # Returns
    /// Date in YY/MM/DD format
    pub fn get_formatted_date(&self) -> String {
        format!(
            "{}/{}/{}",
            &self.date[0..2],
            &self.date[2..4],
            &self.date[4..6]
        )
    }

    /// Gets the time component as a formatted string
    ///
    /// # Returns
    /// Time in HH:MM format
    pub fn get_formatted_time(&self) -> String {
        format!("{}:{}", &self.time[0..2], &self.time[2..4])
    }

    /// Gets the UTC offset as a formatted string
    ///
    /// # Returns
    /// UTC offset in ±HH:MM format
    pub fn get_formatted_offset(&self) -> String {
        format!(
            "{}{}:{}",
            self.offset_sign,
            &self.offset_time[0..2],
            &self.offset_time[2..4]
        )
    }

    /// Returns the complete datetime with offset as a formatted string
    ///
    /// # Returns
    /// Complete datetime in YY/MM/DD HH:MM ±HH:MM format
    pub fn get_formatted_datetime(&self) -> String {
        format!(
            "{} {} {}",
            self.get_formatted_date(),
            self.get_formatted_time(),
            self.get_formatted_offset()
        )
    }

    /// Validates the field according to SWIFT standards
    ///
    /// # Returns
    /// `true` if the field is valid, `false` otherwise
    pub fn is_valid(&self) -> bool {
        // Re-validate all components
        self.date.len() == 6
            && self.date.chars().all(|c| c.is_ascii_digit())
            && self.time.len() == 4
            && self.time.chars().all(|c| c.is_ascii_digit())
            && (self.offset_sign == "+" || self.offset_sign == "-")
            && self.offset_time.len() == 4
            && self.offset_time.chars().all(|c| c.is_ascii_digit())
    }
}

impl SwiftField for Field13D {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":13D:") {
            stripped // Remove ":13D:" prefix
        } else if let Some(stripped) = value.strip_prefix("13D:") {
            stripped // Remove "13D:" prefix
        } else {
            value
        };

        let cleaned = content.trim();

        if cleaned.len() != 15 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: "Field13D must be exactly 15 characters (YYMMDDhhmm±hhmm)".to_string(),
            });
        }

        let date = &cleaned[0..6];
        let time = &cleaned[6..10];
        let offset_sign = &cleaned[10..11];
        let offset_time = &cleaned[11..15];

        Self::new(date, time, offset_sign, offset_time).map_err(|e| {
            crate::ParseError::InvalidFieldFormat {
                field_tag: "13D".to_string(),
                message: e,
            }
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":13D:{}{}{}{}",
            self.date, self.time, self.offset_sign, self.offset_time
        )
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate date (YYMMDD)
        if self.date.len() != 6 || !self.date.chars().all(|c| c.is_ascii_digit()) {
            errors.push(ValidationError::ValueValidation {
                field_tag: "13D".to_string(),
                message: "Date must be exactly 6 digits in YYMMDD format".to_string(),
            });
        }

        // Validate time (hhmm)
        if self.time.len() != 4 || !self.time.chars().all(|c| c.is_ascii_digit()) {
            errors.push(ValidationError::ValueValidation {
                field_tag: "13D".to_string(),
                message: "Time must be exactly 4 digits in hhmm format".to_string(),
            });
        }

        // Validate offset sign
        if self.offset_sign != "+" && self.offset_sign != "-" {
            errors.push(ValidationError::ValueValidation {
                field_tag: "13D".to_string(),
                message: "UTC offset sign must be + or -".to_string(),
            });
        }

        // Validate offset time
        if self.offset_time.len() != 4 || !self.offset_time.chars().all(|c| c.is_ascii_digit()) {
            errors.push(ValidationError::ValueValidation {
                field_tag: "13D".to_string(),
                message: "UTC offset time must be exactly 4 digits in hhmm format".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "6!n4!n1!x4!n"
    }
}

impl fmt::Display for Field13D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Field13D: {} {} {}{}",
            self.get_formatted_date(),
            self.get_formatted_time(),
            self.offset_sign,
            &self.offset_time
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field13d_creation_valid() {
        let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
        assert_eq!(field.date, "240315");
        assert_eq!(field.time, "1430");
        assert_eq!(field.offset_sign, "+");
        assert_eq!(field.offset_time, "0530");
        assert!(field.is_valid());
    }

    #[test]
    fn test_field13d_creation_negative_offset() {
        let field = Field13D::new("240315", "0900", "-", "0500").unwrap();
        assert_eq!(field.offset_sign, "-");
        assert_eq!(field.get_formatted_offset(), "-05:00");
    }

    #[test]
    fn test_field13d_invalid_date() {
        let result = Field13D::new("24031", "1430", "+", "0530");
        assert!(result.is_err());

        let result = Field13D::new("24abc1", "1430", "+", "0530");
        assert!(result.is_err());

        let result = Field13D::new("241315", "1430", "+", "0530"); // Invalid month
        assert!(result.is_err());

        let result = Field13D::new("240132", "1430", "+", "0530"); // Invalid day
        assert!(result.is_err());
    }

    #[test]
    fn test_field13d_invalid_time() {
        let result = Field13D::new("240315", "143", "+", "0530");
        assert!(result.is_err());

        let result = Field13D::new("240315", "2430", "+", "0530"); // Invalid hour
        assert!(result.is_err());

        let result = Field13D::new("240315", "1460", "+", "0530"); // Invalid minute
        assert!(result.is_err());
    }

    #[test]
    fn test_field13d_invalid_offset() {
        let result = Field13D::new("240315", "1430", "x", "0530");
        assert!(result.is_err());

        let result = Field13D::new("240315", "1430", "+", "053");
        assert!(result.is_err());

        let result = Field13D::new("240315", "1430", "+", "1500"); // Invalid offset > 14:00
        assert!(result.is_err());
    }

    #[test]
    fn test_field13d_parse() {
        let field = Field13D::parse("2403151430+0530").unwrap();
        assert_eq!(field.date, "240315");
        assert_eq!(field.time, "1430");
        assert_eq!(field.offset_sign, "+");
        assert_eq!(field.offset_time, "0530");

        let field = Field13D::parse(":13D:2403150900-0500").unwrap();
        assert_eq!(field.offset_sign, "-");
        assert_eq!(field.offset_time, "0500");
    }

    #[test]
    fn test_field13d_to_swift_string() {
        let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
        assert_eq!(field.to_swift_string(), "2403151430+0530");
    }

    #[test]
    fn test_field13d_formatting() {
        let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
        assert_eq!(field.get_formatted_date(), "24/03/15");
        assert_eq!(field.get_formatted_time(), "14:30");
        assert_eq!(field.get_formatted_offset(), "+05:30");
        assert_eq!(field.get_formatted_datetime(), "24/03/15 14:30 +05:30");
    }

    #[test]
    fn test_field13d_format_spec() {
        assert_eq!(Field13D::format_spec(), "6!n4!n1!x4!n");
    }

    #[test]
    fn test_field13d_display() {
        let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
        let display = format!("{}", field);
        assert!(display.contains("24/03/15"));
        assert!(display.contains("14:30"));
        assert!(display.contains("+0530"));
    }

    #[test]
    fn test_field13d_serialization() {
        let field = Field13D::new("240315", "1430", "+", "0530").unwrap();
        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: Field13D = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field, deserialized);
    }

    #[test]
    fn test_field13d_edge_cases() {
        // Test midnight
        let field = Field13D::new("240315", "0000", "+", "0000").unwrap();
        assert_eq!(field.get_formatted_time(), "00:00");

        // Test end of day
        let field = Field13D::new("240315", "2359", "-", "1400").unwrap();
        assert_eq!(field.get_formatted_time(), "23:59");
        assert_eq!(field.get_formatted_offset(), "-14:00");
    }
}
