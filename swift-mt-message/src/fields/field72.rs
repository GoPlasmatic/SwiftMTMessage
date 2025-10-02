use super::field_utils::parse_multiline_text;
use crate::errors::ParseError;
use crate::traits::SwiftField;
use serde::{Deserialize, Serialize};

///   **Field 72: Sender to Receiver Information**
///
/// ## Purpose
/// Specifies additional information for the Receiver or other specified party in
/// financial messages. This field provides structured communication between financial
/// institutions, enabling additional instructions, clarifications, and institutional
/// coordination that supplements the main transaction details.
///
/// ## Format Specification
/// - **Swift Format**: `6*35x`
/// - **Structure**: Up to 6 lines of 35 characters each
/// - **Content**: Structured narrative format with specific codes
/// - **Line Format**: `/8c/[additional information]` (Code)(Narrative)
///
/// ## Business Context Applications
/// - **Institutional Communication**: Additional instructions between banks
/// - **Processing Instructions**: Specific handling requirements
/// - **Regulatory Information**: Compliance-related communications
/// - **Operational Coordination**: Coordination between correspondent banks
///
/// ## Network Validation Requirements
/// - **Line Structure**: Each code must be between slashes at line beginning
/// - **Continuation**: Continuation text starts with '//'
/// - **Prohibited Codes**: /REJT/ and /RETN/ codes not allowed (Error T81)
/// - **ERI Exclusion**: Must not include ERI (Error T82)
/// - **Character Set**: Must use valid SWIFT character set
///
/// ## Structured Code Requirements
/// ### Mandatory Code Format
/// - **Line 1**: `/8c/[additional information]` - Code followed by narrative
/// - **Lines 2-6**: Continuation with '//' or new codes
/// - **Code Uniqueness**: Each code should appear only once
/// - **Format Compliance**: Exact adherence to code structure required
///
/// ### Primary Code: INS (Instructing Institution)
/// - **Purpose**: Identifies instructing institution
/// - **Format**: /INS/[BIC code]
/// - **Validation**: Must be followed by valid BIC
/// - **Uniqueness**: Must be unique within message
/// - **Usage**: Critical for institutional identification
///
/// ## Regional Considerations
/// - **European Networks**: SEPA and TARGET2 institutional communications
/// - **US Systems**: Federal Reserve and commercial bank coordination
/// - **Asian Markets**: Regional institutional communication requirements
/// - **Cross-Border**: International institutional coordination
///
/// ## Common Codes and Usage
/// ### Institutional Codes
/// - **ACC**: Account information and details
/// - **BENF**: Beneficiary related information
/// - **CNTR**: Country specific information
/// - **INST**: Instruction for next agent
/// - **INT**: Intermediary information
/// - **PHONBEN**: Phone number of beneficiary
/// - **PHONORD**: Phone number of ordering party
/// - **REC**: Receiver information
/// - **TELE**: Telecommunication details
///
/// ### Processing Instructions
/// - **BNF**: Details of beneficiary
/// - **COMM**: Commission and charges
/// - **DETL**: Transaction details
/// - **INTA**: Instructing agent
/// - **PAYA**: Paying agent
/// - **RECD**: Received from
/// - **SVCLVL**: Service level
///
/// ## Error Prevention Guidelines
/// - **Code Validation**: Verify codes are properly formatted and valid
/// - **Slash Format**: Ensure correct slash placement for codes
/// - **Character Limits**: Respect 35-character line limit
/// - **Prohibited Content**: Avoid restricted codes (REJT, RETN)
///
/// ## Related Fields Integration
/// - **Field 70**: Remittance Information (beneficiary details)
/// - **Field 77B**: Regulatory Reporting (compliance information)
/// - **Field 23E**: Instruction Code (processing instructions)
/// - **Field 33B**: Currency/Instructed Amount (amount details)
///
/// ## Compliance Framework
/// - **Regulatory Reporting**: Compliance information transmission
/// - **Anti-Money Laundering**: Additional verification details
/// - **Sanctions Screening**: Supplementary screening information
/// - **Audit Trail**: Complete institutional communication record
///
/// ## Best Practices
/// - **Clear Instructions**: Use unambiguous instruction codes
/// - **Structured Format**: Follow code/narrative structure
/// - **Concise Content**: Keep information brief and relevant
/// - **Code Consistency**: Use standard codes consistently
///
/// ## Message Type Usage
/// - **MT 103**: Payment instructions and institutional details
/// - **MT 202**: Financial institution transfer instructions
/// - **MT 199**: Free format message with structured codes
/// - **MT 299**: Free format financial institution transfer
///
/// ## Examples
/// ```logic
/// :72:/ACC/GB29NWBK60161331926819
/// :72:/BNF/BENEFICIARY DETAILS
/// :72:/PHONBEN/+1-555-123-4567
/// :72:/INST/CREDIT ACCOUNT IMMEDIATELY
/// ```
///
/// ## See Also
/// - Swift FIN User Handbook: Sender to Receiver Information
/// - MT Standards: Field 72 Specifications
/// - Correspondent Banking: Institutional Communication Standards
/// - Payment Processing: Bank-to-Bank Information Guidelines
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field72 {
    /// Sender to receiver information
    ///
    /// Format: 6*35x - Up to 6 lines of 35 characters each
    /// Contains structured institutional communications with codes and narrative
    /// Line 1: /8c/[additional information], subsequent lines: continuation or new codes
    pub information: Vec<String>,
}

impl SwiftField for Field72 {
    fn parse(input: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Parse as multiline text (up to 6 lines, 35 chars each)
        let information = parse_multiline_text(input, 6, 35)?;

        if information.is_empty() {
            return Err(ParseError::InvalidFormat {
                message: "Field 72 must have at least one line of information".to_string(),
            });
        }

        Ok(Field72 { information })
    }

    fn to_swift_string(&self) -> String {
        format!(":72:{}", self.information.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field72_single_line() {
        let field = Field72::parse("/BNF/BENEFICIARY DETAILS").unwrap();
        assert_eq!(field.information.len(), 1);
        assert_eq!(field.information[0], "/BNF/BENEFICIARY DETAILS");
    }

    #[test]
    fn test_field72_multiline() {
        let input = "/BNF/BENEFICIARY DETAILS\n/ACC/ACCOUNT INFO\n/REC/RECEIVER INFO";
        let field = Field72::parse(input).unwrap();
        assert_eq!(field.information.len(), 3);
        assert_eq!(field.information[0], "/BNF/BENEFICIARY DETAILS");
        assert_eq!(field.information[1], "/ACC/ACCOUNT INFO");
        assert_eq!(field.information[2], "/REC/RECEIVER INFO");
    }

    #[test]
    fn test_field72_continuation() {
        let input = "/INST/LONG INSTRUCTION THAT\n//CONTINUES ON NEXT LINE\n//AND ANOTHER LINE";
        let field = Field72::parse(input).unwrap();
        assert_eq!(field.information.len(), 3);
        assert!(field.information[1].starts_with("//"));
        assert!(field.information[2].starts_with("//"));
    }

    #[test]
    fn test_field72_to_swift_string() {
        let field = Field72 {
            information: vec!["/CODE1/INFO1".to_string(), "/CODE2/INFO2".to_string()],
        };
        assert_eq!(field.to_swift_string(), ":72:/CODE1/INFO1\n/CODE2/INFO2");
    }

    #[test]
    fn test_field72_max_lines() {
        let input = "LINE1\nLINE2\nLINE3\nLINE4\nLINE5\nLINE6";
        let field = Field72::parse(input).unwrap();
        assert_eq!(field.information.len(), 6);

        // Test that 7 lines would fail
        let too_many = "LINE1\nLINE2\nLINE3\nLINE4\nLINE5\nLINE6\nLINE7";
        assert!(Field72::parse(too_many).is_err());
    }

    #[test]
    fn test_field72_line_length() {
        // Test max length line (35 chars)
        let max_line = format!("/ACC/{}", "A".repeat(30));
        assert_eq!(max_line.len(), 35);
        let field = Field72::parse(&max_line).unwrap();
        assert_eq!(field.information[0].len(), 35);

        // Test too long line (36 chars)
        let too_long = "A".repeat(36);
        assert!(Field72::parse(&too_long).is_err());
    }
}
