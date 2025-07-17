//! Validation rules constants for SWIFT MT messages

/// Validation rules for MT103 messages
pub const MT103_VALIDATION_RULES: &str = r#"{
    "rules": [
        {
            "id": "MT103_BASIC",
            "description": "Basic MT103 validation",
            "condition": true
        }
    ]
}"#;

/// Validation rules for MT202 messages
pub const MT202_VALIDATION_RULES: &str = r#"{
    "rules": [
        {
            "id": "MT202_BASIC", 
            "description": "Basic MT202 validation",
            "condition": true
        }
    ]
}"#;

/// Default validation rules for any message type
pub const DEFAULT_VALIDATION_RULES: &str = r#"{
    "rules": [
        {
            "id": "DEFAULT_BASIC",
            "description": "Basic validation",
            "condition": true
        }
    ]
}"#;