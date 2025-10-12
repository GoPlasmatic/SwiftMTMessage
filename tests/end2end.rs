use dataflow_rs::Engine;
use dataflow_rs::engine::{AsyncFunctionHandler, Message, Workflow};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use swift_mt_message::plugin::register_swift_mt_functions;

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

/// End-to-end test for SWIFT MT message processing pipeline using dataflow engine
///
/// This test creates a complete dataflow workflow that:
/// 1. Generates sample data from schema using datafake
/// 2. Publishes the data to MT message format
/// 3. Parses the MT message back to structured format
/// 4. Validates the message against SWIFT rules
/// 5. Transforms back to MT format for round-trip verification
///
/// The workflow is executed as a pipeline with each task passing its output
/// to the next step through the message context. The workflow is defined in
/// JSON format and executed by the dataflow engine.
///
/// Environment variables:
/// - `TEST_MESSAGE_TYPE`: Test specific message type (e.g., "MT103")
/// - `TEST_SCENARIO`: Test specific scenario (e.g., "urgent_payment")
/// - `TEST_DEBUG`: Enable debug output for failures
/// - `TEST_STOP_ON_FAILURE`: Stop testing on first failure (useful with TEST_DEBUG)
/// - `TEST_SAMPLE_COUNT`: Number of samples per scenario (default: 10)
///
/// Examples:
/// ```bash
/// # Test all scenarios
/// cargo test test_swift_mt_workflow_pipeline
///
/// # Test specific message type
/// TEST_MESSAGE_TYPE=MT103 cargo test test_swift_mt_workflow_pipeline
///
/// # Debug specific scenario with single sample
/// TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=urgent_payment TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test test_swift_mt_workflow_pipeline -- --nocapture
///
/// # Test with stop on first failure
/// TEST_DEBUG=1 TEST_STOP_ON_FAILURE=1 cargo test test_swift_mt_workflow_pipeline -- --nocapture
/// ```
#[tokio::test]
async fn test_swift_mt_workflow_pipeline() {
    // Get test parameters from environment variables (same as round_trip_test.rs)
    let message_type = env::var("TEST_MESSAGE_TYPE").ok();
    let scenario_name = env::var("TEST_SCENARIO").ok();
    let debug_mode = env::var("TEST_DEBUG").is_ok();
    let stop_on_failure = env::var("TEST_STOP_ON_FAILURE").is_ok();
    let samples_str = env::var("TEST_SAMPLE_COUNT").unwrap_or_else(|_| "10".to_string());
    let samples_per_scenario = samples_str.parse::<usize>().unwrap_or(10);

    // Create the dataflow engine with registered SWIFT MT functions
    let mut custom_functions: HashMap<String, Box<dyn AsyncFunctionHandler + Send + Sync>> =
        HashMap::new();

    // Register all SWIFT MT plugin functions
    for (name, handler) in register_swift_mt_functions() {
        custom_functions.insert(name.to_string(), handler);
    }

    // Prepare test cases based on environment variables
    let test_cases = get_test_cases(message_type.as_deref(), scenario_name.as_deref());

    if test_cases.is_empty() {
        panic!("No test cases found for the given parameters");
    }

    // Create workflows for unique message types
    let mut workflows = Vec::new();
    let mut processed_types = std::collections::HashSet::new();
    for (mt_type, _, _) in &test_cases {
        if processed_types.insert(mt_type.clone()) {
            workflows.push(create_swift_mt_workflow(mt_type));
        }
    }

    // Create the engine with workflows and custom functions
    let engine = Engine::new(workflows, Some(custom_functions));

    let mut all_results = Vec::new();
    let mut failure_count = 0;

    // Run tests for each scenario
    for (message_type, scenario, description) in &test_cases {
        if debug_mode {
            println!("\n========================================");
            println!("Testing {} - {}", message_type, scenario);
            if scenario != description {
                println!("Description: {}", description);
            }
            println!("========================================");
        }

        // Run multiple samples per scenario
        for sample_idx in 0..samples_per_scenario {
            let schema = match load_scenario_schema(message_type, scenario) {
                Ok(schema) => schema,
                Err(e) => {
                    if debug_mode {
                        eprintln!(
                            "Failed to load schema for {}/{}: {}",
                            message_type, scenario, e
                        );
                    }
                    failure_count += 1;
                    continue;
                }
            };

            // Create message with schema as payload (for generate_mt plugin)
            let mut message = Message::from_value(&schema);

            // Set the data fields for workflow routing and tracking
            message
                .data_mut()
                .as_object_mut()
                .unwrap()
                .insert("message_type".to_string(), json!(message_type));
            message
                .data_mut()
                .as_object_mut()
                .unwrap()
                .insert("scenario".to_string(), json!(scenario));

            // Important: invalidate cache after modifying data
            message.invalidate_context_cache();

            if debug_mode {
                println!("\nDebug - Initial message created with:");
                println!("  message_type: {}", message_type);
                println!("  scenario: {}", scenario);
                println!(
                    "  payload (schema): {} bytes",
                    serde_json::to_string(&schema).unwrap_or_default().len()
                );

                // Verify the data was set
                if let Some(obj) = message.data().as_object() {
                    println!(
                        "  Data fields present: {:?}",
                        obj.keys().collect::<Vec<_>>()
                    );
                }
            }

            // Add metadata for tracking
            message.metadata_mut().as_object_mut().unwrap().insert(
                "test_info".to_string(),
                json!({
                    "message_type": message_type,
                    "scenario": scenario,
                    "sample_index": sample_idx,
                    "start_time": chrono::Utc::now().to_rfc3339()
                }),
            );

            // Process the message through the engine
            let result = engine.process_message(&mut message).await;

            if debug_mode {
                // Debug: Print what's in the message data after processing
                println!("\nDebug - Message data before processing:");
                if let Some(obj) = message.data().as_object() {
                    for (key, _) in obj {
                        println!("  - {}", key);
                    }
                }

                println!("\nDebug - Workflow execution result: {:?}", result);

                println!("\nDebug - Message data after processing:");
                if let Some(obj) = message.data().as_object() {
                    println!("  Total fields in data: {}", obj.len());

                    // Check for validation_result explicitly
                    if !obj.contains_key("validation_result") {
                        println!("  WARNING: validation_result field is missing!");
                    }

                    for (key, value) in obj {
                        println!("  - {}", key);

                        // Show detailed output for key fields to debug the structure
                        if (key == "sample_json"
                            || key == "sample_mt"
                            || key == "validation_result"
                            || key == "mt_json")
                            && (message_type == "MT104"
                                || (scenario == "standard" && message_type == "MT103"))
                        {
                            println!("\nDebug - {} structure:", key);
                            if let Some(json_data) = value.get("json_data") {
                                // Show the top-level keys
                                if let Some(json_obj) = json_data.as_object() {
                                    println!(
                                        "  json_data keys: {:?}",
                                        json_obj.keys().collect::<Vec<_>>()
                                    );

                                    // Show fields structure
                                    if let Some(fields) = json_obj.get("fields")
                                        && let Some(fields_obj) = fields.as_object()
                                    {
                                        println!(
                                            "  fields keys: {:?}",
                                            fields_obj.keys().collect::<Vec<_>>()
                                        );

                                        // Show a sample field
                                        if let Some(field_20) = fields_obj.get("20") {
                                            println!(
                                                "  field 20 structure: {}",
                                                serde_json::to_string_pretty(field_20)
                                                    .unwrap_or_default()
                                            );
                                        }
                                        if let Some(field_50) = fields_obj.get("50") {
                                            println!(
                                                "  field 50 structure: {}",
                                                serde_json::to_string_pretty(field_50)
                                                    .unwrap_or_default()
                                            );
                                        }
                                    }
                                } else {
                                    println!("  json_data is not an object!");
                                }
                            } else {
                                println!("  No json_data field in {}!", key);
                                println!(
                                    "  {} keys: {:?}",
                                    key,
                                    value.as_object().map(|o| o.keys().collect::<Vec<_>>())
                                );

                                // For MT104, show fields
                                if message_type == "MT104" && key == "sample_json" {
                                    if let Some(obj) = value.as_object() {
                                        if let Some(fields) = obj.get("fields") {
                                            println!(
                                                "  MT104 fields: {}",
                                                serde_json::to_string_pretty(fields)
                                                    .unwrap_or_default()
                                            );
                                        }
                                    }
                                }
                            }
                        } else if key == "sample_mt" {
                            println!("\nDebug - Sample MT structure:");
                            if let Some(mt_str) = value.get("mt_message").and_then(|v| v.as_str()) {
                                println!("  MT message exists, full message:");
                                println!("  {}", mt_str);
                            } else if let Some(mt_str) = value.as_str() {
                                println!("  MT message as string, full message:");
                                println!("  {}", mt_str);
                            } else {
                                println!("  MT message structure: {:?}", value);
                            }
                        } else if key == "validation_result" {
                            println!("\nDebug - Validation Result:");
                            if let Some(valid) = value.get("valid").and_then(|v| v.as_bool()) {
                                println!("  Valid: {}", valid);
                            }
                            if let Some(errors) = value.get("errors").and_then(|v| v.as_array()) {
                                println!("  Errors count: {}", errors.len());
                                for (i, err) in errors.iter().take(3).enumerate() {
                                    println!("    Error {}: {:?}", i + 1, err);
                                }
                            }
                        }
                    }

                    // Check if the workflow was triggered
                    if obj.is_empty() {
                        println!(
                            "\nWARNING: Message data is empty - workflow may not have been triggered!"
                        );
                        println!("Expected workflow condition: data.message_type == 'MT103'");
                        if let Some(mt) = obj.get("message_type") {
                            println!("Actual message_type value: {:?}", mt);
                        }
                    }
                } else {
                    println!("  Message data is not an object!");
                }
            }

            // Analyze the results
            let test_result = match result {
                Ok(_) => {
                    // Analyze the workflow results
                    let workflow_result = analyze_workflow_results(&message, debug_mode);

                    TestResult {
                        message_type: message_type.to_string(),
                        scenario: scenario.to_string(),
                        workflow_completed: true,
                        generate_success: workflow_result.generate_success,
                        publish_success: workflow_result.publish_success,
                        parse_success: workflow_result.parse_success,
                        validation_passed: workflow_result.validate_success,
                        transform_success: workflow_result.transform_success,
                        round_trip_success: workflow_result.round_trip_match,
                        error: None,
                    }
                }
                Err(e) => {
                    if debug_mode {
                        eprintln!("\nWorkflow execution failed: {:?}", e);
                    }

                    TestResult {
                        message_type: message_type.to_string(),
                        scenario: scenario.to_string(),
                        workflow_completed: false,
                        generate_success: false,
                        publish_success: false,
                        parse_success: false,
                        validation_passed: false,
                        transform_success: false,
                        round_trip_success: false,
                        error: Some(format!("{:?}", e)),
                    }
                }
            };

            // Track failures
            if !test_result.is_fully_successful() {
                failure_count += 1;
                if debug_mode {
                    println!("\n❌ Sample {} failed:", sample_idx);
                    println!("  Workflow Steps:");
                    println!(
                        "    1. Generate: {}",
                        status_symbol(test_result.generate_success)
                    );
                    println!(
                        "    2. Publish: {}",
                        status_symbol(test_result.publish_success)
                    );
                    println!("    3. Parse: {}", status_symbol(test_result.parse_success));
                    println!(
                        "    4. Validate: {}",
                        status_symbol(test_result.validation_passed)
                    );
                    println!(
                        "    5. Transform: {}",
                        status_symbol(test_result.transform_success)
                    );
                    println!(
                        "  Round-trip: {}",
                        status_symbol(test_result.round_trip_success)
                    );

                    if let Some(ref error) = test_result.error {
                        println!("  Error: {}", error);
                    }
                }

                if stop_on_failure {
                    eprintln!("\n⛔ Stopping on first failure (TEST_STOP_ON_FAILURE=1)");
                    all_results.push(test_result);
                    break;
                }
            }

            all_results.push(test_result);
        }

        if stop_on_failure && failure_count > 0 {
            break;
        }
    }

    // Print summary
    print_test_summary(&all_results);

    // Assert based on results
    if failure_count > 0 {
        panic!(
            "\n❌ Workflow test failed: {} out of {} tests failed",
            failure_count,
            all_results.len()
        );
    } else {
        println!("\n✅ All {} workflow tests passed!", all_results.len());
    }
}

/// Get test cases based on environment variables
fn get_test_cases(
    message_type: Option<&str>,
    scenario: Option<&str>,
) -> Vec<(String, String, String)> {
    match (message_type, scenario) {
        (None, None) => {
            // Test all message types and scenarios
            get_all_test_cases()
        }
        (Some(mt), None) => {
            // Test all scenarios for given message type
            // Load scenarios with descriptions from index.json
            let index_path = format!("test_scenarios/{}/index.json", mt.to_lowercase());

            if let Ok(content) = fs::read_to_string(&index_path) {
                if let Ok(index) = serde_json::from_str::<ScenarioIndex>(&content) {
                    index
                        .scenarios
                        .into_iter()
                        .map(|s| {
                            let name = s.file.trim_end_matches(".json");
                            (mt.to_string(), name.to_string(), s.description)
                        })
                        .collect()
                } else {
                    // Fallback if index parsing fails
                    get_scenarios_for_message_type(mt)
                        .into_iter()
                        .map(|s| (mt.to_string(), s.clone(), s))
                        .collect()
                }
            } else {
                // Fallback if index doesn't exist
                get_scenarios_for_message_type(mt)
                    .into_iter()
                    .map(|s| (mt.to_string(), s.clone(), s))
                    .collect()
            }
        }
        (Some(mt), Some(sc)) => {
            // Test specific message type and scenario
            vec![(mt.to_string(), sc.to_string(), sc.to_string())]
        }
        (None, Some(_)) => {
            // Invalid: scenario without message type
            eprintln!("Warning: TEST_SCENARIO requires TEST_MESSAGE_TYPE");
            vec![]
        }
    }
}

/// Get all available test cases from the test_scenarios directory
fn get_all_test_cases() -> Vec<(String, String, String)> {
    let mut test_cases = Vec::new();
    let scenarios_dir = Path::new("test_scenarios");

    if let Ok(entries) = fs::read_dir(scenarios_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Some(dir_name) = path.file_name().and_then(|s| s.to_str())
                && dir_name.starts_with("mt")
            {
                let message_type = dir_name.to_uppercase();

                // Try to load index.json first
                let index_path = path.join("index.json");
                if index_path.exists()
                    && let Ok(content) = fs::read_to_string(&index_path)
                    && let Ok(index) = serde_json::from_str::<ScenarioIndex>(&content)
                {
                    for scenario_info in index.scenarios {
                        let scenario_name = scenario_info.file.trim_end_matches(".json");
                        test_cases.push((
                            message_type.clone(),
                            scenario_name.to_string(),
                            scenario_info.description,
                        ));
                    }
                    continue;
                }

                // Fallback to getting scenarios without index
                let scenarios = get_scenarios_fallback(&message_type);
                for scenario in scenarios {
                    test_cases.push((message_type.clone(), scenario.clone(), scenario));
                }
            }
        }
    }

    // Return all test cases without limiting
    test_cases
}

/// Get scenarios for a specific message type using index.json
fn get_scenarios_for_message_type(message_type: &str) -> Vec<String> {
    let index_path = format!("test_scenarios/{}/index.json", message_type.to_lowercase());

    match fs::read_to_string(&index_path) {
        Ok(content) => match serde_json::from_str::<ScenarioIndex>(&content) {
            Ok(index) => index
                .scenarios
                .into_iter()
                .map(|s| {
                    // Extract the scenario name from the file name (remove .json extension)
                    s.file.trim_end_matches(".json").to_string()
                })
                .collect(),
            Err(e) => {
                eprintln!("Failed to parse index.json for {}: {}", message_type, e);
                Vec::new()
            }
        },
        Err(e) => {
            eprintln!("Failed to read index.json for {}: {}", message_type, e);
            // Fallback to directory listing
            get_scenarios_fallback(message_type)
        }
    }
}

/// Fallback method to get scenarios by directory listing
fn get_scenarios_fallback(message_type: &str) -> Vec<String> {
    let mut scenarios = Vec::new();
    let scenario_dir = Path::new("test_scenarios").join(message_type.to_lowercase());

    if let Ok(entries) = fs::read_dir(&scenario_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json")
                && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                && stem != "index"
            {
                scenarios.push(stem.to_string());
            }
        }
    }

    scenarios
}

/// Load scenario schema from file
fn load_scenario_schema(message_type: &str, scenario: &str) -> Result<Value, String> {
    let file_path = format!(
        "test_scenarios/{}/{}.json",
        message_type.to_lowercase(),
        scenario
    );

    fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))
        .and_then(|content| {
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON from {}: {}", file_path, e))
        })
}

/// Create the SWIFT MT processing workflow from JSON definition
fn create_swift_mt_workflow(message_type: &str) -> Workflow {
    // Define the workflow in JSON format for better readability and maintainability
    let workflow_json = json!({
        "id": format!("swift_mt_{}_workflow", message_type.to_lowercase()),
        "name": format!("SWIFT {} Processing Pipeline", message_type),
        "description": format!("End-to-end processing pipeline for {} messages", message_type),
        "priority": 0,
        "condition": {
            "==": [
                {"var": "data.message_type"},
                message_type
            ]
        },
        "tasks": [
            {
                "id": "step_1_generate",
                "name": "Generate Sample JSON",
                "description": "Generate sample JSON data from schema using datafake",
                "function": {
                    "name": "generate_mt",
                    "input": {
                        "target": "sample_json"
                    }
                },
            },
            {
                "id": "step_2_publish",
                "name": "Publish to MT Format",
                "description": "Convert sample JSON data to SWIFT MT message format",
                "function": {
                    "name": "publish_mt",
                    "input": {
                        "source": "sample_json",
                        "target": "sample_mt"
                    }
                },
            },
            {
                "id": "step_3_validate",
                "name": "Validate MT Message",
                "description": "Validate MT message against SWIFT standards",
                "function": {
                    "name": "validate_mt",
                    "input": {
                        "source": "sample_mt",
                        "target": "validation_result"
                    }
                },
            },
            {
                "id": "step_4_parse",
                "name": "Parse MT Message",
                "description": "Parse MT message back to structured JSON format",
                "function": {
                    "name": "parse_mt",
                    "input": {
                        "source": "sample_mt",
                        "target": "mt_json"
                    }
                },
            }
        ],
    });

    // Convert JSON to Workflow struct using from_json
    let workflow_str =
        serde_json::to_string(&workflow_json).expect("Failed to serialize workflow JSON");
    Workflow::from_json(&workflow_str).expect("Failed to parse workflow JSON")
}

/// Normalize JSON values to handle float/integer differences and remove null fields
fn normalize_json_value(value: &Value) -> Value {
    match value {
        Value::Number(n) => {
            // Convert all numbers to their canonical form
            if let Some(f) = n.as_f64() {
                // If it's a whole number, keep it as integer
                if f.fract() == 0.0 && f.is_finite() {
                    Value::Number(serde_json::Number::from(f as i64))
                } else {
                    value.clone()
                }
            } else {
                value.clone()
            }
        }
        Value::Object(obj) => {
            let mut normalized = serde_json::Map::new();
            for (k, v) in obj {
                // Skip null values to handle optional fields that weren't in original
                if !v.is_null() {
                    normalized.insert(k.clone(), normalize_json_value(v));
                }
            }
            Value::Object(normalized)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(normalize_json_value).collect()),
        _ => value.clone(),
    }
}

/// Check if round-trip was successful by comparing sample_json with mt_json
fn check_round_trip_success(message: &Message) -> bool {
    // Extract the original sample_json and the parsed mt_json
    let sample_json = message.data().get("sample_json");
    let mt_json = message.data().get("mt_json");

    match (sample_json, mt_json) {
        (Some(original), Some(parsed)) => {
            // sample_json has a json_data wrapper, mt_json doesn't
            let original_data = original.get("json_data").unwrap_or(original);

            // Normalize both values to handle float/integer differences
            let normalized_orig = normalize_json_value(original_data);
            let normalized_parsed = normalize_json_value(parsed);

            // Compare normalized JSON
            let orig_str = serde_json::to_string(&normalized_orig).unwrap_or_default();
            let parsed_str = serde_json::to_string(&normalized_parsed).unwrap_or_default();

            // Debug: Print first few keys to see the difference
            if std::env::var("TEST_DEBUG").unwrap_or_default() == "1" && orig_str != parsed_str {
                println!("\n  Debug round-trip comparison:");
                println!("    JSON strings are different");
                if let (Some(orig_obj), Some(parsed_obj)) =
                    (normalized_orig.as_object(), normalized_parsed.as_object())
                {
                    println!(
                        "    Original keys: {:?}",
                        orig_obj.keys().collect::<Vec<_>>()
                    );
                    println!(
                        "    Parsed keys: {:?}",
                        parsed_obj.keys().collect::<Vec<_>>()
                    );

                    // Compare each top-level field
                    for key in orig_obj.keys() {
                        let orig_val =
                            serde_json::to_string(orig_obj.get(key).unwrap()).unwrap_or_default();
                        let parsed_val =
                            serde_json::to_string(parsed_obj.get(key).unwrap_or(&Value::Null))
                                .unwrap_or_default();
                        if orig_val != parsed_val {
                            println!("    Mismatch in '{}' field!", key);
                            if key == "fields" {
                                // Show field-level differences
                                if let (Some(orig_fields), Some(parsed_fields)) = (
                                    orig_obj.get("fields").and_then(|f| f.as_object()),
                                    parsed_obj.get("fields").and_then(|f| f.as_object()),
                                ) {
                                    // Collect all unique keys from both orig and parsed
                                    let mut all_keys: std::collections::HashSet<_> =
                                        orig_fields.keys().collect();
                                    all_keys.extend(parsed_fields.keys());

                                    for field_key in all_keys {
                                        let orig_field = orig_fields.get(field_key);
                                        let parsed_field = parsed_fields.get(field_key);

                                        match (orig_field, parsed_field) {
                                            (Some(o), Some(p)) => {
                                                let orig_str =
                                                    serde_json::to_string(o).unwrap_or_default();
                                                let parsed_str =
                                                    serde_json::to_string(p).unwrap_or_default();
                                                if orig_str != parsed_str {
                                                    println!("      Field {} differs:", field_key);
                                                    println!("        Original: {}", orig_str);
                                                    println!("        Parsed: {}", parsed_str);
                                                }
                                            }
                                            (Some(o), None) => {
                                                println!(
                                                    "      Field {} only in original:",
                                                    field_key
                                                );
                                                println!(
                                                    "        Original: {}",
                                                    serde_json::to_string(o).unwrap_or_default()
                                                );
                                            }
                                            (None, Some(p)) => {
                                                println!(
                                                    "      Field {} only in parsed:",
                                                    field_key
                                                );
                                                println!(
                                                    "        Parsed: {}",
                                                    serde_json::to_string(p).unwrap_or_default()
                                                );
                                            }
                                            (None, None) => {} // Shouldn't happen
                                        }
                                    }
                                }
                            } else {
                                println!("      Original: {}", orig_val);
                                println!("      Parsed: {}", parsed_val);
                            }
                        }
                    }
                }
            }

            // Compare normalized strings
            orig_str == parsed_str
        }
        _ => false,
    }
}

/// Check workflow step results
fn analyze_workflow_results(message: &Message, debug_mode: bool) -> WorkflowResult {
    let mut result = WorkflowResult {
        generate_success: message.data().get("sample_json").is_some(),
        publish_success: message.data().get("sample_mt").is_some(),
        ..Default::default()
    };

    // Step 3: Validate - Check validation results
    if let Some(validation) = message.data().get("validation_result") {
        result.validate_success = validation
            .get("valid")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if let Some(errors) = validation.get("errors").and_then(|e| e.as_array()) {
            result.validation_errors = errors
                .iter()
                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                .collect();
        }
    } else if debug_mode {
        println!("\n  WARNING: No validation_result found in message data");
    }

    // Step 4: Parse - Check if mt_json was created
    result.parse_success = message.data().get("mt_json").is_some();

    // Round-trip: Compare sample_json with mt_json
    result.round_trip_match = check_round_trip_success(message);

    // Transform is no longer a separate step
    result.transform_success = result.parse_success;

    if debug_mode && (!result.is_fully_successful()) {
        println!("\nWorkflow Step Results:");
        println!("  1. Generate: {}", status_symbol(result.generate_success));
        println!("  2. Publish: {}", status_symbol(result.publish_success));
        println!("  3. Parse: {}", status_symbol(result.parse_success));
        println!("  4. Validate: {}", status_symbol(result.validate_success));
        println!(
            "  5. Transform: {}",
            status_symbol(result.transform_success)
        );
        println!(
            "  Round-trip match: {}",
            status_symbol(result.round_trip_match)
        );

        if !result.validation_errors.is_empty() {
            println!("\n  Validation errors:");
            for error in &result.validation_errors {
                println!("    - {}", error);
            }
        }

        // Debug round-trip comparison
        if !result.round_trip_match {
            println!("\n  Round-trip comparison failed - sample_json vs mt_json differ");
        }
    }

    result
}

#[derive(Debug, Default)]
struct WorkflowResult {
    generate_success: bool,
    publish_success: bool,
    parse_success: bool,
    validate_success: bool,
    transform_success: bool,
    round_trip_match: bool,
    validation_errors: Vec<String>,
}

impl WorkflowResult {
    fn is_fully_successful(&self) -> bool {
        self.generate_success
            && self.publish_success
            && self.parse_success
            && self.validate_success
            && self.transform_success
            && self.round_trip_match
    }
}

/// Test result structure tracking each workflow step
#[derive(Debug)]
struct TestResult {
    message_type: String,
    scenario: String,
    workflow_completed: bool,
    generate_success: bool,
    publish_success: bool,
    parse_success: bool,
    validation_passed: bool,
    transform_success: bool,
    round_trip_success: bool,
    error: Option<String>,
}

impl TestResult {
    fn is_fully_successful(&self) -> bool {
        self.workflow_completed
            && self.generate_success
            && self.publish_success
            && self.parse_success
            && self.validation_passed
            && self.transform_success
            && self.round_trip_success
    }
}

/// Print test summary
fn print_test_summary(results: &[TestResult]) {
    // Group results by scenario
    let mut scenario_results: HashMap<String, Vec<&TestResult>> = HashMap::new();

    for result in results {
        let key = format!("{}/{}", result.message_type, result.scenario);
        scenario_results.entry(key).or_default().push(result);
    }

    // Sort scenarios for consistent output
    let mut sorted_scenarios: Vec<_> = scenario_results.iter().collect();
    sorted_scenarios.sort_by_key(|(key, _)| key.as_str());

    println!(
        "\n╔════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗"
    );
    println!(
        "║                                                        SWIFT MT Workflow Pipeline Test Results                                                                     ║"
    );
    println!(
        "╠═════════╤══════════════════════════════════════════════════════════╤════════╤══════════════╤═════════════╤═════════════╤══════════════╤═════════════╤══════════════╣"
    );
    println!(
        "║ Message │ Scenario                                                 │Samples │   Generate   │   Publish   │    Parse    │   Validate   │  Transform  │  Round-trip  ║"
    );
    println!(
        "╟─────────┼──────────────────────────────────────────────────────────┼────────┼──────────────┼─────────────┼─────────────┼──────────────┼─────────────┼──────────────╢"
    );

    for (scenario_key, scenario_tests) in sorted_scenarios {
        let parts: Vec<&str> = scenario_key.split('/').collect();
        let message_type = parts.first().unwrap_or(&"");
        let scenario_name = parts.get(1).unwrap_or(&"").trim_end_matches(".json"); // Remove .json extension if present

        let total = scenario_tests.len();
        let generate_pass = scenario_tests.iter().filter(|r| r.generate_success).count();
        let publish_pass = scenario_tests.iter().filter(|r| r.publish_success).count();
        let parse_pass = scenario_tests.iter().filter(|r| r.parse_success).count();
        let validation_pass = scenario_tests
            .iter()
            .filter(|r| r.validation_passed)
            .count();
        let transform_pass = scenario_tests
            .iter()
            .filter(|r| r.transform_success)
            .count();
        let roundtrip_pass = scenario_tests
            .iter()
            .filter(|r| r.round_trip_success)
            .count();

        // Format status strings with exact widths
        let generate_str = format!(
            "{:>3}/{:<3} {}",
            generate_pass,
            total,
            pass_fail_symbol(generate_pass, total)
        );
        let publish_str = format!(
            "{:>3}/{:<3} {}",
            publish_pass,
            total,
            pass_fail_symbol(publish_pass, total)
        );
        let parse_str = format!(
            "{:>3}/{:<3} {}",
            parse_pass,
            total,
            pass_fail_symbol(parse_pass, total)
        );
        let validate_str = format!(
            "{:>3}/{:<3} {}",
            validation_pass,
            total,
            pass_fail_symbol(validation_pass, total)
        );
        let transform_str = format!(
            "{:>3}/{:<3} {}",
            transform_pass,
            total,
            pass_fail_symbol(transform_pass, total)
        );
        let roundtrip_str = format!(
            "{:>3}/{:<3} {:>2}",
            roundtrip_pass,
            total,
            pass_fail_symbol(roundtrip_pass, total)
        );

        println!(
            "║ {:^7} │ {:<56} │{:^9}│ {:^10} │ {:^10} │ {:^10} │ {:^11} │ {:^10} │ {:^11} ║",
            message_type,
            scenario_name,
            total,
            generate_str,
            publish_str,
            parse_str,
            validate_str,
            transform_str,
            roundtrip_str
        );
    }

    println!(
        "╚═════════╧══════════════════════════════════════════════════════════╧═════════╧═════════════╧═════════════╧═════════════╧══════════════╧═════════════╧══════════════╝"
    );

    // Summary statistics
    let total = results.len();
    let _workflow_success = results.iter().filter(|r| r.workflow_completed).count();
    let generate_success = results.iter().filter(|r| r.generate_success).count();
    let publish_success = results.iter().filter(|r| r.publish_success).count();
    let parse_success = results.iter().filter(|r| r.parse_success).count();
    let validation_success = results.iter().filter(|r| r.validation_passed).count();
    let transform_success = results.iter().filter(|r| r.transform_success).count();
    let roundtrip_success = results.iter().filter(|r| r.round_trip_success).count();
    let fully_successful = results.iter().filter(|r| r.is_fully_successful()).count();

    println!("\n📊 Summary:");
    println!("   Total test samples: {}", total);
    println!(
        "   Fully successful: {} ({}%)",
        fully_successful,
        percentage(fully_successful, total)
    );
    println!("\n   Step Success Rates:");
    println!(
        "   1. Generate: {} ({}%)",
        generate_success,
        percentage(generate_success, total)
    );
    println!(
        "   2. Publish: {} ({}%)",
        publish_success,
        percentage(publish_success, total)
    );
    println!(
        "   3. Parse: {} ({}%)",
        parse_success,
        percentage(parse_success, total)
    );
    println!(
        "   4. Validate: {} ({}%)",
        validation_success,
        percentage(validation_success, total)
    );
    println!(
        "   5. Transform: {} ({}%)",
        transform_success,
        percentage(transform_success, total)
    );
    println!(
        "   Round-trip match: {} ({}%)",
        roundtrip_success,
        percentage(roundtrip_success, total)
    );
}

fn pass_fail_symbol(pass_count: usize, total_count: usize) -> &'static str {
    if total_count == 0 {
        "⏭️"
    } else if pass_count == total_count {
        "✅"
    } else if pass_count == 0 {
        "❌"
    } else {
        " ⚠️  "
    }
}

fn status_symbol(success: bool) -> &'static str {
    if success { "✅" } else { "❌" }
}

fn percentage(value: usize, total: usize) -> usize {
    if total == 0 { 0 } else { (value * 100) / total }
}
