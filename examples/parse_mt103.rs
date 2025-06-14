use swift_mt_message::{SwiftField, SwiftParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The MT103 message from the user
    let raw_mt103 = r#"{1:F01CHASUS33AXXX0000000000}{2:I103DEUTDEFFAXXXN}{3:{113:SEPA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}}{4:
:13C:/123045+0/+0100/-0500
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
:59A:/DE89370400440532013000
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

    println!("🚀 Parsing MT103 Message with SwiftMTMessage Library");
    println!("{}", "=".repeat(60));

    let parsed_message = SwiftParser::parse_auto(raw_mt103)?;
    // Parse the message using the new architecture

    if let Some(parsed_message) = parsed_message.into_mt103() {
        println!("✅ Successfully parsed MT103 message!");
        println!();

        // Display key information
        println!("📋 Message Information:");
        println!("  Message Type: {}", parsed_message.message_type);
        println!("  Basic Header: {:?}", parsed_message.basic_header);
        println!(
            "  Application Header: {:?}",
            parsed_message.application_header
        );
        if let Some(user_header) = &parsed_message.user_header {
            println!("  User Header: {:?}", user_header);
        }
        println!();

        // Display parsed fields
        println!("💼 Parsed Fields:");
        println!(
            "  Transaction Reference: {}",
            parsed_message.fields.field_20.transaction_reference
        );
        println!(
            "  Bank Operation Code: {}",
            parsed_message.fields.field_23b.bank_operation_code
        );
        println!(
            "  Value Date: {}",
            parsed_message.fields.field_32a.value_date
        );
        println!("  Currency: {}", parsed_message.fields.field_32a.currency);
        println!("  Amount: {}", parsed_message.fields.field_32a.amount);
        println!("  Ordering Customer: {:?}", parsed_message.fields.field_50);

        // Handle optional fields
        if let Some(field_52a) = &parsed_message.fields.field_52a {
            println!("  Ordering Institution: {}", field_52a.bic);
        }

        if let Some(field_57a) = &parsed_message.fields.field_57a {
            println!("  Account With Institution: {}", field_57a.bic());
        }

        println!(
            "  Beneficiary Customer Account: {:?}",
            parsed_message.fields.field_59
        );
        println!(
            "  Beneficiary Customer Lines: {:?}",
            parsed_message.fields.field_59
        );

        if let Some(field_70) = &parsed_message.fields.field_70 {
            println!("  Remittance Information: {:?}", field_70.information);
        }

        println!(
            "  Details of Charges: {}",
            parsed_message.fields.field_71a.details_of_charges
        );

        // Display additional optional fields if present
        if let Some(field_33b) = &parsed_message.fields.field_33b {
            println!(
                "  Instructed Amount: {} {}",
                field_33b.currency(),
                field_33b.amount()
            );
        }

        if let Some(field_36) = &parsed_message.fields.field_36 {
            println!("  Exchange Rate: {}", field_36.rate());
        }

        if let Some(field_71f) = &parsed_message.fields.field_71f {
            println!(
                "  Sender's Charges: {} {}",
                field_71f.currency(),
                field_71f.amount()
            );
        }

        if let Some(field_72) = &parsed_message.fields.field_72 {
            println!("  Sender to Receiver Info: {:?}", field_72.information());
        }

        if let Some(field_77b) = &parsed_message.fields.field_77b {
            println!("  Regulatory Reporting: {:?}", field_77b.information());
        }

        println!();

        // Serialize to JSON
        println!("🔄 Converting to JSON...");
        let json_output = serde_json::to_string_pretty(&parsed_message)?;

        println!("📄 Complete JSON Output:");
        println!("{}", json_output);

        // Validate the message
        println!();
        println!("✅ Validation Results:");

        // Required fields
        let required_validations = [
            ("Field 20", parsed_message.fields.field_20.validate()),
            ("Field 23B", parsed_message.fields.field_23b.validate()),
            ("Field 32A", parsed_message.fields.field_32a.validate()),
            ("Field 50", parsed_message.fields.field_50.validate()),
            ("Field 59", parsed_message.fields.field_59.validate()),
            ("Field 71A", parsed_message.fields.field_71a.validate()),
        ];

        for (field_name, validation) in required_validations {
            if validation.is_valid {
                println!("  ✅ {}: Valid", field_name);
            } else {
                println!("  ❌ {}: Invalid", field_name);
                for error in &validation.errors {
                    println!("     - {}", error);
                }
            }
        }

        // Optional fields
        if let Some(field_52a) = &parsed_message.fields.field_52a {
            let validation = field_52a.validate();
            if validation.is_valid {
                println!("  ✅ Field 52A: Valid");
            } else {
                println!("  ❌ Field 52A: Invalid");
                for error in &validation.errors {
                    println!("     - {}", error);
                }
            }
        }

        if let Some(field_57a) = &parsed_message.fields.field_57a {
            let validation = field_57a.validate();
            if validation.is_valid {
                println!("  ✅ Field 57A: Valid");
            } else {
                println!("  ❌ Field 57A: Invalid");
                for error in &validation.errors {
                    println!("     - {}", error);
                }
            }
        }

        if let Some(field_70) = &parsed_message.fields.field_70 {
            let validation = field_70.validate();
            if validation.is_valid {
                println!("  ✅ Field 70: Valid");
            } else {
                println!("  ❌ Field 70: Invalid");
                for error in &validation.errors {
                    println!("     - {}", error);
                }
            }
        }
    } else {
        println!("❌ Failed to parse MT103 message");
    }

    Ok(())
}
