# Changelog

All notable changes to SwiftMTMessage will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2025-01-26

### Added
- **Error Collection Mode**: New parsing mode that collects all field errors instead of failing fast
  - `SwiftParser::with_config()` - Configure parser behavior
  - `parse_with_errors()` - Returns `ParseResult` enum with detailed error information
  - `ParseResult::PartialSuccess` - Extract valid fields even with some errors
- **Parser Configuration**: New `ParserConfig` struct with options:
  - `fail_fast` - Choose between fail-fast and error collection modes
  - `validate_optional_fields` - Control validation of optional fields
  - `collect_all_errors` - Enable comprehensive error collection
- **Enhanced Error Types**:
  - `ParseErrorCollection` - Container for multiple parse errors
  - `ParseError::MultipleErrors` variant - Represents collected errors
- **Backward Compatibility**: Existing `parse()` method continues to work unchanged

### Changed
- SwiftParser is now instance-based instead of static to support configuration
- Message macro generation updated to support error collection in `from_fields_with_config`
- Enhanced error handling documentation with v3.0 features

### Fixed
- All existing functionality remains unchanged for backward compatibility

## [2.x] - Previous Versions

### Features
- Macro-driven architecture with `#[derive(SwiftField)]` and `#[derive(SwiftMessage)]`
- Comprehensive SWIFT MT message support (MT101-MT950)
- Sample data generation with JSON configuration
- Enhanced error reporting with contextual information
- Type-safe field parsing and validation
- Serde-like JSON serialization optimized for financial data