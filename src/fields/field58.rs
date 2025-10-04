use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 58: Beneficiary Institution**
///
/// ## Purpose
/// Specifies the ultimate recipient institution of the funds being transferred in specialized
/// payment scenarios. This field identifies the final institutional beneficiary when the
/// payment is destined for a financial institution rather than a customer account.
/// Used in institutional transfers, central bank operations, and specialized financial
/// market transactions where the beneficiary is itself a financial institution.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured institutional beneficiary
/// - **Option D**: Party identifier with name/address - detailed institutional beneficiary
///
/// ## Business Context Applications
/// - **Institutional Transfers**: Payments between financial institutions
/// - **Central Bank Operations**: Transfers to/from central banks and monetary authorities
/// - **Market Infrastructure**: Payments to clearing houses, settlement systems
/// - **Correspondent Banking**: Institutional correspondent relationship transfers
///
/// ## Usage Rules and Conditions
/// - **Mandatory Context**: Required when beneficiary is a financial institution
/// - **Option A Preference**: Option A must be used whenever possible
/// - **Option D Exception**: Only in exceptional circumstances or regulatory requirements
/// - **MT 200/201 Consistency**: Must match Field 52A content if from MT 200/201 transfer
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Institutional Status**: Beneficiary must be recognized financial institution
/// - **Service Capability**: Institution must be capable of receiving institutional transfers
/// - **Regulatory Compliance**: Must meet regulatory requirements for institutional recipients
///
/// ## Extended Clearing Codes (Option D)
/// ### Specialized Institution Codes
/// - **CH**: CHIPS Universal Identifier - 6!n format for CHIPS participants
/// - **CP**: CHIPS Participant - Direct CHIPS participation identifier
/// - **FW**: Fedwire Routing - 9!n format for Federal Reserve routing
/// - **RU**: Russian Central Bank - Russian Federation central bank identifier
/// - **SW**: Swiss Clearing - Swiss national clearing system identifier
///
/// ## See Also
/// - Swift FIN User Handbook: Beneficiary Institution Specifications
/// - Central Bank Guidelines: Institutional Transfer Requirements
/// - Financial Market Infrastructure: Institutional Settlement Standards
/// - Regulatory Framework: Institutional Transfer Compliance
///
///   **Field 58A: Beneficiary Institution (BIC with Party Identifier)**
///
/// Structured institutional beneficiary identification using BIC code with optional party identifier.
/// Preferred option for institutional transfers and financial institution beneficiaries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field58A {
    /// Optional party identifier for institutional account or system reference
    ///
    /// Format: [/1!a][/34x] - Single character code + up to 34 character identifier
    /// Used for institutional account identification and system-specific routing
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the beneficiary institution
    ///
    /// Format: 4!a2!a2!c[3!c] - 8 or 11 character BIC code
    /// Must be registered financial institution capable of receiving institutional transfers
    pub bic: String,
}

///   **Field 58D: Beneficiary Institution (Party Identifier with Name and Address)**
///
/// Detailed institutional beneficiary identification with full name and address information.
/// Used only in exceptional circumstances when structured BIC identification is not available.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field58D {
    /// Optional party identifier for institutional system reference
    ///
    /// Format: [/1!a][/34x] - Single character code + up to 34 character identifier
    /// May contain extended clearing codes: CH (CHIPS), CP (CHIPS Participant),
    /// FW (Fedwire), RU (Russian Central Bank), SW (Swiss Clearing)
    pub party_identifier: Option<String>,

    /// Name and address of the beneficiary institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field58 {
    #[serde(rename = "58A")]
    A(Field58A),
    #[serde(rename = "58D")]
    D(Field58D),
}

impl SwiftField for Field58A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        let mut party_identifier = None;
        let mut bic_line_idx = 0;

        // Check for optional party identifier on first line
        if !lines.is_empty() && lines[0].starts_with('/') {
            party_identifier = Some(lines[0].to_string());
            bic_line_idx = 1;
        }

        // Ensure we have a BIC line
        if bic_line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 58A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field58A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = ":58A:".to_string();

        if let Some(ref party_id) = self.party_identifier {
            if !party_id.starts_with('/') {
                result.push('/');
            }
            result.push_str(party_id);
            result.push('\n');
        }

        result.push_str(&self.bic);
        result
    }
}

impl SwiftField for Field58D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let mut lines = input.lines().collect::<Vec<_>>();
        let mut party_identifier = None;

        // Check if first line has party identifier
        if let Some(first_line) = lines.first()
            && first_line.starts_with('/')
        {
            // Find where party identifier ends and name starts
            if let Some(slash_end) =
                first_line[1..].find(|c: char| !c.is_alphanumeric() && c != '/')
            {
                let party_part = &first_line[..slash_end + 1];
                party_identifier = Some(party_part.to_string());

                // Update first line to remove party identifier
                let remaining = &first_line[slash_end + 1..];
                lines[0] = remaining;
            } else if first_line.chars().all(|c| c.is_alphanumeric() || c == '/') {
                // Entire first line is party identifier
                party_identifier = Some(first_line.to_string());
                lines.remove(0);
            }
        }

        // Parse name and address lines (max 4 lines, max 35 chars each)
        let mut name_and_address = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i >= 4 {
                break;
            }
            if line.len() > 35 {
                return Err(ParseError::InvalidFormat {
                    message: format!("Field 58D line {} exceeds 35 characters", i + 1),
                });
            }
            parse_swift_chars(line, &format!("Field 58D line {}", i + 1))?;
            name_and_address.push(line.to_string());
        }

        if name_and_address.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 58D must contain name and address information".to_string(),
            });
        }

        Ok(Field58D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = ":58D:".to_string();

        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
        }

        result.push_str(&self.name_and_address.join("\n"));
        result
    }
}

impl SwiftField for Field58 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try parsing as Field58A first (BIC-based)
        if let Ok(field) = Field58A::parse(input) {
            return Ok(Field58::A(field));
        }

        // Try parsing as Field58D (name and address)
        if let Ok(field) = Field58D::parse(input) {
            return Ok(Field58::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 58 could not be parsed as either option A or D".to_string(),
        })
    }

    fn parse_with_variant(
        value: &str,
        variant: Option<&str>,
        _field_tag: Option<&str>,
    ) -> crate::Result<Self>
    where
        Self: Sized,
    {
        match variant {
            Some("A") => {
                let field = Field58A::parse(value)?;
                Ok(Field58::A(field))
            }
            Some("D") => {
                let field = Field58D::parse(value)?;
                Ok(Field58::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field58::A(field) => field.to_swift_string(),
            Field58::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field58::A(_) => Some("A"),
            Field58::D(_) => Some("D"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field58a_parse_with_bic_only() {
        let field = Field58A::parse("DEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.bic, "DEUTDEFF");
    }

    #[test]
    fn test_field58a_parse_with_party_identifier() {
        let field = Field58A::parse("/CHGS123456DEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("/CHGS123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");
    }

    #[test]
    fn test_field58a_to_swift_string() {
        let field = Field58A {
            party_identifier: Some("/CHGS123456".to_string()),
            bic: "DEUTDEFF".to_string(),
        };
        assert_eq!(field.to_swift_string(), ":58A:/CHGS123456DEUTDEFF");
    }

    #[test]
    fn test_field58d_parse_with_name_only() {
        let input = "DEUTSCHE BANK AG\nFRANKFURT AM MAIN\nGERMANY";
        let field = Field58D::parse(input).unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 3);
        assert_eq!(field.name_and_address[0], "DEUTSCHE BANK AG");
        assert_eq!(field.name_and_address[1], "FRANKFURT AM MAIN");
        assert_eq!(field.name_and_address[2], "GERMANY");
    }

    #[test]
    fn test_field58d_parse_with_party_identifier() {
        let input = "/CH123456\nDEUTSCHE BANK AG\nFRANKFURT AM MAIN";
        let field = Field58D::parse(input).unwrap();
        assert_eq!(field.party_identifier, Some("/CH123456".to_string()));
        assert_eq!(field.name_and_address.len(), 2);
        assert_eq!(field.name_and_address[0], "DEUTSCHE BANK AG");
        assert_eq!(field.name_and_address[1], "FRANKFURT AM MAIN");
    }

    #[test]
    fn test_field58d_line_too_long() {
        let input = "THIS BANK NAME IS MUCH TOO LONG TO BE ACCEPTED IN FIELD 58D";
        assert!(Field58D::parse(input).is_err());
    }

    #[test]
    fn test_field58_enum_to_swift_string() {
        let field_a = Field58::A(Field58A {
            party_identifier: None,
            bic: "DEUTDEFF".to_string(),
        });
        assert_eq!(field_a.to_swift_string(), ":58A:DEUTDEFF");

        let field_d = Field58::D(Field58D {
            party_identifier: None,
            name_and_address: vec!["DEUTSCHE BANK AG".to_string()],
        });
        assert_eq!(field_d.to_swift_string(), ":58D:DEUTSCHE BANK AG");
    }
}
