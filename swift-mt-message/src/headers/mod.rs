use crate::errors::{ParseError, Result};
use serde::{Deserialize, Serialize};

/// Basic Header (Block 1) - Application and service identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BasicHeader {
    /// Application identifier (F = FIN application)
    pub application_id: String,

    /// Service identifier (01 = FIN)
    pub service_id: String,

    /// Logical Terminal (LT) address (12 characters)
    pub logical_terminal: String,

    /// Session number (4 digits)
    pub session_number: String,

    /// Sequence number (6 digits)
    pub sequence_number: String,
}

impl BasicHeader {
    /// Parse basic header from block 1 string
    pub fn parse(block1: &str) -> Result<Self> {
        if block1.len() < 14 {
            return Err(ParseError::InvalidBlockStructure {
                message: format!(
                    "Block 1 too short: expected at least 14 characters, got {}",
                    block1.len()
                ),
            });
        }

        let application_id = block1[0..1].to_string();
        let service_id = block1[1..3].to_string();

        // Extract logical terminal - usually 11-12 characters
        let mut logical_terminal = String::new();
        let mut pos = 3;

        // Find the end of the logical terminal (where digits start for session number)
        while pos < block1.len() && !block1.chars().nth(pos).unwrap_or('0').is_ascii_digit() {
            logical_terminal.push(block1.chars().nth(pos).unwrap());
            pos += 1;
        }

        // Remaining characters are session and sequence numbers
        let remaining = &block1[pos..];
        let session_number = if remaining.len() >= 4 {
            remaining[0..4].to_string()
        } else {
            remaining.to_string()
        };

        let sequence_number = if remaining.len() > 4 {
            remaining[4..].to_string()
        } else {
            String::new()
        };

        Ok(BasicHeader {
            application_id,
            service_id,
            logical_terminal,
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

/// Application Header (Block 2) - Message information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplicationHeader {
    /// Direction (I = Input, O = Output)
    pub direction: String,

    /// Message type (e.g., "103", "202")
    pub message_type: String,

    /// Destination address (12 characters)
    pub destination_address: String,

    /// Priority (U = Urgent, N = Normal, S = System)
    pub priority: String,

    /// Delivery monitoring (1 = Non-delivery notification, 3 = Delivery notification)
    pub delivery_monitoring: Option<String>,

    /// Obsolescence period (3 digits, only for certain message types)
    pub obsolescence_period: Option<String>,
}

impl ApplicationHeader {
    /// Parse application header from block 2 string
    pub fn parse(block2: &str) -> Result<Self> {
        if block2.len() < 17 {
            return Err(ParseError::InvalidBlockStructure {
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

        Ok(ApplicationHeader {
            direction,
            message_type,
            destination_address,
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

        write!(f, "{}", result)
    }
}

/// User Header (Block 3) structure based on SWIFT MT standards
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

    /// Tag 423 - Balance checkpoint date and time (YYMMDDHHMMSS[ss]) - Optional (MIRS only)
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

    /// Tag 433 - Sanctions screening information (3!a/[20x]) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sanctions_screening_info: Option<SanctionsScreeningInfo>,

    /// Tag 434 - Payment controls information (3!a/[20x]) - Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_controls_info: Option<PaymentControlsInfo>,
}

/// Balance checkpoint structure for Tag 423
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BalanceCheckpoint {
    pub date: String,                         // YYMMDD
    pub time: String,                         // HHMMSS
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
    pub code: String,                    // 3!c
    pub additional_info: Option<String>, // 34x (optional)
}

/// Sanctions screening information structure for Tag 433
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SanctionsScreeningInfo {
    pub code_word: String,               // 3!a (AOK, FPO, NOK)
    pub additional_info: Option<String>, // 20x (optional)
}

/// Payment controls information structure for Tag 434
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentControlsInfo {
    pub code_word: String,               // 3!a
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
            result.push_str(&format!("{{103:{}}}", service_id));
        }

        if let Some(ref banking_priority) = self.banking_priority {
            result.push_str(&format!("{{113:{}}}", banking_priority));
        }

        if let Some(ref message_user_ref) = self.message_user_reference {
            result.push_str(&format!("{{108:{}}}", message_user_ref));
        }

        if let Some(ref validation_flag) = self.validation_flag {
            result.push_str(&format!("{{119:{}}}", validation_flag));
        }

        if let Some(ref unique_end_to_end_ref) = self.unique_end_to_end_reference {
            result.push_str(&format!("{{121:{}}}", unique_end_to_end_ref));
        }

        if let Some(ref service_type_id) = self.service_type_identifier {
            result.push_str(&format!("{{111:{}}}", service_type_id));
        }

        if let Some(ref payment_controls) = self.payment_controls_info {
            let mut value = payment_controls.code_word.clone();
            if let Some(ref additional) = payment_controls.additional_info {
                value.push('/');
                value.push_str(additional);
            }
            result.push_str(&format!("{{434:{}}}", value));
        }

        if let Some(ref payment_release) = self.payment_release_information {
            let mut value = payment_release.code.clone();
            if let Some(ref additional) = payment_release.additional_info {
                value.push('/');
                value.push_str(additional);
            }
            result.push_str(&format!("{{165:{}}}", value));
        }

        if let Some(ref sanctions) = self.sanctions_screening_info {
            let mut value = sanctions.code_word.clone();
            if let Some(ref additional) = sanctions.additional_info {
                value.push('/');
                value.push_str(additional);
            }
            result.push_str(&format!("{{433:{}}}", value));
        }

        write!(f, "{}", result)
    }
}

/// Trailer (Block 5) structure based on SWIFT MT standards
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
    pub time: Option<String>, // HHMM (optional)
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
    pub time: Option<String>, // HHMM (optional)
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
    pub time: Option<String>, // HHMM (optional)
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
            result.push_str(&format!("{{CHK:{}}}", checksum));
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
            result.push_str(&format!("{{MAC:{}}}", mac));
        }

        write!(f, "{}", result)
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
