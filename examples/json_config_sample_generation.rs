//! Simple example demonstrating config-based SWIFT MT message sample generation

use swift_mt_message::{
    SwiftMessage,
    messages::MT103,
    sample::{MessageConfig, MessageScenario},
};

fn main() {
    println!("=== Config-Based SWIFT Message Sample Generation ===\n");

    // 1. Generate samples using different scenarios
    generate_scenario_samples();

    // 2. Generate from JSON configuration
    generate_from_json();

    println!("\n✅ Sample generation completed!");
}

fn generate_scenario_samples() {
    println!("1. Scenario-Based Generation:");
    println!("=============================\n");

    // Generate MT103 samples for each scenario
    let scenarios = vec![
        ("Standard", MessageScenario::Standard),
        ("STP Compliant", MessageScenario::StpCompliant),
        ("Cover Payment", MessageScenario::CoverPayment),
        ("Minimal", MessageScenario::Minimal),
        ("Full", MessageScenario::Full),
    ];

    for (name, scenario) in scenarios {
        let config = MessageConfig {
            scenario: Some(scenario.clone()),
            include_optional: matches!(scenario, MessageScenario::Full),
            ..Default::default()
        };

        let message: SwiftMessage<MT103> = SwiftMessage::sample_with_config(&config);

        println!("📄 {name} MT103:");
        println!(
            "  UETR: {}",
            message
                .user_header
                .as_ref()
                .and_then(|h| h.unique_end_to_end_reference.as_ref())
                .unwrap_or(&"None".to_string())
        );
        println!(
            "  Validation: {}",
            message
                .user_header
                .as_ref()
                .and_then(|h| h.validation_flag.as_ref())
                .map(|f| f.as_str())
                .unwrap_or("None")
        );
        println!(
            "  Amount: {} {}\n",
            message.fields.field_32a.currency, message.fields.field_32a.amount
        );
    }
}

fn generate_from_json() {
    println!("\n2. JSON Configuration:");
    println!("======================\n");

    // Example configurations
    let configs = vec![
        (
            "STP Payment",
            r#"{"scenario": "StpCompliant", "include_optional": true, "field_configs": {}}"#,
        ),
        (
            "Cover Payment",
            r#"{"scenario": "CoverPayment", "include_optional": false, "field_configs": {}}"#,
        ),
    ];

    for (name, json) in configs {
        let config: MessageConfig = serde_json::from_str(json).unwrap();
        let mt103: SwiftMessage<MT103> = SwiftMessage::sample_with_config(&config);

        println!("📋 {name} (from JSON):");
        println!("  Config: {json}");
        println!(
            "  Generated UETR: {}",
            mt103
                .user_header
                .as_ref()
                .and_then(|h| h.unique_end_to_end_reference.as_ref())
                .unwrap_or(&"None".to_string())
        );
        println!(
            "  Validation Flag: {}\n",
            mt103
                .user_header
                .as_ref()
                .and_then(|h| h.validation_flag.as_ref())
                .map(|f| f.as_str())
                .unwrap_or("None")
        );
    }

    // Show complete message example
    println!("📜 Complete SWIFT Message Example:");
    println!("==================================");
    let config = MessageConfig {
        scenario: Some(MessageScenario::StpCompliant),
        include_optional: false,
        ..Default::default()
    };

    let message: SwiftMessage<MT103> = SwiftMessage::sample_with_config(&config);
    println!("{}", message.to_mt_message());
}
