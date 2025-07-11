//! Example demonstrating JSON-based configuration for SWIFT MT message sample generation

use std::collections::HashMap;
use swift_mt_message::{
    SwiftField, SwiftMessageBody,
    fields::{Field20, field32a::Field32A},
    messages::{MT103, MT202},
    sample::{FieldConfig, MessageConfig},
};

fn main() {
    println!("=== JSON Configuration-Based Sample Generation ===\n");

    // Example 1: Simple field configuration from JSON
    example_1_field_config_from_json();

    // Example 2: Message configuration from JSON
    example_2_message_config_from_json();

    // Example 3: Multiple scenarios from JSON
    example_3_multiple_scenarios_from_json();

    // Example 4: Complex configuration with validation
    example_4_complex_configuration();

    println!("\n=== JSON Configuration Examples Complete ===");
}

fn example_1_field_config_from_json() {
    println!("1. Field Configuration from JSON:");
    println!("----------------------------------");

    // JSON configuration for Field20 (Transaction Reference)
    let field20_config_json = r#"
    {
        "length_preference": { "Exact": 16 },
        "fixed_values": ["REF2024010001", "REF2024010002", "REF2024010003"],
        "pattern": "^REF[0-9]{10}$"
    }"#;

    // Parse JSON configuration
    let field20_config: FieldConfig =
        serde_json::from_str(field20_config_json).expect("Failed to parse Field20 configuration");

    println!("Field20 JSON Config:");
    println!("{}", serde_json::to_string_pretty(&field20_config).unwrap());

    // Generate samples using this configuration
    println!("\nGenerated Field20 samples:");
    for i in 1..=3 {
        let sample = Field20::sample_with_config(&field20_config);
        println!("  Sample {}: {}", i, sample.to_swift_string());
    }

    // JSON configuration for Field32A (Value Date/Currency/Amount)
    let field32a_config_json = r#"
    {
        "value_range": {
            "Amount": {
                "min": 5000.0,
                "max": 100000.0,
                "currency": "USD"
            }
        }
    }"#;

    let field32a_config: FieldConfig =
        serde_json::from_str(field32a_config_json).expect("Failed to parse Field32A configuration");

    println!("\nField32A JSON Config:");
    println!(
        "{}",
        serde_json::to_string_pretty(&field32a_config).unwrap()
    );

    println!("\nGenerated Field32A sample:");
    let field32a_sample = Field32A::sample_with_config(&field32a_config);
    println!("  {}", field32a_sample.to_swift_string());

    println!();
}

fn example_2_message_config_from_json() {
    println!("2. Message Configuration from JSON:");
    println!("-----------------------------------");

    let message_config_json = r#"
    {
        "include_optional": true,
        "scenario": "StpCompliant",
        "field_configs": {
            "20": {
                "length_preference": { "Exact": 16 },
                "pattern": "^STP[0-9]{13}$"
            },
            "32A": {
                "value_range": {
                    "Amount": {
                        "min": 10000.0,
                        "max": 50000.0,
                        "currency": "EUR"
                    }
                }
            },
            "70": {
                "fixed_values": ["SALARY PAYMENT", "INVOICE PAYMENT", "CONSULTING FEE"]
            }
        }
    }"#;

    // Parse JSON configuration
    let message_config: MessageConfig =
        serde_json::from_str(message_config_json).expect("Failed to parse message configuration");

    println!("Message JSON Config:");
    println!("{}", serde_json::to_string_pretty(&message_config).unwrap());

    // Generate MT103 with this configuration
    println!("\nGenerated MT103 with JSON config:");
    let mt103_sample = MT103::sample_with_config(&message_config);

    println!("  Transaction Reference: {}", mt103_sample.field_20.value);
    println!(
        "  Currency/Amount: {} {}",
        mt103_sample.field_32a.currency, mt103_sample.field_32a.amount
    );
    println!("  Value Date: {:?}", mt103_sample.field_32a.value_date);

    // Show JSON representation
    if let Ok(json) = serde_json::to_string_pretty(&mt103_sample) {
        println!("\nMT103 Sample as JSON (truncated):");
        let lines: Vec<&str> = json.lines().take(20).collect();
        for line in lines {
            println!("  {line}");
        }
        if json.lines().count() > 20 {
            println!("  ... (truncated for readability)");
        }
    }

    println!();
}

fn example_3_multiple_scenarios_from_json() {
    println!("3. Multiple Scenarios from JSON:");
    println!("--------------------------------");

    let scenarios_json = r#"
    [
        {
            "name": "High Value Transaction",
            "config": {
                "include_optional": true,
                "scenario": "Standard",
                "field_configs": {
                    "32A": {
                        "value_range": {
                            "Amount": {
                                "min": 100000.0,
                                "max": 1000000.0,
                                "currency": "USD"
                            }
                        }
                    },
                    "20": {
                        "pattern": "^HV[0-9]{14}$"
                    }
                }
            }
        },
        {
            "name": "Micro Payment",
            "config": {
                "include_optional": false,
                "scenario": "Minimal",
                "field_configs": {
                    "32A": {
                        "value_range": {
                            "Amount": {
                                "min": 1.0,
                                "max": 100.0,
                                "currency": "EUR"
                            }
                        }
                    },
                    "20": {
                        "pattern": "^MP[0-9]{14}$"
                    }
                }
            }
        },
        {
            "name": "Corporate Transfer",
            "config": {
                "include_optional": true,
                "scenario": "StpCompliant",
                "field_configs": {
                    "32A": {
                        "value_range": {
                            "Amount": {
                                "min": 10000.0,
                                "max": 500000.0,
                                "currency": "GBP"
                            }
                        }
                    },
                    "20": {
                        "pattern": "^CORP[0-9]{12}$"
                    },
                    "70": {
                        "fixed_values": ["CORPORATE PAYMENT", "BUSINESS TRANSFER", "COMMERCIAL SETTLEMENT"]
                    }
                }
            }
        }
    ]
    "#;

    #[derive(serde::Deserialize)]
    struct Scenario {
        name: String,
        config: MessageConfig,
    }

    let scenarios: Vec<Scenario> =
        serde_json::from_str(scenarios_json).expect("Failed to parse scenarios");

    for (i, scenario) in scenarios.iter().enumerate() {
        println!("Scenario {}: {}", i + 1, scenario.name);
        println!(
            "  Config: {}",
            serde_json::to_string(&scenario.config).unwrap()
        );

        let mt103_sample = MT103::sample_with_config(&scenario.config);
        println!("  Generated Reference: {}", mt103_sample.field_20.value);
        println!(
            "  Generated Amount: {} {}",
            mt103_sample.field_32a.currency, mt103_sample.field_32a.amount
        );

        if let Some(remittance) = mt103_sample.field_70.as_ref() {
            println!("  Remittance Info: {}", remittance.to_swift_string());
        } else {
            println!("  Remittance Info: (not included)");
        }
        println!();
    }
}

fn example_4_complex_configuration() {
    println!("4. Complex Configuration with Multiple Message Types:");
    println!("----------------------------------------------------");

    let complex_config_json = r#"
    {
        "message_types": {
            "MT103": {
                "include_optional": true,
                "scenario": "StpCompliant",
                "field_configs": {
                    "20": {
                        "length_preference": { "Exact": 16 },
                        "pattern": "^103[0-9]{13}$"
                    },
                    "32A": {
                        "value_range": {
                            "Amount": {
                                "min": 1000.0,
                                "max": 100000.0,
                                "currency": "USD"
                            }
                        }
                    }
                }
            },
            "MT202": {
                "include_optional": false,
                "scenario": "CoverPayment",
                "field_configs": {
                    "20": {
                        "pattern": "^202[0-9]{13}$"
                    },
                    "21": {
                        "pattern": "^REL[0-9]{13}$"
                    },
                    "32A": {
                        "value_range": {
                            "Amount": {
                                "min": 50000.0,
                                "max": 1000000.0,
                                "currency": "EUR"
                            }
                        }
                    }
                }
            }
        }
    }
    "#;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct ComplexConfig {
        message_types: HashMap<String, MessageConfig>,
    }

    let complex_config: ComplexConfig =
        serde_json::from_str(complex_config_json).expect("Failed to parse complex configuration");

    println!("Complex Configuration:");
    println!("{}", serde_json::to_string_pretty(&complex_config).unwrap());

    // Generate samples for each message type
    for (msg_type, config) in complex_config.message_types.iter() {
        println!("\nGenerating {msg_type} with specific config:");

        match msg_type.as_str() {
            "MT103" => {
                let sample = MT103::sample_with_config(config);
                println!("  Reference: {}", sample.field_20.value);
                println!(
                    "  Amount: {} {}",
                    sample.field_32a.currency, sample.field_32a.amount
                );
                println!("  Optional fields included: {}", config.include_optional);
            }
            "MT202" => {
                let sample = MT202::sample_with_config(config);
                println!("  Transaction Ref: {}", sample.field_20.value);
                println!("  Related Ref: {}", sample.field_21.value);
                println!(
                    "  Amount: {} {}",
                    sample.field_32a.currency, sample.field_32a.amount
                );
                println!("  Optional fields included: {}", config.include_optional);
            }
            _ => println!("  Unsupported message type: {msg_type}"),
        }
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_config_json_roundtrip() {
        let config = FieldConfig {
            length_preference: Some(LengthPreference::Exact(16)),
            value_range: Some(ValueRange::Amount {
                min: 1000.0,
                max: 50000.0,
                currency: Some("USD".to_string()),
            }),
            fixed_values: Some(vec!["TEST1".to_string(), "TEST2".to_string()]),
            pattern: Some(r"^[A-Z]{3}[0-9]{13}$".to_string()),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&config).expect("Failed to serialize");

        // Deserialize back
        let parsed_config: FieldConfig =
            serde_json::from_str(&json).expect("Failed to deserialize");

        // Verify roundtrip
        assert_eq!(config.length_preference, parsed_config.length_preference);
        assert_eq!(config.fixed_values, parsed_config.fixed_values);
        assert_eq!(config.pattern, parsed_config.pattern);
    }

    #[test]
    fn test_message_config_json_roundtrip() {
        let mut field_configs = HashMap::new();
        field_configs.insert(
            "20".to_string(),
            FieldConfig {
                length_preference: Some(LengthPreference::Exact(16)),
                ..Default::default()
            },
        );

        let config = MessageConfig {
            include_optional: true,
            field_configs,
            scenario: Some(MessageScenario::StpCompliant),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&config).expect("Failed to serialize");

        // Deserialize back
        let parsed_config: MessageConfig =
            serde_json::from_str(&json).expect("Failed to deserialize");

        // Verify roundtrip
        assert_eq!(config.include_optional, parsed_config.include_optional);
        assert_eq!(config.scenario, parsed_config.scenario);
        assert_eq!(
            config.field_configs.len(),
            parsed_config.field_configs.len()
        );
    }

    #[test]
    fn test_scenario_enum_json() {
        let scenarios = vec![
            MessageScenario::Standard,
            MessageScenario::StpCompliant,
            MessageScenario::CoverPayment,
            MessageScenario::Minimal,
            MessageScenario::Full,
        ];

        for scenario in scenarios {
            let json = serde_json::to_string(&scenario).expect("Failed to serialize scenario");
            let parsed: MessageScenario =
                serde_json::from_str(&json).expect("Failed to deserialize scenario");
            assert_eq!(scenario, parsed);
        }
    }

    #[test]
    fn test_value_range_json() {
        let amount_range = ValueRange::Amount {
            min: 100.0,
            max: 1000.0,
            currency: Some("EUR".to_string()),
        };

        let json = serde_json::to_string(&amount_range).expect("Failed to serialize");
        let parsed: ValueRange = serde_json::from_str(&json).expect("Failed to deserialize");

        match (amount_range, parsed) {
            (
                ValueRange::Amount {
                    min: min1,
                    max: max1,
                    currency: cur1,
                },
                ValueRange::Amount {
                    min: min2,
                    max: max2,
                    currency: cur2,
                },
            ) => {
                assert_eq!(min1, min2);
                assert_eq!(max1, max2);
                assert_eq!(cur1, cur2);
            }
            _ => panic!("Value range types don't match"),
        }
    }
}
