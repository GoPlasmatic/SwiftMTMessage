//! **Field 13: Time/Date Indication**
//!
//! Provides time and date indication with timezone offset for time-sensitive payment processing and settlement timing.

use super::swift_utils::{parse_date_yymmdd, parse_exact_length, parse_numeric, parse_time_hhmm};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

/// Helper module for serializing/deserializing NaiveTime as HHMM string
mod time_format {
    use chrono::{NaiveTime, Timelike};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:02}{:02}", time.hour(), time.minute());
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() != 4 {
            return Err(serde::de::Error::custom("Time must be 4 digits (HHMM)"));
        }
        let hours: u32 = s[0..2].parse().map_err(serde::de::Error::custom)?;
        let minutes: u32 = s[2..4].parse().map_err(serde::de::Error::custom)?;

        NaiveTime::from_hms_opt(hours, minutes, 0)
            .ok_or_else(|| serde::de::Error::custom(format!("Invalid time: {}:{}", hours, minutes)))
    }
}

/// Helper module for serializing/deserializing NaiveDate as YYMMDD string
mod date_format {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format("%y%m%d").to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() != 6 {
            return Err(serde::de::Error::custom("Date must be 6 digits (YYMMDD)"));
        }

        let year: i32 = s[0..2].parse::<i32>().map_err(serde::de::Error::custom)?;
        let year = if year >= 80 { 1900 + year } else { 2000 + year };
        let month: u32 = s[2..4].parse().map_err(serde::de::Error::custom)?;
        let day: u32 = s[4..6].parse().map_err(serde::de::Error::custom)?;

        NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid date: {}/{}/{}", year, month, day))
        })
    }
}

/// **Field 13C: Time Indication**
///
/// Specifies time indication with timezone offset for payment processing events.
///
/// **Format:** `/8c/4!n1!x4!n` (code + time + offset sign + offset)
/// **Valid Codes:** SNDTIME, CLSTIME, RNCTIME, REJTIME, CUTTIME
///
/// **Example:**
/// ```text
/// :13C:/SNDTIME/1230+0100
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field13C {
    /// Time indication code (SNDTIME, CLSTIME, etc.)
    pub code: String,

    /// Time in HHMM format (24-hour)
    #[serde(with = "time_format")]
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub time: NaiveTime,

    /// UTC offset sign (+ or -)
    pub sign: char,

    /// UTC offset in HHMM format
    pub offset: String,
}

impl SwiftField for Field13C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Minimum: /8c/4!n1!x4!n = / + 8 + / + 4 + 1 + 4 = 18 chars minimum
        if input.len() < 10 {
            // At minimum we need /X/ + time + sign + offset
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13C must be at least 10 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse time indication code (must be between slashes)
        if !input.starts_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 13C code must start with '/'".to_string(),
            });
        }

        let end_slash = input[1..].find('/').ok_or(ParseError::InvalidFormat {
            message: "Field 13C code must be enclosed in slashes".to_string(),
        })? + 1;

        if end_slash < 2 {
            return Err(ParseError::InvalidFormat {
                message: "Field 13C code cannot be empty".to_string(),
            });
        }

        let code = input[1..end_slash].to_string();

        // Validate against known codes
        const VALID_CODES: &[&str] = &["SNDTIME", "CLSTIME", "RNCTIME", "REJTIME", "CUTTIME"];
        if !VALID_CODES.contains(&code.as_str()) {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13C code must be one of {:?}, found {}",
                    VALID_CODES, code
                ),
            });
        }

        let remaining = &input[end_slash + 1..];
        if remaining.len() != 9 {
            // 4 (time) + 1 (sign) + 4 (offset)
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13C after code must be exactly 9 characters, found {}",
                    remaining.len()
                ),
            });
        }

        // Parse time (4 digits)
        let time_str = &remaining[0..4];
        parse_numeric(time_str, "Field 13C time")?;
        let time = parse_time_hhmm(time_str)?;

        // Parse UTC offset sign
        let sign_char = remaining.chars().nth(4).unwrap();
        if sign_char != '+' && sign_char != '-' {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13C UTC offset sign must be '+' or '-', found '{}'",
                    sign_char
                ),
            });
        }

        // Parse offset (4 digits)
        let offset = parse_exact_length(&remaining[5..9], 4, "Field 13C offset")?;
        parse_numeric(&offset, "Field 13C offset")?;

        // Validate offset is reasonable (up to 14 hours)
        let offset_hours: u32 = offset[0..2].parse().unwrap();
        let offset_minutes: u32 = offset[2..4].parse().unwrap();
        if offset_hours > 14 || offset_minutes > 59 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13C offset must be valid time offset, found {}:{}",
                    offset_hours, offset_minutes
                ),
            });
        }

        Ok(Field13C {
            code,
            time,
            sign: sign_char,
            offset,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":13C:/{}/{}{}{}",
            self.code,
            self.time.format("%H%M"),
            self.sign,
            self.offset
        )
    }
}

/// **Field 13D: Date/Time Indication**
///
/// Complete date and time indication with timezone offset for precise timestamping.
///
/// **Format:** `6!n4!n1!x4!n` (date + time + offset sign + offset)
///
/// **Example:**
/// ```text
/// :13D:2407191230+0100
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field13D {
    /// Date in YYMMDD format
    #[serde(with = "date_format")]
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub date: NaiveDate,

    /// Time in HHMM format (24-hour)
    #[serde(with = "time_format")]
    #[cfg_attr(feature = "jsonschema", schemars(with = "String"))]
    pub time: NaiveTime,

    /// UTC offset sign (+ or -)
    pub offset_sign: char,

    /// UTC offset in HHMM format
    pub offset: String,
}

impl SwiftField for Field13D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Must be exactly 15 characters: 6 (date) + 4 (time) + 1 (sign) + 4 (offset)
        if input.len() != 15 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13D must be exactly 15 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse date (first 6 digits)
        let date = parse_date_yymmdd(&input[0..6])?;

        // Parse time (next 4 digits)
        let time_str = &input[6..10];
        parse_numeric(time_str, "Field 13D time")?;
        let time = parse_time_hhmm(time_str)?;

        // Parse UTC offset sign
        let offset_sign = input.chars().nth(10).unwrap();
        if offset_sign != '+' && offset_sign != '-' {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13D UTC offset sign must be '+' or '-', found '{}'",
                    offset_sign
                ),
            });
        }

        // Parse offset (last 4 digits)
        let offset = parse_exact_length(&input[11..15], 4, "Field 13D offset")?;
        parse_numeric(&offset, "Field 13D offset")?;

        // Validate offset is reasonable
        let offset_hours: u32 = offset[0..2].parse().unwrap();
        let offset_minutes: u32 = offset[2..4].parse().unwrap();
        if offset_hours > 14 || offset_minutes > 59 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 13D offset must be valid time offset, found {}:{}",
                    offset_hours, offset_minutes
                ),
            });
        }

        Ok(Field13D {
            date,
            time,
            offset_sign,
            offset,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":13D:{}{}{}{}",
            self.date.format("%y%m%d"),
            self.time.format("%H%M"),
            self.offset_sign,
            self.offset
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field13c_valid() {
        let field = Field13C::parse("/SNDTIME/1230+0100").unwrap();
        assert_eq!(field.code, "SNDTIME");
        assert_eq!(field.time.format("%H%M").to_string(), "1230");
        assert_eq!(field.sign, '+');
        assert_eq!(field.offset, "0100");
        assert_eq!(field.to_swift_string(), ":13C:/SNDTIME/1230+0100");

        let field = Field13C::parse("/CLSTIME/0900-0500").unwrap();
        assert_eq!(field.code, "CLSTIME");
        assert_eq!(field.time.format("%H%M").to_string(), "0900");
        assert_eq!(field.sign, '-');
        assert_eq!(field.offset, "0500");
    }

    #[test]
    fn test_field13c_invalid() {
        // Missing slashes
        assert!(Field13C::parse("SNDTIME1230+0100").is_err());

        // Invalid code
        assert!(Field13C::parse("/BADCODE/1230+0100").is_err());

        // Invalid time
        assert!(Field13C::parse("/SNDTIME/2500+0100").is_err());

        // Invalid sign
        assert!(Field13C::parse("/SNDTIME/1230*0100").is_err());

        // Invalid offset
        assert!(Field13C::parse("/SNDTIME/1230+2500").is_err());

        // Wrong length
        assert!(Field13C::parse("/SNDTIME/1230+01").is_err());
    }

    #[test]
    fn test_field13d_valid() {
        let field = Field13D::parse("2407191230+0100").unwrap();
        assert_eq!(field.date.format("%y%m%d").to_string(), "240719");
        assert_eq!(field.time.format("%H%M").to_string(), "1230");
        assert_eq!(field.offset_sign, '+');
        assert_eq!(field.offset, "0100");
        assert_eq!(field.to_swift_string(), ":13D:2407191230+0100");

        let field = Field13D::parse("2412310000-0800").unwrap();
        assert_eq!(field.date.format("%y%m%d").to_string(), "241231");
        assert_eq!(field.time.format("%H%M").to_string(), "0000");
        assert_eq!(field.offset_sign, '-');
        assert_eq!(field.offset, "0800");
    }

    #[test]
    fn test_field13d_invalid() {
        // Wrong length
        assert!(Field13D::parse("2407191230+01").is_err());
        assert!(Field13D::parse("2407191230+010000").is_err());

        // Invalid date
        assert!(Field13D::parse("9913321230+0100").is_err());

        // Invalid time
        assert!(Field13D::parse("2407192500+0100").is_err());

        // Invalid sign
        assert!(Field13D::parse("2407191230*0100").is_err());

        // Invalid offset
        assert!(Field13D::parse("2407191230+2500").is_err());
    }
}
