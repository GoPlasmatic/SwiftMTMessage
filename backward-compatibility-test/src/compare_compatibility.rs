use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "compare-compatibility")]
#[command(
    about = "Compare JSON outputs between old and new versions to detect compatibility issues"
)]
struct Args {
    /// Path to old version JSON files
    #[arg(long, default_value = "output/old_version")]
    old_dir: PathBuf,

    /// Path to new version JSON files
    #[arg(long, default_value = "output/new_version")]
    new_dir: PathBuf,

    /// Generate detailed comparison report
    #[arg(long)]
    detailed: bool,

    /// Only show breaking changes
    #[arg(long)]
    breaking_only: bool,

    /// Output report to file
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct ComparisonResult {
    file_name: String,
    status: ComparisonStatus,
    differences: Vec<Difference>,
}

#[derive(Debug, Clone)]
enum ComparisonStatus {
    Identical,
    Compatible,     // New fields added, but no fields removed or changed
    BreakingChange, // Fields removed or changed types
    ParseError(String),
}

#[derive(Debug, Clone)]
struct Difference {
    path: String,
    diff_type: DifferenceType,
    old_value: Option<Value>,
    new_value: Option<Value>,
    severity: Severity,
}

#[derive(Debug, Clone)]
enum DifferenceType {
    FieldAdded,
    FieldRemoved,
    TypeChanged,
    ValueChanged,
}

#[derive(Debug, Clone)]
enum Severity {
    Info,     // New fields added (backward compatible)
    Warning,  // Value changes that might affect behavior
    Breaking, // Structure changes that break compatibility
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", "🔍 Backward Compatibility Analysis".blue().bold());
    println!("Old version: {}", args.old_dir.display());
    println!("New version: {}", args.new_dir.display());
    println!();

    let mut results = Vec::new();
    let mut total_files = 0;
    let mut identical_files = 0;
    let mut compatible_files = 0;
    let mut breaking_files = 0;
    let mut error_files = 0;

    // Get all JSON files from old directory
    let old_files: HashSet<String> = get_json_files(&args.old_dir)?
        .into_iter()
        .map(|p| p.file_stem().unwrap().to_string_lossy().to_string())
        .collect();

    let new_files: HashSet<String> = get_json_files(&args.new_dir)?
        .into_iter()
        .map(|p| p.file_stem().unwrap().to_string_lossy().to_string())
        .collect();

    // Check for missing files
    for file in &old_files {
        if !new_files.contains(file) {
            println!("{} File missing in new version: {}", "❌".red(), file);
            breaking_files += 1;
        }
    }

    for file in &new_files {
        if !old_files.contains(file) {
            println!("{} New file in new version: {}", "➕".green(), file);
        }
    }

    // Compare common files
    for file_name in old_files.intersection(&new_files) {
        total_files += 1;

        let old_path = args.old_dir.join(format!("{}.json", file_name));
        let new_path = args.new_dir.join(format!("{}.json", file_name));

        match compare_files(&old_path, &new_path, file_name) {
            Ok(result) => {
                match &result.status {
                    ComparisonStatus::Identical => {
                        identical_files += 1;
                        if !args.breaking_only {
                            println!("{} {} - Identical", "✅".green(), file_name);
                        }
                    }
                    ComparisonStatus::Compatible => {
                        compatible_files += 1;
                        if !args.breaking_only {
                            println!(
                                "{} {} - Compatible (new fields added)",
                                "🟢".green(),
                                file_name
                            );
                        }
                    }
                    ComparisonStatus::BreakingChange => {
                        breaking_files += 1;
                        println!("{} {} - Breaking changes detected", "🔴".red(), file_name);
                    }
                    ComparisonStatus::ParseError(err) => {
                        error_files += 1;
                        println!("{} {} - Parse error: {}", "⚠️".yellow(), file_name, err);
                    }
                }

                if args.detailed && !result.differences.is_empty() {
                    for diff in &result.differences {
                        print_difference(diff);
                    }
                    println!();
                }

                results.push(result);
            }
            Err(e) => {
                error_files += 1;
                println!("{} {} - Error: {}", "❌".red(), file_name, e);
            }
        }
    }

    println!();
    println!("{}", "📊 Summary".blue().bold());
    println!("Total files compared: {}", total_files);
    println!("{} Identical: {}", "✅".green(), identical_files);
    println!("{} Compatible: {}", "🟢".green(), compatible_files);
    println!("{} Breaking changes: {}", "🔴".red(), breaking_files);
    println!("{} Errors: {}", "⚠️".yellow(), error_files);

    let compatibility_percentage = if total_files > 0 {
        ((identical_files + compatible_files) as f64 / total_files as f64) * 100.0
    } else {
        100.0
    };

    println!("Compatibility: {:.1}%", compatibility_percentage);

    if breaking_files > 0 {
        println!();
        println!("{}", "⚠️  BREAKING CHANGES DETECTED!".red().bold());
        println!("Please review the changes above before releasing.");
        std::process::exit(1);
    } else {
        println!();
        println!("{}", "✅ No breaking changes detected!".green().bold());
    }

    // Generate report if requested
    if let Some(output_path) = args.output {
        generate_report(&results, &output_path)?;
        println!("Report written to: {}", output_path.display());
    }

    Ok(())
}

fn get_json_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

fn compare_files(old_path: &Path, new_path: &Path, file_name: &str) -> Result<ComparisonResult> {
    let old_content = fs::read_to_string(old_path)
        .with_context(|| format!("Failed to read old file: {}", old_path.display()))?;

    let new_content = fs::read_to_string(new_path)
        .with_context(|| format!("Failed to read new file: {}", new_path.display()))?;

    let old_json: Value = match serde_json::from_str(&old_content) {
        Ok(json) => json,
        Err(e) => {
            return Ok(ComparisonResult {
                file_name: file_name.to_string(),
                status: ComparisonStatus::ParseError(format!("Old file parse error: {}", e)),
                differences: Vec::new(),
            });
        }
    };

    let new_json: Value = match serde_json::from_str(&new_content) {
        Ok(json) => json,
        Err(e) => {
            return Ok(ComparisonResult {
                file_name: file_name.to_string(),
                status: ComparisonStatus::ParseError(format!("New file parse error: {}", e)),
                differences: Vec::new(),
            });
        }
    };

    let differences = compare_values(&old_json, &new_json, String::new());

    let status = determine_status(&differences);

    Ok(ComparisonResult {
        file_name: file_name.to_string(),
        status,
        differences,
    })
}

fn compare_values(old: &Value, new: &Value, path: String) -> Vec<Difference> {
    let mut differences = Vec::new();

    if old == new {
        return differences;
    }

    match (old, new) {
        (Value::Object(old_map), Value::Object(new_map)) => {
            // Check for removed fields
            for (key, old_value) in old_map {
                let current_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                if let Some(new_value) = new_map.get(key) {
                    differences.extend(compare_values(old_value, new_value, current_path));
                } else {
                    // Check if the removed field was null - if so, it's compatible, not breaking
                    let severity = if old_value.is_null() {
                        Severity::Info // Removing a null field is compatible
                    } else {
                        Severity::Breaking // Removing a field with actual value is breaking
                    };

                    differences.push(Difference {
                        path: current_path,
                        diff_type: DifferenceType::FieldRemoved,
                        old_value: Some(old_value.clone()),
                        new_value: None,
                        severity,
                    });
                }
            }

            // Check for added fields
            for (key, new_value) in new_map {
                if !old_map.contains_key(key) {
                    let current_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };

                    differences.push(Difference {
                        path: current_path,
                        diff_type: DifferenceType::FieldAdded,
                        old_value: None,
                        new_value: Some(new_value.clone()),
                        severity: Severity::Info,
                    });
                }
            }
        }
        (Value::Array(old_arr), Value::Array(new_arr)) => {
            // For arrays, we'll do a simple length and element comparison
            if old_arr.len() != new_arr.len() {
                differences.push(Difference {
                    path: format!("{}.length", path),
                    diff_type: DifferenceType::ValueChanged,
                    old_value: Some(Value::Number(old_arr.len().into())),
                    new_value: Some(Value::Number(new_arr.len().into())),
                    severity: Severity::Warning,
                });
            }

            for (i, (old_item, new_item)) in old_arr.iter().zip(new_arr.iter()).enumerate() {
                let current_path = format!("{}[{}]", path, i);
                differences.extend(compare_values(old_item, new_item, current_path));
            }
        }
        _ => {
            // Different types or values
            let severity = if old.is_null() || new.is_null() {
                Severity::Warning
            } else if std::mem::discriminant(old) != std::mem::discriminant(new) {
                Severity::Breaking
            } else {
                Severity::Warning
            };

            differences.push(Difference {
                path,
                diff_type: if std::mem::discriminant(old) != std::mem::discriminant(new) {
                    DifferenceType::TypeChanged
                } else {
                    DifferenceType::ValueChanged
                },
                old_value: Some(old.clone()),
                new_value: Some(new.clone()),
                severity,
            });
        }
    }

    differences
}

fn determine_status(differences: &[Difference]) -> ComparisonStatus {
    if differences.is_empty() {
        return ComparisonStatus::Identical;
    }

    let has_breaking = differences
        .iter()
        .any(|d| matches!(d.severity, Severity::Breaking));

    if has_breaking {
        ComparisonStatus::BreakingChange
    } else {
        ComparisonStatus::Compatible
    }
}

fn print_difference(diff: &Difference) {
    let icon = match diff.severity {
        Severity::Info => "ℹ️",
        Severity::Warning => "⚠️",
        Severity::Breaking => "🔴",
    };

    let color = match diff.severity {
        Severity::Info => "blue",
        Severity::Warning => "yellow",
        Severity::Breaking => "red",
    };

    // Special handling for null field removals
    let description = match (&diff.diff_type, &diff.old_value) {
        (DifferenceType::FieldRemoved, Some(Value::Null)) => {
            format!("{:?} (null field - compatible)", diff.diff_type)
        }
        _ => format!("{:?}", diff.diff_type),
    };

    println!(
        "  {} {} at {}",
        icon,
        description.color(color),
        diff.path
    );

    if let Some(old_val) = &diff.old_value {
        println!("    Old: {}", format_value_compact(old_val));
    }

    if let Some(new_val) = &diff.new_value {
        println!("    New: {}", format_value_compact(new_val));
    }
}

fn format_value_compact(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(_) => format!("[array with {} items]", value.as_array().unwrap().len()),
        Value::Object(_) => format!(
            "{{object with {} fields}}",
            value.as_object().unwrap().len()
        ),
    }
}

fn generate_report(results: &[ComparisonResult], output_path: &Path) -> Result<()> {
    let mut report = String::new();

    report.push_str("# Backward Compatibility Report\n\n");

    let mut identical = 0;
    let mut compatible = 0;
    let mut breaking = 0;
    let mut errors = 0;

    for result in results {
        match result.status {
            ComparisonStatus::Identical => identical += 1,
            ComparisonStatus::Compatible => compatible += 1,
            ComparisonStatus::BreakingChange => breaking += 1,
            ComparisonStatus::ParseError(_) => errors += 1,
        }
    }

    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total files: {}\n", results.len()));
    report.push_str(&format!("- Identical: {}\n", identical));
    report.push_str(&format!("- Compatible: {}\n", compatible));
    report.push_str(&format!("- Breaking changes: {}\n", breaking));
    report.push_str(&format!("- Errors: {}\n\n", errors));

    if breaking > 0 {
        report.push_str("## ⚠️ Breaking Changes Detected\n\n");

        for result in results {
            if matches!(result.status, ComparisonStatus::BreakingChange) {
                report.push_str(&format!("### {}\n\n", result.file_name));

                for diff in &result.differences {
                    if matches!(diff.severity, Severity::Breaking) {
                        let description = match (&diff.diff_type, &diff.old_value) {
                            (DifferenceType::FieldRemoved, Some(Value::Null)) => {
                                format!("{:?} (null field - compatible)", diff.diff_type)
                            }
                            _ => format!("{:?}", diff.diff_type),
                        };

                        report
                            .push_str(&format!("- **{}** at `{}`\n", description, diff.path));

                        if let Some(old_val) = &diff.old_value {
                            report.push_str(&format!(
                                "  - Old: `{}`\n",
                                format_value_compact(old_val)
                            ));
                        }

                        if let Some(new_val) = &diff.new_value {
                            report.push_str(&format!(
                                "  - New: `{}`\n",
                                format_value_compact(new_val)
                            ));
                        }

                        report.push('\n');
                    }
                }
            }
        }

        // Add a section for compatible null field removals if any exist
        let mut has_null_removals = false;
        for result in results {
            for diff in &result.differences {
                if matches!(diff.diff_type, DifferenceType::FieldRemoved) 
                    && matches!(diff.old_value, Some(Value::Null))
                    && matches!(diff.severity, Severity::Info) {
                    if !has_null_removals {
                        report.push_str("## ℹ️ Compatible Changes (Null Field Removals)\n\n");
                        has_null_removals = true;
                    }
                    report.push_str(&format!("- **{}**: `{}` (was null)\n", result.file_name, diff.path));
                }
            }
        }
        
        if has_null_removals {
            report.push_str("\nThese null field removals are considered compatible changes.\n\n");
        }
    }

    fs::write(output_path, report)
        .with_context(|| format!("Failed to write report to {}", output_path.display()))?;

    Ok(())
}
