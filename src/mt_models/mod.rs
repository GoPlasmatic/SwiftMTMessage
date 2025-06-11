//! MT Models - SWIFT Message Type Models and Field Definitions
//!
//! This module contains the structured field definitions and message type models
//! for SWIFT MT messages, organized according to the proposal design.

pub mod fields;
pub mod mt103;
pub mod mt103_stp;
pub mod mt202;

// Re-export field types for convenience
pub use fields::beneficiary::*;
pub use fields::charges::*;
pub use fields::common::*;
pub use fields::ordering_customer::*;

// Re-export message types
pub use mt103::MT103;
pub use mt103_stp::{MT103STP, STPRuleViolation, STPValidationReport};
pub use mt202::MT202;
