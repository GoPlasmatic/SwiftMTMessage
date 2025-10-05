//! # SWIFT MT Message Fields
//!
//! Type-safe field definitions for all SWIFT MT message types with parsing,
//! validation, and serialization support.
//!
//! ## Field Categories
//! - **Reference:** Transaction/message references (20, 21)
//! - **Amount:** Currencies, amounts, exchange rates (32, 33, 36)
//! - **Party:** Customer and institution IDs (50-59)
//! - **Date/Time:** Value dates, execution dates (30, 32A)
//! - **Instruction:** Processing codes (23, 71)
//! - **Information:** Remittance and additional info (70, 72)
//!
//! ## Format Notation
//! - `n` = numeric (0-9)
//! - `a` = alphabetic (A-Z, a-z)
//! - `c` = uppercase (A-Z)
//! - `x` = any char
//! - `d` = decimal with precision
//! - `!` = exact length, `*` = max length
//!
//! ## Usage
//! ```rust
//! use swift_mt_message::fields::{Field20, Field32A};
//! use swift_mt_message::SwiftField;
//!
//! # fn main() -> swift_mt_message::Result<()> {
//! let ref_field = Field20::parse("TXN123456")?;
//! let amt_field = Field32A::parse("240315USD1000,00")?;
//! # Ok(())
//! # }
//! ```

// Utility modules
pub mod field_utils;
pub mod swift_utils;

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
