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
#[validation_rules(date_valid = true, date_reasonable = true)]
#[business_logic(date_analysis = true, timing_analysis = true)]
pub struct Field30 {
    #[component("6!n", parser = "swift_date")]
    pub date: NaiveDate,
}

impl Field30 {
    /// Create a new Field30 from NaiveDate
    pub fn from_date(date: NaiveDate) -> Self {
        Self { date }
    }

    /// Create a new Field30 from year, month, day components
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        Some(Self::from_date(date))
    }

    /// Create a new Field30 with specified date string (for compatibility)
    pub fn new(date_str: &str) -> Self {
        // For backward compatibility, create a default date if parsing fails
        let date = if date_str.len() == 6 {
            let year_str = &date_str[0..2];
            let month_str = &date_str[2..4];
            let day_str = &date_str[4..6];

            if let (Ok(year_val), Ok(month), Ok(day)) = (
                year_str.parse::<i32>(),
                month_str.parse::<u32>(),
                day_str.parse::<u32>(),
            ) {
                let year = year_val + 2000;
                NaiveDate::from_ymd_opt(year, month, day)
                    .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            } else {
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
            }
        } else {
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        };

        Self { date }
    }

    /// Get the execution date as string (for compatibility)
    pub fn date(&self) -> String {
        format!(
            "{:02}{:02}{:02}",
            self.date.year() % 100,
            self.date.month(),
            self.date.day()
        )
    }

    /// Get the date formatted as YYYY-MM-DD for display purposes
    pub fn format_readable(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }

    /// Get the underlying NaiveDate
    pub fn naive_date(&self) -> NaiveDate {
        self.date
    }
}

// The macro auto-generates all date analysis methods, parsing, validation, and serialization.
// This includes: is_today(), is_future(), is_past(), is_weekend(), days_from_today(),
// format_readable(), year(), month(), day(), to_naive_date(), execution_date(), and more.
