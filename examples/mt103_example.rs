//! Comprehensive MT103 Example
//!
//! This example demonstrates the complete MT103 workflow:
//! 1. Parse a SWIFT MT103 message
//! 2. Validate business rules
//! 3. Display field details
//! 4. Convert to JSON and print
//!
//! This combines functionality from all other examples into one simple demonstration.

use swift_mt_message::{
    field_parser::{SwiftField, SwiftMessage},
    json::ToJson,
    mt_models::mt103::MT103,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏦 Comprehensive MT103 Example");
    println!("================================\n");

    // Sample MT103 message with various optional fields
    let mt103_message = r#"{1:F01CHASUS33AXXX0000000000}{2:I103DEUTDEFFAXXXN}{3:{113:ROMF}}{4:
:13C:/123045+0/+0100/-0500
:20:INVOICE12345
:23B:CRED
:23E:INTC/COMPLIANCE CHECK
:26T:A01
:32A:241231USD1234567,89
:33B:EUR1200000,00
:36:1,2345
:50K:/1234567890
ACME CORPORATION LTD
123 BUSINESS STREET
NEW YORK NY 10001 US
:51A:CHASUS33
:59A:/9876543210
DEUTDEFF
:70:/INV/INVOICE12345
/RFB/PAYMENT FOR SERVICES
CONSULTING SERVICES Q4 2024
PROJECT ALPHA BETA
:71A:OUR
:71F:USD25,00
:71G:EUR15,50
:72:/ACC/WEEKLY PAYMENT INSTRUCTION
/INS/PROCESS SAME DAY
PRIORITY PROCESSING REQUIRED
:77B:/ORDERRES/BE//MEILAAN 1, 1000 BRU
REGULATORY COMPLIANCE INFO
TRADE FINANCE RELATED
-}"#;

    // Step 1: Parse the SWIFT message
    println!("📝 Step 1: Parsing SWIFT Message");
    println!("--------------------------------");

    let swift_message = SwiftMessage::parse(mt103_message)?;
    println!("✅ Successfully parsed SWIFT message");
    println!("   Message Type: {}", swift_message.message_type);
    println!("   Number of Fields: {}", swift_message.fields.len());
    println!("   Fields: {:?}", swift_message.field_order);
    println!();

    // Step 2: Convert to MT103 structure
    println!("🏗️  Step 2: Converting to MT103 Structure");
    println!("------------------------------------------");

    let mt103 = MT103::from_swift_message(swift_message.clone())?;
    println!("✅ Successfully converted to MT103");
    println!();

    // Step 3: Display field details
    println!("📋 Step 3: Field Details");
    println!("------------------------");

    // Required fields
    println!("🔴 MANDATORY FIELDS:");
    println!(
        "   20 (Transaction Reference): {}",
        mt103.field_20.transaction_reference
    );
    println!(
        "   23B (Bank Operation Code): {}",
        mt103.field_23b.bank_operation_code
    );
    println!(
        "   32A (Value Date/Currency/Amount): {} {} {:.2}",
        mt103.field_32a.format_date(),
        mt103.field_32a.currency,
        mt103.field_32a.amount
    );
    println!(
        "   50 (Ordering Customer): {}",
        mt103
            .field_50
            .to_swift_string()
            .lines()
            .next()
            .unwrap_or("")
    );

    // Handle Field59 enum properly
    match &mt103.field_59 {
        swift_mt_message::mt_models::fields::beneficiary::Field59::A(field) => {
            println!("   59A (Beneficiary): {}", field.bic);
        }
        swift_mt_message::mt_models::fields::beneficiary::Field59::NoOption(field) => {
            println!(
                "   59 (Beneficiary): {}",
                field
                    .beneficiary_customer
                    .first()
                    .unwrap_or(&"".to_string())
            );
        }
    }

    println!(
        "   71A (Details of Charges): {}",
        mt103.field_71a.details_of_charges
    );
    println!();

    // Optional fields
    println!("🔵 OPTIONAL FIELDS:");
    if let Some(field) = &mt103.field_13c {
        println!("   13C (Time Indication): {}", field);
    }
    if let Some(field) = &mt103.field_23e {
        println!("   23E (Instruction Code): {}", field);
    }
    if let Some(field) = &mt103.field_26t {
        println!("   26T (Transaction Type Code): {}", field);
    }
    if let Some(field) = &mt103.field_33b {
        println!(
            "   33B (Currency/Instructed Amount): {} {:.2}",
            field.currency, field.amount
        );
    }
    if let Some(field) = &mt103.field_36 {
        println!("   36 (Exchange Rate): {:.6}", field.rate());
    }
    if let Some(field) = &mt103.field_51a {
        println!("   51A (Sending Institution): {}", field);
    }
    if let Some(field) = &mt103.field_70 {
        println!(
            "   70 (Remittance Information): {} lines",
            field.information.len()
        );
    }
    if let Some(field) = &mt103.field_71f {
        println!(
            "   71F (Sender's Charges): {} {:.2}",
            field.currency, field.amount
        );
    }
    if let Some(field) = &mt103.field_71g {
        println!(
            "   71G (Receiver's Charges): {} {:.2}",
            field.currency, field.amount
        );
    }
    if let Some(field) = &mt103.field_72 {
        println!(
            "   72 (Sender to Receiver Info): {} lines",
            field.information.len()
        );
    }
    if let Some(field) = &mt103.field_77b {
        println!(
            "   77B (Regulatory Reporting): {} lines",
            field.information.len()
        );
    }
    println!();

    // Step 4: Validate business rules
    println!("✅ Step 4: Business Rules Validation");
    println!("------------------------------------");

    match mt103.validate_business_rules() {
        Ok(report) => {
            println!("📊 Validation Results:");
            println!(
                "   Overall Valid: {}",
                if report.overall_valid {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Total Rules Checked: {}", report.results.len());
            println!("   Failed Rules: {}", report.failure_count());

            if !report.overall_valid {
                println!("\n❌ VALIDATION FAILURES:");
                for failure in report.get_failures() {
                    println!("   • {}: {}", failure.rule_name, failure.message);
                }
            }

            println!("\n📝 All Rule Results:");
            for result in &report.results {
                let status = if result.passed { "✅" } else { "❌" };
                println!("   {} {}", status, result.rule_name);
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }
    println!();

    // Step 5: Convert to JSON and display
    println!("🔄 Step 5: JSON Conversion");
    println!("--------------------------");

    match swift_message.to_json_string() {
        Ok(json_string) => {
            println!("✅ Successfully converted to JSON");

            // Parse and pretty-print the JSON
            let json_value: serde_json::Value = serde_json::from_str(&json_string)?;
            let pretty_json = serde_json::to_string_pretty(&json_value)?;

            println!("\n📄 JSON Output:");
            println!("{}", pretty_json);
        }
        Err(e) => {
            println!("❌ JSON conversion error: {}", e);
        }
    }

    println!("\n🎉 Example completed successfully!");
    println!("   ✅ Parsed MT103 message");
    println!("   ✅ Validated business rules");
    println!("   ✅ Displayed field details");
    println!("   ✅ Converted to JSON");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt103_example() {
        // Ensure the example runs without errors
        assert!(main().is_ok());
    }
}
