use swift_mt_message::{ParsedSwiftMessage, SwiftParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample SWIFT message (MT900 - Confirmation of Debit)
    // This example works with any supported message type (MT103, MT202, MT205, MT900)
    let raw_swift_message = r#"{1:F01BANKUS33XXXAXXX0000000000}
{2:I103BANKDEFFXXXXXXXN}
{3:{121:550e8400-e29b-41d4-a716-446655440000}}
{4:
:20:INSTR123456
:23B:CRED
:32A:250627EUR1000
:33B:USD1100
:36:1,1
:50:/ACC-US-123456789
Jane Smith
Apartment 4B
:52A:BANKUS33XXX/DBTR-AGENT-ACC-123
:56A:INTRMGB2LXX/INTER-ACC-123
:57A:BANKDEFFXXX/CDTR-AGENT-ACC-456
:59:/DE89370400440532013000
John Doe
Building C
:70:/ROC/550e8400-e29b-41d4-a716-446655440000
Payment for invoice INV-2025-001234
Thank you for your business
Reference: CONTRACT-2025-789
:71A:SHA
:71F:EUR5
:71G:EUR10
:72:Please process with high priority
Additional instructions for next agent
:77B:Export payment for goods
Additional regulatory information
:23E:INTC
-}"#;

    println!("🔍 SWIFT Message Auto-Parser with JSON Conversion");
    println!("{}", "=".repeat(60));

    // Parse with automatic message type detection
    println!("\n📊 Parsing with automatic message type detection...");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse_auto(raw_swift_message) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed SWIFT message!");

            // Display detected message type
            println!(
                "🏷️  Detected Message Type: MT{}",
                parsed_message.message_type()
            );

            // Display basic message information (generic for all message types)
            display_message_info(&parsed_message);

            // Convert to JSON and display
            convert_to_json(&parsed_message)?;
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
                swift_mt_message::ParseError::InvalidFormat { message } => {
                    println!("💡 The message format is invalid: {message}");
                }
                swift_mt_message::ParseError::MissingRequiredField { field_tag } => {
                    println!("💡 Missing required field: {field_tag}");
                }
                _ => {
                    println!("💡 Check the message format and try again.");
                }
            }
        }
    }

    Ok(())
}

fn display_message_info(parsed_message: &ParsedSwiftMessage) {
    println!("\n📋 Message Information:");
    println!("  Message Type: MT{}", parsed_message.message_type());

    // Extract basic header info (available for all message types)
    let basic_info = match parsed_message {
        ParsedSwiftMessage::MT103(msg) => (
            &msg.basic_header,
            &msg.application_header,
            msg.user_header.as_ref(),
            msg.field_order.len(),
        ),
        ParsedSwiftMessage::MT104(msg) => (
            &msg.basic_header,
            &msg.application_header,
            msg.user_header.as_ref(),
            msg.field_order.len(),
        ),
        ParsedSwiftMessage::MT107(msg) => (
            &msg.basic_header,
            &msg.application_header,
            msg.user_header.as_ref(),
            msg.field_order.len(),
        ),
        ParsedSwiftMessage::MT202(msg) => (
            &msg.basic_header,
            &msg.application_header,
            msg.user_header.as_ref(),
            msg.field_order.len(),
        ),
        ParsedSwiftMessage::MT205(msg) => (
            &msg.basic_header,
            &msg.application_header,
            msg.user_header.as_ref(),
            msg.field_order.len(),
        ),
        ParsedSwiftMessage::MT900(msg) => (
            &msg.basic_header,
            &msg.application_header,
            msg.user_header.as_ref(),
            msg.field_order.len(),
        ),
        ParsedSwiftMessage::MT192(msg) => (
            &msg.basic_header,
            &msg.application_header,
            msg.user_header.as_ref(),
            msg.field_order.len(),
        ),
        _ => {
            println!("Unknown message type");
            return;
        }
    };

    println!("  Basic Header: {:?}", basic_info.0);
    println!("  Application Header: {:?}", basic_info.1);

    if let Some(user_header) = basic_info.2 {
        println!("  User Header: {user_header:?}");
    }

    println!("  Number of Fields: {}", basic_info.3);
}

fn convert_to_json(parsed_message: &ParsedSwiftMessage) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Converting SWIFT Message to JSON:");

    // Convert complete message to JSON
    let full_json = serde_json::to_string_pretty(parsed_message)?;
    println!("\n📄 Complete Message JSON:");
    println!("{}", truncate_json(&full_json, 1000));

    Ok(())
}

fn truncate_json(json: &str, max_length: usize) -> String {
    if json.len() <= max_length {
        json.to_string()
    } else {
        let truncated = &json[..max_length];
        // Try to cut at a complete line
        if let Some(last_newline) = truncated.rfind('\n') {
            format!(
                "{}...\n  (truncated - {} more bytes)",
                &json[..last_newline],
                json.len() - last_newline
            )
        } else {
            format!(
                "{}...\n  (truncated - {} more bytes)",
                truncated,
                json.len() - max_length
            )
        }
    }
}
