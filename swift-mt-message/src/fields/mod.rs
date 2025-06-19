pub mod common;

pub mod field12;
pub mod field13c;
pub mod field13d;
pub mod field20;
pub mod field21;
pub mod field23;
pub mod field23b;
pub mod field23e;
pub mod field25;
pub mod field26t;
pub mod field28c;
pub mod field28d;
pub mod field30;
pub mod field32a;
pub mod field36;
pub mod field50;

pub mod field59;
pub mod field70;
pub mod field71a;
pub mod field72;
pub mod field75;
pub mod field76;
pub mod field77b;
pub mod field77t;

pub mod field28;
pub mod field34f;
pub mod field37h;
pub mod field61;
pub mod field86;
pub mod field90c;
pub mod field90d;

// Balance field modules removed - now handled by common::balance_field
// pub mod field60a; // Removed - replaced by macro-generated Field60A in common
// pub mod field60f; // Removed - replaced by macro-generated Field60F in common
// pub mod field62a; // Removed - replaced by macro-generated Field62A in common
// pub mod field62f; // Removed - replaced by macro-generated Field62F in common
// pub mod field64;  // Removed - replaced by macro-generated Field64 in common
// pub mod field65;  // Removed - replaced by macro-generated Field65 in common

// Re-export all field types for easy access
pub use common::{
    Field60A, Field60F, Field62A, Field62F, Field64, Field65, GenericAccountField,
    GenericBalanceField, GenericBicField, GenericCurrencyAmountField, GenericNameAddressField,
    GenericPartyField, MultiLineField,
};

pub use field12::Field12;
pub use field13c::Field13C;
pub use field13d::Field13D;
pub use field20::Field20;
pub use field21::Field21;
pub use field23::Field23;
pub use field23b::Field23B;
pub use field23e::Field23E;
pub use field25::Field25;
pub use field26t::Field26T;
pub use field28c::Field28C;
pub use field28d::Field28D;
pub use field30::Field30;
pub use field32a::Field32A;
pub use field36::Field36;
pub use field50::{Field50, Field50F, Field50K};

pub use field59::{Field59, Field59F};
pub use field70::Field70;
pub use field71a::Field71A;
pub use field72::Field72;
pub use field75::Field75;
pub use field76::Field76;
pub use field77b::Field77B;
pub use field77t::Field77T;

// Re-export common types for convenience
pub use crate::common::BIC;

pub use field28::Field28;
pub use field34f::Field34F;
pub use field37h::Field37H;
pub use field61::Field61;
pub use field86::Field86;
pub use field90c::Field90C;
pub use field90d::Field90D;

// Balance fields now come from common module
// pub use field60a::Field60A; // Replaced by common::Field60A
// pub use field60f::Field60F; // Replaced by common::Field60F
// pub use field62a::Field62A; // Replaced by common::Field62A
// pub use field62f::Field62F; // Replaced by common::Field62F
// pub use field64::Field64;   // Replaced by common::Field64
// pub use field65::Field65;   // Replaced by common::Field65
