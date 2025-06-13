# Swift MT Message Parser

A modern Rust library for parsing SWIFT MT (Message Type) financial messages with **macro-based field definitions** and **serde-like automatic serialization**. Built for financial institutions requiring type-safe, high-performance SWIFT message processing.

## 🚀 Key Features

- **Macro-Driven Architecture**: `#[derive(SwiftField)]` and `#[derive(SwiftMessage)]` for automatic field and message generation
- **Serde-Like Design**: Familiar serialization patterns adapted for financial messaging standards
- **Type-safe Parsing**: Dedicated field structs with automatic validation
- **Comprehensive Field Support**: All MT103 fields with proper SWIFT compliance
- **Zero-Copy Parsing**: Efficient parsing with minimal memory allocations
- **Financial-Grade Validation**: Strict SWIFT compliance with comprehensive error reporting

## 🏗️ Macro-Based Architecture

### SwiftField Derive Macro

Define SWIFT fields with automatic parsing, validation, and serialization:

```rust
use swift_mt_message::SwiftField;

#[derive(SwiftField)]
#[format("4!c")]
pub struct Field23B {
    #[format("4!c")]
    pub bank_operation_code: String,
}

// Automatically generates:
// - parse() method with format validation
// - to_swift_string() method
// - validate() method with SWIFT rules
// - Serde serialization/deserialization
```

### SwiftMessage Derive Macro

Compose complete MT messages using field macros:

```rust
use swift_mt_message::{SwiftMessage, swift_serde};

#[derive(SwiftMessage)]
#[swift_serde(rename_all = "FIELD_TAGS")]
pub struct MT103 {
    #[field("20")]
    pub transaction_reference: Field20,
    
    #[field("23B")]
    pub bank_operation_code: Field23B,
    
    #[field("32A")]
    pub value_date_currency_amount: Field32A,
    
    // Automatically handles:
    // - Field validation
    // - SWIFT format compliance
    // - JSON serialization with field tags
    // - Error propagation
}
```

## 🎯 Serde-Like Design for Financial Messages

### Automatic Serialization

The library provides serde-like automatic serialization optimized for financial data:

```rust
use serde_json;
use swift_mt_message::{SwiftParser, SwiftMessage, messages::MT103};

// Parse SWIFT message
let mt103: SwiftMessage<MT103> = SwiftParser::parse(raw_swift_message)?;

// Automatic JSON serialization with financial field tags
let json = serde_json::to_string_pretty(&mt103)?;
```

**Output (Financial-Optimized JSON):**
```json
{
  "message_type": "103",
  "fields": {
    "20": {
      "transaction_reference": "FT21234567890"
    },
    "23B": {
      "bank_operation_code": "CRED"
    },
    "32A": {
      "date": "2021-03-15",
      "currency_code": "EUR",
      "amount": 1234567.89
    }
  }
}
```

### Custom Financial Serialization

Complex financial fields use custom serialization for clean JSON:

```rust
// Field 50 (Ordering Customer) - Complex enum with 3 variants
#[derive(SwiftField)]
pub enum Field50 {
    A(Field50A),  // Account + BIC
    F(Field50F),  // Party + Address
    K(Field50K),  // Name + Address
}

// Custom serialization flattens the structure:
// Instead of: {"50": {"K": {"name_and_address": [...]}}}
// Produces:   {"50": {"name_and_address": [...]}}
```

## 📋 Financial Field Types

### Institution Fields (Macro-Generated)

All institution fields are generated with consistent structure:

```rust
#[derive(SwiftField)]
#[format("institution")]
pub struct Field52A {
    pub account_line_indicator: Option<String>,
    pub account_number: Option<String>,
    pub bic: String,
}

// Auto-generated methods:
// - validate_bic() - 8 or 11 character validation
// - validate_account() - IBAN/account format checking
// - to_swift_format() - Proper SWIFT field formatting
```

### Complex Financial Enums

```rust
#[derive(SwiftField)]
pub enum Field50 {
    #[variant("A")]
    A(Field50A),  // Account + BIC format
    
    #[variant("F")]  
    F(Field50F),  // Party identifier + Address
    
    #[variant("K")]
    K(Field50K),  // Name + Address only
}

// Automatic variant detection during parsing
// Smart serialization without enum wrappers
```

## 🔧 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
swift-mt-message = "2.0.0"
```

## 📖 Usage Examples

### Basic Financial Message Processing

```rust
use swift_mt_message::{SwiftParser, SwiftMessage, messages::MT103};

let raw_mt103 = r#"{1:F01BANKDEFF0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:ACME CORPORATION
123 BUSINESS AVENUE
NEW YORK NY 10001
:52A:BANKDEFF
:57A:DEUTDEFF
:59A:/DE89370400440532013000
DEUTDEFF
:70:PAYMENT FOR SERVICES
:71A:OUR
-}"#;

// Macro-powered parsing with automatic validation
let parsed: SwiftMessage<MT103> = SwiftParser::parse(raw_mt103)?;

// Serde-like JSON serialization
let json = serde_json::to_string_pretty(&parsed)?;
println!("Financial Message JSON: {}", json);
```

### Working with Financial Field Macros

```rust
use swift_mt_message::fields::{Field50, Field50K, Field59, Field59A};

// Macro-generated field creation with validation
let ordering_customer = Field50::K(Field50K::new(vec![
    "ACME CORPORATION".to_string(),
    "123 BUSINESS AVENUE".to_string(),
    "NEW YORK NY 10001".to_string(),
])?);

let beneficiary = Field59::A(Field59A::new(
    Some("DE89370400440532013000".to_string()),
    "DEUTDEFF"
)?);

// Automatic SWIFT format generation
println!("SWIFT Format: {}", ordering_customer.to_swift_string());
```

### Financial Validation

```rust
use swift_mt_message::fields::Field52A;

// Macro-generated validation with financial rules
let institution = Field52A::new(
    Some("A".to_string()),           // Account line indicator
    Some("12345678901234567890".to_string()), // Account number
    "DEUTDEFF"                       // BIC code
)?;

// Automatic validation includes:
// - BIC format (8 or 11 characters)
// - Account number length (max 34 chars)
// - SWIFT compliance checking
assert!(institution.validate().is_valid);
```

## 🏗️ Macro Architecture

### Field Generation Pipeline

```
SWIFT Field Definition
        ↓
#[derive(SwiftField)] Macro
        ↓
Generated Implementation:
├── parse() - Format-aware parsing
├── validate() - SWIFT compliance
├── to_swift_string() - Format generation
└── Serde traits - JSON serialization
```

### Message Composition

```
Individual Fields (Macro-Generated)
        ↓
#[derive(SwiftMessage)] Macro
        ↓
Complete MT Message:
├── Field validation pipeline
├── Message structure validation
├── Automatic header handling
└── Financial JSON serialization
```

## 🎯 Financial-Grade Features

### SWIFT Compliance
- **Format Validation**: Automatic SWIFT format checking
- **Field Length Limits**: Enforced character limits per SWIFT standards
- **BIC Validation**: Strict 8/11 character BIC code validation
- **Currency Codes**: ISO 4217 currency validation
- **Date Formats**: SWIFT-compliant date parsing (YYMMDD)

### Performance Optimizations
- **Zero-Copy Parsing**: Minimal memory allocations during parsing
- **Compile-Time Generation**: Macro-generated code for optimal performance
- **Efficient Serialization**: Custom serialization for financial data structures
- **Memory Safety**: Rust's ownership system prevents financial data corruption

### Error Handling
- **Structured Errors**: Detailed error types for financial message validation
- **Field-Level Errors**: Precise error reporting with field tags
- **Compliance Reporting**: SWIFT standard violation reporting
- **Recovery Strategies**: Graceful handling of malformed financial data

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

Test with financial message examples:

```bash
cargo test --features financial-examples -- --nocapture
```

## 📚 Macro Reference

### SwiftField Attributes

| Attribute | Purpose | Example |
|-----------|---------|---------|
| `#[format("4!c")]` | Field format specification | 4 characters, alphabetic |
| `#[variant("A")]` | Enum variant tag | Field50A variant |
| `#[validate(bic)]` | Custom validation | BIC code validation |

### SwiftMessage Attributes

| Attribute | Purpose | Example |
|-----------|---------|---------|
| `#[field("20")]` | SWIFT field tag | Transaction reference |
| `#[optional]` | Optional field | Non-mandatory fields |
| `#[swift_serde(...)]` | Serialization control | Field tag mapping |

## 🔍 Financial Validation Rules

- **BIC Codes**: 8 or 11 characters, alphanumeric (SWIFT standard)
- **Account Numbers**: Maximum 34 characters (IBAN compliance)
- **Currency Codes**: 3-character ISO 4217 codes
- **Amount Formats**: Decimal precision with currency-specific rules
- **Date Formats**: YYMMDD format with leap year validation
- **Message Structure**: Complete MT message validation

## 🤝 Contributing

Contributions welcome! Please ensure:
- Financial compliance testing
- Macro documentation updates
- SWIFT standard adherence

## 📄 License

Apache License Version 2.0 - See [LICENSE](LICENSE) file for details.

## 🔗 Financial Standards

- [SWIFT Standards](https://www.swift.com/standards)
- [MT Message Types](https://www.swift.com/standards/mt-messages)
