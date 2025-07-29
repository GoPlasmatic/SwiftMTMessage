//! Sample generation functionality for SWIFT MT messages
//!
//! This module provides utilities for generating sample SWIFT MT messages
//! based on test scenarios using datafake-rs for dynamic data generation.

use crate::errors::{self, Result};
use crate::scenario_config::{find_scenario_by_name, find_scenario_for_message_type};
use crate::swift_message::SwiftMessage;
use crate::traits::SwiftMessageBody;
use datafake_rs::DataGenerator;

/// Generate a sample SWIFT MT message based on test scenarios
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
    // Load the scenario configuration JSON
    let scenario_json = if let Some(name) = scenario_name {
        find_scenario_by_name(message_type, name)?
    } else {
        find_scenario_for_message_type(message_type)?
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
