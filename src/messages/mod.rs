//! # SWIFT MT Message Types
//!
//! Comprehensive message type implementations for SWIFT MT (Message Type) financial messages.
//! Each message struct provides parsing, validation, and serialization.
//!
//! ## Message Categories
//! - **Category 1 (MT1xx):** Customer payments and cheques
//! - **Category 2 (MT2xx):** Financial institution transfers
//! - **Category 9 (MT9xx):** Cash management and customer statements
//!
//! ## Usage
//! ```rust
//! use swift_mt_message::messages::MT103;
//! use swift_mt_message::traits::SwiftMessageBody;
//!
//! # fn main() -> swift_mt_message::Result<()> {
//! let mt103 = MT103::parse_from_block4(":20:REF123\n:23B:CRED\n...")?;
//! let mt_string = mt103.to_mt_string();
//! # Ok(())
//! # }
//! ```

// Message modules
pub mod mt101;
pub mod mt103;
pub mod mt104;
pub mod mt107;
pub mod mt110;
pub mod mt111;
pub mod mt112;
pub mod mt190;
pub mod mt191;
pub mod mt192;
pub mod mt196;
pub mod mt199;
pub mod mt200;
pub mod mt202;
pub mod mt204;
pub mod mt205;
pub mod mt210;
pub mod mt290;
pub mod mt291;
pub mod mt292;
pub mod mt296;
pub mod mt299;
pub mod mt900;
pub mod mt910;
pub mod mt920;
pub mod mt935;
pub mod mt940;
pub mod mt941;
pub mod mt942;
pub mod mt950;

// Re-export message types
pub use mt101::{MT101, MT101Transaction};
pub use mt103::MT103;
pub use mt104::{MT104, MT104Transaction};
pub use mt107::{MT107, MT107Transaction};
pub use mt110::{MT110, MT110Cheque};
pub use mt111::MT111;
pub use mt112::MT112;
pub use mt190::MT190;
pub use mt191::MT191;
pub use mt192::MT192;
pub use mt196::MT196;
pub use mt199::MT199;
pub use mt290::MT290;
pub use mt291::MT291;
pub use mt292::MT292;
pub use mt296::MT296;
pub use mt299::MT299;
pub use mt900::MT900;
pub use mt910::MT910;
pub use mt920::{MT920, MT920Sequence};
pub use mt935::{MT935, MT935RateChange};
pub use mt940::{MT940, MT940StatementLine};
pub use mt941::MT941;
pub use mt942::{MT942, MT942StatementLine};
pub use mt950::MT950;

// Re-export MT20x messages
pub use mt200::MT200;
pub use mt202::MT202;
pub use mt204::{MT204, MT204Transaction};
pub use mt205::MT205;
pub use mt210::MT210;
