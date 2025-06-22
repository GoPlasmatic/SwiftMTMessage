use swift_mt_message::{SwiftParser, ValidationResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The MT103 message from the user
    let raw_mt103 = r#"{1:F01BANKBEBBAXXX0000000000}
{2:I103BANKDEFFXXXXN}
{3:{113:SEPA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}}
{4:
:20:RET123456
:21:REF987654321
:23B:CRED
:32A:250615EUR123456,78
:50K:/1234567890
John Doe
123 Street
City, Country
:59:/9876543210
Jane Smith
456 Avenue
Another City, Country
:70:RETURN OF FUNDS
:71A:OUR
:72:/RETN/INVALID ACCOUNT
-}"#;

    println!("🚀 MT103 Parsing and Validation Example");
    println!("{}", "=".repeat(60));
    println!();

    // Parse the message using the SwiftMTMessage library
    match SwiftParser::parse_auto(raw_mt103) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed SWIFT message!");
            println!();

            println!("🔍 Message Type: {}", parsed_message.message_type());

            if let Some(mt103_message) = parsed_message.as_mt103() {
                println!("🔍 Message Reject Codes: {}", mt103_message.has_reject_codes());
                println!("🔍 Message Return Code: {}", mt103_message.has_return_codes());
                // Demonstrate validation capabilities using the new wrapper-level validation
                run_comprehensive_validation(mt103_message)?;

                // Show the JSON output
                show_json_output(&parsed_message)?;
            } else {
                println!("❌ Expected MT103 message, got different type");
            }
        }
        Err(parse_error) => {
            println!("❌ Failed to parse SWIFT message: {}", parse_error);
            return Err(parse_error.into());
        }
    }

    Ok(())
}

/// Comprehensive validation demonstration for MT103
fn run_comprehensive_validation(
    mt103_message: &swift_mt_message::SwiftMessage<swift_mt_message::messages::MT103>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 COMPREHENSIVE VALIDATION DEMONSTRATION");
    println!("{}", "=".repeat(60));
    println!();

    // Run message-level business validation using the new wrapper-level method
    println!("🏦 Message-Level Business Rules Validation:");
    println!("--------------------------------------------");
    let business_validation = mt103_message.validate_business_rules();
    print_validation_result("Business Rules", &business_validation);
    println!();

    Ok(())
}

/// Print a formatted validation result
fn print_validation_result(field_name: &str, validation: &ValidationResult) {
    if validation.is_valid {
        println!("  ✅ {}: Valid", field_name);
    } else {
        println!("  ❌ {}: Invalid", field_name);
        for error in &validation.errors {
            println!("     └─ Error: {}", error);
        }
    }

    if !validation.warnings.is_empty() {
        for warning in &validation.warnings {
            println!("     ⚠️  Warning: {}", warning);
        }
    }
}

/// Show JSON output
fn show_json_output(
    parsed_message: &swift_mt_message::ParsedSwiftMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📄 JSON CONVERSION DEMONSTRATION");
    println!("{}", "=".repeat(60));
    println!();

    // Convert to JSON format
    println!("🔄 Converting to JSON format...");
    println!();

    match serde_json::to_string_pretty(&parsed_message) {
        Ok(json_output) => {
            println!("📄 Complete JSON Output:");
            println!("{}", "=".repeat(60));
            println!("{}", json_output);
            println!("{}", "=".repeat(60));
            println!();
            println!("✅ Successfully converted MT103 to JSON format!");
        }
        Err(json_error) => {
            println!("❌ Failed to convert to JSON: {}", json_error);
            return Err(json_error.into());
        }
    }

    Ok(())
}
