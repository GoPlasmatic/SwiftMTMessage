//! Example demonstrating sample generation for SWIFT MT messages and fields

use swift_mt_message::{
    SwiftField, SwiftMessageBody,
    fields::{Field20, Field70, field32a::Field32A, field50::Field50},
    messages::{MT103, MT202, MT940},
    sample::{FieldConfig, LengthPreference, MessageConfig, MessageScenario, ValueRange},
};

fn main() {
    println!("=== SWIFT MT Message Sample Generation Examples ===\n");

    // Example 1: Generate sample fields
    println!("1. Field Sample Generation:");
    println!("----------------------------");

    // Simple field
    let field20 = Field20::sample();
    println!("Field20 (Reference): {}", field20.to_swift_string());

    // Currency and amount field
    let field32a = Field32A::sample();
    println!(
        "Field32A (Value Date/Currency/Amount): {}",
        field32a.to_swift_string()
    );

    // Complex enum field
    let field50 = Field50::sample();
    println!("Field50 (Ordering Customer): {}", field50.to_swift_string());

    // Multi-line field
    let field70 = Field70::sample();
    println!(
        "Field70 (Remittance Information): {}",
        field70.to_swift_string()
    );

    println!();

    // Example 2: Generate minimal MT103 message
    println!("2. Minimal MT103 Message (Required Fields Only):");
    println!("------------------------------------------------");
    let mt103_minimal = MT103::sample();
    println!("Transaction Reference: {}", mt103_minimal.field_20.value);
    println!("Bank Operation Code: {}", mt103_minimal.field_23b.value);
    println!("Value Date: {:?}", mt103_minimal.field_32a.value_date);
    println!("Currency: {}", mt103_minimal.field_32a.currency);
    println!("Amount: {}", mt103_minimal.field_32a.amount);

    // Convert to JSON to see structure
    if let Ok(json) = serde_json::to_string_pretty(&mt103_minimal) {
        println!("\nMT103 Minimal as JSON:");
        println!("{json}");
    }

    println!();

    // Example 3: Generate full MT103 message
    println!("3. Full MT103 Message (All Fields):");
    println!("-----------------------------------");
    let mt103_full = MT103::sample_full();

    // Check which optional fields were populated
    println!("Optional fields populated:");
    if mt103_full.field_72.is_some() {
        println!("  - Field 72 (Sender to Receiver Information)");
    }
    if mt103_full.field_77b.is_some() {
        println!("  - Field 77B (Regulatory Reporting)");
    }
    if mt103_full.field_71g.is_some() {
        println!("  - Field 71G (Charges Information)");
    }

    println!();

    // Example 4: Generate with configuration
    println!("4. Configured Message Generation:");
    println!("---------------------------------");

    // Create configuration
    let mut field_configs = std::collections::HashMap::new();
    field_configs.insert(
        "32A".to_string(),
        FieldConfig {
            value_range: Some(ValueRange::Amount {
                min: 10000.0,
                max: 50000.0,
                currency: Some("EUR".to_string()),
            }),
            ..Default::default()
        },
    );

    let config = MessageConfig {
        include_optional: true,
        field_configs,
        scenario: Some(MessageScenario::StpCompliant),
    };

    let mt103_configured = MT103::sample_with_config(&config);
    println!("Configured MT103:");
    println!(
        "  Amount: {} {}",
        mt103_configured.field_32a.currency, mt103_configured.field_32a.amount
    );

    println!();

    // Example 5: Generate other message types
    println!("5. Other Message Types:");
    println!("-----------------------");

    // MT202 Cover Payment
    let mt202 = MT202::sample();
    println!("MT202 Transaction Reference: {}", mt202.field_20.value);
    println!("MT202 Related Reference: {}", mt202.field_21.value);

    // MT940 Customer Statement
    let mt940 = MT940::sample();
    println!(
        "MT940 Statement Number: {}",
        mt940.field_28c.statement_number
    );
    println!("MT940 Account: {}", mt940.field_25.value);

    println!();

    // Example 6: Field-level configuration
    println!("6. Field-Level Sample Configuration:");
    println!("------------------------------------");

    let field_config = FieldConfig {
        length_preference: Some(LengthPreference::Exact(16)),
        fixed_values: Some(vec![
            "REF2023120001".to_string(),
            "REF2023120002".to_string(),
        ]),
        ..Default::default()
    };

    // Note: sample_with_config would choose from fixed_values if implemented
    let configured_field20 = Field20::sample_with_config(&field_config);
    println!(
        "Configured Field20: {}",
        configured_field20.to_swift_string()
    );

    println!();

    // Example 7: Validate generated samples
    println!("7. Validation of Generated Samples:");
    println!("-----------------------------------");

    // Validate individual fields
    let field20_validation = mt103_minimal.field_20.validate();
    println!(
        "Field20 validation: {}",
        if field20_validation.is_valid {
            "VALID"
        } else {
            "INVALID"
        }
    );

    let field50_validation = field50.validate();
    println!(
        "Field50 validation: {}",
        if field50_validation.is_valid {
            "VALID"
        } else {
            "INVALID"
        }
    );

    println!("\n=== Sample Generation Complete ===");
}
