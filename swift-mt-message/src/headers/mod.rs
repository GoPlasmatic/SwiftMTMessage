//! # SWIFT Message Headers and Trailers
//!
//! ## Purpose
//! Comprehensive header and trailer structures for SWIFT MT messages, implementing the complete
//! SWIFT FIN block structure including Basic Header (Block 1), Application Header (Block 2),
//! User Header (Block 3), and Trailer (Block 5).
//!
//! ## Block Structure
//! - **Block 1**: Basic Header - Sender identification and routing information
//! - **Block 2**: Application Header - Message type and delivery information
//! - **Block 3**: User Header - Optional user-defined fields and references
//! - **Block 5**: Trailer - Optional authentication and delivery confirmation
//!
//! ## Features
//! - **Complete SWIFT Compliance**: Follows SWIFT User Handbook specifications
//! - **Type-Safe Parsing**: Strongly-typed header structures with validation
//! - **Authentication Support**: MAC and authentication key handling
//! - **Sample Generation**: Realistic header generation for testing
//! - **Network Validation**: BIC validation and routing verification

use crate::errors::{ParseError, Result};
use serde::{Deserialize, Serialize};

/// **Basic Header (Block 1): SWIFT Message Identification and Routing**
///
/// ## Purpose
/// The Basic Header constitutes the first mandatory block of every SWIFT message, providing
/// essential identification, routing, and sequencing information. This header enables the
/// SWIFT network to authenticate the sender, route the message appropriately, and maintain
/// message sequence integrity across the global financial messaging infrastructure.
///
/// ## Format Specification
/// - **Block Format**: `{1:F01SSSSSSSSSCCC0000NNNNNN}`
/// - **Total Length**: Exactly 25 characters
/// - **Structure**: Application ID + Service ID + LT Address + Session + Sequence
/// - **Character Set**: Alphanumeric, uppercase only
///
/// ## Business Context Applications
/// - **Message Authentication**: Sender identification and verification
/// - **Network Routing**: Logical terminal addressing for message delivery
/// - **Session Management**: Message sequencing within communication sessions
/// - **Audit Trail**: Complete message tracking and reconciliation
///
/// ## Component Breakdown
/// ### Application Identifier (1 character)
/// - **F**: FIN application (Financial messages)
/// - **A**: GPA application (General Purpose Application)
/// - **L**: GPA application (for certain message types)
/// - **Usage**: Determines message processing rules and validation
///
/// ### Service Identifier (2 characters)
/// - **01**: FIN service (standard financial messages)
/// - **03**: FIN Copy service (third-party copy)
/// - **05**: GPA service
/// - **21**: ACK/NAK service
///
/// ### Logical Terminal Address (12 characters)
/// - **BIC Code**: First 8 characters (institution identifier)
/// - **Terminal Code**: Next 1 character (logical terminal)
/// - **Branch Code**: Last 3 characters (XXX for head office)
/// - **Format**: Must be valid SWIFT-connected BIC
///
/// ## Network Validation Requirements
/// - **BIC Validation**: Must be active SWIFT participant
/// - **Service Compatibility**: Service ID must match message type
/// - **Session Validity**: Session number must be within valid range
/// - **Sequence Continuity**: Sequence numbers must be sequential
///
/// ## Session and Sequence Management
/// ### Session Number (4 digits)
/// - **Range**: 0000-9999
/// - **Purpose**: Groups related messages within a session
/// - **Reset**: Can be reset based on bilateral agreement
/// - **Tracking**: Used for message reconciliation
///
/// ### Sequence Number (6 digits)
/// - **Range**: 000000-999999
/// - **Increment**: Sequential within session
/// - **Uniqueness**: Combined with session ensures message uniqueness
/// - **Recovery**: Critical for message recovery procedures
///
/// ## Regional Considerations
/// - **Time Zones**: LT address determines processing time zone
/// - **Operating Hours**: Service availability based on regional center
/// - **Holiday Schedules**: Regional holiday impacts on processing
/// - **Regulatory Compliance**: Regional reporting requirements
///
/// ## Error Prevention Guidelines
/// - **BIC Accuracy**: Verify sender BIC is authorized for service
/// - **Sequence Management**: Maintain strict sequence number control
/// - **Session Coordination**: Coordinate session numbers with correspondent
/// - **Format Compliance**: Ensure exact 25-character length
///
/// ## Security and Authentication
/// - **Sender Authentication**: BIC must match authenticated connection
/// - **Message Integrity**: Header contributes to message MAC calculation
/// - **Non-repudiation**: Sender identification cannot be disputed
/// - **Audit Trail**: Complete tracking from sender to receiver
///
/// ## Integration with Other Blocks
/// - **Block 2**: Message type and routing continuation
/// - **Block 3**: Optional user header for enhanced services
/// - **Block 4**: Message text validated based on Block 1 service
/// - **Block 5**: Trailer with checksums and authentication
///
/// ## Compliance Framework
/// - **SWIFT Standards**: Full compliance with FIN interface standards
/// - **Service Level Agreements**: Performance guarantees based on service
/// - **Regulatory Reporting**: Sender identification for compliance
/// - **Audit Requirements**: Complete message trail maintenance
///
/// ## Best Practices
/// - **BIC Management**: Keep BIC directory updated
/// - **Sequence Control**: Implement robust sequence management
/// - **Session Planning**: Plan session number allocation
/// - **Error Recovery**: Implement sequence gap detection
///
/// ## See Also
/// - SWIFT FIN Interface Standards: Block 1 Specifications
/// - BIC Directory: Valid SWIFT Participant Codes
/// - Session Management Guide: Best Practices
/// - Message Sequencing: Control and Recovery Procedures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BasicHeader {
    /// Application identifier
    ///
    /// Format: 1!a - Single alphabetic character
    /// Values: F (FIN), A (GPA), L (GPA legacy)
    /// Determines message processing rules and validation requirements
    pub application_id: String,

    /// Service identifier
    ///
    /// Format: 2!n - Two numeric characters
    /// Common values: 01 (FIN), 03 (FIN Copy), 05 (GPA), 21 (ACK/NAK)
    /// Specifies the SWIFT service type for message handling
    pub service_id: String,

    /// Logical Terminal (LT) address
    ///
    /// Format: 12!c - 12 alphanumeric characters
    /// Structure: 8-char BIC + 1-char terminal + 3-char branch
    /// Uniquely identifies the sending terminal in SWIFT network
    pub logical_terminal: String,

    /// Sender BIC extracted from logical terminal
    ///
    /// Format: 8!c - First 8 characters of logical terminal
    /// The sending institution's Bank Identifier Code
    pub sender_bic: String,

    /// Session number
    ///
    /// Format: 4!n - Four numeric digits (0000-9999)
    /// Groups related messages within a communication session
    /// Used for message reconciliation and recovery procedures
    pub session_number: String,

    /// Sequence number
    ///
    /// Format: 6!n - Six numeric digits (000000-999999)
    /// Sequential message counter within session
    /// Critical for message ordering and duplicate detection
    pub sequence_number: String,
}

impl BasicHeader {
    /// Parse basic header from block 1 string
    pub fn parse(block1: &str) -> Result<Self> {
        if block1.len() < 21 {
            return Err(ParseError::InvalidBlockStructure {
                block: "1".to_string(),
                message: format!(
                    "Block 1 too short: expected at least 21 characters, got {}",
                    block1.len()
                ),
            });
        }

        let application_id = block1[0..1].to_string();
        let service_id = block1[1..3].to_string();
        let logical_terminal = block1[3..15].to_string();
        let session_number = block1[15..19].to_string();
        let sequence_number = block1[19..].to_string();
        let sender_bic = logical_terminal[0..8].to_string();

        Ok(BasicHeader {
            application_id,
            service_id,
            logical_terminal,
            sender_bic,
            session_number,
            sequence_number,
        })
    }
}

impl std::fmt::Display for BasicHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.application_id,
            self.service_id,
            self.logical_terminal,
            self.session_number,
            self.sequence_number
        )
    }
}

/// **Application Header (Block 2): Message Type and Routing Information**
///
/// ## Purpose
/// The Application Header provides essential message type identification and routing
/// information, enabling the SWIFT network to properly categorize, prioritize, and
/// deliver messages. This header determines processing rules, delivery monitoring,
/// and time-critical handling requirements for financial messages.
///
/// ## Format Specification
/// - **Input Format**: `{2:I103DDDDDDDDDDDDP[M][OOO]}`
/// - **Output Format**: `{2:O103HHMMDDDDDDDDDDDDYYYYMMDDHHMMNNNNNN}`
/// - **Direction Dependent**: Structure varies for input (I) vs output (O)
/// - **Variable Length**: 18-21 characters for input, fixed for output
///
/// ## Business Context Applications
/// - **Message Classification**: Determines processing rules by message type
/// - **Priority Handling**: Urgent vs normal message processing
/// - **Delivery Assurance**: Monitoring and non-delivery notifications
/// - **Time Management**: Obsolescence period for time-sensitive messages
///
/// ## Direction Indicator
/// ### Input Messages (I)
/// - **Sender Perspective**: Messages being sent to SWIFT
/// - **Validation**: Full message validation applied
/// - **Routing**: To destination specified in header
/// - **Storage**: Stored in sender's message archive
///
/// ### Output Messages (O)
/// - **Receiver Perspective**: Messages delivered from SWIFT
/// - **Delivery**: Includes delivery timestamp and MIR
/// - **Status**: Confirmed successful delivery
/// - **Archive**: Stored in receiver's message archive
///
/// ## Message Type Classification
/// ### Category 1: Customer Payments (MT 1nn)
/// - **MT 103**: Single Customer Credit Transfer
/// - **MT 110**: Advice of Cheque
/// - **Priority**: Often urgent for same-day value
///
/// ### Category 2: Bank Transfers (MT 2nn)
/// - **MT 202**: General Financial Institution Transfer
/// - **MT 202COV**: Cover Payment
/// - **Priority**: High priority for bank liquidity
///
/// ### Category 9: Balance and Status (MT 9nn)
/// - **MT 940**: Customer Statement
/// - **MT 950**: Statement Message
/// - **Priority**: Normal, end-of-day processing
///
/// ## Priority Management
/// ### Urgent Priority (U)
/// - **Processing**: Immediate, ahead of normal messages
/// - **Use Cases**: Time-critical payments, cut-off deadlines
/// - **Delivery**: Fastest available route
/// - **Cost**: Premium pricing may apply
///
/// ### Normal Priority (N)
/// - **Processing**: Standard queue processing
/// - **Use Cases**: Regular payments and messages
/// - **Delivery**: Standard delivery timeframes
/// - **Cost**: Standard message pricing
///
/// ### System Priority (S)
/// - **Processing**: System-generated messages
/// - **Use Cases**: ACKs, NAKs, system notifications
/// - **Delivery**: Highest priority delivery
/// - **Access**: Reserved for SWIFT system use
///
/// ## Delivery Monitoring Options
/// ### Non-Delivery Warning (1)
/// - **Timeout**: Warning if not delivered within set time
/// - **Action**: Sender notified of delivery delay
/// - **Use Case**: Important but not critical messages
///
/// ### Delivery Notification (3)
/// - **Confirmation**: Positive delivery confirmation required
/// - **Notification**: MT 011 sent upon successful delivery
/// - **Use Case**: Critical messages requiring confirmation
///
/// ### No Monitoring (blank)
/// - **Standard**: Default delivery without monitoring
/// - **Notification**: No delivery status updates
/// - **Use Case**: Routine, non-critical messages
///
/// ## Obsolescence Period
/// - **Format**: 3 numeric digits (003-999)
/// - **Unit**: 5-minute intervals
/// - **Maximum**: 999 = 83 hours
/// - **Purpose**: Message validity timeout
/// - **Action**: Automatic cancellation if not delivered
///
/// ## Network Validation Requirements
/// - **BIC Validation**: Destination must be valid SWIFT participant
/// - **Message Type**: Must be valid for sender's subscription
/// - **Priority Rules**: Certain messages restricted to normal priority
/// - **Monitoring Compatibility**: Not all messages support monitoring
///
/// ## Regional Considerations
/// - **Cut-off Times**: Regional deadlines for urgent messages
/// - **Processing Windows**: Regional operating hours impact
/// - **Holiday Handling**: Regional holidays affect delivery
/// - **Regulatory Priority**: Some regions mandate priority levels
///
/// ## Error Prevention Guidelines
/// - **BIC Verification**: Confirm destination BIC is reachable
/// - **Type Validation**: Ensure message type is authorized
/// - **Priority Selection**: Use appropriate priority level
/// - **Monitoring Choice**: Select monitoring based on criticality
///
/// ## Integration with Other Blocks
/// - **Block 1**: Sender identification coordination
/// - **Block 3**: Service options based on message type
/// - **Block 4**: Content validation per message type
/// - **Block 5**: Delivery confirmations and status
///
/// ## Compliance Framework
/// - **Message Standards**: Type-specific validation rules
/// - **Priority Policies**: Fair use of urgent priority
/// - **Delivery SLAs**: Service level compliance
/// - **Audit Trail**: Complete routing documentation
///
/// ## Processing Impact
/// - **Queue Position**: Priority determines processing order
/// - **Validation Depth**: Message type determines checks
/// - **Routing Path**: Optimal path based on priority
/// - **Cost Calculation**: Priority affects message pricing
///
/// ## Best Practices
/// - **Priority Discipline**: Reserve urgent for true urgency
/// - **Monitoring Selection**: Match monitoring to risk level
/// - **Type Accuracy**: Ensure correct message type selection
/// - **Destination Validation**: Verify BIC before sending
///
/// ## See Also
/// - SWIFT FIN User Handbook: Block 2 Specifications
/// - Message Type Catalog: Complete MT Message List
/// - Priority Guidelines: Best Practices for Priority Selection
/// - Delivery Monitoring: Service Options and Usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplicationHeader {
    /// Message direction indicator
    ///
    /// Format: 1!a - Single alphabetic character
    /// Values: I (Input to SWIFT), O (Output from SWIFT)
    /// Determines message format and processing perspective
    pub direction: String,

    /// Message type
    ///
    /// Format: 3!n - Three numeric digits
    /// Examples: 103 (Customer Transfer), 202 (Bank Transfer), 940 (Statement)
    /// Determines validation rules and processing requirements
    pub message_type: String,

    /// Destination address
    ///
    /// Format: 12!c - 12 alphanumeric characters  
    /// Structure: 8-char BIC + 1-char terminal + 3-char branch
    /// Identifies the receiving terminal in SWIFT network
    pub destination_address: String,

    /// Receiver BIC extracted from destination address
    ///
    /// Format: 8!c - First 8 characters of destination
    /// The receiving institution's Bank Identifier Code
    pub receiver_bic: String,

    /// Message priority
    ///
    /// Format: 1!a - Single alphabetic character
    /// Values: U (Urgent), N (Normal), S (System)
    /// Determines processing priority and delivery speed
    pub priority: String,

    /// Delivery monitoring option
    ///
    /// Format: 1!n - Single numeric digit (optional)
    /// Values: 1 (Non-delivery warning), 3 (Delivery notification)
    /// Specifies delivery confirmation requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_monitoring: Option<String>,

    /// Obsolescence period
    ///
    /// Format: 3!n - Three numeric digits (optional)
    /// Range: 003-999 (units of 5 minutes)
    /// Message validity timeout for automatic cancellation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsolescence_period: Option<String>,
}

impl ApplicationHeader {
    /// Parse application header from block 2 string
    pub fn parse(block2: &str) -> Result<Self> {
        if block2.len() < 17 {
            return Err(ParseError::InvalidBlockStructure {
                block: "2".to_string(),
                message: format!(
                    "Block 2 too short: expected at least 18 characters, got {}",
                    block2.len()
                ),
            });
        }

        let direction = block2[0..1].to_string();
        let message_type = block2[1..4].to_string();
        let destination_address = block2[4..16].to_string();
        let priority = block2[16..17].to_string();

        let delivery_monitoring = if block2.len() >= 18 {
            Some(block2[17..18].to_string())
        } else {
            None
        };

        let obsolescence_period = if block2.len() >= 21 {
            Some(block2[18..21].to_string())
        } else {
            None
        };

        let receiver_bic = destination_address[0..8].to_string();

        Ok(ApplicationHeader {
            direction,
            message_type,
            destination_address,
            receiver_bic,
            priority,
            delivery_monitoring,
            obsolescence_period,
        })
    }
}

impl std::fmt::Display for ApplicationHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!(
            "{}{}{}{}",
            self.direction, self.message_type, self.destination_address, self.priority
        );

        if let Some(ref delivery_monitoring) = self.delivery_monitoring {
            result.push_str(delivery_monitoring);
        }

        if let Some(ref obsolescence_period) = self.obsolescence_period {
            result.push_str(obsolescence_period);
        }

        write!(f, "{result}")
    }
}

/// **User Header (Block 3): Extended Service Options and Controls**
///
/// ## Purpose
/// The User Header provides optional extended functionality for SWIFT messages,
/// enabling advanced services, enhanced straight-through processing (STP),
/// compliance controls, and end-to-end transaction tracking. This header has
/// become increasingly important for regulatory compliance, particularly with
/// SWIFT gpi tracking and enhanced payment transparency requirements.
///
/// ## Format Specification
/// - **Block Format**: `{3:{tag:value}{tag:value}...}`
/// - **Tag Format**: Numeric tags with structured values
/// - **Nesting**: Tags enclosed in curly braces
/// - **Optional Block**: Entire block 3 may be omitted
///
/// ## Business Context Applications
/// - **SWIFT gpi Tracking**: End-to-end payment tracking via UETR
/// - **STP Enhancement**: Validation flags for automated processing
/// - **Regulatory Compliance**: Sanctions screening and payment controls
/// - **Service Enhancement**: Additional service options and features
///
/// ## Critical Tags for Modern Payments
/// ### Tag 121: Unique End-to-End Transaction Reference (UETR)
/// - **Format**: 36 characters UUID (8-4-4-4-12 format)
/// - **Purpose**: Global payment tracking across entire payment chain
/// - **Mandatory**: Required for SWIFT gpi participant banks
/// - **Persistence**: Must remain unchanged through payment lifecycle
///
/// ### Tag 119: Validation Flag
/// - **STP**: Straight-Through Processing capability
/// - **REMIT**: Remittance information present
/// - **COV**: Cover payment indicator
/// - **RFDD**: Request for Direct Debit
///
/// ## Service Identifiers
/// ### Tag 103: Service Identifier
/// - **Purpose**: Identifies specific SWIFT services
/// - **Values**: Service-specific codes (e.g., "FIN")
/// - **Usage**: Mainly for FIN Copy service
/// - **Processing**: Affects message routing and copying
///
/// ### Tag 111: Service Type Identifier  
/// - **Format**: 3 numeric digits
/// - **Purpose**: Sub-categorizes service types
/// - **Common**: "001" for standard processing
/// - **Impact**: May affect fee calculation
///
/// ## Message Control and Reference
/// ### Tag 108: Message User Reference (MUR)
/// - **Format**: Up to 16 characters
/// - **Purpose**: Sender's unique reference
/// - **Usage**: Transaction tracking and reconciliation
/// - **Uniqueness**: Should be unique per sender
///
/// ### Tag 113: Banking Priority
/// - **Format**: 4 characters
/// - **Values**: NORM, HIGH, URGP
/// - **Purpose**: Internal bank priority handling
/// - **Note**: Different from network priority
///
/// ## Compliance and Screening Tags
/// ### Tag 433: Sanctions Screening Information
/// - **AOK**: All OK - Passed screening
/// - **FPO**: False Positive Override
/// - **NOK**: Not OK - Requires review
/// - **Additional**: Optional 20 character details
///
/// ### Tag 434: Payment Controls Information
/// - **Format**: 3-letter code + optional details
/// - **Purpose**: Payment control status
/// - **Usage**: Compliance and regulatory controls
/// - **Processing**: May trigger manual review
///
/// ## FIN Copy Service Tags
/// ### Tag 115: Addressee Information
/// - **Format**: Up to 32 characters
/// - **Purpose**: Third-party copy recipient
/// - **Service**: FIN Copy service only
/// - **Delivery**: Additional message copy sent
///
/// ### Tag 165: Payment Release Information
/// - **Format**: 3-char code + optional 34 chars
/// - **Service**: FINInform service
/// - **Purpose**: Payment release notifications
/// - **Usage**: Corporate payment factories
///
/// ## Message Recovery Tags (MIRS)
/// ### Tag 106: Message Input Reference (MIR)
/// - **Format**: 28 characters structured
/// - **Components**: Date + LT + Session + Sequence
/// - **Purpose**: Original message reference
/// - **Usage**: Message recovery and reconciliation
///
/// ### Tag 423: Balance Checkpoint
/// - **Format**: YYMMDDHHMMSS\[ss\]
/// - **Purpose**: Balance snapshot timing
/// - **Service**: MIRS recovery service
/// - **Precision**: Optional hundredths of second
///
/// ### Tag 424: Related Reference
/// - **Format**: Up to 16 characters
/// - **Purpose**: Links related messages
/// - **Usage**: Message chains and corrections
/// - **Service**: MIRS functionality
///
/// ## Network Validation Requirements
/// - **Tag Compatibility**: Some tags require specific services
/// - **Value Validation**: Each tag has specific format rules
/// - **Service Subscription**: Tags available per service agreement
/// - **Mandatory Combinations**: Some tags require others
///
/// ## Regional and Regulatory Impact
/// - **SWIFT gpi**: Tag 121 mandatory for participants
/// - **EU Regulations**: Enhanced screening requirements
/// - **US Compliance**: Specific control requirements
/// - **Local Rules**: Additional regional tag usage
///
/// ## STP Processing Impact
/// ### Validation Flag Effects
/// - **STP Flag**: Enables full automation
/// - **Format Restrictions**: Stricter field validation
/// - **Character Sets**: Limited to STP-safe characters
/// - **Processing Speed**: Faster automated handling
///
/// ## Error Prevention Guidelines
/// - **UETR Format**: Ensure valid UUID format
/// - **Service Compatibility**: Verify tag availability
/// - **Value Formats**: Follow exact specifications
/// - **Mandatory Rules**: Include required combinations
///
/// ## Integration with Other Blocks
/// - **Block 1**: Service must match subscription
/// - **Block 2**: Message type affects available tags
/// - **Block 4**: Validation flags affect field rules
/// - **Block 5**: Some tags reflected in trailer
///
/// ## Compliance Framework
/// - **Regulatory Mandates**: Screening and control requirements
/// - **Audit Trail**: Enhanced tracking via UETR
/// - **Service Agreements**: Tag usage per agreement
/// - **Privacy Rules**: Data handling requirements
///
/// ## Best Practices
/// - **UETR Generation**: Use proper UUID libraries
/// - **Reference Uniqueness**: Ensure MUR uniqueness
/// - **Screening Accuracy**: Accurate screening codes
/// - **Service Alignment**: Use appropriate service tags
///
/// ## Future Evolution
/// - **ISO 20022 Alignment**: Mapping considerations
/// - **Enhanced Tracking**: Additional tracking features
/// - **Compliance Evolution**: New regulatory tags
/// - **Service Innovation**: New service identifiers
///
/// ## See Also
/// - SWIFT FIN User Handbook: Block 3 Tag Specifications
/// - SWIFT gpi Standards: UETR Implementation Guide
/// - STP Guidelines: Validation Flag Requirements
/// - Compliance Framework: Screening Tag Usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserHeader {
    /// Tag 103 - Service Identifier (3!a) - Mandatory for FINcopy Service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_identifier: Option<String>,

    /// Tag 113 - Banking Priority (4!x) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banking_priority: Option<String>,

    /// Tag 108 - Message User Reference (16!x) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_user_reference: Option<String>,

    /// Tag 119 - Validation Flag (8c) - Optional (STP, REMIT, RFDD, COV)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_flag: Option<String>,

    /// Tag 423 - Balance checkpoint date and time (YYMMDDHHMMSS\[ss\]) - Optional (MIRS only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_checkpoint: Option<BalanceCheckpoint>,

    /// Tag 106 - Message Input Reference MIR (28c) - Optional (MIRS only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_input_reference: Option<MessageInputReference>,

    /// Tag 424 - Related reference (16x) - Optional (MIRS only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_reference: Option<String>,

    /// Tag 111 - Service type identifier (3!n) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type_identifier: Option<String>,

    /// Tag 121 - Unique end-to-end transaction reference (UUID format) - Mandatory for GPI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_end_to_end_reference: Option<String>,

    /// Tag 115 - Addressee Information (32x) - Optional (FINCopy only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressee_information: Option<String>,

    /// Tag 165 - Payment release information receiver (3!c/34x) - Optional (FINInform only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_release_information: Option<PaymentReleaseInfo>,

    /// Tag 433 - Sanctions screening information (3!a/\[20x\]) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sanctions_screening_info: Option<SanctionsScreeningInfo>,

    /// Tag 434 - Payment controls information (3!a/\[20x\]) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_controls_info: Option<PaymentControlsInfo>,
}

/// Balance checkpoint structure for Tag 423
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BalanceCheckpoint {
    pub date: String, // YYMMDD
    pub time: String, // HHMMSS

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hundredths_of_second: Option<String>, // ss (optional)
}

/// Message Input Reference structure for Tag 106
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageInputReference {
    pub date: String,            // YYMMDD
    pub lt_identifier: String,   // 12 characters
    pub branch_code: String,     // 3!c
    pub session_number: String,  // 4!n
    pub sequence_number: String, // 6!n
}

/// Payment release information structure for Tag 165
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentReleaseInfo {
    pub code: String, // 3!c

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>, // 34x (optional)
}

/// Sanctions screening information structure for Tag 433
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SanctionsScreeningInfo {
    pub code_word: String, // 3!a (AOK, FPO, NOK)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>, // 20x (optional)
}

/// Payment controls information structure for Tag 434
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentControlsInfo {
    pub code_word: String, // 3!a

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>, // 20x (optional)
}

impl UserHeader {
    /// Parse user header from block 3 string using structured parsing
    pub fn parse(block3: &str) -> Result<Self> {
        let mut user_header = UserHeader::default();

        // Parse nested tags in format {tag:value}
        // Simple parsing for now - more sophisticated regex parsing can be added later
        if block3.contains("{103:") {
            if let Some(start) = block3.find("{103:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.service_identifier =
                        Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{113:") {
            if let Some(start) = block3.find("{113:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.banking_priority = Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{108:") {
            if let Some(start) = block3.find("{108:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.message_user_reference =
                        Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{119:") {
            if let Some(start) = block3.find("{119:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.validation_flag = Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{423:") {
            if let Some(start) = block3.find("{423:") {
                if let Some(end) = block3[start..].find('}') {
                    let value = &block3[start + 5..start + end];
                    user_header.balance_checkpoint = Self::parse_balance_checkpoint(value);
                }
            }
        }

        if block3.contains("{106:") {
            if let Some(start) = block3.find("{106:") {
                if let Some(end) = block3[start..].find('}') {
                    let value = &block3[start + 5..start + end];
                    user_header.message_input_reference =
                        Self::parse_message_input_reference(value);
                }
            }
        }

        if block3.contains("{424:") {
            if let Some(start) = block3.find("{424:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.related_reference =
                        Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{111:") {
            if let Some(start) = block3.find("{111:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.service_type_identifier =
                        Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{121:") {
            if let Some(start) = block3.find("{121:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.unique_end_to_end_reference =
                        Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{115:") {
            if let Some(start) = block3.find("{115:") {
                if let Some(end) = block3[start..].find('}') {
                    user_header.addressee_information =
                        Some(block3[start + 5..start + end].to_string());
                }
            }
        }

        if block3.contains("{165:") {
            if let Some(start) = block3.find("{165:") {
                if let Some(end) = block3[start..].find('}') {
                    let value = &block3[start + 5..start + end];
                    user_header.payment_release_information =
                        Self::parse_payment_release_info(value);
                }
            }
        }

        if block3.contains("{433:") {
            if let Some(start) = block3.find("{433:") {
                if let Some(end) = block3[start..].find('}') {
                    let value = &block3[start + 5..start + end];
                    user_header.sanctions_screening_info =
                        Self::parse_sanctions_screening_info(value);
                }
            }
        }

        if block3.contains("{434:") {
            if let Some(start) = block3.find("{434:") {
                if let Some(end) = block3[start..].find('}') {
                    let value = &block3[start + 5..start + end];
                    user_header.payment_controls_info = Self::parse_payment_controls_info(value);
                }
            }
        }

        Ok(user_header)
    }

    /// Parse balance checkpoint from tag value
    fn parse_balance_checkpoint(value: &str) -> Option<BalanceCheckpoint> {
        if value.len() >= 12 {
            Some(BalanceCheckpoint {
                date: value[0..6].to_string(),
                time: value[6..12].to_string(),
                hundredths_of_second: if value.len() > 12 {
                    Some(value[12..].to_string())
                } else {
                    None
                },
            })
        } else {
            None
        }
    }

    /// Parse message input reference from tag value
    fn parse_message_input_reference(value: &str) -> Option<MessageInputReference> {
        if value.len() >= 28 {
            Some(MessageInputReference {
                date: value[0..6].to_string(),
                lt_identifier: value[6..18].to_string(),
                branch_code: value[18..21].to_string(),
                session_number: value[21..25].to_string(),
                sequence_number: value[25..].to_string(),
            })
        } else {
            None
        }
    }

    /// Parse payment release info from tag value
    fn parse_payment_release_info(value: &str) -> Option<PaymentReleaseInfo> {
        if value.len() >= 3 {
            let code = value[0..3].to_string();
            let additional_info = if value.len() > 4 && value.chars().nth(3) == Some('/') {
                Some(value[4..].to_string())
            } else {
                None
            };
            Some(PaymentReleaseInfo {
                code,
                additional_info,
            })
        } else {
            None
        }
    }

    /// Parse sanctions screening info from tag value
    fn parse_sanctions_screening_info(value: &str) -> Option<SanctionsScreeningInfo> {
        if value.len() >= 3 {
            let code_word = value[0..3].to_string();
            let additional_info = if value.len() > 4 && value.chars().nth(3) == Some('/') {
                Some(value[4..].to_string())
            } else {
                None
            };
            Some(SanctionsScreeningInfo {
                code_word,
                additional_info,
            })
        } else {
            None
        }
    }

    /// Parse payment controls info from tag value
    fn parse_payment_controls_info(value: &str) -> Option<PaymentControlsInfo> {
        if value.len() >= 3 {
            let code_word = value[0..3].to_string();
            let additional_info = if value.len() > 4 && value.chars().nth(3) == Some('/') {
                Some(value[4..].to_string())
            } else {
                None
            };
            Some(PaymentControlsInfo {
                code_word,
                additional_info,
            })
        } else {
            None
        }
    }
}

impl std::fmt::Display for UserHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        if let Some(ref service_id) = self.service_identifier {
            result.push_str(&format!("{{103:{service_id}}}"));
        }

        if let Some(ref banking_priority) = self.banking_priority {
            result.push_str(&format!("{{113:{banking_priority}}}"));
        }

        if let Some(ref message_user_ref) = self.message_user_reference {
            result.push_str(&format!("{{108:{message_user_ref}}}"));
        }

        if let Some(ref validation_flag) = self.validation_flag {
            result.push_str(&format!("{{119:{validation_flag}}}"));
        }

        if let Some(ref unique_end_to_end_ref) = self.unique_end_to_end_reference {
            result.push_str(&format!("{{121:{unique_end_to_end_ref}}}"));
        }

        if let Some(ref service_type_id) = self.service_type_identifier {
            result.push_str(&format!("{{111:{service_type_id}}}"));
        }

        if let Some(ref payment_controls) = self.payment_controls_info {
            let mut value = payment_controls.code_word.clone();
            if let Some(ref additional) = payment_controls.additional_info {
                value.push('/');
                value.push_str(additional);
            }
            result.push_str(&format!("{{434:{value}}}"));
        }

        if let Some(ref payment_release) = self.payment_release_information {
            let mut value = payment_release.code.clone();
            if let Some(ref additional) = payment_release.additional_info {
                value.push('/');
                value.push_str(additional);
            }
            result.push_str(&format!("{{165:{value}}}"));
        }

        if let Some(ref sanctions) = self.sanctions_screening_info {
            let mut value = sanctions.code_word.clone();
            if let Some(ref additional) = sanctions.additional_info {
                value.push('/');
                value.push_str(additional);
            }
            result.push_str(&format!("{{433:{value}}}"));
        }

        write!(f, "{result}")
    }
}

/// **Trailer (Block 5): Message Security and Control Information**
///
/// ## Purpose
/// The Trailer block provides essential security, authentication, and control
/// information for SWIFT messages. It ensures message integrity through checksums,
/// enables duplicate detection, supports message authentication, and provides
/// system-level control information critical for secure and reliable message delivery.
///
/// ## Format Specification
/// - **Block Format**: `{5:{tag:value}{tag:value}...}`
/// - **Tag Types**: Three-letter tags with optional values
/// - **Security Tags**: MAC and CHK for authentication
/// - **Control Tags**: Various operational controls
///
/// ## Business Context Applications
/// - **Message Integrity**: Checksum validation for data integrity
/// - **Security Authentication**: MAC for message authentication
/// - **Duplicate Detection**: Prevention of duplicate processing
/// - **Operational Control**: Test messages and system controls
///
/// ## Security and Authentication Tags
/// ### CHK: Checksum (Mandatory)
/// - **Format**: 12 hexadecimal characters
/// - **Calculation**: Algorithm-based on message content
/// - **Purpose**: Detect transmission errors
/// - **Validation**: Automatic by SWIFT network
/// - **Failure Action**: Message rejection
///
/// ### MAC: Message Authentication Code
/// - **Format**: Variable length hexadecimal
/// - **Algorithm**: Agreed between parties
/// - **Purpose**: Authenticate message origin
/// - **Usage**: High-value or sensitive messages
/// - **Bilateral**: Requires key exchange
///
/// ## Duplicate Control Tags
/// ### PDM: Possible Duplicate Message
/// - **Format**: Optional time + MOR
/// - **Purpose**: Warn of possible duplicate
/// - **Action**: Receiver should check for duplicates
/// - **Components**: Time (HHMM) + Message Output Reference
///
/// ### PDE: Possible Duplicate Emission
/// - **Format**: Optional time + MIR  
/// - **Purpose**: Sender suspects duplicate sent
/// - **Usage**: Network recovery scenarios
/// - **Components**: Time (HHMM) + Message Input Reference
///
/// ## Operational Control Tags
/// ### TNG: Test and Training
/// - **Format**: Empty tag (presence only)
/// - **Purpose**: Identifies test messages
/// - **Processing**: Should not affect production
/// - **Usage**: Testing and training environments
/// - **Warning**: Must not be processed as live
///
/// ### DLM: Delayed Message
/// - **Format**: Empty tag (presence only)
/// - **Purpose**: Indicates delayed transmission
/// - **Cause**: Network issues or recovery
/// - **Action**: Check value dates and cut-offs
///
/// ## Reference and Tracking Tags
/// ### MRF: Message Reference
/// - **Format**: Date + Time + MIR
/// - **Purpose**: Reference related messages
/// - **Usage**: Corrections and cancellations
/// - **Components**: YYMMDD + HHMM + full MIR
///
/// ### SYS: System Originated Message
/// - **Format**: Optional time + MIR
/// - **Purpose**: System-generated messages
/// - **Examples**: Automatic responses
/// - **Processing**: May have special handling
///
/// ## Message Reference Structures
/// ### Message Input Reference (MIR)
/// - **Date**: YYMMDD format
/// - **LT Identifier**: 12-character sending LT
/// - **Session**: 4-digit session number
/// - **Sequence**: 6-digit sequence number
/// - **Usage**: Unique message identification
///
/// ### Message Output Reference (MOR)
/// - **Format**: Same structure as MIR
/// - **Perspective**: Receiver's reference
/// - **Purpose**: Delivery confirmation
/// - **Tracking**: End-to-end message tracking
///
/// ## Network Validation Requirements
/// - **CHK Mandatory**: All messages must have checksum
/// - **Tag Order**: Specific ordering requirements
/// - **Format Compliance**: Exact format specifications
/// - **Value Validation**: Tag-specific validations
///
/// ## Security Considerations
/// ### Checksum Protection
/// - **Coverage**: Entire message content
/// - **Algorithm**: SWIFT-specified calculation
/// - **Tampering**: Detects any modification
/// - **Reliability**: Very low false positive rate
///
/// ### MAC Authentication
/// - **Bilateral Agreement**: Key management required
/// - **Algorithm Choice**: Per agreement
/// - **Non-repudiation**: Proves message origin
/// - **Legal Standing**: Admissible evidence
///
/// ## Duplicate Detection Mechanisms
/// ### System Design
/// - **Detection Window**: Configurable timeframe
/// - **Reference Tracking**: MIR/MOR correlation
/// - **Recovery Support**: Post-incident reconciliation
/// - **Audit Trail**: Complete duplicate history
///
/// ### Processing Rules
/// - **PDM Messages**: Manual review recommended
/// - **Duplicate Window**: Typically 24-48 hours
/// - **Action Required**: Verify before processing
/// - **Documentation**: Record resolution actions
///
/// ## Operational Guidelines
/// ### Test Message Handling
/// - **TNG Identification**: Clear test marking
/// - **Environment Separation**: Test vs production
/// - **Processing Prevention**: Automatic filtering
/// - **Audit Exclusion**: Separate test reporting
///
/// ### Delayed Message Processing
/// - **DLM Recognition**: Special handling required
/// - **Value Date Check**: Verify still valid
/// - **Cut-off Impact**: May miss deadlines
/// - **Notification**: Alert relevant parties
///
/// ## Error Prevention Guidelines
/// - **CHK Calculation**: Automatic by system
/// - **Tag Formatting**: Follow exact specifications
/// - **Reference Accuracy**: Verify MIR/MOR format
/// - **Test Separation**: Clear test identification
///
/// ## Integration with Other Blocks
/// - **Block 1-4**: Content for checksum calculation
/// - **Block 1**: Session/sequence for references
/// - **Block 2**: Message type affects trailer options
/// - **Block 3**: Some services require specific tags
///
/// ## Compliance Framework
/// - **Security Standards**: Cryptographic requirements
/// - **Audit Requirements**: Trailer preservation
/// - **Legal Admissibility**: Authentication standards
/// - **Regulatory Compliance**: Security controls
///
/// ## Recovery and Reconciliation
/// ### Message Recovery
/// - **Reference Tracking**: Via MIR/MOR
/// - **Duplicate Resolution**: PDM/PDE handling
/// - **Audit Support**: Complete tag history
/// - **Dispute Resolution**: Authentication proof
///
/// ### System Recovery
/// - **Checkpoint References**: Recovery points
/// - **Sequence Verification**: Gap detection
/// - **Duplicate Prevention**: During recovery
/// - **Integrity Validation**: CHK verification
///
/// ## Best Practices
/// - **Security First**: Always validate CHK
/// - **MAC Usage**: For high-value messages
/// - **Duplicate Vigilance**: Check PDM warnings
/// - **Test Clarity**: Clearly mark test messages
///
/// ## See Also
/// - SWIFT FIN Security Guide: Authentication Standards
/// - Checksum Algorithms: Technical Specifications
/// - Duplicate Detection: Best Practices Guide
/// - MAC Implementation: Bilateral Agreement Templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Trailer {
    /// CHK - Checksum (12!h) - Mandatory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,

    /// TNG - Test & Training Message - Optional (empty tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_and_training: Option<bool>,

    /// PDE - Possible Duplicate Emission - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_duplicate_emission: Option<PossibleDuplicateEmission>,

    /// DLM - Delayed Message - Optional (empty tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delayed_message: Option<bool>,

    /// MRF - Message Reference - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,

    /// PDM - Possible Duplicate Message - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_duplicate_message: Option<PossibleDuplicateMessage>,

    /// SYS - System Originated Message - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_originated_message: Option<SystemOriginatedMessage>,

    /// MAC - Message Authentication Code - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
}

/// Possible Duplicate Emission structure for PDE tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PossibleDuplicateEmission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>, // HHMM (optional)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_input_reference: Option<MessageInputReference>, // MIR (optional)
}

/// Message Reference structure for MRF tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReference {
    pub date: String,                                   // YYMMDD
    pub full_time: String,                              // HHMM
    pub message_input_reference: MessageInputReference, // MIR
}

/// Possible Duplicate Message structure for PDM tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PossibleDuplicateMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>, // HHMM (optional)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_output_reference: Option<MessageOutputReference>, // MOR (optional)
}

/// Message Output Reference structure (similar to MIR but for output)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageOutputReference {
    pub date: String,            // YYMMDD
    pub lt_identifier: String,   // 12 characters
    pub branch_code: String,     // 3!c
    pub session_number: String,  // 4!n
    pub sequence_number: String, // 6!n
}

/// System Originated Message structure for SYS tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemOriginatedMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>, // HHMM (optional)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_input_reference: Option<MessageInputReference>, // MIR (optional)
}

impl Trailer {
    /// Parse trailer from block 5 string using structured parsing
    pub fn parse(block5: &str) -> Result<Self> {
        let mut trailer = Trailer::default();

        // Extract common tags if present
        if block5.contains("{CHK:") {
            if let Some(start) = block5.find("{CHK:") {
                if let Some(end) = block5[start..].find('}') {
                    trailer.checksum = Some(block5[start + 5..start + end].to_string());
                }
            }
        }

        if block5.contains("{TNG}") {
            trailer.test_and_training = Some(true);
        }

        if block5.contains("{DLM}") {
            trailer.delayed_message = Some(true);
        }

        if block5.contains("{MAC:") {
            if let Some(start) = block5.find("{MAC:") {
                if let Some(end) = block5[start..].find('}') {
                    trailer.mac = Some(block5[start + 5..start + end].to_string());
                }
            }
        }

        // More complex parsing for structured tags can be added here
        // For now, implementing basic tag extraction

        Ok(trailer)
    }
}

impl std::fmt::Display for Trailer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        if let Some(ref checksum) = self.checksum {
            result.push_str(&format!("{{CHK:{checksum}}}"));
        }

        if let Some(true) = self.test_and_training {
            result.push_str("{TNG}");
        }

        if let Some(true) = self.delayed_message {
            result.push_str("{DLM}");
        }

        if let Some(ref possible_duplicate_emission) = self.possible_duplicate_emission {
            result.push_str(&format!(
                "{{PDE:{}}}",
                possible_duplicate_emission.time.as_deref().unwrap_or("")
            ));
        }

        if let Some(ref message_reference) = self.message_reference {
            result.push_str(&format!("{{MRF:{}}}", message_reference.date));
        }

        if let Some(ref mac) = self.mac {
            result.push_str(&format!("{{MAC:{mac}}}"));
        }

        write!(f, "{result}")
    }
}

/// Sample generation functions for SWIFT headers
impl BasicHeader {
    /// Generate a sample BasicHeader with realistic values
    pub fn sample() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let sender_bic_code = crate::sample::generate_valid_bic();
        let logical_terminal = format!("{sender_bic_code}XXXX");

        BasicHeader {
            application_id: "F".to_string(),
            service_id: "01".to_string(),
            logical_terminal: logical_terminal.clone(),
            sender_bic: sender_bic_code.to_string(),
            session_number: format!("{:04}", rng.gen_range(1000..9999)),
            sequence_number: format!("{:06}", rng.gen_range(100000..999999)),
        }
    }

    /// Generate a sample BasicHeader with custom BIC
    pub fn sample_with_bic(bic: &str) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let logical_terminal = format!("{bic}XXXX");

        BasicHeader {
            application_id: "F".to_string(),
            service_id: "01".to_string(),
            logical_terminal: logical_terminal.clone(),
            sender_bic: bic.to_string(),
            session_number: format!("{:04}", rng.gen_range(1000..9999)),
            sequence_number: format!("{:06}", rng.gen_range(100000..999999)),
        }
    }
}

impl ApplicationHeader {
    /// Generate a sample ApplicationHeader with realistic values
    pub fn sample(message_type: &str) -> Self {
        let receiver_bic_code = crate::sample::generate_valid_bic();
        let destination_address = format!("{receiver_bic_code}XXXX");

        ApplicationHeader {
            direction: "I".to_string(),
            message_type: message_type.to_string(),
            destination_address: destination_address.clone(),
            receiver_bic: receiver_bic_code.to_string(),
            priority: "N".to_string(),
            delivery_monitoring: Some("3".to_string()),
            obsolescence_period: None,
        }
    }

    /// Generate a sample ApplicationHeader with custom BIC and priority
    pub fn sample_with_config(message_type: &str, receiver_bic: &str, priority: &str) -> Self {
        let destination_address = format!("{receiver_bic}XXXX");

        ApplicationHeader {
            direction: "I".to_string(),
            message_type: message_type.to_string(),
            destination_address: destination_address.clone(),
            receiver_bic: receiver_bic.to_string(),
            priority: priority.to_string(),
            delivery_monitoring: Some("3".to_string()),
            obsolescence_period: None,
        }
    }
}

impl UserHeader {
    /// Generate a sample UserHeader with UETR for CBPR+ compliance
    pub fn sample() -> Self {
        Self::sample_with_scenario(None)
    }

    /// Generate a minimal UserHeader with only UETR
    pub fn sample_minimal() -> Self {
        Self::sample_with_scenario(Some(&crate::sample::MessageScenario::Minimal))
    }

    /// Generate a full UserHeader with multiple optional fields
    pub fn sample_full() -> Self {
        Self::sample_with_scenario(Some(&crate::sample::MessageScenario::Full))
    }

    /// Generate a UserHeader with scenario-specific configuration
    pub fn sample_with_scenario(scenario: Option<&crate::sample::MessageScenario>) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Set validation flag based on scenario
        let validation_flag = match scenario {
            Some(crate::sample::MessageScenario::StpCompliant) => Some("STP".to_string()),
            Some(crate::sample::MessageScenario::CoverPayment) => Some("COV".to_string()),
            Some(crate::sample::MessageScenario::Minimal) => None,
            Some(crate::sample::MessageScenario::Full) => {
                // For full scenario, randomly pick between different validation flags
                let flags = ["STP", "REMIT", "COV"];
                Some(flags[rng.gen_range(0..flags.len())].to_string())
            }
            _ => {
                // For standard scenario, sometimes include validation flags
                if rng.gen_bool(0.7) {
                    let flags = ["STP", "REMIT"];
                    Some(flags[rng.gen_range(0..flags.len())].to_string())
                } else {
                    None
                }
            }
        };

        match scenario {
            Some(crate::sample::MessageScenario::Minimal) => UserHeader {
                service_identifier: None,
                banking_priority: None,
                message_user_reference: None,
                validation_flag,
                balance_checkpoint: None,
                message_input_reference: None,
                related_reference: None,
                service_type_identifier: None,
                unique_end_to_end_reference: Some(crate::sample::generate_uetr()),
                addressee_information: None,
                payment_release_information: None,
                sanctions_screening_info: None,
                payment_controls_info: None,
            },
            Some(crate::sample::MessageScenario::Full) => UserHeader {
                service_identifier: Some("FIN".to_string()),
                banking_priority: Some("NORM".to_string()),
                message_user_reference: Some(crate::sample::generate_reference()),
                validation_flag,
                balance_checkpoint: None,
                message_input_reference: None,
                related_reference: Some(crate::sample::generate_reference()),
                service_type_identifier: Some("001".to_string()),
                unique_end_to_end_reference: Some(crate::sample::generate_uetr()),
                addressee_information: Some("BANK OF ENGLAND".to_string()),
                payment_release_information: None,
                sanctions_screening_info: if rng.gen_bool(0.3) {
                    Some(SanctionsScreeningInfo {
                        code_word: "AOK".to_string(),
                        additional_info: None,
                    })
                } else {
                    None
                },
                payment_controls_info: None,
            },
            _ => {
                // Standard, StpCompliant, CoverPayment scenarios
                UserHeader {
                    service_identifier: None,
                    banking_priority: None,
                    message_user_reference: Some(crate::sample::generate_reference()),
                    validation_flag,
                    balance_checkpoint: None,
                    message_input_reference: None,
                    related_reference: None,
                    service_type_identifier: Some("001".to_string()),
                    unique_end_to_end_reference: Some(crate::sample::generate_uetr()),
                    addressee_information: None,
                    payment_release_information: None,
                    sanctions_screening_info: None,
                    payment_controls_info: None,
                }
            }
        }
    }
}

impl Trailer {
    /// Generate a sample Trailer with realistic values
    pub fn sample() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Trailer {
            checksum: if rng.gen_bool(0.5) {
                Some(format!("{:08X}", rng.r#gen::<u32>()))
            } else {
                None
            },
            test_and_training: None,
            possible_duplicate_emission: None,
            delayed_message: None,
            message_reference: None,
            possible_duplicate_message: None,
            system_originated_message: None,
            mac: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_header_parse() {
        let block1 = "F01BANKDEFFAXXX0123456789";
        let header = BasicHeader::parse(block1).unwrap();

        assert_eq!(header.application_id, "F");
        assert_eq!(header.service_id, "01");
        assert_eq!(header.logical_terminal, "BANKDEFFAXXX");
        assert_eq!(header.session_number, "0123");
        assert_eq!(header.sequence_number, "456789");
    }

    #[test]
    fn test_application_header_parse() {
        let block2 = "I103BANKDEFFAXXXU3003";
        let header = ApplicationHeader::parse(block2).unwrap();

        assert_eq!(header.direction, "I");
        assert_eq!(header.message_type, "103");
        assert_eq!(header.destination_address, "BANKDEFFAXXX");
        assert_eq!(header.priority, "U");
        assert_eq!(header.delivery_monitoring, Some("3".to_string()));
        assert_eq!(header.obsolescence_period, Some("003".to_string()));
    }
}
