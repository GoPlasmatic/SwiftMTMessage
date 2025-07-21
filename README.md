<div align="center">
  <img src="https://avatars.githubusercontent.com/u/207296579?s=200&v=4" alt="Plasmatic Logo" width="120" height="120">
  
  # SwiftMTMessage
  
  **Enterprise-Grade SWIFT MT Message Processing Library**
  
  *Macro-driven, type-safe parsing with automatic test data generation*
  
  [![Release Crates](https://github.com/GoPlasmatic/SwiftMTMessage/actions/workflows/crate-publish.yml/badge.svg)](https://github.com/GoPlasmatic/SwiftMTMessage/actions/workflows/crate-publish.yml)
  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
  [![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
  [![Crates.io](https://img.shields.io/crates/v/swift-mt-message.svg)](https://crates.io/crates/swift-mt-message)

  <p>
    <a href="https://github.com/GoPlasmatic">🏢 Organization</a> •
    <a href="https://docs.rs/swift-mt-message">📖 Documentation</a> •
    <a href="https://github.com/GoPlasmatic/SwiftMTMessage/issues">🐛 Issues</a>  
  </p>
</div>

---

A modern Rust library for parsing SWIFT MT (Message Type) financial messages with **macro-based field definitions** and **serde-like automatic serialization**. Built for financial institutions requiring type-safe, high-performance SWIFT message processing with comprehensive test data generation.

## 🚀 Key Features

- **Macro-Driven Architecture**: `#[derive(SwiftField)]` and `#[derive(SwiftMessage)]` for automatic field and message generation
- **Serde-Like Design**: Familiar serialization patterns adapted for financial messaging standards
- **Type-safe Parsing**: Dedicated field structs with automatic validation
- **Sample Data Generation**: Automatic generation of valid SWIFT test data with JSON configuration support
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
// - sample() and sample_with_config() methods
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
    // - Sample data generation
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

## 🎲 Sample Data Generation

### Automatic Test Data Creation

Generate valid SWIFT test data automatically using the same macro-driven approach:

```rust
use swift_mt_message::{fields::Field20, messages::MT103, SwiftField, SwiftMessageBody};

// Generate individual field samples
let transaction_ref = Field20::sample();
println!("Generated reference: {}", transaction_ref.to_swift_string());
// Output: :20:ABC123DEF4567890

// Generate complete message samples
let mt103_sample = MT103::sample();
let json = serde_json::to_string_pretty(&mt103_sample)?;
println!("Sample MT103:\n{}", json);
```

### JSON Configuration-Based Generation

Customize sample generation with JSON configurations for precise test scenarios:

```rust
use swift_mt_message::sample::{FieldConfig, MessageConfig, ValueRange, LengthPreference};

// Configure field-specific generation
let field_config_json = r#"
{
    "length_preference": { "Exact": 16 },
    "pattern": "^STP[0-9]{13}$",
    "value_range": {
        "Amount": {
            "min": 10000.0,
            "max": 50000.0,
            "currency": "EUR"
        }
    }
}
"#;

let config: FieldConfig = serde_json::from_str(field_config_json)?;
let custom_sample = Field20::sample_with_config(&config);
```

### Multi-Scenario Test Generation

Generate test data for different financial scenarios:

```rust
let scenarios_json = r#"
[
    {
        "name": "High Value Transaction",
        "config": {
            "include_optional": true,
            "scenario": "Standard",
            "field_configs": {
                "32A": {
                    "value_range": {
                        "Amount": {
                            "min": 100000.0,
                            "max": 1000000.0,
                            "currency": "USD"
                        }
                    }
                }
            }
        }
    },
    {
        "name": "STP Compliant Payment",
        "config": {
            "include_optional": true,
            "scenario": "StpCompliant",
            "field_configs": {
                "20": {
                    "pattern": "^STP[0-9]{13}$"
                }
            }
        }
    }
]
"#;

let scenarios: Vec<TestScenario> = serde_json::from_str(scenarios_json)?;
for scenario in scenarios {
    let sample = MT103::sample_with_config(&scenario.config);
    println!("Scenario '{}': {}", scenario.name, sample.field_20.value);
}
```

### Format-Aware Generation

Sample generation respects SWIFT format specifications automatically:

```rust
// Field with format specification "6!n" (exactly 6 numeric characters)
#[derive(SwiftField)]
#[format("6!n")]
pub struct DateField {
    #[format("6!n")]
    pub date: String,
}

let sample = DateField::sample();
// Generates: "240315" (valid YYMMDD format)
```

### Predefined Scenarios

Built-in scenarios for common testing needs:

- **Standard**: Basic compliant messages
- **StpCompliant**: Straight Through Processing optimized
- **CoverPayment**: Cover payment message format
- **Minimal**: Only mandatory fields
- **Full**: All fields populated

```rust
use swift_mt_message::sample::MessageScenario;

let config = MessageConfig {
    include_optional: true,
    scenario: Some(MessageScenario::StpCompliant),
    field_configs: HashMap::new(),
};

let stp_sample = MT103::sample_with_config(&config);
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

// Macro-powered parsing with enhanced error handling
match SwiftParser::parse::<MT103>(raw_mt103) {
    Ok(parsed) => {
        // Serde-like JSON serialization
        let json = serde_json::to_string_pretty(&parsed)?;
        println!("Financial Message JSON: {}", json);
    }
    Err(e) => {
        // Enhanced error reporting
        eprintln!("Parse error: {}", e.brief_message());
        eprintln!("\nDetails:\n{}", e.debug_report());
        eprintln!("\n{}", e.format_with_context(raw_mt103));
    }
}
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
├── sample() - Test data generation
├── sample_with_config() - Configurable generation
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
├── Sample data generation
└── Financial JSON serialization
```

## 🎯 Financial-Grade Features

### SWIFT Compliance
- **Format Validation**: Automatic SWIFT format checking
- **Field Length Limits**: Enforced character limits per SWIFT standards
- **BIC Validation**: Strict 8/11 character BIC code validation
- **Currency Codes**: ISO 4217 currency validation
- **Date Formats**: SWIFT-compliant date parsing (YYMMDD)

### Enhanced Error Context
- **Component-Level Errors**: Identifies exact field component that failed
- **Position Tracking**: Line number and field position in original message
- **Format Hints**: Expected format shown in error messages
- **Debug Reports**: Tree-formatted error output with actionable hints
- **Context Display**: Shows surrounding message lines for debugging

### Performance Optimizations
- **Zero-Copy Parsing**: Minimal memory allocations during parsing
- **Compile-Time Generation**: Macro-generated code for optimal performance
- **Efficient Serialization**: Custom serialization for financial data structures
- **Memory Safety**: Rust's ownership system prevents financial data corruption

### Enhanced Error Handling
- **Contextual Errors**: Rich error information with field tags, components, and positions
- **Debug-Friendly**: Tree-formatted error reports with hints and suggestions
- **Position Tracking**: Line numbers and field positions preserved throughout parsing
- **Message Context**: Errors show surrounding lines from original message
- **Recovery Strategies**: Detailed error information enables targeted recovery

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

Test with financial message examples:

```bash
cargo test --features financial-examples -- --nocapture
```

Run sample generation examples:

```bash
# Basic sample generation
cargo run --example sample_generation

# JSON configuration-based generation
cargo run --example json_config_sample_generation

# Parse with enhanced error display
cargo run --example parse_auto -- path/to/message.txt
```

## 📚 Macro Reference

### SwiftField Attributes

| Attribute | Purpose | Example |
|-----------|---------|---------|
| `#[format("4!c")]` | Field format specification | 4 characters, alphabetic |
| `#[variant("A")]` | Enum variant tag | Field50A variant |
| `#[validate(bic)]` | Custom validation | BIC code validation |
| `#[sample(generator)]` | Custom sample generator | Specialized test data |

### SwiftMessage Attributes

| Attribute | Purpose | Example |
|-----------|---------|---------|
| `#[field("20")]` | SWIFT field tag | Transaction reference |
| `#[optional]` | Optional field | Non-mandatory fields |
| `#[swift_serde(...)]` | Serialization control | Field tag mapping |
| `#[sample_scenario(...)]` | Default sample scenario | StpCompliant generation |

## 🔍 Financial Validation Rules

- **BIC Codes**: 8 or 11 characters, alphanumeric (SWIFT standard)
- **Account Numbers**: Maximum 34 characters (IBAN compliance)
- **Currency Codes**: 3-character ISO 4217 codes
- **Amount Formats**: Decimal precision with currency-specific rules
- **Date Formats**: YYMMDD format with leap year validation
- **Message Structure**: Complete MT message validation

## 🎯 Sample Generation Features

- **Format-Driven**: Generates data based on SWIFT format specifications
- **Validation-Aware**: All generated data passes SWIFT compliance checks
- **JSON Configurable**: External configuration for test scenarios
- **Scenario-Based**: Predefined scenarios for common testing needs
- **Type-Safe**: Generated samples match field type constraints
- **Reproducible**: Configurable random seed for deterministic testing

## 🤝 Contributing

We welcome contributions! Please ensure:
- Financial compliance testing
- Macro documentation updates
- SWIFT standard adherence
- Test coverage for new features

## 🏢 About Plasmatic

SwiftMTMessage is developed by [Plasmatic](https://github.com/GoPlasmatic), a technology organization focused on building open-source financial infrastructure tools. We believe in:

- **🔓 Open Source**: Transparent, community-driven development
- **🛡️ Security First**: Financial-grade security and compliance
- **⚡ Performance**: High-performance solutions for enterprise needs
- **🌍 Global Standards**: Supporting international financial protocols

## 📄 License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE))

## 🔗 Related Projects

- [Reframe](https://github.com/GoPlasmatic/Reframe) - SWIFT MT ↔ ISO 20022 Transformation Engine
- [MXMessage](https://github.com/GoPlasmatic/MXMessage) - MX Message - ISO20022 Parser Library

---

<div align="center">
  <p>Built with ❤️ by the <a href="https://github.com/GoPlasmatic">Plasmatic</a> team</p>
</div>
