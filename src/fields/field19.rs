//! **Field 19: Sum of Amounts**
//!
//! Sum of all individual transaction amounts in sequence transactions for reconciliation and validation.

use super::swift_utils::parse_amount;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 19: Sum of Amounts**
///
/// Sum of all individual transaction amounts in sequence for batch reconciliation.
///
/// **Format:** `17d` (up to 17 digits with decimal comma)
/// **Constraints:** Must equal sum of all Field 32B amounts
///
/// **Example:**
/// ```text
/// :19:123456,78
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field19 {
    /// Sum of transaction amounts (must match sum of Field 32B)
    pub amount: f64,
}

impl SwiftField for Field19 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let amount = parse_amount(input)?;

        Ok(Field19 { amount })
    }

    fn to_swift_string(&self) -> String {
        format!(":19:{}", format_swift_amount(self.amount))
    }
}

/// Format amount for SWIFT output with comma as decimal separator
fn format_swift_amount(amount: f64) -> String {
    let formatted = format!("{:.2}", amount);
    formatted.replace('.', ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field19_parse() {
        let field = Field19::parse("123456.78").unwrap();
        assert_eq!(field.amount, 123456.78);

        let field = Field19::parse("123456,78").unwrap();
        assert_eq!(field.amount, 123456.78);

        let field = Field19::parse("1000").unwrap();
        assert_eq!(field.amount, 1000.0);
    }

    #[test]
    fn test_field19_to_swift_string() {
        let field = Field19 { amount: 123456.78 };
        assert_eq!(field.to_swift_string(), ":19:123456,78");

        let field = Field19 { amount: 1000.0 };
        assert_eq!(field.to_swift_string(), ":19:1000,00");
    }

    #[test]
    fn test_field19_parse_invalid() {
        assert!(Field19::parse("abc").is_err());
        assert!(Field19::parse("").is_err());
    }
}
