// Plugin module for dataflow-rs integration
// Exposes SWIFT MT functions as AsyncFunctionHandler implementations

pub mod generate;
pub mod parse;
pub mod publish;
pub mod validate;

use dataflow_rs::engine::AsyncFunctionHandler;

// Re-export the main plugin functions
pub use generate::Generate;
pub use parse::Parse;
pub use publish::Publish;
pub use validate::Validate;

/// Register all SWIFT MT plugin functions for use in dataflow engine
pub fn register_swift_mt_functions()
-> Vec<(&'static str, Box<dyn AsyncFunctionHandler + Send + Sync>)> {
    vec![
        ("parse_mt", Box::new(Parse)),
        ("publish_mt", Box::new(Publish)),
        ("validate_mt", Box::new(Validate)),
        ("generate_mt", Box::new(Generate)),
    ]
}
