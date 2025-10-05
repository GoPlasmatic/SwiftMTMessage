use super::swift_utils::{parse_max_length, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 21 NoOption: Transaction Reference**
///
/// Basic transaction reference for customer payment instructions.
///
/// **Format:** `16x` (max 16 chars)
/// **Constraints:** No leading/trailing slashes, no consecutive slashes
///
/// **Example:**
/// ```text
/// :21:REF20240719001
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21NoOption {
    /// Transaction reference (max 16 chars, no slashes at start/end or consecutive)
    pub reference: String,
}

impl SwiftField for Field21NoOption {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Parse the reference with max length of 16
        let reference = parse_max_length(input, 16, "Field 21 reference")?;

        // Validate SWIFT character set
        parse_swift_chars(&reference, "Field 21 reference")?;

        // Additional validation: no leading/trailing slashes
        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21 reference cannot start or end with '/'".to_string(),
            });
        }

        // Additional validation: no consecutive slashes
        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21 reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21NoOption { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21:{}", self.reference)
    }
}

/// **Field 21C: Customer-Specific Reference**
///
/// Extended reference for customer-specific transactions and treasury operations.
///
/// **Format:** `35x` (max 35 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21C {
    /// Customer reference (max 35 chars)
    pub reference: String,
}

impl SwiftField for Field21C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 35, "Field 21C reference")?;
        parse_swift_chars(&reference, "Field 21C reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21C reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21C reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21C { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21C:{}", self.reference)
    }
}

/// **Field 21D: Deal Reference**
///
/// Deal reference for treasury and money market transactions.
///
/// **Format:** `35x` (max 35 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21D {
    /// Deal reference (max 35 chars)
    pub reference: String,
}

impl SwiftField for Field21D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 35, "Field 21D reference")?;
        parse_swift_chars(&reference, "Field 21D reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21D reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21D reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21D { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21D:{}", self.reference)
    }
}

/// **Field 21E: Related Reference**
///
/// Reference to related transaction or instruction for linking operations.
///
/// **Format:** `35x` (max 35 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21E {
    /// Related reference (max 35 chars)
    pub reference: String,
}

impl SwiftField for Field21E {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 35, "Field 21E reference")?;
        parse_swift_chars(&reference, "Field 21E reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21E reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21E reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21E { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21E:{}", self.reference)
    }
}

/// **Field 21F: File Reference**
///
/// File reference for batch payment operations and MT102 messages.
///
/// **Format:** `16x` (max 16 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21F {
    /// File reference (max 16 chars)
    pub reference: String,
}

impl SwiftField for Field21F {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 16, "Field 21F reference")?;
        parse_swift_chars(&reference, "Field 21F reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21F reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21F reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21F { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21F:{}", self.reference)
    }
}

/// **Field 21R: Related File Reference**
///
/// Reference to related file for linking batch operations.
///
/// **Format:** `16x` (max 16 chars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field21R {
    /// Related file reference (max 16 chars)
    pub reference: String,
}

impl SwiftField for Field21R {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let reference = parse_max_length(input, 16, "Field 21R reference")?;
        parse_swift_chars(&reference, "Field 21R reference")?;

        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 21R reference cannot start or end with '/'".to_string(),
            });
        }

        if reference.contains("//") {
            return Err(ParseError::InvalidFormat {
                message: "Field 21R reference cannot contain consecutive slashes '//'".to_string(),
            });
        }

        Ok(Field21R { reference })
    }

    fn to_swift_string(&self) -> String {
        format!(":21R:{}", self.reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field21_no_option() {
        let field = Field21NoOption::parse("REF20240719001").unwrap();
        assert_eq!(field.reference, "REF20240719001");
        assert_eq!(field.to_swift_string(), ":21:REF20240719001");

        // Test max length
        assert!(Field21NoOption::parse("1234567890ABCDEF").is_ok());
        assert!(Field21NoOption::parse("1234567890ABCDEFG").is_err());

        // Test slash validation
        assert!(Field21NoOption::parse("/REF123").is_err());
        assert!(Field21NoOption::parse("REF123/").is_err());
        assert!(Field21NoOption::parse("REF//123").is_err());
    }

    #[test]
    fn test_field21c() {
        let field = Field21C::parse("TREASURY/SWAP/2024/07/19/001").unwrap();
        assert_eq!(field.reference, "TREASURY/SWAP/2024/07/19/001");

        // Test max length (35 chars)
        let long_ref = "12345678901234567890123456789012345";
        assert!(Field21C::parse(long_ref).is_ok());
        assert!(Field21C::parse(&format!("{}X", long_ref)).is_err());
    }

    #[test]
    fn test_field21d() {
        let field = Field21D::parse("FX-DEAL-20240719-EUR-USD").unwrap();
        assert_eq!(field.reference, "FX-DEAL-20240719-EUR-USD");
        assert_eq!(field.to_swift_string(), ":21D:FX-DEAL-20240719-EUR-USD");
    }

    #[test]
    fn test_field21e() {
        let field = Field21E::parse("ORIGINAL-REF-123456").unwrap();
        assert_eq!(field.reference, "ORIGINAL-REF-123456");
    }

    #[test]
    fn test_field21f() {
        let field = Field21F::parse("BATCH-20240719").unwrap();
        assert_eq!(field.reference, "BATCH-20240719");

        // Test 16 char limit
        assert!(Field21F::parse("1234567890ABCDEF").is_ok());
        assert!(Field21F::parse("1234567890ABCDEFG").is_err());
    }

    #[test]
    fn test_field21r() {
        let field = Field21R::parse("FILE-REF-001").unwrap();
        assert_eq!(field.reference, "FILE-REF-001");
        assert_eq!(field.to_swift_string(), ":21R:FILE-REF-001");
    }
}
