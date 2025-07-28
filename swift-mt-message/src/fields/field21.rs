use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

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
/// :21:CUST/20250719/001     // Customer payment reference
/// :21C:FX/USD/EUR/12345     // Treasury deal reference
/// :21D:SPOT20250719001      // Deal reference for spot transaction
/// :21E:REL/MT103/20250719   // Related transaction reference
/// :21F:BATCH001             // File reference for batch
/// :21R:RELBATCH001          // Related file reference
/// ```
///
/// ## Regional Considerations
/// - **European Payments**: SEPA reference standards compliance
/// - **US Payments**: FedWire reference format considerations
/// - **Asian Markets**: Local reference numbering schemes
/// - **Cross-Border**: International reference coordination
///
/// ## Error Prevention
/// - **Reference Validation**: Verify uniqueness within institutional context
/// - **Format Checking**: Ensure compliance with character set restrictions
/// - **Length Verification**: Confirm reference length within option limits
/// - **Slash Validation**: Check proper slash usage and positioning
///
/// ## Related Fields
/// - **Field 20**: Sender's Reference (primary transaction identifier)
/// - **Field 11**: MT Reference (message-level reference)
/// - **Field 72**: Sender to Receiver Information (additional reference details)
/// - **Block Headers**: Message references in application headers
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21NoOption {
    /// Transaction reference (up to 16 characters)
    ///
    /// Format: 16x - Alphanumeric with Swift character set restrictions
    /// Must not start/end with slash or contain consecutive slashes
    #[component("16x")]
    pub reference: String,
}

///   **Field 21C: Customer-Specific Reference**
///
/// Extended reference capability for customer-specific transaction identification
/// in treasury operations and complex financial transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21C {
    /// Customer reference (up to 35 characters)
    ///
    /// Format: 35x - Extended alphanumeric reference for complex transactions
    /// Used in treasury operations requiring detailed reference information
    #[component("35x")]
    pub reference: String,
}

///   **Field 21D: Deal Reference**
///
/// Deal reference for treasury and money market transactions, providing
/// detailed identification for financial market operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21D {
    /// Deal reference (up to 35 characters)
    ///
    /// Format: 35x - Deal identification in treasury and money markets
    /// Essential for tracking foreign exchange and derivatives transactions
    #[component("35x")]
    pub reference: String,
}

///   **Field 21E: Related Reference**
///
/// Related reference for linking transactions and instructions across
/// multiple messages in a transaction chain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21E {
    /// Related reference (up to 35 characters)
    ///
    /// Format: 35x - Cross-reference to link related instructions
    /// Enables transaction correlation and audit trail maintenance
    #[component("35x")]
    pub reference: String,
}

///   **Field 21F: File Reference**
///
/// File reference for batch operations, enabling grouping and identification
/// of multiple transactions processed together.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21F {
    /// File reference (up to 16 characters)
    ///
    /// Format: 16x - Batch identification for grouped transactions
    /// Used in MT102 and other batch processing scenarios
    #[component("16x")]
    pub reference: String,
}

///   **Field 21R: Related File Reference**
///
/// Related file reference for linking to previously sent file references
/// in batch processing scenarios.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field21R {
    /// Related file reference (up to 16 characters)
    ///
    /// Format: 16x - Reference to previously sent file or batch
    /// Enables correlation between related batch operations
    #[component("16x")]
    pub reference: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftField;
    
    #[test]
    fn test_field21r_parsing() {
        // First test serialization to understand the format
        let field = Field21R {
            reference: "REFBATCH001".to_string(),
        };
        let serialized = field.to_swift_string();
        println!("Serialized Field21R: '{}'", serialized);
        
        // Test parsing
        let test_cases = vec![
            ("REFBATCH001", "REFBATCH001"),
            ("ABC123", "ABC123"),
            ("TEST/REF/001", "TEST/REF/001"),
            ("1234567890123456", "1234567890123456"), // Max 16 chars
        ];
        
        for (input, expected) in test_cases {
            let result = Field21R::parse(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            assert_eq!(result.unwrap().reference, expected);
        }
        
        // Test invalid cases
        let invalid_cases = vec![
            "12345678901234567", // Too long (17 chars)
            "//INVALID", // Starts with consecutive slashes  
            "INVALID//", // Ends with consecutive slashes
            "INV//ALID", // Contains consecutive slashes
        ];
        
        for input in invalid_cases {
            let result = Field21R::parse(input);
            assert!(result.is_err(), "Should have failed to parse: {}", input);
        }
    }
}
