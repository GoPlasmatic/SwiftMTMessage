use std::env;
use std::fs;
use swift_mt_message::{ParsedSwiftMessage, SwiftParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the raw message either from file or use default
    let raw_swift_message = if let Some(filename) = env::args().nth(1) {
        // Read from file
        fs::read_to_string(&filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?
    } else {
        // Use default MT104 sample message (23E=CHQB in seq A, so NO 23E in seq B, seq C mandatory)
        r#"{1:F01SENDERBICXXXX0000000000}
{2:I104RECEIVERBICXXXN}
{3:{113:CBPR}{119:RFDD}{121:bca1d755-27f0-4986-96df-e0e4f7f53c10}}
{4:
:20:COLLECTREF002
:23E:RFDD
:21R:CUST/REF/002
:30:250722
:50K:/9999999999
DIRECT DEBIT ORIGINATOR
999 CORPORATE BLVD
CHICAGO, IL 60601
:21:TRANS003
:23E:RFDD
:32B:USD3000,00
:59:/2000000004
ALICE WILLIAMS
789 PINE ROAD
SEATTLE, WA 98101
:70:SUBSCRIPTION FEE JULY
:21:TRANS004
:23E:RFDD
:32B:USD2000,00
:59:/2000000005
CHARLIE BROWN
321 MAPLE AVE
BOSTON, MA 02101
:70:MEMBERSHIP FEE Q3
-}
{5:{CHK:ABCD1234567890}}"#
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
                swift_mt_message::ParseError::InvalidFieldFormat {
                    field_tag,
                    component_name,
                    value,
                    format_spec,
                    ..
                } => {
                    println!("💡 Invalid field format:");
                    println!("   Field: {field_tag}");
                    println!("   Component: {component_name}");
                    println!("   Value: '{value}'");
                    println!("   Expected: {format_spec}");
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

    // Perform validation based on message type
    let validation_errors = match parsed_message {
        ParsedSwiftMessage::MT101(mt101) => {
            println!("🔍 Validating MT101 message...");
            mt101.validate_business_rules()
        }
        ParsedSwiftMessage::MT103(mt103) => {
            println!("🔍 Validating MT103 message...");
            mt103.validate_business_rules()
        }
        ParsedSwiftMessage::MT104(mt104) => {
            println!("🔍 Validating MT104 message...");
            mt104.validate_business_rules()
        }
        ParsedSwiftMessage::MT202(mt202) => {
            println!("🔍 Validating MT202 message...");
            mt202.validate_business_rules()
        }
        ParsedSwiftMessage::MT205(mt205) => {
            println!("🔍 Validating MT205 message...");
            mt205.validate_business_rules()
        }
        ParsedSwiftMessage::MT900(mt900) => {
            println!("🔍 Validating MT900 message...");
            mt900.validate_business_rules()
        }
        _ => {
            println!("❌ Unsupported message type for validation.");
            return Ok(());
        }
    };

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
