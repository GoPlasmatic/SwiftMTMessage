# Test Scenarios

This directory contains comprehensive test scenarios for SWIFT MT messages, designed to cover various real-world use cases in payment processing and financial messaging.

## Structure

Each message type has its own directory containing:
- Multiple scenario JSON files (e.g., `standard.json`, `stp.json`, `high_value.json`)
- An `index.json` file listing all available scenarios
- A `README.md` file documenting each scenario

```
test_scenarios/
├── mt101/
│   ├── index.json
│   ├── standard.json
│   ├── bulk_payment.json
│   ├── ... (more scenarios)
│   └── README.md
├── mt103/
│   ├── index.json
│   ├── standard.json
│   ├── stp.json
│   ├── ... (more scenarios)
│   └── README.md
└── README.md (this file)
```

## Testing Framework

The `round_trip_test.rs` file provides comprehensive testing capabilities:

### Running Tests

```bash
# Test all message types and all scenarios (100 samples each)
cargo test round_trip_scenarios

# Test all scenarios for a specific message type
cargo test round_trip_scenarios -- MT103

# Test a specific scenario
cargo test round_trip_scenarios -- MT103 standard
```

### Test Process

For each scenario, the test:
1. Generates 100 sample messages using the scenario configuration
2. Converts each to MT format
3. Parses the MT format back to structured data
4. Validates the parsed message
5. Performs JSON round-trip serialization
6. Compares the original and round-trip results

### Scenario Configuration

Each scenario file uses a consistent JSON structure:
```json
{
  "variables": {
    // Dynamic values using datafake generation
    "sender_bic": {"fake": ["bic"]},
    "amount": {"fake": ["f64", 1000.0, 50000.0]}
  },
  "schema": {
    "basic_header": { ... },
    "application_header": { ... },
    "message_type": "103",
    "fields": {
      // Field definitions
    }
  }
}
```

## Message Types

### MT101 - Request for Transfer
- **Scenarios**: 9 comprehensive scenarios
- **Use Cases**: Corporate payment requests, bulk payments, direct debits
- **See**: [mt101/README.md](mt101/README.md)

### MT103 - Single Customer Credit Transfer
- **Scenarios**: 12 comprehensive scenarios
- **Use Cases**: Wire transfers, SWIFT payments, cross-border transactions
- **See**: [mt103/README.md](mt103/README.md)

## Adding New Scenarios

1. Create a new `.json` file in the appropriate message type directory
2. Follow the existing structure and naming conventions
3. Add the scenario name to the `index.json` file
4. Document the scenario in the message type's README.md
5. Test the scenario:
   ```bash
   cargo test round_trip_scenarios -- MT103 your_new_scenario
   ```

## Scenario Naming Conventions

- `standard` - Basic scenario with common fields
- `minimal` - Minimum required fields only
- `stp` - Straight-through processing
- `high_value` - Large amount transactions
- `fx_conversion` - Foreign exchange scenarios
- `urgent_payment` - Time-critical transfers
- `bulk_payment` - Multiple transactions
- `rejection` - Payment rejection scenarios
- `return` - Payment return/reversal

## Validation

All scenarios are validated for:
- ✅ Valid JSON syntax
- ✅ Consistent structure across scenarios
- ✅ Appropriate field usage for use case
- ✅ SWIFT MT standard compliance
- ✅ Successful round-trip conversion
- ✅ 100% parsing success rate

## Debugging Failed Tests

If a test fails:
1. Run the specific scenario to isolate the issue:
   ```bash
   cargo test round_trip_scenarios -- MT103 problematic_scenario
   ```

2. Use the generate_sample example to inspect output:
   ```bash
   cargo run --example generate_sample MT103 problematic_scenario
   ```

3. Check the test output for specific error details (parse, validation, or round-trip stage)

## Performance

Each scenario generates and tests 100 samples to ensure:
- Consistent generation across multiple runs
- Coverage of random value ranges
- Stability of parsing and validation
- No edge case failures

Testing all scenarios typically takes 1-2 minutes depending on hardware.