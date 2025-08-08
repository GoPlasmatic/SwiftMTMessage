//! Sample generation functionality for SWIFT MT messages
//!
//! This module provides utilities for generating sample SWIFT MT messages
//! based on test scenarios using datafake-rs for dynamic data generation.

use crate::errors::{self, Result};
use crate::scenario_config::{
    find_scenario_by_name_with_config, find_scenario_for_message_type_with_config, ScenarioConfig,
};
use crate::swift_message::SwiftMessage;
use crate::traits::SwiftMessageBody;
use datafake_rs::DataGenerator;
use std::path::PathBuf;

/// Generate a sample SWIFT MT message based on test scenarios with custom configuration
///
/// This function loads a test scenario configuration for the specified message type
/// and generates a realistic SWIFT message using datafake-rs for dynamic data generation.
///
/// # Arguments
///
/// * `message_type` - The MT message type (e.g., "MT103", "MT202")
/// * `scenario_name` - Optional scenario name. If None, uses the default scenario
/// * `config` - Configuration for scenario file paths
///
/// # Returns
///
/// Returns a complete SwiftMessage object with headers and the message body
///
/// # Example
///
/// ```no_run
/// # use swift_mt_message::{generate_sample_with_config, ScenarioConfig, SwiftMessage, messages::mt103::MT103};
/// # use std::path::PathBuf;
/// // Generate with custom paths
/// let config = ScenarioConfig::with_paths(vec![PathBuf::from("./my_scenarios")]);
/// let mt103_msg: SwiftMessage<MT103> = generate_sample_with_config("MT103", None, &config).unwrap();
/// println!("{}", mt103_msg.to_mt_message());
/// ```
pub fn generate_sample_with_config<T>(
    message_type: &str,
    scenario_name: Option<&str>,
    config: &ScenarioConfig,
) -> Result<SwiftMessage<T>>
where
    T: SwiftMessageBody + serde::de::DeserializeOwned,
{
    // Load the scenario configuration JSON
    let scenario_json = if let Some(name) = scenario_name {
        find_scenario_by_name_with_config(message_type, name, config)?
    } else {
        find_scenario_for_message_type_with_config(message_type, config)?
    };

    // Create datafake-rs generator from the scenario
    let generator = DataGenerator::from_value(scenario_json).map_err(|e| {
        errors::ParseError::InvalidFormat {
            message: format!("Failed to create datafake generator: {e:?}"),
        }
    })?;

    // Generate the data
    let generated_data = generator
        .generate()
        .map_err(|e| errors::ParseError::InvalidFormat {
            message: format!("datafake-rs generation failed: {e:?}"),
        })?;

    // Convert generated data to string for parsing
    let generated_json = serde_json::to_string_pretty(&generated_data).map_err(|e| {
        errors::ParseError::InvalidFormat {
            message: format!("Failed to serialize generated data: {e}"),
        }
    })?;

    // Parse the generated JSON into the complete SwiftMessage
    serde_json::from_str(&generated_json).map_err(|e| errors::ParseError::InvalidFormat {
        message: format!("Failed to parse generated JSON into SwiftMessage<{message_type}>: {e}"),
    })
}

/// Generate a sample SWIFT MT message based on test scenarios (backward compatibility)
///
/// This function loads a test scenario configuration for the specified message type
/// and generates a realistic SWIFT message using datafake-rs for dynamic data generation.
///
/// # Arguments
///
/// * `message_type` - The MT message type (e.g., "MT103", "MT202")
/// * `scenario_name` - Optional scenario name. If None, uses the default scenario
///
/// # Returns
///
/// Returns a complete SwiftMessage object with headers and the message body
///
/// # Example
///
/// ```no_run
/// # use swift_mt_message::{generate_sample, SwiftMessage, messages::mt103::MT103};
/// // Generate a standard MT103 message
/// let mt103_msg: SwiftMessage<MT103> = generate_sample("MT103", None).unwrap();
/// println!("{}", mt103_msg.to_mt_message());
///
/// // Generate a specific scenario
/// let mt103_high_value: SwiftMessage<MT103> = generate_sample("MT103", Some("high_value_payment")).unwrap();
/// ```
pub fn generate_sample<T>(
    message_type: &str,
    scenario_name: Option<&str>,
) -> Result<SwiftMessage<T>>
where
    T: SwiftMessageBody + serde::de::DeserializeOwned,
{
    generate_sample_with_config(message_type, scenario_name, &ScenarioConfig::default())
}

/// A builder for generating SWIFT MT message samples with custom configuration
///
/// The `SampleGenerator` provides a fluent interface for configuring and generating
/// SWIFT MT message samples with custom scenario paths.
///
/// # Example
///
/// ```no_run
/// # use swift_mt_message::{SampleGenerator, SwiftMessage, messages::mt103::MT103};
/// # use std::path::PathBuf;
/// let generator = SampleGenerator::new()
///     .with_path(PathBuf::from("./custom_scenarios"))
///     .with_path(PathBuf::from("./backup_scenarios"));
///
/// let mt103: SwiftMessage<MT103> = generator.generate("MT103", None).unwrap();
/// let mt103_stp: SwiftMessage<MT103> = generator.generate("MT103", Some("stp")).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct SampleGenerator {
    config: ScenarioConfig,
}

impl Default for SampleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SampleGenerator {
    /// Create a new sample generator with default configuration
    pub fn new() -> Self {
        Self {
            config: ScenarioConfig::default(),
        }
    }

    /// Create a sample generator with specific configuration
    pub fn with_config(config: ScenarioConfig) -> Self {
        Self { config }
    }

    /// Add a path to search for scenario files
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.config = self.config.add_path(path);
        self
    }

    /// Set multiple paths to search for scenario files (replaces existing paths)
    pub fn with_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.config = self.config.set_paths(paths);
        self
    }

    /// Generate a sample SWIFT MT message
    ///
    /// # Arguments
    ///
    /// * `message_type` - The MT message type (e.g., "MT103", "MT202")
    /// * `scenario_name` - Optional scenario name. If None, uses the default scenario
    pub fn generate<T>(
        &self,
        message_type: &str,
        scenario_name: Option<&str>,
    ) -> Result<SwiftMessage<T>>
    where
        T: SwiftMessageBody + serde::de::DeserializeOwned,
    {
        generate_sample_with_config(message_type, scenario_name, &self.config)
    }

    /// Get a reference to the current configuration
    pub fn config(&self) -> &ScenarioConfig {
        &self.config
    }
}
