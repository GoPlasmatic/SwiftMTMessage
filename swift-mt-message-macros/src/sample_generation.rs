//! Type-safe sample generation for SWIFT field patterns
//!
//! This module provides compile-time type-safe sample generation based on
//! validated format specifications. Each format specification knows how to
//! generate appropriate sample data for its expected type.

use crate::format_validation::{FormatSpecType, ValidatedFormatSpec};
use proc_macro2::TokenStream;
use quote::quote;

/// Constraints for sample generation
#[derive(Debug, Clone)]
#[allow(dead_code)] // Phase 3 infrastructure - constraint types for enhanced sample generation
pub enum SampleConstraint {
    /// Must be a valid currency code
    ValidCurrency,
    /// Must be a valid BIC code
    ValidBic,
    /// Must be a valid date in specified format
    ValidDate { format: DateFormat },
    /// Must be a valid amount with currency precision
    ValidAmount { currency: Option<String> },
    /// Must match specific pattern
    Pattern { regex: String },
    /// Must be from a predefined list
    OneOf { values: Vec<String> },
    /// Custom validation function
    Custom { validator_name: String },
}

/// Enhanced configuration for sample generation with type constraints
#[derive(Debug, Clone)]
pub struct SampleConfig {
    /// Generate minimal or full samples
    pub scenario: SampleScenario,
    /// Use specific currency codes
    pub currency: Option<String>,
    /// Custom constraints for generation
    pub constraints: Vec<SampleConstraint>,
    /// Context for smart generation (e.g., field name, message type)
    pub context: SampleContext,
}

/// Date formats for constraints
#[derive(Debug, Clone)]
#[allow(dead_code)] // Phase 3 infrastructure - date format types for constraints
pub enum DateFormat {
    YYMMDD,
    YYYYMMDD,
    HHMM,
    HHMMSS,
}

/// Context for intelligent sample generation
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // Phase 3 infrastructure - context for smart sample generation
pub struct SampleContext {
    /// Field name for context-aware generation
    pub field_name: Option<String>,
    /// Message type for context-aware generation  
    pub message_type: Option<String>,
    /// Related field values for consistency
    pub related_fields: std::collections::HashMap<String, String>,
}

/// Sample generation scenarios
#[derive(Debug, Clone, PartialEq)]
pub enum SampleScenario {
    /// Minimal valid sample
    #[allow(dead_code)]
    Minimal,
    /// Full featured sample
    Full,
}

/// Type-safe sample generator trait with enhanced constraints
#[allow(dead_code)] // Phase 3 infrastructure - trait methods for advanced sample generation
pub trait SampleGenerator<T> {
    /// Generate a sample value with default settings
    fn generate_sample(&self) -> T;

    /// Generate a sample value with configuration and constraints
    fn generate_with_config(&self, config: &SampleConfig) -> T;

    /// Generate a sample value with specific constraints only
    fn generate_with_constraints(&self, constraints: &[SampleConstraint]) -> T;

    /// Validate if a generated sample meets the constraints
    fn validate_sample(&self, sample: &T, constraints: &[SampleConstraint]) -> bool;
}

/// Smart sample generator that applies constraints and context
#[allow(dead_code)] // Phase 3 infrastructure - smart sample generator for context-aware generation
pub struct SmartSampleGenerator;

#[allow(dead_code)] // Phase 3 infrastructure - smart sample generation implementation
impl SmartSampleGenerator {
    /// Generate context-aware samples based on field names and types
    pub fn generate_smart_sample(spec: &ValidatedFormatSpec, config: &SampleConfig) -> String {
        // Apply context-aware generation
        if let Some(field_name) = &config.context.field_name {
            if let Some(smart_value) = generate_context_aware_sample(field_name, spec, config) {
                return smart_value;
            }
        }

        // Apply constraints
        for constraint in &config.constraints {
            if let Some(constrained_value) = apply_constraint(constraint, spec, config) {
                return constrained_value;
            }
        }

        // Fallback to standard generation
        generate_standard_sample(spec, config)
    }
}

/// Generate context-aware samples based on field names
#[allow(dead_code)] // Phase 3 infrastructure - context-aware sample generation functions
fn generate_context_aware_sample(
    field_name: &str,
    spec: &ValidatedFormatSpec,
    config: &SampleConfig,
) -> Option<String> {
    match field_name.to_lowercase().as_str() {
        "currency" | "ccy" => Some(config.currency.clone().unwrap_or_else(|| "USD".to_string())),
        "bic" | "bank_code" => Some(generate_sample_bic(spec)),
        "amount" | "amt" => Some(generate_sample_amount(spec, config)),
        "date" => Some(generate_sample_date(spec)),
        "time" => Some(generate_sample_time(spec)),
        "reference" | "ref" => Some(generate_sample_reference(spec)),
        "account" | "acc" => Some(generate_sample_account(spec)),
        "name" => Some(generate_sample_name(spec)),
        "address" => Some(generate_sample_address(spec)),
        "country" => Some("US".to_string()),
        _ => None,
    }
}

/// Apply specific constraints to sample generation
fn apply_constraint(
    constraint: &SampleConstraint,
    spec: &ValidatedFormatSpec,
    config: &SampleConfig,
) -> Option<String> {
    match constraint {
        SampleConstraint::ValidCurrency => Some(generate_valid_currency()),
        SampleConstraint::ValidBic => Some(generate_sample_bic(spec)),
        SampleConstraint::ValidDate { format } => Some(generate_date_with_format(format)),
        SampleConstraint::ValidAmount { currency } => {
            let default_currency = "USD".to_string();
            let ccy = currency
                .as_ref()
                .or(config.currency.as_ref())
                .unwrap_or(&default_currency);
            Some(generate_amount_for_currency(ccy, spec))
        }
        SampleConstraint::Pattern { regex: _ } => {
            // For now, return None to use standard generation
            // In a full implementation, would generate matching the regex
            None
        }
        SampleConstraint::OneOf { values } => {
            if !values.is_empty() {
                Some(values[0].clone())
            } else {
                None
            }
        }
        SampleConstraint::Custom { validator_name: _ } => {
            // For now, return None to use standard generation
            // In a full implementation, would call the custom validator
            None
        }
    }
}

/// Context-aware sample generation functions
fn generate_sample_bic(spec: &ValidatedFormatSpec) -> String {
    match &spec.spec_type {
        FormatSpecType::FixedAlphabetic { length: 8 } => "CHASUS33".to_string(),
        FormatSpecType::FixedAlphabetic { length: 11 } => "CHASUS33XXX".to_string(),
        FormatSpecType::VariableAlphabetic { max_length } => {
            if *max_length >= 11 {
                "CHASUS33XXX".to_string()
            } else {
                "CHASUS33".to_string()
            }
        }
        _ => "CHASUS33".to_string(),
    }
}

fn generate_sample_amount(spec: &ValidatedFormatSpec, config: &SampleConfig) -> String {
    let currency = config.currency.as_deref().unwrap_or("USD");
    generate_amount_for_currency(currency, spec)
}

fn generate_sample_date(spec: &ValidatedFormatSpec) -> String {
    match &spec.spec_type {
        FormatSpecType::FixedNumeric { length: 6 } => "241201".to_string(), // YYMMDD
        FormatSpecType::FixedNumeric { length: 8 } => "20241201".to_string(), // YYYYMMDD
        _ => "241201".to_string(),
    }
}

fn generate_sample_time(spec: &ValidatedFormatSpec) -> String {
    match &spec.spec_type {
        FormatSpecType::FixedNumeric { length: 4 } => "1430".to_string(), // HHMM
        FormatSpecType::FixedNumeric { length: 6 } => "143045".to_string(), // HHMMSS
        _ => "1430".to_string(),
    }
}

fn generate_sample_reference(spec: &ValidatedFormatSpec) -> String {
    match &spec.spec_type {
        FormatSpecType::VariableAny { max_length } => {
            let base = "TXN";
            let suffix = "20241201123456789";
            let combined = format!("{base}{suffix}");
            if combined.len() <= *max_length {
                combined
            } else {
                combined[..*max_length].to_string()
            }
        }
        _ => "TXN123456789".to_string(),
    }
}

fn generate_sample_account(spec: &ValidatedFormatSpec) -> String {
    match &spec.spec_type {
        FormatSpecType::VariableAny { max_length } => {
            let base = "123456789012345678901234";
            if base.len() <= *max_length {
                base.to_string()
            } else {
                base[..*max_length].to_string()
            }
        }
        _ => "1234567890".to_string(),
    }
}

fn generate_sample_name(spec: &ValidatedFormatSpec) -> String {
    match &spec.spec_type {
        FormatSpecType::VariableAny { max_length } => {
            let name = "JOHN DOE ENTERPRISES LTD";
            if name.len() <= *max_length {
                name.to_string()
            } else {
                name[..*max_length].to_string()
            }
        }
        _ => "JOHN DOE".to_string(),
    }
}

fn generate_sample_address(spec: &ValidatedFormatSpec) -> String {
    match &spec.spec_type {
        FormatSpecType::VariableAny { max_length } => {
            let address = "123 MAIN STREET, NEW YORK, NY";
            if address.len() <= *max_length {
                address.to_string()
            } else {
                address[..*max_length].to_string()
            }
        }
        _ => "123 MAIN ST".to_string(),
    }
}

fn generate_valid_currency() -> String {
    "USD".to_string()
}

fn generate_date_with_format(format: &DateFormat) -> String {
    match format {
        DateFormat::YYMMDD => "241201".to_string(),
        DateFormat::YYYYMMDD => "20241201".to_string(),
        DateFormat::HHMM => "1430".to_string(),
        DateFormat::HHMMSS => "143045".to_string(),
    }
}

fn generate_amount_for_currency(currency: &str, spec: &ValidatedFormatSpec) -> String {
    let precision = match currency {
        "JPY" | "KRW" => 0,         // No decimal places
        "BHD" | "KWD" | "OMR" => 3, // 3 decimal places
        _ => 2,                     // Most currencies use 2 decimal places
    };

    match &spec.spec_type {
        FormatSpecType::Decimal { max_digits } => {
            let integer_part = if *max_digits > precision + 2 {
                "12345".to_string()
            } else {
                "123".to_string()
            };

            if precision == 0 {
                integer_part
            } else {
                format!("{}.{}", integer_part, "0".repeat(precision))
            }
        }
        _ => match precision {
            0 => "12345".to_string(),
            3 => "12345.000".to_string(),
            _ => "12345.67".to_string(),
        },
    }
}

fn generate_standard_sample(spec: &ValidatedFormatSpec, config: &SampleConfig) -> String {
    // This delegates to the existing generate_sample_for_spec logic
    match &spec.spec_type {
        FormatSpecType::FixedAlphabetic { length } => {
            let generator = AlphabeticGenerator {
                length: Some(*length),
                max_length: None,
            };
            generator.generate_with_config(config)
        }
        FormatSpecType::VariableAlphabetic { max_length } => {
            let generator = AlphabeticGenerator {
                length: None,
                max_length: Some(*max_length),
            };
            generator.generate_with_config(config)
        }
        FormatSpecType::FixedNumeric { length } => {
            let generator = NumericGenerator {
                length: Some(*length),
                max_length: None,
            };
            generator.generate_with_config(config)
        }
        FormatSpecType::VariableNumeric { max_length } => {
            let generator = NumericGenerator {
                length: None,
                max_length: Some(*max_length),
            };
            generator.generate_with_config(config)
        }
        FormatSpecType::Decimal { max_digits } => {
            let generator = DecimalGenerator {
                max_digits: *max_digits,
            };
            generator.generate_with_config(config)
        }
        FormatSpecType::FixedCharacterSet { length } => {
            let generator = CharacterSetGenerator {
                length: Some(*length),
                max_length: None,
            };
            generator.generate_with_config(config)
        }
        FormatSpecType::VariableCharacterSet { max_length } => {
            let generator = CharacterSetGenerator {
                length: None,
                max_length: Some(*max_length),
            };
            generator.generate_with_config(config)
        }
        FormatSpecType::VariableAny { max_length } => generate_any_character_sample(*max_length),
        FormatSpecType::Optional { inner } => generate_standard_sample(inner, config),
        FormatSpecType::Repetitive { count: _, inner } => generate_standard_sample(inner, config),
        FormatSpecType::MultiComponent { components } => {
            if !components.is_empty() {
                generate_standard_sample(&components[0], config)
            } else {
                "SAMPLE".to_string()
            }
        }
    }
}

/// Alphabetic sample generator
pub struct AlphabeticGenerator {
    pub length: Option<usize>,
    pub max_length: Option<usize>,
}

impl SampleGenerator<String> for AlphabeticGenerator {
    fn generate_sample(&self) -> String {
        if let Some(length) = self.length {
            generate_alphabetic_fixed(length)
        } else if let Some(max_length) = self.max_length {
            generate_alphabetic_variable(max_length)
        } else {
            "ABC".to_string()
        }
    }

    fn generate_with_config(&self, config: &SampleConfig) -> String {
        // Check for context-aware generation first
        if let Some(field_name) = &config.context.field_name {
            match field_name.to_lowercase().as_str() {
                "currency" | "ccy" => {
                    if let Some(currency) = &config.currency {
                        return currency.clone();
                    }
                }
                "country" => return "US".to_string(),
                _ => {}
            }
        }

        // Apply constraints
        for constraint in &config.constraints {
            match constraint {
                SampleConstraint::ValidCurrency => return generate_valid_currency(),
                SampleConstraint::OneOf { values } => {
                    if !values.is_empty() {
                        return values[0].clone();
                    }
                }
                _ => {}
            }
        }

        // Standard generation based on scenario
        match config.scenario {
            SampleScenario::Minimal => {
                if let Some(length) = self.length {
                    "A".repeat(length)
                } else {
                    "A".to_string()
                }
            }
            SampleScenario::Full => self.generate_sample(),
        }
    }

    fn generate_with_constraints(&self, constraints: &[SampleConstraint]) -> String {
        for constraint in constraints {
            match constraint {
                SampleConstraint::ValidCurrency => return generate_valid_currency(),
                SampleConstraint::OneOf { values } => {
                    if !values.is_empty() {
                        return values[0].clone();
                    }
                }
                _ => {}
            }
        }
        self.generate_sample()
    }

    fn validate_sample(&self, sample: &String, constraints: &[SampleConstraint]) -> bool {
        for constraint in constraints {
            match constraint {
                SampleConstraint::ValidCurrency => {
                    let valid_currencies = ["USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD"];
                    if !valid_currencies.contains(&sample.as_str()) {
                        return false;
                    }
                }
                SampleConstraint::OneOf { values } => {
                    if !values.contains(sample) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }
}

/// Numeric sample generator
pub struct NumericGenerator {
    pub length: Option<usize>,
    pub max_length: Option<usize>,
}

impl SampleGenerator<String> for NumericGenerator {
    fn generate_sample(&self) -> String {
        if let Some(length) = self.length {
            generate_numeric_fixed(length)
        } else if let Some(max_length) = self.max_length {
            generate_numeric_variable(max_length)
        } else {
            "123".to_string()
        }
    }

    fn generate_with_config(&self, config: &SampleConfig) -> String {
        // Apply constraints
        for constraint in &config.constraints {
            if let SampleConstraint::ValidDate { format } = constraint {
                return generate_date_with_format(format);
            }
        }

        match config.scenario {
            SampleScenario::Minimal => {
                if let Some(length) = self.length {
                    "1".repeat(length)
                } else {
                    "1".to_string()
                }
            }
            SampleScenario::Full => self.generate_sample(),
        }
    }

    fn generate_with_constraints(&self, constraints: &[SampleConstraint]) -> String {
        for constraint in constraints {
            if let SampleConstraint::ValidDate { format } = constraint {
                return generate_date_with_format(format);
            }
        }
        self.generate_sample()
    }

    fn validate_sample(&self, sample: &String, constraints: &[SampleConstraint]) -> bool {
        for constraint in constraints {
            if let SampleConstraint::ValidDate { format: _ } = constraint {
                // Basic validation - all characters should be digits
                if !sample.chars().all(|c| c.is_ascii_digit()) {
                    return false;
                }
            }
        }
        true
    }
}

/// Decimal sample generator
pub struct DecimalGenerator {
    pub max_digits: usize,
}

impl SampleGenerator<String> for DecimalGenerator {
    fn generate_sample(&self) -> String {
        match self.max_digits {
            1..=3 => "12.50".to_string(),
            4..=6 => "1234.56".to_string(),
            7..=12 => "123456.78".to_string(),
            _ => "1234567890.12".to_string(),
        }
    }

    fn generate_with_config(&self, config: &SampleConfig) -> String {
        // Apply constraints
        for constraint in &config.constraints {
            if let SampleConstraint::ValidAmount { currency } = constraint {
                let default_currency = "USD".to_string();
                let ccy = currency
                    .as_ref()
                    .or(config.currency.as_ref())
                    .unwrap_or(&default_currency);
                return generate_amount_for_currency(
                    ccy,
                    &ValidatedFormatSpec {
                        pattern: format!("{}d", self.max_digits),
                        spec_type: FormatSpecType::Decimal {
                            max_digits: self.max_digits,
                        },
                    },
                );
            }
        }

        match config.scenario {
            SampleScenario::Minimal => "1.00".to_string(),
            SampleScenario::Full => match self.max_digits {
                1..=6 => "12345.67".to_string(),
                _ => "1234567890.12".to_string(),
            },
        }
    }

    fn generate_with_constraints(&self, constraints: &[SampleConstraint]) -> String {
        for constraint in constraints {
            if let SampleConstraint::ValidAmount { currency } = constraint {
                let default_currency = "USD".to_string();
                let ccy = currency.as_ref().unwrap_or(&default_currency);
                return generate_amount_for_currency(
                    ccy,
                    &ValidatedFormatSpec {
                        pattern: format!("{}d", self.max_digits),
                        spec_type: FormatSpecType::Decimal {
                            max_digits: self.max_digits,
                        },
                    },
                );
            }
        }
        self.generate_sample()
    }

    fn validate_sample(&self, sample: &String, constraints: &[SampleConstraint]) -> bool {
        for constraint in constraints {
            if let SampleConstraint::ValidAmount { currency: _ } = constraint {
                // Basic decimal validation
                if !sample.contains('.') {
                    return false;
                }
                let parts: Vec<&str> = sample.split('.').collect();
                if parts.len() != 2 {
                    return false;
                }
                if !parts[0].chars().all(|c| c.is_ascii_digit())
                    || !parts[1].chars().all(|c| c.is_ascii_digit())
                {
                    return false;
                }
            }
        }
        true
    }
}

/// Character set sample generator
pub struct CharacterSetGenerator {
    pub length: Option<usize>,
    pub max_length: Option<usize>,
}

impl SampleGenerator<String> for CharacterSetGenerator {
    fn generate_sample(&self) -> String {
        if let Some(length) = self.length {
            generate_character_set_fixed(length)
        } else if let Some(max_length) = self.max_length {
            generate_character_set_variable(max_length)
        } else {
            "A1B2".to_string()
        }
    }

    fn generate_with_config(&self, config: &SampleConfig) -> String {
        // Apply constraints
        for constraint in &config.constraints {
            if let SampleConstraint::ValidBic = constraint {
                if let Some(length) = self.length {
                    return match length {
                        8 => "CHASUS33".to_string(),
                        11 => "CHASUS33XXX".to_string(),
                        _ => "CHAS".to_string(),
                    };
                }
            }
        }

        match config.scenario {
            SampleScenario::Minimal => {
                if let Some(length) = self.length {
                    "A".repeat(length.min(10))
                } else {
                    "A".to_string()
                }
            }
            SampleScenario::Full => self.generate_sample(),
        }
    }

    fn generate_with_constraints(&self, constraints: &[SampleConstraint]) -> String {
        for constraint in constraints {
            if let SampleConstraint::ValidBic = constraint {
                if let Some(length) = self.length {
                    return match length {
                        8 => "CHASUS33".to_string(),
                        11 => "CHASUS33XXX".to_string(),
                        _ => "CHAS".to_string(),
                    };
                }
            }
        }
        self.generate_sample()
    }

    fn validate_sample(&self, sample: &String, constraints: &[SampleConstraint]) -> bool {
        for constraint in constraints {
            if let SampleConstraint::ValidBic = constraint {
                // Basic BIC validation - should be alphanumeric
                if !sample.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return false;
                }
                // BIC should be 8 or 11 characters
                if sample.len() != 8 && sample.len() != 11 {
                    return false;
                }
            }
        }
        true
    }
}

/// Generate sample token stream for a validated format specification
#[allow(dead_code)]
pub fn generate_sample_for_spec(
    spec: &ValidatedFormatSpec,
    config: Option<&SampleConfig>,
) -> TokenStream {
    let sample_value = match &spec.spec_type {
        FormatSpecType::FixedAlphabetic { length } => {
            let generator = AlphabeticGenerator {
                length: Some(*length),
                max_length: None,
            };
            if let Some(cfg) = config {
                generator.generate_with_config(cfg)
            } else {
                generator.generate_sample()
            }
        }

        FormatSpecType::VariableAlphabetic { max_length } => {
            let generator = AlphabeticGenerator {
                length: None,
                max_length: Some(*max_length),
            };
            if let Some(cfg) = config {
                generator.generate_with_config(cfg)
            } else {
                generator.generate_sample()
            }
        }

        FormatSpecType::FixedNumeric { length } => {
            let generator = NumericGenerator {
                length: Some(*length),
                max_length: None,
            };
            if let Some(cfg) = config {
                generator.generate_with_config(cfg)
            } else {
                generator.generate_sample()
            }
        }

        FormatSpecType::VariableNumeric { max_length } => {
            let generator = NumericGenerator {
                length: None,
                max_length: Some(*max_length),
            };
            if let Some(cfg) = config {
                generator.generate_with_config(cfg)
            } else {
                generator.generate_sample()
            }
        }

        FormatSpecType::Decimal { max_digits } => {
            let generator = DecimalGenerator {
                max_digits: *max_digits,
            };
            if let Some(cfg) = config {
                generator.generate_with_config(cfg)
            } else {
                generator.generate_sample()
            }
        }

        FormatSpecType::FixedCharacterSet { length } => {
            let generator = CharacterSetGenerator {
                length: Some(*length),
                max_length: None,
            };
            if let Some(cfg) = config {
                generator.generate_with_config(cfg)
            } else {
                generator.generate_sample()
            }
        }

        FormatSpecType::VariableCharacterSet { max_length } => {
            let generator = CharacterSetGenerator {
                length: None,
                max_length: Some(*max_length),
            };
            if let Some(cfg) = config {
                generator.generate_with_config(cfg)
            } else {
                generator.generate_sample()
            }
        }

        FormatSpecType::VariableAny { max_length } => generate_any_character_sample(*max_length),

        FormatSpecType::Optional { inner } => {
            let inner_sample = generate_sample_for_spec(inner, config);
            return quote! { Some(#inner_sample) };
        }

        FormatSpecType::Repetitive { count, inner } => {
            let inner_sample = generate_sample_for_spec(inner, config);
            let samples = (0..*count).map(|_| inner_sample.clone());
            return quote! { vec![#(#samples),*] };
        }

        FormatSpecType::MultiComponent { components } => {
            let samples: Vec<TokenStream> = components
                .iter()
                .map(|component| generate_sample_for_spec(component, config))
                .collect();
            return quote! { (#(#samples),*) };
        }
    };

    quote! { #sample_value.to_string() }
}

// Helper functions for generating specific patterns

fn generate_alphabetic_fixed(length: usize) -> String {
    match length {
        1 => "A".to_string(),
        2 => "US".to_string(),
        3 => "USD".to_string(),
        4 => "CHAS".to_string(),
        8 => "CHASUS33".to_string(),
        11 => "CHASUS33XXX".to_string(),
        _ => "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..length.min(26)].to_string(),
    }
}

fn generate_alphabetic_variable(max_length: usize) -> String {
    let base = "SAMPLE";
    if max_length >= base.len() {
        base.to_string()
    } else {
        base[..max_length].to_string()
    }
}

fn generate_numeric_fixed(length: usize) -> String {
    match length {
        1 => "1".to_string(),
        2 => "12".to_string(),
        3 => "123".to_string(),
        4 => "1230".to_string(),     // HHMM format
        6 => "241201".to_string(),   // YYMMDD format
        8 => "20241201".to_string(), // YYYYMMDD format
        _ => "1234567890".repeat((length / 10) + 1)[..length].to_string(),
    }
}

fn generate_numeric_variable(max_length: usize) -> String {
    let target_length = (max_length / 2).max(1);
    generate_numeric_fixed(target_length)
}

fn generate_character_set_fixed(length: usize) -> String {
    match length {
        1 => "A".to_string(),
        3 => "A1B".to_string(),
        4 => "A1B2".to_string(),
        8 => "CHASUS33".to_string(),
        11 => "CHASUS33XXX".to_string(),
        _ => {
            let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            chars.chars().cycle().take(length).collect()
        }
    }
}

fn generate_character_set_variable(max_length: usize) -> String {
    let target_length = (max_length / 2).max(1);
    generate_character_set_fixed(target_length)
}

fn generate_any_character_sample(max_length: usize) -> String {
    match max_length {
        1..=16 => "Sample Text".to_string(),
        17..=35 => "Sample Transaction Reference".to_string(),
        36..=50 => "Sample Long Description or Address Line".to_string(),
        _ => "Sample Very Long Text Field with Extended Content for Testing".to_string(),
    }
}

impl Default for SampleConfig {
    fn default() -> Self {
        Self {
            scenario: SampleScenario::Full,
            currency: None,
            constraints: Vec::new(),
            context: SampleContext::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format_validation::validate_format_spec;
    use proc_macro2::Span;

    #[test]
    fn test_alphabetic_generator() {
        let generator = AlphabeticGenerator {
            length: Some(3),
            max_length: None,
        };
        let sample = generator.generate_sample();
        assert_eq!(sample.len(), 3);
        assert!(sample.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_numeric_generator() {
        let generator = NumericGenerator {
            length: Some(6),
            max_length: None,
        };
        let sample = generator.generate_sample();
        assert_eq!(sample.len(), 6);
        assert!(sample.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_decimal_generator() {
        let generator = DecimalGenerator { max_digits: 15 };
        let sample = generator.generate_sample();
        assert!(sample.contains('.'));
    }

    #[test]
    fn test_sample_generation_for_spec() {
        let span = Span::call_site();
        let spec = validate_format_spec("3!a", span).unwrap();
        let tokens = generate_sample_for_spec(&spec, None);
        // Should generate valid TokenStream
        assert!(!tokens.to_string().is_empty());
    }
}
