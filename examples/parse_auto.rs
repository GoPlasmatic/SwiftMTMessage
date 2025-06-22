use swift_mt_message::{ParsedSwiftMessage, SwiftParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample SWIFT message (MT900 - Confirmation of Debit)
    // This example works with any supported message type (MT103, MT202, MT205, MT900)
    let raw_swift_message = r#"{1:F01BANKBEBBAXXX0000000000}
{2:I900BANKDEFFXXXXN}
{3:{113:CBPR}{121:123e4567-e89b-12d3-a456-426614174000}}
{4:
:20:C11126A1378
:21:MT10345678901
:25:/1234567890123456
:32A:250622USD12500,00
:13D:2506221015+0530
:52A:BANKUS33XXX
:72:/INS/DEUTDEFFXXX
/ACC/US123456789
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

            // Demonstrate JSON field extraction
            demonstrate_json_extraction(&parsed_message)?;
        }
        Err(e) => {
            println!("❌ Failed to parse SWIFT message: {:?}", e);

            // Provide helpful information about the error
            match e {
                swift_mt_message::ParseError::UnsupportedMessageType { message_type } => {
                    println!(
                        "💡 The message type '{}' is not currently supported by parse_auto.",
                        message_type
                    );
                    println!("   Supported types: MT103, MT202, MT205, MT900");
                }
                swift_mt_message::ParseError::InvalidFormat { message } => {
                    println!("💡 The message format is invalid: {}", message);
                }
                swift_mt_message::ParseError::MissingRequiredField { field_tag } => {
                    println!("💡 Missing required field: {}", field_tag);
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
    };

    println!("  Basic Header: {:?}", basic_info.0);
    println!("  Application Header: {:?}", basic_info.1);

    if let Some(user_header) = basic_info.2 {
        println!("  User Header: {:?}", user_header);
    }

    println!("  Number of Fields: {}", basic_info.3);
}

fn convert_to_json(parsed_message: &ParsedSwiftMessage) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Converting SWIFT Message to JSON:");

    // Convert complete message to JSON
    let full_json = serde_json::to_string_pretty(parsed_message)?;
    println!("\n📄 Complete Message JSON:");
    println!("{}", truncate_json(&full_json, 1000));

    // Extract and display just the fields portion
    let json_value: serde_json::Value = serde_json::to_value(parsed_message)?;

    // Get the fields section based on message type
    let fields_json = match parsed_message.message_type() {
        "103" => json_value.get("MT103").and_then(|v| v.get("fields")),
        "202" => json_value.get("MT202").and_then(|v| v.get("fields")),
        "205" => json_value.get("MT205").and_then(|v| v.get("fields")),
        "900" => json_value.get("MT900").and_then(|v| v.get("fields")),
        _ => None,
    };

    if let Some(fields) = fields_json {
        let fields_pretty = serde_json::to_string_pretty(fields)?;
        println!("\n📋 Fields Only JSON:");
        println!("{}", truncate_json(&fields_pretty, 800));
        println!("\n📊 JSON Sizes:");
        println!("  Complete Message: {} bytes", full_json.len());
        println!("  Fields Only: {} bytes", fields_pretty.len());
    }

    Ok(())
}

fn demonstrate_json_extraction(
    parsed_message: &ParsedSwiftMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 Generic JSON Field Extraction:");

    // Convert to generic JSON Value for manipulation
    let json_value: serde_json::Value = serde_json::to_value(parsed_message)?;

    // Extract message type
    if let Some(msg_type) = json_value.get("message_type") {
        println!("  Message Type from JSON: {}", msg_type);
    }

    // Extract fields directly from the JSON structure
    if let Some(fields) = json_value.get("fields") {
        println!("\n  📝 Available Fields:");

        // List all available fields
        if let serde_json::Value::Object(field_map) = fields {
            for (field_name, _) in field_map.iter() {
                println!("    - {}", field_name);
            }
        }

        // Try to extract some common fields that many message types have
        println!("\n  🔍 Common Field Extraction Examples:");

        // Field 20 (Transaction Reference) - common across many message types
        if let Some(field_20) = fields.get("field_20") {
            if let Some(value) = field_20.get("value") {
                println!("    Transaction Reference (20): {}", value);
            }
        }

        // Field 32A (Value Date/Currency/Amount) - common in many message types
        if let Some(field_32a) = fields.get("field_32a") {
            if let Some(currency) = field_32a.get("currency") {
                println!("    Currency (32A): {}", currency);
            }
            if let Some(amount) = field_32a.get("amount") {
                println!("    Amount (32A): {}", amount);
            }
            if let Some(value_date) = field_32a.get("value_date") {
                println!("    Value Date (32A): {}", value_date);
            }
        }

        // Field 21 (Related Reference) - common across message types
        if let Some(field_21) = fields.get("field_21") {
            if let Some(value) = field_21.get("value") {
                println!("    Related Reference (21): {}", value);
            }
        }

        // Field 25 (Account Identification) - common in some message types
        if let Some(field_25) = fields.get("field_25") {
            if let Some(value) = field_25.get("value") {
                println!("    Account Identification (25): {}", value);
            }
        }

        // Field 52A (Ordering Institution) - common in some message types
        if let Some(field_52a) = fields.get("field_52a") {
            if let Some(bic) = field_52a.get("bic") {
                println!("    Ordering Institution (52A): {}", bic);
            }
        }

        // Field 72 (Sender to Receiver Info) - common in some message types
        if let Some(field_72) = fields.get("field_72") {
            if let Some(lines) = field_72.get("lines") {
                if let serde_json::Value::Array(line_array) = lines {
                    println!(
                        "    Sender to Receiver Info (72): {} lines",
                        line_array.len()
                    );
                }
            }
        }
    } else {
        println!("\n  No 'fields' key found in JSON");
        println!(
            "  Available top-level keys: {:?}",
            json_value
                .as_object()
                .map(|obj| obj.keys().collect::<Vec<_>>())
        );
    }

    // Create a generic custom JSON structure
    let custom_json = serde_json::json!({
        "parsed_info": {
            "message_type": format!("MT{}", parsed_message.message_type()),
            "parsing_method": "automatic_detection",
            "json_serializable": true,
            "timestamp": chrono::Utc::now().to_rfc3339()
        },
        "message_summary": {
            "has_user_header": json_value.get("user_header").is_some(),
            "field_count": json_value.get("fields")
                .and_then(|v| v.as_object())
                .map(|obj| obj.len())
                .unwrap_or(0)
        }
    });

    println!("\n📋 Custom Generic JSON Structure:");
    println!("{}", serde_json::to_string_pretty(&custom_json)?);

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
