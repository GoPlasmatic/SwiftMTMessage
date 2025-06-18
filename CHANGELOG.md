# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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