use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 13C: Time Indication**
///
/// ## Purpose
/// Specifies time indication related to payment processing with timezone offset information.
/// This field is critical for time-sensitive payments, settlement timing, and regulatory
/// compliance in cross-border transactions requiring precise timing documentation.
///
/// ## Format
/// - **Swift Format**: `/8c/4!n1!x4!n`
/// - **Components**:
///   - `/8c/`: Time indication code enclosed in slashes (8 characters)
///   - `4!n`: Time in HHMM format (24-hour clock)
///   - `1!x`: Sign indicator (+ or -)
///   - `4!n`: Time offset in HHMM format
///
/// ## Presence
/// - **Status**: Optional in MT102 Settlement Details sequence
/// - **Swift Error Codes**: T38 (invalid time), T16 (invalid offset), T15 (invalid sign)
/// - **Usage Context**: Payment timing and settlement coordination
///
/// ## Valid Time Indication Codes
/// - **CLSTIME**: CLS Time - Time by which funding payment must be credited to CLS Bank's account (CET)
/// - **RNCTIME**: Receive Time - Time at which TARGET2 payment was credited at receiving central bank (CET)
/// - **SNDTIME**: Send Time - Time at which TARGET2 payment was debited at sending central bank (CET)
/// - **REJTIME**: Rejection Time - Time when payment was rejected or returned
/// - **CUTTIME**: Cut-off Time - Latest time for payment processing
///
/// ## Network Validation Rules
/// - **Time Format**: Must be valid time in HHMM format (00:00 to 23:59)
/// - **Offset Hours**: Must be 00-13 hours
/// - **Offset Minutes**: Must be 00-59 minutes
/// - **Sign Validation**: Must be exactly + or - character
/// - **Code Validation**: Time indication code must be valid and recognized
///
/// ## Usage Rules
/// - **Timezone Context**: Time zone identified by offset against UTC (ISO 8601 standard)
/// - **Multiple Indications**: Multiple time indications can be provided in repetitive sequences
/// - **Settlement Coordination**: Used for coordinating settlement timing across time zones
/// - **Regulatory Compliance**: Required for certain cross-border payment regulations
///
/// ## Business Applications
/// - **Settlement Systems**: CLS Bank settlement timing coordination
/// - **TARGET2 Payments**: European central bank payment timing
/// - **Regulatory Reporting**: Time-stamping for compliance requirements
/// - **STP Processing**: Automated time-based processing decisions
///
/// ## Examples
/// ```logic
/// :13C:/SNDTIME/1430+0100    // Sent at 14:30 CET (UTC+1)
/// :13C:/CLSTIME/0930-0500    // CLS deadline 09:30 EST (UTC-5)
/// :13C:/RNCTIME/1615+0000    // Received at 16:15 UTC
/// :13C:/CUTTIME/1700+0200    // Cut-off at 17:00 CEST (UTC+2)
/// ```
///
/// ## Regional Considerations
/// - **European Payments**: CET/CEST timing for TARGET2 and SEPA
/// - **US Payments**: EST/EDT timing for Federal Reserve systems
/// - **Asian Markets**: Local time zones for regional clearing systems
/// - **Global Coordination**: UTC reference for international settlements
///
/// ## Error Prevention
/// - **Time Validation**: Ensure time is valid 24-hour format
/// - **Offset Accuracy**: Verify timezone offset matches actual timezone
/// - **Code Selection**: Use appropriate time indication code for context
/// - **Business Logic**: Ensure timing aligns with settlement windows
///
/// ## Related Fields
/// - **Field 32A**: Value Date (settlement date coordination)
/// - **Field 13D**: Date/Time Indication (alternative time specification)
/// - **Field 72**: Sender to Receiver Information (additional timing details)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13C {
    /// Time indication code with slashes
    ///
    /// Format: /8c/ - Valid codes: SNDTIME, CLSTIME, RNCTIME, REJTIME, CUTTIME
    /// Specifies the type of time being indicated
    #[component("/8c/")]
    pub code: String,

    /// Time in 24-hour format
    ///
    /// Format: 4!n (HHMM) - Must be valid time (00:00 to 23:59)
    /// Represents the actual time for the indicated event
    #[component("4!n")]
    pub time: String,

    /// Timezone offset sign
    ///
    /// Format: 1!x - Must be + (ahead of UTC) or - (behind UTC)
    /// Indicates direction of timezone offset from UTC
    #[component("1!x")]
    pub sign: String,

    /// Timezone offset amount
    ///
    /// Format: 4!n (HHMM) - Hours (00-13), Minutes (00-59)
    /// Specifies the offset from UTC time
    #[component("4!n")]
    pub offset: String,
}

/// **Field 13D: Date/Time Indication**
///
/// ## Purpose
/// Specifies complete date and time indication with UTC offset for precise timestamp
/// documentation. This field provides comprehensive temporal information for payment
/// processing events, audit trails, and regulatory compliance requirements.
///
/// ## Format
/// - **Swift Format**: `6!n4!n1!x4!n`
/// - **Components**:
///   - `6!n`: Date in YYMMDD format
///   - `4!n`: Time in HHMM format (24-hour clock)
///   - `1!x`: UTC offset sign (+ or -)
///   - `4!n`: UTC offset (typically in HHMM format)
///
/// ## Presence
/// - **Status**: Optional in most message types, mandatory for time-critical transactions
/// - **Swift Error Codes**: T40 (invalid date), T38 (invalid time), T16 (invalid offset)
/// - **Usage Context**: Comprehensive timestamp requirements
///
/// ## Network Validation Rules
/// - **Date Validation**: Must be valid calendar date in YYMMDD format
/// - **Time Validation**: Must be valid time in HHMM format (00:00 to 23:59)
/// - **Offset Sign**: Must be + (ahead of UTC) or - (behind UTC)
/// - **Offset Range**: UTC offset must be within valid timezone ranges
///
/// ## Usage Rules
/// - **Complete Timestamp**: Provides both date and time in single field
/// - **Timezone Awareness**: UTC offset enables precise time zone identification
/// - **Audit Trail**: Creates comprehensive timestamp for transaction events
/// - **Cross-Border Coordination**: Enables time coordination across different time zones
///
/// ## Business Applications
/// - **Transaction Timestamping**: Precise recording of payment events
/// - **Regulatory Compliance**: Meeting time-stamping requirements for reporting
/// - **Audit Documentation**: Comprehensive temporal records for compliance
/// - **System Integration**: Enabling time-aware processing across systems
///
/// ## Examples
/// ```logic
/// :13D:2501251430+0100    // January 25, 2025 at 14:30 CET (UTC+1)
/// :13D:2512311159-0500    // December 31, 2025 at 11:59 EST (UTC-5)
/// :13D:2506151200+0000    // June 15, 2025 at 12:00 UTC
/// :13D:2509301800+0900    // September 30, 2025 at 18:00 JST (UTC+9)
/// ```
///
/// ## Regional Considerations
/// - **European Markets**: CET/CEST for European business hours
/// - **US Markets**: EST/EDT for US business hours
/// - **Asian Markets**: Local time zones for regional processing
/// - **Global Operations**: UTC for international coordination
///
/// ## Error Prevention
/// - **Date Validation**: Ensure date is valid and within reasonable business range
/// - **Time Validation**: Verify time is valid 24-hour format
/// - **Offset Accuracy**: Confirm UTC offset matches actual timezone
/// - **Business Logic**: Ensure timestamp aligns with business processes
///
/// ## Comparison with Field 13C
/// - **Field 13C**: Focuses on time indication with codes (SNDTIME, CLSTIME, etc.)
/// - **Field 13D**: Provides complete date/time without specific indication codes
/// - **Usage Context**: 13C for specific timing events, 13D for general timestamps
/// - **Format Difference**: 13C includes time indication codes, 13D is pure date/time
///
/// ## Related Fields
/// - **Field 13C**: Time Indication (alternative time specification with codes)
/// - **Field 32A**: Value Date (settlement date without time component)
/// - **Field 30**: Execution Date (date-only specifications)
/// - **Block Headers**: Message timestamps (system-level timing)
///
/// ## Technical Implementation
/// - **Date Handling**: Uses `chrono::NaiveDate` for robust date parsing
/// - **Time Handling**: Uses `chrono::NaiveTime` for time validation
/// - **Offset Storage**: String format for flexible UTC offset representation
/// - **Validation**: Automatic format validation through Swift macro system
///
/// ## See Also
/// - Swift FIN User Handbook: Date/Time Specifications
/// - ISO 8601: International Date/Time Standards
/// - SWIFT Network Rules: Timestamp Requirements
/// - Regional Payment Guides: Local Time Zone Considerations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13D {
    /// Date component in YYMMDD format
    ///
    /// Format: 6!n - Must be valid calendar date
    /// Used for complete date specification with time
    #[component("6!n")]
    pub date: NaiveDate,

    /// Time component in HHMM format
    ///
    /// Format: 4!n - 24-hour clock (00:00 to 23:59)
    /// Provides precise time specification
    #[component("4!n")]
    pub time: NaiveTime,

    /// UTC offset direction sign
    ///
    /// Format: 1!x - Must be + (ahead of UTC) or - (behind UTC)
    /// Indicates timezone relationship to UTC
    #[component("1!x")]
    pub offset_sign: char,

    /// UTC offset amount
    ///
    /// Format: 4!n - Typically HHMM format for hours and minutes
    /// Specifies the numerical offset from UTC
    #[component("4!n")]
    pub offset_seconds: String,
}
