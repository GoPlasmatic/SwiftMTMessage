//! # Field 13: Time/Date Indication
//!
//! ## Purpose
//! Provides time and date indication capabilities for payment processing with timezone offset information.
//! These fields are critical for time-sensitive payments, settlement timing, and regulatory compliance
//! in cross-border transactions requiring precise timing documentation.
//!
//! ## Options Overview
//! - **Option C**: Time Indication with codes (SNDTIME, CLSTIME, etc.)
//! - **Option D**: Complete Date/Time indication with UTC offset
//!
//! ## Format Specifications
//! ### Option C Format
//! - **Swift Format**: `/8c/4!n1!x4!n`
//! - **Components**: Time indication code + Time + UTC offset sign + offset amount
//!
//! ### Option D Format
//! - **Swift Format**: `6!n4!n1!x4!n`
//! - **Components**: Date + Time + UTC offset sign + offset amount
//!
//! ## Valid Time Indication Codes (Option C)
//! - **CLSTIME**: CLS Time - Funding payment deadline for CLS Bank (CET)
//! - **RNCTIME**: Receive Time - TARGET2 payment credit time at receiving central bank
//! - **SNDTIME**: Send Time - TARGET2 payment debit time at sending central bank
//! - **REJTIME**: Rejection Time - Payment rejection or return timestamp
//! - **CUTTIME**: Cut-off Time - Latest processing time for payments
//!
//! ## Usage Guidelines
//! ### When to Use Option C (Time Indication)
//! - **Settlement Systems**: CLS Bank settlement timing coordination
//! - **TARGET2 Payments**: European central bank payment timing
//! - **Cut-off Management**: Processing deadline specifications
//! - **Event Timing**: Specific time-based payment events
//!
//! ### When to Use Option D (Date/Time Indication)
//! - **Transaction Timestamping**: Precise recording of payment events
//! - **Audit Documentation**: Comprehensive temporal records for compliance
//! - **Cross-Border Coordination**: Time coordination across different time zones
//! - **Regulatory Compliance**: Meeting time-stamping requirements for reporting
//!
//! ## Network Validation Rules
//! - **Time Format**: Must be valid time in HHMM format (00:00 to 23:59)
//! - **Date Format**: Must be valid calendar date in YYMMDD format (Option D)
//! - **Offset Sign**: Must be exactly + (ahead of UTC) or - (behind UTC)
//! - **Offset Range**: UTC offset must be within valid timezone ranges (typically ±13 hours)
//! - **Code Validation**: Time indication codes must be valid and recognized (Option C)
//!
//! ## Business Applications
//! ### Settlement Coordination
//! - **Timezone Management**: UTC offset enables precise time zone identification
//! - **Multiple Indications**: Multiple time indications can be provided in sequences
//! - **Settlement Timing**: Coordination of settlement timing across time zones
//! - **Processing Windows**: Definition of processing cut-off times
//!
//! ### Regulatory Compliance
//! - **Timestamp Requirements**: Meeting regulatory time-stamping requirements
//! - **Audit Trails**: Creating comprehensive timestamp records for compliance
//! - **Cross-Border Rules**: Supporting international payment timing regulations
//! - **System Integration**: Enabling time-aware processing across systems
//!
//! ## Regional Considerations
//! - **European Payments**: CET/CEST timing for TARGET2 and SEPA systems
//! - **US Payments**: EST/EDT timing for Federal Reserve systems
//! - **Asian Markets**: Local time zones for regional clearing systems
//! - **Global Coordination**: UTC reference for international settlements
//!
//! ## Timezone Offset Guidelines
//! - **Standard Offsets**: Most timezone offsets are in full hour increments
//! - **Special Cases**: Some regions use 30 or 45-minute offsets
//! - **Daylight Saving**: Offsets change with daylight saving time transitions
//! - **UTC Reference**: All offsets calculated relative to Coordinated Universal Time
//!
//! ## Error Prevention Guidelines
//! - **Time Validation**: Ensure time is valid 24-hour format
//! - **Date Validation**: Verify date is valid and within reasonable business range
//! - **Offset Accuracy**: Confirm timezone offset matches actual timezone
//! - **Code Selection**: Use appropriate time indication code for context (Option C)
//! - **Business Logic**: Ensure timing aligns with settlement windows and business processes
//!
//! ## Related Fields Integration
//! - **Field 32A**: Value Date (settlement date coordination)
//! - **Field 30**: Execution Date (date-only specifications)
//! - **Field 72**: Sender to Receiver Information (additional timing details)
//! - **Block Headers**: Message timestamps (system-level timing)
//!
//! ## Technical Implementation
//! - **Date Handling**: Uses `chrono::NaiveDate` for robust date parsing
//! - **Time Handling**: Uses `chrono::NaiveTime` for time validation
//! - **Offset Storage**: String format for flexible UTC offset representation
//! - **Validation**: Automatic format validation through Swift macro system
//!
//! ## See Also
//! - Swift FIN User Handbook: Date/Time Specifications
//! - ISO 8601: International Date/Time Standards
//! - SWIFT Network Rules: Timestamp Requirements
//! - Regional Payment Guides: Local Time Zone Considerations

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
/// Time indication variant of [Field 13 module](index.html). Specifies time indication related
/// to payment processing with timezone offset and specific timing codes.
///
/// **Components:**
/// - Time indication code (/8c/, e.g., SNDTIME, CLSTIME)
/// - Time (4!n, HHMM format)
/// - UTC offset sign (1!x, + or -)
/// - UTC offset amount (4!n, HHMM format)
///
/// For complete documentation, see the [Field 13 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field13C {
    /// Time indication code with slashes
    ///
    /// Format: /8c/ - Valid codes: SNDTIME, CLSTIME, RNCTIME, REJTIME, CUTTIME
    /// Specifies the type of time being indicated
    pub code: String,

    /// Time in 24-hour format
    ///
    /// Format: 4!n (HHMM) - Must be valid time (00:00 to 23:59)
    /// Represents the actual time for the indicated event
    #[serde(with = "time_format")]
    pub time: NaiveTime,

    /// Timezone offset sign
    ///
    /// Format: 1!x - Must be + (ahead of UTC) or - (behind UTC)
    /// Indicates direction of timezone offset from UTC
    pub sign: char,

    /// Timezone offset amount
    ///
    /// Format: 4!n (HHMM) - Hours (00-13), Minutes (00-59)
    /// Specifies the offset from UTC time
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
            code: format!("/{}/", code),
            time,
            sign: sign_char,
            offset,
        })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":13C:{}{}{}{}",
            self.code,
            self.time.format("%H%M"),
            self.sign,
            self.offset
        )
    }
}

/// **Field 13D: Date/Time Indication**
///
/// Complete date/time variant of [Field 13 module](index.html). Specifies complete date and time
/// indication with UTC offset for precise timestamp documentation.
///
/// **Components:**
/// - Date (6!n, YYMMDD format)
/// - Time (4!n, HHMM format)
/// - UTC offset sign (1!x, + or -)
/// - UTC offset amount (4!n)
///
/// For complete documentation, see the [Field 13 module](index.html).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field13D {
    /// Date component in YYMMDD format
    ///
    /// Format: 6!n - Must be valid calendar date
    /// Used for complete date specification with time
    #[serde(with = "date_format")]
    pub date: NaiveDate,

    /// Time component in HHMM format
    ///
    /// Format: 4!n - 24-hour clock (00:00 to 23:59)
    /// Provides precise time specification
    #[serde(with = "time_format")]
    pub time: NaiveTime,

    /// UTC offset direction sign
    ///
    /// Format: 1!x - Must be + (ahead of UTC) or - (behind UTC)
    /// Indicates timezone relationship to UTC
    pub offset_sign: char,

    /// UTC offset amount
    ///
    /// Format: 4!n - HHMM format for hours and minutes
    /// Specifies the numerical offset from UTC
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
        assert_eq!(field.code, "/SNDTIME/");
        assert_eq!(field.time.format("%H%M").to_string(), "1230");
        assert_eq!(field.sign, '+');
        assert_eq!(field.offset, "0100");
        assert_eq!(field.to_swift_string(), "/SNDTIME/1230+0100");

        let field = Field13C::parse("/CLSTIME/0900-0500").unwrap();
        assert_eq!(field.code, "/CLSTIME/");
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
        assert_eq!(field.to_swift_string(), "2407191230+0100");

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
