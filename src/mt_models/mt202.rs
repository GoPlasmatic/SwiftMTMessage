//! MT202 - General Financial Institution Transfer
//!
//! This message is sent by or on behalf of a financial institution to another
//! financial institution to order the transfer of funds.

use crate::errors::Result;
use crate::field_parser::SwiftMessage;
use crate::mt_models::fields::common::{Field20, Field32A};
use serde::{Deserialize, Serialize};

/// MT202 - General Financial Institution Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT202 {
    /// Field 20: Transaction Reference Number
    pub field_20: Field20,
    /// Field 21: Related Reference (placeholder)
    pub field_21: Option<String>,
    /// Field 32A: Value Date, Currency Code, Amount
    pub field_32a: Field32A,
}

impl MT202 {
    /// Create MT202 from generic SwiftMessage (placeholder implementation)
    pub fn from_swift_message(_message: SwiftMessage) -> Result<Self> {
        todo!("MT202 implementation not yet complete")
    }

    /// Convert to generic SwiftMessage (placeholder implementation)
    pub fn to_swift_message(&self) -> SwiftMessage {
        todo!("MT202 implementation not yet complete")
    }
}
