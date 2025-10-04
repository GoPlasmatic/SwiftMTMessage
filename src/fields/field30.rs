use super::swift_utils::parse_date_yymmdd;
use crate::traits::SwiftField;
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

///   **Field 30: Date Specifications**
///
/// ## Purpose
/// Specifies various types of dates critical to financial transaction processing, including
/// execution dates, value dates, settlement dates, and other time-sensitive information.
/// This field family provides precise temporal specifications required for proper transaction
/// timing, settlement coordination, and regulatory compliance.
///
/// ## Format
/// - **Swift Format**: `6!n` (Basic), `8!n` (Extended variants)
/// - **Description**: Date in YYMMDD or YYYYMMDD format
/// - **Date Validation**: Must represent valid calendar dates
/// - **Business Date**: Must align with business day conventions
///
/// ## Presence
/// - **Status**: Conditional/Mandatory depending on message type and business requirements
/// - **Swift Error Codes**: T40 (invalid date), T50 (format violation)
/// - **Usage Context**: Transaction timing and settlement coordination
///
/// ## Usage Rules
/// - **Valid Dates**: Must represent actual calendar dates
/// - **Business Logic**: Must comply with business day and settlement conventions
/// - **Time Zones**: Interpreted in appropriate business time zone context
/// - **Forward Dating**: Future dates must be within reasonable business limits
///
/// ## Network Validation Rules
/// - **Date Format**: Must follow exact YYMMDD or YYYYMMDD format
/// - **Calendar Validation**: Must be valid calendar date
/// - **Business Rules**: Must comply with market-specific business day rules
/// - **Range Validation**: Must be within acceptable date ranges for business context
///
/// ## Date Types and Applications
///
/// ### Execution Date (Basic Field 30)
/// - **Format**: `6!n` (YYMMDD)
/// - **Purpose**: Date when transaction should be executed
/// - **Usage**: Customer payment instructions, trade settlements
/// - **Business Context**: Determines when payment processing begins
///
/// ### Value Date Applications
/// - **Settlement**: Date funds become available
/// - **Interest**: Date for interest calculations
/// - **Trade**: Date for trade settlement
/// - **Currency Exchange**: Date for FX rate application
///
/// ### Premium Payment Date (Field 30V)
/// - **Format**: `8!n` (YYYYMMDD)
/// - **Purpose**: Date premium is paid for option contracts
/// - **Usage**: FX options, derivative contracts
/// - **Business Context**: Critical for option contract timing
///
/// ## Business Context
/// - **Payment Processing**: Determines transaction execution timing
/// - **Settlement Coordination**: Aligns settlement across counterparties
/// - **Interest Calculations**: Provides basis for accrual calculations
/// - **Regulatory Compliance**: Meets timing requirements for various regulations
///
/// ## Examples
/// ```logic
/// :30:250719      // July 19, 2025 (execution date)
/// :30V:20250719   // July 19, 2025 (premium payment date)
/// :30T:250720     // July 20, 2025 (trade date)
/// :30P:250721     // July 21, 2025 (processing date)
/// ```
///
/// ## Date Calculation Logic
/// - **Business Days**: Excludes weekends and holidays
/// - **Settlement Cycles**: Standard T+0, T+1, T+2, T+3 settlements
/// - **Cut-off Times**: Coordination with daily processing cut-offs
/// - **Time Zones**: Market-specific time zone considerations
///
/// ## Regional Considerations
/// - **European Markets**: TARGET2 business day calendar
/// - **US Markets**: Federal Reserve business day calendar
/// - **Asian Markets**: Local holiday and business day calendars
/// - **Cross-Border**: Coordination across multiple market calendars
///
/// ## Error Prevention
/// - **Date Validation**: Verify date is valid calendar date
/// - **Business Day Check**: Ensure date complies with business day conventions
/// - **Range Verification**: Confirm date is within reasonable business range
/// - **Market Calendar**: Check against relevant market holiday calendars
///
/// ## Related Fields
/// - **Field 32A**: Value Date, Currency, Amount (settlement information)
/// - **Field 61**: Statement Line (transaction dates)
/// - **Field 13C/13D**: Time Indication (precise timing information)
/// - **Block Headers**: Message timestamps
///
/// ## Settlement Coordination
/// - **Same Day Settlement**: T+0 processing requirements
/// - **Next Day Settlement**: T+1 standard processing
/// - **Standard Settlement**: T+2 typical market practice
/// - **Extended Settlement**: T+3 or longer for specific instruments
///
/// ## Processing Impact
/// - **Batch Processing**: Date-based transaction grouping
/// - **Real-Time Processing**: Immediate execution date processing
/// - **Schedule Processing**: Future-dated transaction scheduling
/// - **Exception Handling**: Holiday and weekend date adjustments
///
/// ## Compliance Framework
/// - **Regulatory Timing**: Meeting regulatory execution requirements
/// - **Market Rules**: Compliance with market settlement rules
/// - **Audit Trail**: Maintaining accurate date records
/// - **Documentation**: Proper date documentation for compliance
///
/// ## STP Compliance
/// - **Date Standardization**: Consistent date format for automation
/// - **Validation Enhancement**: Automated date validation and correction
/// - **Processing Rules**: Date-based automated processing logic
/// - **Exception Management**: Automated handling of date-related exceptions
///
/// ## See Also
/// - Swift FIN User Handbook: Date Field Specifications
/// - Settlement Guidelines: Business Day Conventions
/// - Market Calendars: Holiday and Business Day References
/// - Processing Standards: Date-Based Transaction Handling
///
///   **Field 30: Execution Date**
///
/// Basic execution date specification for transaction processing timing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field30 {
    /// Execution date
    ///
    /// Format: 6!n (YYMMDD) - Date when transaction should be executed
    /// Must be valid calendar date and comply with business day conventions
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
