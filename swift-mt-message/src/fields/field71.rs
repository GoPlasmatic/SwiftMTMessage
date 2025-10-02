use super::swift_utils::{parse_amount, parse_currency, parse_exact_length, parse_uppercase};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 71: Charges and Fee Information**
///
/// ## Purpose
/// Specifies charge allocation and fee details for payment transactions. This field family
/// determines which party bears transaction costs and provides detailed charge amounts
/// for various fees associated with payment processing. Essential for transparent
/// cost allocation and compliance with payment regulations.
///
/// ## Field Options Overview
/// - **Field 71A**: Details of Charges (charge allocation code)
/// - **Field 71F**: Sender's Charges (specific charge amounts)
/// - **Field 71G**: Receiver's Charges (additional charge amounts)
///
/// ## Business Context Applications
/// - **Payment Processing**: Charge allocation in MT 103 and other payment messages
/// - **Cost Transparency**: Clear identification of transaction costs
/// - **Regulatory Compliance**: Meeting charge disclosure requirements
/// - **Customer Communication**: Transparent fee structure communication
///
/// ## Charge Allocation Principles
/// ### Allocation Options (Field 71A)
/// - **BEN**: Beneficiary bears all charges
/// - **OUR**: Ordering customer bears all charges
/// - **SHA**: Shared charges (sender pays own bank, beneficiary pays others)
///
/// ### Charge Types
/// - **Correspondent Charges**: Fees charged by intermediary banks
/// - **Beneficiary Bank Charges**: Fees charged by receiving bank
/// - **Service Charges**: Additional service fees
/// - **Conversion Charges**: Currency conversion fees
///
/// ## Regional Considerations
/// - **European Payments**: SEPA charge regulations and transparency requirements
/// - **US Payments**: Federal Reserve and commercial bank fee structures
/// - **Asian Markets**: Local charge allocation practices
/// - **Cross-Border**: International payment fee coordination
///
/// ## Error Prevention Guidelines
/// - **Code Validation**: Verify charge allocation codes are valid
/// - **Amount Verification**: Confirm charge amounts are reasonable
/// - **Currency Consistency**: Ensure charge currency matches context
/// - **Disclosure Compliance**: Meet regulatory charge disclosure requirements
///
/// ## Related Fields Integration
/// - **Field 32A**: Value Date, Currency, Amount (transaction amount context)
/// - **Field 33B**: Currency/Instructed Amount (original amount before charges)
/// - **Field 72**: Sender to Receiver Information (charge instructions)
/// - **Field 64**: Closing Available Balance (net amount after charges)
///
/// ## Compliance Framework
/// - **Regulatory Requirements**: Charge transparency and disclosure regulations
/// - **Consumer Protection**: Clear charge communication requirements
/// - **Fee Regulation**: Compliance with local fee regulation standards
/// - **Audit Documentation**: Complete charge allocation documentation
///
/// ## See Also
/// - Swift FIN User Handbook: Charge Field Specifications
/// - Payment Regulations: Charge Transparency Requirements
/// - Banking Fee Standards: International Charge Allocation
/// - Customer Protection: Charge Disclosure Guidelines
///
///   **Field 71A: Details of Charges**
///
/// Specifies which party will bear the charges for the transaction.
/// Mandatory field in payment messages for charge allocation transparency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71A {
    /// Charge allocation code
    ///
    /// Format: 3!a - Three alphabetic characters
    /// Values: BEN (Beneficiary), OUR (Ordering customer), SHA (Shared)
    /// Error T08 if invalid code used
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

///   **Field 71F: Sender's Charges**
///
/// Specifies the currency and amount of charges to be borne by the sender.
/// Used to detail specific charge amounts in sender's currency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71F {
    /// Currency of sender's charges
    ///
    /// Format: 3!a - ISO 4217 currency code (USD, EUR, GBP, etc.)
    /// Must be valid currency for charge specification
    pub currency: String,

    /// Amount of sender's charges
    ///
    /// Format: 15d - Decimal amount with comma separator
    /// Precision must match currency requirements
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

        // Parse currency (first 3 characters)
        let currency = parse_currency(&input[0..3])?;

        // Parse amount (remaining characters, max 15 digits)
        let amount_str = &input[3..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 71F amount is required".to_string(),
            });
        }

        let amount = parse_amount(amount_str)?;

        Ok(Field71F { currency, amount })
    }

    fn to_swift_string(&self) -> String {
        format!(":71F:{}{}", self.currency, self.amount)
    }
}

///   **Field 71G: Receiver's Charges**
///
/// Specifies the currency and amount of charges to be borne by the receiver.
/// Used in conjunction with Field 71F for complete charge specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71G {
    /// Currency of receiver's charges
    ///
    /// Format: 3!a - ISO 4217 currency code
    /// Must match or be convertible to receiver's currency
    pub currency: String,

    /// Amount of receiver's charges
    ///
    /// Format: 15d - Decimal amount with proper precision
    /// Must be positive value within reasonable limits
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

        // Parse currency (first 3 characters)
        let currency = parse_currency(&input[0..3])?;

        // Parse amount (remaining characters, max 15 digits)
        let amount_str = &input[3..];
        if amount_str.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 71G amount is required".to_string(),
            });
        }

        let amount = parse_amount(amount_str)?;

        Ok(Field71G { currency, amount })
    }

    fn to_swift_string(&self) -> String {
        format!(":71G:{}{}", self.currency, self.amount)
    }
}

///   **Field 71B: Details of Charges**
///
/// Specifies detailed information about charges, interest and other adjustments.
/// Used in MT n90 messages (MT190, MT290, etc.) to provide comprehensive charge details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field71B {
    /// Details of charges
    ///
    /// Format: 6*35x - Up to 6 lines of 35 characters each
    /// Contains detailed breakdown of charges, interest and other adjustments
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
        assert_eq!(field.to_swift_string(), ":71F:USD250.75");

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
        assert_eq!(field.to_swift_string(), ":71G:CAD99.99");

        // Test format spec
    }
}
