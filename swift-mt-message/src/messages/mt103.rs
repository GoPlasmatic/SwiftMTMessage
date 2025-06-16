use crate::{SwiftMessage, fields::*, swift_serde};
use serde::{Deserialize, Serialize};

/// # MT103: Single Customer Credit Transfer
///
/// ## Overview
/// MT103 is the most widely used SWIFT message type for single customer credit transfers,
/// enabling the secure and standardized transfer of funds between financial institutions
/// on behalf of their customers. This message type facilitates cross-border payments,
/// domestic wire transfers, and serves as the backbone of the global payments infrastructure.
/// The MT103 supports various processing modes including standard processing, STP
/// (Straight Through Processing), and REMIT (structured remittance information).
///
/// ## Message Type Specification
/// **Message Type**: `103`  
/// **Category**: Customer Payments and Cheques (Category 1)  
/// **Usage**: Single Customer Credit Transfer  
/// **Processing**: Real-time gross settlement (RTGS) and net settlement  
/// **Network**: SWIFT FIN (Financial network)  
///
/// ### Message Variants
/// ```text
/// MT103        - Standard single customer credit transfer
/// MT103.STP    - Straight Through Processing variant (automated)
/// MT103.REMIT  - Enhanced remittance information variant
/// MT103.COV    - Cover message for correspondent banking
/// ```
///
/// ## Message Structure
/// The MT103 message consists of mandatory and optional fields organized in a specific sequence:
///
/// ### Mandatory Fields (Core Requirements)
/// - **Field 20**: Transaction Reference Number (sender's unique reference)
/// - **Field 23B**: Bank Operation Code (processing instruction)
/// - **Field 32A**: Value Date/Currency/Amount (settlement details)
/// - **Field 50A/F/K**: Ordering Customer (payment originator)
/// - **Field 59A/No Option**: Beneficiary Customer (payment recipient)
/// - **Field 71A**: Details of Charges (charge allocation)
///
/// ### Optional Fields (Enhanced Processing)
/// ```text
/// Field 13C   - Time Indication (processing timing)
/// Field 23E   - Instruction Code (special instructions)
/// Field 26T   - Transaction Type Code (regulatory reporting)
/// Field 33B   - Currency/Instructed Amount (original amount)
/// Field 36    - Exchange Rate (FX conversion rate)
/// Field 51A   - Sending Institution (message sender)
/// Fields 52A/D - Ordering Institution (customer's bank)
/// Fields 53A/B/D - Sender's Correspondent (correspondent bank)
/// Fields 54A/B/D - Receiver's Correspondent (receiving correspondent)  
/// Fields 55A/B/D - Third Reimbursement Institution (reimbursement bank)
/// Fields 56A/C/D - Intermediary Institution (routing bank)
/// Fields 57A/B/C/D - Account With Institution (beneficiary's bank)
/// Field 70    - Remittance Information (payment details)
/// Fields 71F/G - Charges (sender/receiver charges)
/// Field 72    - Sender to Receiver Information (processing instructions)
/// Field 77B   - Regulatory Reporting (compliance information)
/// Field 77T   - Envelope Contents (MT103.REMIT only)
/// ```
///
/// ## Business Applications
///
/// ### Primary Use Cases
/// - **Cross-border payments**: International wire transfers between countries
/// - **Domestic high-value transfers**: Large-value same-currency transfers
/// - **Trade finance settlements**: Payment for goods and services in international trade
/// - **Treasury operations**: Interbank funding and liquidity management
/// - **Corporate payments**: Business-to-business payment settlements
/// - **Retail banking**: High-value customer-initiated transfers
///
/// ### Industry Sectors
/// - **Banking**: Correspondent banking relationships and customer services
/// - **Corporate Treasury**: Multinational corporation payment processing
/// - **Trade Finance**: Letters of credit and trade settlement
/// - **Investment Banking**: Securities settlement and margin calls
/// - **Insurance**: Claims settlement and premium payments
/// - **Real Estate**: Property purchase and mortgage settlements
///
/// ## Message Variants Deep Dive
///
/// ### MT103 Core (Standard Processing)
/// - Full flexibility in field usage and institutional routing
/// - Manual intervention allowed at any processing stage
/// - Support for all field options and correspondent relationships
/// - Traditional correspondent banking model support
/// - Detailed remittance information in Field 70
///
/// ### MT103.STP (Straight Through Processing)
/// **STP Compliance Requirements:**
/// - All institutional fields (52, 53, 54, 55, 56, 57) must use Option A (BIC only)
/// - Field 51A (Sending Institution) is **prohibited**
/// - Field 23E limited to specific codes: CORT, INTC, SDVA, REPA
/// - Field 72 cannot contain RETN/REJT codes or ERI information
/// - Automated end-to-end processing without manual intervention
/// - Enhanced data quality and faster settlement times
/// - Reduced operational costs and processing errors
///
/// ### MT103.REMIT (Enhanced Remittance)
/// **REMIT Specific Features:**
/// - Field 77T (Envelope Contents) is **mandatory**
/// - Field 70 (Remittance Information) is **prohibited**
/// - Structured remittance data for automated reconciliation
/// - Enhanced invoice and payment matching capabilities
/// - Support for detailed remittance advice
/// - Regulatory compliance for electronic invoicing
///
/// ## Routing and Settlement Patterns
///
/// ### Direct Settlement
/// ```text
/// Ordering Bank → Account With Institution
/// (Fields 52A → 57A)
/// ```
///
/// ### Correspondent Banking Chain
/// ```text
/// Ordering Bank → Sender's Correspondent → Receiver's Correspondent → Account With Institution
/// (Fields 52A → 53A → 54A → 57A)
/// ```
///
/// ### Complex Multi-Institution Routing
/// ```text
/// Ordering Bank → Sender's Correspondent → Third Reimbursement →
/// Intermediary → Receiver's Correspondent → Account With Institution
/// (Fields 52A → 53A → 55A → 56A → 54A → 57A)
/// ```
///
/// ## Field Relationships and Dependencies
///
/// ### Cross-Currency Transactions
/// - **Field 33B** (Instructed Amount) required if different from Field 32A currency
/// - **Field 36** (Exchange Rate) mandatory for currency conversion
/// - Enhanced regulatory reporting may be required in Field 77B
///
/// ### Charge Processing
/// - **Field 71A** (Details of Charges): OUR/BEN/SHA allocation
/// - **Field 71F** (Sender's Charges): Specific sender charge amounts
/// - **Field 71G** (Receiver's Charges): Specific receiver charge amounts
/// - Charge fields must be consistent with Field 71A allocation
///
/// ### Institutional Routing Validation
/// - BIC codes must be valid and active in SWIFT directory
/// - Correspondent relationships must exist between institutions
/// - Account numbers must be valid for the specified institution
/// - Routing must comply with sanctions and compliance rules
///
/// ## Validation Rules and Compliance
///
/// ### Network Validated Rules (SWIFT Standards)
/// - **T27**: BIC codes must be registered and active
/// - **T11**: Date formats must be valid (YYMMDD)
/// - **T40**: Currency codes must be valid ISO 4217
/// - **T43**: Amount formats must follow currency precision rules
/// - **T61**: All characters must be from SWIFT character set
/// - **C32**: Field 32A amount must be positive
/// - **C50**: Ordering customer format must be consistent
/// - **C51**: Field 51A not allowed in MT103.STP
/// - **C59**: Beneficiary customer format must be consistent
/// - **C71**: Charge codes must be valid (OUR/BEN/SHA)
///
/// ### Business Rule Validations
/// - Transaction reference (Field 20) should be unique per day per sender
/// - Value date (Field 32A) should be valid business day for currency
/// - Exchange rate (Field 36) should be reasonable for currency pair
/// - Institutional chain should form valid correspondent relationships
/// - Remittance information should comply with character set restrictions
/// - Field 72 structured codes should follow SWIFT format specifications
///
/// ### Regulatory Compliance
/// - **Anti-Money Laundering (AML)**: Customer identification and screening
/// - **Know Your Customer (KYC)**: Enhanced due diligence requirements
/// - **Sanctions Screening**: OFAC, EU, UN, and local sanctions lists
/// - **Regulatory Reporting**: CTR, SAR, and jurisdiction-specific reports
/// - **Data Privacy**: GDPR, PCI-DSS, and financial privacy regulations
/// - **Cross-Border Reporting**: Balance of payments and statistical reporting
///
/// ## Error Handling and Return Scenarios
///
/// ### Field 72 Structured Codes for Returns/Rejects
/// ```text
/// /RETN/AC01/    - Account identifier incorrect
/// /RETN/AC04/    - Account closed
/// /RETN/AC06/    - Account blocked
/// /RETN/AG01/    - Credit transfer forbidden on this account
/// /RETN/AM05/    - Duplication of payment
/// /RETN/BE05/    - Party in the payment chain unknown
/// /RETN/CURR/    - Incorrect currency
/// /RETN/DT01/    - Invalid date
/// /RETN/RF01/    - Not unique end-to-end reference
/// /REJT/AC01/    - Account identifier incorrect (rejected)
/// /REJT/AC03/    - Account identifier invalid
/// /REJT/AG03/    - Transaction type not supported
/// /REJT/RR01/    - Missing debtor account
/// ```
///
/// ### Processing Status Indicators
/// - **ACSC**: AcceptedSettlementCompleted
/// - **ACSP**: AcceptedSettlementInProcess  
/// - **PDNG**: Pending
/// - **RJCT**: Rejected
/// - **CANC**: Cancelled
/// - **ACWC**: AcceptedWithChange
///
/// ## Code Examples
///
/// ### Basic MT103 Creation
/// ```rust
/// use swift_mt_message::{messages::MT103, fields::*};
/// use chrono::NaiveDate;
///
/// // Create mandatory fields
/// let field_20 = Field20::new("FT21234567890".to_string());
/// let field_23b = Field23B::new("CRED".to_string());
/// let field_32a = Field32A::new(
///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
///     "USD".to_string(),
///     1000000.00
/// );
/// let field_50 = Field50::K(Field50K::new(vec![
///     "ACME CORPORATION".to_string(),
///     "123 BUSINESS AVENUE".to_string(),
///     "NEW YORK NY 10001".to_string()
/// ]).unwrap());
/// let field_59 = Field59::A(Field59A::new(
///     Some("GB33BUKB20201555555555".to_string()),
///     "DEUTDEFF"
/// ).unwrap());
/// let field_71a = Field71A::new("OUR".to_string());
///
/// // Create basic MT103
/// let mt103 = MT103::new(
///     field_20, field_23b, field_32a, field_50, field_59, field_71a
/// );
/// ```
///
/// ### MT103.STP Compliant Message
/// ```rust
/// use swift_mt_message::{messages::MT103, fields::*};
/// use chrono::NaiveDate;
///
/// // Create required mandatory fields
/// let field_20 = Field20::new("FT21034567890123".to_string());
/// let field_23b = Field23B::new("CRED".to_string());
/// let field_32a = Field32A::new(
///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
///     "USD".to_string(),
///     1000000.00
/// );
/// let field_50 = Field50::K(Field50K::new(vec!["ACME CORPORATION".to_string()]).unwrap());
/// let field_59 = Field59::NoOption(Field59Basic::new(vec!["BENEFICIARY NAME".to_string()]).unwrap());
/// let field_71a = Field71A::new("OUR".to_string());
///
/// // STP-compliant MT103 with institutional Option A fields only
/// let mt103_stp = MT103::new_complete(
///     field_20, field_23b, field_32a, field_50, field_59, field_71a,
///     None, // field_13c
///     Some(Field23E::new("INTC", None).unwrap()), // STP-allowed code
///     None, // field_26t
///     None, // field_33b
///     None, // field_36
///     None, // field_51a - NOT allowed in STP
///     Some(Field52A::new(None, None, "CHASUS33XXX").unwrap()),
///     None, // field_52d - NOT allowed in STP
///     Some(Field53A::new(None, None, "DEUTDEFFXXX").unwrap()),
///     None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None
/// );
///
/// assert!(mt103_stp.is_stp_compliant());
/// ```
///
/// ### MT103.REMIT Message
/// ```rust
/// use swift_mt_message::{messages::MT103, fields::*};
/// use chrono::NaiveDate;
///
/// // Create required mandatory fields
/// let field_20 = Field20::new("FT21034567890123".to_string());
/// let field_23b = Field23B::new("CRED".to_string());
/// let field_32a = Field32A::new(
///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
///     "USD".to_string(),
///     1000000.00
/// );
/// let field_50 = Field50::K(Field50K::new(vec!["ACME CORPORATION".to_string()]).unwrap());
/// let field_59 = Field59::NoOption(Field59Basic::new(vec!["BENEFICIARY NAME".to_string()]).unwrap());
/// let field_71a = Field71A::new("OUR".to_string());
///
/// // MT103.REMIT with structured remittance data
/// let field_77t = Field77T::new("R", "D", "REMITTANCE-2024-001").unwrap();
///
/// let mt103_remit = MT103::new_complete(
///     field_20, field_23b, field_32a, field_50, field_59, field_71a,
///     None, None, None, None, None, None, None, None, None, None, None,
///     None, None, None, None, None, None, None, None, None, None, None,
///     None, None,
///     None, // field_70 - NOT allowed in REMIT
///     None, None, None, None,
///     Some(field_77t) // field_77t - MANDATORY in REMIT
/// );
///
/// assert!(mt103_remit.is_remit());
/// assert!(mt103_remit.is_remit_compliant());
/// ```
///
/// ### Cross-Currency Transaction
/// ```rust
/// use swift_mt_message::{messages::MT103, fields::*};
/// use chrono::NaiveDate;
///
/// // Create required mandatory fields
/// let field_20 = Field20::new("FT21034567890123".to_string());
/// let field_23b = Field23B::new("CRED".to_string());
/// let field_50 = Field50::K(Field50K::new(vec!["ACME CORPORATION".to_string()]).unwrap());
/// let field_59 = Field59::NoOption(Field59Basic::new(vec!["BENEFICIARY NAME".to_string()]).unwrap());
/// let field_71a = Field71A::new("OUR".to_string());
///
/// // EUR to USD conversion with exchange rate
/// let field_32a = Field32A::new(
///     NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
///     "USD".to_string(),
///     1000000.00 // Settlement amount in USD
/// );
/// let field_33b = Field33B::new("EUR", 850000.00).unwrap(); // Original EUR amount
/// let field_36 = Field36::new(1.1765).unwrap(); // EUR/USD rate
///
/// let mt103_fx = MT103::new_complete(
///     field_20, field_23b, field_32a, field_50, field_59, field_71a,
///     None, None, None,
///     Some(field_33b), // Original instructed amount
///     Some(field_36),  // Exchange rate
///     None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None
/// );
///
/// assert!(mt103_fx.is_cross_currency());
/// assert!(mt103_fx.has_required_exchange_rate());
/// ```
///
///
/// Complete implementation with all possible MT103 fields for 100% compliance
/// Supports STP (Straight Through Processing), RETN (Return), and REJT (Reject) scenarios
///
/// Uses normalized field tags (without option letters) for flexibility while supporting
/// all possible option variations through enum types for institutional fields.
#[swift_serde]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[swift_message(mt = "103")]
pub struct MT103 {
    // ================================
    // MANDATORY FIELDS (Required for all MT103 messages)
    // ================================
    /// **Transaction Reference Number** - Field 20
    ///
    /// Unique sender's reference identifying this specific payment transaction.
    /// Used throughout the payment lifecycle for tracking, reconciliation, and audit.
    /// Must be unique within the sender's system per business day.
    ///
    /// **Format**: Up to 16 alphanumeric characters  
    /// **Usage**: Mandatory in all MT103 variants  
    /// **Business Rule**: Should follow sender's reference numbering scheme
    #[field("20")]
    pub field_20: Field20,

    /// **Bank Operation Code** - Field 23B
    ///
    /// Specifies the type of banking operation being performed.
    /// Determines processing rules, routing behavior, and regulatory requirements.
    ///
    /// **Format**: Exactly 4 alphabetic characters  
    /// **Common Values**: CRED (Credit Transfer), CRTS (Credit Transfer Same Day)  
    /// **STP Impact**: Affects automated processing eligibility  
    /// **Usage**: Mandatory in all MT103 variants
    #[field("23B")]
    pub field_23b: Field23B,

    /// **Value Date/Currency/Amount** - Field 32A
    ///
    /// Core settlement details specifying when, in what currency, and for what amount
    /// the transaction should be processed. The value date determines when funds
    /// become available to the beneficiary.
    ///
    /// **Components**: Date (YYMMDD) + Currency (3 chars) + Amount (decimal)  
    /// **Business Rule**: Value date should be valid business day for currency  
    /// **Settlement**: Determines actual settlement timing  
    /// **Usage**: Mandatory in all MT103 variants
    #[field("32A")]
    pub field_32a: Field32A,

    /// **Ordering Customer** - Field 50A/F/K
    ///
    /// Identifies the party that orders the credit transfer (payment originator).
    /// This is typically the customer on whose behalf the payment is being made.
    /// Different options provide varying levels of detail and identification methods.
    ///
    /// **Options**: A (Account+BIC), F (Party ID+Name/Address), K (Name/Address only)  
    /// **KYC Impact**: Critical for customer identification and compliance  
    /// **AML Requirement**: Must contain sufficient information for screening  
    /// **Usage**: Mandatory in all MT103 variants
    #[field("50")]
    pub field_50: Field50,

    /// **Beneficiary Customer** - Field 59A/No Option
    ///
    /// Identifies the ultimate recipient of the credit transfer.
    /// This is the final beneficiary who will receive the funds being transferred.
    /// Proper identification is crucial for successful payment delivery.
    ///
    /// **Options**: A (Account+BIC), No Option (Account+Name/Address)  
    /// **Delivery**: Critical for successful payment completion  
    /// **Compliance**: Subject to sanctions screening and validation  
    /// **Usage**: Mandatory in all MT103 variants
    #[field("59")]
    pub field_59: Field59,

    /// **Details of Charges** - Field 71A
    ///
    /// Specifies how transaction charges should be allocated between the
    /// ordering customer, beneficiary customer, and any intermediary banks.
    ///
    /// **Values**: OUR (sender pays), BEN (receiver pays), SHA (shared)  
    /// **Impact**: Affects final amount received by beneficiary  
    /// **Regulatory**: May be restricted in certain jurisdictions  
    /// **Usage**: Mandatory in all MT103 variants
    #[field("71A")]
    pub field_71a: Field71A,

    // ================================
    // OPTIONAL FIELDS (Enhanced processing and compliance support)
    // Complete implementation for 100% MT103 compliance including STP/RETN/REJT scenarios
    // ================================
    /// **Time Indication** - Field 13C (Optional)
    ///
    /// Provides specific timing instructions for payment processing,
    /// including target execution times and UTC offset information.
    ///
    /// **Usage**: Optional, used for time-sensitive payments  
    /// **STP Impact**: May affect automated processing timing  
    /// **Format**: Time + UTC offsets for coordination  
    /// **Business Value**: Enables precise timing control
    #[field("13C")]
    pub field_13c: Option<Field13C>,

    /// **Instruction Code** - Field 23E (Optional)
    ///
    /// Specifies special processing instructions for the transaction,
    /// such as communication requirements or handling procedures.
    ///
    /// **STP Restriction**: Limited to CORT, INTC, SDVA, REPA in STP messages  
    /// **Usage**: Optional, affects processing workflow  
    /// **Common Codes**: INTC (intracompany), HOLD (hold payment), PHON (phone)  
    /// **Processing Impact**: May require manual intervention
    #[field("23E")]
    pub field_23e: Option<Field23E>,

    /// **Transaction Type Code** - Field 26T (Optional)
    ///
    /// Regulatory reporting code for balance of payments and statistical purposes.
    /// Required in certain jurisdictions for cross-border payments reporting.
    ///
    /// **Format**: 3-character alphanumeric code  
    /// **Regulatory**: Required for BoP reporting in many jurisdictions  
    /// **Categories**: A-series (goods), B-series (services), F-series (financial)  
    /// **Compliance**: Critical for regulatory compliance
    #[field("26T")]
    pub field_26t: Option<Field26T>,

    /// **Currency/Instructed Amount** - Field 33B (Optional)
    ///
    /// Original amount and currency as instructed by the ordering customer,
    /// before any currency conversion or charge deduction. Used in FX transactions.
    ///
    /// **FX Requirement**: Mandatory for cross-currency transactions  
    /// **Usage**: Shows original instruction before conversion  
    /// **Audit Trail**: Provides complete transaction history  
    /// **Relationship**: Must differ from Field 32A for FX transactions
    #[field("33B")]
    pub field_33b: Option<Field33B>,

    /// **Exchange Rate** - Field 36 (Optional)
    ///
    /// Exchange rate applied for currency conversion between the instructed
    /// amount (Field 33B) and settlement amount (Field 32A).
    ///
    /// **FX Requirement**: Mandatory when Field 33B present with different currency  
    /// **Format**: Up to 12 digits with 5 decimal places maximum  
    /// **Business Rule**: Rate should be reasonable for currency pair  
    /// **Compliance**: Subject to regulatory FX reporting requirements
    #[field("36")]
    pub field_36: Option<Field36>,

    /// **Sending Institution** - Field 51A (Optional)
    ///
    /// Identifies the financial institution sending the message.
    /// Used in correspondent banking to identify the actual message originator.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Usage**: Optional in standard MT103, forbidden in STP  
    /// **Format**: BIC code with optional account information  
    /// **Correspondent Banking**: Critical for routing and settlement
    #[field("51A")]
    pub field_51a: Option<Field51A>,

    // ================================
    // INSTITUTIONAL ROUTING FIELDS (Payment routing chain)
    // These fields define the correspondent banking chain and routing path
    // ================================
    /// **Ordering Institution** - Field 52A (Optional)
    ///
    /// Identifies the financial institution of the ordering customer.
    /// This is the bank where the ordering customer holds their account
    /// and from which the payment originates.
    ///
    /// **STP Requirement**: Only Option A (BIC-based) allowed in STP messages  
    /// **Format**: Optional account indicator + optional account + BIC  
    /// **Routing Role**: First institution in the correspondent banking chain  
    /// **Business Rule**: Must have correspondent relationship with next institution
    #[field("52A")]
    pub field_52a: Option<Field52A>,

    /// **Ordering Institution** - Field 52D (Optional)
    ///
    /// Alternative format for ordering institution using name and address
    /// instead of BIC code. Provides more detailed institutional information
    /// but requires manual processing.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Up to 4 lines of name and address (35 chars each)  
    /// **Processing**: Requires manual intervention for routing  
    /// **Usage**: Used when BIC is not available or for domestic routing
    #[field("52D")]
    pub field_52d: Option<Field52D>,

    /// **Sender's Correspondent** - Field 53A (Optional)
    ///
    /// Identifies the correspondent bank of the sending institution.
    /// Acts as an intermediary in the correspondent banking relationship
    /// between the sender and receiver.
    ///
    /// **STP Requirement**: Only Option A (BIC-based) allowed in STP messages  
    /// **Format**: Optional account indicator + optional account + BIC  
    /// **Routing Role**: Facilitates cross-border payment routing  
    /// **Relationship**: Must have correspondent agreements with sender and receiver
    #[field("53A")]
    pub field_53a: Option<Field53A>,

    /// **Sender's Correspondent** - Field 53B (Optional)
    ///
    /// Alternative format using party identifier instead of BIC.
    /// Allows identification through clearing codes or other party identifiers.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Optional account + party identifier (up to 35 chars)  
    /// **Processing**: May require manual routing decisions  
    /// **Usage**: Domestic clearing systems or non-BIC routing
    #[field("53B")]
    pub field_53b: Option<Field53B>,

    /// **Sender's Correspondent** - Field 53D (Optional)
    ///
    /// Name and address format for sender's correspondent when BIC
    /// or party identifier is not available or sufficient.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Up to 4 lines of name and address (35 chars each)  
    /// **Processing**: Requires manual intervention for institution identification  
    /// **Usage**: Legacy systems or domestic correspondent relationships
    #[field("53D")]
    pub field_53d: Option<Field53D>,

    /// **Receiver's Correspondent** - Field 54A (Optional)
    ///
    /// Identifies the correspondent bank of the receiving institution.
    /// Critical for cross-border payments where direct correspondent
    /// relationships may not exist.
    ///
    /// **STP Requirement**: Only Option A (BIC-based) allowed in STP messages  
    /// **Format**: Optional account indicator + optional account + BIC  
    /// **Routing Role**: Final correspondent before beneficiary institution  
    /// **Settlement**: Often handles final settlement to beneficiary bank
    #[field("54A")]
    pub field_54a: Option<Field54A>,

    /// **Receiver's Correspondent** - Field 54B (Optional)
    ///
    /// Alternative format using party identifier for receiver's correspondent.
    /// Enables routing through domestic clearing systems.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Optional account + party identifier (up to 35 chars)  
    /// **Processing**: May require manual routing and settlement decisions  
    /// **Clearing**: Often used for domestic ACH or clearing system routing
    #[field("54B")]
    pub field_54b: Option<Field54B>,

    /// **Receiver's Correspondent** - Field 54D (Optional)
    ///
    /// Name and address format for receiver's correspondent institution.
    /// Used when standard institutional identification is insufficient.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Up to 4 lines of name and address (35 chars each)  
    /// **Processing**: Requires manual intervention for routing  
    /// **Usage**: Complex correspondent relationships or legacy systems
    #[field("54D")]
    pub field_54d: Option<Field54D>,

    /// **Third Reimbursement Institution** - Field 55A (Optional)
    ///
    /// Identifies an additional reimbursement bank in complex correspondent
    /// banking arrangements. Used for multi-hop correspondent relationships.
    ///
    /// **STP Requirement**: Only Option A (BIC-based) allowed in STP messages  
    /// **Format**: Optional account indicator + optional account + BIC  
    /// **Routing Role**: Intermediate reimbursement in correspondent chain  
    /// **Usage**: Complex cross-border routing with multiple correspondents
    #[field("55A")]
    pub field_55a: Option<Field55A>,

    /// **Third Reimbursement Institution** - Field 55B (Optional)
    ///
    /// Alternative format for third reimbursement institution using
    /// party identifier instead of BIC code.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Optional account + party identifier (up to 35 chars)  
    /// **Processing**: Requires manual reimbursement processing  
    /// **Complexity**: Adds additional correspondent relationship complexity
    #[field("55B")]
    pub field_55b: Option<Field55B>,

    /// **Third Reimbursement Institution** - Field 55D (Optional)
    ///
    /// Name and address format for third reimbursement institution.
    /// Provides detailed institutional information for complex routing.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Up to 4 lines of name and address (35 chars each)  
    /// **Processing**: Requires manual intervention for reimbursement  
    /// **Usage**: Highly complex correspondent banking arrangements
    #[field("55D")]
    pub field_55d: Option<Field55D>,

    /// **Intermediary Institution** - Field 56A (Optional)
    ///
    /// Identifies an intermediary bank in the payment routing chain.
    /// Acts as a pass-through institution between correspondents.
    ///
    /// **STP Requirement**: Only Option A (BIC-based) allowed in STP messages  
    /// **Format**: Optional account indicator + optional account + BIC  
    /// **Routing Role**: Facilitates payment routing between correspondents
    #[field("56A")]
    pub field_56a: Option<Field56A>,

    /// **Intermediary Institution** - Field 56C (Optional)
    ///
    /// Account-based identification for intermediary institution.
    /// Specifies the account at the intermediary for routing purposes.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Account number (up to 34 characters)  
    /// **Processing**: Requires account-based routing decisions  
    /// **Usage**: Specific account routing through intermediary banks
    #[field("56C")]
    pub field_56c: Option<Field56C>,

    /// **Intermediary Institution** - Field 56D (Optional)
    ///
    /// Name and address format for intermediary institution when
    /// other identification methods are insufficient.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Up to 4 lines of name and address (35 chars each)  
    /// **Processing**: Requires manual intervention for routing  
    /// **Complexity**: Adds manual processing overhead to payment chain
    #[field("56D")]
    pub field_56d: Option<Field56D>,

    /// **Account With Institution** - Field 57A (Optional)
    ///
    /// Identifies the financial institution where the beneficiary
    /// customer's account is held. This is the final destination bank.
    ///
    /// **STP Requirement**: Only Option A (BIC-based) allowed in STP messages  
    /// **Format**: Optional account indicator + optional account + BIC  
    /// **Routing Role**: Final destination bank for payment settlement  
    /// **Critical**: Essential for successful payment delivery
    #[field("57A")]
    pub field_57a: Option<Field57A>,

    /// **Account With Institution** - Field 57B (Optional)
    ///
    /// Alternative format using party identifier for the beneficiary's bank.
    /// Enables routing through domestic clearing systems to final destination.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Optional account + party identifier (up to 35 chars)  
    /// **Processing**: May require manual intervention for final delivery  
    /// **Usage**: Domestic clearing systems or non-SWIFT final settlement
    #[field("57B")]
    pub field_57b: Option<Field57B>,

    /// **Account With Institution** - Field 57C (Optional)
    ///
    /// Account-based identification for the beneficiary's bank.
    /// Specifies the specific account for final settlement.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Account number (up to 34 characters)  
    /// **Processing**: Requires account-based final settlement  
    /// **Usage**: Specific account routing for final payment delivery
    #[field("57C")]
    pub field_57c: Option<Field57C>,

    /// **Account With Institution** - Field 57D (Optional)
    ///
    /// Name and address format for the beneficiary's bank when
    /// standard identification methods are not available or sufficient.
    ///
    /// **STP Restriction**: **PROHIBITED** in MT103.STP messages  
    /// **Format**: Up to 4 lines of name and address (35 chars each)  
    /// **Processing**: Requires manual intervention for final delivery  
    /// **Risk**: Higher risk of payment delays or delivery failures
    #[field("57D")]
    pub field_57d: Option<Field57D>,

    // ================================
    // PAYMENT DETAILS AND INFORMATION FIELDS
    // Additional payment information, charges, and processing instructions
    // ================================
    /// **Remittance Information** - Field 70 (Optional)
    ///
    /// Free-format text providing details about the purpose of the payment,
    /// invoice references, and other remittance information for the beneficiary.
    ///
    /// **REMIT Restriction**: **PROHIBITED** in MT103.REMIT messages (use Field 77T)  
    /// **Format**: Up to 4 lines of 35 characters each  
    /// **Purpose**: Payment description, invoice numbers, contract references  
    /// **Reconciliation**: Critical for beneficiary payment matching and reconciliation
    #[field("70")]
    pub field_70: Option<Field70>,

    // ================================
    // CHARGE AND FEE FIELDS
    // Detailed charge information beyond the basic charge allocation
    // ================================
    /// **Sender's Charges** - Field 71F (Optional)
    ///
    /// Specifies the exact amount of charges to be borne by the ordering customer.
    /// Provides transparency in charge allocation and enables precise cost calculation.
    ///
    /// **Format**: Currency code (3 chars) + Amount (decimal)  
    /// **Relationship**: Must be consistent with Field 71A charge allocation  
    /// **Transparency**: Enables clear communication of sender charge amounts  
    /// **Accounting**: Critical for accurate charge accounting and reconciliation
    #[field("71F")]
    pub field_71f: Option<Field71F>,

    /// **Receiver's Charges** - Field 71G (Optional)
    ///
    /// Specifies the exact amount of charges to be borne by the beneficiary customer.
    /// Enables transparent communication of costs that will be deducted from payment.
    ///
    /// **Format**: Currency code (3 chars) + Amount (decimal)  
    /// **Relationship**: Must be consistent with Field 71A charge allocation  
    /// **Impact**: Amount will be deducted from final payment to beneficiary  
    /// **Disclosure**: May be required for regulatory compliance and transparency
    #[field("71G")]
    pub field_71g: Option<Field71G>,

    // ================================
    // PROCESSING AND COMPLIANCE FIELDS
    // Special processing instructions and regulatory compliance information
    // ================================
    /// **Sender to Receiver Information** - Field 72 (Optional)
    ///
    /// Critical field for processing instructions, return/reject codes, and
    /// regulatory information. Essential for STP processing and exception handling.
    ///
    /// **STP Critical**: Contains structured codes for automated processing  
    /// **Return/Reject**: Used for RETN/REJT codes and error communication  
    /// **Format**: Up to 6 lines of 35 characters each  
    /// **Structured Codes**: /CODE/subcod/narrative format for automation  
    /// **Compliance**: May contain regulatory reporting codes and instructions
    #[field("72")]
    pub field_72: Option<Field72>,

    /// **Regulatory Reporting** - Field 77B (Optional)
    ///
    /// Contains regulatory information required for compliance with local
    /// and international reporting requirements, particularly for cross-border payments.
    ///
    /// **BoP Reporting**: Balance of payments statistical reporting  
    /// **Format**: Up to 3 lines of 35 characters each  
    /// **Compliance**: Required for certain jurisdictions and payment types  
    /// **Content**: May include country codes, purpose codes, and regulatory references
    #[field("77B")]
    pub field_77b: Option<Field77B>,

    /// **Envelope Contents** - Field 77T (Optional)
    ///
    /// **MT103.REMIT ONLY**: Contains structured remittance data envelope information.
    /// Mandatory for MT103.REMIT messages, prohibited in standard MT103.
    ///
    /// **REMIT Requirement**: **MANDATORY** in MT103.REMIT messages  
    /// **Standard Restriction**: **PROHIBITED** in standard MT103 messages  
    /// **Format**: Envelope type + format + identifier  
    /// **Structured Data**: References external structured remittance information  
    /// **Automation**: Enables automated invoice matching and reconciliation
    #[field("77T")]
    pub field_77t: Option<Field77T>,
}

impl MT103 {
    /// Create a new MT103 with required fields only
    pub fn new(
        field_20: Field20,
        field_23b: Field23B,
        field_32a: Field32A,
        field_50: Field50,
        field_59: Field59,
        field_71a: Field71A,
    ) -> Self {
        Self {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c: None,
            field_23e: None,
            field_26t: None,
            field_33b: None,
            field_36: None,
            field_51a: None,
            field_52a: None,
            field_52d: None,
            field_53a: None,
            field_53b: None,
            field_53d: None,
            field_54a: None,
            field_54b: None,
            field_54d: None,
            field_55a: None,
            field_55b: None,
            field_55d: None,
            field_56a: None,
            field_56c: None,
            field_56d: None,
            field_57a: None,
            field_57b: None,
            field_57c: None,
            field_57d: None,
            field_70: None,
            field_71f: None,
            field_71g: None,
            field_72: None,
            field_77b: None,
            field_77t: None,
        }
    }

    /// Create a new MT103 with all fields for complete STP/RETN/REJT support
    #[allow(clippy::too_many_arguments)]
    pub fn new_complete(
        field_20: Field20,
        field_23b: Field23B,
        field_32a: Field32A,
        field_50: Field50,
        field_59: Field59,
        field_71a: Field71A,
        field_13c: Option<Field13C>,
        field_23e: Option<Field23E>,
        field_26t: Option<Field26T>,
        field_33b: Option<Field33B>,
        field_36: Option<Field36>,
        field_51a: Option<Field51A>,
        field_52a: Option<Field52A>,
        field_52d: Option<Field52D>,
        field_53a: Option<Field53A>,
        field_53b: Option<Field53B>,
        field_53d: Option<Field53D>,
        field_54a: Option<Field54A>,
        field_54b: Option<Field54B>,
        field_54d: Option<Field54D>,
        field_55a: Option<Field55A>,
        field_55b: Option<Field55B>,
        field_55d: Option<Field55D>,
        field_56a: Option<Field56A>,
        field_56c: Option<Field56C>,
        field_56d: Option<Field56D>,
        field_57a: Option<Field57A>,
        field_57b: Option<Field57B>,
        field_57c: Option<Field57C>,
        field_57d: Option<Field57D>,
        field_70: Option<Field70>,
        field_71f: Option<Field71F>,
        field_71g: Option<Field71G>,
        field_72: Option<Field72>,
        field_77b: Option<Field77B>,
        field_77t: Option<Field77T>,
    ) -> Self {
        Self {
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            field_13c,
            field_23e,
            field_26t,
            field_33b,
            field_36,
            field_51a,
            field_52a,
            field_52d,
            field_53a,
            field_53b,
            field_53d,
            field_54a,
            field_54b,
            field_54d,
            field_55a,
            field_55b,
            field_55d,
            field_56a,
            field_56c,
            field_56d,
            field_57a,
            field_57b,
            field_57c,
            field_57d,
            field_70,
            field_71f,
            field_71g,
            field_72,
            field_77b,
            field_77t,
        }
    }

    /// Get the transaction reference
    pub fn transaction_reference(&self) -> &str {
        self.field_20.transaction_reference()
    }

    /// Get the operation code
    pub fn operation_code(&self) -> &str {
        self.field_23b.operation_code()
    }

    /// Get the currency code
    pub fn currency_code(&self) -> &str {
        self.field_32a.currency_code()
    }

    /// Get the transaction amount as decimal
    pub fn amount_decimal(&self) -> f64 {
        self.field_32a.amount_decimal()
    }

    /// Get the charge code
    pub fn charge_code(&self) -> &str {
        self.field_71a.charge_code()
    }

    /// Get the instructed amount if present
    pub fn instructed_amount(&self) -> Option<&Field33B> {
        self.field_33b.as_ref()
    }

    /// Get the exchange rate if present
    pub fn exchange_rate(&self) -> Option<&Field36> {
        self.field_36.as_ref()
    }

    /// Get sender's charges if present
    pub fn senders_charges(&self) -> Option<&Field71F> {
        self.field_71f.as_ref()
    }

    /// Get receiver's charges if present
    pub fn receivers_charges(&self) -> Option<&Field71G> {
        self.field_71g.as_ref()
    }

    /// Get sender to receiver information if present (critical for STP/RETN/REJT)
    pub fn sender_to_receiver_info(&self) -> Option<&Field72> {
        self.field_72.as_ref()
    }

    /// Get regulatory reporting if present
    pub fn regulatory_reporting(&self) -> Option<&Field77B> {
        self.field_77b.as_ref()
    }

    /// Get sending institution if present
    pub fn sending_institution(&self) -> Option<&Field51A> {
        self.field_51a.as_ref()
    }

    /// Get envelope contents if present (MT103.REMIT only)
    pub fn envelope_contents(&self) -> Option<&Field77T> {
        self.field_77t.as_ref()
    }

    /// Get ordering institution (any option) if present
    pub fn ordering_institution(&self) -> Option<String> {
        if let Some(field_52a) = &self.field_52a {
            Some(field_52a.bic().to_string())
        } else {
            self.field_52d
                .as_ref()
                .map(|field_52d| field_52d.name_and_address().join(" "))
        }
    }

    /// Get sender's correspondent (any option) if present
    pub fn senders_correspondent(&self) -> Option<String> {
        if let Some(field_53a) = &self.field_53a {
            Some(field_53a.bic().to_string())
        } else {
            self.field_53b
                .as_ref()
                .map(|field_53b| field_53b.party_identifier().to_string())
                .or_else(|| {
                    self.field_53d
                        .as_ref()
                        .map(|field_53d| field_53d.name_and_address().join(" "))
                })
        }
    }

    /// Get receiver's correspondent (any option) if present
    pub fn receivers_correspondent(&self) -> Option<String> {
        if let Some(field_54a) = &self.field_54a {
            Some(field_54a.bic().to_string())
        } else {
            self.field_54b
                .as_ref()
                .map(|field_54b| field_54b.party_identifier().to_string())
                .or_else(|| {
                    self.field_54d
                        .as_ref()
                        .map(|field_54d| field_54d.name_and_address().join(" "))
                })
        }
    }

    /// Get third reimbursement institution (any option) if present
    pub fn third_reimbursement_institution(&self) -> Option<String> {
        if let Some(field_55a) = &self.field_55a {
            Some(field_55a.bic().to_string())
        } else {
            self.field_55b
                .as_ref()
                .map(|field_55b| field_55b.party_identifier().to_string())
                .or_else(|| {
                    self.field_55d
                        .as_ref()
                        .map(|field_55d| field_55d.name_and_address().join(" "))
                })
        }
    }

    /// Get intermediary institution (any option) if present
    pub fn intermediary_institution(&self) -> Option<String> {
        if let Some(field_56a) = &self.field_56a {
            Some(field_56a.bic().to_string())
        } else {
            self.field_56c
                .as_ref()
                .map(|field_56c| field_56c.account_number().to_string())
                .or_else(|| {
                    self.field_56d
                        .as_ref()
                        .map(|field_56d| field_56d.name_and_address().join(" "))
                })
        }
    }

    /// Get account with institution (any option) if present
    pub fn account_with_institution(&self) -> Option<String> {
        if let Some(field_57a) = &self.field_57a {
            Some(field_57a.bic().to_string())
        } else {
            self.field_57b
                .as_ref()
                .map(|field_57b| field_57b.party_identifier().to_string())
                .or_else(|| {
                    self.field_57c
                        .as_ref()
                        .map(|field_57c| field_57c.account_number().to_string())
                })
                .or_else(|| {
                    self.field_57d
                        .as_ref()
                        .map(|field_57d| field_57d.lines.join(" "))
                })
        }
    }

    /// Get remittance information if present
    pub fn remittance_information(&self) -> Option<&Field70> {
        self.field_70.as_ref()
    }

    /// Get time indication if present
    pub fn time_indication(&self) -> Option<&Field13C> {
        self.field_13c.as_ref()
    }

    /// Get instruction code if present
    pub fn instruction_code(&self) -> Option<&Field23E> {
        self.field_23e.as_ref()
    }

    /// Get transaction type code if present
    pub fn transaction_type_code(&self) -> Option<&Field26T> {
        self.field_26t.as_ref()
    }

    /// Check if all required fields are present and valid
    pub fn validate_structure(&self) -> bool {
        // All required fields are enforced by the struct, so if we can construct it,
        // the structure is valid. Individual field validation is handled
        // by the SwiftField trait implementations.
        true
    }

    /// Check if this is a cross-currency transaction
    pub fn is_cross_currency(&self) -> bool {
        if let Some(field_33b) = &self.field_33b {
            field_33b.currency() != self.field_32a.currency_code()
        } else {
            false
        }
    }

    /// Check if exchange rate is provided for cross-currency transactions
    pub fn has_required_exchange_rate(&self) -> bool {
        if self.is_cross_currency() {
            self.field_36.is_some()
        } else {
            true // Not required for same-currency transactions
        }
    }

    /// Check if this message has RETN (return) indicators in field 72
    pub fn has_return_codes(&self) -> bool {
        if let Some(field_72) = &self.field_72 {
            field_72
                .information
                .iter()
                .any(|line| line.contains("/RETN/") || line.to_uppercase().contains("RETURN"))
        } else {
            false
        }
    }

    /// Check if this message has REJT (reject) indicators in field 72
    pub fn has_reject_codes(&self) -> bool {
        if let Some(field_72) = &self.field_72 {
            field_72
                .information
                .iter()
                .any(|line| line.contains("/REJT/") || line.to_uppercase().contains("REJECT"))
        } else {
            false
        }
    }

    /// Check if this is an STP (Straight Through Processing) compliant message
    pub fn is_stp_compliant(&self) -> bool {
        // STP compliance requirements based on MT103.STP specifications:
        // 1. Field 51A is not allowed in STP
        // 2. Fields 52, 54, 55, 56, 57 may only use option A
        // 3. Field 53 may only use options A and B (not D)
        // 4. If field 53B is used, Party Identifier must be present
        // 5. Field 23E may only contain codes CORT, INTC, SDVA, REPA
        // 6. Field 72 codes REJT/RETN must not be used
        // 7. Field 72 must not include ERI information
        // 8. Subfield 1 (Account) of field 59a is always mandatory in STP

        // Field 51A is not allowed in STP
        if self.field_51a.is_some() {
            return false;
        }

        // Check institutional fields - only specific options allowed for STP
        // Field 52: Only option A allowed (D is prohibited)
        if self.field_52d.is_some() {
            return false;
        }

        // Field 53: Options A and B allowed (D is prohibited)
        if self.field_53d.is_some() {
            return false;
        }

        // If field 53B is used, Party Identifier must be present
        if let Some(field_53b) = &self.field_53b {
            // Check if Party Identifier is present and not empty
            if field_53b.party_identifier().is_empty() {
                return false;
            }
        }

        // Fields 54, 55: Only option A allowed (B and D are prohibited)
        if self.field_54b.is_some() || self.field_54d.is_some() {
            return false;
        }

        if self.field_55b.is_some() || self.field_55d.is_some() {
            return false;
        }

        // Fields 56, 57: Only option A allowed (C and D are prohibited)
        if self.field_56c.is_some() || self.field_56d.is_some() {
            return false;
        }

        if self.field_57b.is_some() || self.field_57c.is_some() || self.field_57d.is_some() {
            return false;
        }

        // Check field 23E for STP-allowed codes
        if let Some(field_23e) = &self.field_23e {
            let allowed_codes = ["CORT", "INTC", "SDVA", "REPA"];
            if !allowed_codes.contains(&field_23e.instruction_code.as_str()) {
                return false;
            }
        }

        // Check field 72 for REJT/RETN codes (not allowed in STP)
        if self.has_return_codes() || self.has_reject_codes() {
            return false;
        }

        // Field 59: Account subfield is mandatory in STP
        match &self.field_59 {
            Field59::A(field_59a) => {
                // For option A, account is optional but becomes mandatory in STP
                if field_59a.account().is_none() {
                    return false;
                }
            }
            Field59::F(_field_59f) => {
                // For option F, we don't have an account field, but we need at least party identifier
                // In STP context, Field 59F is typically used with proper party identification
                // which is validated in the constructor, so we just continue
            }
            Field59::NoOption(field_59_basic) => {
                // For no option, we need to check if beneficiary customer lines are present
                // This is a simplified check - the lines should contain account information
                if field_59_basic.beneficiary_customer().is_empty() {
                    return false;
                }
                // Note: More sophisticated parsing would be needed for full account validation
                // in the no-option format, but basic presence check is done here
            }
        }

        true
    }

    /// Check if this is an MT103.REMIT message
    pub fn is_remit(&self) -> bool {
        // MT103.REMIT is identified by the presence of Field 77T
        // and absence of Field 70 (remittance information)
        self.field_77t.is_some() && self.field_70.is_none()
    }

    /// Check if message is REMIT compliant
    pub fn is_remit_compliant(&self) -> bool {
        if !self.is_remit() {
            return false;
        }

        // Field 77T is mandatory for MT103.REMIT
        if self.field_77t.is_none() {
            return false;
        }

        // Field 70 is not allowed in MT103.REMIT
        if self.field_70.is_some() {
            return false;
        }

        // Additional REMIT validations can be added here
        true
    }

    /// Get the message variant type
    pub fn get_variant(&self) -> &'static str {
        if self.is_remit() {
            "MT103.REMIT"
        } else if self.is_stp_compliant() {
            "MT103.STP"
        } else {
            "MT103"
        }
    }

    /// Get all institution fields in routing order for payment processing
    pub fn get_routing_chain(&self) -> Vec<(&str, String)> {
        let mut chain = Vec::new();

        // Ordering Institution
        if let Some(bic) = self.ordering_institution() {
            chain.push(("Ordering Institution", bic));
        }

        // Sender's Correspondent
        if let Some(correspondent) = self.senders_correspondent() {
            chain.push(("Sender's Correspondent", correspondent));
        }

        // Receiver's Correspondent
        if let Some(correspondent) = self.receivers_correspondent() {
            chain.push(("Receiver's Correspondent", correspondent));
        }

        // Third Reimbursement Institution
        if let Some(institution) = self.third_reimbursement_institution() {
            chain.push(("Third Reimbursement", institution));
        }

        // Intermediary Institution
        if let Some(institution) = self.intermediary_institution() {
            chain.push(("Intermediary", institution));
        }

        // Account With Institution
        if let Some(institution) = self.account_with_institution() {
            chain.push(("Account With Institution", institution));
        }

        chain
    }

    /// Extract structured information from field 72 for STP/RETN/REJT processing
    pub fn get_field_72_structured_info(&self) -> Option<Vec<(String, String)>> {
        if let Some(field_72) = &self.field_72 {
            let mut structured_info = Vec::new();

            for line in &field_72.information {
                if line.starts_with('/') && line.len() > 3 {
                    // Extract code and narrative
                    if let Some(end_pos) = line[1..].find('/') {
                        let code = &line[1..end_pos + 1];
                        let narrative = &line[end_pos + 2..];
                        structured_info.push((code.to_string(), narrative.to_string()));
                    }
                }
            }

            if structured_info.is_empty() {
                None
            } else {
                Some(structured_info)
            }
        } else {
            None
        }
    }

    /// Get return/reject reason codes from field 72
    pub fn get_return_reject_codes(&self) -> Vec<String> {
        let mut codes = Vec::new();

        if let Some(structured_info) = self.get_field_72_structured_info() {
            for (code, narrative) in structured_info {
                if code == "RETN" || code == "REJT" {
                    codes.push(format!("{}: {}", code, narrative));
                }
            }
        }

        codes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt103_creation() {
        use chrono::NaiveDate;

        let field_20 = Field20::new("FT21234567890".to_string());
        let field_23b = Field23B::new("CRED".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            "USD".to_string(),
            1000000.00
        );
        let field_50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        let field_59 =
            Field59::NoOption(Field59Basic::new(vec!["JANE SMITH".to_string()]).unwrap());
        let field_71a = Field71A::new("OUR".to_string());

        let mt103 = MT103::new(
            field_20, field_23b, field_32a, field_50, field_59, field_71a,
        );

        assert_eq!(mt103.transaction_reference(), "FT21234567890");
        assert_eq!(mt103.operation_code(), "CRED");
        assert_eq!(mt103.currency_code(), "USD");
        assert_eq!(mt103.charge_code(), "OUR");
    }

    #[test]
    fn test_mt103_message_type() {
        use crate::SwiftMessageBody;
        assert_eq!(MT103::message_type(), "103");
    }

    #[test]
    fn test_mt103_json_field_names() {
        use chrono::NaiveDate;

        let field_20 = Field20::new("FT21234567890".to_string());
        let field_23b = Field23B::new("CRED".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2021, 3, 15).unwrap(),
            "EUR".to_string(),
            1234567.89,
        );
        let field_50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        let field_59 =
            Field59::NoOption(Field59Basic::new(vec!["JANE SMITH".to_string()]).unwrap());
        let field_71a = Field71A::new("OUR".to_string());

        let mt103 = MT103::new(
            field_20, field_23b, field_32a, field_50, field_59, field_71a,
        );

        // Serialize to JSON
        let json = serde_json::to_string(&mt103).unwrap();

        // Verify that JSON uses SWIFT field tags, not struct field names
        assert!(json.contains("\"20\":"));
        assert!(json.contains("\"23B\":"));
        assert!(json.contains("\"32A\":"));
        assert!(json.contains("\"50\":"));
        assert!(json.contains("\"59\":"));
        assert!(json.contains("\"71A\":"));

        // Verify that JSON does NOT contain struct field names
        assert!(!json.contains("\"field_20\":"));
        assert!(!json.contains("\"field_23b\":"));
        assert!(!json.contains("\"field_32a\":"));
        assert!(!json.contains("\"field_50\":"));
        assert!(!json.contains("\"field_59\":"));
        assert!(!json.contains("\"field_71a\":"));
    }

    #[test]
    fn test_mt103_remit_functionality() {
        use chrono::NaiveDate;

        let field_20 = Field20::new("FT21234567890".to_string());
        let field_23b = Field23B::new("CRED".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2021, 3, 15).unwrap(),
            "EUR".to_string(),
            1234567.89,
        );
        let field_50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        let field_59 =
            Field59::NoOption(Field59Basic::new(vec!["JANE SMITH".to_string()]).unwrap());
        let field_71a = Field71A::new("OUR".to_string());
        let field_77t = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();

        let mt103_remit = MT103::new_complete(
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_77t),
        );

        assert!(mt103_remit.is_remit());
        assert!(mt103_remit.is_remit_compliant());
        assert_eq!(mt103_remit.get_variant(), "MT103.REMIT");
        assert!(mt103_remit.envelope_contents().is_some());
    }

    #[test]
    fn test_mt103_field51a_support() {
        use chrono::NaiveDate;

        let field_20 = Field20::new("FT21234567890".to_string());
        let field_23b = Field23B::new("CRED".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2021, 3, 15).unwrap(),
            "EUR".to_string(),
            1234567.89,
        );
        let field_50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        let field_59 =
            Field59::NoOption(Field59Basic::new(vec!["JANE SMITH".to_string()]).unwrap());
        let field_71a = Field71A::new("OUR".to_string());
        let field_51a = Field51A::new(None, None, "CHASUS33XXX").unwrap();

        let mt103_with_51a = MT103::new_complete(
            field_20,
            field_23b,
            field_32a,
            field_50,
            field_59,
            field_71a,
            None,
            None,
            None,
            None,
            None,
            Some(field_51a),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(mt103_with_51a.sending_institution().is_some());
        assert!(!mt103_with_51a.is_stp_compliant()); // Field 51A not allowed in STP
        assert_eq!(mt103_with_51a.get_variant(), "MT103");
    }

    #[test]
    fn test_mt103_variant_detection() {
        use chrono::NaiveDate;

        let field_20 = Field20::new("FT21234567890".to_string());
        let field_23b = Field23B::new("CRED".to_string());
        let field_32a = Field32A::new(
            NaiveDate::from_ymd_opt(2021, 3, 15).unwrap(),
            "EUR".to_string(),
            1234567.89,
        );
        let field_50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()]).unwrap());
        let field_59 =
            Field59::NoOption(Field59Basic::new(vec!["JANE SMITH".to_string()]).unwrap());
        let field_71a = Field71A::new("OUR".to_string());

        // Standard MT103
        let mt103_standard = MT103::new(
            field_20.clone(),
            field_23b.clone(),
            field_32a.clone(),
            field_50.clone(),
            field_59.clone(),
            field_71a.clone(),
        );
        assert_eq!(mt103_standard.get_variant(), "MT103.STP"); // Basic message is STP compliant

        // MT103.REMIT
        let field_77t = Field77T::new("R", "D", "REMITTANCE-2024-001234567890").unwrap();
        let mt103_remit = MT103::new_complete(
            field_20.clone(),
            field_23b.clone(),
            field_32a.clone(),
            field_50.clone(),
            field_59.clone(),
            field_71a.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(field_77t),
        );
        assert_eq!(mt103_remit.get_variant(), "MT103.REMIT");
    }
}
