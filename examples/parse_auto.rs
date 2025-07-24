use std::env;
use std::fs;
use swift_mt_message::{ParsedSwiftMessage, SwiftParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the raw message either from file or use default
    let raw_swift_message = if let Some(filename) = env::args().nth(1) {
        // Read from file
        println!("📁 Reading file: {}", filename);
        fs::read_to_string(&filename)
            .map_err(|e| format!("Failed to read file '{filename}': {e}"))?
    } else {
        // Use default MT110 sample message
        r#"{1:F01BANKUS33AXXX0000000000}{2:I210BANKDEFAXXXXN}{4:
:20:REF210TEST001
:25:USD12345678901234
:30:241215
:21:RELREF001
:32B:USD1000000,00
:50:ORDERING CUSTOMER NAME
NEW YORK NY 10001
:56A:DEUTDEFFXXX
:21:RELREF002
:32B:EUR2000000,00
:52A:BANKDE55XXX
:56A:DEUTDEFFXXX
:21:RELREF003
:32B:GBP3000000,00
:50K:/GB1234567890
ANOTHER CUSTOMER
LONDON
-}"#
        .to_string()
    };

    println!("🔍 SWIFT Message Auto-Parser with JSON Conversion");
    println!("{}", "=".repeat(60));

    // Parse with automatic message type detection
    println!("\n📊 Parsing with automatic message type detection...");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse_auto(&raw_swift_message) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed SWIFT message!");

            // Display detected message type
            println!(
                "🏷️  Detected Message Type: MT{}",
                parsed_message.message_type()
            );

            // Convert to JSON and display
            convert_to_json(&parsed_message)?;

            // Validate the message
            validate_message(&parsed_message)?;
        }
        Err(e) => {
            println!("❌ Failed to parse SWIFT message: {e:?}");

            // Provide helpful information about the error
            match e {
                swift_mt_message::ParseError::UnsupportedMessageType { message_type } => {
                    println!(
                        "💡 The message type '{message_type}' is not currently supported by parse_auto."
                    );
                    println!("   Supported types: MT103, MT202, MT205, MT900");
                }
                swift_mt_message::ParseError::InvalidFieldFormat(err) => {
                    println!("💡 Invalid field format:");
                    println!("   Field: {}", err.field_tag);
                    println!("   Component: {}", err.component_name);
                    println!("   Value: '{}'", err.value);
                    println!("   Expected: {}", err.format_spec);
                }
                swift_mt_message::ParseError::MissingRequiredField {
                    field_tag,
                    field_name,
                    message_type,
                    ..
                } => {
                    println!(
                        "💡 Missing required field: {field_tag} ({field_name}) in {message_type}"
                    );
                }
                swift_mt_message::ParseError::FieldParsingFailed {
                    field_tag,
                    field_type,
                    position,
                    original_error,
                } => {
                    println!("💡 Field parsing failed:");
                    println!("   Field tag: {}", field_tag);
                    println!("   Field type: {}", field_type);
                    println!("   Position: {}", position);
                    println!("   Error: {}", original_error);
                    
                    // Extract field content for debugging
                    if let Some(field_start) = raw_swift_message.find(&format!(":{}", field_tag)) {
                        let field_content = &raw_swift_message[field_start..];
                        // Find next field or end of block
                        let end_pos = field_content[1..].find(':').unwrap_or(field_content.len() - 1) + 1;
                        let field_text = &field_content[..end_pos];
                        println!("\n📋 Raw field content:");
                        println!("{}", field_text);
                        
                        // Check variant
                        if field_text.len() > 3 && field_text.chars().nth(3).unwrap_or(' ').is_ascii_alphabetic() {
                            println!("   Detected variant: {}", field_text.chars().nth(3).unwrap());
                        }
                    }
                }
                _ => {
                    println!("💡 Check the message format and try again.");
                }
            }
        }
    }

    Ok(())
}

fn convert_to_json(parsed_message: &ParsedSwiftMessage) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Converting SWIFT Message to JSON:");

    // Convert complete message to JSON
    let full_json = serde_json::to_string_pretty(parsed_message)?;
    println!("\n📄 Complete Message JSON:");
    println!("{full_json}");

    Ok(())
}

fn validate_message(parsed_message: &ParsedSwiftMessage) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n✨ Validating SWIFT Message:");
    println!("{}", "-".repeat(50));

    let validation_errors = parsed_message.validate();

    // Display validation results
    if validation_errors.errors.is_empty() {
        println!("\n✅ Message validation passed!");
    } else {
        println!(
            "\n❌ Message validation failed with {} error(s):",
            validation_errors.errors.len()
        );

        for (index, error) in validation_errors.errors.iter().enumerate() {
            println!("\n   {}. {}", index + 1, error);
        }
    }

    Ok(())
}
