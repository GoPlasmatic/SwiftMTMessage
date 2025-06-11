# SWIFT Configuration System

This directory contains JSON configuration files for SWIFT message field validation rules and mandatory field definitions.

## Current Configuration Files

### mandatory_fields.json
Defines which fields are mandatory for each message type:

```json
{
  "message_types": {
    "103": ["20", "23B", "32A", "50", "59", "71A"],
    "102": ["20", "23B", "32A", "50", "71A"],
    "202": ["20", "32A", "52A", "58A"],
    "199": ["20", "79"],
    "192": ["20", "21"],
    "195": ["20", "21"],
    "196": ["20", "21"],
    "197": ["20", "21"],
    "940": ["20", "25", "28C", "60F", "62F"],
    "941": ["20", "25", "28C"],
    "942": ["20", "25", "28C", "34F"]
  }
}
```

## Configuration System Architecture

The configuration system supports external JSON files for:

1. **Mandatory Fields**: Define required fields per message type
2. **Field Validation Rules**: SWIFT format validation patterns
3. **Business Logic Rules**: JSONLogic rules for complex validation

## Usage in Code

### Loading Configuration

```rust
use swift_mt_message::config::Config;

// Load default configuration (loads from config/mandatory_fields.json)
let config = Config::load_default()?;

// Load from custom file
let config = Config::load_from_file("path/to/custom_config.json")?;

// Get mandatory fields for a message type
let mandatory_fields = config.get_mandatory_fields("103");
println!("MT103 mandatory fields: {:?}", mandatory_fields);
```

### Configuration Structure

The main configuration struct supports:

```rust
pub struct Config {
    pub mandatory_fields: MandatoryFieldsConfig,
    pub field_validations: FieldValidationsConfig,
    // Additional config sections can be added here
}
```

### Field Validation Configuration

```rust
pub struct FieldValidationsConfig {
    pub patterns: HashMap<String, ValidationPattern>,
    pub field_rules: HashMap<String, FieldRule>,
}

pub struct ValidationPattern {
    pub pattern: String,
    pub description: String,
    pub examples: Vec<String>,
}

pub struct FieldRule {
    pub tag: String,
    pub format: String,
    pub description: String,
    pub mandatory_for: Vec<String>,
    pub options: Option<HashMap<String, String>>,
}
```

## Built-in Configuration

The library includes hardcoded defaults that can be overridden by configuration files:

### Default Mandatory Fields

- **MT103**: 20, 23B, 32A, 50, 59, 71A
- **MT102**: 20, 23B, 32A, 50, 71A  
- **MT202**: 20, 32A, 52A, 58A
- **MT199**: 20, 79
- **MT192/195/196/197**: 20, 21
- **MT940**: 20, 25, 28C, 60F, 62F
- **MT941**: 20, 25, 28C
- **MT942**: 20, 25, 28C, 34F

### Default Validation Patterns

Common SWIFT format patterns are built into the configuration system:

- `16x` - Up to 16 characters
- `4!c` - Exactly 4 alphanumeric characters  
- `6!n` - Exactly 6 numeric characters
- `3!a` - Exactly 3 alphabetic characters
- `6!n3!a15d` - 6 digits + 3 letters + up to 15 digits with decimal
- `4*35x` - Up to 4 lines of 35 characters each

## Extending Configuration

### Adding New Message Types

1. Add the message type to `mandatory_fields.json`:

```json
{
  "message_types": {
    "104": ["20", "23B", "32A", "50A", "59A"]
  }
}
```

2. The configuration will be automatically loaded and used for validation.

### Adding Field Validation Rules

Create or extend the field validation configuration:

```json
{
  "field_validations": {
    "patterns": {
      "account_number": {
        "pattern": "34x",
        "description": "Account identification",
        "examples": ["/12345678", "12345678901234567890"]
      }
    },
    "field_rules": {
      "25": {
        "tag": "25",
        "format": "35x",
        "description": "Account Identification",
        "mandatory_for": ["940", "950"],
        "options": null
      }
    }
  }
}
```

### Business Rules Configuration

For complex validation rules using JSONLogic:

```json
{
  "rules": [
    {
      "name": "MT103_Amount_Validation",
      "description": "Amount must be positive for MT103",
      "logic": {
        ">": [
          {"var": "fields.32A.amount"},
          0
        ]
      }
    }
  ]
}
```

## Integration with Validation

### Field-Level Validation

```rust
use swift_mt_message::{
    field_parser::SwiftMessage,
    config::Config,
    validator::FormatRules
};

let config = Config::load_default()?;
let rules = FormatRules::from_config(&config)?;

// Validate individual fields
for (tag, field) in &message.fields {
    if let Err(e) = field.validate(&rules) {
        println!("Field {} validation failed: {}", tag, e);
    }
}
```

### Message-Level Validation

```rust
use swift_mt_message::{
    mt_models::mt103::MT103,
    validation::validate_mt_message_with_rules
};

let mt103 = MT103::from_swift_message(message)?;

// Validate business rules using external rule file
match mt103.validate_business_rules() {
    Ok(report) => {
        if !report.overall_valid {
            for failure in report.get_failures() {
                println!("Rule '{}' failed: {}", failure.rule_name, failure.message);
            }
        }
    }
    Err(e) => println!("Validation error: {}", e),
}
```

## Configuration File Discovery

The configuration system looks for files in this order:

1. Explicitly provided file path
2. `config/mandatory_fields.json` (relative to crate root)
3. Built-in hardcoded defaults

This allows for:

- **Development**: Use default embedded configuration
- **Testing**: Override with test-specific configuration
- **Production**: Use environment-specific configuration files

## Benefits of External Configuration

1. **Maintainability**: Update rules without code changes
2. **Flexibility**: Different environments can use different rule sets
3. **Extensibility**: Easy to add new fields and message types
4. **Compliance**: Align with different regional SWIFT implementations
5. **Testing**: Easy to test edge cases with custom configurations
6. **Deployment**: Configuration can be updated independently of code

## Future Enhancements

Planned configuration features:

- **Hot Reload**: Reload configuration without restarting
- **Configuration Validation**: JSON schema validation for config files
- **Hierarchical Configuration**: Environment-specific overrides
- **Configuration API**: REST API for dynamic configuration updates
- **Configuration Versioning**: Track configuration changes over time 