# SWIFT MT Message Dataflow Plugins

This directory contains dataflow-rs plugin implementations for SWIFT MT message processing. These plugins integrate with the [dataflow-rs](https://github.com/Plasmatic/dataflow-rs) workflow engine to provide a complete message processing pipeline.

## Overview

The MT plugin system provides four core operations for SWIFT MT message handling:

1. **Generate** - Create sample MT messages from datafake scenarios
2. **Publish** - Convert JSON to SWIFT MT format
3. **Validate** - Validate MT messages against SWIFT standards and business rules
4. **Parse** - Convert MT messages back to structured JSON

## Parameter Naming Convention

All plugins follow a consistent **source/target** naming pattern:

- **`source`** - The field name containing input data (where data comes from)
- **`target`** - The field name where output will be stored (where data goes to)

This convention:
- ✅ Provides clear semantic meaning
- ✅ Avoids confusion with dataflow-rs `input` configuration object
- ✅ Makes data flow direction explicit in workflows
- ✅ Maintains consistency across all plugins

## Plugin Reference

### 1. Generate Plugin (`generate_mt`)

Generates sample SWIFT MT messages using datafake scenarios.

**Parameters:**
- `target` (required): Field name where generated JSON data will be stored

**Input:**
- Reads datafake scenario from message payload

**Output:**
- JSON object containing:
  - `json_data`: Complete MT message structure
  - `message_type`: Detected message type (e.g., "MT103")

**Example:**
```json
{
  "function": {
    "name": "generate_mt",
    "input": {
      "target": "sample_json"
    }
  }
}
```

**Use Cases:**
- Test data generation
- Schema validation testing
- Sample message creation for documentation

---

### 2. Publish Plugin (`publish_mt`)

Converts JSON MT message data to SWIFT MT format.

**Parameters:**
- `source` (required): Field name containing JSON message data
- `target` (required): Field name where MT output will be stored

**Input:**
- JSON object with MT message structure

**Output:**
- SWIFT MT compliant message string

**Example:**
```json
{
  "function": {
    "name": "publish_mt",
    "input": {
      "source": "sample_json",
      "target": "sample_mt"
    }
  }
}
```

**Features:**
- Auto-detects message type from JSON structure
- Supports all MT message types (MT101-MT950)
- Generates SWIFT-compliant message format
- Validates structure during serialization

---

### 3. Validate Plugin (`validate_mt`)

Validates SWIFT MT messages against standards and business rules.

**Parameters:**
- `source` (required): Field name containing MT message to validate
- `target` (required): Field name where validation results will be stored

**Input:**
- SWIFT MT message string

**Output:**
- JSON object with validation results:
  ```json
  {
    "valid": true/false,
    "errors": ["error message 1", "error message 2", ...],
    "timestamp": "2025-10-12T10:30:00Z"
  }
  ```

**Example:**
```json
{
  "function": {
    "name": "validate_mt",
    "input": {
      "source": "sample_mt",
      "target": "validation_result"
    }
  }
}
```

**Validation Checks:**
- SWIFT message structure and syntax
- Field format and pattern compliance
- Required field validation
- Network validation rules (CBPR+)
- Business rule validation

---

### 4. Parse Plugin (`parse_mt`)

Parses SWIFT MT messages into structured JSON format.

**Parameters:**
- `source` (required): Field name containing MT message
- `target` (required): Field name where parsed JSON will be stored

**Input:**
- SWIFT MT message string

**Output:**
- JSON object with structured message data
- Metadata including message type and method (normal/stp/cover/reject/return)

**Example:**
```json
{
  "function": {
    "name": "parse_mt",
    "input": {
      "source": "sample_mt",
      "target": "mt_json"
    }
  }
}
```

**Features:**
- Auto-detects message type from MT format
- Handles all supported MT message types
- Preserves all message structure and data
- Type-safe parsing with validation
- Detects STP, Cover, Reject, and Return messages

---

## Complete Workflow Example

Here's a complete end-to-end workflow demonstrating all four plugins:

```json
{
  "id": "mt_processing_pipeline",
  "name": "SWIFT MT Processing Pipeline",
  "description": "Complete message processing with generation, publishing, validation, and parsing",
  "priority": 0,
  "tasks": [
    {
      "id": "step_1_generate",
      "name": "Generate Sample Data",
      "description": "Generate sample MT message from datafake scenario",
      "function": {
        "name": "generate_mt",
        "input": {
          "target": "sample_json"
        }
      }
    },
    {
      "id": "step_2_publish",
      "name": "Publish to MT",
      "description": "Convert JSON to SWIFT MT format",
      "function": {
        "name": "publish_mt",
        "input": {
          "source": "sample_json",
          "target": "sample_mt"
        }
      }
    },
    {
      "id": "step_3_validate",
      "name": "Validate MT",
      "description": "Validate message against SWIFT standards",
      "function": {
        "name": "validate_mt",
        "input": {
          "source": "sample_mt",
          "target": "validation_result"
        }
      }
    },
    {
      "id": "step_4_parse",
      "name": "Parse MT",
      "description": "Parse MT back to structured JSON",
      "function": {
        "name": "parse_mt",
        "input": {
          "source": "sample_mt",
          "target": "mt_json"
        }
      }
    }
  ]
}
```

**Data Flow Visualization:**

```
                    ┌─────────────────┐
                    │  Datafake       │
                    │  Scenario       │
                    │  (Payload)      │
                    └────────┬────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │   generate_mt   │
                    │  target: json   │
                    └────────┬────────┘
                             │
                    sample_json (JSON)
                             │
                             ▼
                    ┌─────────────────┐
                    │   publish_mt    │
                    │  source: json   │
                    │  target: mt     │
                    └────────┬────────┘
                             │
                    sample_mt (MT String)
                             │
                    ┌────────┴────────┐
                    │                 │
                    ▼                 ▼
           ┌─────────────────┐ ┌─────────────────┐
           │   validate_mt   │ │    parse_mt     │
           │  source: mt     │ │  source: mt     │
           │  target: result │ │  target: parsed │
           └────────┬────────┘ └────────┬────────┘
                    │                   │
                    ▼                   ▼
          validation_result        mt_json (JSON)
```

## Common Usage Patterns

### Pattern 1: Generate and Publish

Generate test data and convert to MT:

```json
{
  "tasks": [
    {
      "function": {
        "name": "generate_mt",
        "input": {"target": "msg_data"}
      }
    },
    {
      "function": {
        "name": "publish_mt",
        "input": {
          "source": "msg_data",
          "target": "msg_mt"
        }
      }
    }
  ]
}
```

### Pattern 2: Parse and Validate

Parse incoming MT and validate:

```json
{
  "tasks": [
    {
      "function": {
        "name": "parse_mt",
        "input": {
          "source": "incoming_mt",
          "target": "parsed_data"
        }
      }
    },
    {
      "function": {
        "name": "validate_mt",
        "input": {
          "source": "incoming_mt",
          "target": "validation"
        }
      }
    }
  ]
}
```

### Pattern 3: Round-Trip Testing

Test JSON → MT → JSON conversion:

```json
{
  "tasks": [
    {
      "function": {
        "name": "publish_mt",
        "input": {
          "source": "original_json",
          "target": "mt_output"
        }
      }
    },
    {
      "function": {
        "name": "parse_mt",
        "input": {
          "source": "mt_output",
          "target": "parsed_json"
        }
      }
    }
  ]
}
```

## Integration with Dataflow-rs

### Registering Plugins

```rust
use swift_mt_message::plugin::register_swift_mt_functions;
use dataflow_rs::Engine;

// Register all MT plugins
let custom_functions = register_swift_mt_functions()
    .into_iter()
    .collect::<HashMap<_, _>>();

// Create engine with workflows and plugins
let engine = Engine::new(workflows, Some(custom_functions));
```

### Processing Messages

```rust
use dataflow_rs::engine::Message;

// Create message with datafake scenario
let scenario = load_datafake_scenario("MT103", "urgent_payment");
let mut message = Message::from_value(&scenario);

// Process through workflow
let result = engine.process_message(&mut message).await?;

// Access results
let generated_json = message.data().get("sample_json");
let mt_output = message.data().get("sample_mt");
let validation_result = message.data().get("validation_result");
let parsed_json = message.data().get("mt_json");
```

## Error Handling

All plugins return structured errors through the dataflow-rs error system:

```rust
use dataflow_rs::engine::error::{DataflowError, Result};

// Example error types:
// - DataflowError::Validation("'source' parameter is required")
// - DataflowError::Validation("Field 'data' not found in message")
// - DataflowError::Validation("MT parsing error: ...")
// - DataflowError::Validation("JSON serialization error: ...")
```

## Testing

Run the end-to-end test suite:

```bash
# Test all message types and scenarios
cargo test test_swift_mt_workflow_pipeline

# Test specific message type
TEST_MESSAGE_TYPE=MT103 cargo test test_swift_mt_workflow_pipeline

# Debug specific scenario
TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=urgent_payment \
  TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 \
  cargo test test_swift_mt_workflow_pipeline -- --nocapture
```

## Architecture

### Module Structure

```
src/plugin/
├── mod.rs           # Plugin registration and exports
├── generate.rs      # Generate plugin implementation
├── parse.rs         # Parse plugin implementation
├── publish.rs       # Publish plugin implementation
├── validate.rs      # Validate plugin implementation
└── README.md        # This file
```

### Key Design Decisions

1. **Consistent Parameter Naming**: `source`/`target` pattern avoids confusion and improves clarity
2. **Field-based I/O**: Plugins read from and write to message data fields (not direct payload manipulation)
3. **Type Safety**: All operations use strongly-typed structs from the SWIFT MT library
4. **Format Detection**: Automatic message type detection from MT structure
5. **Error Propagation**: Structured error handling with clear error messages

## Supported Message Types

The plugin system supports all SWIFT MT message types in this library:

### Category 1 - Customer Payments and Cheques
- **MT101** - Request for Transfer
- **MT103** - Single Customer Credit Transfer
- **MT104** - Direct Debit and Request for Debit Transfer
- **MT107** - General Direct Debit Message

### Category 1 - Treasury Markets
- **MT110** - Advice of Cheque(s)
- **MT111** - Request for Stop Payment of a Cheque
- **MT112** - Status of a Request for Stop Payment of a Cheque

### Category 1 - Documentary Credits
- **MT190** - Advice of Charges, Interest and Other Adjustments
- **MT191** - Request for Payment of Charges, Interest and Other Expenses
- **MT192** - Request for Cancellation
- **MT196** - Answers to Message Type 192 Request for Cancellation
- **MT199** - Free Format Message

### Category 2 - Financial Institution Transfers
- **MT200** - Financial Institution Transfer for its Own Account
- **MT202** - General Financial Institution Transfer
- **MT204** - Financial Markets Direct Debit Message
- **MT205** - Financial Institution Transfer Execution
- **MT210** - Notice to Receive

### Category 2 - Notifications
- **MT290** - Advice of Charges, Interest and Other Adjustments
- **MT291** - Request for Payment of Charges, Interest and Other Expenses
- **MT292** - Request for Cancellation
- **MT296** - Answers to Message Type 292 Request for Cancellation
- **MT299** - Free Format Message

### Category 9 - Cash Management and Customer Status
- **MT900** - Confirmation of Debit
- **MT910** - Confirmation of Credit
- **MT920** - Request Message
- **MT935** - Rate and Balance Information
- **MT940** - Customer Statement Message
- **MT941** - Balance Report
- **MT942** - Interim Transaction Report
- **MT950** - Statement Message

See `tests/` directory for example scenarios for each message type.

## Contributing

When adding new plugins or modifying existing ones:

1. Follow the `source`/`target` parameter naming convention
2. Update this README with new functionality
3. Add comprehensive tests in `tests/`
4. Ensure error messages are clear and actionable
5. Add datafake scenarios for new message types

## License

This plugin system is part of the SwiftMTMessage library and follows the same license terms.
