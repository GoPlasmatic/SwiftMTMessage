use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 77B: Regulatory Reporting
///
/// ## Overview
/// Field 77B contains regulatory reporting information in SWIFT payment messages, providing
/// data required by regulatory authorities for compliance, monitoring, and statistical purposes.
/// This field supports various regulatory requirements including anti-money laundering (AML),
/// know your customer (KYC), foreign exchange reporting, and other jurisdiction-specific
/// compliance obligations. The information helps authorities track cross-border payments
/// and ensure compliance with local and international regulations.
///
/// ## Format Specification
/// **Format**: `3*35x`
/// - **3*35x**: Up to 3 lines of 35 characters each
/// - **Line structure**: Structured regulatory codes and information
/// - **Character set**: SWIFT character set (A-Z, 0-9, and limited special characters)
/// - **Line separation**: Each line on separate row
///
/// ## Structure
/// ```text
/// /ORDERRES/US/1234567890123456
/// /BENEFRES/DE/9876543210987654
/// /PURP/TRADE
/// │                              │
/// └──────────────────────────────┘
///        Up to 35 characters per line
///        Maximum 3 lines
/// ```
///
/// ## Field Components
/// - **Ordering Country**: Country code of ordering customer
/// - **Beneficiary Country**: Country code of beneficiary customer
/// - **Purpose Code**: Transaction purpose or category
/// - **Regulatory Codes**: Authority-specific reporting codes
/// - **Additional Information**: Supplementary compliance data
///
/// ## Usage Context
/// Field 77B is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Regulatory compliance**: Meeting reporting requirements
/// - **AML/KYC reporting**: Anti-money laundering compliance
/// - **Foreign exchange reporting**: FX transaction monitoring
/// - **Statistical reporting**: Economic and trade statistics
/// - **Sanctions screening**: Compliance with sanctions regimes
/// - **Tax reporting**: Supporting tax authority requirements
///
/// ## Common Regulatory Codes
/// ### /ORDERRES/ - Ordering Customer Residence
/// - **Format**: `/ORDERRES/CC/identifier`
/// - **CC**: ISO 3166-1 two-letter country code
/// - **identifier**: Customer identification number
/// - **Purpose**: Identifies ordering customer's country of residence
///
/// ### /BENEFRES/ - Beneficiary Residence
/// - **Format**: `/BENEFRES/CC/identifier`
/// - **CC**: ISO 3166-1 two-letter country code
/// - **identifier**: Beneficiary identification number
/// - **Purpose**: Identifies beneficiary's country of residence
///
/// ### /PURP/ - Purpose Code
/// - **Format**: `/PURP/code`
/// - **code**: Transaction purpose code
/// - **Examples**: TRADE, SALA, PENS, DIVI, LOAN
/// - **Purpose**: Categorizes transaction purpose
///
/// ## Examples
/// ```text
/// :77B:/ORDERRES/US/1234567890
/// └─── US ordering customer with ID
///
/// :77B:/ORDERRES/DE/9876543210
/// /BENEFRES/GB/5555666677
/// /PURP/TRADE
/// └─── Complete regulatory reporting with purpose
///
/// :77B:/BENEFRES/JP/1111222233
/// /PURP/SALA
/// └─── Japanese beneficiary for salary payment
///
/// :77B:/ORDERRES/CH/7777888899
/// /BENEFRES/FR/4444555566
/// └─── Cross-border payment reporting
/// ```
///
/// ## Purpose Codes
/// - **TRADE**: Trade-related payments
/// - **SALA**: Salary and wage payments
/// - **PENS**: Pension payments
/// - **DIVI**: Dividend payments
/// - **LOAN**: Loan-related payments
/// - **RENT**: Rental payments
/// - **ROYALTY**: Royalty payments
/// - **FEES**: Professional fees
/// - **INSUR**: Insurance payments
/// - **INVEST**: Investment-related payments
///
/// ## Country Code Guidelines
/// - **ISO 3166-1**: Must use standard two-letter country codes
/// - **Active codes**: Should use currently valid country codes
/// - **Residence**: Based on customer's country of residence
/// - **Jurisdiction**: May differ from bank location
/// - **Compliance**: Must align with regulatory requirements
///
/// ## Validation Rules
/// 1. **Line count**: Maximum 3 lines
/// 2. **Line length**: Maximum 35 characters per line
/// 3. **Character set**: SWIFT character set only
/// 4. **Country codes**: Must be valid ISO 3166-1 codes
/// 5. **Format structure**: Must follow structured format
/// 6. **Content validation**: Codes must be meaningful
/// 7. **Regulatory compliance**: Must meet jurisdiction requirements
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Maximum 3 lines allowed (Error: T26)
/// - Each line maximum 35 characters (Error: T50)
/// - Must use SWIFT character set only (Error: T61)
/// - Country codes must be valid (Error: T52)
/// - Format must follow regulatory structure (Error: T77)
/// - Purpose codes should be recognized (Warning: W77)
/// - Field required for certain jurisdictions (Error: M77)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Field77B {
    /// Regulatory reporting information lines (up to 3 lines of 35 characters each)
    pub information: Vec<String>,
    /// Ordering country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering_country: Option<String>,
    /// Beneficiary country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_country: Option<String>,
}

impl Field77B {
    /// Create a new Field77B with validation
    pub fn new(information: Vec<String>) -> Result<Self, crate::ParseError> {
        if information.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77B".to_string(),
                message: "Regulatory reporting information cannot be empty".to_string(),
            });
        }

        if information.len() > 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77B".to_string(),
                message: "Too many lines (max 3)".to_string(),
            });
        }

        for (i, line) in information.iter().enumerate() {
            if line.len() > 35 {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "77B".to_string(),
                    message: format!("Line {} too long (max 35 characters)", i + 1),
                });
            }

            // Validate characters (printable ASCII)
            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return Err(crate::ParseError::InvalidFieldFormat {
                    field_tag: "77B".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        // Extract country codes from the information lines
        let mut ordering_country = None;
        let mut beneficiary_country = None;

        for line in &information {
            if line.starts_with("/ORDERRES/") {
                // Extract country code after /ORDERRES/
                if let Some(country_part) = line.strip_prefix("/ORDERRES/") {
                    // Take the first part before any additional slashes or content
                    let country = country_part.split('/').next().unwrap_or("").to_string();
                    if !country.is_empty() {
                        ordering_country = Some(country);
                    }
                }
            }
            if line.starts_with("/BENEFRES/") {
                // Extract country code after /BENEFRES/
                if let Some(country_part) = line.strip_prefix("/BENEFRES/") {
                    // Take the first part before any additional slashes or content
                    let country = country_part.split('/').next().unwrap_or("").to_string();
                    if !country.is_empty() {
                        beneficiary_country = Some(country);
                    }
                }
            }
        }

        Ok(Field77B {
            information,
            ordering_country,
            beneficiary_country,
        })
    }

    /// Create from a single string, splitting on newlines
    pub fn from_string(content: impl Into<String>) -> Result<Self, crate::ParseError> {
        let content = content.into();
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self::new(lines)
    }

    /// Get the information lines
    pub fn information(&self) -> &[String] {
        &self.information
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.information.len()
    }

    /// Get a specific line by index
    pub fn line(&self, index: usize) -> Option<&str> {
        self.information.get(index).map(|s| s.as_str())
    }

    /// Add a line of information
    pub fn add_line(&mut self, line: String) -> Result<(), crate::ParseError> {
        if self.information.len() >= 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77B".to_string(),
                message: "Cannot add more lines (max 3)".to_string(),
            });
        }

        if line.len() > 35 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77B".to_string(),
                message: "Line too long (max 35 characters)".to_string(),
            });
        }

        if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77B".to_string(),
                message: "Line contains invalid characters".to_string(),
            });
        }

        self.information.push(line);
        Ok(())
    }

    /// Check if this contains ordering country information
    pub fn has_ordering_country(&self) -> bool {
        self.ordering_country.is_some()
    }

    /// Check if this contains beneficiary country information
    pub fn has_beneficiary_country(&self) -> bool {
        self.beneficiary_country.is_some()
    }

    /// Extract ordering country code if present
    pub fn ordering_country(&self) -> Option<&str> {
        self.ordering_country.as_deref()
    }

    /// Extract beneficiary country code if present
    pub fn beneficiary_country(&self) -> Option<&str> {
        self.beneficiary_country.as_deref()
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!("Regulatory Reporting ({} lines)", self.line_count())
    }
}

impl SwiftField for Field77B {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":77B:") {
            stripped // Remove ":77B:" prefix
        } else if let Some(stripped) = value.strip_prefix("77B:") {
            stripped // Remove "77B:" prefix
        } else {
            value
        };

        let content = content.trim();

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "77B".to_string(),
                message: "Field content cannot be empty".to_string(),
            });
        }

        Self::from_string(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":77B:{}", self.information.join("\n"))
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate line count
        if self.information.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "77B".to_string(),
                message: "Information cannot be empty".to_string(),
            });
        }

        if self.information.len() > 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "77B".to_string(),
                expected: "max 3 lines".to_string(),
                actual: self.information.len(),
            });
        }

        // Validate each line
        for (i, line) in self.information.iter().enumerate() {
            if line.len() > 35 {
                errors.push(ValidationError::LengthValidation {
                    field_tag: "77B".to_string(),
                    expected: format!("max 35 characters for line {}", i + 1),
                    actual: line.len(),
                });
            }

            if !line.chars().all(|c| c.is_ascii() && !c.is_control()) {
                errors.push(ValidationError::FormatValidation {
                    field_tag: "77B".to_string(),
                    message: format!("Line {} contains invalid characters", i + 1),
                });
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "3*35x"
    }
}

impl std::fmt::Display for Field77B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.information.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field77b_creation() {
        let lines = vec!["/ORDERRES/DE".to_string(), "/BENEFRES/BE".to_string()];
        let field = Field77B::new(lines.clone()).unwrap();
        assert_eq!(field.information(), &lines);
        assert_eq!(field.line_count(), 2);
    }

    #[test]
    fn test_field77b_from_string() {
        let content = "/ORDERRES/DE\n/BENEFRES/BE\nREGPORT123";
        let field = Field77B::from_string(content).unwrap();
        assert_eq!(field.line_count(), 3);
        assert_eq!(field.line(0), Some("/ORDERRES/DE"));
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
        assert_eq!(field.line(2), Some("REGPORT123"));
    }

    #[test]
    fn test_field77b_parse() {
        let field = Field77B::parse("/ORDERRES/DE\n/BENEFRES/BE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("/ORDERRES/DE"));
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
    }

    #[test]
    fn test_field77b_parse_with_prefix() {
        let field = Field77B::parse(":77B:/ORDERRES/DE\n/BENEFRES/BE").unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(0), Some("/ORDERRES/DE"));
    }

    #[test]
    fn test_field77b_to_swift_string() {
        let lines = vec!["/ORDERRES/DE".to_string(), "/BENEFRES/BE".to_string()];
        let field = Field77B::new(lines).unwrap();
        assert_eq!(field.to_swift_string(), ":77B:/ORDERRES/DE\n/BENEFRES/BE");
    }

    #[test]
    fn test_field77b_add_line() {
        let mut field = Field77B::new(vec!["/ORDERRES/DE".to_string()]).unwrap();
        field.add_line("/BENEFRES/BE".to_string()).unwrap();
        assert_eq!(field.line_count(), 2);
        assert_eq!(field.line(1), Some("/BENEFRES/BE"));
    }

    #[test]
    fn test_field77b_country_extraction() {
        let field =
            Field77B::new(vec!["/ORDERRES/DE".to_string(), "/BENEFRES/BE".to_string()]).unwrap();

        assert!(field.has_ordering_country());
        assert!(field.has_beneficiary_country());
        assert_eq!(field.ordering_country(), Some("DE"));
        assert_eq!(field.beneficiary_country(), Some("BE"));
    }

    #[test]
    fn test_field77b_country_extraction_with_additional_info() {
        // Test the format from the backup version: "/ORDERRES/DE//REGULATORY INFO"
        let field = Field77B::new(vec![
            "/ORDERRES/DE//REGULATORY INFO".to_string(),
            "SOFTWARE LICENSE COMPLIANCE".to_string(),
            "TRADE RELATED TRANSACTION".to_string(),
        ])
        .unwrap();

        assert!(field.has_ordering_country());
        assert!(!field.has_beneficiary_country());
        assert_eq!(field.ordering_country(), Some("DE"));
        assert_eq!(field.beneficiary_country(), None);
    }

    #[test]
    fn test_field77b_no_country_codes() {
        let field = Field77B::new(vec![
            "REGULATORY INFORMATION".to_string(),
            "NO COUNTRY CODES HERE".to_string(),
        ])
        .unwrap();

        assert!(!field.has_ordering_country());
        assert!(!field.has_beneficiary_country());
        assert_eq!(field.ordering_country(), None);
        assert_eq!(field.beneficiary_country(), None);
    }

    #[test]
    fn test_field77b_too_many_lines() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(), // Too many
        ];
        let result = Field77B::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field77b_line_too_long() {
        let lines = vec!["A".repeat(36)]; // 36 characters, max is 35
        let result = Field77B::new(lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_field77b_empty() {
        let result = Field77B::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_field77b_validation() {
        let field = Field77B::new(vec!["/ORDERRES/DE".to_string()]).unwrap();
        let validation = field.validate();
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_field77b_display() {
        let field =
            Field77B::new(vec!["/ORDERRES/DE".to_string(), "/BENEFRES/BE".to_string()]).unwrap();
        assert_eq!(format!("{}", field), "/ORDERRES/DE\n/BENEFRES/BE");
    }

    #[test]
    fn test_field77b_description() {
        let field =
            Field77B::new(vec!["/ORDERRES/DE".to_string(), "/BENEFRES/BE".to_string()]).unwrap();
        assert_eq!(field.description(), "Regulatory Reporting (2 lines)");
    }

    #[test]
    fn test_field77b_add_line_max_reached() {
        let mut field = Field77B::new(vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
        ])
        .unwrap();

        let result = field.add_line("Line 4".to_string());
        assert!(result.is_err());
    }
}
