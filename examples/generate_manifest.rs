//! Generate JSON Schemas and Plugin Manifest for SwiftMTMessage
//!
//! This example generates:
//! 1. JSON Schema files for all supported SWIFT MT message types
//! 2. A manifest.json containing plugin metadata extracted from the schemas
//!
//! Run with: cargo run --example generate_manifest --features jsonschema

use schemars::schema_for;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use swift_mt_message::{
    MT101, MT103, MT104, MT107, MT110, MT111, MT112, MT190, MT191, MT192, MT196, MT199, MT200,
    MT202, MT204, MT205, MT210, MT290, MT291, MT292, MT296, MT299, MT900, MT910, MT920, MT935,
    MT940, MT941, MT942, MT950, SwiftMessage,
};

/// Plugin manifest structure
#[derive(Serialize)]
struct Manifest {
    name: String,
    version: String,
    description: String,
    repository: String,
    license: String,
    authors: Vec<String>,
    supported_messages: Vec<MessageInfo>,
}

/// Information about a supported message type
#[derive(Serialize)]
struct MessageInfo {
    #[serde(rename = "type")]
    message_type: String,
    title: String,
    description: String,
    category: String,
    schema_url: String,
}

/// Category mapping from schema format to clean names
fn map_category(raw_category: &str) -> &'static str {
    if raw_category.contains("1") || raw_category.to_lowercase().contains("customer payment") {
        "Customer Payments & Cheques"
    } else if raw_category.contains("2")
        || raw_category
            .to_lowercase()
            .contains("financial institution")
    {
        "Financial Institution Transfers"
    } else if raw_category.contains("9") || raw_category.to_lowercase().contains("cash management")
    {
        "Cash Management & Customer Status"
    } else {
        "Other"
    }
}

/// Extract category from schema description
/// Format: "...**Category:** Category 1 (Customer Payments)"
fn parse_category(description: &str) -> Option<String> {
    // Find **Category:** pattern
    let marker = "**Category:**";
    let start = description.find(marker)?;
    let after_marker = &description[start + marker.len()..];
    // Take until end of line or end of string
    let category = after_marker.lines().next().unwrap_or(after_marker).trim();
    Some(map_category(category).to_string())
}

/// Extract first sentence from description (skipping the title line)
fn extract_first_sentence(description: &str) -> String {
    description
        .lines()
        .skip(1)
        .find(|l| !l.trim().is_empty())
        .unwrap_or("")
        .trim()
        .to_string()
}

/// Extract message info from a schema file
/// Returns (title, description, category)
fn extract_message_info(
    schema_path: &Path,
    message_type: &str,
) -> Option<(String, String, String)> {
    let content = fs::read_to_string(schema_path).ok()?;
    let schema: Value = serde_json::from_str(&content).ok()?;

    // Read from root-level title and description (set by generate_schema)
    let title = schema.get("title")?.as_str()?;
    let description = schema.get("description")?.as_str()?;

    // Navigate to definitions -> MTxxx -> description for category
    let def_description = schema
        .get("definitions")?
        .get(message_type)?
        .get("description")?
        .as_str()?;

    let category = parse_category(def_description).unwrap_or_else(|| "Other".to_string());

    Some((title.to_string(), description.to_string(), category))
}

fn generate_schema<T: schemars::JsonSchema>(name: &str, dir: &Path) -> std::io::Result<()> {
    let schema = schema_for!(T);
    let mut json: Value = serde_json::to_value(&schema)?;

    // Extract info from definitions and update root title/description
    if let Some(desc) = json
        .pointer(&format!("/definitions/{}/description", name))
        .and_then(|v| v.as_str())
    {
        let first_line = desc.lines().next().unwrap_or("");
        let title = first_line.replace("**", "");
        let first_sentence = extract_first_sentence(desc);

        json["title"] = Value::String(title);
        json["description"] = Value::String(first_sentence);
    }

    let output = serde_json::to_string_pretty(&json)?;
    let path = dir.join(format!("{}.schema.json", name));
    fs::write(&path, output)?;
    println!("  Generated: {}", path.display());
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Read version from environment or use default
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "3.1.4".to_string());

    // Allow overriding version via command line argument
    let args: Vec<String> = std::env::args().collect();
    let version = if args.len() > 1 {
        args[1].clone()
    } else {
        version
    };

    let output_dir = Path::new("schemas");
    fs::create_dir_all(output_dir)?;

    println!(
        "Generating JSON Schemas and Manifest for SwiftMTMessage v{}...\n",
        version
    );

    // List of message types to generate
    let message_types = [
        "MT101", "MT103", "MT104", "MT107", "MT110", "MT111", "MT112", "MT190", "MT191", "MT192",
        "MT196", "MT199", "MT200", "MT202", "MT204", "MT205", "MT210", "MT290", "MT291", "MT292",
        "MT296", "MT299", "MT900", "MT910", "MT920", "MT935", "MT940", "MT941", "MT942", "MT950",
    ];

    // Step 1: Generate all JSON schemas
    println!("Step 1: Generating JSON Schemas...");
    generate_schema::<SwiftMessage<MT101>>("MT101", output_dir)?;
    generate_schema::<SwiftMessage<MT103>>("MT103", output_dir)?;
    generate_schema::<SwiftMessage<MT104>>("MT104", output_dir)?;
    generate_schema::<SwiftMessage<MT107>>("MT107", output_dir)?;
    generate_schema::<SwiftMessage<MT110>>("MT110", output_dir)?;
    generate_schema::<SwiftMessage<MT111>>("MT111", output_dir)?;
    generate_schema::<SwiftMessage<MT112>>("MT112", output_dir)?;
    generate_schema::<SwiftMessage<MT190>>("MT190", output_dir)?;
    generate_schema::<SwiftMessage<MT191>>("MT191", output_dir)?;
    generate_schema::<SwiftMessage<MT192>>("MT192", output_dir)?;
    generate_schema::<SwiftMessage<MT196>>("MT196", output_dir)?;
    generate_schema::<SwiftMessage<MT199>>("MT199", output_dir)?;
    generate_schema::<SwiftMessage<MT200>>("MT200", output_dir)?;
    generate_schema::<SwiftMessage<MT202>>("MT202", output_dir)?;
    generate_schema::<SwiftMessage<MT204>>("MT204", output_dir)?;
    generate_schema::<SwiftMessage<MT205>>("MT205", output_dir)?;
    generate_schema::<SwiftMessage<MT210>>("MT210", output_dir)?;
    generate_schema::<SwiftMessage<MT290>>("MT290", output_dir)?;
    generate_schema::<SwiftMessage<MT291>>("MT291", output_dir)?;
    generate_schema::<SwiftMessage<MT292>>("MT292", output_dir)?;
    generate_schema::<SwiftMessage<MT296>>("MT296", output_dir)?;
    generate_schema::<SwiftMessage<MT299>>("MT299", output_dir)?;
    generate_schema::<SwiftMessage<MT900>>("MT900", output_dir)?;
    generate_schema::<SwiftMessage<MT910>>("MT910", output_dir)?;
    generate_schema::<SwiftMessage<MT920>>("MT920", output_dir)?;
    generate_schema::<SwiftMessage<MT935>>("MT935", output_dir)?;
    generate_schema::<SwiftMessage<MT940>>("MT940", output_dir)?;
    generate_schema::<SwiftMessage<MT941>>("MT941", output_dir)?;
    generate_schema::<SwiftMessage<MT942>>("MT942", output_dir)?;
    generate_schema::<SwiftMessage<MT950>>("MT950", output_dir)?;

    // Step 2: Extract metadata from generated schemas
    println!("\nStep 2: Extracting metadata from schemas...");
    let mut supported_messages = Vec::new();

    for msg_type in &message_types {
        let schema_path = output_dir.join(format!("{}.schema.json", msg_type));
        let (title, description, category) = extract_message_info(&schema_path, msg_type)
            .unwrap_or_else(|| {
                (
                    msg_type.to_string(),
                    msg_type.to_string(),
                    "Other".to_string(),
                )
            });

        println!("  {}: {} ({})", msg_type, title, category);

        supported_messages.push(MessageInfo {
            message_type: msg_type.to_string(),
            title,
            description,
            category,
            schema_url: format!(
                "https://github.com/GoPlasmatic/SwiftMTMessage/releases/download/v{}/{}.schema.json",
                version, msg_type
            ),
        });
    }

    // Step 3: Generate manifest.json
    println!("\nStep 3: Generating manifest.json...");
    let manifest = Manifest {
        name: "swift-mt-message".to_string(),
        version: version.clone(),
        description: "A fast, type-safe Rust implementation of SWIFT MT message parsing with comprehensive field support, derive macros, and validation.".to_string(),
        repository: "https://github.com/GoPlasmatic/SwiftMTMessage".to_string(),
        license: "Apache-2.0".to_string(),
        authors: vec!["Plasmatic Engineering <shankar@goplasmatic.io>".to_string()],
        supported_messages,
    };

    let json = serde_json::to_string_pretty(&manifest)?;
    let manifest_path = output_dir.join("manifest.json");
    fs::write(&manifest_path, &json)?;

    // Display summary grouped by category
    println!(
        "\nGenerated manifest with {} message types:",
        message_types.len()
    );
    let mut categories: HashMap<&str, Vec<&str>> = HashMap::new();
    for msg in &manifest.supported_messages {
        categories
            .entry(msg.category.as_str())
            .or_default()
            .push(&msg.message_type);
    }

    for (category, types) in &categories {
        println!("  {}: {}", category, types.join(", "));
    }

    println!("\nSchemas written to: {}/", output_dir.display());
    println!("Manifest written to: {}", manifest_path.display());
    Ok(())
}
