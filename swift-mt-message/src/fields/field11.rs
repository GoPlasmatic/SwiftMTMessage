use super::swift_utils::parse_swift_digits;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// Field 11R: MT Reference (Option R)
/// Used in acknowledgment and response messages to reference the original message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field11R {
    /// Message type of the original message being referenced (3!n)
    pub message_type: String,

    /// Date of the original message in YYMMDD format (6!n)
    #[serde(with = "date_string")]
    pub date: NaiveDate,

    /// Optional session number (4!n)
    pub session_number: Option<String>,

    /// Optional input sequence number (6!n)
    pub input_sequence_number: Option<String>,
}

impl SwiftField for Field11R {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut remaining = input;

        // Parse message type (3!n)
        if remaining.len() < 3 {
            return Err(ParseError::InvalidFormat {
                message: "Field 11R message type requires exactly 3 digits".to_string(),
            });
        }
        let message_type = parse_swift_digits(&remaining[..3], "Field 11R message type")?;
        remaining = &remaining[3..];

        // Parse date (6!n for YYMMDD)
        if remaining.len() < 6 {
            return Err(ParseError::InvalidFormat {
                message: "Field 11R date requires exactly 6 digits".to_string(),
            });
        }
        let date_str = parse_swift_digits(&remaining[..6], "Field 11R date")?;
        remaining = &remaining[6..];

        // Parse date
        let year = 2000
            + date_str[0..2]
                .parse::<i32>()
                .map_err(|_| ParseError::InvalidFormat {
                    message: "Invalid year in Field 11R".to_string(),
                })?;
        let month = date_str[2..4]
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidFormat {
                message: "Invalid month in Field 11R".to_string(),
            })?;
        let day = date_str[4..6]
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidFormat {
                message: "Invalid day in Field 11R".to_string(),
            })?;

        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| ParseError::InvalidFormat {
                message: format!("Invalid date in Field 11R: {}", date_str),
            })?;

        // Parse optional session number (4!n)
        let session_number =
            if remaining.len() >= 4 && remaining[..4].chars().all(|c| c.is_ascii_digit()) {
                let session = Some(remaining[..4].to_string());
                remaining = &remaining[4..];
                session
            } else {
                None
            };

        // Parse optional input sequence number (6!n)
        let input_sequence_number =
            if remaining.len() >= 6 && remaining[..6].chars().all(|c| c.is_ascii_digit()) {
                Some(remaining[..6].to_string())
            } else {
                None
            };

        Ok(Field11R {
            message_type,
            date,
            session_number,
            input_sequence_number,
        })
    }

    fn to_swift_string(&self) -> String {
        let date_str = format!(
            "{:02}{:02}{:02}",
            self.date.year() % 100,
            self.date.month(),
            self.date.day()
        );

        let mut result = format!(":11R:{}{}", self.message_type, date_str);

        if let Some(ref session) = self.session_number {
            result.push_str(session);
        }

        if let Some(ref seq) = self.input_sequence_number {
            result.push_str(seq);
        }

        result
    }
}

/// Field 11S: MT Reference (Option S)
/// Used in cancellation requests and status inquiry messages for transaction control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field11S {
    /// Message type of the original message being referenced (3!n)
    pub message_type: String,

    /// Date of the original message in YYMMDD format (6!n)
    #[serde(with = "date_string")]
    pub date: NaiveDate,

    /// Optional session number (4!n)
    pub session_number: Option<String>,

    /// Optional input sequence number (6!n)
    pub input_sequence_number: Option<String>,
}

// Custom serialization for dates as strings
mod date_string {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map_err(serde::de::Error::custom)
    }
}

impl SwiftField for Field11S {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut remaining = input;

        // Parse message type (3!n)
        if remaining.len() < 3 {
            return Err(ParseError::InvalidFormat {
                message: "Field 11S message type requires exactly 3 digits".to_string(),
            });
        }
        let message_type = parse_swift_digits(&remaining[..3], "Field 11S message type")?;
        remaining = &remaining[3..];

        // Parse date (6!n for YYMMDD)
        if remaining.len() < 6 {
            return Err(ParseError::InvalidFormat {
                message: "Field 11S date requires exactly 6 digits".to_string(),
            });
        }
        let date_str = parse_swift_digits(&remaining[..6], "Field 11S date")?;
        remaining = &remaining[6..];

        // Parse date
        let year = 2000
            + date_str[0..2]
                .parse::<i32>()
                .map_err(|_| ParseError::InvalidFormat {
                    message: "Invalid year in Field 11S".to_string(),
                })?;
        let month = date_str[2..4]
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidFormat {
                message: "Invalid month in Field 11S".to_string(),
            })?;
        let day = date_str[4..6]
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidFormat {
                message: "Invalid day in Field 11S".to_string(),
            })?;

        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| ParseError::InvalidFormat {
                message: format!("Invalid date in Field 11S: {}", date_str),
            })?;

        // Parse optional session number (4!n)
        let session_number =
            if remaining.len() >= 4 && remaining[..4].chars().all(|c| c.is_ascii_digit()) {
                let session = Some(remaining[..4].to_string());
                remaining = &remaining[4..];
                session
            } else {
                None
            };

        // Parse optional input sequence number (6!n)
        let input_sequence_number =
            if remaining.len() >= 6 && remaining[..6].chars().all(|c| c.is_ascii_digit()) {
                Some(remaining[..6].to_string())
            } else {
                None
            };

        Ok(Field11S {
            message_type,
            date,
            session_number,
            input_sequence_number,
        })
    }

    fn to_swift_string(&self) -> String {
        let date_str = format!(
            "{:02}{:02}{:02}",
            self.date.year() % 100,
            self.date.month(),
            self.date.day()
        );

        let mut result = format!(":11S:{}{}", self.message_type, date_str);

        if let Some(ref session) = self.session_number {
            result.push_str(session);
        }

        if let Some(ref seq) = self.input_sequence_number {
            result.push_str(seq);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field11r_parse() {
        // Test with all components
        let field = Field11R::parse("1032407191234567890").unwrap();
        assert_eq!(field.message_type, "103");
        assert_eq!(field.date.year(), 2024);
        assert_eq!(field.date.month(), 7);
        assert_eq!(field.date.day(), 19);
        assert_eq!(field.session_number, Some("1234".to_string()));
        assert_eq!(field.input_sequence_number, Some("567890".to_string()));

        // Test without optional components
        let field = Field11R::parse("202240315").unwrap();
        assert_eq!(field.message_type, "202");
        assert_eq!(field.date.year(), 2024);
        assert_eq!(field.date.month(), 3);
        assert_eq!(field.date.day(), 15);
        assert_eq!(field.session_number, None);
        assert_eq!(field.input_sequence_number, None);

        // Test with session number only
        let field = Field11R::parse("9402407191234").unwrap();
        assert_eq!(field.message_type, "940");
        assert_eq!(field.session_number, Some("1234".to_string()));
        assert_eq!(field.input_sequence_number, None);
    }

    #[test]
    fn test_field11s_parse() {
        // Test with all components
        let field = Field11S::parse("1922407191234567890").unwrap();
        assert_eq!(field.message_type, "192");
        assert_eq!(field.date.year(), 2024);
        assert_eq!(field.date.month(), 7);
        assert_eq!(field.date.day(), 19);
        assert_eq!(field.session_number, Some("1234".to_string()));
        assert_eq!(field.input_sequence_number, Some("567890".to_string()));

        // Test without optional components
        let field = Field11S::parse("292240315").unwrap();
        assert_eq!(field.message_type, "292");
        assert_eq!(field.date.year(), 2024);
        assert_eq!(field.date.month(), 3);
        assert_eq!(field.date.day(), 15);
        assert_eq!(field.session_number, None);
        assert_eq!(field.input_sequence_number, None);
    }

    #[test]
    fn test_field11r_to_swift_string() {
        let field = Field11R {
            message_type: "103".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
            session_number: Some("1234".to_string()),
            input_sequence_number: Some("567890".to_string()),
        };
        assert_eq!(field.to_swift_string(), ":11R:1032407191234567890");

        let field = Field11R {
            message_type: "202".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            session_number: None,
            input_sequence_number: None,
        };
        assert_eq!(field.to_swift_string(), ":11R:202240315");
    }

    #[test]
    fn test_field11s_to_swift_string() {
        let field = Field11S {
            message_type: "192".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
            session_number: Some("1234".to_string()),
            input_sequence_number: Some("567890".to_string()),
        };
        assert_eq!(field.to_swift_string(), ":11S:1922407191234567890");

        let field = Field11S {
            message_type: "292".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            session_number: None,
            input_sequence_number: None,
        };
        assert_eq!(field.to_swift_string(), ":11S:292240315");
    }
}

/// Field 11: MT and Date of Original Message
/// Used in MT196 and other messages to reference the message type and date of the original message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field11 {
    /// Message type of the original message being referenced (3!n)
    pub message_type: String,

    /// Date of the original message in YYMMDD format (6!n)
    pub date: NaiveDate,
}

impl SwiftField for Field11 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Field 11 requires at least 9 characters (3 for MT + 6 for date)
        if input.len() < 9 {
            return Err(ParseError::InvalidFormat {
                message: "Field 11 requires at least 9 characters (3 for MT + 6 for date)".to_string(),
            });
        }

        // Parse message type (3!n)
        let message_type = parse_swift_digits(&input[..3], "Field 11 message type")?;

        // Parse date (6!n for YYMMDD)
        let date_str = parse_swift_digits(&input[3..9], "Field 11 date")?;

        // Parse date
        let year = 2000
            + date_str[0..2]
                .parse::<i32>()
                .map_err(|_| ParseError::InvalidFormat {
                    message: "Invalid year in Field 11".to_string(),
                })?;
        let month = date_str[2..4]
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidFormat {
                message: "Invalid month in Field 11".to_string(),
            })?;
        let day = date_str[4..6]
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidFormat {
                message: "Invalid day in Field 11".to_string(),
            })?;

        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| ParseError::InvalidFormat {
                message: format!("Invalid date in Field 11: {}", date_str),
            })?;

        Ok(Field11 {
            message_type,
            date,
        })
    }

    fn to_swift_string(&self) -> String {
        let date_str = format!(
            "{:02}{:02}{:02}",
            self.date.year() % 100,
            self.date.month(),
            self.date.day()
        );
        format!(":11:{}{}", self.message_type, date_str)
    }
}

#[cfg(test)]
mod field11_tests {
    use super::*;

    #[test]
    fn test_field11_parse() {
        let field = Field11::parse("192240719").unwrap();
        assert_eq!(field.message_type, "192");
        assert_eq!(field.date.year(), 2024);
        assert_eq!(field.date.month(), 7);
        assert_eq!(field.date.day(), 19);
    }

    #[test]
    fn test_field11_to_swift_string() {
        let field = Field11 {
            message_type: "196".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
        };
        assert_eq!(field.to_swift_string(), ":11:196240719");
    }
}
