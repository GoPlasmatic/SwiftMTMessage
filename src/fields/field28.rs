use super::swift_utils::{parse_swift_digits, split_at_first};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 28: Statement Number/Sequence Number**
///
/// Statement numbering with optional sequence for account statements.
///
/// **Format:** `5n[/2n]` (statement number + optional sequence)
///
/// **Example:**
/// ```text
/// :28:12345/01
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field28 {
    /// Statement number (up to 5 digits)
    pub statement_number: u32,

    /// Optional sequence number (up to 2 digits)
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

/// **Field 28C: Extended Statement Number/Sequence Number**
///
/// Extended statement numbering with larger sequence capacity.
///
/// **Format:** `5n[/5n]` (statement number + optional extended sequence)
///
/// **Example:**
/// ```text
/// :28C:98765/00123
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field28C {
    /// Statement number (up to 5 digits)
    pub statement_number: u32,

    /// Optional extended sequence number (up to 5 digits)
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

/// **Field 28D: Message Index/Total**
///
/// Message indexing for batch operations and completeness verification.
///
/// **Format:** `5n/5n` (index/total)
/// **Constraints:** Index must not exceed total
///
/// **Example:**
/// ```text
/// :28D:001/010
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field28D {
    /// Message index (current position, up to 5 digits)
    pub index: u32,

    /// Total message count (up to 5 digits)
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
