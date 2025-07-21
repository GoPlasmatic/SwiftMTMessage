use swift_mt_message::{ParsedSwiftMessage, SwiftParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample SWIFT message (MT900 - Confirmation of Debit)
    // This example works with any supported message type (MT103, MT202, MT205, MT900)
    let raw_swift_message = r#"{1:F01CHASUS33AXXX0000000000}{2:I103DEUTDEFFAXXXN}{3:{113:SEPA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}}{4:
:13C:/SNDTIME/1200+0100
:20:STP2024123456
:23B:CRED
:23E:INTC/COMPLIANCE
:26T:A01
:32A:241231USD1500000,00
:33B:EUR1375000,00
:36:1,0909
:50K:/1234567890
GLOBAL TECH CORPORATION
456 INNOVATION DRIVE
SAN FRANCISCO CA 94105 US
:52A:CHASUS33
:53A:BNPAFRPP
:54A:DEUTDEFF
:57A:DEUTDEFF
:59:/DE89370400440532013000
DEUTDEFF
:70:/INV/INVOICE-2024-Q4-789
/RFB/SOFTWARE LICENSE PAYMENT
ENTERPRISE SOFTWARE LICENSES
ANNUAL SUBSCRIPTION RENEWAL
:71A:SHA
:71F:USD50,00
:72:/ACC/STANDARD PROCESSING
/INS/COMPLY WITH LOCAL REGS
AUTOMATED STP PROCESSING
:77B:/ORDERRES/DE//REGULATORY INFO
SOFTWARE LICENSE COMPLIANCE
TRADE RELATED TRANSACTION
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
