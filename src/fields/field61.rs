use super::swift_utils::{parse_amount, parse_date_yymmdd, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

///   **Field 61: Statement Line**
///
/// ## Purpose
/// Represents individual transaction entries in customer statement messages (MT 940),
/// providing detailed information about each debit or credit transaction affecting
/// the account balance. This field is fundamental to statement processing, enabling
/// detailed transaction tracking, reconciliation, and audit trail maintenance.
///
/// ## Format Specification
/// - **Swift Format**: `6!n[4!n]2a[1!a]15d1!a3!c[16x][//16x][34x]`
/// - **Complex Structure**: Multiple components with optional elements
/// - **Variable Length**: Components can be present or absent based on transaction type
/// - **Structured Data**: Each component serves specific business purpose
///
/// ## Business Context Applications
/// - **Customer Statements**: Core component of MT 940 Customer Statement Message
/// - **Transaction Detail**: Complete transaction information for account holders
/// - **Reconciliation**: Detailed transaction data for account reconciliation
/// - **Audit Trail**: Complete transaction history for compliance and audit
///
/// ## Component Structure
/// ### Mandatory Components
/// - **Value Date**: Date when transaction affects account balance
/// - **Debit/Credit Mark**: Transaction direction (debit/credit)
/// - **Amount**: Transaction amount in account currency
/// - **Transaction Type**: Classification of transaction type
///
/// ### Optional Components
/// - **Entry Date**: Date transaction was posted (if different from value date)
/// - **Funds Code**: Availability of funds (immediate/float)
/// - **Customer Reference**: Transaction reference for customer
/// - **Bank Reference**: Bank's internal transaction reference
/// - **Supplementary Details**: Additional transaction information
///
/// ## Network Validation Requirements
/// - **Date Validation**: Value date must be valid calendar date
/// - **Amount Format**: Decimal amount with proper precision
/// - **Reference Format**: References must follow specified format rules
/// - **Transaction Code**: Must be valid transaction type code
/// - **Character Set**: All components must use valid character sets
///
/// ## Transaction Processing
/// ### Balance Impact
/// - **Debit Transactions**: Reduce account balance (payments, charges, withdrawals)
/// - **Credit Transactions**: Increase account balance (deposits, transfers, interest)
/// - **Reversal Transactions**: Correct previous transaction errors
/// - **Adjustment Transactions**: Administrative balance adjustments
///
/// ### Transaction Types
/// - **Customer Transfers**: Payments and receipts
/// - **Bank Services**: Fees, charges, and service transactions
/// - **Interest**: Interest credits and debits
/// - **Foreign Exchange**: Currency conversion transactions
///
/// ## Regional Considerations
/// - **European Banking**: SEPA transaction processing and reporting
/// - **US Banking**: ACH, wire transfer, and check processing
/// - **Asian Markets**: Local payment system integration
/// - **Cross-Border**: International transaction processing
///
/// ## Error Prevention Guidelines
/// - **Date Consistency**: Verify dates are logical and within statement period
/// - **Amount Verification**: Confirm amount format and precision
/// - **Reference Validation**: Ensure references follow format requirements
/// - **Balance Verification**: Confirm transactions sum to balance changes
///
/// ## Related Fields Integration
/// - **Field 60**: Opening Balance (starting position)
/// - **Field 62**: Closing Balance (ending position after transactions)
/// - **Field 64**: Closing Available Balance (availability impact)
/// - **Field 86**: Information to Account Owner (additional details)
///
/// ## Compliance Framework
/// - **Banking Regulations**: Transaction reporting requirements
/// - **Audit Standards**: Complete transaction documentation
/// - **Customer Rights**: Detailed transaction information provision
/// - **Data Retention**: Transaction history retention requirements
///
/// ## See Also
/// - Swift FIN User Handbook: Statement Line Specifications
/// - MT 940 Message Standards: Customer Statement Processing
/// - Transaction Processing: Banking Transaction Standards
/// - Account Statement Requirements: Regional Banking Regulations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field61 {
    /// Value date (6!n format, YYMMDD)
    pub value_date: NaiveDate,

    /// Optional entry date (4!n format, MMDD)
    pub entry_date: Option<String>,

    /// Debit/Credit mark (2a format: D, C, RD, RC)
    pub debit_credit_mark: String,

    /// Optional funds code (1!a format)
    pub funds_code: Option<char>,

    /// Amount (15d format)
    pub amount: f64,

    /// Transaction type identification code (4!a format)
    pub transaction_type: String,

    /// Customer reference (16x format - up to 16 characters)
    pub customer_reference: String,

    /// Bank reference (16x format, preceded by //)
    pub bank_reference: Option<String>,

    /// Optional supplementary details (34x format)
    pub supplementary_details: Option<String>,
}

impl SwiftField for Field61 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Format: 6!n[4!n]2a[1!a]15d1!a3!c[16x][//16x][34x]
        if input.len() < 15 {
            return Err(ParseError::InvalidFormat {
                message: "Field 61 must be at least 15 characters long".to_string(),
            });
        }

        let mut pos = 0;

        // Parse value date (6 digits, mandatory)
        if input.len() < pos + 6 {
            return Err(ParseError::InvalidFormat {
                message: "Field 61 missing value date".to_string(),
            });
        }
        let value_date_str = &input[pos..pos + 6];
        let value_date = parse_date_yymmdd(value_date_str)?;
        pos += 6;

        // Parse optional entry date (4 digits)
        let mut entry_date = None;
        if pos + 4 <= input.len() && input[pos..pos + 4].chars().all(|c| c.is_ascii_digit()) {
            entry_date = Some(input[pos..pos + 4].to_string());
            pos += 4;
        }

        // Parse debit/credit mark (2 characters maximum, but could be 1)
        if pos >= input.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 61 missing debit/credit mark".to_string(),
            });
        }

        let mut dc_mark_len = 1;
        if pos + 1 < input.len() {
            let two_char = &input[pos..pos + 2];
            if two_char == "RD" || two_char == "RC" {
                dc_mark_len = 2;
            }
        }

        let debit_credit_mark = input[pos..pos + dc_mark_len].to_string();
        if !["D", "C", "RD", "RC"].contains(&debit_credit_mark.as_str()) {
            return Err(ParseError::InvalidFormat {
                message: format!("Field 61 invalid debit/credit mark: {}", debit_credit_mark),
            });
        }
        pos += dc_mark_len;

        // Parse optional funds code (1 character)
        let mut funds_code = None;
        if pos < input.len() && input.chars().nth(pos).unwrap().is_alphabetic() {
            funds_code = Some(input.chars().nth(pos).unwrap());
            pos += 1;
        }

        // Parse amount - find the next alphabetic character to determine where amount ends
        let amount_start = pos;
        while pos < input.len()
            && (input.chars().nth(pos).unwrap().is_ascii_digit()
                || input.chars().nth(pos).unwrap() == ','
                || input.chars().nth(pos).unwrap() == '.')
        {
            pos += 1;
        }

        if pos == amount_start {
            return Err(ParseError::InvalidFormat {
                message: "Field 61 missing amount".to_string(),
            });
        }

        let amount_str = &input[amount_start..pos];
        let amount = parse_amount(amount_str)?;

        // Parse transaction type (4 characters: 1!a3!c)
        if pos + 4 > input.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 61 missing transaction type".to_string(),
            });
        }

        let transaction_type = input[pos..pos + 4].to_string();
        parse_swift_chars(&transaction_type, "Field 61 transaction type")?;
        pos += 4;

        // Parse customer reference (up to 16 characters until // or end)
        let remaining = &input[pos..];
        let (customer_ref_part, after_customer_ref) =
            if let Some(double_slash_pos) = remaining.find("//") {
                (
                    remaining[..double_slash_pos].to_string(),
                    Some(&remaining[double_slash_pos + 2..]),
                )
            } else {
                (remaining.to_string(), None)
            };

        // Split customer reference and supplementary details
        let customer_reference;
        let supplementary_details;

        if customer_ref_part.len() <= 16 {
            customer_reference = customer_ref_part;
            supplementary_details = None;
        } else {
            customer_reference = customer_ref_part[..16].to_string();
            if customer_ref_part.len() > 16 {
                supplementary_details = Some(customer_ref_part[16..].to_string());
            } else {
                supplementary_details = None;
            }
        }

        // Parse bank reference (after //)
        let bank_reference = if let Some(bank_ref_str) = after_customer_ref {
            if bank_ref_str.len() > 16 {
                Some(bank_ref_str[..16].to_string())
            } else if !bank_ref_str.is_empty() {
                Some(bank_ref_str.to_string())
            } else {
                None
            }
        } else {
            None
        };

        // Validate customer reference length
        if customer_reference.len() > 16 {
            return Err(ParseError::InvalidFormat {
                message: "Field 61 customer reference exceeds 16 characters".to_string(),
            });
        }

        parse_swift_chars(&customer_reference, "Field 61 customer reference")?;

        if let Some(ref bank_ref) = bank_reference {
            parse_swift_chars(bank_ref, "Field 61 bank reference")?;
        }

        if let Some(ref supp_details) = supplementary_details {
            if supp_details.len() > 34 {
                return Err(ParseError::InvalidFormat {
                    message: "Field 61 supplementary details exceed 34 characters".to_string(),
                });
            }
            parse_swift_chars(supp_details, "Field 61 supplementary details")?;
        }

        Ok(Field61 {
            value_date,
            entry_date,
            debit_credit_mark,
            funds_code,
            amount,
            transaction_type,
            customer_reference,
            bank_reference,
            supplementary_details,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = format!(":61:{}", self.value_date.format("%y%m%d"));

        if let Some(ref entry_date) = self.entry_date {
            result.push_str(entry_date);
        }

        result.push_str(&self.debit_credit_mark);

        if let Some(funds_code) = self.funds_code {
            result.push(funds_code);
        }

        result.push_str(&format!("{:.2}", self.amount).replace('.', ","));
        result.push_str(&self.transaction_type);
        result.push_str(&self.customer_reference);

        if let Some(ref bank_reference) = self.bank_reference {
            result.push_str("//");
            result.push_str(bank_reference);
        }

        if let Some(ref supplementary_details) = self.supplementary_details {
            result.push_str(supplementary_details);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_field61_parse_basic() {
        let field = Field61::parse("231225D1234,56NTRFREF123456").unwrap();
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(field.entry_date, None);
        assert_eq!(field.debit_credit_mark, "D");
        assert_eq!(field.funds_code, None);
        assert_eq!(field.amount, 1234.56);
        assert_eq!(field.transaction_type, "NTRF");
        assert_eq!(field.customer_reference, "REF123456");
        assert_eq!(field.bank_reference, None);
        assert_eq!(field.supplementary_details, None);
    }

    #[test]
    fn test_field61_parse_with_entry_date() {
        let field = Field61::parse("2312251226C500,00NTRFREF789//BANK456").unwrap();
        assert_eq!(
            field.value_date,
            NaiveDate::from_ymd_opt(2023, 12, 25).unwrap()
        );
        assert_eq!(field.entry_date, Some("1226".to_string()));
        assert_eq!(field.debit_credit_mark, "C");
        assert_eq!(field.funds_code, None);
        assert_eq!(field.amount, 500.00);
        assert_eq!(field.transaction_type, "NTRF");
        assert_eq!(field.customer_reference, "REF789");
        assert_eq!(field.bank_reference, Some("BANK456".to_string()));
    }

    #[test]
    fn test_field61_parse_with_funds_code() {
        let field = Field61::parse("231225DF100,00NTRFCUSTREF").unwrap();
        assert_eq!(field.debit_credit_mark, "D");
        assert_eq!(field.funds_code, Some('F'));
        assert_eq!(field.amount, 100.00);
    }

    #[test]
    fn test_field61_parse_reversal() {
        let field = Field61::parse("231225RD1000,00NTRFREVREF123").unwrap();
        assert_eq!(field.debit_credit_mark, "RD");
        assert_eq!(field.amount, 1000.00);
    }

    #[test]
    fn test_field61_to_swift_string() {
        let field = Field61 {
            value_date: NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
            entry_date: Some("1226".to_string()),
            debit_credit_mark: "C".to_string(),
            funds_code: Some('F'),
            amount: 1234.56,
            transaction_type: "NTRF".to_string(),
            customer_reference: "REF123456".to_string(),
            bank_reference: Some("BANK789".to_string()),
            supplementary_details: None,
        };

        assert_eq!(
            field.to_swift_string(),
            ":61:2312251226CF1234,56NTRFREF123456//BANK789"
        );
    }

    #[test]
    fn test_field61_invalid_debit_credit_mark() {
        assert!(Field61::parse("231225X1234,56NTRFREF123").is_err());
    }

    #[test]
    fn test_field61_too_short() {
        assert!(Field61::parse("23122").is_err());
    }
}
