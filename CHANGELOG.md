# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.3.5] - 2025-01-11

### 🐛 Bug Fixes

#### Sample Generation Configuration Support
- **Fixed Field32A sample generation ignoring configuration**: Resolved critical issue where `sample_with_config()` was not respecting `ValueRange::Amount` constraints for amount and currency fields
- **Amount Range Compliance**: Field32A now properly generates amounts within configured min/max ranges instead of using default random values
- **Currency Configuration**: Field32A now uses the specified currency from `ValueRange::Amount.currency` configuration instead of generating random currencies
- **Enhanced Macro Implementation**: Added `generate_component_sample_with_config()` function to properly handle configuration parameters in derived `SwiftField` implementations
- **Message-Level Configuration**: Improved message-level `sample_with_config()` to properly pass field-specific configurations to individual fields
- **Backward Compatibility**: All existing `sample()` methods continue to work unchanged

#### Technical Implementation
- **Added UUID Support**: Added `uuid` dependency for realistic UETR generation in sample data
- **Enhanced Field Generation**: f64 fields with decimal formats (15d, 12d) now respect `ValueRange::Amount` constraints
- **Currency Field Logic**: String fields with `currency_code` validation now check configuration before falling back to random generation
- **Configuration Propagation**: Proper configuration passing from message-level to field-level sample generation

### 🔧 Technical Improvements

#### Dependencies
- **Added uuid = "1.17"**: For realistic UETR (Unique End-to-End Transaction Reference) generation
- **Updated Cargo.lock**: Added required dependencies for UUID functionality

#### Testing
- **Comprehensive Test Coverage**: All sample generation functions validate configuration compliance
- **Range Validation**: Automated testing ensures generated amounts stay within configured bounds
- **Currency Validation**: Automated testing ensures configured currencies are used consistently

## [2.3.0] - 2024-12-20

### 🎲 New Features

#### Sample Data Generation System
- **Macro-Generated Sample Methods**: Added `sample()` and `sample_with_config()` methods to all `SwiftField` implementations
- **Message-Level Generation**: Added `sample()`, `sample_minimal()`, `sample_full()`, and `sample_with_config()` methods to all `SwiftMessage` implementations
- **Format-Aware Generation**: Automatic generation based on SWIFT format specifications (3!a, 6!n, 15d, etc.)
- **Validation-Aware**: All generated data passes SWIFT compliance checks
- **Type-Safe**: Generated samples match field type constraints and validation rules

#### JSON Configuration Support
- **FieldConfig**: Configurable length preferences, value ranges, fixed values, and regex patterns
- **MessageConfig**: Message-level configuration with scenario support and field-specific overrides
- **Predefined Scenarios**: Built-in scenarios for common testing needs:
  - `Standard`: Basic compliant messages
  - `StpCompliant`: Straight Through Processing optimized
  - `CoverPayment`: Cover payment message format
  - `Minimal`: Only mandatory fields
  - `Full`: All fields populated
- **Serde Integration**: Full JSON serialization/deserialization support for all configuration types

#### Specialized Generators
- **BIC Codes**: Valid 8/11 character BIC code generation
- **Currency Codes**: ISO 4217 compliant currency code generation
- **Date Generation**: SWIFT-compliant date formats (YYMMDD, YYYYMMDD)
- **Amount Generation**: Decimal amounts with currency-specific precision
- **Name/Address Generation**: Realistic institution and customer data
- **Transaction Codes**: Valid SWIFT transaction and instruction codes

### 📚 Documentation & Examples

#### Comprehensive Examples
- **examples/sample_generation.rs**: Basic sample generation demonstration
- **examples/json_config_sample_generation.rs**: Advanced JSON configuration examples
- **examples/simple_sample_test.rs**: Simple testing utilities
- **SAMPLE_GENERATOR.md**: Technical specification and design documentation

#### Enhanced README
- **Sample Generation Section**: Comprehensive documentation with usage examples
- **Plasmatic Branding**: Updated to follow Plasmatic organization template
- **Professional Header**: Added logo, badges, and navigation links
- **Organization Links**: Community engagement through discussions and contribution links
- **Related Projects**: Cross-references to other Plasmatic projects like Reframe

### 🔧 Technical Implementation

#### Macro Enhancements
- **SwiftField Derive**: Enhanced to generate sample implementations automatically
- **SwiftMessage Derive**: Enhanced to generate message-level sample methods
- **Custom Implementations**: Special handling for complex enum fields (Field50, Field59)
- **Format Parser**: SWIFT format specification parser for automatic constraint application

#### Architecture Improvements
- **sample.rs Module**: Self-contained sample generation utilities and configuration types
- **Clean Integration**: Follows the same architecture pattern as validation functions
- **Minimal Boilerplate**: Macro-generated implementations reduce manual code
- **Configurable Randomness**: Support for reproducible test data generation

#### Dependencies
- **Added rand = "0.8"**: Random number generation for sample data
- **Workspace Integration**: Added to workspace Cargo.toml for consistent versioning

### 🧪 Testing & Quality

#### Comprehensive Test Coverage
- **Unit Tests**: All sample generation functions have dedicated tests
- **Integration Tests**: Full message generation and validation testing
- **JSON Roundtrip Tests**: Configuration serialization/deserialization validation
- **Format Compliance Tests**: Generated data validates against SWIFT standards

#### Production Ready
- **Financial Grade**: Suitable for production use in financial institutions
- **Performance Optimized**: Efficient generation with minimal memory allocations
- **Error Handling**: Proper error propagation and validation

## [2.1.0]

### 🐛 Bug Fixes

#### Field Parsing Regression Fix
- **Fixed field tag prefix parsing**: Resolved critical regression where `GenericNameAddressField`, `GenericPartyField`, and `GenericAccountField` types were incorrectly including field tag prefixes in parsed content
- **Affected Fields**: 52D, 53D, 56D, and other name/address fields were showing content like `:52D:NOTPROVIDED` instead of just `NOTPROVIDED`
- **Root Cause**: Message parsing macro was using generic `SwiftField::parse()` instead of specialized `parse_with_tag()` method for certain field types
- **Solution**: Enhanced `derive_swift_message` macro to automatically detect field types that require tag-aware parsing and call `parse_with_tag()` with the correct field tag
- **Backward Compatibility**: This fix restores correct parsing behavior and maintains backward compatibility with previous versions

#### Backward Compatibility Testing System
- **Added comprehensive backward compatibility test suite** in `backward-compatibility-test/` directory
- **Automated testing**: Compares JSON outputs between published and local versions to detect breaking changes
- **Test Coverage**: All MT103, MT202, and MT205 test data files are validated for compatibility
- **CI Integration Ready**: Shell script for automated testing in CI/CD pipelines
- **Detailed Reporting**: Provides specific field-level difference analysis with compatibility assessment

### 🔧 Technical Improvements

#### Macro Enhancements
- **Smart Field Type Detection**: Enhanced `derive_swift_message` macro to automatically detect field types requiring specialized parsing
- **Improved Code Generation**: Better handling of `Option<T>` and `Vec<T>` field types with tag-aware parsing
- **Code Quality**: Improved formatting and readability of generated parsing code

## [2.0.0] - 2024-12-19

### 🚀 Major Enhancements

#### Complex Field Structures
- **Field 50 (Ordering Customer)**: Implemented full enum-based structure with 3 variants
  - `Field50A`: Account + BIC format (`[/account]\nBIC`)
  - `Field50F`: Party identifier + Name/Address lines
  - `Field50K`: Name/Address lines only (up to 4 lines, 35 chars each)
- **Field 59 (Beneficiary Customer)**: Implemented full enum-based structure with 2 variants
  - `Field59A`: Account + BIC format (`[/account]\nBIC`)
  - `Field59Basic`: Basic beneficiary lines (up to 4 lines, 35 chars each)

#### Institution Fields Enhancement
- **Added `account_line_indicator`** to all institution fields (52A-57A)
- **Enhanced Field 52A**: Complete structure restoration with account_line_indicator, account_number, and BIC
- **Fields 53A-57A**: All now support optional account_line_indicator field
- **Comprehensive validation**: BIC length (8 or 11 characters), account number limits

#### Field Name Alignment
- **Field 20**: `value` → `transaction_reference` (matches old version)
- **Field 23B**: `code` → `bank_operation_code` (matches old version)
- **Field 70**: `remittance_info` → `information` (matches old version)
- **Field 71A**: `code` → `details_of_charges` (matches old version)

#### Type System Improvements
- **Field 32A**: Simplified to use primitive types
  - `date`: `NaiveDate` (YYMMDD format)
  - `currency_code`: `String` (3-character ISO codes)
  - `amount`: `f64` (decimal representation)

### 🎨 JSON Serialization Revolution

#### Flattened JSON Structure
- **Custom Serialization**: Implemented custom `Serialize`/`Deserialize` for Field50 and Field59
- **Eliminated Enum Wrappers**: Clean JSON without nested enum variant layers
- **Before**: `{"50": {"K": {"name_and_address": [...]}}}`
- **After**: `{"50": {"name_and_address": [...]}}`

#### Smart Deserialization
- **Field Detection**: Automatic variant detection based on field presence
  - Field50: `bic` → 50A, `party_identifier` → 50F, `name_and_address` only → 50K
  - Field59: `bic` → 59A, `beneficiary_customer` → 59Basic
- **Bidirectional**: Perfect round-trip serialization/deserialization
- **Type Safety**: Full validation during deserialization with proper error messages

### 🔧 Validation & Compliance

#### Enhanced Validation Rules
- **BIC Validation**: Strict 8 or 11 character validation for all BIC fields
- **Account Number Limits**: Maximum 34 characters for account numbers
- **Line Length Validation**: Maximum 35 characters per line for name/address fields
- **Line Count Limits**: Maximum 4 lines for multi-line fields
- **Format Compliance**: Full SWIFT MT format compliance

#### Comprehensive Error Handling
- **Detailed Error Messages**: Specific validation error messages with field tags
- **ParseError Types**: Structured error types for different validation failures
- **Graceful Degradation**: Proper error handling for malformed input

### 📚 Documentation & Testing

#### Comprehensive Documentation
- **Updated README**: Complete field reference, usage examples, architecture overview
- **Enhanced lib.rs**: Updated documentation with new field structures and JSON examples
- **Field Reference Tables**: Detailed field format specifications
- **Usage Examples**: Real-world examples for all field types

#### Extensive Test Coverage
- **223 Unit Tests**: Comprehensive test coverage for all fields and functionality
- **JSON Serialization Tests**: Specific tests for flattened JSON structure
- **Validation Tests**: Tests for all validation rules and error conditions
- **Integration Tests**: Full MT103 message parsing and serialization tests
- **Doc Tests**: Working documentation examples

### 🏗️ Architecture Improvements

#### Field Hierarchy
```
SwiftField Trait
├── Simple Fields (Field20, Field23B, etc.)
├── Complex Enum Fields
│   ├── Field50 (A/F/K variants)
│   └── Field59 (A/Basic variants)
└── Institution Fields (Field52A-57A)
```

#### Parsing Strategy
- **Tag-based Parsing**: `parse_with_tag()` methods for explicit variant selection
- **Smart Detection**: Automatic variant detection from input format
- **Flexible Input**: Handles various input formats with and without field tag prefixes

### 🔄 Migration from 1.x

#### Breaking Changes
- Field50 and Field59 are now enums instead of simple structs
- JSON serialization format changed (flattened structure)
- Some field names changed to match SWIFT specifications
- BIC validation is now stricter (8 or 11 characters only)

#### Migration Guide
```rust
// Old (1.x)
let field50 = Field50::new(vec!["JOHN DOE".to_string()]);

// New (2.x)
let field50 = Field50::K(Field50K::new(vec!["JOHN DOE".to_string()])?);
```

### 📊 Performance & Quality

#### Code Quality
- **Rust Best Practices**: Follows Rust idioms and best practices
- **Memory Safety**: Zero unsafe code, leveraging Rust's safety guarantees
- **Error Handling**: Comprehensive Result-based error handling
- **Type Safety**: Strong typing throughout the codebase

#### Performance
- **Efficient Parsing**: Optimized parsing algorithms
- **Minimal Allocations**: Efficient memory usage
- **Fast Serialization**: Custom serialization for optimal performance

## [1.x] - Previous Versions

### Legacy Features
- Basic field parsing
- Simple JSON serialization
- Limited field validation
- Basic MT103 support

---

## Migration Notes

When upgrading from 1.x to 2.x, please note:

1. **Field50 and Field59** are now enums - update your code to use the appropriate variants
2. **JSON format** has changed - update any JSON parsing logic
3. **Field names** have been aligned with SWIFT specifications
4. **BIC validation** is stricter - ensure your BIC codes are 8 or 11 characters
5. **Institution fields** now support account_line_indicator

For detailed migration examples, see the README.md file. 