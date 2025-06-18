pub mod field13c;
pub mod field20;
pub mod field21;
pub mod field23b;
pub mod field23e;
pub mod field26t;
pub mod field32a;
pub mod field36;
pub mod field50;

pub mod field59;
pub mod field70;
pub mod field71a;
pub mod field72;
pub mod field77b;
pub mod field77t;

// Re-export all field types for easy access
pub use field13c::Field13C;
pub use field20::Field20;
pub use field21::Field21;
pub use field23b::Field23B;
pub use field23e::Field23E;
pub use field26t::Field26T;
pub use field32a::Field32A;
pub use field36::Field36;
pub use field50::{Field50, Field50F, Field50K};

pub use field59::{Field59, Field59F};
pub use field70::Field70;
pub use field71a::Field71A;
pub use field72::Field72;
pub use field77b::Field77B;
pub use field77t::Field77T;

// Re-export common types for convenience
pub use crate::common::BIC;
