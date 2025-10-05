//! Example demonstrating error handling in Swift MT Parser
//!
//! This example shows how to handle parsing errors when parsing SWIFT MT messages.

use swift_mt_message::ParseResult;
use swift_mt_message::messages::MT103;
use swift_mt_message::parser::SwiftParser;

fn main() {
    // Example MT103 message with errors
    let mt103_with_errors = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:INVALID REF WITH SPACES
:23B:INVALID_CODE
:32A:24031XUSD1000,00
:50K:JOHN DOE
123 MAIN ST
NEW YORK, NY
:52A:INVALID BIC
:59:DE89370400440532013000
BENEFICIARY NAME
BENEFICIARY ADDRESS
:70:PAYMENT DETAILS
WITH MULTIPLE LINES
EXCEEDING LIMIT
AND MORE LINES
:71A:INVALID_CHARGE_CODE
-}"#;

    println!("=== Swift MT Parser Error Handling Example ===\n");

    // Example 1: Traditional parsing (stops at first error)
    println!("1. Traditional parsing:");
    match SwiftParser::parse::<MT103>(mt103_with_errors) {
        Ok(msg) => println!("✓ Message parsed successfully: {:?}", msg.fields.field_20),
        Err(e) => println!("✗ Parsing failed: {e}"),
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 2: Error collection mode - collect all errors
    println!("2. Error collection mode (collect all errors):");
    let parser = SwiftParser::new();

    match parser.parse_with_errors::<MT103>(mt103_with_errors) {
        Ok(ParseResult::Success(msg)) => {
            println!("✓ Message parsed successfully without errors");
            println!("  Transaction Reference: {:?}", msg.fields.field_20);
        }
        Ok(ParseResult::PartialSuccess(msg, errors)) => {
            println!(
                "⚠ Message parsed with {} non-critical errors:",
                errors.len()
            );
            for (i, error) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error.brief_message());
            }
            println!("\n  Transaction Reference: {:?}", msg.fields.field_20);
        }
        Ok(ParseResult::Failure(errors)) => {
            println!("✗ Message parsing failed with {} errors:", errors.len());
            for (i, error) in errors.iter().enumerate() {
                println!("\n  Error {}:", i + 1);
                println!("  {}", error.brief_message());
            }
        }
        Err(e) => {
            println!("✗ Unexpected error: {e}");
            println!("  Debug: {e:?}");
        }
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 3: Valid message to show success case
    println!("3. Valid message (no errors):");
    let valid_mt103 = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:REF123456
:23B:CRED
:32A:240315USD1000,00
:50K:JOHN DOE
123 MAIN ST
NEW YORK, NY
:59:DE89370400440532013000
BENEFICIARY NAME
:71A:SHA
-}"#;

    match parser.parse_with_errors::<MT103>(valid_mt103) {
        Ok(ParseResult::Success(msg)) => {
            println!("✓ Message parsed successfully without errors");
            println!("  Transaction Reference: {}", msg.fields.field_20.reference);
            println!(
                "  Amount: {} {}",
                msg.fields.field_32a.amount, msg.fields.field_32a.currency
            );
        }
        Ok(ParseResult::PartialSuccess(_, errors)) => {
            println!("⚠ Message parsed with {} non-critical errors", errors.len());
        }
        Ok(ParseResult::Failure(errors)) => {
            println!("✗ Message parsing failed with {} errors", errors.len());
        }
        Err(e) => println!("✗ Unexpected error: {e}"),
    }

    println!("\n{}\n", "=".repeat(50));

    // Example 4: Using the convenience method
    println!("4. Using parse_message convenience method:");
    match parser.parse_message::<MT103>(mt103_with_errors) {
        Ok(msg) => {
            println!("✓ Message parsed successfully");
            println!("  Transaction Reference: {:?}", msg.fields.field_20);
        }
        Err(e) => {
            println!("✗ Parsing failed: {e}");
        }
    }
}
