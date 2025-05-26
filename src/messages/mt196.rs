//! MT196: Answers

use serde::{Deserialize, Serialize};
use crate::common::{Field, MessageBlock, tags};
use crate::error::{MTError, Result};
use crate::messages::{extract_text_block, find_field, find_fields, get_required_field_value, get_optional_field_value, MTMessageType};

/// MT196: Answers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MT196 {
    /// All fields from the text block
    fields: Vec<Field>,
}

impl MT196 {
    /// Get transaction reference number (Field 20)
    pub fn transaction_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, tags::SENDER_REFERENCE)
    }

    /// Get related reference (Field 21) - Reference of the query being answered
    pub fn related_reference(&self) -> Result<String> {
        get_required_field_value(&self.fields, "21")
    }

    /// Get answer type (Field 75) - Type of answer being provided
    pub fn answer_type(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "75")
    }

    /// Get original query message type (Field 11S) - optional
    pub fn original_query_message_type(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "11S")
    }

    /// Get answer details (Field 79) - Details of the answer
    pub fn answer_details(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "79")
    }

    /// Get answering institution (Field 52A) - optional
    pub fn answering_institution(&self) -> Option<String> {
        get_optional_field_value(&self.fields, tags::ORDERING_INSTITUTION)
    }

    /// Get querying institution (Field 58A) - optional
    pub fn querying_institution(&self) -> Option<String> {
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

    /// Get answer details (Field 77A) - optional
    pub fn detailed_answer(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77A")
    }

    /// Get all answer details (Field 77A) - can have multiple
    pub fn all_detailed_answers(&self) -> Vec<String> {
        find_fields(&self.fields, "77A")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get copy of original query (Field 79) - optional
    pub fn copy_of_original_query(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "79")
    }

    /// Get status information (Field 76) - optional
    pub fn status_information(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "76")
    }

    /// Get all status information (Field 76) - can have multiple
    pub fn all_status_information(&self) -> Vec<String> {
        find_fields(&self.fields, "76")
            .into_iter()
            .map(|field| field.value().to_string())
            .collect()
    }

    /// Get confirmation code (Field 77B) - optional
    pub fn confirmation_code(&self) -> Option<String> {
        get_optional_field_value(&self.fields, "77B")
    }
}

impl MTMessageType for MT196 {
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

        Ok(MT196 { fields })
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

    fn create_test_mt196() -> MT196 {
        let fields = vec![
            Field::new("20", "ANSWER123456"),
            Field::new("21", "QUERY987654321"),
            Field::new("11S", "195"),
            Field::new("75", "PAYMENT STATUS RESPONSE"),
            Field::new("52A", "ANSWERING BANK\nADDRESS"),
            Field::new("58A", "QUERYING BANK\nADDRESS"),
            Field::new("72", "RESPONSE TO YOUR INQUIRY"),
            Field::new("72", "PAYMENT CONFIRMED"),
            Field::new("77A", "PAYMENT EXECUTED ON 2021-03-15"),
            Field::new("77A", "BENEFICIARY CREDITED"),
            Field::new("76", "COMPLETED"),
            Field::new("76", "NO ISSUES"),
            Field::new("77B", "CONF123"),
            Field::new("79", "COPY OF ORIGINAL QUERY..."),
        ];
        MT196 { fields }
    }

    #[test]
    fn test_transaction_reference() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.transaction_reference().unwrap(), "ANSWER123456");
    }

    #[test]
    fn test_related_reference() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.related_reference().unwrap(), "QUERY987654321");
    }

    #[test]
    fn test_answer_type() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.answer_type().unwrap(), "PAYMENT STATUS RESPONSE");
    }

    #[test]
    fn test_original_query_message_type() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.original_query_message_type().unwrap(), "195");
    }

    #[test]
    fn test_answering_institution() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.answering_institution().unwrap(), "ANSWERING BANK\nADDRESS");
    }

    #[test]
    fn test_querying_institution() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.querying_institution().unwrap(), "QUERYING BANK\nADDRESS");
    }

    #[test]
    fn test_narratives() {
        let mt196 = create_test_mt196();
        let narratives = mt196.narratives();
        assert_eq!(narratives.len(), 2);
        assert_eq!(narratives[0], "RESPONSE TO YOUR INQUIRY");
        assert_eq!(narratives[1], "PAYMENT CONFIRMED");
    }

    #[test]
    fn test_all_detailed_answers() {
        let mt196 = create_test_mt196();
        let answers = mt196.all_detailed_answers();
        assert_eq!(answers.len(), 2);
        assert_eq!(answers[0], "PAYMENT EXECUTED ON 2021-03-15");
        assert_eq!(answers[1], "BENEFICIARY CREDITED");
    }

    #[test]
    fn test_all_status_information() {
        let mt196 = create_test_mt196();
        let status = mt196.all_status_information();
        assert_eq!(status.len(), 2);
        assert_eq!(status[0], "COMPLETED");
        assert_eq!(status[1], "NO ISSUES");
    }

    #[test]
    fn test_confirmation_code() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.confirmation_code().unwrap(), "CONF123");
    }

    #[test]
    fn test_copy_of_original_query() {
        let mt196 = create_test_mt196();
        assert_eq!(mt196.copy_of_original_query().unwrap(), "COPY OF ORIGINAL QUERY...");
    }

    #[test]
    fn test_get_field() {
        let mt196 = create_test_mt196();
        let field = mt196.get_field("20").unwrap();
        assert_eq!(field.value(), "ANSWER123456");
    }

    #[test]
    fn test_get_all_fields() {
        let mt196 = create_test_mt196();
        let fields = mt196.get_all_fields();
        assert_eq!(fields.len(), 14);
    }
} 