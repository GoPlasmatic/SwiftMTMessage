use super::swift_utils::{parse_amount, parse_date_yymmdd, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// **Field 61: Statement Line**
///
/// Individual transaction entry in account statements (MT 940).
///
/// **Format:** `6!n[4!n]2a[1!a]15d1!a3!c[16x][//16x][34x]`
/// **Constraints:** D/C mark must be D, C, RD, or RC
///
/// **Example:**
/// ```text
/// :61:231225D1234,56NTRFREF123456
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field61 {
    /// Value date (YYMMDD)
    pub value_date: NaiveDate,

    /// Entry date (MMDD, optional)
    pub entry_date: Option<String>,

    /// Debit/Credit mark (D, C, RD, or RC)
    pub debit_credit_mark: String,

    /// Funds code (optional)
    pub funds_code: Option<char>,

    /// Transaction amount
    pub amount: f64,

    /// Transaction type code (4 chars)
    pub transaction_type: String,

    /// Customer reference (max 16 chars)
    pub customer_reference: String,

    /// Bank reference (max 16 chars, optional)
    pub bank_reference: Option<String>,

    /// Supplementary details (max 34 chars, optional)
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

        // Customer reference is up to 16 characters
        let customer_reference;
        let mut supplementary_details = None;

        if customer_ref_part.len() <= 16 {
            customer_reference = customer_ref_part;
        } else {
            customer_reference = customer_ref_part[..16].to_string();
            // If customer ref part is > 16 chars and no //, rest is supplementary details
            if after_customer_ref.is_none() && customer_ref_part.len() > 16 {
                supplementary_details = Some(customer_ref_part[16..].to_string());
            }
        }

        // Parse bank reference and supplementary details (after //)
        // Format after //: bank_reference[16x][\n]supplementary_details[34x]
        // Supplementary details may be on a new line or directly concatenated
        let bank_reference = if let Some(bank_ref_str) = after_customer_ref {
            // Check if there's a newline separating bank ref from supplementary details
            if let Some(newline_pos) = bank_ref_str.find('\n') {
                // Bank reference is before newline, supplementary details after
                let bank_ref = bank_ref_str[..newline_pos].to_string();
                if newline_pos + 1 < bank_ref_str.len() {
                    supplementary_details = Some(bank_ref_str[newline_pos + 1..].to_string());
                }
                Some(bank_ref)
            } else if bank_ref_str.len() > 16 {
                // No newline, but string is longer than bank ref max
                // First 16 chars = bank reference, rest = supplementary details
                supplementary_details = Some(bank_ref_str[16..].to_string());
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

            // Supplementary details come on new line after bank reference if present
            if let Some(ref supplementary_details) = self.supplementary_details {
                result.push('\n');
                result.push_str(supplementary_details);
            }
        } else if let Some(ref supplementary_details) = self.supplementary_details {
            // If no bank reference but supplementary details exist, append after customer ref
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

    #[test]
    fn test_field61_with_supplementary_details() {
        // Test with bank reference and supplementary details (newline separated)
        let field =
            Field61::parse("2412201220C10000,00NMSCREF100000//BA1-1234567890\nDUPLICATE-SEQ-1")
                .unwrap();
        assert_eq!(field.customer_reference, "REF100000");
        assert_eq!(field.bank_reference, Some("BA1-1234567890".to_string()));
        assert_eq!(
            field.supplementary_details,
            Some("DUPLICATE-SEQ-1".to_string())
        );

        // Test round-trip
        let swift_str = field.to_swift_string();
        let reparsed = Field61::parse(&swift_str.replace(":61:", "")).unwrap();
        assert_eq!(reparsed.customer_reference, field.customer_reference);
        assert_eq!(reparsed.bank_reference, field.bank_reference);
        assert_eq!(reparsed.supplementary_details, field.supplementary_details);
    }
}
