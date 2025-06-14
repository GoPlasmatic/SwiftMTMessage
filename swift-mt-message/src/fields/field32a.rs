use crate::{SwiftField, ValidationResult};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

/// # Field 32A: Value Date, Currency Code, Amount
///
/// ## Overview
/// Field 32A is a composite field that contains three critical components of a financial
/// transaction: the value date (when the transaction becomes effective), the currency code
/// (ISO 4217 standard), and the transaction amount. This field is fundamental to SWIFT MT
/// messages and serves as the primary transaction specification in most payment messages.
///
/// ## Format Specification
/// **Format**: `6!n3!a15d`
/// - **6!n**: Value date in YYMMDD format (6 numeric characters)
/// - **3!a**: Currency code (3 alphabetic characters, ISO 4217)
/// - **15d**: Amount with up to 15 digits including decimal places
///
/// ### Component Details
/// 1. **Value Date (YYMMDD)**:
///    - Year: 2-digit year (YY) - assumes 20YY for years 00-99
///    - Month: 2-digit month (01-12)
///    - Day: 2-digit day (01-31, depending on month)
///    - Must be a valid calendar date
///
/// 2. **Currency Code (3!a)**:
///    - ISO 4217 standard currency codes
///    - Exactly 3 alphabetic characters
///    - Case-insensitive input, stored as uppercase
///    - Examples: USD, EUR, GBP, JPY, CHF
///
/// 3. **Amount (15d)**:
///    - Up to 15 digits including decimal places
///    - Decimal separator: comma (,) in SWIFT format
///    - No thousands separators
///    - Must be positive (> 0)
///    - Precision: typically 2 decimal places for most currencies
///
/// ## Usage Context
/// Field 32A appears in numerous SWIFT MT message types:
///
/// ### Primary Usage
/// - **MT103**: Single Customer Credit Transfer - transaction amount and value date
/// - **MT202**: General Financial Institution Transfer - settlement amount
/// - **MT202COV**: Cover for customer credit transfer - cover amount
/// - **MT205**: Financial Institution Transfer for its Own Account
///
/// ### Secondary Usage
/// - **MT400**: Advice of Payment - payment amount
/// - **MT410**: Acknowledgement - acknowledged amount
/// - **MT420**: Tracer - traced amount
/// - **MT900**: Confirmation of Debit - debited amount
/// - **MT910**: Confirmation of Credit - credited amount
///
/// ## Business Applications
/// - **Payment processing**: Core transaction specification
/// - **Settlement**: Value dating for settlement systems
/// - **Accounting**: Transaction recording and reconciliation
/// - **Compliance**: AML/KYC amount thresholds
/// - **Risk management**: Exposure calculation and limits
/// - **Reporting**: Regulatory and management reporting
/// - **FX processing**: Currency conversion and hedging
/// - **Liquidity management**: Cash flow planning
///
/// ## Value Dating Rules
/// Value dates must follow specific business rules:
///
/// ### Standard Rules
/// - **Same day value**: Value date = current business date
/// - **Next day value**: Value date = next business date
/// - **Forward value**: Value date > current date (up to 1 year typically)
/// - **Back value**: Value date < current date (limited, usually same week)
///
/// ### Currency-Specific Rules
/// - **USD**: T+0 or T+1 settlement
/// - **EUR**: T+1 settlement (TARGET2)
/// - **GBP**: T+0 settlement (CHAPS)
/// - **JPY**: T+0 or T+1 settlement
/// - **Exotic currencies**: May require T+2 or longer
///
/// ### Holiday Considerations
/// - Value dates must be valid business days
/// - Consider both sending and receiving country holidays
/// - Weekend adjustments follow market conventions
/// - Holiday calendars vary by currency and market
///
/// ## Amount Formatting Rules
/// 1. **Decimal separator**: Always comma (,) in SWIFT format
/// 2. **No thousands separators**: 1234567,89 not 1,234,567.89
/// 3. **Leading zeros**: Not required (123,45 not 0000123,45)
/// 4. **Trailing zeros**: Required for decimal places (100,00 not 100)
/// 5. **Maximum precision**: Varies by currency (typically 2 decimal places)
///
/// ## Currency Code Validation
/// - Must be valid ISO 4217 currency code
/// - Active currencies only (not historical or test codes)
/// - Some restricted currencies may require special handling
/// - Cryptocurrency codes follow ISO 4217 digital currency standards
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Value date must be valid calendar date (Error: T50)
/// - Currency code must be valid ISO 4217 (Error: T52)
/// - Amount must be positive and properly formatted (Error: T40)
/// - Value date should be reasonable business date (Warning: recommended)
/// - Currency should be actively traded (Warning: recommended)
///
///
/// ## Examples
/// ```text
/// :32A:240315USD1234567,89
/// └─── Value: March 15, 2024, USD 1,234,567.89
///
/// :32A:240401EUR500000,00
/// └─── Value: April 1, 2024, EUR 500,000.00
///
/// :32A:240228GBP75000,50
/// └─── Value: February 28, 2024, GBP 75,000.50
/// ```
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field32A {
    /// Value date when the transaction becomes effective
    ///
    /// Specifies the date on which the transaction amount should be
    /// credited or debited to the beneficiary's account. Must be a
    /// valid calendar date and typically a business day.
    ///
    /// **Format**: YYMMDD (6 numeric characters)
    /// **Range**: Valid calendar dates
    /// **Business rules**: Should be valid business day for currency
    ///
    /// # Examples
    /// - March 15, 2024 → `NaiveDate::from_ymd_opt(2024, 3, 15)`
    /// - December 31, 2023 → `NaiveDate::from_ymd_opt(2023, 12, 31)`
    pub value_date: NaiveDate,

    /// ISO 4217 currency code (3 alphabetic characters)
    ///
    /// Specifies the currency of the transaction amount using the
    /// international standard ISO 4217 currency codes.
    ///
    /// **Format**: Exactly 3 uppercase alphabetic characters
    /// **Standard**: ISO 4217 (International Organization for Standardization)
    /// **Case handling**: Automatically converted to uppercase
    ///
    /// # Common Currencies
    /// - `"USD"` - United States Dollar
    /// - `"EUR"` - Euro
    /// - `"GBP"` - British Pound Sterling
    /// - `"JPY"` - Japanese Yen
    /// - `"CHF"` - Swiss Franc
    /// - `"CAD"` - Canadian Dollar
    /// - `"AUD"` - Australian Dollar
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1000.00
    /// );
    /// assert_eq!(field.currency, "USD");
    /// ```
    pub currency: String,

    /// Transaction amount as decimal value
    ///
    /// The monetary amount of the transaction expressed as a floating-point
    /// number. Must be positive and should respect the precision rules
    /// for the specified currency.
    ///
    /// **Range**: Must be positive (> 0.0)
    /// **Precision**: Typically 2 decimal places for most currencies
    /// **Special cases**: JPY typically has 0 decimal places
    ///
    /// # Examples
    /// - `1234567.89` - One million, two hundred thirty-four thousand, five hundred sixty-seven and 89 cents
    /// - `100.00` - One hundred units
    /// - `0.01` - One cent (minimum for most currencies)
    pub amount: f64,

    /// Raw amount string as received (preserves original formatting)
    ///
    /// Maintains the original string representation of the amount as
    /// received in the SWIFT message, preserving the exact formatting
    /// including decimal separator and precision.
    ///
    /// **Format**: SWIFT standard with comma as decimal separator
    /// **Preservation**: Maintains original precision and formatting
    /// **Usage**: For exact reproduction of original message format
    ///
    /// # Examples
    /// - `"1234567,89"` - SWIFT format with comma separator
    /// - `"100,00"` - Two decimal places preserved
    /// - `"0,01"` - Leading zero preserved
    pub raw_amount: String,
}

impl SwiftField for Field32A {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":32A:") {
            stripped // Remove ":32A:" prefix
        } else if let Some(stripped) = value.strip_prefix("32A:") {
            stripped // Remove "32A:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.len() < 9 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Field too short (minimum 9 characters: YYMMDDCCCAMOUNT)".to_string(),
            });
        }

        // Parse date (YYMMDD format)
        let date_str = &content[0..6];
        let year = format!("20{}", &date_str[0..2]);
        let month = &date_str[2..4];
        let day = &date_str[4..6];
        let full_date_str = format!("{}-{}-{}", year, month, day);

        let value_date = NaiveDate::parse_from_str(&full_date_str, "%Y-%m-%d").map_err(|_| {
            crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid date format".to_string(),
            }
        })?;

        // Parse currency (3 characters)
        let currency = content[6..9].to_string().to_uppercase();

        // Parse amount (remaining characters)
        let raw_amount = content[9..].to_string();
        let amount = raw_amount.replace(',', ".").parse::<f64>().map_err(|_| {
            crate::ParseError::InvalidFieldFormat {
                field_tag: "32A".to_string(),
                message: "Invalid amount format".to_string(),
            }
        })?;

        Ok(Self {
            value_date,
            currency,
            amount,
            raw_amount,
        })
    }

    fn to_swift_string(&self) -> String {
        // Format date as YYMMDD
        let date_str = format!(
            "{:02}{:02}{:02}",
            self.value_date.year() % 100,
            self.value_date.month(),
            self.value_date.day()
        );

        format!(":32A:{}{}{}", date_str, self.currency, self.raw_amount)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate currency length
        if self.currency.len() != 3 {
            errors.push(crate::ValidationError::LengthValidation {
                field_tag: "32A".to_string(),
                expected: "3 characters".to_string(),
                actual: self.currency.len(),
            });
        }

        // Validate currency contains only letters
        if !self.currency.chars().all(|c| c.is_alphabetic()) {
            errors.push(crate::ValidationError::ValueValidation {
                field_tag: "32A".to_string(),
                message: "Currency must contain only alphabetic characters".to_string(),
            });
        }

        // Validate amount is positive
        if self.amount <= 0.0 {
            errors.push(crate::ValidationError::ValueValidation {
                field_tag: "32A".to_string(),
                message: "Amount must be positive".to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "6!n3!a15d"
    }
}

impl Field32A {
    /// Create a new Field32A
    ///
    /// Creates a new Field32A instance with the specified value date,
    /// currency, and amount. The amount is automatically formatted
    /// according to SWIFT standards.
    ///
    /// # Arguments
    /// * `value_date` - The value date for the transaction
    /// * `currency` - ISO 4217 currency code (will be converted to uppercase)
    /// * `amount` - Transaction amount (must be positive)
    ///
    /// # Returns
    /// A new Field32A instance
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1234.56
    /// );
    /// assert_eq!(field.amount, 1234.56);
    /// assert_eq!(field.currency, "USD");
    /// ```
    pub fn new(value_date: NaiveDate, currency: String, amount: f64) -> Self {
        // Format amount with comma as decimal separator (SWIFT standard)
        let raw_amount = format!("{:.2}", amount).replace('.', ",");
        Self {
            value_date,
            currency: currency.to_uppercase(),
            amount,
            raw_amount,
        }
    }

    /// Create from raw values
    ///
    /// Creates a Field32A instance from raw string amount, preserving
    /// the original formatting while parsing the numeric value.
    ///
    /// # Arguments
    /// * `value_date` - The value date for the transaction
    /// * `currency` - ISO 4217 currency code
    /// * `raw_amount` - Amount string in SWIFT format
    ///
    /// # Returns
    /// Result containing the Field32A instance or parse error
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let field = Field32A::from_raw(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "EUR".to_string(),
    ///     "1000,50".to_string()
    /// ).unwrap();
    /// assert_eq!(field.amount, 1000.50);
    /// assert_eq!(field.raw_amount, "1000,50");
    /// ```
    pub fn from_raw(
        value_date: NaiveDate,
        currency: String,
        raw_amount: String,
    ) -> Result<Self, std::num::ParseFloatError> {
        let amount = raw_amount.replace(',', ".").parse::<f64>()?;
        Ok(Self {
            value_date,
            currency: currency.to_uppercase(),
            amount,
            raw_amount,
        })
    }

    /// Get the currency code
    ///
    /// Returns the ISO 4217 currency code for this transaction.
    ///
    /// # Returns
    /// Currency code as string slice
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "GBP".to_string(),
    ///     500.00
    /// );
    /// assert_eq!(field.currency_code(), "GBP");
    /// ```
    pub fn currency_code(&self) -> &str {
        &self.currency
    }

    /// Get the amount as decimal
    ///
    /// Returns the transaction amount as a floating-point number.
    ///
    /// # Returns
    /// Amount as f64
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1234.56
    /// );
    /// assert_eq!(field.amount_decimal(), 1234.56);
    /// ```
    pub fn amount_decimal(&self) -> f64 {
        self.amount
    }

    /// Format date as YYMMDD string
    ///
    /// Returns the value date formatted as a 6-character string
    /// in YYMMDD format as used in SWIFT messages.
    ///
    /// # Returns
    /// Date string in YYMMDD format
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1000.00
    /// );
    /// assert_eq!(field.date_string(), "240315");
    /// ```
    pub fn date_string(&self) -> String {
        format!(
            "{:02}{:02}{:02}",
            self.value_date.year() % 100,
            self.value_date.month(),
            self.value_date.day()
        )
    }

    /// Check if the currency is a major currency
    ///
    /// Determines if the currency is one of the major internationally
    /// traded currencies with high liquidity.
    ///
    /// # Returns
    /// `true` if the currency is a major currency
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let usd_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1000.00
    /// );
    /// assert!(usd_field.is_major_currency());
    ///
    /// let exotic_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "XYZ".to_string(),
    ///     1000.00
    /// );
    /// assert!(!exotic_field.is_major_currency());
    /// ```
    pub fn is_major_currency(&self) -> bool {
        matches!(
            self.currency.as_str(),
            "USD" | "EUR" | "GBP" | "JPY" | "CHF" | "CAD" | "AUD" | "NZD" | "SEK" | "NOK" | "DKK"
        )
    }

    /// Check if the currency typically has decimal places
    ///
    /// Determines if the currency typically uses decimal places
    /// in amount representation. Some currencies like JPY typically
    /// don't use decimal places.
    ///
    /// # Returns
    /// `true` if the currency typically uses decimal places
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let usd_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1000.00
    /// );
    /// assert!(usd_field.has_decimal_places());
    ///
    /// let jpy_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "JPY".to_string(),
    ///     1000.00
    /// );
    /// assert!(!jpy_field.has_decimal_places());
    /// ```
    pub fn has_decimal_places(&self) -> bool {
        !matches!(
            self.currency.as_str(),
            "JPY" | "KRW" | "VND" | "IDR" | "CLP" | "PYG" | "UGX" | "RWF" | "GNF" | "MGA"
        )
    }

    /// Get the typical decimal places for this currency
    ///
    /// Returns the number of decimal places typically used
    /// for this currency in financial transactions.
    ///
    /// # Returns
    /// Number of decimal places (0, 2, or 3)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let usd_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1000.00
    /// );
    /// assert_eq!(usd_field.decimal_places(), 2);
    ///
    /// let jpy_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "JPY".to_string(),
    ///     1000.00
    /// );
    /// assert_eq!(jpy_field.decimal_places(), 0);
    /// ```
    pub fn decimal_places(&self) -> u8 {
        match self.currency.as_str() {
            // Currencies with no decimal places
            "JPY" | "KRW" | "VND" | "IDR" | "CLP" | "PYG" | "UGX" | "RWF" | "GNF" | "MGA" => 0,
            // Currencies with 3 decimal places
            "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
            // Most currencies use 2 decimal places
            _ => 2,
        }
    }

    /// Check if the amount is a high-value transaction
    ///
    /// Determines if the transaction amount exceeds typical
    /// high-value thresholds that may require special handling.
    ///
    /// # Returns
    /// `true` if this is considered a high-value transaction
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let high_value = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1500000.00
    /// );
    /// assert!(high_value.is_high_value_transaction());
    ///
    /// let normal_value = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     50000.00
    /// );
    /// assert!(!normal_value.is_high_value_transaction());
    /// ```
    pub fn is_high_value_transaction(&self) -> bool {
        // High-value thresholds vary by currency
        let threshold = match self.currency.as_str() {
            "USD" | "EUR" | "GBP" | "CHF" | "CAD" | "AUD" => 1_000_000.0,
            "JPY" => 100_000_000.0,
            "SEK" | "NOK" | "DKK" => 10_000_000.0,
            _ => 1_000_000.0, // Default threshold
        };

        self.amount >= threshold
    }

    /// Get the settlement timing for this currency
    ///
    /// Returns the typical settlement timing for transactions
    /// in this currency based on market conventions.
    ///
    /// # Returns
    /// Settlement timing description
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let usd_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1000.00
    /// );
    /// assert_eq!(usd_field.settlement_timing(), "T+0/T+1 (Same day or next day)");
    /// ```
    pub fn settlement_timing(&self) -> &'static str {
        match self.currency.as_str() {
            "USD" => "T+0/T+1 (Same day or next day)",
            "EUR" => "T+1 (Next day via TARGET2)",
            "GBP" => "T+0 (Same day via CHAPS)",
            "JPY" => "T+0/T+1 (Same day or next day)",
            "CHF" => "T+0 (Same day via SIC)",
            "CAD" => "T+0/T+1 (Same day or next day)",
            "AUD" => "T+1 (Next day via RITS)",
            "SEK" => "T+1 (Next day via RIX)",
            "NOK" => "T+0 (Same day via NBO)",
            "DKK" => "T+1 (Next day via Kronos2)",
            _ => "T+2 or longer (Depends on currency and market)",
        }
    }

    /// Check if this is a same-day value transaction
    ///
    /// Determines if the value date is the same as today's date,
    /// indicating same-day value requirements.
    ///
    /// # Returns
    /// `true` if the value date is today
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::{NaiveDate, Utc};
    /// let today = Utc::now().date_naive();
    /// let field = Field32A::new(today, "USD".to_string(), 1000.00);
    /// assert!(field.is_same_day_value());
    /// ```
    pub fn is_same_day_value(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        self.value_date == today
    }

    /// Check if this is a forward-dated transaction
    ///
    /// Determines if the value date is in the future,
    /// indicating a forward-dated transaction.
    ///
    /// # Returns
    /// `true` if the value date is in the future
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::{NaiveDate, Utc, Duration};
    /// let future_date = Utc::now().date_naive() + Duration::days(5);
    /// let field = Field32A::new(future_date, "USD".to_string(), 1000.00);
    /// assert!(field.is_forward_dated());
    /// ```
    pub fn is_forward_dated(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        self.value_date > today
    }

    /// Check if this is a back-dated transaction
    ///
    /// Determines if the value date is in the past,
    /// indicating a back-dated transaction.
    ///
    /// # Returns
    /// `true` if the value date is in the past
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::{NaiveDate, Utc, Duration};
    /// let past_date = Utc::now().date_naive() - Duration::days(2);
    /// let field = Field32A::new(past_date, "USD".to_string(), 1000.00);
    /// assert!(field.is_back_dated());
    /// ```
    pub fn is_back_dated(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        self.value_date < today
    }

    /// Get days until value date
    ///
    /// Returns the number of days between today and the value date.
    /// Positive values indicate future dates, negative values indicate past dates.
    ///
    /// # Returns
    /// Number of days (positive for future, negative for past, 0 for today)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::{NaiveDate, Utc, Duration};
    /// let future_date = Utc::now().date_naive() + Duration::days(3);
    /// let field = Field32A::new(future_date, "USD".to_string(), 1000.00);
    /// assert_eq!(field.days_until_value_date(), 3);
    /// ```
    pub fn days_until_value_date(&self) -> i64 {
        let today = chrono::Utc::now().date_naive();
        (self.value_date - today).num_days()
    }

    /// Format amount with proper currency precision
    ///
    /// Formats the amount according to the typical precision
    /// rules for the currency.
    ///
    /// # Returns
    /// Formatted amount string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let usd_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1234.56
    /// );
    /// assert_eq!(usd_field.formatted_amount(), "1234.56");
    ///
    /// let jpy_field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "JPY".to_string(),
    ///     1234.00
    /// );
    /// assert_eq!(jpy_field.formatted_amount(), "1234");
    /// ```
    pub fn formatted_amount(&self) -> String {
        let decimal_places = self.decimal_places();
        match decimal_places {
            0 => format!("{:.0}", self.amount.round()),
            2 => format!("{:.2}", self.amount),
            3 => format!("{:.3}", self.amount),
            _ => format!("{:.2}", self.amount), // Default to 2
        }
    }

    /// Get comprehensive transaction description
    ///
    /// Returns a detailed description of the transaction including
    /// value date, currency, amount, and transaction characteristics.
    ///
    /// # Returns
    /// Formatted description string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field32A;
    /// # use chrono::NaiveDate;
    /// let field = Field32A::new(
    ///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
    ///     "USD".to_string(),
    ///     1500000.00
    /// );
    /// let desc = field.comprehensive_description();
    /// assert!(desc.contains("USD"));
    /// assert!(desc.contains("1500000.00"));
    /// assert!(desc.contains("2024-03-15"));
    /// ```
    pub fn comprehensive_description(&self) -> String {
        let value_timing = if self.is_same_day_value() {
            "Same-day value"
        } else if self.is_forward_dated() {
            "Forward-dated"
        } else if self.is_back_dated() {
            "Back-dated"
        } else {
            "Standard value"
        };

        let amount_category = if self.is_high_value_transaction() {
            "High-value"
        } else {
            "Standard"
        };

        let currency_type = if self.is_major_currency() {
            "Major currency"
        } else {
            "Other currency"
        };

        format!(
            "Value Date: {} | Currency: {} ({}) | Amount: {} {} | Settlement: {} | Category: {} {}",
            self.value_date,
            self.currency,
            currency_type,
            self.formatted_amount(),
            self.currency,
            self.settlement_timing(),
            amount_category,
            value_timing
        )
    }
}

impl std::fmt::Display for Field32A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.date_string(),
            self.currency,
            self.raw_amount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_field32a_creation() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR".to_string(), 1234567.89);

        assert_eq!(field.value_date.year(), 2021);
        assert_eq!(field.value_date.month(), 3);
        assert_eq!(field.value_date.day(), 15);
        assert_eq!(field.currency_code(), "EUR");
        assert_eq!(field.amount, 1234567.89);
    }

    #[test]
    fn test_field32a_parse() {
        let field = Field32A::parse("210315EUR1234567,89").unwrap();
        assert_eq!(field.value_date.year(), 2021);
        assert_eq!(field.value_date.month(), 3);
        assert_eq!(field.value_date.day(), 15);
        assert_eq!(field.currency_code(), "EUR");
        assert_eq!(field.amount, 1234567.89);
    }

    #[test]
    fn test_field32a_date_string() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR".to_string(), 1234567.89);

        assert_eq!(field.date_string(), "210315");
    }

    #[test]
    fn test_field32a_to_swift_string() {
        let date = NaiveDate::from_ymd_opt(2021, 3, 15).unwrap();
        let field = Field32A::new(date, "EUR".to_string(), 1234567.89);

        assert_eq!(field.to_swift_string(), ":32A:210315EUR1234567,89");
    }

    #[test]
    fn test_field32a_major_currencies() {
        let major_currencies = [
            "USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK", "DKK",
        ];
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        for currency in major_currencies {
            let field = Field32A::new(date, currency.to_string(), 1000.00);
            assert!(
                field.is_major_currency(),
                "Currency {} should be major currency",
                currency
            );
        }

        let exotic_currencies = ["XYZ", "ABC", "THB", "MXN"];
        for currency in exotic_currencies {
            let field = Field32A::new(date, currency.to_string(), 1000.00);
            assert!(
                !field.is_major_currency(),
                "Currency {} should not be major currency",
                currency
            );
        }
    }

    #[test]
    fn test_field32a_decimal_places() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        // Currencies with no decimal places
        let no_decimal_currencies = ["JPY", "KRW", "VND"];
        for currency in no_decimal_currencies {
            let field = Field32A::new(date, currency.to_string(), 1000.00);
            assert_eq!(
                field.decimal_places(),
                0,
                "Currency {} should have 0 decimal places",
                currency
            );
            assert!(!field.has_decimal_places());
        }

        // Currencies with 3 decimal places
        let three_decimal_currencies = ["BHD", "JOD", "KWD"];
        for currency in three_decimal_currencies {
            let field = Field32A::new(date, currency.to_string(), 1000.00);
            assert_eq!(
                field.decimal_places(),
                3,
                "Currency {} should have 3 decimal places",
                currency
            );
            assert!(field.has_decimal_places());
        }

        // Currencies with 2 decimal places (most common)
        let two_decimal_currencies = ["USD", "EUR", "GBP"];
        for currency in two_decimal_currencies {
            let field = Field32A::new(date, currency.to_string(), 1000.00);
            assert_eq!(
                field.decimal_places(),
                2,
                "Currency {} should have 2 decimal places",
                currency
            );
            assert!(field.has_decimal_places());
        }
    }

    #[test]
    fn test_field32a_high_value_transactions() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        // High-value USD transaction
        let high_usd = Field32A::new(date, "USD".to_string(), 1_500_000.00);
        assert!(high_usd.is_high_value_transaction());

        let normal_usd = Field32A::new(date, "USD".to_string(), 500_000.00);
        assert!(!normal_usd.is_high_value_transaction());

        // High-value JPY transaction (different threshold)
        let high_jpy = Field32A::new(date, "JPY".to_string(), 150_000_000.00);
        assert!(high_jpy.is_high_value_transaction());

        let normal_jpy = Field32A::new(date, "JPY".to_string(), 50_000_000.00);
        assert!(!normal_jpy.is_high_value_transaction());
    }

    #[test]
    fn test_field32a_settlement_timing() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        let test_cases = [
            ("USD", "T+0/T+1 (Same day or next day)"),
            ("EUR", "T+1 (Next day via TARGET2)"),
            ("GBP", "T+0 (Same day via CHAPS)"),
            ("JPY", "T+0/T+1 (Same day or next day)"),
            ("CHF", "T+0 (Same day via SIC)"),
            ("XYZ", "T+2 or longer (Depends on currency and market)"),
        ];

        for (currency, expected_timing) in test_cases {
            let field = Field32A::new(date, currency.to_string(), 1000.00);
            assert_eq!(
                field.settlement_timing(),
                expected_timing,
                "Settlement timing mismatch for currency {}",
                currency
            );
        }
    }

    #[test]
    fn test_field32a_value_date_analysis() {
        let today = chrono::Utc::now().date_naive();
        let future_date = today + chrono::Duration::days(5);
        let past_date = today - chrono::Duration::days(3);

        // Same-day value
        let same_day = Field32A::new(today, "USD".to_string(), 1000.00);
        assert!(same_day.is_same_day_value());
        assert!(!same_day.is_forward_dated());
        assert!(!same_day.is_back_dated());
        assert_eq!(same_day.days_until_value_date(), 0);

        // Forward-dated
        let forward = Field32A::new(future_date, "USD".to_string(), 1000.00);
        assert!(!forward.is_same_day_value());
        assert!(forward.is_forward_dated());
        assert!(!forward.is_back_dated());
        assert_eq!(forward.days_until_value_date(), 5);

        // Back-dated
        let back = Field32A::new(past_date, "USD".to_string(), 1000.00);
        assert!(!back.is_same_day_value());
        assert!(!back.is_forward_dated());
        assert!(back.is_back_dated());
        assert_eq!(back.days_until_value_date(), -3);
    }

    #[test]
    fn test_field32a_formatted_amount() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        // USD with 2 decimal places
        let usd_field = Field32A::new(date, "USD".to_string(), 1234.56);
        assert_eq!(usd_field.formatted_amount(), "1234.56");

        // JPY with 0 decimal places
        let jpy_field = Field32A::new(date, "JPY".to_string(), 1234.00);
        assert_eq!(jpy_field.formatted_amount(), "1234");

        // KWD with 3 decimal places
        let kwd_field = Field32A::new(date, "KWD".to_string(), 1234.567);
        assert_eq!(kwd_field.formatted_amount(), "1234.567");
    }

    #[test]
    fn test_field32a_from_raw() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        let field = Field32A::from_raw(date, "EUR".to_string(), "1000,50".to_string()).unwrap();
        assert_eq!(field.amount, 1000.50);
        assert_eq!(field.raw_amount, "1000,50");
        assert_eq!(field.currency, "EUR");

        // Test with dot separator
        let field = Field32A::from_raw(date, "USD".to_string(), "2500.75".to_string()).unwrap();
        assert_eq!(field.amount, 2500.75);
        assert_eq!(field.raw_amount, "2500.75");
    }

    #[test]
    fn test_field32a_comprehensive_description() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        // High-value USD transaction
        let field = Field32A::new(date, "USD".to_string(), 1_500_000.00);
        let desc = field.comprehensive_description();

        assert!(desc.contains("USD"));
        assert!(desc.contains("1500000.00"));
        assert!(desc.contains("2024-03-15"));
        assert!(desc.contains("Major currency"));
        assert!(desc.contains("High-value"));
        assert!(desc.contains("T+0/T+1"));

        // Normal EUR transaction
        let field = Field32A::new(date, "EUR".to_string(), 50_000.00);
        let desc = field.comprehensive_description();

        assert!(desc.contains("EUR"));
        assert!(desc.contains("50000.00"));
        assert!(desc.contains("Standard"));
        assert!(desc.contains("T+1"));
    }

    #[test]
    fn test_field32a_validation() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let field = Field32A::new(date, "USD".to_string(), 1000.00);

        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field32a_serialization() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let field = Field32A::new(date, "USD".to_string(), 1234.56);

        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: Field32A = serde_json::from_str(&serialized).unwrap();

        assert_eq!(field.value_date, deserialized.value_date);
        assert_eq!(field.currency, deserialized.currency);
        assert_eq!(field.amount, deserialized.amount);
        assert_eq!(field.raw_amount, deserialized.raw_amount);
    }

    #[test]
    fn test_field32a_business_logic_combinations() {
        let today = chrono::Utc::now().date_naive();

        // Major currency, high-value, forward-dated
        let future_date = today + chrono::Duration::days(7);
        let field = Field32A::new(future_date, "USD".to_string(), 2_000_000.00);

        assert!(field.is_major_currency());
        assert!(field.is_high_value_transaction());
        assert!(field.is_forward_dated());
        assert!(field.has_decimal_places());
        assert_eq!(field.decimal_places(), 2);
        assert_eq!(field.settlement_timing(), "T+0/T+1 (Same day or next day)");

        // Exotic currency, normal value, back-dated
        let past_date = today - chrono::Duration::days(2);
        let field = Field32A::new(past_date, "THB".to_string(), 100_000.00);

        assert!(!field.is_major_currency());
        assert!(!field.is_high_value_transaction());
        assert!(field.is_back_dated());
        assert!(field.has_decimal_places());
        assert_eq!(field.decimal_places(), 2);
        assert_eq!(
            field.settlement_timing(),
            "T+2 or longer (Depends on currency and market)"
        );
    }

    #[test]
    fn test_field32a_edge_cases() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(); // Leap year

        // Very small amount
        let small_field = Field32A::new(date, "USD".to_string(), 0.01);
        assert_eq!(small_field.amount, 0.01);
        assert!(!small_field.is_high_value_transaction());

        // Very large amount
        let large_field = Field32A::new(date, "USD".to_string(), 999_999_999.99);
        assert!(large_field.is_high_value_transaction());

        // JPY with fractional amount (should still work)
        let jpy_field = Field32A::new(date, "JPY".to_string(), 1000.50);
        assert_eq!(jpy_field.formatted_amount(), "1001"); // Rounded to 0 decimal places
    }

    #[test]
    fn test_field32a_real_world_scenarios() {
        // Scenario 1: International wire transfer
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let wire = Field32A::new(date, "USD".to_string(), 50_000.00);

        assert_eq!(wire.currency_code(), "USD");
        assert!(wire.is_major_currency());
        assert!(!wire.is_high_value_transaction());
        assert_eq!(wire.settlement_timing(), "T+0/T+1 (Same day or next day)");
        assert_eq!(wire.formatted_amount(), "50000.00");

        // Scenario 2: High-value EUR payment
        let eur_payment = Field32A::new(date, "EUR".to_string(), 1_200_000.00);

        assert!(eur_payment.is_high_value_transaction());
        assert_eq!(
            eur_payment.settlement_timing(),
            "T+1 (Next day via TARGET2)"
        );

        // Scenario 3: JPY transaction
        let jpy_payment = Field32A::new(date, "JPY".to_string(), 5_000_000.00);

        assert!(!jpy_payment.has_decimal_places());
        assert_eq!(jpy_payment.decimal_places(), 0);
        assert_eq!(jpy_payment.formatted_amount(), "5000000");

        // Scenario 4: Same-day GBP payment
        let today = chrono::Utc::now().date_naive();
        let gbp_payment = Field32A::new(today, "GBP".to_string(), 75_000.00);

        assert!(gbp_payment.is_same_day_value());
        assert_eq!(gbp_payment.settlement_timing(), "T+0 (Same day via CHAPS)");
        assert_eq!(gbp_payment.days_until_value_date(), 0);
    }

    #[test]
    fn test_field32a_currency_specific_behavior() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

        // Test specific currency behaviors
        let test_cases = [
            (
                "USD",
                true,
                2,
                1_000_000.0,
                "T+0/T+1 (Same day or next day)",
            ),
            ("EUR", true, 2, 1_000_000.0, "T+1 (Next day via TARGET2)"),
            (
                "JPY",
                true,
                0,
                100_000_000.0,
                "T+0/T+1 (Same day or next day)",
            ),
            (
                "KWD",
                false,
                3,
                1_000_000.0,
                "T+2 or longer (Depends on currency and market)",
            ),
            (
                "THB",
                false,
                2,
                1_000_000.0,
                "T+2 or longer (Depends on currency and market)",
            ),
        ];

        for (currency, is_major, decimal_places, high_value_threshold, settlement) in test_cases {
            let field = Field32A::new(date, currency.to_string(), 1000.00);

            assert_eq!(
                field.is_major_currency(),
                is_major,
                "Major currency check failed for {}",
                currency
            );
            assert_eq!(
                field.decimal_places(),
                decimal_places,
                "Decimal places check failed for {}",
                currency
            );
            assert_eq!(
                field.settlement_timing(),
                settlement,
                "Settlement timing check failed for {}",
                currency
            );

            // Test high-value threshold
            let high_value_field = Field32A::new(date, currency.to_string(), high_value_threshold);
            assert!(
                high_value_field.is_high_value_transaction(),
                "High value threshold check failed for {}",
                currency
            );
        }
    }
}
