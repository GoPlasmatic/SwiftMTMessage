//! MT195: Queries

use crate::common::{Field, MessageBlock, tags};
use crate::error::{MTError, Result};
use crate::messages::{
    MTMessageType, extract_text_block, find_field, find_fields, get_optional_field_value,
    get_required_field_value,
};
use serde::{Deserialize, Serialize};

/// MT195: Queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT195 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT195 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get related reference (Field 21) - Reference of the message being queried
    pub fn related_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, "21")
    }

    /// Get query type (Field 75) - Type of query being made
    pub fn query_type(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "75")
    }

    /// Get queried message type (Field 11S) - optional
    pub fn queried_message_type(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "11S")
    }

    /// Get query details (Field 79) - Details of the query
    pub fn query_details(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "79")
    }

    /// Get querying institution (Field 52A) - optional
    pub fn querying_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get queried institution (Field 58A) - optional
    pub fn queried_institution(&self) -> Option<String> {
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

    /// Get enquiry details (Field 77A) - optional
    pub fn enquiry_details(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77A")
    }

    /// Get copy of queried message (Field 79) - optional
    pub fn copy_of_queried_message(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "79")
    }

    /// Get all enquiry details (Field 77A) - can have multiple
    pub fn all_enquiry_details(&self) -> Vec<String> {
        find_fields(&self.fields, "77A")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }
}

impl MTMessageType for MT195 {
    fn from_blocks(blocks: Vec<MessageBlock>) -> Result<Self> {
        let fields = extract_text_block(&blocks)?;

        // Validate required fields are present
        let required_fields = [
            tags::SENDER_REFERENCE, // Field 20
            "21",                   // Related reference
        ];

        for &field_tag in &required_fields {
            if !fields.iter().any(|f| f.tag.as_str() == field_tag) {
                return Err(MTError::missing_required_field(field_tag));
            }
        }

        Ok(MT195 { fields })
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

    fn create_test_mt195() -> MT195 {
        let fields = vec![
            Field::new("20", "QUERY123456"),
            Field::new("21", "ORIG987654321"),
            Field::new("11S", "103"),
            Field::new("75", "PAYMENT STATUS INQUIRY"),
            Field::new("52A", "QUERYING BANK\nADDRESS"),
            Field::new("58A", "QUERIED BANK\nADDRESS"),
            Field::new("72", "URGENT RESPONSE REQUIRED"),
            Field::new("72", "CUSTOMER INQUIRY"),
            Field::new("77A", "PLEASE CONFIRM PAYMENT STATUS"),
            Field::new("77A", "PROVIDE EXECUTION DATE"),
            Field::new("79", "COPY OF ORIGINAL MESSAGE..."),
        ];
        MT195 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt195 = create_test_mt195();
        assert_eq!(mt195.transaction_reference().unwrap(), "QUERY123456");
    }

    #[test]
    fn test_related_reference() {
        let mt195 = create_test_mt195();
        assert_eq!(mt195.related_reference().unwrap(), "ORIG987654321");
    }

    #[test]
    fn test_query_type() {
        let mt195 = create_test_mt195();
        assert_eq!(mt195.query_type().unwrap(), "PAYMENT STATUS INQUIRY");
    }

    #[test]
    fn test_queried_message_type() {
        let mt195 = create_test_mt195();
        assert_eq!(mt195.queried_message_type().unwrap(), "103");
    }

    #[test]
    fn test_querying_institution() {
        let mt195 = create_test_mt195();
        assert_eq!(
            mt195.querying_institution().unwrap(),
            "QUERYING BANK\nADDRESS"
        );
    }

    #[test]
    fn test_queried_institution() {
        let mt195 = create_test_mt195();
        assert_eq!(
            mt195.queried_institution().unwrap(),
            "QUERIED BANK\nADDRESS"
        );
    }

    #[test]
    fn test_narratives() {
        let mt195 = create_test_mt195();
        let narratives = mt195.narratives();
        assert_eq!(narratives.len(), 2);
        assert_eq!(narratives[0], "URGENT RESPONSE REQUIRED");
        assert_eq!(narratives[1], "CUSTOMER INQUIRY");
    }

    #[test]
    fn test_all_enquiry_details() {
        let mt195 = create_test_mt195();
        let details = mt195.all_enquiry_details();
        assert_eq!(details.len(), 2);
        assert_eq!(details[0], "PLEASE CONFIRM PAYMENT STATUS");
        assert_eq!(details[1], "PROVIDE EXECUTION DATE");
    }

    #[test]
    fn test_copy_of_queried_message() {
        let mt195 = create_test_mt195();
        assert_eq!(
            mt195.copy_of_queried_message().unwrap(),
            "COPY OF ORIGINAL MESSAGE..."
        );
    }

    #[test]
    fn test_get_field() {
        let mt195 = create_test_mt195();
        let field = mt195.get_field("20").unwrap();
        assert_eq!(field.value(), "QUERY123456");
    }

    #[test]
    fn test_get_all_fields() {
        let mt195 = create_test_mt195();
        let fields = mt195.get_all_fields();
        assert_eq!(fields.len(), 11);
    }
}
