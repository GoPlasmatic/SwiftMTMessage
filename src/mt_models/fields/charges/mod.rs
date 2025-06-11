//! Details of charges fields (Field 71A, 71F, 71G)

pub mod field_71a;
pub mod field_71f;
pub mod field_71g;

// Re-export field types
pub use field_71a::Field71A;
pub use field_71f::Field71F;
pub use field_71g::Field71G;
