//! # SWIFT Message Headers and Trailers
//!
//! SWIFT FIN block structures for MT message headers and trailers.
//!
//! ## Block Structure
//! - **Block 1:** Basic Header - Sender identification and routing
//! - **Block 2:** Application Header - Message type and delivery info
//! - **Block 3:** User Header - Optional service tags (UETR, validation flags)
//! - **Block 5:** Trailer - Security (MAC, checksum) and control tags
//!
//! ## Usage
//! ```rust
//! use swift_mt_message::headers::{BasicHeader, ApplicationHeader};
//!
//! # fn main() -> swift_mt_message::Result<()> {
//! let basic = BasicHeader::parse("F01DEUTDEFFAXXX0000123456")?;
//! let app = ApplicationHeader::parse("I103CHASUS33AXXXN")?;
//! # Ok(())
//! # }
//! ```

use crate::errors::{ParseError, Result};
use serde::{Deserialize, Serialize};

/// **Block 1: Basic Header**
///
/// Sender identification and message routing information.
///
/// **Format:** `F01SSSSSSSSSCCC0000NNNNNN` (25 chars)
/// - App ID (1): F=FIN, A=GPA
/// - Service (2): 01=FIN, 03=FIN Copy
/// - LT Address (12): BIC + terminal + branch
/// - Session (4): 0000-9999
/// - Sequence (6): 000000-999999
///
/// **Example:** `F01DEUTDEFFAXXX0000123456`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct BasicHeader {
    /// Application ID (F, A, L)
    pub application_id: String,
    /// Service ID (01, 03, 05, 21)
    pub service_id: String,
    /// Logical terminal address (12 chars)
    pub logical_terminal: String,
    /// Sender BIC (8 chars)
    pub sender_bic: String,
    /// Session number (4 digits)
    pub session_number: String,
    /// Sequence number (6 digits)
    pub sequence_number: String,
}

// Custom Serialize/Deserialize to normalize BIC padding
impl serde::Serialize for BasicHeader {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        // Normalize logical_terminal to exactly 12 characters for JSON
        let normalized_logical_terminal = if self.logical_terminal.len() > 12 {
            self.logical_terminal[..12].to_string()
        } else if self.logical_terminal.len() < 12 {
            format!("{:X<12}", self.logical_terminal)
        } else {
            self.logical_terminal.clone()
        };

        let mut state = serializer.serialize_struct("BasicHeader", 5)?;
        state.serialize_field("application_id", &self.application_id)?;
        state.serialize_field("service_id", &self.service_id)?;
        state.serialize_field("logical_terminal", &normalized_logical_terminal)?;
        state.serialize_field("sender_bic", &self.sender_bic)?;
        state.serialize_field("session_number", &self.session_number)?;
        state.serialize_field("sequence_number", &self.sequence_number)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for BasicHeader {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BasicHeaderHelper {
            application_id: String,
            service_id: String,
            logical_terminal: String,
            sender_bic: String,
            session_number: String,
            sequence_number: String,
        }

        let helper = BasicHeaderHelper::deserialize(deserializer)?;

        // Normalize logical_terminal to exactly 12 characters
        let normalized_logical_terminal = if helper.logical_terminal.len() > 12 {
            helper.logical_terminal[..12].to_string()
        } else if helper.logical_terminal.len() < 12 {
            format!("{:X<12}", helper.logical_terminal)
        } else {
            helper.logical_terminal.clone()
        };

        // Use the original sender_bic from the JSON
        // It should match what's in the first part of logical_terminal
        let sender_bic = helper.sender_bic;

        Ok(BasicHeader {
            application_id: helper.application_id,
            service_id: helper.service_id,
            logical_terminal: normalized_logical_terminal,
            sender_bic,
            session_number: helper.session_number,
            sequence_number: helper.sequence_number,
        })
    }
}

impl BasicHeader {
    /// Parse basic header from block 1 string
    pub fn parse(block1: &str) -> Result<Self> {
        // Expected format: F01SSSSSSSSSCCC0000NNNNNN (exactly 25 characters)
        // Where: F=app_id, 01=service_id, SSSSSSSSSCCC=logical_terminal(12), 0000=session(4), NNNNNN=sequence(6)
        if block1.len() != 25 {
            return Err(ParseError::InvalidBlockStructure {
                block: "1".to_string(),
                message: format!(
                    "Block 1 must be exactly 25 characters, got {}",
                    block1.len()
                ),
            });
        }

        let application_id = block1[0..1].to_string();
        let service_id = block1[1..3].to_string();
        let raw_logical_terminal = block1[3..15].to_string();
        let session_number = block1[15..19].to_string();
        let sequence_number = block1[19..25].to_string();

        // Keep the full 12-character logical terminal as stored in the MT format
        // The padding is necessary for the MT format and we handle normalization in tests
        let logical_terminal = raw_logical_terminal;

        // Extract BIC from logical_terminal
        // SWIFT BICs are either 8 or 11 characters
        // The logical_terminal is 12 chars: BIC (8 or 11) + optional terminal ID
        // Common patterns:
        // - 8-char BIC + 4-char terminal: "DEUTDEFFAXXX" -> BIC="DEUTDEFF"
        // - 11-char BIC + 1-char terminal: "DEUTDEFF001A" -> BIC="DEUTDEFF001"

        let sender_bic = if logical_terminal.len() == 12 {
            // Check if this looks like an 8-char BIC with terminal suffix
            // Terminal suffixes are typically single letter + XXX (e.g., AXXX, BXXX)
            // or just XXX for no specific terminal
            let last_four = &logical_terminal[8..12];
            if last_four == "XXXX" || (last_four.len() == 4 && &last_four[1..] == "XXX") {
                // 8-character BIC with terminal suffix
                logical_terminal[0..8].to_string()
            } else {
                // Check if chars 9-11 could be a valid branch code
                let potential_branch = &logical_terminal[8..11];
                if potential_branch.chars().all(|c| c.is_ascii_alphanumeric())
                    && potential_branch != "XXX"
                {
                    // Likely an 11-character BIC
                    logical_terminal[0..11].to_string()
                } else {
                    // Default to 8-character BIC
                    logical_terminal[0..8].to_string()
                }
            }
        } else if logical_terminal.len() >= 11 {
            logical_terminal[0..11].to_string()
        } else if logical_terminal.len() >= 8 {
            logical_terminal[0..8].to_string()
        } else {
            // Should not happen with valid SWIFT messages
            logical_terminal.clone()
        };

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
        // Ensure consistent formatting:
        // - logical_terminal must be exactly 12 characters (8-char BIC + 1-char LT + 3-char branch)
        // - session_number must be exactly 4 digits
        // - sequence_number must be exactly 6 digits

        // Pad or truncate logical_terminal to exactly 12 characters
        let logical_terminal = if self.logical_terminal.len() > 12 {
            self.logical_terminal[..12].to_string()
        } else if self.logical_terminal.len() < 12 {
            // Pad with 'X' to reach 12 characters (standard for missing branch codes)
            format!("{:X<12}", self.logical_terminal)
        } else {
            self.logical_terminal.clone()
        };

        // Ensure session_number is exactly 4 digits, left-padded with zeros
        let session_number = format!(
            "{:0>4}",
            &self.session_number[..self.session_number.len().min(4)]
        );

        // Ensure sequence_number is exactly 6 digits, left-padded with zeros
        let sequence_number = format!(
            "{:0>6}",
            &self.sequence_number[..self.sequence_number.len().min(6)]
        );

        write!(
            f,
            "{}{}{}{}{}",
            self.application_id, self.service_id, logical_terminal, session_number, sequence_number
        )
    }
}

/// **Input Application Header**
///
/// Message being sent to SWIFT network.
///
/// **Format:** `I103DDDDDDDDDDDDP[M][OOO]` (17-21 chars)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct InputApplicationHeader {
    /// Message type (3 digits: 103, 202, 940, etc.)
    pub message_type: String,
    /// Destination address (12 chars: BIC + terminal + branch)
    pub destination_address: String,
    /// Receiver BIC (8 chars)
    pub receiver_bic: String,
    /// Priority (U=Urgent, N=Normal, S=System)
    pub priority: String,
    /// Delivery monitoring (1, 2, 3)
    pub delivery_monitoring: Option<String>,
    /// Obsolescence period (003-999, units of 5 min)
    pub obsolescence_period: Option<String>,
}

// Custom Serialize/Deserialize to normalize destination_address BIC padding
impl serde::Serialize for InputApplicationHeader {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        // Normalize destination_address to exactly 12 characters for JSON
        let normalized_destination_address = if self.destination_address.len() > 12 {
            self.destination_address[..12].to_string()
        } else if self.destination_address.len() < 12 {
            format!("{:X<12}", self.destination_address)
        } else {
            self.destination_address.clone()
        };

        let field_count = 4
            + self.delivery_monitoring.is_some() as usize
            + self.obsolescence_period.is_some() as usize;
        let mut state = serializer.serialize_struct("InputApplicationHeader", field_count)?;
        state.serialize_field("message_type", &self.message_type)?;
        state.serialize_field("destination_address", &normalized_destination_address)?;
        state.serialize_field("receiver_bic", &self.receiver_bic)?;
        state.serialize_field("priority", &self.priority)?;
        if let Some(ref dm) = self.delivery_monitoring {
            state.serialize_field("delivery_monitoring", dm)?;
        }
        if let Some(ref op) = self.obsolescence_period {
            state.serialize_field("obsolescence_period", op)?;
        }
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for InputApplicationHeader {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InputApplicationHeaderHelper {
            message_type: String,
            destination_address: String,
            receiver_bic: String,
            priority: String,
            delivery_monitoring: Option<String>,
            obsolescence_period: Option<String>,
        }

        let helper = InputApplicationHeaderHelper::deserialize(deserializer)?;

        // Normalize destination_address to exactly 12 characters
        let normalized_destination_address = if helper.destination_address.len() > 12 {
            helper.destination_address[..12].to_string()
        } else if helper.destination_address.len() < 12 {
            format!("{:X<12}", helper.destination_address)
        } else {
            helper.destination_address.clone()
        };

        // Use the original receiver_bic from the JSON
        // It should match what's in the first part of destination_address
        let receiver_bic = helper.receiver_bic;

        Ok(InputApplicationHeader {
            message_type: helper.message_type,
            destination_address: normalized_destination_address,
            receiver_bic,
            priority: helper.priority,
            delivery_monitoring: helper.delivery_monitoring,
            obsolescence_period: helper.obsolescence_period,
        })
    }
}

/// **Output Application Header**
///
/// Message delivered from SWIFT network.
///
/// **Format:** `O103HHMMYYYYMMDDDDDDDDDDDDDDNNNNSSSSSSYYYYMMDDHHMMP` (46-47 chars)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct OutputApplicationHeader {
    /// Message type (3 digits)
    pub message_type: String,
    /// Input time (HHMM)
    pub input_time: String,
    /// Message Input Reference (MIR)
    pub mir: MessageInputReference,
    /// Output date (YYMMDD)
    pub output_date: String,
    /// Output time (HHMM)
    pub output_time: String,
    /// Priority (U, N, S)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

/// **Block 2: Application Header**
///
/// Message type and routing information.
/// Direction-dependent: Input (I) for outgoing, Output (O) for incoming messages.
///
/// **Variants:**
/// - **Input:** Message being sent to SWIFT (format: `I103DDDDDDDDDDDDP[M][OOO]`)
/// - **Output:** Message delivered from SWIFT (format: `O103HHMMYYYYMMDDDDDDDDDDDDDDNNNNSSSSSSYYYYMMDDHHMMP`)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[serde(tag = "direction")]
pub enum ApplicationHeader {
    /// Input message sent to SWIFT network
    #[serde(rename = "I")]
    Input(InputApplicationHeader),

    /// Output message delivered from SWIFT network
    #[serde(rename = "O")]
    Output(OutputApplicationHeader),
}

impl ApplicationHeader {
    /// Parse application header from block 2 string
    pub fn parse(block2: &str) -> Result<Self> {
        if block2.len() < 4 {
            return Err(ParseError::InvalidBlockStructure {
                block: "2".to_string(),
                message: format!(
                    "Block 2 too short: expected at least 4 characters, got {}",
                    block2.len()
                ),
            });
        }

        let direction = &block2[0..1];
        let message_type = block2[1..4].to_string();

        match direction {
            "I" => {
                // Input format: {2:I103DDDDDDDDDDDDP[M][OOO]}
                // I + 103 + 12-char destination + priority + optional monitoring + optional obsolescence
                if block2.len() < 17 {
                    return Err(ParseError::InvalidBlockStructure {
                        block: "2".to_string(),
                        message: format!(
                            "Input Block 2 too short: expected at least 17 characters, got {}",
                            block2.len()
                        ),
                    });
                }

                let raw_destination_address = block2[4..16].to_string();
                let priority = block2[16..17].to_string();

                // Keep the full 12-character destination address as stored in the MT format
                // The padding is necessary for the MT format and we handle normalization in tests
                let destination_address = raw_destination_address;

                // Extract BIC from destination_address
                // SWIFT BICs are either 8 or 11 characters
                // If the destination_address ends with XXX or XXXX, it's likely padding
                let receiver_bic = if destination_address.len() == 12 {
                    // Check if this looks like an 8-char BIC with terminal suffix
                    // Terminal suffixes are typically single letter + XXX (e.g., AXXX, BXXX)
                    // or just XXXX for no specific terminal
                    let last_four = &destination_address[8..12];
                    if last_four == "XXXX" || (last_four.len() == 4 && &last_four[1..] == "XXX") {
                        // 8-character BIC with terminal suffix
                        destination_address[0..8].to_string()
                    } else {
                        // Check if chars 9-11 could be a valid branch code
                        let potential_branch = &destination_address[8..11];
                        if potential_branch.chars().all(|c| c.is_ascii_alphanumeric())
                            && potential_branch != "XXX"
                        {
                            // Likely an 11-character BIC
                            destination_address[0..11].to_string()
                        } else {
                            // Default to 8-character BIC
                            destination_address[0..8].to_string()
                        }
                    }
                } else if destination_address.len() >= 11 {
                    destination_address[0..11].to_string()
                } else if destination_address.len() >= 8 {
                    destination_address[0..8].to_string()
                } else {
                    // Should not happen with valid SWIFT messages
                    destination_address.clone()
                };

                // Only parse delivery_monitoring if explicitly present
                let delivery_monitoring = if block2.len() >= 18 {
                    let monitoring = &block2[17..18];
                    // Only set if it's a valid monitoring code (not a space or other character)
                    if monitoring
                        .chars()
                        .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
                    {
                        Some(monitoring.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Only parse obsolescence_period if explicitly present and delivery_monitoring exists
                let obsolescence_period = if delivery_monitoring.is_some() && block2.len() >= 21 {
                    Some(block2[18..21].to_string())
                } else {
                    None
                };

                Ok(ApplicationHeader::Input(InputApplicationHeader {
                    message_type,
                    destination_address,
                    receiver_bic,
                    priority,
                    delivery_monitoring,
                    obsolescence_period,
                }))
            }
            "O" => {
                // Output format: {2:O103HHMMYYYYMMDDDDDDDDDDDDDDNNNNSSSSSSYYYYMMDDHHMMP}
                // Components:
                // O (1) + message_type (3) + input_time (4) + mir (28) + output_date (6) + output_time (4) + priority (1)
                // MIR consists of: date (6) + lt_address (12) + session (4) + sequence (6) = 28 chars
                // Total: 1 + 3 + 4 + 28 + 6 + 4 = 46 characters minimum (priority optional)

                if block2.len() < 46 {
                    return Err(ParseError::InvalidBlockStructure {
                        block: "2".to_string(),
                        message: format!(
                            "Output Block 2 too short: expected at least 46 characters, got {}",
                            block2.len()
                        ),
                    });
                }

                // Parse Output format components according to SWIFT specification:
                let input_time = block2[4..8].to_string(); // HHMM

                // MIR (Message Input Reference) components
                let mir_date = block2[8..14].to_string(); // YYMMDD
                let mir_lt_address = block2[14..26].to_string(); // 12 characters (BIC8 + LT + Branch)
                let mir_session = block2[26..30].to_string(); // 4 digits
                let mir_sequence = block2[30..36].to_string(); // 6 digits

                let output_date = block2[36..42].to_string(); // YYMMDD
                let output_time = block2[42..46].to_string(); // HHMM

                let priority = if block2.len() >= 47 {
                    Some(block2[46..47].to_string())
                } else {
                    None
                };

                // Create MIR structure
                let mir = MessageInputReference {
                    date: mir_date,
                    lt_identifier: mir_lt_address.clone(),
                    branch_code: if mir_lt_address.len() >= 12 {
                        mir_lt_address[9..12].to_string()
                    } else {
                        "XXX".to_string()
                    },
                    session_number: mir_session,
                    sequence_number: mir_sequence,
                };

                Ok(ApplicationHeader::Output(OutputApplicationHeader {
                    message_type,
                    input_time,
                    mir,
                    output_date,
                    output_time,
                    priority,
                }))
            }
            _ => Err(ParseError::InvalidBlockStructure {
                block: "2".to_string(),
                message: format!(
                    "Invalid direction indicator: expected 'I' or 'O', got '{}'",
                    direction
                ),
            }),
        }
    }

    /// Get the message type regardless of direction
    pub fn message_type(&self) -> &str {
        match self {
            ApplicationHeader::Input(header) => &header.message_type,
            ApplicationHeader::Output(header) => &header.message_type,
        }
    }

    /// Get the priority if available
    pub fn priority(&self) -> Option<&str> {
        match self {
            ApplicationHeader::Input(header) => Some(&header.priority),
            ApplicationHeader::Output(header) => header.priority.as_deref(),
        }
    }
}

impl std::fmt::Display for ApplicationHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationHeader::Input(header) => {
                // Delegate to InputApplicationHeader's Display implementation
                write!(f, "{}", header)
            }
            ApplicationHeader::Output(header) => {
                // Output format: O + message_type + input_time + MIR + output_date + output_time + priority
                // MIR = date + lt_address + session + sequence
                let mut result = format!(
                    "O{}{}{}{}{}{}{}{}",
                    header.message_type,
                    header.input_time,
                    header.mir.date,
                    header.mir.lt_identifier,
                    header.mir.session_number,
                    header.mir.sequence_number,
                    header.output_date,
                    header.output_time,
                );

                if let Some(ref priority) = header.priority {
                    result.push_str(priority);
                }

                write!(f, "{result}")
            }
        }
    }
}

impl std::fmt::Display for InputApplicationHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ensure consistent formatting:
        // - message_type is always 3 digits
        // - destination_address must be exactly 12 characters
        // - priority is always 1 character

        // Ensure message_type is exactly 3 characters
        let message_type = format!(
            "{:0>3}",
            &self.message_type[..self.message_type.len().min(3)]
        );

        // Pad or truncate destination_address to exactly 12 characters
        let destination_address = if self.destination_address.len() > 12 {
            self.destination_address[..12].to_string()
        } else if self.destination_address.len() < 12 {
            format!("{:X<12}", self.destination_address)
        } else {
            self.destination_address.clone()
        };

        let mut result = format!("I{}{}{}", message_type, destination_address, self.priority);

        if let Some(ref delivery_monitoring) = self.delivery_monitoring {
            result.push_str(delivery_monitoring);
        }

        if let Some(ref obsolescence_period) = self.obsolescence_period {
            result.push_str(obsolescence_period);
        }

        write!(f, "{result}")
    }
}

impl std::fmt::Display for OutputApplicationHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!(
            "O{}{}{}{}{}{}{}{}",
            self.message_type,
            self.input_time,
            self.mir.date,
            self.mir.lt_identifier,
            self.mir.session_number,
            self.mir.sequence_number,
            self.output_date,
            self.output_time,
        );

        if let Some(ref priority) = self.priority {
            result.push_str(priority);
        }

        write!(f, "{result}")
    }
}

/// **Block 3: User Header**
///
/// Optional service tags and controls for SWIFT messages.
///
/// **Format:** `{3:{tag:value}{tag:value}...}`
/// **Common Tags:**
/// - **121:** UETR (UUID format) - Mandatory for SWIFT gpi
/// - **119:** Validation flag (STP, REMIT, COV, RFDD)
/// - **103:** Service identifier
/// - **108:** Message user reference
/// - **433/434:** Sanctions screening and payment controls
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
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
    #[serde(rename = "service_type_identifier")]
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

/// Balance checkpoint for Tag 423 (MIRS recovery)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct BalanceCheckpoint {
    /// Date (YYMMDD)
    pub date: String,
    /// Time (HHMMSS)
    pub time: String,
    /// Hundredths of second (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hundredths_of_second: Option<String>,
}

/// Message Input Reference (Tag 106, MIR format)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MessageInputReference {
    /// Date (YYMMDD)
    pub date: String,
    /// LT identifier (12 chars)
    pub lt_identifier: String,
    /// Branch code (3 chars)
    pub branch_code: String,
    /// Session number (4 digits)
    pub session_number: String,
    /// Sequence number (6 digits)
    pub sequence_number: String,
}

/// Payment release info for Tag 165 (FINInform)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct PaymentReleaseInfo {
    /// Code (3 chars)
    pub code: String,
    /// Additional info (max 34 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

/// Sanctions screening info for Tag 433
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct SanctionsScreeningInfo {
    /// Code word (AOK, FPO, NOK)
    pub code_word: String,
    /// Additional info (max 20 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

/// Payment controls info for Tag 434
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct PaymentControlsInfo {
    /// Code word (3 chars)
    pub code_word: String,
    /// Additional info (max 20 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl UserHeader {
    /// Parse user header from block 3 string using structured parsing
    pub fn parse(block3: &str) -> Result<Self> {
        let mut user_header = UserHeader::default();

        // Parse nested tags in format {tag:value}
        // Simple parsing for now - more sophisticated regex parsing can be added later
        if block3.contains("{103:")
            && let Some(start) = block3.find("{103:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.service_identifier = Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{113:")
            && let Some(start) = block3.find("{113:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.banking_priority = Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{108:")
            && let Some(start) = block3.find("{108:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.message_user_reference = Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{119:")
            && let Some(start) = block3.find("{119:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.validation_flag = Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{423:")
            && let Some(start) = block3.find("{423:")
            && let Some(end) = block3[start..].find('}')
        {
            let value = &block3[start + 5..start + end];
            user_header.balance_checkpoint = Self::parse_balance_checkpoint(value);
        }

        if block3.contains("{106:")
            && let Some(start) = block3.find("{106:")
            && let Some(end) = block3[start..].find('}')
        {
            let value = &block3[start + 5..start + end];
            user_header.message_input_reference = Self::parse_message_input_reference(value);
        }

        if block3.contains("{424:")
            && let Some(start) = block3.find("{424:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.related_reference = Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{111:")
            && let Some(start) = block3.find("{111:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.service_type_identifier = Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{121:")
            && let Some(start) = block3.find("{121:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.unique_end_to_end_reference =
                Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{115:")
            && let Some(start) = block3.find("{115:")
            && let Some(end) = block3[start..].find('}')
        {
            user_header.addressee_information = Some(block3[start + 5..start + end].to_string());
        }

        if block3.contains("{165:")
            && let Some(start) = block3.find("{165:")
            && let Some(end) = block3[start..].find('}')
        {
            let value = &block3[start + 5..start + end];
            user_header.payment_release_information = Self::parse_payment_release_info(value);
        }

        if block3.contains("{433:")
            && let Some(start) = block3.find("{433:")
            && let Some(end) = block3[start..].find('}')
        {
            let value = &block3[start + 5..start + end];
            user_header.sanctions_screening_info = Self::parse_sanctions_screening_info(value);
        }

        if block3.contains("{434:")
            && let Some(start) = block3.find("{434:")
            && let Some(end) = block3[start..].find('}')
        {
            let value = &block3[start + 5..start + end];
            user_header.payment_controls_info = Self::parse_payment_controls_info(value);
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

        if let Some(ref service_type_identifier) = self.service_type_identifier {
            result.push_str(&format!("{{111:{service_type_identifier}}}"));
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

/// **Block 5: Trailer**
///
/// Security and control information for message integrity and authentication.
///
/// **Format:** `{5:{tag:value}{tag:value}...}`
/// **Key Tags:**
/// - **CHK:** Checksum (12 hex chars) - Mandatory for integrity validation
/// - **MAC:** Message Authentication Code - Optional bilateral authentication
/// - **TNG:** Test and Training flag - Identifies test messages
/// - **PDM/PDE:** Possible duplicate detection tags
/// - **DLM:** Delayed message indicator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
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

/// Possible Duplicate Emission for PDE tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct PossibleDuplicateEmission {
    /// Time (HHMM)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Message Input Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_input_reference: Option<MessageInputReference>,
}

/// Message Reference for MRF tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MessageReference {
    /// Date (YYMMDD)
    pub date: String,
    /// Time (HHMM)
    pub full_time: String,
    /// Message Input Reference
    pub message_input_reference: MessageInputReference,
}

/// Possible Duplicate Message for PDM tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct PossibleDuplicateMessage {
    /// Time (HHMM)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Message Output Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_output_reference: Option<MessageOutputReference>,
}

/// Message Output Reference (MOR format)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct MessageOutputReference {
    /// Date (YYMMDD)
    pub date: String,
    /// LT identifier (12 chars)
    pub lt_identifier: String,
    /// Branch code (3 chars)
    pub branch_code: String,
    /// Session number (4 digits)
    pub session_number: String,
    /// Sequence number (6 digits)
    pub sequence_number: String,
}

/// System Originated Message for SYS tag
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct SystemOriginatedMessage {
    /// Time (HHMM)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Message Input Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_input_reference: Option<MessageInputReference>,
}

impl Trailer {
    /// Parse trailer from block 5 string using structured parsing
    pub fn parse(block5: &str) -> Result<Self> {
        let mut trailer = Trailer::default();

        // Extract common tags if present
        if block5.contains("{CHK:")
            && let Some(start) = block5.find("{CHK:")
            && let Some(end) = block5[start..].find('}')
        {
            trailer.checksum = Some(block5[start + 5..start + end].to_string());
        }

        if block5.contains("{TNG}") {
            trailer.test_and_training = Some(true);
        }

        if block5.contains("{DLM}") {
            trailer.delayed_message = Some(true);
        }

        if block5.contains("{MAC:")
            && let Some(start) = block5.find("{MAC:")
            && let Some(end) = block5[start..].find('}')
        {
            trailer.mac = Some(block5[start + 5..start + end].to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_header_input_parsing() {
        // Test Input message format parsing
        let block2 = "I103DEUTDEFFAXXXN";
        let header = ApplicationHeader::parse(block2).unwrap();

        match header {
            ApplicationHeader::Input(input) => {
                assert_eq!(input.message_type, "103");
                assert_eq!(input.destination_address, "DEUTDEFFAXXX");
                assert_eq!(input.receiver_bic, "DEUTDEFF");
                assert_eq!(input.priority, "N");
                assert_eq!(input.delivery_monitoring, None);
                assert_eq!(input.obsolescence_period, None);
            }
            ApplicationHeader::Output(_) => panic!("Expected Input header, got Output"),
        }
    }

    #[test]
    fn test_application_header_input_parsing_with_monitoring() {
        // Test Input message with delivery monitoring
        let block2 = "I103DEUTDEFFAXXXU3003";
        let header = ApplicationHeader::parse(block2).unwrap();

        match header {
            ApplicationHeader::Input(input) => {
                assert_eq!(input.message_type, "103");
                assert_eq!(input.destination_address, "DEUTDEFFAXXX");
                assert_eq!(input.receiver_bic, "DEUTDEFF");
                assert_eq!(input.priority, "U");
                assert_eq!(input.delivery_monitoring, Some("3".to_string()));
                assert_eq!(input.obsolescence_period, Some("003".to_string()));
            }
            ApplicationHeader::Output(_) => panic!("Expected Input header, got Output"),
        }
    }

    #[test]
    fn test_application_header_output_parsing() {
        // Test Output message format parsing - the exact case from the issue
        let block2 = "O1031535051028DEUTDEFFAXXX08264556280510281535N";
        let header = ApplicationHeader::parse(block2).unwrap();

        match header {
            ApplicationHeader::Output(output) => {
                assert_eq!(output.message_type, "103");
                assert_eq!(output.input_time, "1535");
                assert_eq!(output.output_date, "051028");
                assert_eq!(output.output_time, "1535");
                assert_eq!(output.priority, Some("N".to_string()));

                // Check MIR structure
                assert_eq!(output.mir.date, "051028");
                assert_eq!(output.mir.lt_identifier, "DEUTDEFFAXXX");
                assert_eq!(output.mir.branch_code, "XXX");
                assert_eq!(output.mir.session_number, "0826");
                assert_eq!(output.mir.sequence_number, "455628");
            }
            ApplicationHeader::Input(_) => panic!("Expected Output header, got Input"),
        }
    }

    #[test]
    fn test_application_header_output_parsing_different_message_type() {
        // Test another Output message format
        let block2 = "O2021245051028CHASUS33AXXX08264556280510281245U";
        let header = ApplicationHeader::parse(block2).unwrap();

        match header {
            ApplicationHeader::Output(output) => {
                assert_eq!(output.message_type, "202");
                assert_eq!(output.mir.lt_identifier, "CHASUS33AXXX");
                assert_eq!(output.priority, Some("U".to_string()));
            }
            ApplicationHeader::Input(_) => panic!("Expected Output header, got Input"),
        }
    }

    #[test]
    fn test_application_header_invalid_direction() {
        let block2 = "X103DEUTDEFFAXXXN";
        let result = ApplicationHeader::parse(block2);

        assert!(result.is_err());
        if let Err(ParseError::InvalidBlockStructure { message, .. }) = result {
            assert!(message.contains("Invalid direction indicator"));
        } else {
            panic!("Expected InvalidBlockStructure error");
        }
    }

    #[test]
    fn test_application_header_input_too_short() {
        let block2 = "I103DEUTDEF"; // Too short for Input format
        let result = ApplicationHeader::parse(block2);

        assert!(result.is_err());
    }

    #[test]
    fn test_application_header_output_too_short() {
        let block2 = "O103153505102"; // Too short for Output format (13 characters)
        let result = ApplicationHeader::parse(block2);

        assert!(result.is_err());
        if let Err(ParseError::InvalidBlockStructure { message, .. }) = result {
            // This will now hit the Output-specific check since initial check is for 4 chars
            assert!(message.contains("Output Block 2 too short: expected at least 46 characters"));
        } else {
            panic!("Expected InvalidBlockStructure error");
        }
    }

    #[test]
    fn test_application_header_output_minimum_length_but_still_too_short() {
        // This has 17 characters so it passes initial check but fails Output-specific check
        let block2 = "O10315350510280DE"; // 17 characters, but Output needs 46
        let result = ApplicationHeader::parse(block2);

        assert!(result.is_err());
        if let Err(ParseError::InvalidBlockStructure { message, .. }) = result {
            assert!(message.contains("Output Block 2 too short: expected at least 46 characters"));
        } else {
            panic!("Expected InvalidBlockStructure error");
        }
    }

    #[test]
    fn test_basic_header_parsing() {
        let block1 = "F01DEUTDEFFAXXX0000123456";
        let header = BasicHeader::parse(block1).unwrap();

        assert_eq!(header.application_id, "F");
        assert_eq!(header.service_id, "01");
        assert_eq!(header.logical_terminal, "DEUTDEFFAXXX");
        assert_eq!(header.sender_bic, "DEUTDEFF");
        assert_eq!(header.session_number, "0000");
        assert_eq!(header.sequence_number, "123456");
    }

    #[test]
    fn test_application_header_input_display() {
        let header = ApplicationHeader::Input(InputApplicationHeader {
            message_type: "103".to_string(),
            destination_address: "DEUTDEFFAXXX".to_string(),
            receiver_bic: "DEUTDEFF".to_string(),
            priority: "U".to_string(),
            delivery_monitoring: Some("3".to_string()),
            obsolescence_period: Some("003".to_string()),
        });

        assert_eq!(header.to_string(), "I103DEUTDEFFAXXXU3003");
    }

    #[test]
    fn test_application_header_output_display() {
        let mir = MessageInputReference {
            date: "051028".to_string(),
            lt_identifier: "DEUTDEFFAXXX".to_string(),
            branch_code: "XXX".to_string(),
            session_number: "0826".to_string(),
            sequence_number: "455628".to_string(),
        };

        let header = ApplicationHeader::Output(OutputApplicationHeader {
            message_type: "103".to_string(),
            input_time: "1535".to_string(),
            mir,
            output_date: "051028".to_string(),
            output_time: "1535".to_string(),
            priority: Some("N".to_string()),
        });

        assert_eq!(
            header.to_string(),
            "O1031535051028DEUTDEFFAXXX08264556280510281535N"
        );
    }

    #[test]
    fn test_application_header_helper_methods() {
        let input_header = ApplicationHeader::Input(InputApplicationHeader {
            message_type: "103".to_string(),
            destination_address: "DEUTDEFFAXXX".to_string(),
            receiver_bic: "DEUTDEFF".to_string(),
            priority: "U".to_string(),
            delivery_monitoring: None,
            obsolescence_period: None,
        });

        assert_eq!(input_header.message_type(), "103");
        assert_eq!(input_header.priority(), Some("U"));

        let mir = MessageInputReference {
            date: "051028".to_string(),
            lt_identifier: "DEUTDEFFAXXX".to_string(),
            branch_code: "XXX".to_string(),
            session_number: "0826".to_string(),
            sequence_number: "455628".to_string(),
        };

        let output_header = ApplicationHeader::Output(OutputApplicationHeader {
            message_type: "202".to_string(),
            input_time: "1535".to_string(),
            mir,
            output_date: "051028".to_string(),
            output_time: "1535".to_string(),
            priority: Some("N".to_string()),
        });

        assert_eq!(output_header.message_type(), "202");
        assert_eq!(output_header.priority(), Some("N"));
    }
}
