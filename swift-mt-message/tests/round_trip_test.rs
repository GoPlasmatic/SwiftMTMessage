use std::fs;
use std::path::Path;
use swift_mt_message::{ParsedSwiftMessage, SwiftParser};

#[derive(Debug)]
struct TestResult {
    filename: String,
    parse_status: &'static str,
    validation_status: &'static str,
    roundtrip_status: &'static str,
    error_stage: Option<String>,
}

impl TestResult {
    fn new(filename: String) -> Self {
        Self {
            filename,
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

#[test]
fn test_round_trip_all_files() {
    let test_data_dir = Path::new("../test_data");
    if !test_data_dir.exists() {
        panic!(
            "test_data directory not found at: {:?}",
            test_data_dir.canonicalize()
        );
    }

    let mut test_results = Vec::new();

    // Collect all test files
    let entries = fs::read_dir(test_data_dir).expect("Failed to read test_data directory");
    let mut test_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("txt")) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Sort files for consistent output
    test_files.sort();

    if test_files.is_empty() {
        panic!("No test files found in test_data directory");
    }

    // Process each file
    for file_path in test_files {
        let filename = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut result = TestResult::new(filename);

        // Read file
        let original_content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                result.mark_parse_failed(format!("Read error: {e}"));
                test_results.push(result);
                continue;
            }
        };

        // Parse message
        let parsed_message = match SwiftParser::parse_auto(&original_content) {
            Ok(msg) => {
                result.mark_parse_success();
                msg
            }
            Err(e) => {
                result.mark_parse_failed(
                    format!("{e:?}")
                        .split('\n')
                        .next()
                        .unwrap_or("Unknown")
                        .to_string(),
                );
                test_results.push(result);
                continue;
            }
        };

        // Validate message
        let validation_result = parsed_message.validate();
        if !validation_result.errors.is_empty() {
            let error_summaries: Vec<String> = validation_result
                .errors
                .iter()
                .map(|e| format!("{e}"))
                .collect();
            result.mark_validation_failed(error_summaries);
        } else {
            result.mark_validation_success();
        }

        // Perform round-trip test
        // Step 1: Serialize to JSON
        let json_representation = match serde_json::to_string_pretty(&parsed_message) {
            Ok(json) => json,
            Err(e) => {
                result.mark_roundtrip_failed(&format!("JSON serialize: {e}"));
                test_results.push(result);
                continue;
            }
        };

        // Step 2: Deserialize from JSON
        let deserialized_message: ParsedSwiftMessage =
            match serde_json::from_str(&json_representation) {
                Ok(msg) => msg,
                Err(e) => {
                    result.mark_roundtrip_failed(&format!("JSON deserialize: {e}"));
                    test_results.push(result);
                    continue;
                }
            };

        // Step 3: Regenerate MT message
        let regenerated_mt = match &deserialized_message {
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

        // Step 4: Reparse regenerated MT
        let reparsed_message = match SwiftParser::parse_auto(&regenerated_mt) {
            Ok(msg) => msg,
            Err(e) => {
                result.mark_roundtrip_failed(
                    format!("Reparse: {e:?}")
                        .split('\n')
                        .next()
                        .unwrap_or("Unknown"),
                );
                test_results.push(result);
                continue;
            }
        };

        // Step 5: Compare JSON representations
        let original_json = match serde_json::to_string_pretty(&parsed_message) {
            Ok(json) => json,
            Err(e) => {
                result.mark_roundtrip_failed(&format!("Original JSON: {e}"));
                test_results.push(result);
                continue;
            }
        };

        let reparsed_json = match serde_json::to_string_pretty(&reparsed_message) {
            Ok(json) => json,
            Err(e) => {
                result.mark_roundtrip_failed(&format!("Reparsed JSON: {e}"));
                test_results.push(result);
                continue;
            }
        };

        if original_json == reparsed_json {
            result.mark_roundtrip_success();
        } else {
            result.mark_roundtrip_failed("JSON mismatch");

            // If DEBUG_ROUNDTRIP is set, save the diff for inspection
            if std::env::var("DEBUG_ROUNDTRIP").is_ok() {
                let debug_dir = Path::new("target/roundtrip_debug");
                fs::create_dir_all(debug_dir).ok();

                let base_name = file_path.file_stem().unwrap_or_default().to_string_lossy();
                fs::write(
                    debug_dir.join(format!("{base_name}_original.json")),
                    &original_json,
                )
                .ok();
                fs::write(
                    debug_dir.join(format!("{base_name}_reparsed.json")),
                    &reparsed_json,
                )
                .ok();
            }
        }

        test_results.push(result);
    }

    // Print results table
    print_results_table(&test_results);

    // Determine if test should fail
    let total_files = test_results.len();
    let failed_files: Vec<_> = test_results
        .iter()
        .filter(|r| {
            r.parse_status == "❌" || r.validation_status == "❌" || r.roundtrip_status == "❌"
        })
        .collect();

    if !failed_files.is_empty() {
        println!("\n❌ Failed files ({}):", failed_files.len());
        for result in &failed_files {
            if let Some(ref error) = result.error_stage {
                println!("   {} - {}", result.filename, error);
            }
        }
        println!("\n💡 To debug a specific file, run:");
        println!("   cargo run --example parse_auto test_data/<filename>");

        panic!(
            "Round-trip test failed: {} out of {} files failed",
            failed_files.len(),
            total_files
        );
    }
}

fn print_results_table(results: &[TestResult]) {
    // Calculate column widths
    let filename_width = results
        .iter()
        .map(|r| r.filename.len())
        .max()
        .unwrap_or(8)
        .max(8);

    // Print header
    println!(
        "\n╔{}╤{}╤{}╤{}╗",
        "═".repeat(filename_width + 2),
        "═".repeat(10),
        "═".repeat(13),
        "═".repeat(11)
    );
    println!(
        "║ {:<width$} │ {:^8} │ {:^11} │ {:^9} ║",
        "Filename",
        "Parser",
        "Validation",
        "Roundtrip",
        width = filename_width
    );
    println!(
        "╟{}┼{}┼{}┼{}╢",
        "─".repeat(filename_width + 2),
        "─".repeat(10),
        "─".repeat(13),
        "─".repeat(11)
    );

    // Print rows
    for result in results {
        println!(
            "║ {:<width$} │ {:^7} │ {:^10} │ {:^8} ║",
            result.filename,
            result.parse_status,
            result.validation_status,
            result.roundtrip_status,
            width = filename_width
        );
    }

    // Print footer
    println!(
        "╚{}╧{}╧{}╧{}╝",
        "═".repeat(filename_width + 2),
        "═".repeat(10),
        "═".repeat(13),
        "═".repeat(11)
    );

    // Print summary
    let total = results.len();
    let parse_success = results.iter().filter(|r| r.parse_status == "✅").count();
    let validation_success = results
        .iter()
        .filter(|r| r.validation_status == "✅")
        .count();
    let roundtrip_success = results
        .iter()
        .filter(|r| r.roundtrip_status == "✅")
        .count();

    println!("\n📊 Summary:");
    println!("   Total files: {total}");
    println!(
        "   Parse successful: {} ({}%)",
        parse_success,
        (parse_success * 100) / total
    );
    println!(
        "   Validation successful: {} ({}%)",
        validation_success,
        (validation_success * 100) / total
    );
    println!(
        "   Roundtrip successful: {} ({}%)",
        roundtrip_success,
        (roundtrip_success * 100) / total
    );
}
