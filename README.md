<div align="center">
  <img src="https://avatars.githubusercontent.com/u/207296579?s=200&v=4" alt="Plasmatic Logo" width="120" height="120">

# SwiftMTMessage

**A Rust library for parsing and building SWIFT MT messages.**

*Uses macros for type-safe parsing and automatic test data generation.*

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

-----

SwiftMTMessage is a modern Rust library for handling SWIFT MT financial messages. It uses a macro-driven approach with a `serde`-like feel for defining fields, parsing messages, and serializing data. It's designed for performance and type safety, with powerful features for generating test data.

## 🚀 Key Features

  - **Macro-Driven:** Use `#[derive(SwiftField)]` and `#[derive(SwiftMessage)]` to automatically generate parsing and serialization logic.
  - **`serde`-like API:** A familiar design for developers who have worked with `serde`.
  - **Type-Safe Parsing:** SWIFT fields are parsed into dedicated, validated structs.
  - **Test Data Generation:** Automatically create valid SWIFT test data, with or without JSON configuration.
  - **Comprehensive:** Full support for MT103 fields, with more message types on the way.
  - **Zero-Copy:** Efficient, low-allocation parsing.
  - **Strict Validation:** Enforces SWIFT rules and provides detailed error reports.

## 🏗️ How It Works: Macro-Based Architecture

### `#[derive(SwiftField)]`

Define a SWIFT field once, and let the macro generate the boilerplate.

```rust
use swift_mt_message::SwiftField;

#[derive(SwiftField)]
#[format("4!c")] // Defines the SWIFT format for validation
pub struct Field23B {
    #[format("4!c")]
    pub bank_operation_code: String,
}

// The macro automatically generates:
// - A `parse()` method with format validation
// - A `to_swift_string()` method
// - `validate()` for SWIFT rule compliance
// - `serde` serialization/deserialization traits
```

### `#[derive(SwiftMessage)]`

Compose fields into a complete MT message.

```rust
use swift_mt_message::{SwiftMessage, swift_serde};

#[derive(SwiftMessage)]
#[swift_serde(rename_all = "FIELD_TAGS")] // Maps struct fields to SWIFT tags in JSON
pub struct MT103 {
    #[field("20")]
    pub transaction_reference: Field20,
    
    #[field("23B")]
    pub bank_operation_code: Field23B,
    
    #[field("32A")]
    pub value_date_currency_amount: Field32A,
}

// The macro handles:
// - Field validation and ordering
// - SWIFT format compliance
// - JSON serialization
// - Error propagation
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

### Running Tests

Run the full test suite:

```bash
cargo test
```

Run tests that include financial message examples:

```bash
cargo test --features financial-examples -- --nocapture
```

### Scenario-Based Testing

The library includes comprehensive scenario-based tests for each message type. These tests validate parsing, validation, and round-trip conversion.

Run all scenario tests:

```bash
cargo test round_trip_scenarios -- --nocapture
```

Debug a specific failing scenario:

```bash
# Set environment variables for detailed debugging
TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=cbpr_social_security TEST_DEBUG=1 cargo test round_trip_scenarios -- --nocapture
```

### Test Data Generation

The library can automatically generate valid SWIFT test data using the `datafake-rs` library:

```bash
# Generate sample messages
cargo run --example sample_generation

# Generate from JSON configuration
cargo run --example json_config_sample_generation
```

### Tracking Test Progress

We maintain a scenario tracker (`scenario_tracker.md`) that monitors the status of all test scenarios. When fixing scenarios:

1. Run tests to identify failures
2. Use debug mode to understand the issue
3. Refer to message structures in `swift-mt-message/src/messages/`
4. Fix the scenario JSON files
5. Update the tracker with your progress

Example workflow for fixing scenarios:

```bash
# 1. Run all scenarios to see failures
cargo test round_trip_scenarios -- --nocapture

# 2. Debug a specific failure
TEST_MESSAGE_TYPE=MT192 TEST_SCENARIO=request_cancellation TEST_DEBUG=1 cargo test round_trip_scenarios -- --nocapture

# 3. Fix the scenario file (e.g., test_scenarios/mt192/request_cancellation.json)
# Common fixes include:
# - Replacing unsupported datafake methods with static values
# - Correcting field names to match struct definitions
# - Simplifying complex JSON expressions

# 4. Verify the fix
cargo test round_trip_scenarios::mt192_request_cancellation -- --nocapture

# 5. Update scenario_tracker.md with status and notes
```

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