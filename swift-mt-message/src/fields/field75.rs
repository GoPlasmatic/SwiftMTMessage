use crate::fields::MultiLineField;
use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 75: Queries
///
/// ## Overview
/// Field 75 contains query information with predefined codes and narrative text, used in
/// SWIFT MT messages for stop payment queries and reasons. This field supports structured
/// query codes along with free-form narrative text to provide detailed query information
/// and reasons for payment-related requests.
///
/// ## Format Specification
/// **Format**: `6*35x`
/// - **6*35x**: Up to 6 lines of 35 characters each
/// - **Character set**: SWIFT character set (A-Z, 0-9, and limited special characters)
/// - **Line structure**: Each line can contain predefined codes or narrative text
/// - **Predefined codes**: 3, 18, 19, 20, 21 (with optional narrative following)
/// - **Validation**: Each line must not exceed 35 characters
///
/// ## Structure
/// ```text
/// 3 STOP PAYMENT REQUEST
/// INSUFFICIENT FUNDS DETECTED
/// 18 ACCOUNT CLOSED
/// PLEASE INVESTIGATE URGENTLY
/// ```
///
/// ## Predefined Query Codes
/// - **Code 3**: Stop payment request with reason
/// - **Code 18**: Account status queries (closed, frozen, etc.)
/// - **Code 19**: Payment instruction queries
/// - **Code 20**: Beneficiary verification queries
/// - **Code 21**: Regulatory or compliance queries
///
/// ## Usage Context
/// Field 75 is used in:
/// - **MT111**: Request for Stop Payment of a Cheque
/// - **MT195**: Queries (Customer or FI)
/// - **MT196**: Answers (Customer or FI)
/// - **MT299**: Free Format Message (for queries)
///
/// ### Business Applications
/// - **Stop payment requests**: Providing reasons for cheque stop payments
/// - **Payment inquiries**: Querying status of pending payments
/// - **Account verification**: Requesting account status confirmation
/// - **Regulatory queries**: Compliance-related information requests
/// - **Error resolution**: Investigating payment processing issues
/// - **Documentation requests**: Seeking supporting documentation
///
/// ## Examples
/// ```text
/// :75:3 STOP PAYMENT REQUEST
/// CHEQUE REPORTED STOLEN
/// POLICE REPORT FILED
///
/// :75:18 ACCOUNT STATUS QUERY
/// PLEASE CONFIRM ACCOUNT ACTIVE
/// URGENT PAYMENT PENDING
///
/// :75:20 BENEFICIARY VERIFICATION
/// CONFIRM NAME AND ADDRESS
/// AML COMPLIANCE CHECK
/// ```
///
/// ## Validation Rules
/// 1. **Line count**: Maximum 6 lines
/// 2. **Line length**: Maximum 35 characters per line
/// 3. **Content**: Cannot be empty
/// 4. **Character validation**: SWIFT character set compliance
/// 5. **Predefined codes**: Must be followed by space if used
/// 6. **Narrative text**: Free-form but within character limits
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Maximum 6 lines allowed (Error: T26)
/// - Each line cannot exceed 35 characters (Error: T50)
/// - Must use SWIFT character set only (Error: T61)
/// - Cannot be empty field (Error: T13)
/// - Predefined codes must be properly formatted (Error: T51)
/// - Content must be meaningful and relevant (Error: T52)
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field75 {
    /// Query lines (up to 6 lines of 35 characters each)
    pub queries: Vec<String>,
}

impl MultiLineField for Field75 {
    const MAX_LINES: usize = 6;
    const FIELD_TAG: &'static str = "75";

    fn lines(&self) -> &[String] {
        &self.queries
    }

    fn lines_mut(&mut self) -> &mut Vec<String> {
        &mut self.queries
    }

    fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
        Ok(Field75 { queries: lines })
    }
}

impl Field75 {
    /// Create a new Field75 with validation
    pub fn new(queries: Vec<String>) -> Result<Self, ParseError> {
        <Self as MultiLineField>::new(queries)
    }

    /// Create from a single string, splitting on newlines
    pub fn from_string(content: impl Into<String>) -> Result<Self, ParseError> {
        let content = content.into();
        let lines: Vec<String> = content.lines().map(|s| s.trim().to_string()).collect();
        Self::new(lines)
    }

    /// Get the query lines
    pub fn queries(&self) -> &[String] {
        &self.queries
    }

    /// Check if contains predefined query codes (3, 18, 19, 20, 21)
    pub fn has_predefined_codes(&self) -> bool {
        let predefined_codes = ["3", "18", "19", "20", "21"];
        self.queries.iter().any(|line| {
            predefined_codes.iter().any(|code| {
                line.starts_with(code)
                    && (line.len() == code.len() || line.chars().nth(code.len()) == Some(' '))
            })
        })
    }

    /// Get all predefined codes found in the queries
    pub fn predefined_codes(&self) -> Vec<String> {
        let predefined_codes = ["3", "18", "19", "20", "21"];
        let mut found_codes = Vec::new();

        for line in &self.queries {
            for code in predefined_codes {
                if line.starts_with(code)
                    && (line.len() == code.len() || line.chars().nth(code.len()) == Some(' '))
                {
                    found_codes.push(code.to_string());
                }
            }
        }

        found_codes.sort_unstable();
        found_codes.dedup();
        found_codes
    }

    /// Check if this is a stop payment query (code 3)
    pub fn is_stop_payment_query(&self) -> bool {
        self.queries.iter().any(|line| {
            line.starts_with("3") && (line.len() == 1 || line.chars().nth(1) == Some(' '))
        })
    }

    /// Check if this is an account status query (code 18)
    pub fn is_account_status_query(&self) -> bool {
        self.queries.iter().any(|line| {
            line.starts_with("18") && (line.len() == 2 || line.chars().nth(2) == Some(' '))
        })
    }

    /// Check if this is a payment instruction query (code 19)
    pub fn is_payment_instruction_query(&self) -> bool {
        self.queries.iter().any(|line| {
            line.starts_with("19") && (line.len() == 2 || line.chars().nth(2) == Some(' '))
        })
    }

    /// Check if this is a beneficiary verification query (code 20)
    pub fn is_beneficiary_verification_query(&self) -> bool {
        self.queries.iter().any(|line| {
            line.starts_with("20") && (line.len() == 2 || line.chars().nth(2) == Some(' '))
        })
    }

    /// Check if this is a regulatory/compliance query (code 21)
    pub fn is_regulatory_query(&self) -> bool {
        self.queries.iter().any(|line| {
            line.starts_with("21") && (line.len() == 2 || line.chars().nth(2) == Some(' '))
        })
    }

    /// Get query type description based on predefined codes
    pub fn query_type(&self) -> String {
        let codes = self.predefined_codes();
        if codes.is_empty() {
            "General Query".to_string()
        } else {
            let mut types = Vec::new();
            for code in codes {
                match code.as_str() {
                    "3" => types.push("Stop Payment"),
                    "18" => types.push("Account Status"),
                    "19" => types.push("Payment Instruction"),
                    "20" => types.push("Beneficiary Verification"),
                    "21" => types.push("Regulatory/Compliance"),
                    _ => {}
                }
            }
            if types.is_empty() {
                "General Query".to_string()
            } else {
                types.join(", ")
            }
        }
    }

    /// Get priority level based on query codes (higher number = higher priority)
    pub fn priority_level(&self) -> u8 {
        if self.is_stop_payment_query() {
            5 // Highest priority
        } else if self.is_regulatory_query() {
            4 // High priority
        } else if self.is_beneficiary_verification_query() {
            3 // Medium-high priority
        } else if self.is_account_status_query() || self.is_beneficiary_verification_query() {
            2 // Medium priority
        } else {
            1 // Standard priority
        }
    }

    /// Check if query requires urgent attention
    pub fn is_urgent(&self) -> bool {
        self.priority_level() >= 4
            || self.queries.iter().any(|line| {
                let line_upper = line.to_uppercase();
                line_upper.contains("URGENT")
                    || line_upper.contains("IMMEDIATE")
                    || line_upper.contains("ASAP")
                    || line_upper.contains("PRIORITY")
            })
    }

    /// Get comprehensive description including query type and content
    pub fn comprehensive_description(&self) -> String {
        format!(
            "{} Query ({} lines): {}",
            self.query_type(),
            self.line_count(),
            if self.is_urgent() { "[URGENT] " } else { "" }
        )
    }
}

impl SwiftField for Field75 {
    fn parse(content: &str) -> Result<Self, ParseError> {
        Self::parse_content(content)
    }

    fn to_swift_string(&self) -> String {
        self.to_swift_format()
    }

    fn validate(&self) -> ValidationResult {
        self.validate_multiline()
    }

    fn format_spec() -> &'static str {
        "6*35x"
    }
}

impl std::fmt::Display for Field75 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.queries.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field75_creation() {
        let queries = vec![
            "3 STOP PAYMENT REQUEST".to_string(),
            "INSUFFICIENT FUNDS".to_string(),
        ];
        let field = Field75::new(queries.clone()).unwrap();
        assert_eq!(field.queries(), &queries);
        assert_eq!(field.line_count(), 2);
    }

    #[test]
    fn test_field75_from_string() {
        let content = "3 STOP PAYMENT REQUEST\nINSUFFICIENT FUNDS\n18 ACCOUNT CLOSED";
        let field = Field75::from_string(content).unwrap();
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.queries()[0], "3 STOP PAYMENT REQUEST");
        assert_eq!(field.queries()[1], "INSUFFICIENT FUNDS");
        assert_eq!(field.queries()[2], "18 ACCOUNT CLOSED");
    }

    #[test]
    fn test_field75_predefined_codes() {
        let queries = vec![
            "3 STOP PAYMENT REQUEST".to_string(),
            "18 ACCOUNT CLOSED".to_string(),
            "NARRATIVE TEXT".to_string(),
        ];
        let field = Field75::new(queries).unwrap();
        assert!(field.has_predefined_codes());
        let codes = field.predefined_codes();
        assert_eq!(codes, vec!["18", "3"]);
    }

    #[test]
    fn test_field75_query_type_detection() {
        let field = Field75::new(vec!["3 STOP PAYMENT REQUEST".to_string()]).unwrap();
        assert!(field.is_stop_payment_query());
        assert!(!field.is_account_status_query());
        assert_eq!(field.query_type(), "Stop Payment");

        let field = Field75::new(vec!["18 ACCOUNT STATUS QUERY".to_string()]).unwrap();
        assert!(field.is_account_status_query());
        assert!(!field.is_stop_payment_query());
        assert_eq!(field.query_type(), "Account Status");

        let field = Field75::new(vec!["20 BENEFICIARY VERIFICATION".to_string()]).unwrap();
        assert!(field.is_beneficiary_verification_query());
        assert_eq!(field.query_type(), "Beneficiary Verification");
    }

    #[test]
    fn test_field75_priority_level() {
        let stop_payment = Field75::new(vec!["3 STOP PAYMENT".to_string()]).unwrap();
        assert_eq!(stop_payment.priority_level(), 5);

        let regulatory = Field75::new(vec!["21 COMPLIANCE CHECK".to_string()]).unwrap();
        assert_eq!(regulatory.priority_level(), 4);

        let account_status = Field75::new(vec!["18 ACCOUNT STATUS".to_string()]).unwrap();
        assert_eq!(account_status.priority_level(), 2);

        let general = Field75::new(vec!["GENERAL QUERY".to_string()]).unwrap();
        assert_eq!(general.priority_level(), 1);
    }

    #[test]
    fn test_field75_urgency_detection() {
        let urgent = Field75::new(vec!["3 URGENT STOP PAYMENT".to_string()]).unwrap();
        assert!(urgent.is_urgent());

        let regulatory = Field75::new(vec!["21 COMPLIANCE CHECK".to_string()]).unwrap();
        assert!(regulatory.is_urgent());

        let normal = Field75::new(vec!["GENERAL INQUIRY".to_string()]).unwrap();
        assert!(!normal.is_urgent());

        let urgent_text = Field75::new(vec!["IMMEDIATE ATTENTION REQUIRED".to_string()]).unwrap();
        assert!(urgent_text.is_urgent());
    }

    #[test]
    fn test_field75_validation() {
        // Test too many lines
        let too_many = vec!["1".to_string(); 7];
        assert!(Field75::new(too_many).is_err());

        // Test line too long
        let too_long = vec!["A".repeat(36)];
        assert!(Field75::new(too_long).is_err());

        // Test empty
        assert!(Field75::new(vec![]).is_err());
    }

    #[test]
    fn test_field75_parse() {
        let field = Field75::parse("3 STOP PAYMENT\nINSUFFICIENT FUNDS").unwrap();
        assert_eq!(field.line_count(), 2);
        assert!(field.is_stop_payment_query());
    }

    #[test]
    fn test_field75_to_swift_string() {
        let field = Field75::new(vec![
            "3 STOP PAYMENT REQUEST".to_string(),
            "ACCOUNT HOLDER DECEASED".to_string(),
        ])
        .unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":75:3 STOP PAYMENT REQUEST\nACCOUNT HOLDER DECEASED"
        );
    }

    #[test]
    fn test_field75_display() {
        let field = Field75::new(vec!["3 STOP PAYMENT".to_string(), "URGENT".to_string()]).unwrap();
        assert_eq!(format!("{}", field), "3 STOP PAYMENT\nURGENT");
    }

    #[test]
    fn test_field75_comprehensive_description() {
        let field = Field75::new(vec![
            "3 URGENT STOP PAYMENT".to_string(),
            "CHEQUE STOLEN".to_string(),
        ])
        .unwrap();
        let desc = field.comprehensive_description();
        assert!(desc.contains("Stop Payment Query"));
        assert!(desc.contains("2 lines"));
        assert!(desc.contains("[URGENT]"));
    }

    #[test]
    fn test_field75_multiple_codes() {
        let field = Field75::new(vec![
            "3 STOP PAYMENT".to_string(),
            "18 ACCOUNT CLOSED".to_string(),
            "20 VERIFY BENEFICIARY".to_string(),
        ])
        .unwrap();
        let query_type = field.query_type();
        // The order may vary due to sorting, so check that all types are present
        assert!(query_type.contains("Stop Payment"));
        assert!(query_type.contains("Account Status"));
        assert!(query_type.contains("Beneficiary Verification"));
        assert_eq!(field.priority_level(), 5); // Highest priority from stop payment
    }

    #[test]
    fn test_field75_code_boundary_detection() {
        // Test that "3X" is not detected as code "3"
        let field = Field75::new(vec!["3X NOT A CODE".to_string()]).unwrap();
        assert!(!field.has_predefined_codes());
        assert!(!field.is_stop_payment_query());

        // Test that "3 " is detected as code "3"
        let field = Field75::new(vec!["3 VALID CODE".to_string()]).unwrap();
        assert!(field.has_predefined_codes());
        assert!(field.is_stop_payment_query());

        // Test that "3" alone is detected as code "3"
        let field = Field75::new(vec!["3".to_string()]).unwrap();
        assert!(field.has_predefined_codes());
        assert!(field.is_stop_payment_query());
    }
}
