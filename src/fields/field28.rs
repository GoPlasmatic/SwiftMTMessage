use super::swift_utils::{parse_swift_digits, split_at_first};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 28: Statement Number / Sequence Number / Message Index**
///
/// ## Purpose
/// Provides statement numbering, sequence identification, and message indexing capabilities
/// for financial statements, batch operations, and multi-part message communication.
/// This field enables proper ordering, continuation tracking, and completeness verification
/// of related messages or statement sequences.
///
/// ## Format Variants
/// - **Field 28**: `5n[/2n]` - Statement number with optional sequence
/// - **Field 28C**: `5n[/5n]` - Statement number with extended sequence
/// - **Field 28D**: `5n/5n` - Message index and total count
///
/// ## Presence
/// - **Status**: Mandatory in statement messages and batch operations
/// - **Swift Error Codes**: T40 (invalid number), T51 (format violation)
/// - **Usage Context**: Statement numbering and message sequencing
///
/// ## Usage Rules
/// - **Sequential Numbering**: Numbers must follow logical sequence for statements
/// - **Index Validation**: Message index must not exceed total count in Field 28D
/// - **Completeness**: Enables verification that all messages/statements received
/// - **Ordering**: Facilitates proper chronological ordering of related messages
///
/// ## Network Validation Rules
/// - **Positive Numbers**: All numeric values must be greater than zero
/// - **Range Validation**: Numbers must be within reasonable business limits
/// - **Format Compliance**: Must follow exact numeric format specifications
/// - **Logic Validation**: Index must not exceed total in indexed variants
///
/// ## Field Variants and Usage
///
/// ### Field 28 - Basic Statement/Sequence Number
/// - **Format**: `5n[/2n]`
/// - **Usage**: Account statements with optional sequence numbers
/// - **Statement Number**: Primary identifier for statement period
/// - **Sequence Number**: Sub-sequence within statement period
///
/// ### Field 28C - Extended Statement/Sequence Number
/// - **Format**: `5n[/5n]`
/// - **Usage**: Extended numbering for complex statement structures
/// - **Enhanced Range**: Larger sequence number capacity
/// - **Complex Statements**: Multi-part statements with extensive sequences
///
/// ### Field 28D - Message Index/Total
/// - **Format**: `5n/5n`
/// - **Usage**: Batch message indexing for completeness verification
/// - **Index Number**: Current message position in sequence
/// - **Total Count**: Complete count of messages in batch
/// - **Verification**: Enables receiver to verify all messages received
///
/// ## Business Context
/// - **Statement Management**: Systematic numbering of account statements
/// - **Batch Processing**: Sequencing multiple related transactions
/// - **Audit Trail**: Maintaining proper sequence records for compliance
/// - **Message Integrity**: Ensuring complete message set delivery
///
/// ## Examples
/// ```logic
/// :28:12345              // Statement 12345 (no sequence)
/// :28:12345/01           // Statement 12345, sequence 1
/// :28C:98765/00123       // Extended statement with sequence
/// :28D:001/010           // Message 1 of 10 total
/// :28D:010/010           // Final message (10 of 10)
/// ```
///
/// ## Statement Sequencing Logic
/// - **Daily Statements**: Incremental numbering by business day
/// - **Monthly Statements**: Period-based numbering with daily sequences
/// - **Special Statements**: Ad-hoc numbering for specific requirements
/// - **Continuation**: Sequence numbers for multi-part statements
///
/// ## Batch Message Processing
/// - **Transmission Order**: Sequential transmission of indexed messages
/// - **Completeness Check**: Verification all messages received
/// - **Error Recovery**: Re-transmission of missing message indices
/// - **Processing Logic**: Ordered processing based on index sequence
///
/// ## Regional Considerations
/// - **European Standards**: SEPA statement numbering requirements
/// - **US Banking**: Federal Reserve statement sequence standards
/// - **Asian Markets**: Local statement numbering conventions
/// - **International**: Cross-border statement coordination
///
/// ## Error Prevention
/// - **Number Validation**: Verify numbers are positive and within range
/// - **Sequence Logic**: Ensure logical progression of sequence numbers
/// - **Index Validation**: Confirm message index does not exceed total
/// - **Completeness Check**: Verify all expected messages received
///
/// ## Related Fields
/// - **Field 60**: Opening Balance (statement start information)
/// - **Field 62**: Closing Balance (statement end information)
/// - **Field 64**: Closing Available Balance (additional statement data)
/// - **Block Headers**: Message timestamps and references
///
/// ## Processing Applications
/// - **MT940**: Customer Statement Message (Field 28)
/// - **MT942**: Interim Transaction Report (Field 28C)
/// - **MT101**: Request for Transfer (Field 28D)
/// - **MT102**: Multiple Customer Credit Transfer (Field 28D)
///
/// ## STP Compliance
/// - **Automated Sequencing**: System-generated sequence numbers for STP
/// - **Integrity Validation**: Automated completeness checking
/// - **Exception Handling**: Missing sequence detection and alerts
/// - **Quality Control**: Real-time sequence validation
///
/// ## Compliance and Audit
/// - **Regulatory Reporting**: Sequential statement reporting requirements
/// - **Audit Trail**: Maintaining complete sequence records
/// - **Record Keeping**: Statement number preservation for regulatory periods
/// - **Investigation Support**: Sequence-based transaction reconstruction
///
/// ## See Also
/// - Swift FIN User Handbook: Statement Numbering Standards
/// - MT940/942 Guidelines: Statement Sequence Requirements
/// - Batch Processing Standards: Message Indexing Specifications
/// - Regulatory Guidelines: Statement Numbering Compliance
///   **Field 28: Basic Statement Number/Sequence Number**
///
/// Basic statement numbering with optional sequence for account statements
/// and transaction reports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field28 {
    /// Statement number
    ///
    /// Format: 5n - Up to 5 digits (1-99999)
    /// Primary identifier for statement period or report
    pub statement_number: u32,

    /// Optional sequence number
    ///
    /// Format: [/2n] - Optional 1-2 digits after slash (1-99)
    /// Used for multi-part statements within same period
    pub sequence_number: Option<u8>,
}

impl SwiftField for Field28 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let (statement_str, sequence_str) = split_at_first(input, '/');

        // Parse statement number (5n)
        if statement_str.len() > 5 {
            return Err(ParseError::InvalidFormat {
                message: "Statement number must be at most 5 digits".to_string(),
            });
        }

        parse_swift_digits(&statement_str, "Statement number")?;
        let statement_number: u32 =
            statement_str
                .parse()
                .map_err(|_| ParseError::InvalidFormat {
                    message: "Invalid statement number".to_string(),
                })?;

        // Parse optional sequence number ([/2n])
        let sequence_number = if let Some(seq_str) = sequence_str {
            if seq_str.len() > 2 {
                return Err(ParseError::InvalidFormat {
                    message: "Sequence number must be at most 2 digits".to_string(),
                });
            }
            parse_swift_digits(&seq_str, "Sequence number")?;
            let seq: u8 = seq_str.parse().map_err(|_| ParseError::InvalidFormat {
                message: "Invalid sequence number".to_string(),
            })?;
            Some(seq)
        } else {
            None
        };

        Ok(Field28 {
            statement_number,
            sequence_number,
        })
    }

    fn to_swift_string(&self) -> String {
        if let Some(seq) = self.sequence_number {
            format!(":28:{}/{:02}", self.statement_number, seq)
        } else {
            format!(":28:{}", self.statement_number)
        }
    }
}

///   **Field 28C: Extended Statement Number/Sequence Number**
///
/// Extended statement numbering with larger sequence capacity for
/// complex statement structures and detailed transaction reports.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field28C {
    /// Statement number
    ///
    /// Format: 5n - Up to 5 digits (1-99999)
    /// Primary identifier for statement period
    pub statement_number: u32,

    /// Optional extended sequence number
    ///
    /// Format: [/5n] - Optional 1-5 digits after slash (1-99999)
    /// Enhanced sequence capacity for complex multi-part statements
    pub sequence_number: Option<u32>,
}

impl SwiftField for Field28C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let (statement_str, sequence_str) = split_at_first(input, '/');

        // Parse statement number (5n)
        if statement_str.len() > 5 {
            return Err(ParseError::InvalidFormat {
                message: "Statement number must be at most 5 digits".to_string(),
            });
        }

        parse_swift_digits(&statement_str, "Statement number")?;
        let statement_number: u32 =
            statement_str
                .parse()
                .map_err(|_| ParseError::InvalidFormat {
                    message: "Invalid statement number".to_string(),
                })?;

        // Parse optional sequence number ([/5n])
        let sequence_number = if let Some(seq_str) = sequence_str {
            if seq_str.len() > 5 {
                return Err(ParseError::InvalidFormat {
                    message: "Sequence number must be at most 5 digits".to_string(),
                });
            }
            parse_swift_digits(&seq_str, "Sequence number")?;
            let seq: u32 = seq_str.parse().map_err(|_| ParseError::InvalidFormat {
                message: "Invalid sequence number".to_string(),
            })?;
            Some(seq)
        } else {
            None
        };

        Ok(Field28C {
            statement_number,
            sequence_number,
        })
    }

    fn to_swift_string(&self) -> String {
        if let Some(seq) = self.sequence_number {
            format!(":28C:{}/{}", self.statement_number, seq)
        } else {
            format!(":28C:{}", self.statement_number)
        }
    }
}

///   **Field 28D: Message Index/Total**
///
/// Message indexing for batch operations enabling completeness verification
/// and proper sequencing of related messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field28D {
    /// Message index (current position)
    ///
    /// Format: 5n - Current message number in sequence (1-99999)
    /// Must not exceed total count, enables ordering verification
    pub index: u32,

    /// Total message count
    ///
    /// Format: /5n - Complete count of messages in batch (1-99999)
    /// Enables receiver to verify all messages received
    pub total: u32,
}

impl SwiftField for Field28D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let (index_str, total_str) = split_at_first(input, '/');

        // Parse index number (5n)
        if index_str.len() > 5 {
            return Err(ParseError::InvalidFormat {
                message: "Index number must be at most 5 digits".to_string(),
            });
        }

        parse_swift_digits(&index_str, "Index number")?;
        let index: u32 = index_str.parse().map_err(|_| ParseError::InvalidFormat {
            message: "Invalid index number".to_string(),
        })?;

        // Parse total count (required for Field28D)
        let total_str = total_str.ok_or_else(|| ParseError::InvalidFormat {
            message: "Field28D requires both index and total separated by '/'".to_string(),
        })?;

        if total_str.len() > 5 {
            return Err(ParseError::InvalidFormat {
                message: "Total count must be at most 5 digits".to_string(),
            });
        }

        parse_swift_digits(&total_str, "Total count")?;
        let total: u32 = total_str.parse().map_err(|_| ParseError::InvalidFormat {
            message: "Invalid total count".to_string(),
        })?;

        // Validate that index doesn't exceed total
        if index > total {
            return Err(ParseError::InvalidFormat {
                message: format!("Index {} cannot exceed total {}", index, total),
            });
        }

        if index == 0 || total == 0 {
            return Err(ParseError::InvalidFormat {
                message: "Index and total must be greater than zero".to_string(),
            });
        }

        Ok(Field28D { index, total })
    }

    fn to_swift_string(&self) -> String {
        format!(":28D:{:03}/{:03}", self.index, self.total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field28_parse() {
        // Test with sequence number
        let field = Field28::parse("12345/67").unwrap();
        assert_eq!(field.statement_number, 12345);
        assert_eq!(field.sequence_number, Some(67));

        // Test without sequence number
        let field = Field28::parse("12345").unwrap();
        assert_eq!(field.statement_number, 12345);
        assert_eq!(field.sequence_number, None);

        // Test with single digit sequence
        let field = Field28::parse("123/5").unwrap();
        assert_eq!(field.statement_number, 123);
        assert_eq!(field.sequence_number, Some(5));
    }

    #[test]
    fn test_field28c_parse() {
        // Test with extended sequence number
        let field = Field28C::parse("12345/99999").unwrap();
        assert_eq!(field.statement_number, 12345);
        assert_eq!(field.sequence_number, Some(99999));

        // Test without sequence number
        let field = Field28C::parse("12345").unwrap();
        assert_eq!(field.statement_number, 12345);
        assert_eq!(field.sequence_number, None);
    }

    #[test]
    fn test_field28d_parse() {
        // Test valid index/total
        let field = Field28D::parse("001/010").unwrap();
        assert_eq!(field.index, 1);
        assert_eq!(field.total, 10);

        // Test another valid case
        let field = Field28D::parse("5/5").unwrap();
        assert_eq!(field.index, 5);
        assert_eq!(field.total, 5);
    }

    #[test]
    fn test_field28_to_swift_string() {
        let field = Field28 {
            statement_number: 12345,
            sequence_number: Some(67),
        };
        assert_eq!(field.to_swift_string(), ":28:12345/67");

        let field = Field28 {
            statement_number: 12345,
            sequence_number: None,
        };
        assert_eq!(field.to_swift_string(), ":28:12345");
    }

    #[test]
    fn test_field28c_to_swift_string() {
        let field = Field28C {
            statement_number: 12345,
            sequence_number: Some(99999),
        };
        assert_eq!(field.to_swift_string(), ":28C:12345/99999");
    }

    #[test]
    fn test_field28d_to_swift_string() {
        let field = Field28D {
            index: 1,
            total: 10,
        };
        assert_eq!(field.to_swift_string(), ":28D:001/010");
    }

    #[test]
    fn test_field28d_validation_errors() {
        // Index exceeds total
        assert!(Field28D::parse("11/10").is_err());

        // Zero values
        assert!(Field28D::parse("0/10").is_err());
        assert!(Field28D::parse("1/0").is_err());

        // Missing total
        assert!(Field28D::parse("5").is_err());
    }
}
