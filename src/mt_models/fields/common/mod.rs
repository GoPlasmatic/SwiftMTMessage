//! Common SWIFT MT field implementations
//!
//! This module contains field implementations that are used across multiple MT message types.

pub mod field_13c;
pub mod field_20;
pub mod field_23b;
pub mod field_23e;
pub mod field_26t;
pub mod field_32a;
pub mod field_33b;
pub mod field_36;
pub mod field_70;
pub mod field_72;
pub mod field_77b;

// Re-export field types
pub use field_13c::Field13C;
pub use field_20::Field20;
pub use field_23b::Field23B;
pub use field_23e::Field23E;
pub use field_26t::Field26T;
pub use field_32a::Field32A;
pub use field_33b::Field33B;
pub use field_36::Field36;
pub use field_70::Field70;
pub use field_72::Field72;
pub use field_77b::Field77B;
