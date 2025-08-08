use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use swift_mt_message::{generate_sample, ParsedSwiftMessage, SwiftParser};

#[derive(Debug, Serialize, Deserialize)]
struct ScenarioInfo {
    file: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScenarioIndex {
    message_type: String,
    description: String,
    scenarios: Vec<ScenarioInfo>,
}

#[derive(Debug)]
struct TestResult {
    message_type: String,
    scenario: String,
    parse_status: &'static str,
    validation_status: &'static str,
    roundtrip_status: &'static str,
    error_stage: Option<String>,
}

impl TestResult {
    fn new(message_type: String, scenario: String) -> Self {
        Self {
            message_type,
            scenario,
            parse_status: "⏳",
            validation_status: "⏳",
            roundtrip_status: "⏳",
            error_stage: None,
        }
    }

    fn mark_parse_failed(&mut self, error: String) {
        self.parse_status = "❌";
        self.validation_status = "❔";
        self.roundtrip_status = "❔";
        self.error_stage = Some(format!("Parse: {error}"));
    }

    fn mark_parse_success(&mut self) {
        self.parse_status = "✅";
    }

    fn mark_validation_failed(&mut self, errors: Vec<String>) {
        self.validation_status = "❌";
        self.roundtrip_status = "❔";
        let error_count = errors.len();
        if error_count == 1 {
            self.error_stage = Some(format!("Validation: {}", errors[0]));
        } else {
            self.error_stage = Some(format!("Validation: {error_count} errors"));
        }
    }

    fn mark_validation_success(&mut self) {
        self.validation_status = "✅";
    }

    fn mark_roundtrip_failed(&mut self, stage: &str) {
        self.roundtrip_status = "❌";
        self.error_stage = Some(format!("Roundtrip: {stage}"));
    }

    fn mark_roundtrip_success(&mut self) {
        self.roundtrip_status = "✅";
    }
}

fn test_single_scenario(
    message_type: &str,
    scenario_name: &str,
    test_index: usize,
    debug_mode: bool,
) -> TestResult {
    let mut result = TestResult::new(message_type.to_string(), scenario_name.to_string());

    // Generate sample message
    let generated_message = match message_type {
        "MT101" => generate_sample::<swift_mt_message::messages::mt101::MT101>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT101(Box::new(msg))),
        "MT103" => generate_sample::<swift_mt_message::messages::mt103::MT103>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT103(Box::new(msg))),
        "MT104" => generate_sample::<swift_mt_message::messages::mt104::MT104>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT104(Box::new(msg))),
        "MT107" => generate_sample::<swift_mt_message::messages::mt107::MT107>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT107(Box::new(msg))),
        "MT110" => generate_sample::<swift_mt_message::messages::mt110::MT110>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT110(Box::new(msg))),
        "MT202" => generate_sample::<swift_mt_message::messages::mt202::MT202>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT202(Box::new(msg))),
        "MT205" => generate_sample::<swift_mt_message::messages::mt205::MT205>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT205(Box::new(msg))),
        "MT210" => generate_sample::<swift_mt_message::messages::mt210::MT210>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT210(Box::new(msg))),
        "MT192" => generate_sample::<swift_mt_message::messages::mt192::MT192>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT192(Box::new(msg))),
        "MT196" => generate_sample::<swift_mt_message::messages::mt196::MT196>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT196(Box::new(msg))),
        "MT199" => generate_sample::<swift_mt_message::messages::mt199::MT199>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT199(Box::new(msg))),
        "MT292" => generate_sample::<swift_mt_message::messages::mt292::MT292>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT292(Box::new(msg))),
        "MT296" => generate_sample::<swift_mt_message::messages::mt296::MT296>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT296(Box::new(msg))),
        "MT299" => generate_sample::<swift_mt_message::messages::mt299::MT299>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT299(Box::new(msg))),
        "MT111" => generate_sample::<swift_mt_message::messages::mt111::MT111>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT111(Box::new(msg))),
        "MT112" => generate_sample::<swift_mt_message::messages::mt112::MT112>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT112(Box::new(msg))),
        "MT900" => generate_sample::<swift_mt_message::messages::mt900::MT900>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT900(Box::new(msg))),
        "MT910" => generate_sample::<swift_mt_message::messages::mt910::MT910>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT910(Box::new(msg))),
        "MT920" => generate_sample::<swift_mt_message::messages::mt920::MT920>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT920(Box::new(msg))),
        "MT935" => generate_sample::<swift_mt_message::messages::mt935::MT935>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT935(Box::new(msg))),
        "MT940" => generate_sample::<swift_mt_message::messages::mt940::MT940>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT940(Box::new(msg))),
        "MT941" => generate_sample::<swift_mt_message::messages::mt941::MT941>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT941(Box::new(msg))),
        "MT942" => generate_sample::<swift_mt_message::messages::mt942::MT942>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT942(Box::new(msg))),
        "MT950" => generate_sample::<swift_mt_message::messages::mt950::MT950>(
            message_type,
            Some(scenario_name),
        )
        .map(|msg| ParsedSwiftMessage::MT950(Box::new(msg))),
        // Add more message types as needed
        _ => {
            result.mark_parse_failed(format!("Unsupported message type: {message_type}"));
            return result;
        }
    };

    let parsed_message = match generated_message {
        Ok(msg) => msg,
        Err(e) => {
            result.mark_parse_failed(format!("Generation error: {e}"));
            if debug_mode {
                eprintln!("\n[Test {test_index}] Generation failed: {e}");
                eprintln!("Message type: {message_type}");
                eprintln!("Scenario: {scenario_name}");

                // Try to read the scenario file for debugging
                let scenario_file = format!(
                    "../test_scenarios/{}/{}.json",
                    message_type.to_lowercase(),
                    scenario_name
                );
                if let Ok(content) = std::fs::read_to_string(&scenario_file) {
                    eprintln!("Scenario file content (first 500 chars):");
                    eprintln!("{}", &content[..content.len().min(500)]);
                }

                // Additional debug: try to see what datafake generates
                if let Ok(scenario_json) = swift_mt_message::scenario_config::find_scenario_by_name(
                    message_type,
                    scenario_name,
                ) {
                    if let Ok(generator) = datafake_rs::DataGenerator::from_value(scenario_json) {
                        if let Ok(generated_data) = generator.generate() {
                            if let Ok(json_str) = serde_json::to_string_pretty(&generated_data) {
                                let lines: Vec<&str> = json_str.lines().collect();

                                // Show around line 20 where the error occurs
                                eprintln!("\nGenerated JSON around line 20:");
                                for i in 10..30 {
                                    if i < lines.len() {
                                        eprintln!("{}: {}", i + 1, lines[i]);
                                    }
                                }

                                // Try to deserialize just to see the exact error
                                eprintln!("\nAttempting to deserialize generated JSON...");
                                if message_type == "MT935" {
                                    // First try to extract the fields from the JSON
                                    if let Ok(value) =
                                        serde_json::from_str::<serde_json::Value>(&json_str)
                                    {
                                        if let Some(fields) = value.get("fields") {
                                            eprintln!("\nFound fields object. Attempting to deserialize just the fields...");
                                            match serde_json::from_value::<
                                                swift_mt_message::messages::mt935::MT935,
                                            >(
                                                fields.clone()
                                            ) {
                                                Ok(_) => {
                                                    eprintln!("Fields deserialization succeeded!")
                                                }
                                                Err(e) => {
                                                    eprintln!("Fields deserialization error: {e}");

                                                    // Print the fields JSON for debugging
                                                    if let Ok(fields_str) =
                                                        serde_json::to_string_pretty(fields)
                                                    {
                                                        eprintln!("\nFields JSON:");
                                                        let lines: Vec<&str> =
                                                            fields_str.lines().collect();
                                                        for i in 0..10.min(lines.len()) {
                                                            eprintln!("{}: {}", i + 1, lines[i]);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    match serde_json::from_str::<
                                        swift_mt_message::messages::mt935::MT935,
                                    >(&json_str)
                                    {
                                        Ok(_) => eprintln!(
                                            "Direct deserialization succeeded (shouldn't happen)"
                                        ),
                                        Err(e) => {
                                            eprintln!("\nDirect deserialization error: {e}");
                                            eprintln!(
                                                "Error location: line {}, column {}",
                                                e.line(),
                                                e.column()
                                            );
                                        }
                                    }
                                }

                                // Show around line 35 where new error occurs
                                eprintln!("\nGenerated JSON around line 35:");
                                for i in 25..45 {
                                    if i < lines.len() {
                                        eprintln!("{}: {}", i + 1, lines[i]);
                                    }
                                }

                                // Show around line 56 for newer error
                                eprintln!("\nGenerated JSON around line 56:");
                                for i in 46..66 {
                                    if i < lines.len() {
                                        eprintln!("{}: {}", i + 1, lines[i]);
                                    }
                                }

                                if lines.len() > 82 {
                                    eprintln!("\nGenerated JSON around line 82:");
                                    for i in 70..90 {
                                        if i < lines.len() {
                                            eprintln!("{}: {}", i + 1, lines[i]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return result;
        }
    };

    // Convert to MT format
    let mt_format = match &parsed_message {
        ParsedSwiftMessage::MT101(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT103(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT104(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT107(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT110(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT111(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT112(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT202(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT205(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT210(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT900(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT910(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT920(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT935(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT940(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT941(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT942(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT950(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT192(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT196(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT292(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT296(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT199(msg) => msg.to_mt_message(),
        ParsedSwiftMessage::MT299(msg) => msg.to_mt_message(),
    };

    if debug_mode && (message_type == "MT935" || message_type == "MT940") {
        eprintln!("\n[Test {test_index}] Generated MT message:");
        eprintln!("{}", &mt_format);
    }

    // Parse the MT format back
    let reparsed_message = match SwiftParser::parse_auto(&mt_format) {
        Ok(msg) => {
            result.mark_parse_success();
            msg
        }
        Err(e) => {
            let error_str = format!("{e:?}")
                .split('\n')
                .next()
                .unwrap_or("Unknown")
                .to_string();
            result.mark_parse_failed(error_str.clone());

            if debug_mode {
                eprintln!("\n[Test {test_index}] Parse failed: {error_str}");
                eprintln!("MT message preview (first 500 chars):");
                eprintln!("{}", &mt_format[..mt_format.len().min(500)]);

                // For MT935 and MT940, show the entire message
                if message_type == "MT935" || message_type == "MT940" {
                    eprintln!("\nFull {message_type} message:");
                    eprintln!("{}", &mt_format);
                }

                // If it's a field parsing error, show more context
                if error_str.contains("FieldParsingFailed") {
                    if let Some(field_tag_start) = error_str.find("field_tag: \"") {
                        if let Some(field_tag) = error_str[field_tag_start + 12..].split('"').next()
                        {
                            // Find and display the problematic field in the MT message
                            if let Some(field_pos) = mt_format.find(&format!(":{field_tag}:")) {
                                let field_end = mt_format[field_pos..]
                                    .find("\n:")
                                    .unwrap_or(mt_format.len() - field_pos);
                                let field_content = &mt_format[field_pos..field_pos + field_end];
                                eprintln!("\nProblematic field content:");
                                eprintln!("{field_content}");
                            }
                        }
                    }
                }
            }
            return result;
        }
    };

    // Validate message
    let validation_result = reparsed_message.validate();
    if !validation_result.errors.is_empty() {
        let error_summaries: Vec<String> = validation_result
            .errors
            .iter()
            .map(|e| format!("{e}"))
            .collect();
        result.mark_validation_failed(error_summaries.clone());

        if debug_mode {
            eprintln!("\n[Test {test_index}] Validation failed:");
            for (idx, error) in error_summaries.iter().enumerate() {
                eprintln!("  Error {}: {}", idx + 1, error);
            }

            // Print relevant fields for debugging
            match &reparsed_message {
                ParsedSwiftMessage::MT101(msg) => {
                    eprintln!("\nMessage details:");
                    eprintln!("  Field 20: {:?}", msg.fields.field_20);
                    eprintln!("  Field 21R: {:?}", msg.fields.field_21r);
                    eprintln!("  Field 28D: {:?}", msg.fields.field_28d);
                    eprintln!("  Transactions: {}", msg.fields.transactions.len());
                }
                ParsedSwiftMessage::MT103(msg) => {
                    eprintln!("\nMessage details:");
                    eprintln!("  Field 20: {:?}", msg.fields.field_20);
                    eprintln!("  Field 32A: {:?}", msg.fields.field_32a);
                }
                ParsedSwiftMessage::MT935(msg) => {
                    eprintln!("\nMT935 Message details:");
                    eprintln!("  Field 20: {:?}", msg.fields.field_20);
                    eprintln!("  Rate changes count: {}", msg.fields.rate_changes.len());
                    for (idx, rc) in msg.fields.rate_changes.iter().enumerate() {
                        eprintln!("  Rate change {}:", idx + 1);
                        eprintln!("    Field 23: {:?}", rc.field_23);
                        eprintln!("    Field 25: {:?}", rc.field_25);
                        eprintln!("    Field 30: {:?}", rc.field_30);
                        eprintln!("    Field 37H count: {}", rc.field_37h.len());
                    }
                    eprintln!("  Field 72: {:?}", msg.fields.field_72);
                }
                ParsedSwiftMessage::MT940(msg) => {
                    eprintln!("\nMT940 Message details:");
                    eprintln!("  Field 20: {:?}", msg.fields.field_20);
                    eprintln!("  Field 21: {:?}", msg.fields.field_21);
                    eprintln!("  Field 25: {:?}", msg.fields.field_25);
                    eprintln!("  Field 28C: {:?}", msg.fields.field_28c);
                    eprintln!("  Field 60: {:?}", msg.fields.field_60);
                    eprintln!(
                        "  Statement lines count: {}",
                        msg.fields.statement_lines.len()
                    );
                    for (idx, line) in msg.fields.statement_lines.iter().enumerate() {
                        eprintln!("  Statement line {}:", idx + 1);
                        eprintln!("    Field 61: {:?}", line.field_61);
                        eprintln!("    Field 86: {:?}", line.field_86);
                    }
                    eprintln!("  Field 62: {:?}", msg.fields.field_62);
                    eprintln!("  Field 64: {:?}", msg.fields.field_64);
                    eprintln!("  Field 65: {:?}", msg.fields.field_65);
                    eprintln!("  Field 86: {:?}", msg.fields.field_86);
                }
                _ => {}
            }
        }
    } else {
        result.mark_validation_success();
    }

    // Perform round-trip test
    // Step 1: Serialize to JSON
    let json_representation = match serde_json::to_string_pretty(&parsed_message) {
        Ok(json) => json,
        Err(e) => {
            result.mark_roundtrip_failed(&format!("JSON serialize: {e}"));
            if debug_mode {
                eprintln!("\n[Test {test_index}] JSON serialization failed: {e}");
            }
            return result;
        }
    };

    // Step 2: Deserialize from JSON
    let deserialized_message: ParsedSwiftMessage = match serde_json::from_str(&json_representation)
    {
        Ok(msg) => msg,
        Err(e) => {
            result.mark_roundtrip_failed(&format!("JSON deserialize: {e}"));
            if debug_mode {
                eprintln!("\n[Test {test_index}] JSON deserialization failed: {e}");
                eprintln!("JSON that failed to deserialize:");
                eprintln!(
                    "{}",
                    &json_representation[..json_representation.len().min(1000)]
                );
            }
            return result;
        }
    };

    // Step 3: Compare JSON representations
    let original_json = match serde_json::to_string_pretty(&parsed_message) {
        Ok(json) => json,
        Err(e) => {
            result.mark_roundtrip_failed(&format!("Original JSON: {e}"));
            return result;
        }
    };

    let deserialized_json = match serde_json::to_string_pretty(&deserialized_message) {
        Ok(json) => json,
        Err(e) => {
            result.mark_roundtrip_failed(&format!("Deserialized JSON: {e}"));
            return result;
        }
    };

    if original_json == deserialized_json {
        result.mark_roundtrip_success();
    } else {
        result.mark_roundtrip_failed("JSON mismatch after deserialization");
        if debug_mode {
            eprintln!("\n[Test {test_index}] JSON roundtrip mismatch");
            eprintln!("Original JSON length: {}", original_json.len());
            eprintln!("Deserialized JSON length: {}", deserialized_json.len());

            // Find first difference
            let chars1: Vec<_> = original_json.chars().collect();
            let chars2: Vec<_> = deserialized_json.chars().collect();
            for (i, (c1, c2)) in chars1.iter().zip(chars2.iter()).enumerate() {
                if c1 != c2 {
                    let start = i.saturating_sub(50);
                    let end = (i + 50).min(chars1.len()).min(chars2.len());
                    eprintln!("First difference at position {i}:");
                    eprintln!(
                        "Original: ...{}...",
                        chars1[start..end].iter().collect::<String>()
                    );
                    eprintln!(
                        "Deserialized: ...{}...",
                        chars2[start..end].iter().collect::<String>()
                    );
                    break;
                }
            }
        }
    }

    result
}

fn get_message_types() -> Vec<String> {
    let scenarios_dir = Path::new("../test_scenarios");
    let mut message_types = Vec::new();

    if let Ok(entries) = fs::read_dir(scenarios_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                        if name.starts_with("mt") {
                            message_types.push(name.to_uppercase());
                        }
                    }
                }
            }
        }
    }

    message_types.sort();
    message_types
}

fn get_scenarios_for_message_type(message_type: &str) -> Vec<String> {
    let index_path = format!(
        "../test_scenarios/{}/index.json",
        message_type.to_lowercase()
    );

    match fs::read_to_string(&index_path) {
        Ok(content) => match serde_json::from_str::<ScenarioIndex>(&content) {
            Ok(index) => index.scenarios.into_iter().map(|s| {
                // Extract the scenario name from the file name (remove .json extension)
                s.file.trim_end_matches(".json").to_string()
            }).collect(),
            Err(e) => {
                eprintln!("Failed to parse index.json for {message_type}: {e}");
                Vec::new()
            }
        },
        Err(e) => {
            eprintln!("Failed to read index.json for {message_type}: {e}");
            Vec::new()
        }
    }
}

/// Round-trip test for SWIFT MT message scenarios
///
/// Environment variables:
/// - `TEST_MESSAGE_TYPE`: Test specific message type (e.g., "MT101")
/// - `TEST_SCENARIO`: Test specific scenario (e.g., "urgent_payment")
/// - `TEST_DEBUG`: Enable debug output for failures
/// - `TEST_STOP_ON_FAILURE`: Stop testing on first failure (useful with TEST_DEBUG)
/// - `TEST_SAMPLE_COUNT`: Number of samples per scenario (default: 100)
///
/// Examples:
/// ```bash
/// # Test all scenarios
/// cargo test round_trip_scenarios
///
/// # Test specific message type
/// TEST_MESSAGE_TYPE=MT101 cargo test round_trip_scenarios
///
/// # Debug specific scenario with single sample
/// TEST_MESSAGE_TYPE=MT101 TEST_SCENARIO=urgent_payment TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test round_trip_scenarios -- --nocapture
///
/// # Debug with stop on first failure
/// TEST_MESSAGE_TYPE=MT101 TEST_DEBUG=1 TEST_STOP_ON_FAILURE=1 cargo test round_trip_scenarios -- --nocapture
/// ```
#[test]
fn test_round_trip_scenarios() {
    // Get test parameters from environment variables
    let message_type = env::var("TEST_MESSAGE_TYPE").ok();
    let scenario_name = env::var("TEST_SCENARIO").ok();
    let debug_mode = env::var("TEST_DEBUG").is_ok();
    let stop_on_failure = env::var("TEST_STOP_ON_FAILURE").is_ok();
    let samples_str = env::var("TEST_SAMPLE_COUNT").unwrap_or_else(|_| "100".to_string());
    let samples_per_scenario = samples_str.parse::<usize>().unwrap_or(100);

    let mut test_results = Vec::new();

    if debug_mode {
        eprintln!("🔍 Debug mode enabled");
        eprintln!("   Samples per scenario: {samples_per_scenario}");
        if stop_on_failure {
            eprintln!("   Stop on first failure: enabled");
        }
        if let Some(ref mt) = message_type {
            eprintln!("   Message type: {mt}");
        }
        if let Some(ref sc) = scenario_name {
            eprintln!("   Scenario: {sc}");
        }
        eprintln!();
    }

    match (message_type, scenario_name) {
        (None, None) => {
            // No parameters: test all message types and all scenarios
            let message_types = get_message_types();
            println!("Testing all message types: {message_types:?}");

            for message_type in message_types {
                let scenarios = get_scenarios_for_message_type(&message_type);
                println!("\n{}: Testing {} scenarios", message_type, scenarios.len());

                for scenario in scenarios {
                    println!(
                        "  Testing {message_type}/{scenario} ({samples_per_scenario} samples)..."
                    );
                    for i in 0..samples_per_scenario {
                        let result =
                            test_single_scenario(&message_type, &scenario, i + 1, debug_mode);
                        let is_failure = result.parse_status == "❌"
                            || result.validation_status == "❌"
                            || result.roundtrip_status == "❌";
                        test_results.push(result);

                        if is_failure && stop_on_failure {
                            eprintln!("\n⛔ Stopping on first failure (TEST_STOP_ON_FAILURE=1)");
                            break;
                        }
                    }
                }
            }
        }
        (Some(message_type), None) => {
            // One parameter: test all scenarios for given message type
            let message_type = message_type.to_uppercase();
            let scenarios = get_scenarios_for_message_type(&message_type);

            if scenarios.is_empty() {
                panic!("No scenarios found for message type: {message_type}");
            }

            println!("Testing {}: {} scenarios", message_type, scenarios.len());

            for scenario in scenarios {
                println!("  Testing {message_type}/{scenario} ({samples_per_scenario} samples)...");
                for i in 0..samples_per_scenario {
                    let result = test_single_scenario(&message_type, &scenario, i + 1, debug_mode);
                    test_results.push(result);
                }
            }
        }
        (Some(message_type), Some(scenario)) => {
            // Two parameters: test specific scenario for given message type
            let message_type = message_type.to_uppercase();

            println!("Testing {message_type}/{scenario} ({samples_per_scenario} samples)...");

            for i in 0..samples_per_scenario {
                let result = test_single_scenario(&message_type, &scenario, i + 1, debug_mode);
                test_results.push(result);
            }
        }
        _ => {
            // Invalid combination
            panic!("Invalid test parameters. Use TEST_MESSAGE_TYPE and TEST_SCENARIO environment variables.");
        }
    }

    // Print results summary
    print_results_summary(&test_results);

    // Determine if test should fail
    let failed_results: Vec<_> = test_results
        .iter()
        .enumerate()
        .filter(|(_, r)| {
            r.parse_status == "❌" || r.validation_status == "❌" || r.roundtrip_status == "❌"
        })
        .collect();

    if !failed_results.is_empty() {
        println!("\n❌ Failed tests: {}", failed_results.len());

        // Group failures by message type and scenario
        let mut failure_summary: std::collections::HashMap<String, Vec<(usize, &TestResult)>> =
            std::collections::HashMap::new();

        for (idx, result) in &failed_results {
            let key = format!("{}/{}", result.message_type, result.scenario);
            failure_summary
                .entry(key)
                .or_default()
                .push((*idx, *result));
        }

        println!("\nFailure summary:");
        for (key, failures) in &failure_summary {
            println!("  {} - {} failures", key, failures.len());

            if debug_mode && failures.len() <= 5 {
                // Show details for first few failures
                for (test_idx, result) in failures.iter().take(3) {
                    println!(
                        "    Test #{}: {}",
                        test_idx + 1,
                        result
                            .error_stage
                            .as_ref()
                            .unwrap_or(&"Unknown error".to_string())
                    );
                }
                if failures.len() > 3 {
                    println!("    ... and {} more", failures.len() - 3);
                }
            }
        }

        if debug_mode && failed_results.len() <= 10 {
            println!("\n💡 Tip: To debug a specific failure, run:");
            let (_, first_failure) = &failed_results[0];
            println!("   TEST_MESSAGE_TYPE={} TEST_SCENARIO={} TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test round_trip_scenarios -- --nocapture",
                first_failure.message_type, first_failure.scenario);
        }

        panic!(
            "Round-trip test failed: {} out of {} tests failed",
            failed_results.len(),
            test_results.len()
        );
    }

    println!("\n✅ All {} tests passed!", test_results.len());
}

fn print_results_summary(results: &[TestResult]) {
    // Group results by scenario for aggregate view
    let mut scenario_results: std::collections::HashMap<String, Vec<&TestResult>> =
        std::collections::HashMap::new();

    for result in results {
        let key = format!("{}/{}", result.message_type, result.scenario);
        scenario_results.entry(key).or_default().push(result);
    }

    // Sort scenarios by name
    let mut scenarios: Vec<_> = scenario_results.into_iter().collect();
    scenarios.sort_by_key(|k| k.0.clone());

    // Calculate column widths
    let scenario_width = scenarios
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(20)
        .max(20);

    // Print table header
    println!(
        "\n╔═{}═╤═{}═╤═{}═╤═{}═╗",
        "═".repeat(scenario_width),
        "═".repeat(12),
        "═".repeat(12),
        "═".repeat(12)
    );
    println!(
        "║ {:<width$} │  {:^10}  │ {:^12} │ {:^12} ║",
        "Scenario",
        "Parser",
        "Validation",
        "Roundtrip",
        width = scenario_width
    );
    println!(
        "╟─{}─┼─{}─┼─{}─┼─{}─╢",
        "─".repeat(scenario_width),
        "─".repeat(12),
        "─".repeat(12),
        "─".repeat(12)
    );

    // Print each scenario's aggregate results
    for (scenario_name, results_group) in scenarios {
        let total = results_group.len();
        let parse_success = results_group
            .iter()
            .filter(|r| r.parse_status == "✅")
            .count();
        let validation_success = results_group
            .iter()
            .filter(|r| r.validation_status == "✅")
            .count();
        let roundtrip_success = results_group
            .iter()
            .filter(|r| r.roundtrip_status == "✅")
            .count();

        // Determine status symbols based on success rates
        let parse_symbol = get_status_symbol(parse_success, total);
        let validation_symbol = get_status_symbol(validation_success, total);
        let roundtrip_symbol = get_status_symbol(roundtrip_success, total);

        println!(
            "║ {scenario_name:<scenario_width$} │      {parse_symbol:^2}     │      {validation_symbol:^2}     │      {roundtrip_symbol:^2}     ║"
        );
    }

    // Print table footer
    println!(
        "╚═{}═╧═{}═╧═{}═╧═{}═╝",
        "═".repeat(scenario_width),
        "═".repeat(12),
        "═".repeat(12),
        "═".repeat(12)
    );

    // Print summary statistics
    let total_tests = results.len();
    let parse_success_total = results.iter().filter(|r| r.parse_status == "✅").count();
    let validation_success_total = results
        .iter()
        .filter(|r| r.validation_status == "✅")
        .count();
    let roundtrip_success_total = results
        .iter()
        .filter(|r| r.roundtrip_status == "✅")
        .count();

    println!("\n📊 Summary:");
    println!("   Total tests: {total_tests}");
    println!(
        "   Parse successful: {} ({}%)",
        parse_success_total,
        if total_tests > 0 {
            (parse_success_total * 100) / total_tests
        } else {
            0
        }
    );
    println!(
        "   Validation successful: {} ({}%)",
        validation_success_total,
        if total_tests > 0 {
            (validation_success_total * 100) / total_tests
        } else {
            0
        }
    );
    println!(
        "   Roundtrip successful: {} ({}%)",
        roundtrip_success_total,
        if total_tests > 0 {
            (roundtrip_success_total * 100) / total_tests
        } else {
            0
        }
    );
}

fn get_status_symbol(success_count: usize, total_count: usize) -> &'static str {
    if total_count == 0 {
        "❔"
    } else if success_count == total_count {
        "✅"
    } else if success_count == 0 {
        "❌"
    } else {
        "⚠️"
    }
}
