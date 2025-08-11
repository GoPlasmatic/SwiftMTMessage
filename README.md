<div align="center">
  <img src="https://avatars.githubusercontent.com/u/207296579?s=200&v=4" alt="Plasmatic Logo" width="120" height="120">

# SwiftMTMessage

**A high-performance Rust library for parsing and building SWIFT MT messages.**

*Compliant with SWIFT CBPR+ SR2025 specifications, featuring v3 macro system for enhanced type safety and comprehensive error handling.*

  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
  [![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
  [![Crates.io](https://img.shields.io/crates/v/swift-mt-message.svg)](https://crates.io/crates/swift-mt-message)
  [![Swift CBPR+](https://img.shields.io/badge/Swift-CBPR%2B%20SR2025-green.svg)](https://www.swift.com)

  <p>
    <a href="https://github.com/GoPlasmatic">🏢 Organization</a> •
    <a href="https://docs.rs/swift-mt-message">📖 Documentation</a> •
    <a href="https://github.com/GoPlasmatic/SwiftMTMessage/issues">🐛 Issues</a>  
  </p>
</div>

-----

SwiftMTMessage is a production-ready Rust library for handling SWIFT MT financial messages, fully compliant with **SWIFT CBPR+ SR2025** standards. The v3 architecture features an advanced macro system with compile-time validation, comprehensive error collection, and support for all major MT message types including complex multi-sequence messages.

## 🚀 Key Features

  - **SWIFT CBPR+ SR2025 Compliant:** Full compliance with the latest SWIFT Cross-Border Payments and Reporting Plus standards
  - **v3 Macro System:** Enhanced `#[derive(SwiftField)]` and `#[derive(SwiftMessage)]` with compile-time validation and smart formatting
  - **Comprehensive Coverage:** Support for 23+ MT message types including MT1xx, MT2xx, MT9xx series with multi-sequence parsing
  - **Error Collection:** Permissive parsing mode that collects all validation errors instead of failing fast
  - **Type-Safe Architecture:** Compile-time validation with macro-generated type checks and SWIFT format enforcement
  - **Test Data Generation:** Format-driven sample generation with 400+ real-world test scenarios
  - **Performance Optimized:** Zero-copy parsing, regex caching, and optimized memory allocation
  - **Production Ready:** 100% round-trip test success rate with comprehensive SWIFT standard validation

## 🏗️ How It Works: v3 Macro Architecture

### `#[derive(SwiftField)]` - Enhanced Field Generation

The v3 macro system provides compile-time validation and automatic implementation generation:

```rust
use swift_mt_message::SwiftField;

#[derive(SwiftField)]
#[format("4!c")] // SWIFT format specification with compile-time validation
pub struct Field23B {
    #[format("4!c")]
    pub bank_operation_code: String,
}

// The v3 macro automatically generates:
// - Format-aware parsing with SWIFT CBPR+ compliance
// - Smart serialization with proper field formatting
// - Comprehensive validation with detailed error contexts
// - Sample generation for testing
// - Serde traits with clean JSON output
```

### `#[derive(SwiftMessage)]` - Sequence-Aware Message Composition

The v3 system supports complex multi-sequence messages with automatic field ordering:

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
    
    // Optional fields with CBPR+ validation
    #[field("77T", optional)]
    pub envelope_contents: Option<Field77T>,
}

// The v3 macro provides:
// - Sequence-aware parsing for complex messages (MT104, MT107)
// - CBPR+ compliance validation
// - Error collection in permissive mode
// - Deterministic field ordering
// - Clean JSON serialization without enum wrappers
```

## 🎯 `serde`-like Serialization

The library's `serde`-like design makes it easy to serialize parsed SWIFT messages to JSON.

```rust
use serde_json;
use swift_mt_message::{SwiftParser, SwiftMessage, messages::MT103};

// Parse a raw SWIFT message
let mt103: SwiftMessage<MT103> = SwiftParser::parse(raw_swift_message)?;

// Automatically serialize to a clean JSON structure
let json = serde_json::to_string_pretty(&mt103)?;
```

**Example JSON Output:**

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

Complex fields, like enums with different option structures (e.g., `Field50` with options A, F, or K), are flattened for a cleaner JSON output.


## 🔧 Installation

Add `swift-mt-message` to your `Cargo.toml`:

```toml
[dependencies]
swift-mt-message = "3.0.0"
```

## 📖 Usage

### Basic Parsing

```rust
use swift_mt_message::{SwiftParser, SwiftMessage, messages::MT103};

let raw_mt103 = r#"{1:F01BANKDEFF0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:/123456789
ACME CORPORATION
123 BUSINESS AVENUE
NEW YORK NY 10001
:52A:BANKDEFF
:57A:DEUTDEFF
:59:/DE89370400440532013000
DEUTDEFF
:70:PAYMENT FOR SERVICES
:71A:OUR
-}"#;

match SwiftParser::parse::<MT103>(raw_mt103) {
    Ok(parsed) => {
        let json = serde_json::to_string_pretty(&parsed).unwrap();
        println!("Parsed Message:\n{}", json);
    }
    Err(e) => {
        // Rich error reporting
        eprintln!("Parse error: {}", e.brief_message());
        eprintln!("\nDetails:\n{}", e.debug_report());
        eprintln!("\n{}", e.format_with_context(raw_mt103));
    }
}
```

### Error Collection Mode (v3.0)

Instead of failing on the first error, you can configure the parser to collect all errors. This is useful for processing messages that may have non-critical issues.

```rust
use swift_mt_message::{SwiftParser, ParseResult, ParserConfig, messages::MT103};

// Configure the parser to collect all errors
let parser = SwiftParser::with_config(ParserConfig {
    fail_fast: false,
    collect_all_errors: true,
    ..Default::default()
});

match parser.parse_with_errors::<MT103>(raw_message_with_errors) {
    Ok(ParseResult::Success(msg)) => {
        println!("✓ Message parsed successfully");
    }
    Ok(ParseResult::PartialSuccess(msg, errors)) => {
        println!("⚠ Parsed with {} non-critical errors", errors.len());
        // You can still work with the valid parts of the message
        // for error in errors { ... }
    }
    Ok(ParseResult::Failure(errors)) => {
        println!("✗ Failed with {} errors:", errors.len());
        // for error in errors { ... }
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
}
```

## 🧪 Testing Strategy

SwiftMTMessage v3 includes a comprehensive testing framework with 400+ real-world scenarios across 23 message types, ensuring SWIFT CBPR+ SR2025 compliance.

### Key Testing Features

- **Scenario-Based Testing**: 400+ real-world scenarios covering all MT message types
- **CBPR+ SR2025 Compliance**: 100+ dedicated CBPR+ scenarios for cross-border payment compliance
- **Round-Trip Validation**: Bidirectional JSON ↔ MT conversion with 100% success rate
- **Multi-Sequence Support**: Complex message testing (MT104, MT107) with repeated sequences
- **Error Collection Testing**: Validation of permissive parsing mode with comprehensive error reporting
- **Performance Benchmarks**: Optimized parsing achieving sub-millisecond performance

### Quick Start

```bash
# Run all test scenarios
cargo test round_trip_scenarios -- --nocapture

# Test specific message type
TEST_MESSAGE_TYPE=MT103 cargo test round_trip_scenarios -- --nocapture

# Debug a specific scenario
TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=cbpr_business_payment TEST_DEBUG=1 cargo test round_trip_scenarios -- --nocapture
```

For detailed test scenarios, running instructions, and coverage information, see the [Test Scenarios Documentation](test_scenarios/README.md).

## 📋 Supported Message Types

SwiftMTMessage v3 supports comprehensive parsing and generation for the following MT message types:

### Payment Messages (MT1xx)
- **MT101**: Request for Transfer
- **MT103**: Single Customer Credit Transfer (CBPR+ Enhanced)
- **MT104**: Direct Debit and Request for Debit Transfer
- **MT107**: General Direct Debit Message
- **MT110**: Advice of Cheque(s)
- **MT111**: Stop Payment of a Cheque
- **MT112**: Status of a Request for Stop Payment

### Financial Institution Transfers (MT2xx)
- **MT192**: Request for Cancellation
- **MT196**: Answers (Cancellation/Inquiry)
- **MT199**: Free Format Message
- **MT202**: General Financial Institution Transfer
- **MT205**: Financial Institution Transfer Execution
- **MT210**: Notice to Receive
- **MT292**: Request for Cancellation
- **MT296**: Answers
- **MT299**: Free Format Message

### Cash Management & Statements (MT9xx)
- **MT900**: Confirmation of Debit
- **MT910**: Confirmation of Credit
- **MT920**: Request Message
- **MT935**: Rate Change Advice
- **MT940**: Customer Statement Message
- **MT941**: Balance Report
- **MT942**: Interim Transaction Report
- **MT950**: Statement Message

All message types are fully compliant with SWIFT CBPR+ SR2025 specifications and include comprehensive validation rules.

## 🚀 What's New in v3

### Architecture Improvements
- **Complete Macro System Overhaul**: Rewritten from scratch with enhanced compile-time validation
- **Modular Design**: Separated into dedicated modules (ast, codegen, format, error) for maintainability
- **Sequence-Aware Parsing**: Native support for complex multi-sequence messages

### Enhanced Features
- **Error Collection Mode**: Collect all validation errors instead of failing fast
- **CBPR+ SR2025 Compliance**: Full support for latest SWIFT standards
- **Performance Optimizations**: Regex caching, reduced allocations, zero-copy parsing
- **Comprehensive Documentation**: Added CLAUDE.md for AI-assisted development

### Testing & Quality
- **400+ Test Scenarios**: Real-world scenarios for all message types
- **100% Round-Trip Success**: All messages pass bidirectional conversion
- **SWIFT Error Codes**: Complete implementation of standard SWIFT validation codes

## 🤝 Contributing

Contributions are welcome\! If you'd like to help, please feel free to fork the repository, make your changes, and submit a pull request. We ask that you ensure test coverage for new features and adhere to SWIFT standards.

## 🏢 About Plasmatic

SwiftMTMessage is developed by [Plasmatic](https://github.com/GoPlasmatic), an organization focused on building open-source tools for financial infrastructure. We believe in transparency, security, and performance.

Check out our other projects:

  - [Reframe](https://github.com/GoPlasmatic/Reframe): A SWIFT MT to ISO 20022 (and back) transformation engine.
  - [MXMessage](https://github.com/GoPlasmatic/MXMessage): An ISO 20022 (MX) message parsing library.

## 📄 License

This library is licensed under the Apache License, Version 2.0. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.

-----

<div align="center">
<p>Built with ❤️ by the <a href="[https://github.com/GoPlasmatic](https://github.com/GoPlasmatic)">Plasmatic</a> team</p>
</div>