use crate::fields::*;
use crate::{SwiftMessage, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT210: Notice to Receive
///
/// ## Overview
/// MT210 is used by a financial institution to notify another financial institution
/// of an impending debit to the sender's account held with the receiver, or to request
/// the receiver to provide funds to cover the debit. This message serves as advance
/// notice of funds requirements and facilitates liquidity management between institutions.
///
/// ## Message Type Specification
/// **Message Type**: `210`  
/// **Category**: Financial Institution Transfers (Category 2)  
/// **Usage**: Notice to Receive  
/// **Processing**: Notification and funding request  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ## Message Structure
/// The MT210 message consists of mandatory and optional fields organized in a specific sequence:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 21**: Related Reference (related message or transaction reference)
/// - **Field 30**: Value Date (date when funds are required)
/// - **Field 32B**: Currency and Amount (currency and amount required)
///
/// ### Conditional Fields (Business Rules)
/// - **Field 50a**: Ordering Customer (conditional based on rule C2)
/// - **Field 52a**: Ordering Institution (conditional based on rule C2)
///
/// ### Optional Fields (Enhanced Processing)
/// ```text
/// Field 25    - Account Identification (for multiple accounts)
/// Field 56a   - Intermediary Institution (routing bank)
/// ```
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Liquidity management**: Advance notice of funding requirements
/// - **Correspondent banking**: Notice of impending debits to nostro accounts
/// - **Cash management**: Coordination of funds availability
/// - **Settlement preparation**: Pre-funding for settlement obligations
/// - **Clearing house operations**: Notice for clearing house requirements
/// - **Central bank operations**: Reserve requirement notifications
///
/// ### Industry Sectors
/// - **Correspondent Banking**: Managing nostro/vostro account relationships
/// - **Central Banking**: Central bank and commercial bank coordination
/// - **Clearing Houses**: Settlement system funding coordination
/// - **Commercial Banking**: Inter-bank funding and liquidity management
/// - **Investment Banking**: Securities settlement funding
/// - **Corporate Banking**: Large corporate cash management support
///
/// ## Validation Rules and Compliance
///
/// ### Conditional Rules (SWIFT Standards)
/// - **Rule C1**: Message may include up to 10 notice sequences (if repeated)
/// - **Rule C2**: Either Field 50a or Field 52a must be present, not both
/// - **Rule C3**: Currency must be consistent in all Field 32B instances
///
/// ### Business Rule Validations
/// - Transaction reference (Field 20) should be unique per sender per day
/// - Related reference (Field 21) should link to valid previous transaction
/// - Value date (Field 30) should be valid business day for currency
/// - Currency in Field 32B should be actively traded currency
/// - Amount in Field 32B must be positive
/// - BIC codes must be valid and active in SWIFT directory
///
/// ### Special Handling Requirements
/// - **Option F for Field 50a**: Requires structured identity details using numbered lines
/// - **Option D for Fields 52a/56a**: May include national clearing codes prefixed with double slashes
/// - **Commodity restriction**: Commodities like XAU, XAG, XPD, XPT must not be used
///
/// ## Processing and Settlement
/// - Notice messages require acknowledgment but not settlement
/// - Provides advance warning for liquidity planning
/// - May trigger funding arrangements or credit line usage
/// - Should be processed during business hours for same-day notice
/// - May influence overnight funding decisions and positions
///
/// ## Examples
/// ```text
/// Basic MT210 with ordering institution:
/// :20:FT21050100001234
/// :21:NONREF
/// :30:240315
/// :32B:USD1000000,00
/// :52A:CHASUS33XXX
///
/// MT210 with ordering customer:
/// :20:FT21050100001235
/// :21:NONREF
/// :30:240315
/// :32B:EUR500000,00
/// :50A:/12345678901234567890
/// ACME CORPORATION
/// NEW YORK NY US
/// ```
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "210")]
pub struct MT210 {
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific notice to receive.
    /// Used throughout the notice lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT210 messages  
    /// **Business Rule**: Should follow sender's reference numbering scheme  
    /// **Example**: "FT21050100001234"
    #[field("20")]
    pub field_20: Field20,

    /// **Related Reference** - Field 21
    ///
    /// Reference to a related message or transaction that this MT210 notice
    /// is associated with. Links the notice to previous transactions or
    /// standing arrangements.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT210 messages  
    /// **Relationship**: Links to previous messages or transactions  
    /// **Common Value**: "NONREF" when no specific related reference
    #[field("21")]
    pub field_21: Field21,

    /// **Value Date** - Field 30
    ///
    /// Date when the funds are required or when the debit will occur.
    /// Critical for liquidity planning and funding arrangement timing.
    ///
    /// **Format**: YYMMDD (6 numeric characters)  
    /// **Usage**: Mandatory in all MT210 messages  
    /// **Business Rule**: Must be a valid calendar date  
    /// **Timing**: Should provide reasonable advance notice
    #[field("30")]
    pub field_30: Field30,

    /// **Currency and Amount** - Field 32B
    ///
    /// Specifies the currency and amount for which notice is being given.
    /// This represents the funds that will be required or debited.
    ///
    /// **Format**: 3!a15d (currency + amount)  
    /// **Usage**: Mandatory in all MT210 messages  
    /// **Business Rule**: Amount must be positive  
    /// **Restriction**: Commodity currencies (XAU, XAG, XPD, XPT) not allowed
    #[field("32B")]
    pub field_32b: GenericCurrencyAmountField,

    /// **Account Identification** - Field 25 (Optional)
    ///
    /// Identifies the specific account when multiple accounts exist.
    /// Used for precise account identification in multi-account relationships.
    ///
    /// **Format**: Up to 35 alphanumeric characters  
    /// **Usage**: Optional, used when multiple accounts present  
    /// **Purpose**: Disambiguates account in multi-account scenarios
    #[field("25")]
    pub field_25: Option<Field25>,

    /// **Ordering Customer** - Field 50a (Conditional - Rule C2)
    ///
    /// Identifies the customer on whose behalf the notice is being sent.
    /// Must not appear if Field 52a (Ordering Institution) is present.
    ///
    /// **Options**: C (Account), F (Party ID+Name/Address)  
    /// **Usage**: Conditional based on Rule C2  
    /// **Business Rule**: Either Field 50a OR Field 52a must be present, not both  
    /// **Option F**: Requires structured identity details using numbered lines
    #[field("50")]
    pub field_50a: Option<Field50>,

    /// **Ordering Institution** - Field 52a (Conditional - Rule C2)
    ///
    /// Identifies the financial institution that is ordering or requesting
    /// the notice. Must not appear if Field 50a (Ordering Customer) is present.
    ///
    /// **Options**: A (BIC), D (Name/Address with optional clearing codes)  
    /// **Usage**: Conditional based on Rule C2  
    /// **Business Rule**: Either Field 50a OR Field 52a must be present, not both  
    /// **Option D**: May include national clearing codes prefixed with "//"
    #[field("52")]
    pub field_52a: Option<GenericBicField>,

    /// **Intermediary Institution** - Field 56a (Optional)
    ///
    /// Identifies an intermediary bank for the funds transfer or routing.
    /// Used when direct relationships do not exist between institutions.
    ///
    /// **Options**: A (BIC), D (Name/Address with optional clearing codes)  
    /// **Usage**: Optional, facilitates routing through intermediary  
    /// **Purpose**: Enables complex routing scenarios  
    /// **Option D**: May include national clearing codes prefixed with "//"
    #[field("56")]
    pub field_56a: Option<GenericBicField>,
}

impl MT210 {
    /// Creates a new MT210 with minimal required fields
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_21` - Related reference
    /// * `field_30` - Value date
    /// * `field_32b` - Currency and amount
    ///
    /// # Returns
    /// A new MT210 instance with only mandatory fields populated
    ///
    /// # Examples
    /// ```rust
    /// # use swift_mt_message::messages::MT210;
    /// # use swift_mt_message::fields::*;
    /// let field_20 = Field20::new("FT21050100001234".to_string());
    /// let field_21 = Field21::new("NONREF".to_string());
    /// let field_30 = Field30::new("240315");
    /// let field_32b = GenericCurrencyAmountField::new("USD", 1000000.00).unwrap();
    ///
    /// let mt210 = MT210::new(field_20, field_21, field_30, field_32b);
    /// ```
    pub fn new(
        field_20: Field20,
        field_21: Field21,
        field_30: Field30,
        field_32b: GenericCurrencyAmountField,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_30,
            field_32b,
            field_25: None,
            field_50a: None,
            field_52a: None,
            field_56a: None,
        }
    }

    /// Creates a new MT210 with all fields
    ///
    /// # Arguments
    /// * `field_20` - Transaction reference number
    /// * `field_21` - Related reference
    /// * `field_30` - Value date
    /// * `field_32b` - Currency and amount
    /// * `field_25` - Account identification (optional)
    /// * `field_50a` - Ordering customer (conditional)
    /// * `field_52a` - Ordering institution (conditional)
    /// * `field_56a` - Intermediary institution (optional)
    ///
    /// # Returns
    /// A new MT210 instance with all fields populated
    pub fn new_complete(
        field_20: Field20,
        field_21: Field21,
        field_30: Field30,
        field_32b: GenericCurrencyAmountField,
        field_25: Option<Field25>,
        field_50a: Option<Field50>,
        field_52a: Option<GenericBicField>,
        field_56a: Option<GenericBicField>,
    ) -> Self {
        Self {
            field_20,
            field_21,
            field_30,
            field_32b,
            field_25,
            field_50a,
            field_52a,
            field_56a,
        }
    }

    // Accessor methods for key fields

    /// Returns the transaction reference number
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Returns the related reference
    pub fn related_reference(&self) -> &str {
        self.field_21.related_reference()
    }

    /// Returns the value date as a string in YYMMDD format
    pub fn value_date_string(&self) -> String {
        self.field_30.date().to_string()
    }

    /// Returns the currency code
    pub fn currency_code(&self) -> &str {
        self.field_32b.currency()
    }

    /// Returns the amount as decimal
    pub fn amount_decimal(&self) -> f64 {
        self.field_32b.amount()
    }

    /// Returns the account identification if present
    pub fn account_identification(&self) -> Option<&str> {
        self.field_25.as_ref().map(|f| f.authorisation())
    }

    /// Returns the ordering customer information if present
    pub fn ordering_customer(&self) -> Option<&Field50> {
        self.field_50a.as_ref()
    }

    /// Returns the ordering institution BIC if present
    pub fn ordering_institution_bic(&self) -> Option<&str> {
        self.field_52a.as_ref().map(|f| f.bic())
    }

    /// Returns the intermediary institution BIC if present
    pub fn intermediary_institution_bic(&self) -> Option<&str> {
        self.field_56a.as_ref().map(|f| f.bic())
    }

    // Validation methods

    /// Validates Rule C2: Either Field 50a or Field 52a must be present, not both
    ///
    /// # Returns
    /// `true` if exactly one of Field 50a or Field 52a is present, `false` otherwise
    pub fn validate_rule_c2(&self) -> bool {
        match (&self.field_50a, &self.field_52a) {
            (Some(_), None) => true, // Field 50a present, Field 52a absent
            (None, Some(_)) => true, // Field 52a present, Field 50a absent
            _ => false,              // Both present or both absent
        }
    }

    /// Validates that the currency is not a commodity currency
    ///
    /// # Returns
    /// `true` if currency is valid (not XAU, XAG, XPD, XPT), `false` otherwise
    pub fn validate_no_commodity_currency(&self) -> bool {
        let currency = self.currency_code();
        !matches!(currency, "XAU" | "XAG" | "XPD" | "XPT")
    }

    /// Validates the overall message structure
    ///
    /// # Returns
    /// `true` if all validation rules pass, `false` otherwise
    pub fn validate_structure(&self) -> bool {
        self.validate_rule_c2()
            && self.validate_no_commodity_currency()
            && self.amount_decimal() > 0.0
    }

    /// Checks if this is a customer-initiated notice (has Field 50a)
    ///
    /// # Returns
    /// `true` if Field 50a is present, indicating customer-initiated notice
    pub fn is_customer_initiated(&self) -> bool {
        self.field_50a.is_some()
    }

    /// Checks if this is an institution-initiated notice (has Field 52a)
    ///
    /// # Returns
    /// `true` if Field 52a is present, indicating institution-initiated notice
    pub fn is_institution_initiated(&self) -> bool {
        self.field_52a.is_some()
    }

    /// Checks if intermediary routing is involved
    ///
    /// # Returns
    /// `true` if Field 56a is present, indicating intermediary routing
    pub fn has_intermediary_routing(&self) -> bool {
        self.field_56a.is_some()
    }

    /// Returns a formatted description of the notice
    ///
    /// # Returns
    /// A human-readable description of the MT210 notice
    pub fn get_notice_description(&self) -> String {
        let notice_type = if self.is_customer_initiated() {
            "Customer-initiated"
        } else if self.is_institution_initiated() {
            "Institution-initiated"
        } else {
            "Invalid (missing ordering party)"
        };

        let routing = if self.has_intermediary_routing() {
            " with intermediary routing"
        } else {
            ""
        };

        format!(
            "{} notice to receive {} {:.2} on {}{}",
            notice_type,
            self.currency_code(),
            self.amount_decimal(),
            self.value_date_string(),
            routing
        )
    }

    /// Gets all institutions involved in the routing chain
    ///
    /// # Returns
    /// A vector of tuples containing (role, BIC) for all institutions in the notice
    pub fn get_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        if let Some(ordering_inst) = &self.field_52a {
            chain.push(("Ordering Institution", ordering_inst.bic().to_string()));
        }

        if let Some(intermediary) = &self.field_56a {
            chain.push(("Intermediary Institution", intermediary.bic().to_string()));
        }

        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftMessageBody;

    #[test]
    fn test_mt210_creation() {
        let field_20 = Field20::new("FT21050100001234".to_string());
        let field_21 = Field21::new("NONREF".to_string());
        let field_30 = Field30::new("240315");
        let field_32b = GenericCurrencyAmountField::new("USD", 1000000.00).unwrap();

        let mt210 = MT210::new(field_20, field_21, field_30, field_32b);

        assert_eq!(mt210.transaction_reference(), "FT21050100001234");
        assert_eq!(mt210.related_reference(), "NONREF");
        assert_eq!(mt210.currency_code(), "USD");
        assert_eq!(mt210.amount_decimal(), 1000000.00);
    }

    #[test]
    fn test_mt210_message_type() {
        assert_eq!(MT210::message_type(), "210");
    }

    #[test]
    fn test_mt210_rule_c2_validation() {
        let field_20 = Field20::new("FT21050100001234".to_string());
        let field_21 = Field21::new("NONREF".to_string());
        let field_30 = Field30::new("240315");
        let field_32b = GenericCurrencyAmountField::new("USD", 1000000.00).unwrap();

        // Test with ordering customer (should pass C2)
        let field_50a = Some(Field50::K(
            Field50K::new(vec![
                "ACME CORPORATION".to_string(),
                "NEW YORK NY US".to_string(),
            ])
            .unwrap(),
        ));
        let mt210_customer = MT210::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_30.clone(),
            field_32b.clone(),
            None,
            field_50a,
            None,
            None,
        );
        assert!(mt210_customer.validate_rule_c2());
        assert!(mt210_customer.is_customer_initiated());

        // Test with ordering institution (should pass C2)
        let field_52a = Some(GenericBicField::new(None, None, "CHASUS33XXX".to_string()).unwrap());
        let mt210_institution = MT210::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_30.clone(),
            field_32b.clone(),
            None,
            None,
            field_52a,
            None,
        );
        assert!(mt210_institution.validate_rule_c2());
        assert!(mt210_institution.is_institution_initiated());

        // Test with both (should fail C2)
        let field_50a = Some(Field50::K(
            Field50K::new(vec![
                "ACME CORPORATION".to_string(),
                "NEW YORK NY US".to_string(),
            ])
            .unwrap(),
        ));
        let field_52a = Some(GenericBicField::new(None, None, "CHASUS33XXX".to_string()).unwrap());
        let mt210_both = MT210::new_complete(
            field_20.clone(),
            field_21.clone(),
            field_30.clone(),
            field_32b.clone(),
            None,
            field_50a,
            field_52a,
            None,
        );
        assert!(!mt210_both.validate_rule_c2());

        // Test with neither (should fail C2)
        let mt210_neither = MT210::new(field_20, field_21, field_30, field_32b);
        assert!(!mt210_neither.validate_rule_c2());
    }

    #[test]
    fn test_mt210_commodity_currency_validation() {
        let field_20 = Field20::new("FT21050100001234".to_string());
        let field_21 = Field21::new("NONREF".to_string());
        let field_30 = Field30::new("240315");

        // Test with valid currency
        let field_32b_valid = GenericCurrencyAmountField::new("USD", 1000000.00).unwrap();
        let mt210_valid = MT210::new(
            field_20.clone(),
            field_21.clone(),
            field_30.clone(),
            field_32b_valid,
        );
        assert!(mt210_valid.validate_no_commodity_currency());

        // Test with commodity currencies (should fail)
        for commodity in &["XAU", "XAG", "XPD", "XPT"] {
            let field_32b_commodity = GenericCurrencyAmountField::new(*commodity, 1000.00).unwrap();
            let mt210_commodity = MT210::new(
                field_20.clone(),
                field_21.clone(),
                field_30.clone(),
                field_32b_commodity,
            );
            assert!(!mt210_commodity.validate_no_commodity_currency());
        }
    }

    #[test]
    fn test_mt210_routing_chain() {
        let field_20 = Field20::new("FT21050100001234".to_string());
        let field_21 = Field21::new("NONREF".to_string());
        let field_30 = Field30::new("240315");
        let field_32b = GenericCurrencyAmountField::new("USD", 1000000.00).unwrap();

        // Test with intermediary routing
        let field_52a = Some(GenericBicField::new(None, None, "CHASUS33XXX".to_string()).unwrap());
        let field_56a = Some(GenericBicField::new(None, None, "DEUTDEFFXXX".to_string()).unwrap());

        let mt210 = MT210::new_complete(
            field_20, field_21, field_30, field_32b, None, None, field_52a, field_56a,
        );

        assert!(mt210.has_intermediary_routing());

        let routing_chain = mt210.get_routing_chain();
        assert_eq!(routing_chain.len(), 2);
        assert_eq!(routing_chain[0].0, "Ordering Institution");
        assert_eq!(routing_chain[0].1, "CHASUS33XXX");
        assert_eq!(routing_chain[1].0, "Intermediary Institution");
        assert_eq!(routing_chain[1].1, "DEUTDEFFXXX");
    }

    #[test]
    fn test_mt210_notice_description() {
        let field_20 = Field20::new("FT21050100001234".to_string());
        let field_21 = Field21::new("NONREF".to_string());
        let field_30 = Field30::new("240315");
        let field_32b = GenericCurrencyAmountField::new("USD", 1000000.00).unwrap();
        let field_52a = Some(GenericBicField::new(None, None, "CHASUS33XXX".to_string()).unwrap());

        let mt210 = MT210::new_complete(
            field_20, field_21, field_30, field_32b, None, None, field_52a, None,
        );

        let description = mt210.get_notice_description();
        assert!(description.contains("Institution-initiated notice"));
        assert!(description.contains("USD"));
        assert!(description.contains("1000000.00"));
        assert!(description.contains("240315"));
    }
}
