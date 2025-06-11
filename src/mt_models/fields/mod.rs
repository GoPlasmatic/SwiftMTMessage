//! Field Definitions for SWIFT MT Messages
//!
//! This module contains field definitions organized by category for better maintainability
//! and following the proposal's modular field architecture.

pub mod beneficiary;
pub mod charges;
pub mod common;
pub mod institutions;
pub mod ordering_customer;

// Re-export all field types for convenience
pub use beneficiary::*;
pub use charges::*;
pub use common::*;
pub use institutions::*;
pub use ordering_customer::*;
