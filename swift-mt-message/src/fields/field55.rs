use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_max_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 55: Third Reimbursement Institution**
///
/// ## Purpose
/// Specifies the Receiver's branch when funds are made available through a different
/// institution than specified in Field 53A (Sender's Correspondent). This field enables
/// complex reimbursement chains involving multiple institutions, typically used when
/// the fund availability institution differs from the primary correspondent relationship.
/// Essential for sophisticated correspondent banking arrangements.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured third institution identification
/// - **Option B**: Party identifier with location - domestic third institution routing
/// - **Option D**: Party identifier with name/address - detailed third institution information
///
/// ## Business Context Applications
/// - **Complex Reimbursement**: Multi-institution settlement chains requiring third party
/// - **Branch Specification**: Receiver's branch when funds available through intermediary
/// - **Settlement Optimization**: Efficient routing through specialized institutions
/// - **Regional Networks**: Local institution integration in cross-border payments
///
/// ## Usage Rules and Conditions
/// - **Conditional Presence**: Optional field referenced in Rule C4 correspondent logic
/// - **Field 54A Dependency**: Typically used when Field 54A contains non-Receiver institution
/// - **Receiver Branch**: Usually contains Receiver's branch in complex chains
/// - **Settlement Finalization**: Represents final settlement point for funds availability
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Institution Capability**: Must provide reimbursement and settlement services
/// - **Operational Status**: Institutions must be operational and reachable
/// - **Currency Support**: Must support transaction currency and settlement requirements
///
/// ## Complex Settlement Scenarios
/// ### Multi-Institution Chains
/// - **Field 53A**: Primary correspondent relationship (Sender side)
/// - **Field 54A**: Intermediary institution for fund availability
/// - **Field 55A**: Final settlement institution (typically Receiver's branch)
/// - **Settlement Flow**: Funds flow through multiple institutions to reach final destination
///
/// ### Regional Integration
/// - **Local Presence**: Integration with local banking networks
/// - **Regulatory Compliance**: Meeting local settlement requirements
/// - **Currency Optimization**: Efficient local currency settlement
/// - **Service Specialization**: Leveraging specialized institution capabilities
///
/// ## Risk Management Applications
/// - **Settlement Risk**: Distribution of settlement risk across multiple institutions
/// - **Operational Risk**: Redundancy and backup settlement paths
/// - **Counterparty Risk**: Diversification of counterparty exposure
/// - **Liquidity Management**: Optimization of liquidity across correspondent network
///
/// ## Regional Considerations
/// - **European Networks**: TARGET2 integration and Euro settlement optimization
/// - **US Systems**: Federal Reserve and commercial bank integration
/// - **Asian Markets**: Regional banking network integration and local settlement
/// - **Emerging Markets**: Local institution integration for regulatory compliance
///
/// ## STP Processing Benefits
/// - **Chain Automation**: Automated processing of complex settlement chains
/// - **Exception Handling**: Structured handling of multi-institution scenarios
/// - **Settlement Optimization**: Efficient routing through multiple institutions
/// - **Risk Distribution**: Automated risk assessment across institution chain
///
/// ## Error Prevention Guidelines
/// - **Chain Validation**: Verify complete settlement chain is operational
/// - **Institution Verification**: Confirm all institutions can provide required services
/// - **Relationship Checking**: Validate relationships between all chain participants
/// - **Currency Support**: Ensure all institutions support transaction currency
///
/// ## Related Fields Integration
/// - **Field 53A**: Sender's Correspondent (settlement chain initiation)
/// - **Field 54A**: Receiver's Correspondent (intermediate settlement)
/// - **Field 57A**: Account With Institution (final beneficiary bank)
/// - **Field 32A**: Value Date, Currency, Amount (settlement details)
///
/// ## Compliance Framework
/// - **Multi-Institution Due Diligence**: Enhanced due diligence across institution chain
/// - **Regulatory Coordination**: Compliance across multiple regulatory jurisdictions
/// - **Audit Trail**: Complete documentation of multi-institution settlement path
/// - **Risk Assessment**: Comprehensive risk evaluation across institution chain
///
/// ## Settlement Coordination
/// - **Value Date Alignment**: Coordination of value dates across multiple institutions
/// - **Cut-off Management**: Alignment with multiple institution processing cut-offs
/// - **Holiday Coordination**: Management of multiple market holiday calendars
/// - **Liquidity Planning**: Coordination of liquidity across correspondent network
///
/// ## See Also
/// - Swift FIN User Handbook: Third Reimbursement Institution Specifications
/// - Correspondent Banking Guidelines: Complex Settlement Chain Management
/// - Multi-Institution Settlement: Risk and Operational Considerations
/// - Cross-Border Payments: Advanced Correspondent Banking Arrangements
///
///   **Field 55A: Third Reimbursement Institution (BIC with Party Identifier)**
///
/// Structured third institution identification using BIC code with optional party identifier.
/// Used for complex correspondent banking chains requiring additional institutional routing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field55A {
    /// Optional party identifier for third institution account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for specialized account identification in complex settlement chains
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the third reimbursement institution
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC
    pub bic: String,
}

impl SwiftField for Field55A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.split('\n').collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 55A requires input".to_string(),
            });
        }

        let mut line_idx = 0;
        let mut party_identifier = None;

        // Check for optional party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(format!("/{}", party_id));
            line_idx = 1;
        }

        // Ensure we have a BIC line
        if line_idx >= lines.len() {
            return Err(ParseError::InvalidFormat {
                message: "Field 55A requires BIC code after party identifier".to_string(),
            });
        }

        // Parse BIC code
        let bic = parse_bic(lines[line_idx])?;

        Ok(Field55A {
            party_identifier,
            bic,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            result.push('\n');
        }
        result.push_str(&self.bic);
        result
    }
}

///   **Field 55B: Third Reimbursement Institution (Party Identifier with Location)**
///
/// Domestic third institution routing using party identifier and location details.
/// Used for location-based routing in complex domestic settlement arrangements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field55B {
    /// Optional party identifier for third institution account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for specialized routing in complex settlement chains
    pub party_identifier: Option<String>,

    /// Location information for third institution routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based routing in complex correspondent networks
    pub location: Option<String>,
}

impl SwiftField for Field55B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Ok(Field55B {
                party_identifier: None,
                location: None,
            });
        }

        let lines: Vec<&str> = input.split('\n').collect();
        let mut party_identifier = None;
        let mut location = None;
        let mut line_idx = 0;

        // Check for party identifier on first line
        if !lines.is_empty() && lines[0].starts_with('/') {
            party_identifier = Some(lines[0].to_string());
            line_idx = 1;
        }

        // Remaining line is location
        if line_idx < lines.len() && !lines[line_idx].is_empty() {
            location = Some(parse_max_length(lines[line_idx], 35, "Field55B location")?);
        }

        Ok(Field55B {
            party_identifier,
            location,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            if self.location.is_some() {
                result.push('\n');
            }
        }
        if let Some(ref loc) = self.location {
            result.push_str(loc);
        }
        result
    }
}

///   **Field 55D: Third Reimbursement Institution (Party Identifier with Name and Address)**
///
/// Detailed third institution identification with full name and address information.
/// Used when structured BIC identification is not available for third institution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field55D {
    /// Optional party identifier for third institution account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for specialized routing in complex settlement chains
    pub party_identifier: Option<String>,

    /// Name and address of the third reimbursement institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field55D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.split('\n').collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 55D requires at least one line".to_string(),
            });
        }

        let mut party_identifier = None;
        let mut start_idx = 0;

        // Check for party identifier on first line
        if let Some(party_id) = parse_party_identifier(lines[0])? {
            party_identifier = Some(format!("/{}", party_id));
            start_idx = 1;
        }

        // Parse remaining lines as name and address
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field55D")?;

        Ok(Field55D {
            party_identifier,
            name_and_address,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref party_id) = self.party_identifier {
            result.push_str(party_id);
            result.push('\n');
        }
        for (i, line) in self.name_and_address.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }
            result.push_str(line);
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field55ThirdReimbursementInstitution {
    #[serde(rename = "55A")]
    A(Field55A),
    #[serde(rename = "55B")]
    B(Field55B),
    #[serde(rename = "55D")]
    D(Field55D),
}

impl SwiftField for Field55ThirdReimbursementInstitution {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try parsing as each variant
        // A: Has BIC code (8 or 11 uppercase letters/digits)
        // B: Has optional party identifier and/or location
        // D: Has party identifier and/or multiple lines of name/address

        let lines: Vec<&str> = input.split('\n').collect();
        let last_line = lines.last().unwrap_or(&"");

        // Check if last line looks like a BIC code
        if (8..=11).contains(&last_line.len())
            && last_line
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            // Try parsing as Field55A
            if let Ok(field) = Field55A::parse(input) {
                return Ok(Field55ThirdReimbursementInstitution::A(field));
            }
        }

        // Check for multiple lines suggesting D format
        if lines.len() > 2 || (lines.len() == 2 && !lines[0].starts_with('/')) {
            // Try parsing as Field55D (multiple lines of name/address)
            if let Ok(field) = Field55D::parse(input) {
                return Ok(Field55ThirdReimbursementInstitution::D(field));
            }
        }

        // Try parsing as Field55B (simpler format)
        if let Ok(field) = Field55B::parse(input) {
            return Ok(Field55ThirdReimbursementInstitution::B(field));
        }

        // If all fail, try in order
        if let Ok(field) = Field55A::parse(input) {
            return Ok(Field55ThirdReimbursementInstitution::A(field));
        }
        if let Ok(field) = Field55D::parse(input) {
            return Ok(Field55ThirdReimbursementInstitution::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 55 could not be parsed as any valid option (A, B, or D)".to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field55ThirdReimbursementInstitution::A(field) => field.to_swift_string(),
            Field55ThirdReimbursementInstitution::B(field) => field.to_swift_string(),
            Field55ThirdReimbursementInstitution::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field55ThirdReimbursementInstitution::A(_) => Some("A"),
            Field55ThirdReimbursementInstitution::B(_) => Some("B"),
            Field55ThirdReimbursementInstitution::D(_) => Some("D"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field55a_valid() {
        // Without party identifier
        let field = Field55A::parse("BNPAFRPPXXX").unwrap();
        assert_eq!(field.bic, "BNPAFRPPXXX");
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.to_swift_string(), "BNPAFRPPXXX");

        // With party identifier
        let field = Field55A::parse("/E/55566677\nBNPAFRPP").unwrap();
        assert_eq!(field.bic, "BNPAFRPP");
        assert_eq!(field.party_identifier, Some("/E/55566677".to_string()));
        assert_eq!(field.to_swift_string(), "/E/55566677\nBNPAFRPP");
    }

    #[test]
    fn test_field55b_valid() {
        // Only location
        let field = Field55B::parse("PARIS BRANCH").unwrap();
        assert_eq!(field.location, Some("PARIS BRANCH".to_string()));
        assert_eq!(field.party_identifier, None);

        // With party identifier
        let field = Field55B::parse("/F/99887766\nPARIS").unwrap();
        assert_eq!(field.party_identifier, Some("/F/99887766".to_string()));
        assert_eq!(field.location, Some("PARIS".to_string()));

        // Empty
        let field = Field55B::parse("").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.location, None);
    }

    #[test]
    fn test_field55d_valid() {
        // With party identifier and name/address
        let field =
            Field55D::parse("/E/55566677\nTHIRD BANK\n789 THIRD ST\nPARIS\nFRANCE").unwrap();
        assert_eq!(field.party_identifier, Some("/E/55566677".to_string()));
        assert_eq!(field.name_and_address.len(), 4);
        assert_eq!(field.name_and_address[0], "THIRD BANK");
        assert_eq!(field.name_and_address[3], "FRANCE");

        // Without party identifier
        let field = Field55D::parse("THIRD BANK\nPARIS").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field55_enum() {
        // Parse as A
        let field = Field55ThirdReimbursementInstitution::parse("BNPAFRPPXXX").unwrap();
        assert!(matches!(field, Field55ThirdReimbursementInstitution::A(_)));

        // Parse as B
        let field = Field55ThirdReimbursementInstitution::parse("PARIS BRANCH").unwrap();
        assert!(matches!(field, Field55ThirdReimbursementInstitution::B(_)));

        // Parse as D
        let field =
            Field55ThirdReimbursementInstitution::parse("BANK NAME\nADDRESS LINE 1\nCITY").unwrap();
        assert!(matches!(field, Field55ThirdReimbursementInstitution::D(_)));
    }
}
