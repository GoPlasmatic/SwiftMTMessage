# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

SwiftMTMessage is a Rust library for parsing SWIFT MT (Message Type) financial messages. It uses a macro-driven architecture with `#[derive(SwiftField)]` and `#[derive(SwiftMessage)]` for automatic code generation, similar to serde's approach.

## Common Development Commands

### Build and Test
```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name -- --exact

# Build release version
cargo build --release
```

### Lint and Format
```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt --check

# Run clippy linter
cargo clippy

# Run clippy with all features
cargo clippy --all-features
```

### Documentation
```bash
# Generate and open documentation
cargo doc --open

# Generate docs with private items
cargo doc --document-private-items
```

### Examples
```bash
# Run MT103 parsing example
cargo run --example parse_mt103

# Run MT202 parsing example
cargo run --example parse_mt202

# Run auto-detection example
cargo run --example parse_auto

# Run JSON to MT conversion example
cargo run --example json_to_mt_example
```

### Backward Compatibility Testing
```bash
# Run full compatibility test
./backward-compatibility-test/run_compatibility_test.sh

# Or manually:
cd backward-compatibility-test
cargo build --release --features published
./target/release/generate_old_json
cargo clean
cargo build --release --features local --no-default-features
./target/release/generate_new_json
./target/release/compare_compatibility --detailed --output compatibility_report.md
```

### Sample Generation
```bash
# Generate sample MT messages for testing
cargo run --example sample_generation

# Test sample generation utilities
cargo test sample::tests --lib
```

## Architecture

### Workspace Structure
- **swift-mt-message/**: Core library with field definitions and message parsing
- **swift-mt-message-macros/**: Procedural macros for code generation

### Core Components

1. **Field Layer** (swift-mt-message/src/fields/):
   - Common fields: account.rs, balance.rs, bic.rs, currency.rs, date.rs
   - Specific fields: field_20.rs through field_98.rs
   - Each field implements parsing, validation, and serialization

2. **Message Layer** (swift-mt-message/src/messages/):
   - MT message implementations: mt101.rs through mt950.rs
   - Each message composed of fields using macros

3. **Parser** (swift-mt-message/src/parser.rs):
   - SwiftParser: Main entry point
   - Header parsing: basic, application, user, trailer headers
   - Field extraction and validation

4. **Macro System** (swift-mt-message-macros/):
   - SwiftField derive: Generates field parsing/validation
   - SwiftMessage derive: Generates message structure
   - Component-based field definitions

### Key Design Patterns

1. **Macro-Generated Fields**:
   ```rust
   #[derive(SwiftField)]
   #[format("4!c")]
   pub struct Field23B {
       #[format("4!c")]
       pub bank_operation_code: String,
   }
   ```

2. **Complex Enum Fields**:
   ```rust
   #[derive(SwiftField)]
   pub enum Field50 {
       A(Field50A),  // Account + BIC
       F(Field50F),  // Party + Address
       K(Field50K),  // Name + Address
   }
   ```

3. **Message Composition**:
   ```rust
   #[derive(SwiftMessage)]
   #[swift_serde(rename_all = "FIELD_TAGS")]
   pub struct MT103 {
       #[field("20")]
       pub transaction_reference: Field20,
       // ... other fields
   }
   ```

### Validation Rules
- BIC codes: 8 or 11 characters
- Account numbers: Max 34 characters
- Currency codes: 3-character ISO 4217
- Dates: YYMMDD format
- Amounts: Decimal with currency-specific precision

### Testing Strategy
- Unit tests: Inline with code using `#[cfg(test)]`
- Integration tests: Examples directory
- Test data: test_data/ with real MT messages
- Backward compatibility: Automated JSON comparison
- Sample generation: Automated test data creation

### Sample Generation Features
- **Field-level generation**: All fields support `sample()` and `sample_with_config()`
- **Message-level generation**: Messages support `sample()`, `sample_minimal()`, `sample_full()`
- **Format-driven**: Generation based on SWIFT format specifications (3!a, 6!n, 15d, etc.)
- **Validation-aware**: Generates valid BIC codes, currency codes, dates, amounts
- **Configurable**: Custom constraints, scenarios (STP, Cover Payment), optional field control
- **Macro-generated**: Automatic implementation via derive macros

## Important Notes

- Financial compliance is critical - strict SWIFT standard adherence
- Performance matters - zero-copy parsing where possible
- Type safety is paramount - compile-time validation preferred
- JSON output should be clean without enum wrappers
- Always maintain backward compatibility
- Run compatibility tests before major changes