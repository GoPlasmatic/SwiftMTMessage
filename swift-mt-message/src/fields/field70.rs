use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// Field 70: Remittance Information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("4*35x")]
pub struct Field70 {
    /// Remittance information lines (up to 4 lines of 35 characters each)
    #[format("4*35x")]
    pub information: Vec<String>,
}

impl Field70 {
    pub fn new(information: Vec<String>) -> Self {
        Self { information }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field70_creation() {
        let info = vec!["PAYMENT FOR INVOICE 12345".to_string()];
        let field70 = Field70::new(info.clone());
        assert_eq!(field70.information, info);
    }
}
