use super::swift_utils::{
    format_swift_amount_for_currency, parse_amount_with_currency, parse_currency_non_commodity,
    parse_exact_length, parse_uppercase,
};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 71A: Details of Charges**
///
/// Charge allocation code specifying which party bears transaction costs.
///
/// **Format:** `3!a`
/// **Values:** `BEN` (beneficiary pays), `OUR` (sender pays), `SHA` (shared)
///
/// **Example:**
/// ```text
/// :71A:SHA
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71A {
    /// Charge code: BEN, OUR, or SHA
    pub code: String,
}

impl SwiftField for Field71A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Must be exactly 3 characters
        let code = parse_exact_length(input, 3, "Field 71A code")?;

        // Must be uppercase alphabetic
        parse_uppercase(&code, "Field 71A code")?;

        // Validate against known codes
        const VALID_CODES: &[&str] = &["BEN", "OUR", "SHA"];
        if !VALID_CODES.contains(&code.as_str()) {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 71A code must be one of {:?}, found {}",
                    VALID_CODES, code
                ),
            });
        }

        Ok(Field71A { code })
    }

    fn to_swift_string(&self) -> String {
        format!(":71A:{}", self.code)
    }
}

/// **Field 71F: Sender's Charges**
///
/// Currency and amount of charges borne by sender.
///
/// **Format:** `3!a15d` (currency + amount)
///
/// **Example:**
/// ```text
/// :71F:USD25,00
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71F {
    /// ISO 4217 currency code
    pub currency: String,
    /// Charge amount
    pub amount: f64,
}

impl SwiftField for Field71F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.len() < 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 71F must be at least 4 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse currency (first 3 characters) - T52 + C08 validation
        let currency = parse_currency_non_commodity(&input[0..3])?;

        // Parse amount (remaining characters, max 15 digits) - T40/T43 + C03 validation
        let amount_str = &input[3..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 71F amount is required".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        Ok(Field71F { currency, amount })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":71F:{}{}",
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

/// **Field 71G: Receiver's Charges**
///
/// Currency and amount of charges borne by receiver.
///
/// **Format:** `3!a15d` (currency + amount)
///
/// **Example:**
/// ```text
/// :71G:EUR10,50
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71G {
    /// ISO 4217 currency code
    pub currency: String,
    /// Charge amount
    pub amount: f64,
}

impl SwiftField for Field71G {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.len() < 4 {
            return Err(ParseError::InvalidFormat {
                message: format!(
                    "Field 71G must be at least 4 characters, found {}",
                    input.len()
                ),
            });
        }

        // Parse currency (first 3 characters) - T52 + C08 validation
        let currency = parse_currency_non_commodity(&input[0..3])?;

        // Parse amount (remaining characters, max 15 digits) - T40/T43 + C03 validation
        let amount_str = &input[3..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 71G amount is required".to_string(),
            });
        }

        let amount = parse_amount_with_currency(amount_str, &currency)?;

        Ok(Field71G { currency, amount })
    }

    fn to_swift_string(&self) -> String {
        format!(
            ":71G:{}{}",
            self.currency,
            format_swift_amount_for_currency(self.amount, &self.currency)
        )
    }
}

/// **Field 71B: Details of Charges**
///
/// Detailed charge breakdown including interest and adjustments.
/// Used in MT n90 messages (MT190, MT290, etc.).
///
/// **Format:** `6*35x` (max 6 lines, 35 chars each)
///
/// **Example:**
/// ```text
/// :71B:COMMISSION USD 25.00
/// INTEREST USD 10.50
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71B {
    /// Charge details (max 6 lines, 35 chars each)
    pub details: Vec<String>,
}

impl SwiftField for Field71B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use super::field_utils::parse_multiline_text;

        // Parse as multiline text (up to 6 lines, 35 chars each)
        let details = parse_multiline_text(input, 6, 35)?;

        if details.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 71B must have at least one line of details".to_string(),
            });
        }

        Ok(Field71B { details })
    }

    fn to_swift_string(&self) -> String {
        format!(":71B:{}", self.details.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field71a() {
        // Test valid codes
        let field = Field71A::parse("BEN").unwrap();
        assert_eq!(field.code, "BEN");

        let field = Field71A::parse("OUR").unwrap();
        assert_eq!(field.code, "OUR");

        let field = Field71A::parse("SHA").unwrap();
        assert_eq!(field.code, "SHA");

        // Test invalid length
        assert!(Field71A::parse("BE").is_err());
        assert!(Field71A::parse("BENE").is_err());

        // Test invalid code
        assert!(Field71A::parse("XXX").is_err());

        // Test lowercase (should fail)
        assert!(Field71A::parse("ben").is_err());

        // Test to_swift_string
        let field = Field71A {
            code: "SHA".to_string(),
        };
        assert_eq!(field.to_swift_string(), ":71A:SHA");
    }

    #[test]
    fn test_field71f() {
        // Test basic parsing
        let field = Field71F::parse("USD100.50").unwrap();
        assert_eq!(field.currency, "USD");
        assert_eq!(field.amount, 100.50);

        // Test with comma (European format)
        let field = Field71F::parse("EUR1234,56").unwrap();
        assert_eq!(field.currency, "EUR");
        assert_eq!(field.amount, 1234.56);

        // Test integer amount
        let field = Field71F::parse("GBP500").unwrap();
        assert_eq!(field.currency, "GBP");
        assert_eq!(field.amount, 500.0);

        // Test to_swift_string
        let field = Field71F {
            currency: "USD".to_string(),
            amount: 250.75,
        };
        assert_eq!(field.to_swift_string(), ":71F:USD250,75");

        // Test invalid currency
        assert!(Field71F::parse("US100").is_err());
        assert!(Field71F::parse("123100").is_err());

        // Test missing amount
        assert!(Field71F::parse("USD").is_err());
    }

    #[test]
    fn test_field71b() {
        // Test single line
        let field = Field71B::parse("COMMISSION USD 25.00").unwrap();
        assert_eq!(field.details.len(), 1);
        assert_eq!(field.details[0], "COMMISSION USD 25.00");

        // Test multiline
        let input = "COMMISSION USD 25.00\nINTEREST USD 10.50\nSERVICE FEE USD 5.00";
        let field = Field71B::parse(input).unwrap();
        assert_eq!(field.details.len(), 3);
        assert_eq!(field.details[0], "COMMISSION USD 25.00");
        assert_eq!(field.details[1], "INTEREST USD 10.50");
        assert_eq!(field.details[2], "SERVICE FEE USD 5.00");

        // Test to_swift_string
        assert_eq!(field.to_swift_string(), format!(":71B:{}", input));

        // Test format spec
    }

    #[test]
    fn test_field71g() {
        // Test basic parsing
        let field = Field71G::parse("CHF75.25").unwrap();
        assert_eq!(field.currency, "CHF");
        assert_eq!(field.amount, 75.25);

        // Test large amount
        let field = Field71G::parse("JPY1000000").unwrap();
        assert_eq!(field.currency, "JPY");
        assert_eq!(field.amount, 1000000.0);

        // Test to_swift_string
        let field = Field71G {
            currency: "CAD".to_string(),
            amount: 99.99,
        };
        assert_eq!(field.to_swift_string(), ":71G:CAD99,99");

        // Test format spec
    }
}
