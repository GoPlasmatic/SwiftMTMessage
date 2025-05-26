//! MT192: Request for Cancellation

use serde::{Deserialize, Serialize};
use crate::common::{Field, MessageBlock, tags};
use crate::error::{MTError, Result};
use crate::messages::{extract_text_block, find_field, find_fields, get_required_field_value, get_optional_field_value, MTMessageType};

/// MT192: Request for Cancellation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT192 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT192 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get related reference (Field 21) - Reference of the message to be cancelled
    pub fn related_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, "21")
    }

    /// Get reason for cancellation (Field 75) - optional
    pub fn reason_for_cancellation(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "75")
    }

    /// Get original message type (Field 11S) - optional
    pub fn original_message_type(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "11S")
    }

    /// Get copy of original message (Field 79) - optional
    pub fn copy_of_original_message(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "79")
    }

    /// Get requesting institution (Field 52A) - optional
    pub fn requesting_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get receiving institution (Field 58A) - optional
    pub fn receiving_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "58A")
    }

    /// Get narrative (Field 72) - optional additional information
    pub fn narrative(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "72")
    }

    /// Get all narrative fields (Field 72) - can have multiple
    pub fn narratives(&self) -> Vec<String> {
        find_fields(&self.fields, "72")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }
}

impl MTMessageType for MT192 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;
        
        // Validate required fields are present
        let required_fields = [
            tags::SENDER_REFERENCE, // Field 20
            "21", // Related reference
        ];

        for &field_tag in &required_fields {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        Ok(MT192 { fields })
    }

    fn get_field(&self, tag: &str) -> Option<&Field> {
        find_field(&self.fields, tag)
    }

    fn get_fields(&self, tag: &str) -> Vec<&Field> {
        find_fields(&self.fields, tag)
    }

    fn get_all_fields(&self) -> Vec<&Field> {
        self.fields.iter().collect()
    }

    fn text_fields(&self) -> &[Field] {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Field;

    fn create_test_mt192() -> MT192 {
        let fields = vec![
            Field::new("20", "CANCEL123456"),
            Field::new("21", "ORIG987654321"),
            Field::new("11S", "103"),
            Field::new("75", "DUPLICATE PAYMENT"),
            Field::new("52A", "REQUESTING BANK\nADDRESS"),
            Field::new("58A", "RECEIVING BANK\nADDRESS"),
            Field::new("72", "URGENT CANCELLATION REQUIRED"),
            Field::new("72", "PLEASE CONFIRM RECEIPT"),
            Field::new("79", "COPY OF ORIGINAL MT103 MESSAGE..."),
        ];
        MT192 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt192 = create_test_mt192();
        assert_eq!(mt192.transaction_reference().unwrap(), "CANCEL123456");
    }

    #[test]
    fn test_related_reference() {
        let mt192 = create_test_mt192();
        assert_eq!(mt192.related_reference().unwrap(), "ORIG987654321");
    }

    #[test]
    fn test_reason_for_cancellation() {
        let mt192 = create_test_mt192();
        assert_eq!(mt192.reason_for_cancellation().unwrap(), "DUPLICATE PAYMENT");
    }

    #[test]
    fn test_original_message_type() {
        let mt192 = create_test_mt192();
        assert_eq!(mt192.original_message_type().unwrap(), "103");
    }

    #[test]
    fn test_requesting_institution() {
        let mt192 = create_test_mt192();
        assert_eq!(mt192.requesting_institution().unwrap(), "REQUESTING BANK\nADDRESS");
    }

    #[test]
    fn test_receiving_institution() {
        let mt192 = create_test_mt192();
        assert_eq!(mt192.receiving_institution().unwrap(), "RECEIVING BANK\nADDRESS");
    }

    #[test]
    fn test_narratives() {
        let mt192 = create_test_mt192();
        let narratives = mt192.narratives();
        assert_eq!(narratives.len(), 2);
        assert_eq!(narratives[0], "URGENT CANCELLATION REQUIRED");
        assert_eq!(narratives[1], "PLEASE CONFIRM RECEIPT");
    }

    #[test]
    fn test_copy_of_original_message() {
        let mt192 = create_test_mt192();
        assert_eq!(mt192.copy_of_original_message().unwrap(), "COPY OF ORIGINAL MT103 MESSAGE...");
    }

    #[test]
    fn test_get_field() {
        let mt192 = create_test_mt192();
        let field = mt192.get_field("20").unwrap();
        assert_eq!(field.value(), "CANCEL123456");
    }

    #[test]
    fn test_get_all_fields() {
        let mt192 = create_test_mt192();
        let fields = mt192.get_all_fields();
        assert_eq!(fields.len(), 9);
    }
} 