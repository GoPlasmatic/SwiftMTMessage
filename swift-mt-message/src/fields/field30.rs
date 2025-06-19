use crate::SwiftField;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// # Field 30: Requested Execution Date
///
/// ## Overview
/// Field 30 contains the date when the credit transfers should be executed by the receiving
/// financial institution. This field is critical for batch payment processing as it specifies
/// the requested execution timing for all transactions within the message, enabling proper
/// scheduling and settlement coordination.
///
/// ## Format Specification
/// **Format**: `6!n`
/// - **6!n**: Date in YYMMDD format (6 numeric characters)
/// - **YY**: Year (2-digit, assumes 20YY for 00-99)
/// - **MM**: Month (01-12)
/// - **DD**: Day (01-31, depending on month)
/// - **Validation**: Must be a valid calendar date
/// - **Business rule**: Should be current or future date, not past
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("6!n")]
pub struct Field30 {
    /// Requested execution date in YYMMDD format
    #[format("6!n")]
    pub date: String,
}

impl Field30 {
    /// Create a new Field30 with specified date string
    pub fn new(date: &str) -> Self {
        Self {
            date: date.to_string(),
        }
    }

    /// Create a new Field30 from NaiveDate
    pub fn from_date(date: NaiveDate) -> Self {
        let date_str = format!(
            "{:02}{:02}{:02}",
            date.year() % 100,
            date.month(),
            date.day()
        );
        Self::new(&date_str)
    }

    /// Create a new Field30 from year, month, day components
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        Some(Self::from_date(date))
    }

    /// Get the execution date as string
    pub fn date(&self) -> &str {
        &self.date
    }

    /// Convert to NaiveDate object
    pub fn to_naive_date(&self) -> Option<NaiveDate> {
        self.parse_date().ok()
    }

    /// Get the year component
    pub fn year(&self) -> i32 {
        self.parse_date().map(|d| d.year()).unwrap_or(0)
    }

    /// Get the month component  
    pub fn month(&self) -> u32 {
        self.parse_date().map(|d| d.month()).unwrap_or(0)
    }

    /// Get the day component
    pub fn day(&self) -> u32 {
        self.parse_date().map(|d| d.day()).unwrap_or(0)
    }

    /// Check if the execution date is today
    pub fn is_today(&self) -> bool {
        if let Ok(date) = self.parse_date() {
            let today = chrono::Utc::now().naive_utc().date();
            date == today
        } else {
            false
        }
    }

    /// Check if the execution date is in the future
    pub fn is_future(&self) -> bool {
        if let Ok(date) = self.parse_date() {
            let today = chrono::Utc::now().naive_utc().date();
            date > today
        } else {
            false
        }
    }

    /// Check if the execution date is in the past
    pub fn is_past(&self) -> bool {
        if let Ok(date) = self.parse_date() {
            let today = chrono::Utc::now().naive_utc().date();
            date < today
        } else {
            false
        }
    }

    /// Check if the execution date is a weekend
    pub fn is_weekend(&self) -> bool {
        if let Ok(date) = self.parse_date() {
            let weekday = date.weekday();
            weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun
        } else {
            false
        }
    }

    /// Get the number of days from today to the execution date
    pub fn days_from_today(&self) -> i64 {
        if let Ok(date) = self.parse_date() {
            let today = chrono::Utc::now().naive_utc().date();
            (date - today).num_days()
        } else {
            0
        }
    }

    /// Format the date in a human-readable format
    pub fn format_readable(&self) -> String {
        if let Ok(date) = self.parse_date() {
            date.format("%Y-%m-%d").to_string()
        } else {
            "Invalid Date".to_string()
        }
    }

    /// Get the execution date as NaiveDate - for MT101 compatibility
    pub fn execution_date(&self) -> NaiveDate {
        self.parse_date()
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
    }

    /// Parse the date string into NaiveDate
    fn parse_date(&self) -> Result<NaiveDate, &'static str> {
        if self.date.len() != 6 {
            return Err("Date must be exactly 6 characters");
        }

        let year_str = &self.date[0..2];
        let month_str = &self.date[2..4];
        let day_str = &self.date[4..6];

        let year: i32 = year_str.parse::<i32>().map_err(|_| "Invalid year format")? + 2000;
        let month: u32 = month_str
            .parse::<u32>()
            .map_err(|_| "Invalid month format")?;
        let day: u32 = day_str.parse::<u32>().map_err(|_| "Invalid day format")?;

        NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")
    }
}
