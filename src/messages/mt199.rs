//! MT199: Free Format Message

use serde::{Deserialize, Serialize};

use crate::common::{Field, MessageBlock, tags};
use crate::error::{MTError, Result};
use crate::messages::{
    MTMessageType, extract_text_block, find_field, find_fields, get_optional_field_value,
    get_required_field_value,
};

/// MT199: Free Format Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT199 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT199 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get related reference (Field 21) - Reference to related message (optional)
    pub fn related_reference(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "21")
    }

    /// Get message subject (Field 75) - Subject or purpose of the free format message
    pub fn message_subject(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "75")
    }

    /// Get free format text (Field 79) - Main content of the message
    pub fn free_format_text(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "79")
    }

    /// Get all free format text fields (Field 79) - can have multiple
    pub fn all_free_format_text(&self) -> Vec<String> {
        find_fields(&self.fields, "79")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get sending institution (Field 52A) - Institution sending the message
    pub fn sending_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get receiving institution (Field 58A) - Institution receiving the message
    pub fn receiving_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "58A")
    }

    /// Get narrative (Field 72) - Additional narrative information
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

    /// Get message details (Field 77A) - Additional message details
    pub fn message_details(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77A")
    }

    /// Get all message details (Field 77A) - can have multiple
    pub fn all_message_details(&self) -> Vec<String> {
        find_fields(&self.fields, "77A")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get priority indicator (Field 11S) - Message priority
    pub fn priority_indicator(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "11S")
    }

    /// Get message category (Field 76) - Category or type of free format message
    pub fn message_category(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "76")
    }

    /// Get all message categories (Field 76) - can have multiple
    pub fn all_message_categories(&self) -> Vec<String> {
        find_fields(&self.fields, "76")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get sender information (Field 50K) - Information about the sender
    pub fn sender_information(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_CUSTOMER)
    }

    /// Get receiver information (Field 59) - Information about the receiver
    pub fn receiver_information(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::BENEFICIARY_CUSTOMER)
    }
}

impl MTMessageType for MT199 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;

        // Validate required fields are present - MT199 only requires field 20
        let required_fields = [
            tags::SENDER_REFERENCE, // Field 20
        ];

        for &field_tag in &required_fields {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        Ok(MT199 { fields })
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

    fn create_test_mt199() -> MT199 {
        let fields = vec![
            Field::new("20", "FREE123456"),
            Field::new("21", "REL987654321"),
            Field::new("11S", "HIGH"),
            Field::new("75", "URGENT NOTIFICATION"),
            Field::new("52A", "SENDING BANK\nADDRESS"),
            Field::new("58A", "RECEIVING BANK\nADDRESS"),
            Field::new("50K", "SENDER COMPANY\nCONTACT INFO"),
            Field::new("59", "RECEIVER COMPANY\nCONTACT INFO"),
            Field::new("72", "IMPORTANT NOTICE"),
            Field::new("72", "PLEASE ACKNOWLEDGE"),
            Field::new("77A", "SYSTEM MAINTENANCE SCHEDULED"),
            Field::new("77A", "DOWNTIME: 2 HOURS"),
            Field::new("76", "OPERATIONAL"),
            Field::new("76", "SYSTEM NOTICE"),
            Field::new("79", "Dear Colleagues,\nWe would like to inform you..."),
            Field::new("79", "Please contact us if you have questions."),
        ];
        MT199 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt199 = create_test_mt199();
        assert_eq!(mt199.transaction_reference().unwrap(), "FREE123456");
    }

    #[test]
    fn test_related_reference() {
        let mt199 = create_test_mt199();
        assert_eq!(mt199.related_reference().unwrap(), "REL987654321");
    }

    #[test]
    fn test_message_subject() {
        let mt199 = create_test_mt199();
        assert_eq!(mt199.message_subject().unwrap(), "URGENT NOTIFICATION");
    }

    #[test]
    fn test_priority_indicator() {
        let mt199 = create_test_mt199();
        assert_eq!(mt199.priority_indicator().unwrap(), "HIGH");
    }

    #[test]
    fn test_sending_institution() {
        let mt199 = create_test_mt199();
        assert_eq!(
            mt199.sending_institution().unwrap(),
            "SENDING BANK\nADDRESS"
        );
    }

    #[test]
    fn test_receiving_institution() {
        let mt199 = create_test_mt199();
        assert_eq!(
            mt199.receiving_institution().unwrap(),
            "RECEIVING BANK\nADDRESS"
        );
    }

    #[test]
    fn test_sender_information() {
        let mt199 = create_test_mt199();
        assert_eq!(
            mt199.sender_information().unwrap(),
            "SENDER COMPANY\nCONTACT INFO"
        );
    }

    #[test]
    fn test_receiver_information() {
        let mt199 = create_test_mt199();
        assert_eq!(
            mt199.receiver_information().unwrap(),
            "RECEIVER COMPANY\nCONTACT INFO"
        );
    }

    #[test]
    fn test_narratives() {
        let mt199 = create_test_mt199();
        let narratives = mt199.narratives();
        assert_eq!(narratives.len(), 2);
        assert_eq!(narratives[0], "IMPORTANT NOTICE");
        assert_eq!(narratives[1], "PLEASE ACKNOWLEDGE");
    }

    #[test]
    fn test_all_message_details() {
        let mt199 = create_test_mt199();
        let details = mt199.all_message_details();
        assert_eq!(details.len(), 2);
        assert_eq!(details[0], "SYSTEM MAINTENANCE SCHEDULED");
        assert_eq!(details[1], "DOWNTIME: 2 HOURS");
    }

    #[test]
    fn test_all_message_categories() {
        let mt199 = create_test_mt199();
        let categories = mt199.all_message_categories();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0], "OPERATIONAL");
        assert_eq!(categories[1], "SYSTEM NOTICE");
    }

    #[test]
    fn test_all_free_format_text() {
        let mt199 = create_test_mt199();
        let texts = mt199.all_free_format_text();
        assert_eq!(texts.len(), 2);
        assert_eq!(texts[0], "Dear Colleagues,\nWe would like to inform you...");
        assert_eq!(texts[1], "Please contact us if you have questions.");
    }

    #[test]
    fn test_get_field() {
        let mt199 = create_test_mt199();
        let field = mt199.get_field("20").unwrap();
        assert_eq!(field.value(), "FREE123456");
    }

    #[test]
    fn test_get_all_fields() {
        let mt199 = create_test_mt199();
        let fields = mt199.get_all_fields();
        assert_eq!(fields.len(), 16);
    }
}
