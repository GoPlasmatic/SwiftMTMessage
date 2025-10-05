use super::swift_utils::parse_date_yymmdd;
use crate::traits::SwiftField;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// **Field 30: Date Specifications**
///
/// Specifies various types of dates critical to transaction processing.
///
/// **Format:** `6!n` (YYMMDD) or `8!n` (YYYYMMDD for variants)
/// **Constraints:** Valid calendar date, business day conventions
///
/// **Example:**
/// ```text
/// :30:250719
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field30 {
    /// Execution date (YYMMDD)
    pub execution_date: NaiveDate,
}

impl SwiftField for Field30 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let execution_date = parse_date_yymmdd(input)?;

        Ok(Field30 { execution_date })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":30:{:02}{:02}{:02}",
            self.execution_date.year() % 100,
            self.execution_date.month(),
            self.execution_date.day()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_field30_parse() {
        let field = Field30::parse("240719").unwrap();
        assert_eq!(field.execution_date.year(), 2024);
        assert_eq!(field.execution_date.month(), 7);
        assert_eq!(field.execution_date.day(), 19);

        // Test century logic (50-99 -> 1950-1999)
        let field = Field30::parse("991231").unwrap();
        assert_eq!(field.execution_date.year(), 1999);

        // Test century logic (00-49 -> 2000-2049)
        let field = Field30::parse("250101").unwrap();
        assert_eq!(field.execution_date.year(), 2025);
    }

    #[test]
    fn test_field30_to_swift_string() {
        let field = Field30 {
            execution_date: NaiveDate::from_ymd_opt(2024, 7, 19).unwrap(),
        };
        assert_eq!(field.to_swift_string(), ":30:240719");

        let field = Field30 {
            execution_date: NaiveDate::from_ymd_opt(1999, 12, 31).unwrap(),
        };
        assert_eq!(field.to_swift_string(), ":30:991231");
    }

    #[test]
    fn test_field30_parse_invalid() {
        // Invalid length
        assert!(Field30::parse("12345").is_err());
        assert!(Field30::parse("1234567").is_err());

        // Invalid date
        assert!(Field30::parse("240230").is_err()); // Feb 30th
        assert!(Field30::parse("241301").is_err()); // Month 13

        // Non-numeric
        assert!(Field30::parse("24071a").is_err());
    }
}
