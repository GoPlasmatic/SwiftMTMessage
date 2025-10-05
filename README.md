<div align="center">
  <img src="https://avatars.githubusercontent.com/u/207296579?s=200&v=4" alt="Plasmatic Logo" width="120" height="120">

# SwiftMTMessage

**A high-performance Rust library for parsing and building SWIFT MT messages.**

*Compliant with SWIFT CBPR+ SR2025 specifications, featuring enhanced type safety, comprehensive error handling, and robust validation.*

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

SwiftMTMessage is a production-ready Rust library for handling SWIFT MT financial messages, fully compliant with **SWIFT CBPR+ SR2025** standards. The v3 architecture features enhanced error collection, comprehensive validation with 1,335 SWIFT error codes, and support for 30+ MT message types including complex multi-sequence messages.

## 🚀 Key Features

  - **SWIFT CBPR+ SR2025 Compliant:** Full compliance with the latest SWIFT Cross-Border Payments and Reporting Plus standards
  - **Comprehensive Coverage:** Support for 30+ MT message types including MT1xx, MT2xx, MT9xx series with multi-sequence parsing
  - **Error Collection:** Permissive parsing mode that collects all validation errors instead of failing fast
  - **Type-Safe Architecture:** Robust trait-based system with comprehensive SWIFT format enforcement and validation
  - **1,335 SWIFT Error Codes:** Complete implementation of T-Series, C-Series, D-Series, E-Series, and G-Series validation codes
  - **Test Data Generation:** Format-driven sample generation with 195+ real-world test scenarios
  - **Performance Optimized:** Zero-copy parsing, regex caching, and optimized memory allocation
  - **Production Ready:** 100% round-trip test success rate (195/195) with comprehensive SWIFT standard validation

## 🏗️ How It Works: v3 Architecture

### Type-Safe Field System

The library implements a robust trait-based system for SWIFT field handling with comprehensive validation:

```rust
use swift_mt_message::SwiftField;

// Field structures implement the SwiftField trait
pub struct Field23B {
    pub bank_operation_code: String,
}

impl SwiftField for Field23B {
    fn parse(value: &str) -> Result<Self> {
        // Format-aware parsing with SWIFT CBPR+ compliance
        // Validates 4!c format (4 alphanumeric characters)
        validate_format(value, "4!c")?;
        Ok(Field23B { bank_operation_code: value.to_string() })
    }

    fn to_swift_string(&self) -> String {
        // Smart serialization with proper field formatting
        self.bank_operation_code.clone()
    }
}

// The trait system provides:
// - Format-aware parsing with SWIFT CBPR+ compliance
// - Comprehensive validation with detailed error contexts
// - Sample generation for testing
// - Serde traits with clean JSON output
```

### Message Structure with Sequence Support

The v3 system supports complex multi-sequence messages with deterministic field ordering:

```rust
use swift_mt_message::SwiftMessage;

pub struct MT103 {
    pub transaction_reference: Field20,
    pub bank_operation_code: Field23B,
    pub value_date_currency_amount: Field32A,
    pub envelope_contents: Option<Field77T>, // Optional fields with CBPR+ validation
}

impl SwiftMessageBody for MT103 {
    fn message_type() -> &'static str { "103" }

    fn parse_from_block4(block4: &str) -> Result<Self> {
        // Sequence-aware parsing for complex messages
        // Handles field extraction, validation, and error collection
    }

    fn validate_network_rules(&self, stop_on_first_error: bool) -> Vec<SwiftValidationError> {
        // CBPR+ compliance validation with 1,335 error codes
    }
}

// The system provides:
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
swift-mt-message = "3.1"
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

SwiftMTMessage v3 includes a comprehensive testing framework with 195+ real-world scenarios across 30 message types, ensuring SWIFT CBPR+ SR2025 compliance.

### Key Testing Features

- **Scenario-Based Testing**: 195+ real-world scenarios covering all MT message types
- **CBPR+ SR2025 Compliance**: 100+ dedicated CBPR+ scenarios for cross-border payment compliance
- **Round-Trip Validation**: Bidirectional JSON ↔ MT conversion with 100% success rate (195/195 tests passing)
- **Multi-Sequence Support**: Complex message testing (MT104, MT107) with repeated sequences
- **Error Collection Testing**: Validation of permissive parsing mode with comprehensive error reporting
- **Performance Benchmarks**: Optimized parsing achieving sub-millisecond performance
- **100% Validation Success**: All 1,950 validation tests passing

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

SwiftMTMessage v3 supports comprehensive parsing and generation for **30+ MT message types**:

### Payment Messages (MT1xx) - 7 Types
- **MT101**: Request for Transfer
- **MT103**: Single Customer Credit Transfer (CBPR+ Enhanced)
- **MT104**: Direct Debit and Request for Debit Transfer
- **MT107**: General Direct Debit Message
- **MT110**: Advice of Cheque(s)
- **MT111**: Stop Payment of a Cheque
- **MT112**: Status of a Request for Stop Payment

### System Messages (MT19x) - 4 Types
- **MT190**: Advice of Charges, Interest and Other Adjustments
- **MT191**: Request for Payment of Charges, Interest and Other Expenses
- **MT192**: Request for Cancellation
- **MT196**: Answers (Cancellation/Inquiry)

### Financial Institution Transfers (MT2xx) - 11 Types
- **MT199**: Free Format Message
- **MT200**: Financial Institution Transfer for its Own Account
- **MT202**: General Financial Institution Transfer
- **MT204**: Financial Markets Direct Debit Message
- **MT205**: Financial Institution Transfer Execution
- **MT210**: Notice to Receive
- **MT290**: Advice of Charges, Interest and Other Adjustments
- **MT291**: Request for Payment of Charges, Interest and Other Expenses
- **MT292**: Request for Cancellation
- **MT296**: Answers
- **MT299**: Free Format Message

### Cash Management & Statements (MT9xx) - 8 Types
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
- **Parser-Based Architecture**: Complete migration to robust parser-based system
- **Modular Design**: Separated into dedicated modules (parser, fields, messages, validation) for maintainability
- **Sequence-Aware Parsing**: Native support for complex multi-sequence messages (MT104, MT107, etc.)
- **Enhanced Trait System**: Comprehensive `SwiftField` and `SwiftMessageBody` trait implementations

### Enhanced Features
- **Error Collection Mode**: Collect all validation errors instead of failing fast (v3.0)
- **CBPR+ SR2025 Compliance**: Full support for latest SWIFT standards with 100+ test scenarios
- **1,335 SWIFT Error Codes**: Complete T-Series, C-Series, D-Series, E-Series, and G-Series validation
- **Performance Optimizations**: Regex caching, reduced allocations, zero-copy parsing
- **Comprehensive Documentation**: Enhanced error handling docs and AI-assisted development guide

### Testing & Quality
- **195+ Test Scenarios**: Real-world scenarios for all 30 message types
- **100% Round-Trip Success**: All 195 messages pass bidirectional JSON ↔ MT conversion
- **100% Validation Success**: All 1,950 validation tests passing
- **SWIFT Standards Compliance**: Complete implementation of SWIFT CBPR+ SR2025 validation codes

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