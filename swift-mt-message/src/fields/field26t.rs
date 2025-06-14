use crate::{SwiftField, ValidationError, ValidationResult};
use serde::{Deserialize, Serialize};

/// # Field 26T: Transaction Type Code
///
/// ## Overview
/// Field 26T contains a transaction type code that categorizes the nature of the transaction
/// according to EUROSTAT Balance of Payments (BoP) guidelines. This field is mandatory for
/// statistical reporting purposes and helps financial institutions classify cross-border
/// transactions for regulatory compliance and economic analysis.
///
/// ## Format Specification
/// **Format**: `3!c`
/// - **3!c**: Exactly 3 alphanumeric characters
/// - **Character set**: A-Z, 0-9 (uppercase letters and digits)
/// - **Case handling**: Automatically converted to uppercase
/// - **Validation**: Must be exactly 3 characters, alphanumeric only
///
/// ## EUROSTAT Balance of Payments Categories
/// The transaction type codes follow EUROSTAT BoP methodology for statistical classification:
///
/// ### Goods (A-series)
/// - **A01**: General merchandise on a gross basis - Standard trade in goods
/// - **A02**: Goods for processing - Goods sent for processing abroad
/// - **A03**: Repairs on goods - Repair services on movable goods
///
/// ### Services (B-series)
/// - **B01**: Manufacturing services on physical inputs owned by others
/// - **B02**: Maintenance and repair services n.i.e. (not included elsewhere)
/// - **B03**: Transport services - Passenger and freight transport
///
/// ### Primary Income (C-series)
/// - **C01**: Compensation of employees - Wages, salaries, and benefits
/// - **C02**: Investment income - Dividends, interest, and other investment returns
///
/// ### Secondary Income (D-series)
/// - **D01**: General government transfers - Government-to-government transfers
/// - **D02**: Other sectors transfers - Private transfers and remittances
///
/// ### Capital Account (E-series)
/// - **E01**: Capital transfers - Non-financial asset transfers
/// - **E02**: Acquisition/disposal of non-produced, non-financial assets
///
/// ### Financial Account (F-series)
/// - **F01**: Direct investment - Foreign direct investment flows
/// - **F02**: Portfolio investment - Securities and equity investments
/// - **F03**: Financial derivatives - Derivative instruments
/// - **F04**: Other investment - Loans, deposits, and other financial instruments
/// - **F05**: Reserve assets - Central bank reserves
///
/// ## Usage Context
/// Field 26T is used in various SWIFT MT message types for statistical reporting:
/// - **MT103**: Single Customer Credit Transfer (when required by regulation)
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its Own Account
/// - **Cross-border payments**: Mandatory for EU and many other jurisdictions
///
/// ### Business Applications
/// - **Statistical reporting**: Balance of payments statistics compilation
/// - **Regulatory compliance**: Meeting central bank reporting requirements
/// - **Economic analysis**: Supporting macroeconomic policy decisions
/// - **Risk management**: Transaction categorization for risk assessment
/// - **Audit trails**: Enhanced transaction tracking and classification
/// - **Automated processing**: STP routing based on transaction type
///
/// ## Regulatory Framework
/// Field 26T supports compliance with various regulatory requirements:
///
/// ### European Union
/// - **ECB Regulation**: European Central Bank balance of payments reporting
/// - **EUROSTAT methodology**: Statistical classification standards
/// - **National implementations**: Country-specific BoP reporting requirements
///
/// ### International Standards
/// - **IMF BPM6**: International Monetary Fund Balance of Payments Manual
/// - **OECD guidelines**: Organisation for Economic Co-operation and Development
/// - **BIS reporting**: Bank for International Settlements requirements
///
/// ## Validation Rules
/// 1. **Length**: Must be exactly 3 characters
/// 2. **Character set**: Only alphanumeric characters (A-Z, 0-9)
/// 3. **Case**: Automatically normalized to uppercase
/// 4. **Format compliance**: Must follow EUROSTAT BoP code structure
/// 5. **Non-empty**: Cannot be empty or whitespace only
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Transaction type code must be exactly 3 characters (Error: T26)
/// - Must contain only alphanumeric characters (Error: T61)
/// - Should follow recognized BoP classification (Warning: recommended practice)
/// - Must be consistent with transaction nature (Error: T40)
///
///
/// ## Examples
/// ```text
/// :26T:A01
/// └─── General merchandise trade
///
/// :26T:C02
/// └─── Investment income (dividends, interest)
///
/// :26T:F01
/// └─── Foreign direct investment
///
/// :26T:D02
/// └─── Private transfers/remittances
/// ```
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field26T {
    /// Transaction type code (exactly 3 alphanumeric characters)
    ///
    /// Specifies the transaction type according to EUROSTAT Balance of Payments
    /// guidelines for statistical reporting and regulatory compliance.
    ///
    /// **Format**: Exactly 3 uppercase alphanumeric characters
    /// **Character set**: A-Z, 0-9 only
    /// **Case handling**: Automatically converted to uppercase
    ///
    /// # Standard Categories
    /// - **A-series**: Goods (A01, A02, A03)
    /// - **B-series**: Services (B01, B02, B03)
    /// - **C-series**: Primary Income (C01, C02)
    /// - **D-series**: Secondary Income (D01, D02)
    /// - **E-series**: Capital Account (E01, E02)
    /// - **F-series**: Financial Account (F01-F05)
    ///
    /// # Examples
    /// - `"A01"` - General merchandise on a gross basis
    /// - `"C02"` - Investment income
    /// - `"F01"` - Direct investment
    /// - `"D02"` - Other sectors transfers
    pub transaction_type_code: String,
}

impl SwiftField for Field26T {
    fn parse(value: &str) -> Result<Self, crate::ParseError> {
        let content = if let Some(stripped) = value.strip_prefix(":26T:") {
            stripped // Remove ":26T:" prefix
        } else if let Some(stripped) = value.strip_prefix("26T:") {
            stripped // Remove "26T:" prefix
        } else {
            value
        };

        if content.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Field content cannot be empty after removing tag".to_string(),
            });
        }

        Self::new(content)
    }

    fn to_swift_string(&self) -> String {
        format!(":26T:{}", self.transaction_type_code)
    }

    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();

        // Validate length (3 characters)
        if self.transaction_type_code.len() != 3 {
            errors.push(ValidationError::LengthValidation {
                field_tag: "26T".to_string(),
                expected: "3 characters".to_string(),
                actual: self.transaction_type_code.len(),
            });
        }

        // Validate not empty
        if self.transaction_type_code.is_empty() {
            errors.push(ValidationError::ValueValidation {
                field_tag: "26T".to_string(),
                message: "Transaction type code cannot be empty".to_string(),
            });
        }

        // Validate characters (alphanumeric)
        if !self
            .transaction_type_code
            .chars()
            .all(|c| c.is_alphanumeric() && c.is_ascii())
        {
            errors.push(ValidationError::FormatValidation {
                field_tag: "26T".to_string(),
                message: "Transaction type code must contain only alphanumeric characters"
                    .to_string(),
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }

    fn format_spec() -> &'static str {
        "3!c"
    }
}

impl Field26T {
    /// Create a new Field26T with validation
    pub fn new(transaction_type_code: impl Into<String>) -> crate::Result<Self> {
        let code = transaction_type_code.into().trim().to_uppercase();

        if code.is_empty() {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Transaction type code cannot be empty".to_string(),
            });
        }

        if code.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Transaction type code must be exactly 3 characters".to_string(),
            });
        }

        // Validate characters (alphanumeric)
        if !code.chars().all(|c| c.is_alphanumeric() && c.is_ascii()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "26T".to_string(),
                message: "Transaction type code must contain only alphanumeric characters"
                    .to_string(),
            });
        }

        Ok(Field26T {
            transaction_type_code: code,
        })
    }

    /// Get the transaction type code
    ///
    /// Returns the 3-character transaction type code that specifies
    /// the nature of the transaction for BoP classification.
    ///
    /// # Returns
    /// A string slice containing the transaction type code in uppercase
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let field = Field26T::new("A01").unwrap();
    /// assert_eq!(field.code(), "A01");
    /// ```
    pub fn code(&self) -> &str {
        &self.transaction_type_code
    }

    /// Check if this is a valid EUROSTAT BoP code format
    ///
    /// Validates that the code follows the basic format requirements
    /// for EUROSTAT Balance of Payments classification.
    ///
    /// # Returns
    /// `true` if the code format is valid
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let field = Field26T::new("A01").unwrap();
    /// assert!(field.is_valid_format());
    /// ```
    pub fn is_valid_format(&self) -> bool {
        self.transaction_type_code.len() == 3
            && self
                .transaction_type_code
                .chars()
                .all(|c| c.is_alphanumeric() && c.is_ascii())
    }

    /// Get the BoP category for this transaction type
    ///
    /// Returns the main Balance of Payments category that this
    /// transaction type belongs to.
    ///
    /// # Returns
    /// BoP category as a string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let field = Field26T::new("A01").unwrap();
    /// assert_eq!(field.bop_category(), "Goods");
    /// ```
    pub fn bop_category(&self) -> &'static str {
        match self.transaction_type_code.chars().next() {
            Some('A') => "Goods",
            Some('B') => "Services",
            Some('C') => "Primary Income",
            Some('D') => "Secondary Income",
            Some('E') => "Capital Account",
            Some('F') => "Financial Account",
            _ => "Other",
        }
    }

    /// Check if this is a goods transaction
    ///
    /// Determines if the transaction type represents trade in goods
    /// according to BoP classification.
    ///
    /// # Returns
    /// `true` if this is a goods transaction (A-series)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let goods = Field26T::new("A01").unwrap();
    /// assert!(goods.is_goods_transaction());
    ///
    /// let service = Field26T::new("B01").unwrap();
    /// assert!(!service.is_goods_transaction());
    /// ```
    pub fn is_goods_transaction(&self) -> bool {
        self.transaction_type_code.starts_with('A')
    }

    /// Check if this is a services transaction
    ///
    /// Determines if the transaction type represents trade in services
    /// according to BoP classification.
    ///
    /// # Returns
    /// `true` if this is a services transaction (B-series)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let service = Field26T::new("B03").unwrap();
    /// assert!(service.is_services_transaction());
    /// ```
    pub fn is_services_transaction(&self) -> bool {
        self.transaction_type_code.starts_with('B')
    }

    /// Check if this is an income transaction
    ///
    /// Determines if the transaction type represents income flows
    /// (primary or secondary income) according to BoP classification.
    ///
    /// # Returns
    /// `true` if this is an income transaction (C or D series)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let income = Field26T::new("C02").unwrap();
    /// assert!(income.is_income_transaction());
    ///
    /// let transfer = Field26T::new("D02").unwrap();
    /// assert!(transfer.is_income_transaction());
    /// ```
    pub fn is_income_transaction(&self) -> bool {
        self.transaction_type_code.starts_with('C') || self.transaction_type_code.starts_with('D')
    }

    /// Check if this is a financial transaction
    ///
    /// Determines if the transaction type represents financial flows
    /// according to BoP classification.
    ///
    /// # Returns
    /// `true` if this is a financial transaction (F-series)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let financial = Field26T::new("F01").unwrap();
    /// assert!(financial.is_financial_transaction());
    /// ```
    pub fn is_financial_transaction(&self) -> bool {
        self.transaction_type_code.starts_with('F')
    }

    /// Check if this is a capital account transaction
    ///
    /// Determines if the transaction type represents capital account flows
    /// according to BoP classification.
    ///
    /// # Returns
    /// `true` if this is a capital account transaction (E-series)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let capital = Field26T::new("E01").unwrap();
    /// assert!(capital.is_capital_transaction());
    /// ```
    pub fn is_capital_transaction(&self) -> bool {
        self.transaction_type_code.starts_with('E')
    }

    /// Check if this transaction requires enhanced reporting
    ///
    /// Determines if the transaction type typically requires additional
    /// documentation or enhanced reporting for regulatory purposes.
    ///
    /// # Returns
    /// `true` if enhanced reporting is typically required
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let investment = Field26T::new("F01").unwrap();
    /// assert!(investment.requires_enhanced_reporting());
    /// ```
    pub fn requires_enhanced_reporting(&self) -> bool {
        matches!(
            self.transaction_type_code.as_str(),
            "F01" | "F02" | "F03" | "F05" | "E01" | "E02"
        )
    }

    /// Get the reporting priority level
    ///
    /// Returns the priority level for statistical reporting based on
    /// the transaction type. Higher numbers indicate higher priority.
    ///
    /// # Returns
    /// Priority level (1=low, 2=normal, 3=high, 4=critical)
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let reserves = Field26T::new("F05").unwrap();
    /// assert_eq!(reserves.reporting_priority(), 4);
    /// ```
    pub fn reporting_priority(&self) -> u8 {
        match self.transaction_type_code.as_str() {
            "F05" => 4,                 // Reserve assets - critical
            "F01" | "F02" | "F03" => 3, // Investment flows - high
            "E01" | "E02" => 3,         // Capital account - high
            "A01" | "A02" | "A03" => 2, // Goods - normal
            "B01" | "B02" | "B03" => 2, // Services - normal
            "C01" | "C02" => 2,         // Primary income - normal
            "D01" | "D02" => 1,         // Secondary income - low
            "F04" => 2,                 // Other investment - normal
            _ => 1,                     // Unknown - low
        }
    }

    /// Get human-readable description based on common EUROSTAT BoP codes
    ///
    /// Returns a detailed description of what this transaction type represents
    /// in the context of Balance of Payments classification.
    ///
    /// # Returns
    /// A descriptive string
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let field = Field26T::new("A01").unwrap();
    /// println!("{}", field.description());
    /// ```
    pub fn description(&self) -> &'static str {
        match self.transaction_type_code.as_str() {
            // Common EUROSTAT Balance of Payments codes
            "A01" => "Goods - General merchandise on a gross basis",
            "A02" => "Goods - Goods for processing",
            "A03" => "Goods - Repairs on goods",
            "B01" => "Services - Manufacturing services on physical inputs owned by others",
            "B02" => "Services - Maintenance and repair services n.i.e.",
            "B03" => "Services - Transport",
            "C01" => "Primary income - Compensation of employees",
            "C02" => "Primary income - Investment income",
            "D01" => "Secondary income - General government",
            "D02" => "Secondary income - Other sectors",
            "E01" => "Capital account - Capital transfers",
            "E02" => "Capital account - Acquisition/disposal of non-produced, non-financial assets",
            "F01" => "Financial account - Direct investment",
            "F02" => "Financial account - Portfolio investment",
            "F03" => "Financial account - Financial derivatives",
            "F04" => "Financial account - Other investment",
            "F05" => "Financial account - Reserve assets",
            _ => "Transaction type code (refer to EUROSTAT BoP guidelines)",
        }
    }

    /// Get the statistical significance level
    ///
    /// Returns the statistical significance of this transaction type
    /// for macroeconomic analysis and policy making.
    ///
    /// # Returns
    /// Significance level description
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let field = Field26T::new("F01").unwrap();
    /// assert_eq!(field.statistical_significance(), "High - Key economic indicator");
    /// ```
    pub fn statistical_significance(&self) -> &'static str {
        match self.transaction_type_code.as_str() {
            "F01" | "F02" | "F05" => "High - Key economic indicator",
            "A01" | "B03" | "C02" => "High - Major economic component",
            "A02" | "A03" | "B01" | "B02" => "Medium - Significant for trade analysis",
            "C01" | "F03" | "F04" => "Medium - Important for financial analysis",
            "D01" | "E01" | "E02" => "Medium - Relevant for fiscal analysis",
            "D02" => "Low - Supplementary indicator",
            _ => "Variable - Depends on specific code definition",
        }
    }

    /// Check if this code is well-formed according to BoP standards
    ///
    /// Performs additional validation beyond basic format checking,
    /// ensuring the code follows EUROSTAT BoP classification principles.
    ///
    /// # Returns
    /// `true` if the code is well-formed
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let good_code = Field26T::new("A01").unwrap();
    /// assert!(good_code.is_well_formed());
    ///
    /// let unknown_code = Field26T::new("Z99").unwrap();
    /// assert!(!unknown_code.is_well_formed());
    /// ```
    pub fn is_well_formed(&self) -> bool {
        // Check if it follows the standard BoP category structure
        if let Some('A' | 'B' | 'C' | 'D' | 'E' | 'F') = self.transaction_type_code.chars().next() {
            // Check if the remaining characters are digits (standard pattern)
            self.transaction_type_code[1..]
                .chars()
                .all(|c| c.is_ascii_digit())
        } else {
            false
        }
    }

    /// Get comprehensive transaction details
    ///
    /// Returns a detailed description including the transaction type code,
    /// BoP category, description, and statistical significance.
    ///
    /// # Returns
    /// Formatted string with comprehensive details
    ///
    /// # Example
    /// ```rust
    /// # use swift_mt_message::fields::Field26T;
    /// let field = Field26T::new("F01").unwrap();
    /// println!("{}", field.comprehensive_description());
    /// ```
    pub fn comprehensive_description(&self) -> String {
        format!(
            "{} ({}): {} - Significance: {}",
            self.transaction_type_code,
            self.bop_category(),
            self.description(),
            self.statistical_significance()
        )
    }
}

impl std::fmt::Display for Field26T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.transaction_type_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field26t_creation() {
        let field = Field26T::new("A01").unwrap();
        assert_eq!(field.transaction_type_code, "A01");
        assert_eq!(field.code(), "A01");
    }

    #[test]
    fn test_field26t_parse() {
        let field = Field26T::parse("B02").unwrap();
        assert_eq!(field.transaction_type_code, "B02");
    }

    #[test]
    fn test_field26t_parse_with_prefix() {
        let field = Field26T::parse(":26T:C01").unwrap();
        assert_eq!(field.transaction_type_code, "C01");

        let field = Field26T::parse("26T:D02").unwrap();
        assert_eq!(field.transaction_type_code, "D02");
    }

    #[test]
    fn test_field26t_case_normalization() {
        let field = Field26T::new("c03").unwrap();
        assert_eq!(field.transaction_type_code, "C03");
    }

    #[test]
    fn test_field26t_invalid_length() {
        let result = Field26T::new("AB"); // Too short
        assert!(result.is_err());

        let result = Field26T::new("ABCD"); // Too long
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_invalid_characters() {
        let result = Field26T::new("A@1"); // Invalid character @
        assert!(result.is_err());

        let result = Field26T::new("A-1"); // Invalid character -
        assert!(result.is_err());

        let result = Field26T::new("A.1"); // Invalid character .
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_empty() {
        let result = Field26T::new("");
        assert!(result.is_err());

        let result = Field26T::new("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_field26t_to_swift_string() {
        let field = Field26T::new("D04").unwrap();
        assert_eq!(field.to_swift_string(), ":26T:D04");
    }

    #[test]
    fn test_field26t_validation() {
        let field = Field26T::new("E05").unwrap();
        let result = field.validate();
        assert!(result.is_valid);

        let invalid_field = Field26T {
            transaction_type_code: "INVALID".to_string(),
        };
        let result = invalid_field.validate();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_field26t_format_spec() {
        assert_eq!(Field26T::format_spec(), "3!c");
    }

    #[test]
    fn test_field26t_display() {
        let field = Field26T::new("F06").unwrap();
        assert_eq!(format!("{}", field), "F06");
    }

    #[test]
    fn test_field26t_is_valid_format() {
        let field = Field26T::new("A01").unwrap();
        assert!(field.is_valid_format());

        let invalid_field = Field26T {
            transaction_type_code: "INVALID".to_string(),
        };
        assert!(!invalid_field.is_valid_format());
    }

    #[test]
    fn test_field26t_descriptions() {
        let field = Field26T::new("A01").unwrap();
        assert_eq!(
            field.description(),
            "Goods - General merchandise on a gross basis"
        );

        let field = Field26T::new("B03").unwrap();
        assert_eq!(field.description(), "Services - Transport");

        let field = Field26T::new("F01").unwrap();
        assert_eq!(field.description(), "Financial account - Direct investment");

        let field = Field26T::new("XYZ").unwrap();
        assert_eq!(
            field.description(),
            "Transaction type code (refer to EUROSTAT BoP guidelines)"
        );
    }

    #[test]
    fn test_field26t_common_codes() {
        // Test some common EUROSTAT BoP codes
        let codes = ["A01", "A02", "B01", "C01", "D01", "E01", "F01"];

        for code in codes {
            let field = Field26T::new(code).unwrap();
            assert_eq!(field.code(), code);
            assert!(field.is_valid_format());
            assert!(!field.description().is_empty());
        }
    }

    #[test]
    fn test_field26t_bop_categories() {
        let test_cases = [
            ("A01", "Goods"),
            ("B02", "Services"),
            ("C01", "Primary Income"),
            ("D02", "Secondary Income"),
            ("E01", "Capital Account"),
            ("F01", "Financial Account"),
            ("Z99", "Other"),
        ];

        for (code, expected_category) in test_cases {
            let field = Field26T::new(code).unwrap();
            assert_eq!(
                field.bop_category(),
                expected_category,
                "Category mismatch for code {}",
                code
            );
        }
    }

    #[test]
    fn test_field26t_transaction_type_checks() {
        // Goods transactions
        let goods_codes = ["A01", "A02", "A03"];
        for code in goods_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                field.is_goods_transaction(),
                "Code {} should be goods transaction",
                code
            );
            assert!(!field.is_services_transaction());
            assert!(!field.is_income_transaction());
            assert!(!field.is_financial_transaction());
            assert!(!field.is_capital_transaction());
        }

        // Services transactions
        let services_codes = ["B01", "B02", "B03"];
        for code in services_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                field.is_services_transaction(),
                "Code {} should be services transaction",
                code
            );
            assert!(!field.is_goods_transaction());
            assert!(!field.is_income_transaction());
            assert!(!field.is_financial_transaction());
            assert!(!field.is_capital_transaction());
        }

        // Income transactions
        let income_codes = ["C01", "C02", "D01", "D02"];
        for code in income_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                field.is_income_transaction(),
                "Code {} should be income transaction",
                code
            );
            assert!(!field.is_goods_transaction());
            assert!(!field.is_services_transaction());
            assert!(!field.is_financial_transaction());
            assert!(!field.is_capital_transaction());
        }

        // Financial transactions
        let financial_codes = ["F01", "F02", "F03", "F04", "F05"];
        for code in financial_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                field.is_financial_transaction(),
                "Code {} should be financial transaction",
                code
            );
            assert!(!field.is_goods_transaction());
            assert!(!field.is_services_transaction());
            assert!(!field.is_income_transaction());
            assert!(!field.is_capital_transaction());
        }

        // Capital transactions
        let capital_codes = ["E01", "E02"];
        for code in capital_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                field.is_capital_transaction(),
                "Code {} should be capital transaction",
                code
            );
            assert!(!field.is_goods_transaction());
            assert!(!field.is_services_transaction());
            assert!(!field.is_income_transaction());
            assert!(!field.is_financial_transaction());
        }
    }

    #[test]
    fn test_field26t_enhanced_reporting_requirements() {
        let enhanced_codes = ["F01", "F02", "F03", "F05", "E01", "E02"];
        for code in enhanced_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                field.requires_enhanced_reporting(),
                "Code {} should require enhanced reporting",
                code
            );
        }

        let standard_codes = ["A01", "B01", "C01", "D01", "F04"];
        for code in standard_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                !field.requires_enhanced_reporting(),
                "Code {} should not require enhanced reporting",
                code
            );
        }
    }

    #[test]
    fn test_field26t_reporting_priority() {
        let test_cases = [
            ("F05", 4), // Reserve assets - critical
            ("F01", 3), // Direct investment - high
            ("F02", 3), // Portfolio investment - high
            ("F03", 3), // Financial derivatives - high
            ("E01", 3), // Capital transfers - high
            ("E02", 3), // Non-produced assets - high
            ("A01", 2), // General merchandise - normal
            ("B01", 2), // Manufacturing services - normal
            ("C01", 2), // Compensation of employees - normal
            ("F04", 2), // Other investment - normal
            ("D01", 1), // Government transfers - low
            ("D02", 1), // Other sectors transfers - low
            ("Z99", 1), // Unknown - low
        ];

        for (code, expected_priority) in test_cases {
            let field = Field26T::new(code).unwrap();
            assert_eq!(
                field.reporting_priority(),
                expected_priority,
                "Priority mismatch for code {}",
                code
            );
        }
    }

    #[test]
    fn test_field26t_statistical_significance() {
        let high_significance = ["F01", "F02", "F05", "A01", "B03", "C02"];
        for code in high_significance {
            let field = Field26T::new(code).unwrap();
            let significance = field.statistical_significance();
            assert!(
                significance.starts_with("High"),
                "Code {} should have high significance, got: {}",
                code,
                significance
            );
        }

        let medium_significance = ["A02", "B01", "C01", "F03", "D01", "E01"];
        for code in medium_significance {
            let field = Field26T::new(code).unwrap();
            let significance = field.statistical_significance();
            assert!(
                significance.starts_with("Medium"),
                "Code {} should have medium significance, got: {}",
                code,
                significance
            );
        }

        let low_significance = ["D02"];
        for code in low_significance {
            let field = Field26T::new(code).unwrap();
            let significance = field.statistical_significance();
            assert!(
                significance.starts_with("Low"),
                "Code {} should have low significance, got: {}",
                code,
                significance
            );
        }
    }

    #[test]
    fn test_field26t_well_formed_validation() {
        // Well-formed codes (standard BoP structure)
        let well_formed_codes = [
            "A01", "A02", "A03", "B01", "B02", "B03", "C01", "C02", "D01", "D02", "E01", "E02",
            "F01", "F02", "F03", "F04", "F05",
        ];
        for code in well_formed_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                field.is_well_formed(),
                "Code {} should be well-formed",
                code
            );
        }

        // Poorly formed codes (don't follow standard structure)
        let poorly_formed_codes = ["Z99", "ABC", "12A", "A0A"];
        for code in poorly_formed_codes {
            let field = Field26T::new(code).unwrap();
            assert!(
                !field.is_well_formed(),
                "Code {} should not be well-formed",
                code
            );
        }
    }

    #[test]
    fn test_field26t_comprehensive_description() {
        let field = Field26T::new("F01").unwrap();
        let desc = field.comprehensive_description();
        assert!(desc.contains("F01"));
        assert!(desc.contains("Financial Account"));
        assert!(desc.contains("Direct investment"));
        assert!(desc.contains("High - Key economic indicator"));
    }

    #[test]
    fn test_field26t_serialization() {
        let field = Field26T::new("A01").unwrap();
        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: Field26T = serde_json::from_str(&serialized).unwrap();

        assert_eq!(field.code(), deserialized.code());
        assert_eq!(field.bop_category(), deserialized.bop_category());
        assert_eq!(field.description(), deserialized.description());
        assert_eq!(
            field.statistical_significance(),
            deserialized.statistical_significance()
        );
    }

    #[test]
    fn test_field26t_business_logic_combinations() {
        // Test combinations of business logic
        let field = Field26T::new("F01").unwrap(); // Direct investment
        assert!(field.is_financial_transaction());
        assert!(field.requires_enhanced_reporting());
        assert_eq!(field.reporting_priority(), 3);
        assert_eq!(
            field.statistical_significance(),
            "High - Key economic indicator"
        );
        assert!(field.is_well_formed());

        let field = Field26T::new("D02").unwrap(); // Other sectors transfers
        assert!(field.is_income_transaction());
        assert!(!field.requires_enhanced_reporting());
        assert_eq!(field.reporting_priority(), 1);
        assert_eq!(
            field.statistical_significance(),
            "Low - Supplementary indicator"
        );
        assert!(field.is_well_formed());
    }

    #[test]
    fn test_field26t_edge_cases() {
        // Test edge cases and boundary conditions
        let field = Field26T::new("F05").unwrap(); // Reserve assets
        assert_eq!(field.reporting_priority(), 4); // Highest priority
        assert!(field.requires_enhanced_reporting());
        assert!(field.is_financial_transaction());

        // Test unknown but valid format
        let field = Field26T::new("G01").unwrap();
        assert_eq!(field.bop_category(), "Other");
        assert!(!field.is_well_formed());
        assert_eq!(field.reporting_priority(), 1);
    }

    #[test]
    fn test_field26t_real_world_scenarios() {
        // Scenario 1: International trade transaction
        let trade = Field26T::new("A01").unwrap();
        assert_eq!(
            trade.description(),
            "Goods - General merchandise on a gross basis"
        );
        assert!(trade.is_goods_transaction());
        assert_eq!(trade.reporting_priority(), 2);
        assert!(!trade.requires_enhanced_reporting());

        // Scenario 2: Foreign direct investment
        let fdi = Field26T::new("F01").unwrap();
        assert_eq!(fdi.description(), "Financial account - Direct investment");
        assert!(fdi.is_financial_transaction());
        assert_eq!(fdi.reporting_priority(), 3);
        assert!(fdi.requires_enhanced_reporting());

        // Scenario 3: Personal remittance
        let remittance = Field26T::new("D02").unwrap();
        assert_eq!(remittance.description(), "Secondary income - Other sectors");
        assert!(remittance.is_income_transaction());
        assert_eq!(remittance.reporting_priority(), 1);
        assert!(!remittance.requires_enhanced_reporting());

        // Scenario 4: Central bank reserves
        let reserves = Field26T::new("F05").unwrap();
        assert_eq!(reserves.description(), "Financial account - Reserve assets");
        assert!(reserves.is_financial_transaction());
        assert_eq!(reserves.reporting_priority(), 4);
        assert!(reserves.requires_enhanced_reporting());
    }
}
