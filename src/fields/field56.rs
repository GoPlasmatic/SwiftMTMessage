use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 56: Intermediary**
///
/// ## Purpose
/// Specifies the financial institution through which the transaction must pass to reach
/// the account with institution (Field 57A). This field defines the routing path for
/// payment instructions, enabling proper routing through intermediary banks when direct
/// relationships do not exist. Critical for straight-through processing and automated
/// payment routing in complex banking networks.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured intermediary identification
/// - **Option C**: Party identifier only - simplified intermediary reference
/// - **Option D**: Party identifier with name/address - detailed intermediary information
///
/// ## Business Context Applications
/// - **Payment Routing**: Defines intermediary path to reach beneficiary's bank
/// - **Correspondent Networks**: Utilizes existing correspondent banking relationships
/// - **Cross-Border Payments**: Essential for international payment routing
/// - **Domestic Clearing**: Integration with national payment systems
///
/// ## Special Payment Method Codes
/// ### Critical Routing Instructions
/// - **//FW**: Fedwire routing - Required by US banks for Fedwire processing
/// - **//RT**: Real-Time Gross Settlement - Binding instruction for RTGS systems
/// - **//AU**: Australian payment system routing
/// - **//IN**: Indian payment system routing
///
/// ### Code Usage Rules
/// - **Single Usage**: Codes //FW, //AU, //IN, //RT should appear only once in Field 56A or 57A
/// - **Binding Nature**: //RT code is binding and cannot be followed by other information
/// - **System Integration**: Enables automated processing in national clearing systems
/// - **Precedence**: Special codes take precedence over standard routing information
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Routing Capability**: Intermediaries must provide routing services to destination
/// - **System Support**: Must support specified payment method codes
/// - **Currency Capability**: Must handle transaction currency and settlement
///
/// ## Routing Logic and Rules
/// ### Direct vs. Intermediary Routing
/// - **Direct Access**: When Sender has direct relationship with Account With Institution
/// - **Intermediary Required**: When direct relationship does not exist or is inefficient
/// - **Multi-Hop Routing**: Complex routing through multiple intermediaries
/// - **Optimization**: Selection of most efficient routing path
///
/// ### Payment System Integration
/// - **RTGS Systems**: Real-time gross settlement system routing
/// - **ACH Networks**: Automated clearing house routing
/// - **Wire Networks**: Wire transfer system routing
/// - **Clearing Systems**: National and regional clearing system integration
///
/// ## Regional Payment System Support
/// ### North American Systems
/// - **Fedwire (//FW)**: US Federal Reserve wire transfer system
/// - **ACH**: Automated Clearing House networks
/// - **Canadian Systems**: Integration with Canadian payment networks
///
/// ### Asia-Pacific Systems
/// - **Australian (//AU)**: Australian payment system integration
/// - **Indian (//IN)**: Indian payment system routing
/// - **Regional Networks**: ASEAN and other regional payment systems
///
/// ### European Systems
/// - **TARGET2**: European Central Bank RTGS system
/// - **SEPA**: Single Euro Payments Area routing
/// - **National RTGS**: Country-specific RTGS systems
///
/// ## STP Processing Benefits
/// - **Automated Routing**: System-driven routing based on intermediary identification
/// - **Exception Reduction**: Proper routing reduces payment exceptions and delays
/// - **Straight-Through Processing**: Enhanced STP through structured routing data
/// - **System Integration**: Seamless integration with payment system networks
///
/// ## Error Prevention Guidelines
/// - **Routing Validation**: Verify intermediary can route to destination
/// - **System Compatibility**: Confirm intermediary supports required payment systems
/// - **Code Verification**: Validate special payment method codes are appropriate
/// - **Relationship Checking**: Verify Sender has relationship with intermediary
///
/// ## Related Fields Integration
/// - **Field 57A**: Account With Institution (routing destination)
/// - **Field 53A**: Sender's Correspondent (routing coordination)
/// - **Field 32A**: Value Date, Currency, Amount (routing context)
/// - **Field 72**: Sender to Receiver Information (routing instructions)
///
/// ## Compliance Framework
/// - **Routing Documentation**: Complete routing path documentation
/// - **Intermediary Due Diligence**: Enhanced due diligence on routing intermediaries
/// - **Regulatory Compliance**: Meeting routing and settlement regulations
/// - **Audit Trail**: Comprehensive routing audit trail maintenance
///
/// ## Performance Optimization
/// - **Routing Efficiency**: Selection of optimal routing paths
/// - **Cost Management**: Consideration of routing costs and fees
/// - **Speed Optimization**: Fastest routing path selection
/// - **Reliability**: Most reliable routing path selection
///
/// ## See Also
/// - Swift FIN User Handbook: Intermediary Institution Specifications
/// - Payment System Guides: National Payment System Routing
/// - Correspondent Banking: Intermediary Routing Arrangements
/// - Cross-Border Payments: International Routing Standards
///
///   **Field 56A: Intermediary (BIC with Party Identifier)**
///
/// Structured intermediary identification using BIC code with optional party identifier.
/// Preferred option for automated payment routing and system integration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field56A {
    /// Optional party identifier for routing and payment method codes
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// May contain special codes: //FW (Fedwire), //RT (RTGS), //AU (Australian), //IN (Indian)
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the intermediary institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution with routing capability
    pub bic: String,
}

///   **Field 56C: Intermediary (Party Identifier Only)**
///
/// Simplified intermediary reference using party identifier only.
/// Used when BIC is not required or available for routing purposes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field56C {
    /// Party identifier for intermediary routing
    ///
    /// Format: /34x - Mandatory slash prefix + up to 34 character identifier
    /// Used for domestic routing codes and clearing system identifiers
    pub party_identifier: String,
}

///   **Field 56D: Intermediary (Party Identifier with Name and Address)**
///
/// Detailed intermediary identification with full name and address information.
/// Used when structured BIC identification is not available for intermediary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field56D {
    /// Optional party identifier for routing and payment method codes
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// May contain special routing codes and clearing system identifiers
    pub party_identifier: Option<String>,

    /// Name and address of the intermediary institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field56Intermediary {
    #[serde(rename = "56A")]
    A(Field56A),
    #[serde(rename = "56C")]
    C(Field56C),
    #[serde(rename = "56D")]
    D(Field56D),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field56IntermediaryAD {
    #[serde(rename = "56A")]
    A(Field56A),
    #[serde(rename = "56D")]
    D(Field56D),
}

impl SwiftField for Field56A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 56A cannot be empty".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut bic_line_idx = 0;

        // Check for optional party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(party_id);
            bic_line_idx = 1;
        }

        // Ensure we have a BIC line
        if bic_line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 56A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field56A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(ref id) = self.party_identifier {
            result.push(format!("/{}", id));
        }

        result.push(self.bic.clone());
        format!(":56A:{}", result.join("\n"))
    }
}

impl SwiftField for Field56C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if !input.starts_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 56C must start with '/'".to_string(),
            });
        }

        let identifier = &input[1..];

        if identifier.is_empty() || identifier.len() > 34 {
            return Err(ParseError::InvalidFormat {
                message: "Field 56C party identifier must be 1-34 characters".to_string(),
            });
        }

        parse_swift_chars(identifier, "Field 56C party identifier")?;

        Ok(Field56C {
            party_identifier: identifier.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":56C:/{}", self.party_identifier)
    }
}

impl SwiftField for Field56D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 56D must have at least one line".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut start_idx = 0;

        // Check for party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(party_id);
            start_idx = 1;
        }

        // Parse remaining lines as name and address
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field56D")?;

        Ok(Field56D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(ref id) = self.party_identifier {
            result.push(format!("/{}", id));
        }

        for line in &self.name_and_address {
            result.push(line.clone());
        }

        format!(":56D:{}", result.join("\n"))
    }
}

impl SwiftField for Field56Intermediary {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field56A::parse(input) {
            return Ok(Field56Intermediary::A(field));
        }

        // Try Option C (party identifier only) - must be single line with /
        if input.starts_with('/')
            && !input.contains('\n')
            && let Ok(field) = Field56C::parse(input)
        {
            return Ok(Field56Intermediary::C(field));
        }

        // Try Option D (party identifier with name/address)
        if let Ok(field) = Field56D::parse(input) {
            return Ok(Field56Intermediary::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 56 Intermediary could not be parsed as option A, C or D".to_string(),
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
                let field = Field56A::parse(value)?;
                Ok(Field56Intermediary::A(field))
            }
            Some("C") => {
                let field = Field56C::parse(value)?;
                Ok(Field56Intermediary::C(field))
            }
            Some("D") => {
                let field = Field56D::parse(value)?;
                Ok(Field56Intermediary::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field56Intermediary::A(field) => field.to_swift_string(),
            Field56Intermediary::C(field) => field.to_swift_string(),
            Field56Intermediary::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field56Intermediary::A(_) => Some("A"),
            Field56Intermediary::C(_) => Some("C"),
            Field56Intermediary::D(_) => Some("D"),
        }
    }
}

impl SwiftField for Field56IntermediaryAD {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field56A::parse(input) {
            return Ok(Field56IntermediaryAD::A(field));
        }

        // Try Option D (party identifier with name/address)
        if let Ok(field) = Field56D::parse(input) {
            return Ok(Field56IntermediaryAD::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 56 Intermediary AD could not be parsed as option A or D".to_string(),
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
                let field = Field56A::parse(value)?;
                Ok(Field56IntermediaryAD::A(field))
            }
            Some("D") => {
                let field = Field56D::parse(value)?;
                Ok(Field56IntermediaryAD::D(field))
            }
            _ => {
                // No variant specified, fall back to default parse behavior
                Self::parse(value)
            }
        }
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field56IntermediaryAD::A(field) => field.to_swift_string(),
            Field56IntermediaryAD::D(field) => field.to_swift_string(),
        }
    }
}

// Type aliases for backward compatibility and simplicity
pub type Field56 = Field56Intermediary;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field56a() {
        // With party identifier
        let field = Field56A::parse("/C/US123456\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("C/US123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");

        // With special code //FW
        let field = Field56A::parse("//FW021000018\nCHASUS33XXX").unwrap();
        assert_eq!(field.party_identifier, Some("/FW021000018".to_string()));
        assert_eq!(field.bic, "CHASUS33XXX");

        // Without party identifier
        let field = Field56A::parse("DEUTDEFFXXX").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.bic, "DEUTDEFFXXX");
    }

    #[test]
    fn test_field56c() {
        let field = Field56C::parse("/USCLEARING123").unwrap();
        assert_eq!(field.party_identifier, "USCLEARING123");
        assert_eq!(field.to_swift_string(), ":56C:/USCLEARING123");
    }

    #[test]
    fn test_field56d() {
        // With party identifier
        let field = Field56D::parse("/D/DE123456\nDEUTSCHE BANK\nFRANKFURT").unwrap();
        assert_eq!(field.party_identifier, Some("D/DE123456".to_string()));
        assert_eq!(field.name_and_address.len(), 2);
        assert_eq!(field.name_and_address[0], "DEUTSCHE BANK");

        // Without party identifier
        let field = Field56D::parse("ACME BANK\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field56_invalid() {
        // Invalid BIC
        assert!(Field56A::parse("INVALID").is_err());

        // Missing slash in 56C
        assert!(Field56C::parse("NOSLASH").is_err());

        // Too many lines in 56D
        assert!(Field56D::parse("LINE1\nLINE2\nLINE3\nLINE4\nLINE5").is_err());
    }
}
