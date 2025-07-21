# SwiftMTMessage Codebase Analysis Report

**Last Updated**: After Phase 1 Code Duplication Refactoring

**Note**: Line numbers mentioned in this report may have shifted due to refactoring. The locations and patterns described remain valid but specific line references should be verified.

## Executive Summary

This report analyzes the SwiftMTMessage codebase focusing on:
1. Code duplication patterns ✅ **PARTIALLY COMPLETED**
2. Performance issues with regex compilation and string allocations
3. Error handling redundancy and inconsistencies
4. Memory allocation patterns in parsing
5. Opportunities for const fn and compile-time optimizations

## 1. Code Duplication Patterns

### 1.1 Field Generation Pattern Duplication ✅ **COMPLETED**

**Location**: `swift-mt-message-macros/src/codegen/field.rs`

**Status**: This has been addressed in the Phase 1 refactoring. The duplicated patterns have been extracted into helper functions in `swift-mt-message-macros/src/codegen/helpers.rs`:

- `generate_optional_prefix_field()` - Handles patterns with optional prefixes
- `generate_account_bic_field()` - Handles account/BIC patterns  
- `generate_numbered_lines_field()` - Handles Field59F line numbering pattern

**Result**: ~120 lines of duplicate code removed, 30% reduction in field.rs size.

### 1.2 Error Creation Pattern Duplication 🔄 **PENDING** (Phase 2)

**Location**: `swift-mt-message-macros/src/codegen/field.rs` and `message.rs`

**Status**: Not yet addressed. The error creation pattern is repeated extensively:
```rust
// Repeated pattern in field.rs lines 88-95, 100-107, 128-135
// and message.rs lines 151-158, 193-202, 223-230
crate::errors::ParseError::InvalidFieldFormat {
    field_tag: field_tag.to_string(),
    component_name: component_name.to_string(),
    value: value.to_string(),
    format_spec: format_spec.to_string(),
    position: None,
    inner_error: e.to_string(),
}
```

**Recommendation**: Create error builder functions:
```rust
impl ParseError {
    #[inline]
    pub fn invalid_field_format(
        field_tag: &str,
        component_name: &str,
        value: &str,
        format_spec: &str,
        inner_error: impl std::fmt::Display,
    ) -> Self {
        Self::InvalidFieldFormat {
            field_tag: field_tag.to_string(),
            component_name: component_name.to_string(),
            value: value.to_string(),
            format_spec: format_spec.to_string(),
            position: None,
            inner_error: inner_error.to_string(),
        }
    }
}
```

### 1.3 Sample Implementation Pattern Duplication 🔄 **PENDING**

**Location**: `swift-mt-message-macros/src/codegen/message.rs`

**Status**: Not yet addressed. Lines 316-353, 356-393, 396-433 contain nearly identical patterns for generating sample implementations.

**Recommendation**: Use a single generic function with configuration:
```rust
fn generate_sample_impl_generic(fields: &[MessageField], include_optional: bool, multiple: bool) -> MacroResult<TokenStream> {
    // Single implementation handling all variants
}
```

## 2. Performance Issues

### 2.1 Regex Compilation Issues 🔄 **PENDING**

**Location**: `swift-mt-message-macros/src/codegen/pattern_generators.rs`

**Status**: Not yet addressed. Static regex patterns are being created but not efficiently cached:
```rust
// Lines 50-55, 114-116, 182-185 - Pattern repeated for each field
static PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(#regex_pattern).unwrap()
});
```

**Issue**: Each generated field creates its own static regex, leading to many duplicate compiled regexes.

**Recommendation**: Use a global regex cache:
```rust
// In swift-mt-message/src/parser.rs or a new caching module
pub static REGEX_CACHE: Lazy<DashMap<&'static str, Regex>> = Lazy::new(|| {
    let cache = DashMap::new();
    // Pre-populate with common patterns
    cache.insert("^([A-Z]{3})$", Regex::new("^([A-Z]{3})$").unwrap());
    cache.insert("^(\\d{6})$", Regex::new("^(\\d{6})$").unwrap());
    // ... more common patterns
    cache
});
```

### 2.2 String Allocation Issues 🔄 **PENDING**

**Location**: Multiple files

**Status**: Not yet addressed. Excessive string allocations found in:
1. `swift-mt-message-macros/src/format.rs` - format_to_description function
2. `swift-mt-message/src/parser.rs` - normalize_field_tag function
3. Error creation throughout the codebase

**Examples**:
```rust
// format.rs line 234
parse_swift_format_to_description(pattern) // Returns new String

// parser.rs line 595
Cow::Owned(numeric_part.to_string()) // Unnecessary allocation

// errors.rs - all error creation allocates strings
field_tag: field_tag.to_string(), // Could use &'static str or Cow<'static, str>
```

**Recommendation**: 
1. Use `Cow<'static, str>` for error fields
2. Return `&'static str` from format_to_description where possible
3. Use string interning for common field tags

### 2.3 HashMap Pre-allocation ✅ **ALREADY GOOD**

**Location**: `swift-mt-message/src/parser.rs` line 481

**Status**: Good practice already implemented:
```rust
let mut field_map: HashMap<String, Vec<(String, usize)>> = 
    HashMap::with_capacity(estimated_fields);
```

However, the `Vec` inside could also be pre-allocated based on typical field repetition patterns.

## 3. Error Handling Redundancy

### 3.1 Multiple Error Type Conversions

**Location**: `swift-mt-message/src/errors.rs`

The error system has redundant conversions between:
- `ValidationError` ↔ `SwiftValidationError`
- `SwiftValidationError` → `ParseError`
- `SwiftValidationError` → `ValidationError`

**Issue**: Creates allocation overhead and complexity.

**Recommendation**: Consolidate error types:
```rust
pub enum ParseError {
    // Single validation variant instead of multiple
    Validation {
        code: &'static str,
        field: String,
        message: String,
        category: ValidationCategory,
    },
    // ... other variants
}
```

### 3.2 Error Message Duplication

Many error messages are duplicated across the codebase. Consider using const strings:
```rust
mod error_messages {
    pub const INVALID_FORMAT: &str = "Invalid field format";
    pub const MISSING_REQUIRED: &str = "Missing required field";
    // ... etc
}
```

## 4. Memory Allocation Patterns

### 4.1 Parser Field Consumption

**Location**: `swift-mt-message/src/parser.rs`

The `FieldConsumptionTracker` allocates a new `HashSet` for each field tag:
```rust
// Line 154-162
match self.consumed_indices.entry(tag.to_string()) {
    Entry::Occupied(mut e) => {
        e.get_mut().insert(index);
    }
    Entry::Vacant(e) => {
        let mut set = HashSet::new(); // Allocation here
        set.insert(index);
        e.insert(set);
    }
}
```

**Recommendation**: Use `SmallVec` for fields that typically have few duplicates:
```rust
use smallvec::SmallVec;
consumed_indices: HashMap<String, SmallVec<[usize; 4]>>
```

### 4.2 String Building Patterns

**Location**: `swift-mt-message-macros/src/format.rs`

Good practice of pre-allocating capacity is used:
```rust
// Lines 521-525, 547-553, etc.
let mut result = String::with_capacity(pattern.len() + 3);
```

However, some functions still use format! macro unnecessarily:
```rust
// Line 451 - only for uncommon cases, good
format!(r"(\d{{1,{length}}}(?:[.,]\d+)?)")
```

## 5. Const fn and Compile-time Optimization Opportunities

### 5.1 Format Descriptions

**Location**: `swift-mt-message-macros/src/format.rs`

The `format_to_description` function could be partially const:
```rust
const fn format_to_description_const(pattern: &'static str) -> &'static str {
    match pattern {
        "3!a" => "Exactly 3 uppercase letters (e.g., USD, EUR, SHA)",
        "6!n" => "Exactly 6 digits (e.g., date YYMMDD)",
        // ... other static patterns
        _ => "Unknown format",
    }
}
```

### 5.2 Regex Pattern Building

Many regex patterns are known at compile time and could be const:
```rust
const CURRENCY_PATTERN: &str = "^([A-Z]{3})$";
const DATE_PATTERN: &str = "^(\\d{6})$";
const BIC_PATTERN: &str = "^([A-Z]{4}[A-Z]{2}[A-Z0-9]{2}(?:[A-Z0-9]{3})?)$";
```

### 5.3 Field Tag Normalization

The field tag normalization logic in `parser.rs` could use a const lookup table:
```rust
const PRESERVE_SUFFIX_TAGS: &[&str] = &[
    "11", "13", "23", "26", "32", "33", "50", "52", "53", 
    "54", "55", "56", "57", "58", "59", "71", "77"
];

#[inline]
const fn should_preserve_suffix(numeric_part: &str) -> bool {
    // Binary search or perfect hash at compile time
}
```

## 6. Specific Optimization Recommendations

### 6.1 Inline Small Functions 🔄 **PENDING**

Add `#[inline]` attributes to small, frequently-called functions:
```rust
#[inline]
pub fn mark_consumed(&mut self, tag: &str, index: usize)

#[inline]
fn normalize_field_tag(raw_tag: &str) -> Cow<'_, str>
```

### 6.2 Use SmallString for Short Strings

For field tags and short identifiers:
```rust
use smallstr::SmallString;
type FieldTag = SmallString<[u8; 8]>; // Most tags fit in 8 bytes
```

### 6.3 Lazy Static Optimization

Convert runtime regex compilation to compile-time where possible:
```rust
// Instead of runtime Lazy<Regex>
const PATTERNS: phf::Map<&'static str, &'static str> = phf_map! {
    "3!a" => "^([A-Z]{3})$",
    "6!n" => "^(\\d{6})$",
    // ...
};
```

### 6.4 Reduce Allocations in Error Paths

Use zero-copy techniques:
```rust
pub struct ParseError<'a> {
    field_tag: Cow<'a, str>,
    message: Cow<'a, str>,
    // ...
}
```

## 7. Priority Actions

### Completed ✅
1. **Extract duplicated code patterns into reusable functions** - Phase 1 completed for field generation patterns

### High Priority (Remaining) 🔴
1. Implement global regex cache to eliminate duplicate compilations
2. Reduce string allocations in error creation using Cow types
3. Extract remaining error creation duplication (Phase 2)

### Medium Priority 🟡
1. Use SmallVec for field consumption tracking
2. Add inline attributes to hot-path functions
3. Implement const functions for compile-time known patterns

### Low Priority 🟢
1. Consider using string interning for field tags
2. Optimize error type conversions
3. Use perfect hashing for field tag lookups

## Status Summary

### What's Been Completed
- ✅ Field generation pattern duplication eliminated (saved ~120 lines)
- ✅ Helper functions created for common serialization patterns
- ✅ Fixed Field53B/Field57B serialization bug discovered during refactoring

### What Remains
- 🔄 Error creation pattern duplication (Phase 2)
- 🔄 Sample implementation duplication (Phase 3)
- 🔄 Performance optimizations (regex caching, string allocations)
- 🔄 Memory optimization patterns
- 🔄 Const fn opportunities

## Conclusion

Phase 1 of the refactoring has been successfully completed, addressing the most significant code duplication in field generation. The codebase is now more maintainable with ~30% reduction in macro generation code size. The remaining optimizations focus on performance improvements and further code cleanup.