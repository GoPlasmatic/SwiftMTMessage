// === COMMON FIELD TYPES ===
// Re-export all lightweight common field structures
pub mod common;
pub use common::*;

// === SPECIALIZED FIELDS (Complex fields that need custom structure) ===
pub mod field13c;
pub mod field13d;
pub mod field23;
pub mod field23e;
pub mod field28;
pub mod field28c;
pub mod field28d;
pub mod field32a;
pub mod field34f;
pub mod field36;
pub mod field37h;
pub mod field50;
pub mod field59;
pub mod field61;
pub mod field77t;
pub mod field11s;

// === TYPE ALIASES FOR SEMANTIC CLARITY ===

// Reference fields (using GenericReferenceField)
pub type Field20 = GenericReferenceField; // Transaction Reference
pub type Field21 = GenericReferenceField; // Related Reference

// Simple text fields (using GenericTextField)
pub type Field12 = GenericTextField; // Message Requested
pub type Field23B = GenericTextField; // Bank Operation Code
pub type Field25 = GenericTextField; // Account Identification
pub type Field26T = GenericTextField; // Transaction Type Code
pub type Field30 = GenericTextField; // Value Date
pub type Field71A = GenericTextField; // Details of Charges

// Multiline text fields (using GenericMultiLineTextField)
pub type Field70 = GenericMultiLine4x35; // Remittance Information (4x35)
pub type Field72 = GenericMultiLine6x35; // Sender to Receiver Information (6x35)
pub type Field75 = GenericMultiLine6x35; // Queries (6x35)
pub type Field77B = GenericMultiLine3x35; // Regulatory Reporting (3x35)
pub type Field86 = GenericMultiLine6x65; // Information to Account Owner (6x65)

// Currency/Amount fields (using GenericCurrencyAmountField)
pub type Field33B = GenericCurrencyAmountField; // Instructed Amount
pub type Field71F = GenericCurrencyAmountField; // Sender's Charges
pub type Field71G = GenericCurrencyAmountField; // Receiver's Charges

// Summary fields (using GenericSummaryField)
pub type Field90C = GenericSummaryField; // Sum of Credits
pub type Field90D = GenericSummaryField; // Sum of Debits

// Complex fields (require specialized implementations)
pub use field13c::Field13C;
pub use field13d::Field13D;
pub use field23::Field23;
pub use field23e::Field23E;
pub use field28::Field28;
pub use field28c::Field28C;
pub use field28d::Field28D;
pub use field32a::Field32A;
pub use field34f::Field34F;
pub use field36::Field36;
pub use field37h::Field37H;
pub use field50::{Field50, Field50F, Field50K};
pub use field59::{Field59, Field59Basic, Field59F};
pub use field61::Field61;
pub use field77t::Field77T;
pub use field11s::Field11S;

/// Type alias for Field 60A - Opening Balance (Intermediate)
pub type Field60A = GenericBalanceField;
/// Type alias for Field 60F - Opening Balance (Final/Booked)  
pub type Field60F = GenericBalanceField;
/// Type alias for Field 62A - Closing Balance (Intermediate)
pub type Field62A = GenericBalanceField;
/// Type alias for Field 62F - Closing Balance (Final/Booked)
pub type Field62F = GenericBalanceField;
/// Type alias for Field 64 - Closing Available Balance
pub type Field64 = GenericBalanceField;
/// Type alias for Field 65 - Forward Available Balance
pub type Field65 = GenericBalanceField;
