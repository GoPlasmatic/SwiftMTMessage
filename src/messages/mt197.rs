//! MT197: Copy of a Message

use serde::{Deserialize, Serialize};
use crate::common::{Field, MessageBlock, tags};
use crate::error::{MTError, Result};
use crate::messages::{extract_text_block, find_field, find_fields, get_required_field_value, get_optional_field_value, MTMessageType};

/// MT197: Copy of a Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT197 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT197 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get related reference (Field 21) - Reference of the original message being copied
    pub fn related_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, "21")
    }

    /// Get copy reason (Field 75) - Reason for sending the copy
    pub fn copy_reason(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "75")
    }

    /// Get original message type (Field 11S) - Type of the original message
    pub fn original_message_type(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "11S")
    }

    /// Get copy of original message (Field 79) - Complete copy of the original message
    pub fn copy_of_original_message(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "79")
    }

    /// Get sending institution (Field 52A) - Institution sending the copy
    pub fn sending_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get receiving institution (Field 58A) - Institution receiving the copy
    pub fn receiving_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "58A")
    }

    /// Get narrative (Field 72) - Additional information about the copy
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

    /// Get copy details (Field 77A) - Additional details about the copy
    pub fn copy_details(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77A")
    }

    /// Get all copy details (Field 77A) - can have multiple
    pub fn all_copy_details(&self) -> Vec<String> {
        find_fields(&self.fields, "77A")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get original sender (Field 50K) - Original sender of the copied message
    pub fn original_sender(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_CUSTOMER)
    }

    /// Get original receiver (Field 59) - Original receiver of the copied message
    pub fn original_receiver(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::BENEFICIARY_CUSTOMER)
    }
}

impl MTMessageType for MT197 {
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

        Ok(MT197 { fields })
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

    fn create_test_mt197() -> MT197 {
        let fields = vec![
            Field::new("20", "COPY123456"),
            Field::new("21", "ORIG987654321"),
            Field::new("11S", "103"),
            Field::new("75", "REGULATORY COPY"),
            Field::new("52A", "SENDING BANK\nADDRESS"),
            Field::new("58A", "RECEIVING BANK\nADDRESS"),
            Field::new("50K", "ORIGINAL SENDER\nCOMPANY"),
            Field::new("59", "ORIGINAL RECEIVER\nCOMPANY"),
            Field::new("72", "COPY FOR COMPLIANCE"),
            Field::new("72", "AUDIT TRAIL"),
            Field::new("77A", "COMPLETE MESSAGE COPY"),
            Field::new("77A", "ALL FIELDS INCLUDED"),
            Field::new("79", "FULL COPY OF ORIGINAL MT103..."),
        ];
        MT197 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.transaction_reference().unwrap(), "COPY123456");
    }

    #[test]
    fn test_related_reference() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.related_reference().unwrap(), "ORIG987654321");
    }

    #[test]
    fn test_copy_reason() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.copy_reason().unwrap(), "REGULATORY COPY");
    }

    #[test]
    fn test_original_message_type() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.original_message_type().unwrap(), "103");
    }

    #[test]
    fn test_sending_institution() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.sending_institution().unwrap(), "SENDING BANK\nADDRESS");
    }

    #[test]
    fn test_receiving_institution() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.receiving_institution().unwrap(), "RECEIVING BANK\nADDRESS");
    }

    #[test]
    fn test_original_sender() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.original_sender().unwrap(), "ORIGINAL SENDER\nCOMPANY");
    }

    #[test]
    fn test_original_receiver() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.original_receiver().unwrap(), "ORIGINAL RECEIVER\nCOMPANY");
    }

    #[test]
    fn test_narratives() {
        let mt197 = create_test_mt197();
        let narratives = mt197.narratives();
        assert_eq!(narratives.len(), 2);
        assert_eq!(narratives[0], "COPY FOR COMPLIANCE");
        assert_eq!(narratives[1], "AUDIT TRAIL");
    }

    #[test]
    fn test_all_copy_details() {
        let mt197 = create_test_mt197();
        let details = mt197.all_copy_details();
        assert_eq!(details.len(), 2);
        assert_eq!(details[0], "COMPLETE MESSAGE COPY");
        assert_eq!(details[1], "ALL FIELDS INCLUDED");
    }

    #[test]
    fn test_copy_of_original_message() {
        let mt197 = create_test_mt197();
        assert_eq!(mt197.copy_of_original_message().unwrap(), "FULL COPY OF ORIGINAL MT103...");
    }

    #[test]
    fn test_get_field() {
        let mt197 = create_test_mt197();
        let field = mt197.get_field("20").unwrap();
        assert_eq!(field.value(), "COPY123456");
    }

    #[test]
    fn test_get_all_fields() {
        let mt197 = create_test_mt197();
        let fields = mt197.get_all_fields();
        assert_eq!(fields.len(), 13);
    }
} 