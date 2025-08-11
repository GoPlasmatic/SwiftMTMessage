//! Test scenario configuration module for generating SWIFT MT messages
//!
//! This module simply loads scenario JSON files and passes them to datafake-rs

use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::{ParseError, Result};

/// Configuration for scenario file paths
#[derive(Debug, Clone)]
pub struct ScenarioConfig {
    /// Base paths to search for scenario files
    pub base_paths: Vec<PathBuf>,
}

impl Default for ScenarioConfig {
    fn default() -> Self {
        // Check environment variable first
        if let Ok(env_paths) = env::var("SWIFT_SCENARIO_PATH") {
            let paths = parse_env_paths(&env_paths);
            if !paths.is_empty() {
                return Self { base_paths: paths };
            }
        }

        // Default paths
        Self {
            base_paths: vec![
                PathBuf::from("test_scenarios"),
                PathBuf::from("../test_scenarios"),
            ],
        }
    }
}

impl ScenarioConfig {
    /// Create a new configuration with default paths
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration with specific paths
    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self { base_paths: paths }
    }

    /// Add a path to the configuration
    pub fn add_path(mut self, path: PathBuf) -> Self {
        self.base_paths.push(path);
        self
    }

    /// Clear all paths and set new ones
    pub fn set_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.base_paths = paths;
        self
    }
}

/// Parse environment variable paths
fn parse_env_paths(env_value: &str) -> Vec<PathBuf> {
    // Use OS-specific path separator
    #[cfg(windows)]
    let separator = ';';
    #[cfg(not(windows))]
    let separator = ':';

    env_value
        .split(separator)
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect()
}

/// Load a scenario configuration from a JSON file
pub fn load_scenario_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let content = fs::read_to_string(path).map_err(|e| ParseError::InvalidFormat {
        message: format!("Failed to read scenario file: {e}"),
    })?;

    serde_json::from_str(&content).map_err(|e| ParseError::InvalidFormat {
        message: format!("Failed to parse scenario JSON: {e}"),
    })
}

/// Find and load a scenario for a specific message type with custom configuration
///
/// Looks for configuration files in the following order:
/// 1. {base_path}/{message_type}/standard.json
/// 2. {base_path}/{message_type}/default.json
/// 3. First .json file in {base_path}/{message_type}/
pub fn find_scenario_for_message_type_with_config(
    message_type: &str,
    config: &ScenarioConfig,
) -> Result<Value> {
    for base_path in &config.base_paths {
        let mt_dir = base_path.join(message_type.to_lowercase());

        // Check if directory exists
        if !mt_dir.exists() {
            continue;
        }

        // Try standard.json first
        let standard_path = mt_dir.join("standard.json");
        if standard_path.exists() {
            return load_scenario_json(standard_path);
        }

        // Try default.json
        let default_path = mt_dir.join("default.json");
        if default_path.exists() {
            return load_scenario_json(default_path);
        }

        // Find the first .json file
        if let Ok(entries) = fs::read_dir(&mt_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    return load_scenario_json(path);
                }
            }
        }
    }

    let searched_paths: Vec<String> = config
        .base_paths
        .iter()
        .map(|p| format!("{}/{}", p.display(), message_type.to_lowercase()))
        .collect();

    Err(ParseError::InvalidFormat {
        message: format!(
            "No test scenarios found for message type: {}. Searched in: {}",
            message_type.to_lowercase(),
            searched_paths.join(", ")
        ),
    })
}

/// Find and load a scenario for a specific message type (backward compatibility)
///
/// Looks for configuration files in the following order:
/// 1. test_scenarios/{message_type}/standard.json
/// 2. test_scenarios/{message_type}/default.json
/// 3. First .json file in test_scenarios/{message_type}/
pub fn find_scenario_for_message_type(message_type: &str) -> Result<Value> {
    find_scenario_for_message_type_with_config(message_type, &ScenarioConfig::default())
}

/// Find and load a specific scenario by name with custom configuration
pub fn find_scenario_by_name_with_config(
    message_type: &str,
    scenario_name: &str,
    config: &ScenarioConfig,
) -> Result<Value> {
    for base_path in &config.base_paths {
        let scenario_path = base_path
            .join(message_type.to_lowercase())
            .join(format!("{scenario_name}.json"));

        if scenario_path.exists() {
            return load_scenario_json(scenario_path);
        }
    }

    let tried_paths: Vec<String> = config
        .base_paths
        .iter()
        .map(|p| {
            format!(
                "{}/{}/{}.json",
                p.display(),
                message_type.to_lowercase(),
                scenario_name
            )
        })
        .collect();

    Err(ParseError::InvalidFormat {
        message: format!(
            "Scenario '{}' not found for {}. Tried paths: {}",
            scenario_name,
            message_type,
            tried_paths.join(", ")
        ),
    })
}

/// Find and load a specific scenario by name (backward compatibility)
pub fn find_scenario_by_name(message_type: &str, scenario_name: &str) -> Result<Value> {
    find_scenario_by_name_with_config(message_type, scenario_name, &ScenarioConfig::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scenario_config_default() {
        let config = ScenarioConfig::default();
        assert_eq!(config.base_paths.len(), 2);
        assert_eq!(config.base_paths[0], PathBuf::from("test_scenarios"));
        assert_eq!(config.base_paths[1], PathBuf::from("../test_scenarios"));
    }

    #[test]
    fn test_scenario_config_with_paths() {
        let paths = vec![
            PathBuf::from("/custom/path1"),
            PathBuf::from("/custom/path2"),
        ];
        let config = ScenarioConfig::with_paths(paths.clone());
        assert_eq!(config.base_paths, paths);
    }

    #[test]
    fn test_scenario_config_add_path() {
        let config = ScenarioConfig::new()
            .add_path(PathBuf::from("/path1"))
            .add_path(PathBuf::from("/path2"));
        assert!(config.base_paths.contains(&PathBuf::from("/path1")));
        assert!(config.base_paths.contains(&PathBuf::from("/path2")));
    }

    #[test]
    fn test_scenario_config_set_paths() {
        let config = ScenarioConfig::new()
            .add_path(PathBuf::from("/old"))
            .set_paths(vec![PathBuf::from("/new1"), PathBuf::from("/new2")]);
        assert_eq!(config.base_paths.len(), 2);
        assert!(!config.base_paths.contains(&PathBuf::from("/old")));
        assert!(config.base_paths.contains(&PathBuf::from("/new1")));
        assert!(config.base_paths.contains(&PathBuf::from("/new2")));
    }

    #[test]
    fn test_parse_env_paths_unix() {
        #[cfg(not(windows))]
        {
            let paths = parse_env_paths("/path1:/path2:/path3");
            assert_eq!(paths.len(), 3);
            assert_eq!(paths[0], PathBuf::from("/path1"));
            assert_eq!(paths[1], PathBuf::from("/path2"));
            assert_eq!(paths[2], PathBuf::from("/path3"));
        }
    }

    #[test]
    fn test_parse_env_paths_empty_segments() {
        #[cfg(not(windows))]
        {
            let paths = parse_env_paths("/path1::/path2:");
            assert_eq!(paths.len(), 2);
            assert_eq!(paths[0], PathBuf::from("/path1"));
            assert_eq!(paths[1], PathBuf::from("/path2"));
        }
    }

    #[test]
    fn test_find_scenario_with_custom_config() {
        // Create a temporary directory for test scenarios
        let temp_dir = TempDir::new().unwrap();
        let mt103_dir = temp_dir.path().join("mt103");
        fs::create_dir(&mt103_dir).unwrap();

        // Create a test scenario file
        let scenario_json = r#"{
            "basic_header": {
                "app_id": "F",
                "service_id": "01",
                "ltp": "BANKUS33XXXX",
                "session_number": "0001",
                "sequence_number": "000001"
            }
        }"#;

        let scenario_path = mt103_dir.join("test_scenario.json");
        fs::write(&scenario_path, scenario_json).unwrap();

        // Test with custom config
        let config = ScenarioConfig::with_paths(vec![temp_dir.path().to_path_buf()]);

        // This should find our test scenario
        let result = find_scenario_for_message_type_with_config("MT103", &config);
        assert!(result.is_ok());

        // Test finding by name
        let result = find_scenario_by_name_with_config("MT103", "test_scenario", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scenario_not_found_error() {
        let config = ScenarioConfig::with_paths(vec![PathBuf::from("/nonexistent/path")]);

        let result = find_scenario_for_message_type_with_config("MT999", &config);
        assert!(result.is_err());

        if let Err(e) = result {
            let error_msg = format!("{:?}", e);
            assert!(error_msg.contains("No test scenarios found"));
            assert!(error_msg.contains("mt999"));
        }
    }
}
