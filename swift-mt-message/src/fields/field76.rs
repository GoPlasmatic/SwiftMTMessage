use crate::fields::MultiLineField;
use crate::{SwiftField, ValidationResult, errors::ParseError};
use serde::{Deserialize, Serialize};

/// # Field 76: Answers/Status Information
///
/// ## Overview
/// Field 76 contains status information and answers for stop payment requests and other
/// query responses in SWIFT MT messages. This field provides detailed status updates,
/// processing results, and response information with support for predefined status codes
/// and free-form narrative text to communicate processing outcomes and additional details.
///
/// ## Format Specification
/// **Format**: `6*35x`
/// - **6*35x**: Up to 6 lines of 35 characters each
/// - **Character set**: SWIFT character set (A-Z, 0-9, and limited special characters)
/// - **Line structure**: Each line can contain predefined codes or narrative text
/// - **Status indicators**: ACCEPTED, REJECTED, PENDING, PROCESSED, etc.
/// - **Validation**: Each line must not exceed 35 characters
///
/// ## Structure
/// ```text
/// STOP PAYMENT ACCEPTED
/// PROCESSED SUCCESSFULLY
/// REFERENCE: SP20241201001
/// EFFECTIVE IMMEDIATELY
/// ```
///
/// ## Status Categories
/// - **Success indicators**: ACCEPTED, PROCESSED, COMPLETED, SUCCESS
/// - **Rejection indicators**: REJECTED, DENIED, FAILED, ERROR, INVALID
/// - **Pending indicators**: PENDING, PROCESSING, UNDER REVIEW, QUEUED
/// - **Information codes**: Various predefined response codes
///
/// ## Usage Context
/// Field 76 is used in:
/// - **MT112**: Status of Request for Stop Payment of a Cheque
/// - **MT196**: Answers (Customer or FI)
/// - **MT199**: Free Format Message (for answers)
/// - **MT299**: Free Format Message (for status updates)
///
/// ### Business Applications
/// - **Stop payment responses**: Confirming or rejecting stop payment requests
/// - **Query responses**: Providing answers to customer or institutional queries
/// - **Status updates**: Communicating processing status and outcomes
/// - **Error notifications**: Reporting processing failures and reasons
/// - **Compliance responses**: Regulatory and compliance-related answers
/// - **Documentation provision**: Supplying requested information or documents
///
/// ## Examples
/// ```text
/// :76:STOP PAYMENT ACCEPTED
/// PROCESSED ON 20241201
/// CHEQUE NUMBER 123456
/// EFFECTIVE IMMEDIATELY
///
/// :76:REQUEST REJECTED
/// INSUFFICIENT INFORMATION
/// PLEASE PROVIDE CHEQUE DETAILS
/// CONTACT CUSTOMER SERVICE
///
/// :76:PROCESSING PENDING
/// UNDER COMPLIANCE REVIEW
/// RESPONSE WITHIN 2 BUSINESS DAYS
/// REFERENCE: REV20241201001
/// ```
///
/// ## Validation Rules
/// 1. **Line count**: Maximum 6 lines
/// 2. **Line length**: Maximum 35 characters per line
/// 3. **Content**: Cannot be empty
/// 4. **Character validation**: SWIFT character set compliance
/// 5. **Status clarity**: Must provide clear status indication
/// 6. **Reference information**: Should include relevant references when applicable
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Maximum 6 lines allowed (Error: T26)
/// - Each line cannot exceed 35 characters (Error: T50)
/// - Must use SWIFT character set only (Error: T61)
/// - Cannot be empty field (Error: T13)
/// - Status must be clearly indicated (Error: T51)
/// - Content must be meaningful and relevant (Error: T52)
/// - Must relate to original query or request (Error: T53)
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field76 {
    /// Status information lines (up to 6 lines of 35 characters each)
    pub status_info: Vec<String>,
}

impl MultiLineField for Field76 {
    const MAX_LINES: usize = 6;
    const FIELD_TAG: &'static str = "76";

    fn lines(&self) -> &[String] {
        &self.status_info
    }

    fn lines_mut(&mut self) -> &mut Vec<String> {
        &mut self.status_info
    }

    fn new_with_lines(lines: Vec<String>) -> Result<Self, ParseError> {
        Ok(Field76 { status_info: lines })
    }
}

impl Field76 {
    /// Create a new Field76 with validation
    pub fn new(status_info: Vec<String>) -> Result<Self, ParseError> {
        <Self as MultiLineField>::new(status_info)
    }

    /// Create from a single string, splitting on newlines
    pub fn from_string(content: impl Into<String>) -> Result<Self, ParseError> {
        let content = content.into();
        let lines: Vec<String> = content.lines().map(|s| s.trim().to_string()).collect();
        Self::new(lines)
    }

    /// Get the status information lines
    pub fn status_info(&self) -> &[String] {
        &self.status_info
    }

    /// Check if status indicates successful processing
    pub fn is_successful(&self) -> bool {
        let success_indicators = ["ACCEPTED", "PROCESSED", "COMPLETED", "SUCCESS", "APPROVED"];
        self.status_info.iter().any(|line| {
            let line_upper = line.to_uppercase();
            success_indicators
                .iter()
                .any(|indicator| line_upper.contains(indicator))
        })
    }

    /// Check if status indicates rejection or failure
    pub fn is_rejected(&self) -> bool {
        let rejection_indicators = [
            "REJECTED", "DENIED", "FAILED", "ERROR", "INVALID", "DECLINED",
        ];
        self.status_info.iter().any(|line| {
            let line_upper = line.to_uppercase();
            rejection_indicators
                .iter()
                .any(|indicator| line_upper.contains(indicator))
        })
    }

    /// Check if status indicates pending processing
    pub fn is_pending(&self) -> bool {
        let pending_indicators = [
            "PENDING",
            "PROCESSING",
            "UNDER REVIEW",
            "QUEUED",
            "IN PROGRESS",
        ];
        self.status_info.iter().any(|line| {
            let line_upper = line.to_uppercase();
            pending_indicators
                .iter()
                .any(|indicator| line_upper.contains(indicator))
        })
    }

    /// Get the overall status as a string
    pub fn status(&self) -> &str {
        if self.is_successful() {
            "Successful"
        } else if self.is_rejected() {
            "Rejected"
        } else if self.is_pending() {
            "Pending"
        } else {
            "Unknown"
        }
    }

    /// Get comprehensive description including status and details
    pub fn comprehensive_description(&self) -> String {
        format!("Status: {} ({} lines)", self.status(), self.line_count())
    }
}

impl SwiftField for Field76 {
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

impl std::fmt::Display for Field76 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status_info.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field76_creation() {
        let status_info = vec![
            "STOP PAYMENT ACCEPTED".to_string(),
            "PROCESSED SUCCESSFULLY".to_string(),
        ];
        let field = Field76::new(status_info.clone()).unwrap();
        assert_eq!(field.status_info(), &status_info);
        assert_eq!(field.line_count(), 2);
    }

    #[test]
    fn test_field76_status_detection() {
        // Test successful status
        let success_field = Field76::new(vec!["STOP PAYMENT ACCEPTED".to_string()]).unwrap();
        assert!(success_field.is_successful());
        assert!(!success_field.is_rejected());
        assert!(!success_field.is_pending());
        assert_eq!(success_field.status(), "Successful");

        // Test rejected status
        let reject_field = Field76::new(vec!["REQUEST REJECTED".to_string()]).unwrap();
        assert!(!reject_field.is_successful());
        assert!(reject_field.is_rejected());
        assert!(!reject_field.is_pending());
        assert_eq!(reject_field.status(), "Rejected");

        // Test pending status
        let pending_field = Field76::new(vec!["PROCESSING PENDING".to_string()]).unwrap();
        assert!(!pending_field.is_successful());
        assert!(!pending_field.is_rejected());
        assert!(pending_field.is_pending());
        assert_eq!(pending_field.status(), "Pending");
    }

    #[test]
    fn test_field76_validation() {
        // Test too many lines
        let too_many = vec!["1".to_string(); 7];
        assert!(Field76::new(too_many).is_err());

        // Test line too long
        let too_long = vec!["A".repeat(36)];
        assert!(Field76::new(too_long).is_err());

        // Test empty
        assert!(Field76::new(vec![]).is_err());
    }

    #[test]
    fn test_field76_parse() {
        let field = Field76::parse("STOP PAYMENT ACCEPTED\nPROCESSED SUCCESSFULLY").unwrap();
        assert_eq!(field.line_count(), 2);
        assert!(field.is_successful());
    }

    #[test]
    fn test_field76_to_swift_string() {
        let field = Field76::new(vec![
            "STOP PAYMENT ACCEPTED".to_string(),
            "PROCESSED ON 20241201".to_string(),
        ])
        .unwrap();
        assert_eq!(
            field.to_swift_string(),
            ":76:STOP PAYMENT ACCEPTED\nPROCESSED ON 20241201"
        );
    }

    #[test]
    fn test_field76_display() {
        let field = Field76::new(vec![
            "STOP PAYMENT ACCEPTED".to_string(),
            "EFFECTIVE IMMEDIATELY".to_string(),
        ])
        .unwrap();
        assert_eq!(
            format!("{}", field),
            "STOP PAYMENT ACCEPTED\nEFFECTIVE IMMEDIATELY"
        );
    }
}
