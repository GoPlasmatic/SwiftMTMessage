//! # Field 32: Value Date, Currency Code, Amount
//!
//! ## Purpose
//! Specifies the value date, currency, and settlement amount for payment instructions.
//! This is the core monetary field that defines when and how much will be transferred,
//! serving as the foundation for all payment processing and settlement calculations.
//!
//! ## Options Overview
//! - **Option A**: Value Date + Currency + Amount (complete settlement information)
//! - **Option B**: Currency + Amount (amount without specific value date)
//!
//! ## Format Specifications
//! ### Option A Format
//! - **Swift Format**: `6!n3!a15d`
//! - **Components**:
//!   - `6!n`: Value date (YYMMDD format)
//!   - `3!a`: Currency code (ISO 4217, 3 alphabetic characters)
//!   - `15d`: Amount (up to 15 digits including decimal, comma as decimal separator)
//!
//! ### Option B Format
//! - **Swift Format**: `3!a15d`
//! - **Components**:
//!   - `3!a`: Currency code (ISO 4217, 3 alphabetic characters)
//!   - `15d`: Amount (up to 15 digits including decimal, comma as decimal separator)
//!
//! ## Presence and Usage
//! - **Status**: Mandatory in all payment messages (MT103, MT202, etc.)
//! - **Swift Error Codes**: T40 (invalid date), T52 (invalid currency), T51 (invalid amount)
//! - **Referenced in Rules**: C1, C7, C8, C9 (MT103), currency validation across message types
//!
//! ## Value Date Rules (Option A)
//! - **Format**: YYMMDD (2-digit year, month, day)
//! - **Validation**: Must be a valid calendar date
//! - **Business Rules**: Cannot be more than 1 year in the past or future (typical limit)
//! - **Weekends/Holidays**: System may adjust for banking days depending on currency
//!
//! ## Currency Code Rules
//! - **Standard**: ISO 4217 three-letter currency codes
//! - **Validation**: Must be an active, tradeable currency
//! - **Examples**: USD, EUR, GBP, JPY, CHF, CAD, AUD
//! - **Restrictions**: Some currencies may be restricted for certain corridors
//!
//! ## Amount Rules
//! - **Format**: Up to 15 digits with decimal precision
//! - **Decimal Separator**: Comma (,) for decimal values in Swift format
//! - **Precision**: Typically 2 decimal places, varies by currency (JPY has 0, BHD has 3)
//! - **Range**: Must be positive (> 0), maximum depends on currency and institution limits
//!
//! ## Network Validation Rules
//! - **C1 (MT103)**: If field 33B differs from 32A currency, field 36 (Exchange Rate) required
//! - **C7**: Amount must be positive and properly formatted for currency
//! - **C8**: If charges apply (71F/71G), 33B becomes mandatory for charge calculations
//! - **C9**: Currency in 71G must match 32A currency for charge consistency
//!
//! ## Usage Guidelines
//! - **Settlement**: This amount determines the final settlement obligation
//! - **Exchange Rates**: When currency differs from instructed amount (33B), exchange rate (36) needed
//! - **Charges**: Original instructed amount before any fee deductions
//! - **Precision**: Must respect currency-specific decimal precision rules
//!
//! ## STP Compliance
//! - **Amount Format**: Must comply with STP formatting standards (no trailing zeros)
//! - **Currency Support**: STP corridors may support limited currency pairs
//! - **Validation**: Enhanced validation for STP messages to prevent manual intervention
//!
//! ## Regional Considerations
//! - **SEPA**: EUR payments within SEPA zone have specific amount and date rules
//! - **US Domestic**: USD payments may require different value date handling
//! - **Emerging Markets**: Some currencies have additional restrictions or validations
//!
//! ## Examples
//! ```text
//! :32A:240719EUR1250,50     // July 19, 2024, EUR 1,250.50
//! :32A:240720USD10000,00    // July 20, 2024, USD 10,000.00
//! :32A:240721JPY1500000     // July 21, 2024, JPY 1,500,000 (no decimal)
//! :32B:EUR5000,00          // EUR 5,000.00 (no value date)
//! ```
//!
//! ## Related Fields Integration
//! - **Field 33B**: Instructed Amount (if different from settlement amount)
//! - **Field 36**: Exchange Rate (when 33B currency differs from 32A)
//! - **Field 71F/71G**: Sender's/Receiver's Charges (affect final settlement)
//! - **Field 30**: Execution Date (in some message types)
//!
//! ## Error Prevention
//! - **Invalid Date**: T40 error if date is malformed or unrealistic
//! - **Invalid Currency**: T52 error if currency code not recognized
//! - **Invalid Amount**: T51 error if amount format incorrect or negative
//! - **Business Rule**: C-rule violations if currency/amount conflicts with other fields
//!
//! ## Amount Precision by Currency
//! - **Most Currencies**: 2 decimal places (USD, EUR, GBP, etc.)
//! - **Japanese Yen**: 0 decimal places (JPY)
//! - **Bahraini Dinar**: 3 decimal places (BHD)
//! - **Cryptocurrency**: Variable precision (check current standards)
//!
//! ## See Also
//! - Swift FIN User Handbook: Currency and Amount Specifications
//! - ISO 4217: Currency Code Standard
//! - MT103 Usage Rules: Value Date and Settlement Guidelines
//! - STP Implementation Guide: Amount Format Requirements

use chrono::NaiveDate;
use swift_mt_message_macros::serde_swift_fields;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 32A: Value Date, Currency Code, Amount**
///
/// Complete settlement information variant of [Field 32 module](index.html). Specifies the value date,
/// currency, and settlement amount for payment instructions.
///
/// **Components:**
/// - Value date (6!n, YYMMDD format)
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field32A {
    /// Value date when the payment becomes effective
    ///
    /// Format: 6!n (YYMMDD) - Must be valid calendar date
    /// Business rule: Typically within 1 year of today
    #[component("6!n")]
    pub value_date: Option<NaiveDate>,

    /// ISO 4217 three-letter currency code
    ///
    /// Format: 3!a - Must be valid, active currency
    /// Examples: USD, EUR, GBP, JPY, CHF
    #[component("3!a")]
    pub currency: String,

    /// Settlement amount in the specified currency
    ///
    /// Format: 15d - Up to 15 digits, comma decimal separator
    /// Must be positive, respect currency precision rules
    #[component("15d")]
    pub amount: f64,
}

/// **Field 32B: Currency Code, Amount**
///
/// Currency and amount variant of [Field 32 module](index.html). Specifies currency and amount
/// without a specific value date.
///
/// **Components:**
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field32B {
    /// Currency code (ISO 4217)
    #[component("3!a")]
    pub currency: String,
    /// Amount
    #[component("15d")]
    pub amount: f64,
}

/// **Field 32C: Value Date, Currency Code, Amount (Credit)**
///
/// Credit variant of [Field 32 module](index.html). Specifies the value date,
/// currency, and amount credited. Used in MT n90 messages (MT190, MT290, etc.)
/// to indicate credit adjustments.
///
/// **Components:**
/// - Value date (6!n, YYMMDD format)
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field32C {
    /// Value date when the credit becomes effective
    ///
    /// Format: 6!n (YYMMDD) - Must be valid calendar date
    #[component("6!n")]
    pub value_date: Option<NaiveDate>,

    /// ISO 4217 three-letter currency code
    ///
    /// Format: 3!a - Must be valid, active currency
    #[component("3!a")]
    pub currency: String,

    /// Credit amount in the specified currency
    ///
    /// Format: 15d - Up to 15 digits, comma decimal separator
    #[component("15d")]
    pub amount: f64,
}

/// **Field 32D: Value Date, Currency Code, Amount (Debit)**
///
/// Debit variant of [Field 32 module](index.html). Specifies the value date,
/// currency, and amount debited. Used in MT n90 messages (MT190, MT290, etc.)
/// to indicate debit adjustments.
///
/// **Components:**
/// - Value date (6!n, YYMMDD format)
/// - Currency code (3!a, ISO 4217)
/// - Amount (15d, decimal with comma separator)
///
/// For complete documentation, see the [Field 32 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field32D {
    /// Value date when the debit becomes effective
    ///
    /// Format: 6!n (YYMMDD) - Must be valid calendar date
    #[component("6!n")]
    pub value_date: Option<NaiveDate>,

    /// ISO 4217 three-letter currency code
    ///
    /// Format: 3!a - Must be valid, active currency
    #[component("3!a")]
    pub currency: String,

    /// Debit amount in the specified currency
    ///
    /// Format: 15d - Up to 15 digits, comma decimal separator
    #[component("15d")]
    pub amount: f64,
}

/// **Field 32 Enum: Value Date, Currency, Amount Variants**
///
/// Enum wrapper for [Field 32 module](index.html) variants providing different
/// levels of settlement information detail.
///
/// For complete documentation, see the [Field 32 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field32 {
    A(Field32A),
    B(Field32B),
    C(Field32C),
    D(Field32D),
}
