use swift_mt_message::messages::mt202::MT202;
use swift_mt_message::{SwiftField, SwiftParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Standard MT202 - Institutional Transfer
    let raw_mt202_standard = r#"{1:F01CHASUS33AXXX0000000000}{2:I202DEUTDEFFAXXXN}{3:{113:RTGS}{121:a1b2c3d4-e5f6-7890-1234-567890abcdef}}{4:
:13C:/153045+1/+0100/-0500
:13C:/090000+0/+0000/+0900
:20:FIT2024120001
:21:REL2024119999
:32A:241231USD5000000,00
:52A:CHASUS33XXX
:53A:BNPAFRPPXXX
:54A:DEUTDEFFXXX
:56A:BARCGB22XXX
:57A:UBSWCHZHXXX
:58A:DEUTDEFFXXX
:72:/INT/NOSTRO ACCOUNT FUNDING
/REF/QUARTERLY SETTLEMENT
CORRESPONDENT BANKING OPERATION
-}"#;

    // Example 2: Simple MT202 - Basic Transfer
    let raw_mt202_simple = r#"{1:F01CHASUS33AXXX0000000000}{2:I202DEUTDEFFAXXXN}{4:
:20:FIT2024120002
:21:REL2024119998
:32A:241231EUR750000,00
:58A:DEUTDEFFXXX
-}"#;

    println!("🏦 MT202 Parser - Focus on JSON Conversion");
    println!("{}", "=".repeat(60));

    // Parse and convert Example 1: Full MT202
    println!("\n📊 Example 1: Full MT202 with Optional Fields");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse::<MT202>(raw_mt202_standard) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed MT202 message!");

            // Display basic information
            display_basic_info(&parsed_message);

            // Convert to JSON and display
            convert_to_json(&parsed_message, "Full MT202")?;

            // Show field validation
            validate_fields(&parsed_message);
        }
        Err(e) => {
            println!("❌ Failed to parse MT202: {:?}", e);
        }
    }

    // Parse and convert Example 2: Simple MT202
    println!("\n\n📋 Example 2: Simple MT202 (Minimal Fields)");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse::<MT202>(raw_mt202_simple) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed simple MT202 message!");

            // Display basic information
            display_basic_info(&parsed_message);

            // Convert to JSON and display
            convert_to_json(&parsed_message, "Simple MT202")?;

            // Show field validation
            validate_fields(&parsed_message);
        }
        Err(e) => {
            println!("❌ Failed to parse simple MT202: {:?}", e);
        }
    }

    // Demonstrate JSON field extraction
    println!("\n\n🔍 JSON Field Extraction Examples");
    println!("{}", "-".repeat(50));
    demonstrate_json_extraction()?;

    Ok(())
}

fn display_basic_info(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    println!("\n📋 Message Information:");
    println!("  Message Type: {}", parsed_message.message_type);
    println!("  Basic Header: {:?}", parsed_message.basic_header);
    println!(
        "  Application Header: {:?}",
        parsed_message.application_header
    );

    if let Some(user_header) = &parsed_message.user_header {
        println!("  User Header: {:?}", user_header);
    }

    println!("\n💼 Core Fields:");
    println!(
        "  Transaction Reference (20): {}",
        parsed_message.fields.field_20.value
    );
    println!(
        "  Related Reference (21): {}",
        parsed_message.fields.field_21.value
    );
    println!(
        "  Value Date: {}",
        parsed_message.fields.field_32a.value_date
    );
    println!("  Currency: {}", parsed_message.fields.field_32a.currency);
    println!("  Amount: {:.2}", parsed_message.fields.field_32a.amount);
    println!(
        "  Beneficiary Institution (58A): {}",
        parsed_message.fields.field_58a.bic
    );

    // Display optional fields if present
    if let Some(time_indications) = &parsed_message.fields.field_13c {
        println!(
            "  Time Indications (13C): {} entries",
            time_indications.len()
        );
        for (i, time_ind) in time_indications.iter().enumerate() {
            println!(
                "    {}: {} {} {}",
                i + 1,
                time_ind.time_code,
                time_ind.time,
                time_ind.utc_offset
            );
        }
    }

    if let Some(ordering_inst) = &parsed_message.fields.field_52a {
        println!("  Ordering Institution (52A): {}", ordering_inst.bic);
    }

    if let Some(sender_corr) = &parsed_message.fields.field_53a {
        println!("  Sender's Correspondent (53A): {}", sender_corr.bic);
    }

    if let Some(receiver_corr) = &parsed_message.fields.field_54a {
        println!("  Receiver's Correspondent (54A): {}", receiver_corr.bic);
    }

    if let Some(intermediary) = &parsed_message.fields.field_56a {
        println!("  Intermediary Institution (56A): {}", intermediary.bic);
    }

    if let Some(account_with) = &parsed_message.fields.field_57a {
        println!("  Account With Institution (57A): {}", account_with.bic);
    }

    if let Some(sender_info) = &parsed_message.fields.field_72 {
        println!(
            "  Sender to Receiver Info (72): {} lines",
            sender_info.lines.len()
        );
        for (i, line) in sender_info.lines.iter().enumerate() {
            println!("    {}: {}", i + 1, line);
        }
    }
}

fn convert_to_json(
    parsed_message: &swift_mt_message::SwiftMessage<MT202>,
    title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Converting {} to JSON:", title);

    // Convert complete message to JSON
    let full_json = serde_json::to_string_pretty(parsed_message)?;
    println!("\n📄 Complete Message JSON:");
    println!("{}", truncate_json(&full_json, 800));

    // Convert just the fields to JSON
    let fields_json = serde_json::to_string_pretty(&parsed_message.fields)?;
    println!("\n📋 Fields Only JSON:");
    println!("{}", truncate_json(&fields_json, 600));

    // Convert specific field to JSON (Field 32A)
    let field_32a_json = serde_json::to_string_pretty(&parsed_message.fields.field_32a)?;
    println!("\n💰 Field 32A (Value Date/Currency/Amount) JSON:");
    println!("{}", field_32a_json);

    // Show JSON sizes
    println!("\n📊 JSON Sizes:");
    println!("  Complete Message: {} bytes", full_json.len());
    println!("  Fields Only: {} bytes", fields_json.len());
    println!("  Field 32A Only: {} bytes", field_32a_json.len());

    Ok(())
}

fn validate_fields(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    println!("\n✅ Field Validation:");

    // Validate required fields
    let field_20_validation = parsed_message.fields.field_20.validate();
    print_validation_result("Field 20 (Transaction Reference)", &field_20_validation);

    let field_21_validation = parsed_message.fields.field_21.validate();
    print_validation_result("Field 21 (Related Reference)", &field_21_validation);

    let field_32a_validation = parsed_message.fields.field_32a.validate();
    print_validation_result(
        "Field 32A (Value Date/Currency/Amount)",
        &field_32a_validation,
    );

    let field_58a_validation = parsed_message.fields.field_58a.validate();
    print_validation_result("Field 58A (Beneficiary Institution)", &field_58a_validation);

    // Validate optional fields if present
    if let Some(time_indications) = &parsed_message.fields.field_13c {
        for (i, time_ind) in time_indications.iter().enumerate() {
            let validation = time_ind.validate();
            print_validation_result(&format!("Field 13C[{}] (Time Indication)", i), &validation);
        }
    }

    if let Some(field_52a) = &parsed_message.fields.field_52a {
        let validation = field_52a.validate();
        print_validation_result("Field 52A (Ordering Institution)", &validation);
    }

    if let Some(field_72) = &parsed_message.fields.field_72 {
        let validation = field_72.validate();
        print_validation_result("Field 72 (Sender to Receiver Info)", &validation);
    }
}

fn print_validation_result(field_name: &str, validation: &swift_mt_message::ValidationResult) {
    if validation.is_valid {
        println!("  ✅ {}: Valid", field_name);
    } else {
        println!("  ❌ {}: Invalid", field_name);
        for error in &validation.errors {
            println!("     - {}", error);
        }
    }

    for warning in &validation.warnings {
        println!("  ⚠️  {}: {}", field_name, warning);
    }
}

fn demonstrate_json_extraction() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sample MT202 for demonstration
    let sample_raw = r#"{1:F01CHASUS33AXXX0000000000}{2:I202DEUTDEFFAXXXN}{4:
:20:DEMO123456
:21:REF987654
:32A:241231USD1000000,00
:52A:CHASUS33XXX
:58A:DEUTDEFFXXX
:72:/INT/SAMPLE TRANSFER
/REF/DEMONSTRATION
-}"#;

    match SwiftParser::parse::<MT202>(sample_raw) {
        Ok(parsed_message) => {
            // Example 1: Extract specific values from JSON
            let json_value: serde_json::Value = serde_json::to_value(&parsed_message.fields)?;

            println!("🔍 Extracting specific values from JSON:");
            if let Some(amount) = json_value["field_32a"]["amount"].as_f64() {
                println!("  Amount from JSON: {:.2}", amount);
            }

            if let Some(currency) = json_value["field_32a"]["currency"].as_str() {
                println!("  Currency from JSON: {}", currency);
            }

            if let Some(bic) = json_value["field_58a"]["bic"].as_str() {
                println!("  Beneficiary BIC from JSON: {}", bic);
            }

            // Example 2: Convert JSON back to struct
            println!("\n🔄 Round-trip JSON conversion:");
            let fields_json = serde_json::to_string(&parsed_message.fields)?;
            let deserialized_fields: MT202 = serde_json::from_str(&fields_json)?;

            println!(
                "  Original amount: {:.2}",
                parsed_message.fields.field_32a.amount
            );
            println!(
                "  Deserialized amount: {:.2}",
                deserialized_fields.field_32a.amount
            );
            println!(
                "  Round-trip successful: {}",
                parsed_message.fields.field_32a.amount == deserialized_fields.field_32a.amount
            );

            // Example 3: Create custom JSON structure
            let custom_json = serde_json::json!({
                "transaction": {
                    "reference": parsed_message.fields.field_20.value,
                    "related_reference": parsed_message.fields.field_21.value,
                    "value_date": parsed_message.fields.field_32a.value_date,
                    "currency": parsed_message.fields.field_32a.currency,
                    "amount": parsed_message.fields.field_32a.amount
                },
                "institutions": {
                    "beneficiary": parsed_message.fields.field_58a.bic,
                    "ordering": parsed_message.fields.field_52a.as_ref().map(|f| &f.bic)
                }
            });

            println!("\n📋 Custom JSON structure:");
            println!("{}", serde_json::to_string_pretty(&custom_json)?);
        }
        Err(e) => {
            println!("❌ Failed to parse demonstration message: {:?}", e);
        }
    }

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
