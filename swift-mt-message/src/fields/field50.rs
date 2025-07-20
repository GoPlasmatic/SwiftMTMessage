use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// **Field 50: Ordering Customer**
///
/// ## Purpose
/// Identifies the ordering customer (originator) of the payment instruction. The ordering customer
/// is the party that instructs the sender of the MT103 to execute the payment. Different options
/// provide various levels of detail and identification methods.
///
/// ## Options Overview
/// - **Option A**: Account + BIC identification
/// - **Option F**: Party identifier + BIC  
/// - **Option K**: Account + Name/Address details
/// - **No Option**: Name/Address only (basic identification)
///
/// ## Usage by Message Type
/// - **MT103**: Options A, F, K supported (Field50OrderingCustomerAFK)
/// - **MT101**: Options A, F, K supported for batch payments
/// - **MT102**: Options available depending on batch type
///
/// ## STP Compliance
/// - **STP Preferred**: Options A and F (structured with BIC)
/// - **STP Compatible**: Option K with complete account information
/// - **Manual Processing**: No option may require manual intervention
///
/// ## Format Specifications per Option
/// Each option has specific format requirements detailed in the individual structs below.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50NoOption {
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

/// **Field 50A: Ordering Customer (Account + BIC)**
///
/// ## Purpose
/// Identifies the ordering customer using an account identifier and BIC code.
/// This option provides structured identification suitable for STP processing.
///
/// ## Format
/// - **Line 1**: `[/34x]` - Optional account identifier (max 34 chars)
/// - **Lines 2-5**: `4*(1!n/33x)` - Name and address with line numbers
///
/// ## Usage Rules
/// - **Account Line**: May contain IBAN or other account identifier
/// - **Line Numbering**: Each name/address line must start with line number (1-4)
/// - **STP Compliance**: Preferred option for automated processing
/// - **Validation**: Account format should match domestic standards
///
/// ## Examples
/// ```logic
/// :50A:/DE89370400440532013000
/// 1/HANS MUELLER
/// 2/HAUPTSTRASSE 123
/// 3/10115 BERLIN
/// 4/GERMANY
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50F {
    #[component("35x")]
    pub account: String,
    /// Name and address lines
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

/// **Field 50K: Ordering Customer (Account + Name/Address)**
///
/// ## Purpose
/// Identifies the ordering customer using optional account information and detailed
/// name/address information. Most flexible option for various identification scenarios.
///
/// ## Format
/// - **Line 1**: `[/34x]` - Optional account identifier
/// - **Lines 2-5**: `4*35x` - Name and address (up to 4 lines, 35 chars each)
///
/// ## Usage Rules
/// - **Account Optional**: Account line may be omitted if not available
/// - **Free Format**: Name/address lines do not require line numbering
/// - **Flexibility**: Accommodates various naming and addressing conventions
/// - **STP Compatible**: Acceptable for STP when complete information provided
///
/// ## Examples
/// ```logic
/// :50K:/1234567890
/// JOHN DOE ENTERPRISES
/// 123 BUSINESS PLAZA
/// NEW YORK NY 10001
/// UNITED STATES
///
/// :50K:JANE SMITH
/// PERSONAL ACCOUNT HOLDER
/// 456 RESIDENTIAL STREET
/// LONDON SW1A 1AA
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50K {
    /// Optional account identifier (free format)
    /// Format: [/34x] - Up to 34 characters with leading slash
    #[component("[/34x]")]
    pub account: Option<String>,

    /// Name and address information in free format
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50C {
    /// BIC code
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50L {
    #[component("35x")]
    pub party_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50G {
    #[component("/34x")]
    pub account: String,
    /// Name and address lines
    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50H {
    #[component("/34x")]
    pub account: String,
    /// Name and address lines
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50InstructingParty {
    C(Field50C),
    L(Field50L),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50OrderingCustomerFGH {
    F(Field50F),
    G(Field50G),
    H(Field50H),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50OrderingCustomerAFK {
    A(Field50A),
    F(Field50F),
    K(Field50K),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50OrderingCustomerNCF {
    NoOption(Field50NoOption),
    C(Field50C),
    F(Field50F),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50Creditor {
    A(Field50A),
    K(Field50K),
}
