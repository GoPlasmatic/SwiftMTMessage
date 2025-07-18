use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[cfg(feature = "published")]
use swift_mt_message as swift;
#[cfg(feature = "local")]
use swift_mt_message_local as swift;

fn main() -> Result<()> {
    let test_data_dir = Path::new("../test_data");
    #[cfg(feature = "published")]
    let output_dir = Path::new("output/old_version");
    #[cfg(feature = "local")]
    let output_dir = Path::new("output/new_version");

    // Create output directory
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    println!("Generating JSON files using new version...");
    println!("Test data directory: {}", test_data_dir.display());
    println!("Output directory: {}", output_dir.display());

    // Process all .txt files in test_data directory
    for entry in WalkDir::new(test_data_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            if let Err(e) = process_file(path, output_dir) {
                eprintln!("Error processing {}: {}", path.display(), e);
                continue;
            }
        }
    }

    println!("Completed generating JSON files using new version");
    Ok(())
}

fn process_file(input_path: &Path, output_dir: &Path) -> Result<()> {
    let file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid file name")?;

    println!("Processing: {}", file_stem);

    // Read the SWIFT message
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read file: {}", input_path.display()))?;

    // Parse the message using auto-detection
    let parsed_message = swift::SwiftParser::parse_auto(&content).with_context(|| {
        format!(
            "Failed to parse SWIFT message from {}",
            input_path.display()
        )
    })?;

    // Convert to JSON
    let json =
        serde_json::to_string_pretty(&parsed_message).context("Failed to serialize to JSON")?;

    // Write to output file
    let output_path = output_dir.join(format!("{}.json", file_stem));
    fs::write(&output_path, json)
        .with_context(|| format!("Failed to write JSON to {}", output_path.display()))?;

    println!("  -> Generated: {}", output_path.display());
    Ok(())
}
