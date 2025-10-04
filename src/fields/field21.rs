use super::swift_utils::{parse_max_length, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 21: Related Reference / Transaction Reference**
///
/// ## Purpose
/// Specifies a unique reference assigned by the sending financial institution to unambiguously
/// identify the transaction or instruction. This field serves as a cross-reference to link
/// related messages and facilitates transaction tracking across the payment chain.
///
/// ## Format
/// - **Swift Format**: `16x` (NoOption), `35x` (C, D, E options), `16x` (F, R options)
/// - **Description**: Alphanumeric characters with specific length restrictions per option
/// - **Character Set**: Letters, digits, and limited special characters (excluding consecutive slashes)
///
/// ## Presence
/// - **Status**: Conditional/Optional depending on message type and sequence
/// - **Swift Error Codes**: T26 (invalid characters), T50 (format violation)
/// - **Usage Context**: Transaction identification and cross-referencing
///
/// ## Usage Rules
/// - **Reference Uniqueness**: Must be unique within the context of the sending institution
/// - **Cross-Reference**: Often used to link related instructions or provide trace information
/// - **Slash Restrictions**: Must not start or end with slash, no consecutive slashes allowed
/// - **Transaction Chain**: Enables tracking across multiple message exchanges
///
/// ## Network Validation Rules
/// - **Character Validation**: Only alphanumeric and specific special characters allowed
/// - **Length Validation**: Must not exceed maximum length for specific option
/// - **Format Compliance**: Must follow Swift character set standards
/// - **Slash Rules**: Proper slash usage for structured references
///
/// ## Field Options and Usage
///
/// ### NoOption (16x)
/// - **Usage**: Basic transaction reference in customer payments (MT103)
/// - **Length**: Up to 16 characters
/// - **Purpose**: Simple transaction identification
///
/// ### Option C (35x)
/// - **Usage**: Customer-specific references, often in treasury operations
/// - **Length**: Up to 35 characters
/// - **Purpose**: Extended reference capability for complex transactions
///
/// ### Option D (35x)
/// - **Usage**: Deal reference in treasury and money market transactions
/// - **Length**: Up to 35 characters
/// - **Purpose**: Transaction identification in financial markets
///
/// ### Option E (35x)
/// - **Usage**: Related reference for linked transactions
/// - **Length**: Up to 35 characters
/// - **Purpose**: Cross-referencing between related instructions
///
/// ### Option F (16x)
/// - **Usage**: File reference for batch operations (MT102)
/// - **Length**: Up to 16 characters
/// - **Purpose**: Batch identification and grouping
///
/// ### Option R (16x)
/// - **Usage**: Related file reference
/// - **Length**: Up to 16 characters
/// - **Purpose**: Linking to previously sent file references
///
/// ## Business Context
/// - **Transaction Tracking**: Essential for audit trails and payment investigation
/// - **Reconciliation**: Enables matching of instructions with confirmations
/// - **STP Processing**: Facilitates automated processing and exception handling
/// - **Regulatory Compliance**: Supports regulatory reporting and monitoring requirements
///
/// ## Examples
/// ```logic
/// :21:REF20240719001  (Basic 16x reference)
/// :21C:TREASURY/SWAP/2024/07/19/001  (Customer reference 35x)
/// :21D:FX-DEAL-20240719-EUR-USD  (Deal reference)
/// :21E:ORIGINAL-REF-123456  (Related reference)
/// :21F:BATCH-20240719  (File reference)
/// :21R:FILE-REF-001  (Related file reference)
/// ```
///
/// ## Related Fields
/// - **Field 20**: Sender's Reference (primary transaction identifier)
/// - **Field 61**: Statement Line Reference
/// - **Field 108**: MT message user reference in Block 3
///
/// ## Message Type Usage
/// - **MT102**: Field 21F for file reference, Field 21R for related file reference
/// - **MT103**: Field 21 (no option) for transaction reference
/// - **MT200-295**: Various options for money market and treasury operations
/// - **MT300-395**: Deal references in treasury confirmations
///
/// ## Transaction Lifecycle
/// - **Initiation**: Original reference assignment by sending institution
/// - **Processing**: Reference propagation through payment chain
/// - **Confirmation**: Reference matching in return messages
/// - **Settlement**: Reference inclusion in settlement confirmations
///
/// ## STP Compliance
/// - **Automated Processing**: Reference format standardization for STP
/// - **Exception Handling**: Reference-based transaction investigation
/// - **Matching Logic**: Automated reference correlation across messages
/// - **Quality Control**: Reference validation in STP gateways
///
/// ## Compliance and Audit
/// - **Audit Trail**: Comprehensive transaction reference tracking
/// - **Regulatory Reporting**: Reference inclusion in compliance reports
/// - **Investigation Support**: Reference-based transaction reconstruction
/// - **Documentation**: Reference preservation for regulatory periods
///
/// ## See Also
/// - Swift FIN User Handbook: Reference Field Standards
/// - MT Message Reference Guide: Field 21 Specifications
/// - STP Guidelines: Reference Format Requirements
/// - Payment Processing Standards: Transaction Identification
///
///   **Field 21 NoOption: Basic Transaction Reference**
///
/// Basic transaction reference used in customer payment instructions.
/// Limited to 16 characters for simple transaction identification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21NoOption {
    /// Transaction reference (up to 16 characters)
    ///
    /// Format: 16x - Alphanumeric with Swift character set restrictions
    /// Must not start/end with slash or contain consecutive slashes
    pub reference: String,
}

impl SwiftField for Field21NoOption {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Parse the reference with max length of 16
        let reference = parse_max_length(input, 16, "Field 21 reference")?;

        // Validate SWIFT character set
        parse_swift_chars(&reference, "Field 21 reference")?;

        // Additional validation: no leading/trailing slashes
        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21 reference cannot start or end with '/'".to_string(),
            });
        }

        // Additional validation: no consecutive slashes
        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21 reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21NoOption { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21:{}", self.reference)
    }
}

///   **Field 21C: Customer-Specific Reference**
///
/// Extended reference capability for customer-specific transaction identification
/// in treasury operations and complex financial transactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21C {
    /// Customer reference (up to 35 characters)
    ///
    /// Format: 35x - Extended alphanumeric reference for complex transactions
    /// Used in treasury operations requiring detailed reference information
    pub reference: String,
}

impl SwiftField for Field21C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 35, "Field 21C reference")?;
        parse_swift_chars(&reference, "Field 21C reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21C reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21C reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21C { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21C:{}", self.reference)
    }
}

///   **Field 21D: Deal Reference**
///
/// Deal reference used in treasury and money market transactions
/// for specific deal identification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21D {
    /// Deal reference (up to 35 characters)
    ///
    /// Format: 35x - Deal identification for treasury and money market operations
    /// Typically includes deal type, date, currency pair for FX transactions
    pub reference: String,
}

impl SwiftField for Field21D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 35, "Field 21D reference")?;
        parse_swift_chars(&reference, "Field 21D reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21D reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21D reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21D { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21D:{}", self.reference)
    }
}

///   **Field 21E: Related Reference**
///
/// Reference to a related transaction or instruction,
/// used for linking connected financial operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21E {
    /// Related reference (up to 35 characters)
    ///
    /// Format: 35x - Links to previously sent or related transactions
    /// Essential for transaction chains and amendment references
    pub reference: String,
}

impl SwiftField for Field21E {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 35, "Field 21E reference")?;
        parse_swift_chars(&reference, "Field 21E reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21E reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21E reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21E { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21E:{}", self.reference)
    }
}

///   **Field 21F: File Reference**
///
/// File reference used in batch payment operations,
/// particularly in MT102 messages for batch identification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21F {
    /// File reference (up to 16 characters)
    ///
    /// Format: 16x - Identifies batch or file in bulk payment processing
    /// Critical for MT102 batch payment instructions
    pub reference: String,
}

impl SwiftField for Field21F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 16, "Field 21F reference")?;
        parse_swift_chars(&reference, "Field 21F reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21F reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21F reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21F { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21F:{}", self.reference)
    }
}

///   **Field 21R: Related File Reference**
///
/// Reference to a related file, used for linking batch operations
/// and providing continuity in bulk payment processing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21R {
    /// Related file reference (up to 16 characters)
    ///
    /// Format: 16x - Links to previously sent file or batch reference
    /// Used in conjunction with Field 21F for batch payment chains
    pub reference: String,
}

impl SwiftField for Field21R {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 16, "Field 21R reference")?;
        parse_swift_chars(&reference, "Field 21R reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21R reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21R reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21R { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21R:{}", self.reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field21_no_option() {
        let field = Field21NoOption::parse("REF20240719001").unwrap();
        assert_eq!(field.reference, "REF20240719001");
        assert_eq!(field.to_swift_string(), ":21:REF20240719001");

        // Test max length
        assert!(Field21NoOption::parse("1234567890ABCDEF").is_ok());
        assert!(Field21NoOption::parse("1234567890ABCDEFG").is_err());

        // Test slash validation
        assert!(Field21NoOption::parse("/REF123").is_err());
        assert!(Field21NoOption::parse("REF123/").is_err());
        assert!(Field21NoOption::parse("REF//123").is_err());
    }

    #[test]
    fn test_field21c() {
        let field = Field21C::parse("TREASURY/SWAP/2024/07/19/001").unwrap();
        assert_eq!(field.reference, "TREASURY/SWAP/2024/07/19/001");

        // Test max length (35 chars)
        let long_ref = "12345678901234567890123456789012345";
        assert!(Field21C::parse(long_ref).is_ok());
        assert!(Field21C::parse(&format!("{}X", long_ref)).is_err());
    }

    #[test]
    fn test_field21d() {
        let field = Field21D::parse("FX-DEAL-20240719-EUR-USD").unwrap();
        assert_eq!(field.reference, "FX-DEAL-20240719-EUR-USD");
        assert_eq!(field.to_swift_string(), ":21D:FX-DEAL-20240719-EUR-USD");
    }

    #[test]
    fn test_field21e() {
        let field = Field21E::parse("ORIGINAL-REF-123456").unwrap();
        assert_eq!(field.reference, "ORIGINAL-REF-123456");
    }

    #[test]
    fn test_field21f() {
        let field = Field21F::parse("BATCH-20240719").unwrap();
        assert_eq!(field.reference, "BATCH-20240719");

        // Test 16 char limit
        assert!(Field21F::parse("1234567890ABCDEF").is_ok());
        assert!(Field21F::parse("1234567890ABCDEFG").is_err());
    }

    #[test]
    fn test_field21r() {
        let field = Field21R::parse("FILE-REF-001").unwrap();
        assert_eq!(field.reference, "FILE-REF-001");
        assert_eq!(field.to_swift_string(), ":21R:FILE-REF-001");
    }
}
