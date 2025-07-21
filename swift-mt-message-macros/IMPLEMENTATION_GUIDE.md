# Swift MT Message Implementation Guide

This guide provides the minimal information needed to implement new SWIFT MT message types using the swift-mt-message-macros library.

## Quick Start

### 1. Implementing a Simple Field

```rust
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field20 {
    #[component("16x")]
    pub reference: String,
}
```

The `#[component("format")]` attribute defines the SWIFT format specification.

### 2. Implementing a Complex Field with Multiple Components

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field32A {
    #[component("6!n")]     // YYMMDD format
    pub date: String,
    #[component("3!a")]     // Currency code
    pub currency: String,
    #[component("15d")]     // Amount with decimals
    pub amount: f64,
}
```

### 3. Implementing an Enum Field (Multiple Variants)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field50 {
    A(Field50A),
    F(Field50F),
    K(Field50K),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50A {
    #[component("[/34x]")]
    pub party_identifier: Option<String>,
    #[component("4*(1!n/33x)")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field50K {
    #[component("[/34x]")]
    pub account: Option<String>,
    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}
```

### 4. Implementing a Message

```rust
use crate::fields::*;
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::{serde_swift_fields, SwiftMessage};

#[serde_swift_fields]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
pub struct MT999 {
    // Mandatory fields
    #[field("20")]
    pub field_20: Field20,
    
    #[field("12")]
    pub field_12: Field12,
    
    // Optional fields
    #[field("77E")]
    pub field_77e: Option<Field77E>,
    
    // Repetitive fields
    #[field("79")]
    pub field_79: Option<Vec<Field79>>,
}
```

## SWIFT Format Specifications

### Basic Format Types

| Format | Description | Example | Rust Type |
|--------|-------------|---------|-----------|
| `n` | Numeric (0-9) | `6!n` → "123456" | `String` |
| `a` | Alphabetic (A-Z) | `3!a` → "USD" | `String` |
| `c` | Alphanumeric (A-Z, 0-9) | `4!c` → "AB12" | `String` |
| `x` | Any character | `35x` → "Any text here" | `String` |
| `d` | Decimal number | `15d` → "1234.56" | `f64` |

### Format Modifiers

| Modifier | Description | Example |
|----------|-------------|---------|
| `!` | Fixed length | `3!a` → exactly 3 letters |
| `[...]` | Optional | `[/34x]` → optional account |
| `*` | Repetitive | `4*35x` → up to 4 lines of 35 chars |
| `/` | Literal slash | `/34x` → starts with "/" |

### Common Patterns

- **BIC Code**: `4!a2!a2!c[3!c]` - Bank identifier code
- **Date**: `6!n` or `8!n` - YYMMDD or YYYYMMDD
- **Amount**: `15d` - Decimal with up to 15 digits
- **Account**: `[/34x]` - Optional account starting with "/"
- **Multi-line**: `4*35x` - Up to 4 lines of 35 characters

## Field Type Mapping

The macro automatically handles type conversions:

| SWIFT Format | Rust Type | Notes |
|--------------|-----------|-------|
| Any format | `String` | Default for text |
| `[...]` | `Option<T>` | Optional fields |
| `n*...` | `Vec<String>` | Repetitive fields |
| `d` format | `f64` | Decimal numbers |

## Advanced Features

### Custom Validation Rules

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftMessage)]
#[validation_rules(MT103_VALIDATION_RULES)]
pub struct MT103 {
    // fields...
}

// Define validation rules separately
pub const MT103_VALIDATION_RULES: &[ValidationRule] = &[
    ValidationRule::FieldPresence {
        if_present: "23E",
        then_required: &["50F"],
    },
];
```

### Serde Integration

The `#[serde_swift_fields]` attribute automatically adds proper serialization attributes for clean JSON output without enum variant wrappers.

## Implementation Checklist

When implementing a new MT message:

1. **Define all required fields** - Create field structs with proper format specifications
2. **Handle field variants** - Use enums for fields with multiple options (A/F/K variants)
3. **Create the message struct** - Use `#[derive(SwiftMessage)]` with field attributes
4. **Add serde support** - Include `#[serde_swift_fields]` for JSON serialization
5. **Test parsing** - Ensure your message can parse real SWIFT data
6. **Test serialization** - Verify the output matches SWIFT format

## Troubleshooting

### Common Issues

1. **Parse errors** - Check that your format specifications match the actual SWIFT data
2. **Missing fields** - Ensure all mandatory fields are marked without `Option<>`
3. **Wrong types** - Verify Vec<> for repetitive fields, Option<> for optional
4. **Enum variants** - The variant name (A, F, K) must match the SWIFT variant letter

### Enhanced Error Debugging

The macro-generated code now provides rich error context:

```rust
// Example of enhanced error output
Field Parsing Error:
├─ Field Tag: 32A
├─ Component: amount
├─ Value: '1,000.00'
├─ Expected Format: 15d
├─ Position in Message: Line 5
├─ Details: Invalid decimal number '1,000.00': expected period not comma
└─ Hint: Check SWIFT format specification for field 32A

// The error includes:
// - Exact field and component that failed
// - The actual value that couldn't be parsed
// - Expected format specification
// - Position in the original message
// - Detailed error explanation
```

### Error Types Generated by Macros

The macros automatically generate errors with full context:

1. **Field-level errors** (`InvalidFieldFormat`):
   - Generated when field content doesn't match format
   - Includes field tag, component name, value, and format spec

2. **Missing field errors** (`MissingRequiredField`):
   - Generated when required field is absent
   - Includes field tag, field name, and message type

3. **Parsing failures** (`FieldParsingFailed`):
   - Generated for higher-level field parsing issues
   - Includes position tracking for debugging

4. **Component errors** (`ComponentParseError`):
   - Generated for multi-component field failures
   - Identifies exact component that failed

## Working Examples

To create working examples with the macros, create them in the main `swift-mt-message` crate (not the macro crate) and use these imports:

```rust
// For field examples
use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;
use swift_mt_message::{SwiftField, errors::ParseError};

// For message examples  
use swift_mt_message_macros::{SwiftMessage, serde_swift_fields};
use swift_mt_message::{SwiftMessageBody, SwiftParser};
use std::collections::HashMap;

// Example with enhanced error handling
fn parse_field_with_context(value: &str) -> Result<Field32A, ParseError> {
    match Field32A::parse(value) {
        Ok(field) => Ok(field),
        Err(e) => {
            // The macro-generated code provides rich error context
            eprintln!("Parse failed: {}", e.brief_message());
            eprintln!("Details:\n{}", e.debug_report());
            Err(e)
        }
    }
}
```

The macro crate itself only contains the procedural macros and cannot run full examples since it doesn't have access to the main library's traits and dependencies.

### Error Handling in Generated Code

The macros automatically generate error handling that provides:
- Field tag and component identification
- Format specification details
- Position tracking when available
- Helpful error messages for debugging