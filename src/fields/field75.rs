use super::swift_utils::parse_swift_chars;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

/// **Field 75: Queries**
///
/// Query information in MT n95 series messages for requesting clarification,
/// status updates, or additional details on transactions.
///
/// **Format:** `6*35x` (max 6 lines, 35 chars each)
/// **Used in:** MT 195, 295, 395, 495, 595, 695, 795, 895, 995 (query messages)
///
/// **Example:**
/// ```text
/// :75:QUERY: TRANSACTION STATUS
/// REF: MT103 20240719001
/// PLEASE CONFIRM
/// ```

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct Field75 {
    /// Query information (max 6 lines, 35 chars each)
    pub information: Vec<String>,
}

impl SwiftField for Field75 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut information = Vec::new();

        // Parse up to 6 lines of 35 characters each
        for line in input.lines() {
            if information.len() >= 6 {
                break;
            }

            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!(
                        "Field75 line cannot exceed 35 characters, found {}",
                        line.len()
                    ),
                });
            }

            // Validate SWIFT character set
            parse_swift_chars(line, "Field75 line")?;
            information.push(line.to_string());
        }

        if information.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field75 requires at least one line of information".to_string(),
            });
        }

        Ok(Field75 { information })
    }

    fn to_swift_string(&self) -> String {
        let content = self.information.join("\n");
        format!(":75:{}", content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field75_parse() {
        let field =
            Field75::parse("QUERY: TRANSACTION STATUS\nREF: MT103 20240719001\nPLEASE CONFIRM")
                .unwrap();
        assert_eq!(field.information.len(), 3);
        assert_eq!(field.information[0], "QUERY: TRANSACTION STATUS");
        assert_eq!(field.information[1], "REF: MT103 20240719001");
        assert_eq!(field.information[2], "PLEASE CONFIRM");

        // Single line
        let field = Field75::parse("STATUS REQUEST").unwrap();
        assert_eq!(field.information.len(), 1);
        assert_eq!(field.information[0], "STATUS REQUEST");
    }

    #[test]
    fn test_field75_to_swift_string() {
        let field = Field75 {
            information: vec![
                "QUERY: TRANSACTION STATUS".to_string(),
                "REF: MT103 20240719001".to_string(),
                "PLEASE CONFIRM".to_string(),
            ],
        };
        assert_eq!(
            field.to_swift_string(),
            ":75:QUERY: TRANSACTION STATUS\nREF: MT103 20240719001\nPLEASE CONFIRM"
        );
    }

    #[test]
    fn test_field75_parse_invalid() {
        // Empty input
        assert!(Field75::parse("").is_err());

        // Line too long (over 35 characters)
        assert!(
            Field75::parse(
                "THIS LINE IS DEFINITELY TOO LONG AND EXCEEDS THE THIRTY FIVE CHARACTER LIMIT"
            )
            .is_err()
        );
    }
}
