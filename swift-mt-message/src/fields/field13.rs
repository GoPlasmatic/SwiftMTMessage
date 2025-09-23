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

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

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
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
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
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
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
    /// Format: 4!n - HHMM format for hours and minutes
    /// Specifies the numerical offset from UTC
    #[component("4!n")]
    pub offset_seconds: String,
}
