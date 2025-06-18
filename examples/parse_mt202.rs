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

    // Example 2: MT202.COV - Cover Message for Customer Transfer
    let raw_mt202_cov = r#"{1:F01CHASUS33AXXX0000000000}{2:I202DEUTDEFFAXXXN}{3:{113:COV}{121:f1e2d3c4-b5a6-9087-6543-210987654321}}{4:
:13C:/153045+1/+0100/-0500
:20:COV2024120002
:21:STP2024123456
:32A:241231EUR1375000,00
:52A:CHASUS33XXX
:53A:BNPAFRPPXXX
:58A:DEUTDEFFXXX
:72:/COV/COVER FOR CUSTOMER TRANSFER
/REF/MT103 STP2024123456
INSTITUTIONAL COVER PAYMENT
:50K:/1234567890
GLOBAL TECH CORPORATION
456 INNOVATION DRIVE
SAN FRANCISCO CA 94105 US
:59A:/DE89370400440532013000
DEUTDEFFXXX
:70:/INV/INVOICE-2024-Q4-789
/RFB/SOFTWARE LICENSE PAYMENT
ENTERPRISE SOFTWARE LICENSES
ANNUAL SUBSCRIPTION RENEWAL
:33B:USD1500000,00
-}"#;

    println!("🏦 Parsing MT202 Messages with SwiftMTMessage Library");
    println!("{}", "=".repeat(70));

    // Parse Example 1: Standard MT202
    println!("\n📊 Example 1: Standard MT202 - Institutional Transfer");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse::<MT202>(raw_mt202_standard) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed standard MT202 message!");

            display_message_info(&parsed_message);
            display_institutional_fields(&parsed_message);
            display_routing_analysis(&parsed_message);
            display_timing_analysis(&parsed_message);

            println!("\n🔄 Converting to JSON...");
            let json_output = serde_json::to_string_pretty(&parsed_message)?;
            println!("📄 JSON Output (truncated):");
            println!("{}", truncate_json(&json_output, 500));

            validate_mt202_fields(&parsed_message);
        }
        Err(e) => {
            println!("❌ Failed to parse standard MT202: {:?}", e);
        }
    }

    // Parse Example 2: MT202.COV Cover Message
    println!("\n\n📋 Example 2: MT202.COV - Cover Message");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse::<MT202>(raw_mt202_cov) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed MT202.COV cover message!");

            display_message_info(&parsed_message);
            display_institutional_fields(&parsed_message);
            display_cover_message_fields(&parsed_message);
            display_routing_analysis(&parsed_message);
            display_timing_analysis(&parsed_message);

            println!("\n🔄 Converting to JSON...");
            let json_output = serde_json::to_string_pretty(&parsed_message)?;
            println!("📄 JSON Output (truncated):");
            println!("{}", truncate_json(&json_output, 500));

            validate_mt202_fields(&parsed_message);
        }
        Err(e) => {
            println!("❌ Failed to parse MT202.COV: {:?}", e);
        }
    }

    Ok(())
}

fn display_message_info(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    println!("\n📋 Message Information:");
    println!("  Message Type: {}", parsed_message.message_type);
    println!("  Variant: {}", parsed_message.fields.get_variant());
    println!("  Basic Header: {:?}", parsed_message.basic_header);
    println!(
        "  Application Header: {:?}",
        parsed_message.application_header
    );
    if let Some(user_header) = &parsed_message.user_header {
        println!("  User Header: {:?}", user_header);
    }
}

fn display_institutional_fields(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    println!("\n💼 Core Institutional Transfer Fields:");
    println!(
        "  Transaction Reference: {}",
        parsed_message.fields.transaction_reference()
    );
    println!(
        "  Related Reference: {}",
        parsed_message.fields.related_reference()
    );
    println!(
        "  Value Date: {}",
        parsed_message.fields.field_32a.value_date
    );
    println!(
        "  Currency: {}",
        parsed_message.fields.field_32a.currency_code()
    );
    println!(
        "  Amount: {:.2}",
        parsed_message.fields.field_32a.amount_decimal()
    );
    println!(
        "  Beneficiary Institution: {}",
        parsed_message.fields.beneficiary_institution_bic()
    );

    // Display optional institutional fields
    if let Some(field_52a) = parsed_message.fields.ordering_institution() {
        println!("  Ordering Institution: {}", field_52a.bic());
    }

    if let Some(field_53a) = parsed_message.fields.senders_correspondent() {
        println!("  Sender's Correspondent: {}", field_53a.bic());
    }

    if let Some(field_54a) = parsed_message.fields.receivers_correspondent() {
        println!("  Receiver's Correspondent: {}", field_54a.bic());
    }

    if let Some(field_56a) = parsed_message.fields.intermediary_institution() {
        println!("  Intermediary Institution: {}", field_56a.bic());
    }

    if let Some(field_57a) = parsed_message.fields.account_with_institution() {
        println!("  Account With Institution: {}", field_57a.bic());
    }

    if let Some(field_72) = parsed_message.fields.sender_to_receiver_info() {
        println!("  Processing Instructions:");
        for info in &field_72.information {
            println!("    - {}", info);
        }
    }
}

fn display_cover_message_fields(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    if !parsed_message.fields.is_cover_message_from_fields() {
        return;
    }

    println!("\n🎯 Cover Message Fields (Sequence B):");

    if let Some(field_50a) = parsed_message.fields.ordering_customer() {
        println!("  Ordering Customer: {:?}", field_50a);
    }

    if let Some(field_59a) = parsed_message.fields.beneficiary_customer() {
        println!("  Beneficiary Customer:");
        if let Some(account) = &field_59a.account_number {
            println!("    Account: {}", account);
        }
        println!("    BIC: {}", field_59a.bic());
    }

    if let Some(field_70) = parsed_message.fields.remittance_information() {
        println!("  Remittance Information:");
        for info in &field_70.information {
            println!("    - {}", info);
        }
    }

    if let Some(field_33b) = parsed_message.fields.instructed_amount() {
        println!(
            "  Original Instructed Amount: {} {:.2}",
            field_33b.currency(),
            field_33b.amount()
        );
        if parsed_message.fields.is_cross_currency() {
            println!("  ⚡ Cross-currency transfer detected!");
        }
    }

    // Sequence B institutional fields
    if let Some(field_52a_seq_b) = parsed_message.fields.ordering_institution_seq_b() {
        println!("  Customer Ordering Institution: {}", field_52a_seq_b.bic());
    }

    if let Some(field_56a_seq_b) = parsed_message.fields.intermediary_institution_seq_b() {
        println!("  Customer Intermediary: {}", field_56a_seq_b.bic());
    }

    if let Some(field_57a_seq_b) = parsed_message.fields.account_with_institution_seq_b() {
        println!(
            "  Customer Account With Institution: {}",
            field_57a_seq_b.bic()
        );
    }

    if let Some(field_72_seq_b) = parsed_message.fields.sender_to_receiver_info_seq_b() {
        println!("  Customer Processing Instructions:");
        for info in &field_72_seq_b.information {
            println!("    - {}", info);
        }
    }
}

fn display_routing_analysis(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    println!("\n🛣️  Routing Analysis:");

    // Institutional routing chain
    let institutional_chain = parsed_message.fields.get_routing_chain();
    println!("  Institutional Transfer Chain:");
    for (index, (role, bic)) in institutional_chain.iter().enumerate() {
        println!("    {}. {}: {}", index + 1, role, bic);
    }

    // Customer routing chain (if cover message)
    if parsed_message.fields.is_cover_message_from_fields() {
        let customer_chain = parsed_message.fields.get_customer_routing_chain();
        if !customer_chain.is_empty() {
            println!("  Customer Transaction Chain:");
            for (index, (role, bic)) in customer_chain.iter().enumerate() {
                println!("    {}. {}: {}", index + 1, role, bic);
            }
        }
    }
}

fn display_timing_analysis(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    if let Some(time_indications) = parsed_message.fields.time_indications() {
        println!("\n⏰ Timing Requirements:");

        for (index, field_13c) in time_indications.iter().enumerate() {
            println!("  {}. {}", index + 1, field_13c.description());
        }

        if parsed_message.fields.has_cls_timing() {
            println!("  🔄 CLS (Continuous Linked Settlement) timing detected");
        }

        if parsed_message.fields.has_target_timing() {
            println!("  🎯 TARGET system timing detected");
        }
    }
}

fn validate_mt202_fields(parsed_message: &swift_mt_message::SwiftMessage<MT202>) {
    println!("\n✅ Validation Results:");

    // Required fields validation
    let required_validations = [
        ("Field 20", parsed_message.fields.field_20.validate()),
        ("Field 21", parsed_message.fields.field_21.validate()),
        ("Field 32A", parsed_message.fields.field_32a.validate()),
        ("Field 58A", parsed_message.fields.field_58a.validate()),
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

    // Optional fields validation
    if let Some(field_52a) = &parsed_message.fields.field_52a {
        let validation = field_52a.validate();
        print_field_validation("Field 52A", &validation);
    }

    if let Some(field_53a) = &parsed_message.fields.field_53a {
        let validation = field_53a.validate();
        print_field_validation("Field 53A", &validation);
    }

    if let Some(field_54a) = &parsed_message.fields.field_54a {
        let validation = field_54a.validate();
        print_field_validation("Field 54A", &validation);
    }

    if let Some(field_56a) = &parsed_message.fields.field_56a {
        let validation = field_56a.validate();
        print_field_validation("Field 56A", &validation);
    }

    if let Some(field_57a) = &parsed_message.fields.field_57a {
        let validation = field_57a.validate();
        print_field_validation("Field 57A", &validation);
    }

    if let Some(field_72) = &parsed_message.fields.field_72 {
        let validation = field_72.validate();
        print_field_validation("Field 72", &validation);
    }

    // Cover message specific validations
    if parsed_message.fields.is_cover_message_from_fields() {
        if let Some(field_50a) = &parsed_message.fields.field_50a {
            let validation = field_50a.validate();
            print_field_validation("Field 50A (Cover)", &validation);
        }

        if let Some(field_59a) = &parsed_message.fields.field_59a {
            let validation = field_59a.validate();
            print_field_validation("Field 59A (Cover)", &validation);
        }

        if let Some(field_70) = &parsed_message.fields.field_70 {
            let validation = field_70.validate();
            print_field_validation("Field 70 (Cover)", &validation);
        }

        if let Some(field_33b) = &parsed_message.fields.field_33b {
            let validation = field_33b.validate();
            print_field_validation("Field 33B (Cover)", &validation);
        }
    }

    // Structural validation
    if parsed_message.fields.validate_structure() {
        println!("  ✅ Message Structure: Valid");
    } else {
        println!("  ❌ Message Structure: Invalid");
    }

    if parsed_message.fields.validate_cover_message() {
        println!("  ✅ Cover Message Requirements: Valid");
    } else {
        println!("  ❌ Cover Message Requirements: Invalid");
    }
}

fn print_field_validation(field_name: &str, validation: &swift_mt_message::ValidationResult) {
    if validation.is_valid {
        println!("  ✅ {}: Valid", field_name);
    } else {
        println!("  ❌ {}: Invalid", field_name);
        for error in &validation.errors {
            println!("     - {}", error);
        }
    }

    for warning in &validation.warnings {
        println!("  ⚠️  {}: Warning - {}", field_name, warning);
    }
}

fn truncate_json(json: &str, max_length: usize) -> String {
    if json.len() <= max_length {
        json.to_string()
    } else {
        format!(
            "{}...\n  (JSON output truncated - full output available in actual usage)",
            &json[..max_length]
        )
    }
}
