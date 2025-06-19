use crate::SwiftField;
use serde::{Deserialize, Serialize};

/// # Field 71A: Details of Charges
///
/// ## Overview
/// Field 71A specifies who bears the charges in SWIFT payment messages. This field indicates
/// the party responsible for paying the various fees and charges associated with the payment
/// transaction, including correspondent banking fees, intermediary charges, and beneficiary
/// bank charges. The charge allocation is crucial for cost transparency and proper payment
/// processing in international transfers.
///
/// ## Format Specification
/// **Format**: `3!a`
/// - **3!a**: 3 alphabetic characters indicating charge bearer
/// - **Character set**: Alphabetic characters only (A-Z)
/// - **Case**: Typically uppercase
///
/// ## Structure
/// ```text
/// OUR
/// │││
/// └┴┴─ Charge code (3 letters)
/// ```
///
/// ## Field Components
/// - **Charge Code**: Three-letter code indicating charge responsibility
///   - Must be exactly 3 alphabetic characters
///   - Standard codes defined by SWIFT
///   - Case-insensitive but typically uppercase
///
/// ## Usage Context
/// Field 71A is used in:
/// - **MT103**: Single Customer Credit Transfer
/// - **MT200**: Financial Institution Transfer
/// - **MT202**: General Financial Institution Transfer
/// - **MT202COV**: Cover for customer credit transfer
/// - **MT205**: Financial Institution Transfer for its own account
///
/// ### Business Applications
/// - **Cost allocation**: Determining who pays transaction fees
/// - **Pricing transparency**: Clear fee responsibility indication
/// - **Correspondent banking**: Managing charge arrangements
/// - **Customer agreements**: Implementing charge policies
/// - **Regulatory compliance**: Meeting charge disclosure requirements
/// - **Payment processing**: Automated charge handling
///
/// ## Standard Charge Codes
/// ### BEN (Beneficiary)
/// - **Description**: Beneficiary bears all charges
/// - **Usage**: All fees deducted from payment amount
/// - **Impact**: Beneficiary receives net amount after charges
/// - **Common in**: Incoming payments, salary transfers
///
/// ### OUR (Ordering Customer)
/// - **Description**: Ordering customer bears all charges
/// - **Usage**: All fees paid separately by sender
/// - **Impact**: Beneficiary receives full payment amount
/// - **Common in**: Commercial payments, urgent transfers
///
/// ### SHA (Shared)
/// - **Description**: Charges shared between parties
/// - **Usage**: Sender pays sending charges, beneficiary pays receiving charges
/// - **Impact**: Each party pays their respective bank's fees
/// - **Common in**: Standard international transfers
///
/// ## Examples
/// ```text
/// :71A:OUR
/// └─── Ordering customer bears all charges
///
/// :71A:BEN
/// └─── Beneficiary bears all charges
///
/// :71A:SHA
/// └─── Charges shared between parties
/// ```
///
/// ## Charge Code Details
/// - **OUR**: Ordering customer pays all charges
///   - Sender bank charges: Paid by ordering customer
///   - Correspondent charges: Paid by ordering customer
///   - Beneficiary bank charges: Paid by ordering customer
///   - Result: Beneficiary receives full amount
///
/// - **BEN**: Beneficiary pays all charges
///   - Sender bank charges: Deducted from payment
///   - Correspondent charges: Deducted from payment
///   - Beneficiary bank charges: Deducted from payment
///   - Result: Beneficiary receives net amount
///
/// - **SHA**: Shared charge arrangement
///   - Sender bank charges: Paid by ordering customer
///   - Correspondent charges: Typically shared or negotiated
///   - Beneficiary bank charges: Paid by beneficiary
///   - Result: Beneficiary receives amount minus receiving charges
///
/// ## Validation Rules
/// 1. **Length**: Must be exactly 3 characters
/// 2. **Character type**: Alphabetic characters only
/// 3. **Case**: Case-insensitive but normalized to uppercase
/// 4. **Standard codes**: Should use recognized SWIFT codes
/// 5. **Required field**: Must be present in applicable message types
///
/// ## Network Validated Rules (SWIFT Standards)
/// - Must be exactly 3 alphabetic characters (Error: T15)
/// - Must use valid charge code (Error: T71)
/// - Code must be recognized by SWIFT network (Error: T72)
/// - Field is mandatory in applicable messages (Error: M71)
/// - Must be consistent with message type (Error: C71)
///

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
#[format("3!a")]
pub struct Field71A {
    /// Charge details code (3 characters)
    #[format("3!a")]
    pub details_of_charges: String,
}

impl Field71A {
    /// Create a new Field71A with the given charge code
    pub fn new(details_of_charges: &str) -> Result<Self, crate::ParseError> {
        let normalized = details_of_charges.trim().to_uppercase();

        // Validate format: exactly 3 alphabetic characters
        if normalized.len() != 3 {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71A".to_string(),
                message: "Details of charges must be exactly 3 characters".to_string(),
            });
        }

        if !normalized.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(crate::ParseError::InvalidFieldFormat {
                field_tag: "71A".to_string(),
                message: "Details of charges must contain only alphabetic characters".to_string(),
            });
        }

        Ok(Self {
            details_of_charges: normalized,
        })
    }

    /// Create a new Field71A without validation (for internal use)
    pub fn new_unchecked(details_of_charges: String) -> Self {
        Self {
            details_of_charges: details_of_charges.to_uppercase(),
        }
    }

    /// Get the charge code
    pub fn charge_code(&self) -> &str {
        &self.details_of_charges
    }

    /// Check if this is a standard charge code
    pub fn is_standard_code(&self) -> bool {
        matches!(self.details_of_charges.as_str(), "BEN" | "OUR" | "SHA")
    }

    /// Get human-readable description of the charge code
    pub fn description(&self) -> &'static str {
        match self.details_of_charges.as_str() {
            "BEN" => "Beneficiary bears all charges",
            "OUR" => "Ordering customer bears all charges",
            "SHA" => "Charges shared between ordering customer and beneficiary",
            _ => "Unknown charge code",
        }
    }
}

impl std::fmt::Display for Field71A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details_of_charges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field71a_creation() {
        let field = Field71A::new("OUR").unwrap();
        assert_eq!(field.charge_code(), "OUR");
        assert!(field.is_standard_code());
        assert_eq!(field.description(), "Ordering customer bears all charges");
    }

    #[test]
    fn test_field71a_parse() {
        let field = Field71A::parse("SHA").unwrap();
        assert_eq!(field.details_of_charges, "SHA");
    }

    #[test]
    fn test_field71a_case_insensitive() {
        let field = Field71A::new("ben").unwrap();
        assert_eq!(field.details_of_charges, "BEN");
    }
}
