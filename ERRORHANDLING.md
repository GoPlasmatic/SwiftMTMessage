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

```rust
// Recommended error enum structure
#[derive(Debug, Clone, PartialEq)]
pub enum SwiftValidationError {
    Format(SwiftFormatError),
    Business(SwiftBusinessError),
    Content(SwiftContentError),
    Relation(SwiftRelationError),
    General(SwiftGeneralError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwiftFormatError {
    pub code: String,        // e.g., "T50"
    pub field: String,       // e.g., "32A"
    pub value: String,       // Invalid value
    pub expected: String,    // Expected format
    pub message: String,     // Human-readable description
}
```

### When to Throw Each Error Type

#### During Parsing
- **T-Series errors**: Immediate format validation failures
- **Field format errors**: Invalid characters, length violations, pattern mismatches
- **Syntax errors**: Malformed message structure

#### During Field Validation
- **Content validation**: Currency codes, country codes, BIC format
- **Range validation**: Amount limits, date ranges
- **Enumeration validation**: Invalid code values

#### During Business Rule Validation
- **C-Series errors**: Conditional business logic violations
- **D-Series errors**: Regional requirements, field dependencies
- **E-Series errors**: Complex instruction code logic

#### During Cross-Field Validation
- **Currency consistency**: Related fields with different currencies
- **Amount relationships**: Field totals and calculations
- **Conditional presence**: Required fields based on other field values

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

### User-Friendly Error Messages

```rust
impl Display for SwiftValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SwiftValidationError::Format(err) => {
                write!(f, "Format Error {}: Field {} contains '{}', expected {}. {}",
                    err.code, err.field, err.value, err.expected, err.message)
            }
            SwiftValidationError::Business(err) => {
                write!(f, "Business Rule Violation {}: {} (Field: {})",
                    err.code, err.message, err.field)
            }
            // ... other error types
        }
    }
}
```

### Error Context Enrichment

Provide context-specific guidance:
- **Field-specific help**: What the field should contain
- **Business context**: Why the rule exists
- **Correction suggestions**: How to fix the error
- **Related documentation**: SWIFT standard references

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

### Basic Usage

```rust
use swift_mt_message::{SwiftMessage, SwiftValidationError};

// Parse and validate a message
match SwiftMessage::from_str(message_text) {
    Ok(message) => {
        match message.validate() {
            Ok(_) => println!("Message valid"),
            Err(errors) => {
                for error in errors {
                    println!("Validation Error: {}", error);
                }
            }
        }
    }
    Err(parse_error) => {
        println!("Parse Error: {}", parse_error);
    }
}
```

### Custom Error Handling

```rust
// Handle specific error types
match validation_result {
    Err(SwiftValidationError::Format(fmt_err)) if fmt_err.code == "T50" => {
        println!("Date format error: Use YYMMDD format");
    }
    Err(SwiftValidationError::Content(content_err)) if content_err.code == "D19" => {
        println!("IBAN required for SEPA payments");
    }
    Err(other_error) => {
        println!("Other validation error: {}", other_error);
    }
    Ok(_) => println!("Validation successful"),
}
```

## Conclusion

This error handling system ensures strict compliance with SWIFT standards while providing clear, actionable feedback to developers. The comprehensive error code coverage (1,335 codes) guarantees that all SWIFT validation rules are properly implemented and enforced.

For questions about specific error codes or validation scenarios, refer to the SWIFT Standards Release Guide 2025 or the library's test suite for practical examples.

---

*This documentation is based on SWIFT Standards Release Guide 2025. Error codes and validation rules are subject to updates in future SWIFT standard releases.*