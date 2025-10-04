//! Example of using custom scenario paths for sample generation

use std::path::PathBuf;
use swift_mt_message::{
    SampleGenerator, ScenarioConfig, generate_sample, generate_sample_with_config,
    messages::mt103::MT103,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sample Generation with Custom Paths Example\n");

    // Method 1: Using environment variable
    // Set SWIFT_SCENARIO_PATH=/path1:/path2:/path3 (Unix) or
    // Set SWIFT_SCENARIO_PATH=C:\path1;C:\path2;C:\path3 (Windows)
    println!("1. Using default configuration (checks SWIFT_SCENARIO_PATH env var):");
    match generate_sample::<MT103>("MT103", None) {
        Ok(msg) => {
            println!("   ✓ Generated MT103 using default paths");
            println!(
                "   Transaction Reference: {}",
                msg.fields.field_20.reference
            );
        }
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    println!();

    // Method 2: Using ScenarioConfig with custom paths
    println!("2. Using custom configuration with specific paths:");
    let config = ScenarioConfig::with_paths(vec![
        PathBuf::from("test_scenarios"),
        PathBuf::from("../test_scenarios"),
        PathBuf::from("./custom_scenarios"), // Add custom path
    ]);

    match generate_sample_with_config::<MT103>("MT103", Some("standard"), &config) {
        Ok(msg) => {
            println!("   ✓ Generated MT103 with custom config");
            println!(
                "   Transaction Reference: {}",
                msg.fields.field_20.reference
            );
        }
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    println!();

    // Method 3: Using SampleGenerator builder pattern
    println!("3. Using SampleGenerator with builder pattern:");
    let generator = SampleGenerator::new()
        .with_path(PathBuf::from("./my_scenarios"))
        .with_path(PathBuf::from("./backup_scenarios"))
        .with_path(PathBuf::from("test_scenarios")); // Add default as fallback

    match generator.generate::<MT103>("MT103", Some("minimal")) {
        Ok(msg) => {
            println!("   ✓ Generated MT103 using SampleGenerator");
            println!(
                "   Transaction Reference: {}",
                msg.fields.field_20.reference
            );
        }
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    println!();

    // Method 4: Demonstrate multiple scenario generation with same generator
    println!("4. Generating multiple scenarios with same generator:");
    let generator = SampleGenerator::new();

    let scenarios = vec!["minimal", "standard", "stp"];
    for scenario in scenarios {
        match generator.generate::<MT103>("MT103", Some(scenario)) {
            Ok(msg) => {
                println!(
                    "   ✓ Generated MT103/{}: {}",
                    scenario, msg.fields.field_20.reference
                );
            }
            Err(e) => println!("   ✗ Failed to generate MT103/{}: {}", scenario, e),
        }
    }

    println!("\nDone!");
    Ok(())
}
