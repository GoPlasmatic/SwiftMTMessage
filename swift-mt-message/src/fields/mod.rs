//! # SWIFT MT Message Fields
//!
//! ## Purpose
//! Comprehensive field definitions for all SWIFT MT message types, providing type-safe parsing,
//! validation, and serialization for individual message fields.
//!
//! ## Field Architecture
//! Each field module provides:
//! - **Type-Safe Structures**: Strongly-typed field definitions with validation
//! - **Format Compliance**: SWIFT User Handbook format specification adherence
//! - **Variant Support**: Complex enum fields with multiple format options (e.g., Field50: A/F/K)
//! - **Sample Generation**: Realistic sample data generation for testing
//! - **JSON Serialization**: Clean JSON output without enum wrapper layers
//!
//! ## Field Categories
//! - **Reference Fields**: Transaction references, message references (Field 20, 21)
//! - **Amount Fields**: Currencies, amounts, exchange rates (Field 32, 33, 36)
//! - **Party Fields**: Customer and institution identification (Field 50-59)
//! - **Date/Time Fields**: Value dates, execution dates (Field 30, 32A)
//! - **Instruction Fields**: Processing instructions and codes (Field 23, 71)
//! - **Information Fields**: Remittance and additional information (Field 70, 72)
//!
//! ## Field Format Support
//! All fields support SWIFT format specifications:
//! - `n`: Numeric characters (0-9)
//! - `a`: Alphabetic characters (A-Z, a-z)
//! - `c`: Capital letters (A-Z)
//! - `x`: Any character except spaces
//! - `h`: Hexadecimal characters (0-9, A-F)
//! - `d`: Decimal numbers with precision
//!
//! ## Usage Example
//! ```rust
//! use swift_mt_message::fields::{Field20, Field32A, Field50, Field59};
//!
//! // Parse simple field
//! let field_20 = Field20::parse("TXN123456")?;
//!
//! // Parse complex amount field
//! let field_32a = Field32A::parse("240315USD1000,00")?;
//!
//! // Parse enum field with variant
//! let field_50 = Field50::parse_with_variant("JOHN DOE\n123 MAIN ST", Some("K"), Some("50"))?;
//!
//! // Generate samples
//! let sample_field = Field59::sample();
//! ```

pub mod field11;
pub use field11::*;

pub mod field12;
pub use field12::*;

pub mod field13;
pub use field13::*;

pub mod field19;
pub use field19::*;

pub mod field20;
pub use field20::*;

pub mod field21;
pub use field21::*;

pub mod field23;
pub use field23::*;

pub mod field25;
pub use field25::*;

pub mod field26;
pub use field26::*;

pub mod field28;
pub use field28::*;

pub mod field30;
pub use field30::*;

pub mod field32;
pub use field32::*;

pub mod field33;
pub use field33::*;

pub mod field34;
pub use field34::*;

pub mod field36;
pub use field36::*;

pub mod field37;
pub use field37::*;

pub mod field50;
pub use field50::*;

pub mod field51;
pub use field51::*;

pub mod field52;
pub use field52::*;

pub mod field53;
pub use field53::*;

pub mod field54;
pub use field54::*;

pub mod field55;
pub use field55::*;

pub mod field56;
pub use field56::*;

pub mod field57;
pub use field57::*;

pub mod field58;
pub use field58::*;

pub mod field59;
pub use field59::*;

pub mod field60;
pub use field60::*;

pub mod field61;
pub use field61::*;

pub mod field62;
pub use field62::*;

pub mod field64;
pub use field64::*;

pub mod field65;
pub use field65::*;

pub mod field70;
pub use field70::*;

pub mod field71;
pub use field71::*;

pub mod field72;
pub use field72::*;

pub mod field75;
pub use field75::*;

pub mod field76;
pub use field76::*;

pub mod field77;
pub use field77::*;

pub mod field79;
pub use field79::*;

pub mod field86;
pub use field86::*;

pub mod field90;
pub use field90::*;
