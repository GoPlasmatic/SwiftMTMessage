use super::ComponentParser;

/// Enhanced component parser registry for handling complex SWIFT field patterns
pub struct ComponentParserRegistry;

impl ComponentParserRegistry {
    pub fn new() -> Self {
        ComponentParserRegistry
    }

    pub fn get(&self, pattern: &str) -> Option<ComponentParser> {
        match pattern {
            "6!n" => Some(ComponentParser::SwiftDate),
            "3!a" => Some(ComponentParser::Currency),
            "15d" => Some(ComponentParser::Amount),
            "12d" => Some(ComponentParser::DecimalAmount), // Decimal amount pattern
            "3!n" => Some(ComponentParser::EntryCount),    // Entry count pattern
            "4!n" => Some(ComponentParser::Numeric4),      // 4-digit numeric pattern
            "1!x" => Some(ComponentParser::SingleSign),    // Single sign character
            "4!c" => Some(ComponentParser::AlphaCode),
            "3!c" => Some(ComponentParser::AlphaCode3), // 3-character alphanumeric code
            "16x" => Some(ComponentParser::Alphanumeric),
            "35x" => Some(ComponentParser::Text35), // 35-character text pattern
            "1!a" => Some(ComponentParser::SingleChar),
            "1!a/1!a/35x" => Some(ComponentParser::EnvelopeContent), // Field77T pattern
            "1!a[N]12d" => Some(ComponentParser::InterestRate),      // Field37H pattern
            "[1!a]" => Some(ComponentParser::OptionalSingleChar),    // Optional single char
            "BIC" => Some(ComponentParser::Bic),
            // New patterns for Phase 1
            "5n[/2n]" => Some(ComponentParser::StatementSequence),   // Field28 pattern
            "5n[/5n]" => Some(ComponentParser::StatementSequence5),  // Field28C pattern
            "6!n4!n1!x4!n" => Some(ComponentParser::DateTimeOffset), // Field13D pattern
            "3!a[2!n]11x" => Some(ComponentParser::FunctionCodeRef), // Field23 pattern
            _ => None,
        }
    }
} 