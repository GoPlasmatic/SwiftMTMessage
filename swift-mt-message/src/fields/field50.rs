//! # Field 50: Ordering Customer
//!
//! ## Purpose
//! Identifies the ordering customer (originator) of the payment instruction. The ordering customer
//! is the party that instructs the sender of the MT103 to execute the payment. Different options
//! provide various levels of detail and identification methods for optimal processing efficiency.
//!
//! ## Options Overview
//! - **Option A**: Account + BIC identification (optimal for STP)
//! - **Option F**: Party identifier + BIC (structured identification)
//! - **Option K**: Account + Name/Address details (flexible format)
//! - **Option C**: BIC identification only (institution-based)
//! - **Option G**: Account + BIC (alternative format)
//! - **Option H**: Account + Name/Address (alternative format)
//! - **Option L**: Party identifier only (simplified identification)
//! - **No Option**: Name/Address only (basic identification)
//!
//! ## Usage by Message Type
//! - **MT103**: Options A, F, K supported (Field50OrderingCustomerAFK)
//! - **MT101**: Options A, F, K supported for batch payments
//! - **MT102**: Options available depending on batch type
//! - **Creditor Payments**: Options A, K supported (Field50Creditor)
//! - **Instructing Party**: Options C, L supported (Field50InstructingParty)
//!
//! ## STP Compliance Guidelines
//! ### STP Preferred (Optimal Automation)
//! - **Option A**: Account + BIC - maximum STP efficiency
//! - **Option F**: Party identifier + BIC - structured processing
//! - **Option C**: BIC only - institution-based routing
//!
//! ### STP Compatible (Good Automation)
//! - **Option K**: Account + Name/Address with complete information
//! - **Option G**: Account + BIC alternative format
//! - **Option H**: Account + Name/Address alternative format
//!
//! ### Manual Processing Risk
//! - **No Option**: Name/Address only - may require manual intervention
//! - **Option L**: Party identifier only - limited automation
//!
//! ## Format Selection Guidelines
//! ### When to Use Each Option
//! - **Option A**: Standard customer payments with account and BIC
//! - **Option F**: Enhanced identification requirements
//! - **Option K**: Flexible customer identification scenarios
//! - **Option C**: Institution-to-institution transactions
//! - **Option G/H**: Alternative formats for specific message types
//! - **Option L**: Simplified party identification
//! - **No Option**: Basic customer identification
//!
//! ## Business Context Applications
//! - **Payment Origination**: Customer-initiated payment instructions
//! - **Corporate Payments**: Business-to-business transaction origination
//! - **Retail Payments**: Individual customer payment instructions
//! - **Batch Processing**: Multiple payment origination identification
//!
//! ## Network Validation Requirements
//! - **BIC Validation**: Must be active and reachable in SWIFT network
//! - **Account Validation**: Must conform to local account standards
//! - **Character Set**: Standard SWIFT character set compliance
//! - **Address Standards**: Adequate detail for customer identification
//!
//! ## Compliance Framework
//! - **KYC Standards**: Customer identification and verification
//! - **AML Requirements**: Anti-money laundering originator screening
//! - **Regulatory Documentation**: Complete originator record keeping
//! - **Audit Trail**: Comprehensive origination audit information
//!
//! ## Related Fields Integration
//! - **Field 52A**: Ordering Institution (originator's bank)
//! - **Field 53A**: Sender's Correspondent (routing)
//! - **Field 59**: Beneficiary Customer (payment destination)
//! - **Field 70**: Remittance Information (payment purpose)
//!
//! ## Error Prevention Guidelines
//! - **Complete Information**: Provide full originator identification details
//! - **Accurate Codes**: Verify BIC codes and account numbers
//! - **Format Consistency**: Follow established format conventions
//! - **Compliance Verification**: Screen against sanctions and watch lists
//!
//! ## See Also
//! - Swift FIN User Handbook: Ordering Customer Specifications
//! - KYC Guidelines: Customer Identification Requirements
//! - AML/CFT Compliance: Originator Screening Best Practices
//! - STP Implementation Guide: Ordering Customer Optimization

use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message_macros::serde_swift_fields;

/// **Field 50 (No Option): Ordering Customer**
///
/// Basic variant of [Field 50 module](index.html). Provides ordering customer identification
/// using name and address information only.
///
/// **Components:**
/// - Name and address lines (4*35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50NoOption {
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

/// **Field 50A: Ordering Customer (Option A)**
///
/// Account + BIC variant of [Field 50 module](index.html). Provides structured identification
/// using optional account identifier and numbered name/address lines.
///
/// **Components:**
/// - Party identifier (optional, \[/34x\])
/// - Name and address lines (4*(1!n/33x))
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50A {
    /// Optional account identifier (IBAN, account number, etc.)
    /// Format: [/34x] - Up to 34 characters with leading slash
    #[component("[/34x]")]
    pub party_identifier: Option<String>,

    /// Name and address lines with mandatory line numbering
    /// Format: 4*(1!n/33x) - Line number + slash + 33 character text
    #[component("4*(1!n/33x)")]
    pub name_and_address: Vec<String>,
}

/// **Field 50F: Ordering Customer (Option F)**
///
/// Party identifier + BIC variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Account (35x)
/// - BIC (4!a2!a2!c\[3!c\])
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50F {
    #[component("35x")]
    pub account: String,
    /// Name and address lines
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

/// **Field 50K: Ordering Customer (Option K)**
///
/// Flexible variant of [Field 50 module](index.html). Provides ordering customer identification
/// using optional account information and free-format name/address details.
///
/// **Components:**
/// - Account (optional, \[/34x\])
/// - Name and address lines (4*35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50K {
    /// Optional account identifier (free format)
    /// Format: \[/34x\] - Up to 34 characters with leading slash
    #[component("[/34x]")]
    pub account: Option<String>,

    /// Name and address information in free format
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

/// **Field 50C: Ordering Customer (Option C)**
///
/// BIC-only variant of [Field 50 module](index.html).
///
/// **Components:**
/// - BIC (4!a2!a2!c\[3!c\])
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50C {
    /// BIC code
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

/// **Field 50L: Ordering Customer (Option L)**
///
/// Party identifier variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Party identifier (35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50L {
    #[component("35x")]
    pub party_identifier: String,
}

/// **Field 50G: Ordering Customer (Option G)**
///
/// Account + BIC variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Account (/34x)
/// - BIC (4!a2!a2!c\[3!c\])
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50G {
    #[component("/34x")]
    pub account: String,
    /// Name and address lines
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

/// **Field 50H: Ordering Customer (Option H)**
///
/// Account + Name/Address variant of [Field 50 module](index.html).
///
/// **Components:**
/// - Account (/34x)
/// - Name and address lines (4*35x)
///
/// For complete documentation, see the [Field 50 module](index.html).
#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub struct Field50H {
    #[component("/34x")]
    pub account: String,
    /// Name and address lines
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field50InstructingParty {
    C(Field50C),
    L(Field50L),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field50OrderingCustomerFGH {
    F(Field50F),
    G(Field50G),
    H(Field50H),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field50OrderingCustomerAFK {
    A(Field50A),
    F(Field50F),
    K(Field50K),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field50OrderingCustomerNCF {
    NoOption(Field50NoOption),
    C(Field50C),
    F(Field50F),
}

#[serde_swift_fields]
#[derive(Debug, Clone, PartialEq, SwiftField, Serialize, Deserialize)]
pub enum Field50Creditor {
    A(Field50A),
    K(Field50K),
}
