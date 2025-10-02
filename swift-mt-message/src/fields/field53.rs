use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_max_length};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 53: Sender's Correspondent**
///
/// ## Purpose
/// Specifies the account or branch of the Sender through which reimbursement will occur
/// in correspondent banking arrangements. This field defines the reimbursement path between
/// the Sender and Receiver, enabling proper settlement coordination in cross-border payments.
/// Critical for establishing clear correspondent banking relationships and settlement flows.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured correspondent identification
/// - **Option B**: Party identifier with location - domestic correspondent routing
/// - **Option D**: Party identifier with name/address - detailed correspondent information
///
/// ## Business Context Applications
/// - **Reimbursement Routing**: Defines settlement path between correspondent banks
/// - **Nostro Account Management**: Specifies accounts used for correspondent settlements
/// - **Currency Settlement**: Enables currency-specific correspondent arrangements
/// - **Cross-Border Payments**: Essential for international payment routing and settlement
///
/// ## Usage Rules and Conditions
/// - **Conditional Presence**: Required when no direct account relationship exists (Rule C4)
/// - **Direct Relationships**: Omitted when unique bilateral account relationship exists
/// - **Multiple Accounts**: Option B with party identifier when multiple accounts exist
/// - **Cover Messages**: May trigger MT 202 COV requirement for certain configurations
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Account Validity**: Party identifiers must reference valid correspondent accounts
/// - **Reachability**: Correspondent institutions must be operational and reachable
/// - **Currency Support**: Correspondents must support transaction currency
///
/// ## Correspondent Banking Logic
/// ### Direct Account Relationships
/// - **Unique Accounts**: When only one account exists, field may be omitted
/// - **Bilateral Agreements**: Pre-agreed account arrangements remove field requirement
/// - **Standard Currencies**: Common currency pairs with established relationships
///
/// ### Multiple Account Scenarios
/// - **Currency-Specific**: Different correspondents for different currencies
/// - **Service-Specific**: Specialized correspondents for different services
/// - **Geographic**: Regional correspondents for specific market coverage
/// - **Risk Management**: Diversified correspondent relationships for risk mitigation
///
/// ## Cover Message Requirements
/// - **Non-Receiver Branch**: Field 53A containing non-Receiver branch triggers cover message
/// - **MT 202 COV**: Cover payment message required for certain correspondent configurations
/// - **Settlement Coordination**: Ensures proper settlement through correspondent network
/// - **Regulatory Compliance**: Meets regulatory requirements for payment transparency
///
/// ## Regional Considerations
/// - **European Networks**: TARGET2 correspondent arrangements and SEPA integration
/// - **US Systems**: Fedwire correspondent relationships and dollar clearing
/// - **Asian Markets**: Regional correspondent networks and local currency clearing
/// - **Cross-Border**: Multi-currency correspondent arrangements and settlement
///
/// ## Risk Management Applications
/// - **Counterparty Risk**: Correspondent bank risk assessment and management
/// - **Settlement Risk**: Mitigation through established correspondent relationships
/// - **Operational Risk**: Backup correspondent arrangements for business continuity
/// - **Regulatory Risk**: Compliance with correspondent banking regulations
///
/// ## STP Processing Benefits
/// - **Automated Routing**: System-driven correspondent routing based on clear identification
/// - **Settlement Efficiency**: Streamlined settlement through established relationships
/// - **Exception Reduction**: Proper correspondent identification reduces processing delays
/// - **Straight-Through Processing**: Enhanced STP rates through structured correspondent data
///
/// ## Error Prevention Guidelines
/// - **Relationship Verification**: Confirm correspondent relationships are active
/// - **Account Validation**: Verify correspondent accounts are operational
/// - **Currency Checking**: Ensure correspondent supports transaction currency
/// - **Format Compliance**: Exact adherence to option format requirements
///
/// ## Related Fields Integration
/// - **Field 52A**: Ordering Institution (institutional hierarchy)
/// - **Field 54A**: Receiver's Correspondent (settlement coordination)
/// - **Field 57A**: Account With Institution (final delivery arrangement)
/// - **Field 32A**: Value Date, Currency, Amount (settlement details)
///
/// ## Compliance Framework
/// - **Correspondent Due Diligence**: Enhanced due diligence on correspondent relationships
/// - **Regulatory Reporting**: Correspondent banking relationship reporting requirements
/// - **AML Compliance**: Anti-money laundering considerations in correspondent banking
/// - **Sanctions Screening**: Correspondent bank sanctions screening requirements
///
/// ## Settlement Coordination
/// - **Nostro Management**: Coordination with nostro account balances and limits
/// - **Value Dating**: Alignment with correspondent value dating practices
/// - **Cut-off Times**: Coordination with correspondent processing cut-offs
/// - **Holiday Calendars**: Consideration of correspondent market holidays
///
/// ## See Also
/// - Swift FIN User Handbook: Sender's Correspondent Specifications
/// - Correspondent Banking Guidelines: Relationship Management Standards
/// - Settlement Systems: Cross-Border Settlement Mechanisms
/// - Risk Management: Correspondent Banking Risk Assessment
///
///   **Field 53A: Sender's Correspondent (BIC with Party Identifier)**
///
/// Structured correspondent identification using BIC code with optional party identifier.
/// Preferred option for automated correspondent banking processing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field53A {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for nostro account identification and correspondent routing
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the sender's correspondent
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC
    pub bic: String,
}

impl SwiftField for Field53A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.split('\n').collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 53A requires input".to_string(),
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
                message: "Field 53A requires BIC code after party identifier".to_string(),
            });
        }

        // Parse BIC code
        let bic = parse_bic(lines[line_idx])?;

        Ok(Field53A {
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
        format!(":53A:{}", result)
    }
}

///   **Field 53B: Sender's Correspondent (Party Identifier with Location)**
///
/// Domestic correspondent routing using party identifier and location details.
/// Used when multiple correspondent accounts exist and location-based routing is required.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field53B {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for nostro account identification when multiple accounts exist
    pub party_identifier: Option<String>,

    /// Location information for correspondent routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based correspondent routing within domestic systems
    pub location: Option<String>,
}

impl SwiftField for Field53B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Ok(Field53B {
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
            location = Some(parse_max_length(lines[line_idx], 35, "Field53B location")?);
        }

        Ok(Field53B {
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
        format!(":53B:{}", result)
    }
}

///   **Field 53D: Sender's Correspondent (Party Identifier with Name and Address)**
///
/// Detailed correspondent identification with full name and address information.
/// Used when structured BIC identification is not available for correspondent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field53D {
    /// Optional party identifier for correspondent account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for nostro account identification and correspondent routing
    pub party_identifier: Option<String>,

    /// Name and address of the sender's correspondent
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains correspondent name, address, city, country details
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field53D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.split('\n').collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 53D requires at least one line".to_string(),
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
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field53D")?;

        Ok(Field53D {
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
        format!(":53D:{}", result)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field53SenderCorrespondent {
    #[serde(rename = "53A")]
    A(Field53A),
    #[serde(rename = "53B")]
    B(Field53B),
    #[serde(rename = "53D")]
    D(Field53D),
}

impl SwiftField for Field53SenderCorrespondent {
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
            // Try parsing as Field53A
            if let Ok(field) = Field53A::parse(input) {
                return Ok(Field53SenderCorrespondent::A(field));
            }
        }

        // Check for multiple lines suggesting D format
        if lines.len() > 2 || (lines.len() == 2 && !lines[0].starts_with('/')) {
            // Try parsing as Field53D (multiple lines of name/address)
            if let Ok(field) = Field53D::parse(input) {
                return Ok(Field53SenderCorrespondent::D(field));
            }
        }

        // Try parsing as Field53B (simpler format)
        if let Ok(field) = Field53B::parse(input) {
            return Ok(Field53SenderCorrespondent::B(field));
        }

        // If all fail, try in order
        if let Ok(field) = Field53A::parse(input) {
            return Ok(Field53SenderCorrespondent::A(field));
        }
        if let Ok(field) = Field53D::parse(input) {
            return Ok(Field53SenderCorrespondent::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 53 could not be parsed as any valid option (A, B, or D)".to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field53SenderCorrespondent::A(field) => field.to_swift_string(),
            Field53SenderCorrespondent::B(field) => field.to_swift_string(),
            Field53SenderCorrespondent::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field53SenderCorrespondent::A(_) => Some("A"),
            Field53SenderCorrespondent::B(_) => Some("B"),
            Field53SenderCorrespondent::D(_) => Some("D"),
        }
    }
}

// Type alias for backward compatibility and simplicity
pub type Field53 = Field53SenderCorrespondent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field53a_valid() {
        // Without party identifier
        let field = Field53A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.bic, "CHASUS33XXX");
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.to_swift_string(), ":53A:CHASUS33XXX");

        // With party identifier
        let field = Field53A::parse("/C/12345678\nCHASUS33").unwrap();
        assert_eq!(field.bic, "CHASUS33");
        assert_eq!(field.party_identifier, Some("/C/12345678".to_string()));
        assert_eq!(field.to_swift_string(), ":53A:/C/12345678\nCHASUS33");
    }

    #[test]
    fn test_field53b_valid() {
        // Only location
        let field = Field53B::parse("NEW YORK BRANCH").unwrap();
        assert_eq!(field.location, Some("NEW YORK BRANCH".to_string()));
        assert_eq!(field.party_identifier, None);

        // With party identifier
        let field = Field53B::parse("/D/98765432\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("/D/98765432".to_string()));
        assert_eq!(field.location, Some("NEW YORK".to_string()));

        // Empty
        let field = Field53B::parse("").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.location, None);
    }

    #[test]
    fn test_field53d_valid() {
        // With party identifier and name/address
        let field =
            Field53D::parse("/C/12345678\nCORRESPONDENT BANK\n123 MAIN ST\nNEW YORK\nUSA").unwrap();
        assert_eq!(field.party_identifier, Some("/C/12345678".to_string()));
        assert_eq!(field.name_and_address.len(), 4);
        assert_eq!(field.name_and_address[0], "CORRESPONDENT BANK");
        assert_eq!(field.name_and_address[3], "USA");

        // Without party identifier
        let field = Field53D::parse("CORRESPONDENT BANK\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field53_enum() {
        // Parse as A
        let field = Field53SenderCorrespondent::parse("CHASUS33XXX").unwrap();
        assert!(matches!(field, Field53SenderCorrespondent::A(_)));

        // Parse as B
        let field = Field53SenderCorrespondent::parse("NEW YORK BRANCH").unwrap();
        assert!(matches!(field, Field53SenderCorrespondent::B(_)));

        // Parse as D
        let field = Field53SenderCorrespondent::parse("BANK NAME\nADDRESS LINE 1\nCITY").unwrap();
        assert!(matches!(field, Field53SenderCorrespondent::D(_)));
    }
}
