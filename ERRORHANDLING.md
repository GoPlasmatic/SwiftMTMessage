# SWIFT MT Message Error Handling Guide

This document provides comprehensive guidance on error handling in the SwiftMTMessage library, based on official SWIFT Standards Release Guide 2025. The library implements strict validation according to SWIFT network validation rules to ensure financial message compliance.

## Overview

The SwiftMTMessage library implements SWIFT standard error codes to provide precise feedback when message parsing or validation fails. These error codes match those used by the SWIFT network, ensuring consistency across financial messaging systems.

**Total Error Codes**: 1,335 unique error codes across all MT categories  
**Standards Compliance**: SWIFT Standards Release Guide 2025  
**Coverage**: All MT message types (Categories 1-9 and Common Group)

## Error Code Categories

### T-Series: Technical/Format Validation (275 codes)
Technical validation errors for field format, structure, and basic syntax compliance.

#### Common T-Series Error Codes

| Code | Description | When Thrown | Example |
|------|-------------|-------------|---------|
| **T08** | Invalid code in field | Field contains invalid enumerated value | Field 71A contains "ABC" instead of "BEN", "OUR", or "SHA" |
| **T26** | Invalid slash usage | Field starts/ends with '/' or contains '//' | Field 59 contains "//ABC" or "ABC//" |
| **T27** | Invalid BIC code format | BIC doesn't match required pattern | Field 57A contains "ABCD12" instead of "ABCDUS33XXX" |
| **T28** | Invalid BIC code length | BIC length not 8 or 11 characters | Field 52A contains "ABCDUS3" (7 chars) |
| **T29** | Invalid BIC code structure | BIC structure violates ISO 9362 | Field 53A contains "123DUS33XXX" (starts with numbers) |
| **T40** | Invalid amount format | Amount doesn't match decimal pattern | Field 32A contains "1234.567" for JPY (no decimals allowed) |
| **T43** | Amount exceeds maximum digits | Amount has too many digits for currency | Field 33B contains "123456789012345.12" (exceeds 15 digits) |
| **T45** | Invalid identifier code format | Party identifier format violation | Field 50A contains invalid account number format |
| **T50** | Invalid date format | Date not in YYMMDD format | Field 30 contains "2023-12-25" instead of "231225" |
| **T52** | Invalid currency code | Currency not ISO 4217 compliant | Field 32A contains "XYZ" instead of "USD" |
| **T56** | Invalid structured address | Address format violation | Field 59 contains improperly formatted address lines |
| **T73** | Invalid country code | Country code not ISO 3166 compliant | Field 59 contains "XX" instead of "US" |

### C-Series: Conditional/Business Rules (57 codes)
Business logic validation for conditional fields and cross-field relationships.

#### Common C-Series Error Codes

| Code | Description | When Thrown | Example |
|------|-------------|-------------|---------|
| **C02** | Currency code mismatch | Related fields have different currencies | Field 32A has "USD" but field 71G has "EUR" |
| **C03** | Amount format validation | Amount violates currency-specific rules | EUR amount "1234.567" (EUR allows max 2 decimals) |
| **C08** | Commodity currency not allowed | Precious metal currency codes prohibited | Field 32A contains "XAU" (gold) in MT103 |
| **C81** | Conditional field dependency | Required field missing based on condition | Field 56A present but required field 57A missing |

### D-Series: Data/Content Validation (77 codes)
Content-specific validation rules including regional requirements and field dependencies.

#### Common D-Series Error Codes

| Code | Description | When Thrown | Example |
|------|-------------|-------------|---------|
| **D17** | Field presence requirement | Mandatory field missing in sequence | Field 50A must be present in sequence A or each sequence B |
| **D18** | Mutually exclusive placement | Fields incorrectly placed between sequences | Field 71A in wrong sequence position |
| **D19** | IBAN mandatory for SEPA | IBAN required for European countries | EU payment without IBAN in beneficiary field |
| **D20** | Field 71A presence rules | Charge field placement violation | Field 71A missing when required between sequences |
| **D22** | Exchange rate dependency | Field 36 required when applicable | Different currencies without exchange rate |
| **D49** | Field 33B mandatory for EU | Instructed amount required for EU transfers | Intra-European payment missing field 33B |
| **D50** | SHA charge restrictions | Field restrictions for shared charges | Field 71F present with SHA charge code |
| **D51** | Field 33B with charge fields | Instructed amount required with charges | Charge fields present without field 33B |
| **D75** | Exchange rate mandatory | Rate required for currency differences | Field 32A "USD" and 33B "EUR" without field 36 |
| **D79** | Field 71G dependency | Charge field dependency between sequences | Field 71G in sequence B requires sequence C |
| **D93** | Account restrictions by code | Account limitations based on operation | CHQB operation with restricted account format |

### E-Series: Enhanced/Field Relation Validation (86 codes)
Advanced validation for instruction codes, field options, and complex business rules.

#### Common E-Series Error Codes

| Code | Description | When Thrown | Example |
|------|-------------|-------------|---------|
| **E01** | Instruction code restrictions | Invalid code combination | Field 23E with SPRI can only contain SDVA, TELB, PHOB, INTC |
| **E02** | Prohibited instruction codes | Instruction code not allowed | Field 23E contains SSTD/SPAY when prohibited |
| **E03** | Field option restrictions | Invalid field option for context | Field 53A option D not allowed with SPRI/SSTD/SPAY |
| **E04** | Party identifier requirements | Required identifier missing | Party identifier mandatory in specific contexts |
| **E05** | Field 54A option restrictions | Invalid option for intermediary | Field 54A using prohibited option |
| **E06** | Multiple field dependency | Multiple fields required together | Field 55A present requires both 53A and 54A |
| **E07** | Field 55A option restrictions | Invalid option for receiving agent | Field 55A using prohibited option |
| **E09** | Party identifier mandatory | Identifier required for option D | Field 57A option D without party identifier |
| **E10** | Beneficiary account mandatory | Account required for operation code | Account mandatory in beneficiary for specific codes |
| **E13** | OUR charge restrictions | Field limitations for payer charges | Field 71F not allowed with OUR charges |
| **E15** | BEN charge requirements | Field mandatory for beneficiary charges | Field 71F mandatory with BEN charges |
| **E16** | Field restrictions with SPRI | Field not allowed with instruction | Field 56A not allowed when SPRI present |
| **E17** | Clearing code requirements | Code required for option C | Option C requires clearing code |
| **E18** | Account restrictions CHQB | Account limitations for cheque books | CHQB operation code account restrictions |
| **E44** | Instruction code dependencies | Code dependency on field presence | Instruction code requires specific field |
| **E45** | Instruction code field dependencies | Field dependency on instruction code | Field required when instruction code present |

### G-Series: General/Field Validation (823 codes)
The largest category covering general field validation across all MT categories, particularly prominent in Categories 2, 3, 5-9.

#### Sample G-Series Error Codes

| Code | Description | When Thrown | Example |
|------|-------------|-------------|---------|
| **G001** | Field format violation | General field format error | Field doesn't match specified format pattern |
| **G050** | Field content validation | Invalid field content | Field content violates specific business rules |
| **G100** | Sequence validation | Message sequence error | Fields not in required sequence order |

## Error Code Usage by MT Categories

### Category 1: Customer Payments and Cheques (MT 101-199)
- **Primary Error Series**: T, C, D, E
- **Key Focus**: Payment instruction validation, beneficiary requirements, charge handling
- **Common Scenarios**: BIC validation, IBAN requirements, charge code logic

### Category 2: Financial Institution Transfers (MT 200-299)
- **Primary Error Series**: G, C, T
- **Key Focus**: Bank-to-bank transfer validation, correspondent banking
- **Common Scenarios**: Intermediary bank validation, cover payment rules

### Category 3: Treasury Markets - FX/Money Markets/Derivatives (MT 300-399)
- **Primary Error Series**: C, D, E, T
- **Key Focus**: Trade confirmation, settlement instructions, rate validation
- **Common Scenarios**: Currency pair validation, settlement date rules

### Category 4: Collections and Cash Letters (MT 400-499)
- **Primary Error Series**: Limited validation codes
- **Key Focus**: Documentary collection instructions
- **Common Scenarios**: Document handling, collection terms

### Category 5: Securities Markets (MT 500-599)
- **Primary Error Series**: C, D, E, G
- **Key Focus**: Securities trade confirmation, settlement, corporate actions
- **Common Scenarios**: ISIN validation, settlement date rules, quantity validation

### Category 6: Treasury Markets - Commodities & Reference Data (MT 600-699)
- **Primary Error Series**: C, D, E, G
- **Key Focus**: Commodity trade confirmation, reference data validation
- **Common Scenarios**: Commodity codes, delivery terms, pricing validation

### Category 7: Documentary Credits and Guarantees (MT 700-799)
- **Primary Error Series**: C, D, E, G
- **Key Focus**: Letter of credit validation, guarantee terms
- **Common Scenarios**: Credit terms validation, document requirements

### Category 8: Travellers Cheques (MT 800-899)
- **Primary Error Series**: C, G
- **Key Focus**: Traveller cheque processing
- **Common Scenarios**: Cheque validation, issuer verification

### Category 9: Cash Management and Customer Status (MT 900-999)
- **Primary Error Series**: C, G
- **Key Focus**: Account statements, balance reporting, status messages
- **Common Scenarios**: Balance validation, statement formatting

## Implementation Guidelines

### Error Handling Architecture

### Enhanced Error Types

The library now uses enhanced error types that provide rich contextual information:

```rust
// Main parse error enum with enhanced variants
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ParseError {
    // Field format error with full context
    #[error("Invalid field format - Field: {field_tag}, Component: {component_name}, Value: '{value}', Expected: {format_spec}")]
    InvalidFieldFormat {
        field_tag: String,        // SWIFT field tag (e.g., "50K", "32A")
        component_name: String,   // Component within field (e.g., "currency", "amount")
        value: String,           // The actual value that failed
        format_spec: String,     // Expected format specification
        position: Option<usize>, // Position in original message
        inner_error: String,     // Detailed parsing error
    },

    // Missing required field with context
    #[error("Missing required field {field_tag} ({field_name}) in {message_type}")]
    MissingRequiredField {
        field_tag: String,       // SWIFT field tag
        field_name: String,      // Rust field name in struct
        message_type: String,    // Message type (MT103, MT202, etc.)
        position_in_block4: Option<usize>, // Expected position
    },

    // Field parsing failure with position
    #[error("Failed to parse field {field_tag} of type {field_type} at position {position}")]
    FieldParsingFailed {
        field_tag: String,
        field_type: String,
        position: usize,         // Encoded position (line << 16 | field_pos)
        original_error: String,
    },

    // Component parse error
    #[error("Component parse error in field {field_tag}: {component_name} (index {component_index})")]
    ComponentParseError {
        field_tag: String,
        component_index: usize,
        component_name: String,
        expected_format: String,
        actual_value: String,
    },

    // Invalid block structure
    #[error("Invalid block {block} structure: {message}")]
    InvalidBlockStructure {
        block: String,           // Block number (1-5)
        message: String,         // Detailed error message
    },
}

// SWIFT validation errors remain structured by series
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SwiftValidationError {
    Format(Box<SwiftFormatError>),    // T-Series
    Business(Box<SwiftBusinessError>), // C-Series  
    Content(Box<SwiftContentError>),   // D-Series
    Relation(Box<SwiftRelationError>), // E-Series
    General(Box<SwiftGeneralError>),   // G-Series
}
```

### Enhanced Error Methods

The enhanced errors provide helpful methods for debugging:

```rust
impl ParseError {
    /// Get a detailed debug report with tree formatting
    pub fn debug_report(&self) -> String { /* ... */ }
    
    /// Get a concise error message for logging
    pub fn brief_message(&self) -> String { /* ... */ }
    
    /// Format error with original message context
    pub fn format_with_context(&self, original_message: &str) -> String { /* ... */ }
}
```

### When to Use Each Error Type

#### Enhanced Parse Errors (Primary)

1. **InvalidFieldFormat** - Use when field content doesn't match expected format
   - Component-level parsing failures
   - Format specification violations
   - Type conversion errors
   - Includes position tracking for debugging

2. **MissingRequiredField** - Use when mandatory field is absent
   - Message-level field requirements
   - Includes field name and message type context
   - Tracks expected position in block 4

3. **FieldParsingFailed** - Use for general field parsing failures
   - Higher-level field errors
   - Includes encoded position (line number + field position)
   - Preserves original error details

4. **ComponentParseError** - Use for specific component failures
   - Multi-component field errors
   - Identifies exact component that failed
   - Includes expected vs actual format

5. **InvalidBlockStructure** - Use for SWIFT block structure errors
   - Block 1-5 parsing failures
   - Malformed message structure
   - Block-specific error details

#### SWIFT Validation Errors (Secondary)

- **T-Series errors**: Format validation after parsing
- **C-Series errors**: Business rule validation
- **D-Series errors**: Regional and content requirements
- **E-Series errors**: Complex relationship validation
- **G-Series errors**: General field validation

### Line Number Tracking

The library tracks line numbers for error reporting:

```rust
// FieldParsingFailed now includes the line number where the field was found
ParseError::FieldParsingFailed {
    field_tag: "23B",          // Which field failed
    field_type: "Field23B",    // The type being parsed
    position: 2,               // Line number in the message
    original_error: "...",     // Detailed error description
}
```

### Error Recovery Strategies

1. **Immediate Failure**: Critical format errors that prevent parsing
2. **Collect and Report**: Non-critical errors for batch validation
3. **Contextual Guidance**: Suggest corrections based on error type
4. **Progressive Validation**: Validate in stages (format → content → business rules)

## Regional Validation Rules

### SEPA (Single Euro Payments Area) Requirements
- **D19**: IBAN mandatory for EU countries
- **D49**: Field 33B mandatory for intra-European transfers
- **Covered Countries**: AD, AT, BE, BG, CH, CY, CZ, DE, DK, EE, ES, FI, FR, GI, GR, HR, HU, IE, IS, IT, LI, LT, LU, LV, MC, MT, NL, NO, PL, PT, RO, SE, SI, SK, SM, VA

### Charge Code Validation
- **SHA (Shared)**: D50 restrictions apply
- **OUR (Payer)**: E13 restrictions apply  
- **BEN (Beneficiary)**: E15 requirements apply

### Amount and Currency Validation
- **Currency-Specific Decimals**: JPY (0), EUR (2), BHD (3)
- **Maximum Amount Digits**: 15 digits total including decimals
- **Commodity Currencies**: XAU, XAG, XPD, XPT prohibited in payment messages

## Testing and Validation

### Test Categories

1. **Format Validation Tests**: Each T-series error code
2. **Business Rule Tests**: Each C, D, E-series scenario  
3. **Regional Compliance Tests**: SEPA and geographic rules
4. **Cross-Field Validation Tests**: Multi-field dependencies
5. **Edge Case Tests**: Boundary conditions and corner cases

### Sample Test Cases

```rust
#[test]
fn test_invalid_bic_format() {
    let invalid_bic = "ABCD12"; // Too short
    let result = parse_field_57a(&invalid_bic);
    assert!(matches!(result, Err(SwiftValidationError::Format(
        SwiftFormatError { code: "T28", .. }
    ))));
}

#[test]
fn test_sepa_iban_requirement() {
    let eu_payment = create_mt103_without_iban("DE", "FR");
    let result = validate_message(&eu_payment);
    assert!(matches!(result, Err(SwiftValidationError::Content(
        SwiftContentError { code: "D19", .. }
    ))));
}
```

## Error Message Guidelines

### Enhanced Error Display

The enhanced errors provide rich, actionable error messages:

```rust
// Example error output with debug_report():
Field Parsing Error:
├─ Field Tag: 32A
├─ Component: amount
├─ Value: '1,000.00'
├─ Expected Format: 15d (decimal with period separator)
├─ Position in Message: Line 5
├─ Details: Invalid decimal number: expected period not comma
└─ Hint: Check SWIFT format specification for field 32A

// Example with format_with_context():
Field Parsing Failed:
├─ Field Tag: 32A
├─ Field Type: Field32A
├─ Position: 327681  // Line 5, field position 1
├─ Error: Component parse error
└─ Hint: Check the field value matches the expected type

Context:
    3 │ :23B:CRED
    4 │ :50K:JOHN DOE
>>> 5 │ :32A:240315USD1,000.00
    6 │ :59:JANE SMITH
    7 │ :71A:SHA
```

### Error Context Methods

```rust
// Get detailed debug output
let debug_info = error.debug_report();

// Get brief message for logs
let log_msg = error.brief_message();
// Output: "Field 32A component 'amount' format error"

// Get error with surrounding context
let context = error.format_with_context(&original_message);
```

### Practical Error Handling

```rust
use swift_mt_message::{SwiftParser, ParseError};

match SwiftParser::parse::<MT103>(&message) {
    Ok(parsed) => process_message(parsed),
    Err(e) => match e {
        ParseError::InvalidFieldFormat { 
            field_tag, 
            component_name,
            value,
            format_spec,
            .. 
        } => {
            eprintln!("Field {} error: {} '{}' doesn't match {}", 
                     field_tag, component_name, value, format_spec);
            // Provide specific guidance based on component
            match component_name.as_str() {
                "currency" => eprintln!("Use 3-letter ISO currency code"),
                "amount" => eprintln!("Use decimal point, not comma"),
                "date" => eprintln!("Use YYMMDD format"),
                _ => eprintln!("Check SWIFT format guide")
            }
        },
        ParseError::MissingRequiredField { field_tag, field_name, .. } => {
            eprintln!("Missing {}: {} is required", field_tag, field_name);
        },
        _ => eprintln!("Parse error: {}", e.debug_report())
    }
}
```

## Migration and Compatibility

### Version Compatibility
- **2025 Standards**: Current implementation
- **Future Standards**: Extensible error code system
- **Backward Compatibility**: Support for older message versions

### Error Code Evolution
- New error codes added in SWIFT standard updates
- Deprecated error codes maintained for compatibility
- Version-specific validation rules

## Performance Considerations

### Validation Order
1. **Fast Format Checks**: Basic syntax and format
2. **Content Validation**: Field-specific rules
3. **Business Rules**: Complex conditional logic
4. **Cross-Validation**: Multi-field relationships

### Early Exit Strategies
- Stop on critical format errors
- Continue for business rule validation
- Batch non-critical errors for reporting

## Integration Examples

### Basic Usage with Enhanced Errors

```rust
use swift_mt_message::{SwiftParser, ParseError, messages::MT103};

// Parse with enhanced error handling
match SwiftParser::parse::<MT103>(&raw_message) {
    Ok(message) => {
        println!("Successfully parsed MT103");
        // Additional validation if needed
        match message.validate() {
            Ok(_) => println!("Message valid"),
            Err(validation_errors) => {
                for error in validation_errors {
                    println!("Validation: {}", error);
                }
            }
        }
    }
    Err(e) => {
        // Use enhanced error methods
        eprintln!("Parse failed: {}", e.brief_message());
        eprintln!("\nDetails:\n{}", e.debug_report());
        
        // Show context if available
        if let Some(context) = get_original_message() {
            eprintln!("\n{}", e.format_with_context(&context));
        }
    }
}
```

### Production Error Handling

```rust
use swift_mt_message::{SwiftParser, ParseError, ParsedSwiftMessage};
use log::{error, warn, info};

/// Process SWIFT message with comprehensive error handling
fn process_swift_message(raw: &str) -> Result<ProcessedMessage, ProcessingError> {
    match SwiftParser::parse_auto(raw) {
        Ok(parsed) => {
            info!("Parsed message type: {}", parsed.message_type());
            process_parsed_message(parsed)
        }
        Err(e) => {
            // Log brief error for monitoring
            error!("SWIFT parse error: {}", e.brief_message());
            
            // Detailed logging for debugging
            error!("Parse details: {}", e.debug_report());
            
            // Handle specific error types
            match e {
                ParseError::InvalidFieldFormat { field_tag, component_name, .. } => {
                    warn!("Field {} component {} failed validation", field_tag, component_name);
                    Err(ProcessingError::InvalidField(field_tag))
                }
                ParseError::MissingRequiredField { field_tag, message_type, .. } => {
                    warn!("Required field {} missing in {}", field_tag, message_type);
                    Err(ProcessingError::MissingField(field_tag))
                }
                ParseError::InvalidBlockStructure { block, .. } => {
                    error!("Malformed block {}", block);
                    Err(ProcessingError::MalformedMessage)
                }
                _ => Err(ProcessingError::GeneralParseError(e.to_string()))
            }
        }
    }
}
```

### Error Recovery Strategies

```rust
/// Attempt to recover from parsing errors
fn parse_with_recovery(raw: &str) -> Result<PartialMessage, String> {
    match SwiftParser::parse_auto(raw) {
        Ok(msg) => Ok(PartialMessage::Complete(msg)),
        Err(e) => {
            match e {
                ParseError::MissingRequiredField { field_tag, .. } => {
                    // Try parsing as draft/incomplete message
                    info!("Attempting partial parse without field {}", field_tag);
                    parse_as_draft(raw)
                }
                ParseError::InvalidFieldFormat { field_tag, position, .. } => {
                    // Skip invalid field and continue
                    if let Some(pos) = position {
                        let line_num = pos >> 16;
                        info!("Skipping invalid field {} at line {}", field_tag, line_num);
                        parse_without_field(raw, &field_tag)
                    } else {
                        Err(format!("Cannot recover from field {} error", field_tag))
                    }
                }
                _ => {
                    // Log full context for investigation
                    error!("Unrecoverable error:\n{}", e.format_with_context(raw));
                    Err(e.to_string())
                }
            }
        }
    }
}
```

## Best Practices with Enhanced Errors

### 1. Use Error Context Methods

Always leverage the enhanced error methods for better debugging:

```rust
match result {
    Err(e) => {
        // For user-facing messages
        println!("Error: {}", e.brief_message());
        
        // For detailed logs
        log::error!("{}", e.debug_report());
        
        // For debugging with context
        if let Some(original) = get_original_message() {
            eprintln!("{}", e.format_with_context(&original));
        }
    }
    Ok(_) => { /* ... */ }
}
```

### 2. Component-Level Error Handling

Handle errors at the component level for precise recovery:

```rust
match error {
    ParseError::InvalidFieldFormat { component_name, .. } => {
        match component_name.as_str() {
            "currency" => suggest_valid_currencies(),
            "amount" => explain_decimal_format(),
            "date" => show_date_format_examples(),
            _ => show_general_format_help()
        }
    }
    _ => { /* ... */ }
}
```

### 3. Line-Aware Debugging

Use line number information for targeted debugging:

```rust
if let ParseError::FieldParsingFailed { position, field_tag, .. } = error {
    println!("Error parsing field {} at line {}", field_tag, position);
    // Show specific line from original message
    show_message_line(original_message, position);
}
```

### 4. Error Recovery Patterns

Implement graceful degradation based on error types:

```rust
fn parse_with_fallback(message: &str) -> ParseResult {
    match SwiftParser::parse(message) {
        Ok(msg) => Ok(msg),
        Err(ParseError::MissingRequiredField { field_tag, .. }) 
            if is_recoverable_field(&field_tag) => {
            // Try with default value
            parse_with_default_field(message, &field_tag)
        }
        Err(ParseError::InvalidFieldFormat { field_tag, position, .. }) => {
            // Skip invalid field and continue
            parse_excluding_field(message, &field_tag, position)
        }
        Err(e) => Err(e)
    }
}
```

## Conclusion

The enhanced error handling system in SwiftMTMessage provides both SWIFT standards compliance (1,335 error codes) and rich contextual information for effective debugging. The dual approach of enhanced parse errors for immediate feedback and SWIFT validation errors for compliance ensures developers have all the tools needed for robust financial message processing.

Key benefits:
- **Precise Error Location**: Line numbers and field positions
- **Component Identification**: Exact component that failed within complex fields
- **Actionable Messages**: Clear format specifications and hints
- **Debugging Context**: Surrounding message lines for investigation
- **Standards Compliance**: Full SWIFT error code coverage

For questions about specific error codes or validation scenarios, refer to the SWIFT Standards Release Guide 2025 or the library's test suite for practical examples.

---

*This documentation is based on SWIFT Standards Release Guide 2025. Error codes and validation rules are subject to updates in future SWIFT standard releases.*