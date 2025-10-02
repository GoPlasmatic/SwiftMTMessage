use super::field_utils::{parse_name_and_address, parse_party_identifier};
use super::swift_utils::{parse_bic, parse_swift_chars};
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 57: Account With Institution**
///
/// ## Purpose
/// Specifies the financial institution that services the account for the beneficiary customer
/// (Field 59A). This field identifies the beneficiary's bank where the account is maintained
/// and where funds will ultimately be credited. Essential for final delivery of funds and
/// beneficiary account identification. Critical component of payment settlement chain.
///
/// ## Format Options Overview
/// - **Option A**: BIC with optional party identifier - structured beneficiary bank identification
/// - **Option B**: Party identifier with location - domestic beneficiary bank routing
/// - **Option C**: Party identifier only - simplified beneficiary bank reference
/// - **Option D**: Party identifier with name/address - detailed beneficiary bank information
///
/// ## Business Context Applications
/// - **Beneficiary Bank**: Institution maintaining beneficiary's account
/// - **Final Settlement**: Ultimate destination for payment funds
/// - **Account Services**: Institution providing account services to beneficiary
/// - **Regulatory Reporting**: Required for beneficiary institution identification
///
/// ## Usage Rules and Conditions
/// - **Conditional Presence**: Required based on Rules C5 and C10
/// - **Receiver Default**: When absent, Receiver is also the account with institution
/// - **IBAN Compatibility**: Applicable even when Field 59A contains IBAN
/// - **Direct Settlement**: Enables direct settlement when Receiver is account institution
///
/// ## Special Payment Method Codes
/// ### Critical Settlement Instructions
/// - **//FW**: Fedwire routing - Required by US banks for Fedwire settlement
/// - **//RT**: Real-Time Gross Settlement - Binding instruction for RTGS systems
/// - **//AU**: Australian payment system settlement
/// - **//IN**: Indian payment system settlement
///
/// ### Code Usage Rules
/// - **Single Usage**: Codes //FW, //AU, //IN, //RT should appear only once in Field 56A or 57A
/// - **Binding Nature**: //RT code is binding and cannot be followed by other information
/// - **Final Settlement**: Ensures proper final settlement through appropriate systems
/// - **System Integration**: Enables automated settlement in national payment systems
///
/// ## Network Validation Requirements
/// - **BIC Registration**: All BIC codes must be registered financial institutions
/// - **Account Services**: Institution must provide account services to beneficiaries
/// - **Settlement Capability**: Must support final settlement in transaction currency
/// - **Regulatory Compliance**: Must meet beneficiary institution requirements
///
/// ## Settlement Logic and Processing
/// ### Direct Settlement
/// - **Receiver as Account Institution**: Simplest settlement scenario
/// - **Direct Relationship**: When Sender has direct relationship with beneficiary bank
/// - **Bilateral Agreements**: Pre-established settlement arrangements
/// - **Currency Considerations**: Direct settlement in transaction currency
///
/// ### Intermediated Settlement
/// - **Through Intermediary**: Settlement via Field 56A intermediary
/// - **Correspondent Network**: Utilizing correspondent banking relationships
/// - **Multi-Hop Settlement**: Complex settlement chains with multiple institutions
/// - **Optimization**: Most efficient settlement path selection
///
/// ## Regional Payment System Integration
/// ### North American Systems
/// - **Fedwire (//FW)**: US Federal Reserve final settlement system
/// - **ACH Networks**: Automated clearing house final settlement
/// - **Canadian Systems**: Canadian payment system final settlement
///
/// ### European Systems
/// - **TARGET2**: European Central Bank RTGS final settlement
/// - **SEPA**: Single Euro Payments Area account crediting
/// - **National Systems**: Country-specific final settlement systems
///
/// ### Asia-Pacific Systems
/// - **Australian (//AU)**: Australian payment system final settlement
/// - **Indian (//IN)**: Indian payment system final settlement
/// - **Regional Networks**: ASEAN and other regional final settlement
///
/// ## Beneficiary Protection and Compliance
/// - **Account Verification**: Ensuring beneficiary account exists and is operational
/// - **Name Matching**: Coordination with beneficiary customer name in Field 59A
/// - **Regulatory Requirements**: Meeting beneficiary institution reporting requirements
/// - **Sanctions Screening**: Beneficiary institution sanctions compliance
///
/// ## STP Processing Benefits
/// - **Automated Settlement**: System-driven final settlement based on clear identification
/// - **Account Integration**: Direct integration with beneficiary account systems
/// - **Exception Reduction**: Proper institution identification reduces settlement failures
/// - **Straight-Through Processing**: Enhanced STP through structured settlement data
///
/// ## Error Prevention Guidelines
/// - **Institution Verification**: Confirm institution provides account services
/// - **Account Relationship**: Verify institution-beneficiary account relationship
/// - **System Compatibility**: Ensure institution supports required settlement systems
/// - **Currency Support**: Confirm institution handles transaction currency
///
/// ## Related Fields Integration
/// - **Field 59A**: Beneficiary Customer (account holder identification)
/// - **Field 56A**: Intermediary (settlement routing)
/// - **Field 32A**: Value Date, Currency, Amount (settlement details)
///
///   **Field 57A: Account With Institution (BIC with Party Identifier)**
///
/// Structured beneficiary bank identification using BIC code with optional party identifier.
/// Preferred option for automated processing and correspondent banking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field57A {
    /// Optional party identifier for clearing system or account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for special payment method codes like //FW, //RT, //AU, //IN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_identifier: Option<String>,

    /// Bank Identifier Code of the beneficiary's bank
    ///
    /// Format: 4!a2!a2!c\[3!c\] - 8 or 11 character BIC code
    /// Must be registered financial institution BIC
    pub bic: String,
}

impl SwiftField for Field57A {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 57A cannot be empty".to_string(),
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
                message: "Field 57A missing BIC code".to_string(),
            });
        }

        let bic = parse_bic(lines[bic_line_idx])?;

        Ok(Field57A {
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
        format!(":57A:{}", result.join("\n"))
    }
}

///   **Field 57B: Account With Institution (Party Identifier with Location)**
///
/// Domestic routing information using party identifier and location details.
/// Used for national clearing systems requiring location-based routing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field57B {
    /// Optional party identifier for clearing system or account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for domestic clearing systems and institutional references
    pub party_identifier: Option<String>,

    /// Location information for domestic routing
    ///
    /// Format: \[35x\] - Up to 35 character location identifier
    /// Used for location-based routing within domestic systems
    pub location: Option<String>,
}

impl SwiftField for Field57B {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if input.is_empty() {
            return Ok(Field57B {
                party_identifier: None,
                location: None,
            });
        }

        let lines: Vec<&str> = input.lines().collect();
        let mut party_identifier = None;
        let mut location = None;
        let mut current_idx = 0;

        // Check for party identifier
        if !lines.is_empty()
            && let Some(party_id) = parse_party_identifier(lines[0])?
        {
            party_identifier = Some(party_id);
            current_idx = 1;
        }

        // Check for location
        if current_idx < lines.len() {
            let loc = lines[current_idx];
            if !loc.is_empty() && loc.len() <= 35 {
                parse_swift_chars(loc, "Field 57B location")?;
                location = Some(loc.to_string());
            }
        }

        Ok(Field57B {
            party_identifier,
            location,
        })
    }

    fn to_swift_string(&self) -> String {
        let mut result = Vec::new();

        if let Some(ref id) = self.party_identifier {
            result.push(format!("/{}", id));
        }

        if let Some(ref loc) = self.location {
            result.push(loc.clone());
        }

        format!(":57B:{}", result.join("\n"))
    }
}

///   **Field 57C: Account With Institution (Party Identifier Only)**
///
/// Simplified institutional reference using party identifier only.
/// Used when BIC is not required or available for institutional identification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field57C {
    /// Party identifier for institutional reference
    ///
    /// Format: /34x - Mandatory slash prefix + up to 34 character identifier
    /// Used for domestic institutional references and clearing codes
    pub party_identifier: String,
}

impl SwiftField for Field57C {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        if !input.starts_with('/') {
            return Err(ParseError::InvalidFormat {
                message: "Field 57C must start with '/'".to_string(),
            });
        }

        let identifier = &input[1..];

        if identifier.is_empty() || identifier.len() > 34 {
            return Err(ParseError::InvalidFormat {
                message: "Field 57C party identifier must be 1-34 characters".to_string(),
            });
        }

        parse_swift_chars(identifier, "Field 57C party identifier")?;

        Ok(Field57C {
            party_identifier: identifier.to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        format!(":57C:/{}", self.party_identifier)
    }
}

///   **Field 57D: Account With Institution (Party Identifier with Name and Address)**
///
/// Detailed institutional identification with full name and address information.
/// Used when structured BIC identification is not available or sufficient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field57D {
    /// Optional party identifier for clearing system or account reference
    ///
    /// Format: \[/1!a\]\[/34x\] - Single character code + up to 34 character identifier
    /// Used for domestic clearing systems and institutional references
    pub party_identifier: Option<String>,

    /// Name and address of the account with institution
    ///
    /// Format: 4*35x - Up to 4 lines of 35 characters each
    /// Contains institution name, address, city, country details
    pub name_and_address: Vec<String>,
}

impl SwiftField for Field57D {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 57D must have at least one line".to_string(),
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
        let name_and_address = parse_name_and_address(&lines, start_idx, "Field57D")?;

        Ok(Field57D {
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

        format!(":57D:{}", result.join("\n"))
    }
}

/// Enum for Field57 Account With Institution variants (A, B, C, D)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Field57 {
    #[serde(rename = "57A")]
    A(Field57A),
    #[serde(rename = "57B")]
    B(Field57B),
    #[serde(rename = "57C")]
    C(Field57C),
    #[serde(rename = "57D")]
    D(Field57D),
}

impl SwiftField for Field57 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Try Option A (BIC-based) first
        if let Ok(field) = Field57A::parse(input) {
            return Ok(Field57::A(field));
        }

        // Try Option C (party identifier only)
        if input.starts_with('/')
            && !input.contains('\n')
            && let Ok(field) = Field57C::parse(input)
        {
            return Ok(Field57::C(field));
        }

        // Try Option B (party identifier with location)
        if let Ok(field) = Field57B::parse(input) {
            return Ok(Field57::B(field));
        }

        // Try Option D (party identifier with name/address)
        if let Ok(field) = Field57D::parse(input) {
            return Ok(Field57::D(field));
        }

        Err(ParseError::InvalidFormat {
            message: "Field 57 could not be parsed as option A, B, C or D".to_string(),
        })
    }

    fn to_swift_string(&self) -> String {
        match self {
            Field57::A(field) => field.to_swift_string(),
            Field57::B(field) => field.to_swift_string(),
            Field57::C(field) => field.to_swift_string(),
            Field57::D(field) => field.to_swift_string(),
        }
    }

    fn get_variant_tag(&self) -> Option<&'static str> {
        match self {
            Field57::A(_) => Some("A"),
            Field57::B(_) => Some("B"),
            Field57::C(_) => Some("C"),
            Field57::D(_) => Some("D"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field57a() {
        // With payment method code
        let field = Field57A::parse("//FW123456\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("/FW123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");

        // With standard party identifier
        let field = Field57A::parse("/C/US123456\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("C/US123456".to_string()));
        assert_eq!(field.bic, "DEUTDEFF");

        // Without party identifier
        let field = Field57A::parse("CHASUS33XXX").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.bic, "CHASUS33XXX");
    }

    #[test]
    fn test_field57b() {
        // With party identifier and location
        let field = Field57B::parse("/A/12345\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("A/12345".to_string()));
        assert_eq!(field.location, Some("NEW YORK".to_string()));

        // Empty
        let field = Field57B::parse("").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.location, None);
    }

    #[test]
    fn test_field57c() {
        let field = Field57C::parse("/UKCLEARING123").unwrap();
        assert_eq!(field.party_identifier, "UKCLEARING123");
        assert_eq!(field.to_swift_string(), ":57C:/UKCLEARING123");
    }

    #[test]
    fn test_field57d() {
        // With payment method code
        let field = Field57D::parse("//FW\nCHASE BANK\nNEW YORK").unwrap();
        assert_eq!(field.party_identifier, Some("/FW".to_string()));
        assert_eq!(field.name_and_address.len(), 2);
        assert_eq!(field.name_and_address[0], "CHASE BANK");

        // Without party identifier
        let field = Field57D::parse("BENEFICIARY BANK\nLONDON").unwrap();
        assert_eq!(field.party_identifier, None);
        assert_eq!(field.name_and_address.len(), 2);
    }

    #[test]
    fn test_field57_payment_method_codes() {
        // Fedwire code
        let field = Field57A::parse("//FW\nCHASUS33").unwrap();
        assert_eq!(field.party_identifier, Some("/FW".to_string()));

        // RTGS code
        let field = Field57A::parse("//RT\nDEUTDEFF").unwrap();
        assert_eq!(field.party_identifier, Some("/RT".to_string()));

        // Australian code
        let field = Field57A::parse("//AU\nANZBAU3M").unwrap();
        assert_eq!(field.party_identifier, Some("/AU".to_string()));

        // Indian code
        let field = Field57A::parse("//IN\nHDFCINBB").unwrap();
        assert_eq!(field.party_identifier, Some("/IN".to_string()));
    }

    #[test]
    fn test_field57_invalid() {
        // Invalid BIC
        assert!(Field57A::parse("INVALID").is_err());

        // Missing slash in 57C
        assert!(Field57C::parse("NOSLASH").is_err());

        // Too many lines in 57D
        assert!(Field57D::parse("LINE1\nLINE2\nLINE3\nLINE4\nLINE5").is_err());
    }
}
