use crate::{SwiftMessage, fields::*};
use serde::{Deserialize, Serialize};

/// MT202: General Financial Institution Transfer
///
/// This message type is used by financial institutions to transfer funds
/// for their own account or for account of a customer to a receiving institution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "202")]
pub struct MT202 {
    /// Field 20: Transaction Reference
    #[field("20")]
    pub field_20: Field20,

    /// Field 32A: Value Date, Currency Code, Amount
    #[field("32A")]
    pub field_32a: Field32A,
}

impl MT202 {
    /// Create a new MT202 with the given fields
    pub fn new(field_20: Field20, field_32a: Field32A) -> Self {
        Self {
            field_20,
            field_32a,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the currency code
    pub fn currency_code(&self) -> &str {
        self.field_32a.currency_code()
    }
}
