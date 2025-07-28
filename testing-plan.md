# SWIFT MT Message Testing Plan

## Overview

This document describes the scenario-based testing approach for SWIFT MT messages, replacing static test data files with dynamic, configurable test scenarios. Each scenario configuration is self-contained and generates realistic test data using a combination of pre-generated variables, the `datafake-rs` library, and static values.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Configuration Schema](#configuration-schema)
3. [Variable System](#variable-system)
4. [Field Generation Methods](#field-generation-methods)
5. [Examples](#examples)
6. [Best Practices](#best-practices)
7. [Reference](#reference)

## Architecture Overview

### Key Components

```
test_scenarios/
├── mt103/
│   ├── standard_payment.json
│   ├── stp_compliant.json
│   ├── high_value_edd.json
│   └── rejection_scenario.json
├── mt202/
│   ├── bank_to_bank.json
│   ├── cover_payment.json
│   └── liquidity_transfer.json
└── mt9xx/
    ├── account_statement.json
    └── balance_report.json
```

### Core Principles

1. **Self-Contained Scenarios**: Each JSON configuration contains everything needed to generate a complete test message
2. **Local Variables**: Variables are generated once per sample and shared between fields within that sample
3. **Realistic Data**: Uses the `datafake-rs` library to generate realistic names, addresses, BIC codes, and other financial data
4. **Field-Specific Logic**: Each field type can have its own generation logic and validation

## Configuration Schema

### Basic Structure

Each scenario configuration is a self-contained JSON file with the following structure supported by datafake-rs:

```json
{
  "variables": {
    // Variables generated once per sample, shared across fields
  },
  
  "schema": {
    // Message structure including headers and fields
  }
}
```

Note: The scenario metadata (id, name, description, message_type) should be encoded in the filename or directory structure rather than within the JSON configuration itself.

## Variable System

Variables are pre-generated values that can be shared across multiple fields in a single message. They are defined at the root level and generated once per sample.

### Purpose of Variables

Variables solve the problem of data consistency within a message. For example:
- The same currency code must appear in multiple fields
- Sender information must be consistent across different fields
- Related amounts (base amount, fees, total) must be mathematically correct

### Variable Definition

Variables are defined in the `variables` section at the root of the configuration:

```json
{
  "variables": {
    "currency": "USD",
    "sender_bic": "CHASUS33XXX",
    "amount": 150000.00,
    "value_date": "241215"
  }
}
```

### Using Variables in Fields

Variables are referenced using the `${variable_name}` syntax:

```json
{
  "schema": {
    "fields": {
      "32A": {
        "value": "${value_date}${currency}${amount}"
      },
      "52A": {
        "value": "${sender_bic}"
      }
    }
  }
}
```

## Field Generation Methods

There are three ways to generate field values in the test scenarios:

### 1. Using Variables (${variable_name})

Reference pre-generated variables for data that needs to be consistent across fields:

```json
{
  "variables": {
    "currency": "EUR",
    "amount": 25000.50,
    "sender_bic": "DEUTDEFFXXX"
  },
  "schema": {
    "fields": {
      "32A": {
        "value": "241215${currency}${amount}"
      },
      "52A": {
        "value": "${sender_bic}"
      }
    }
  }
}
```

### 2. Using the datafake-rs Library

Generate realistic data directly in field configurations using the `datafake-rs` library with JSONLogic syntax:

```json
{
  "schema": {
    "fields": {
      "50K": {
        "account": "1234567890",
        "name_address": [
          {"fake": ["name"]},
          {"fake": ["street_address"]},
          {"fake": ["city_name"]},
          {"fake": ["country_name"]}
        ]
      },
      "52A": {
        "value": {"fake": ["bic"]}
      }
    }
  }
}
```

### 3. Using Static Values

Provide fixed values for fields that don't need variation:

```json
{
  "schema": {
    "fields": {
      "23B": "CRED",
      "71A": "SHA",
      "72": "/INS/CHQB"
    }
  }
}
```

### Field Configuration Options

Each field can have additional configuration options:

```json
{
  "field_tag": {
    "value": "...",              // The field value (using any of the 3 methods above)
    "optional": true,            // Whether the field is optional
    "probability": 0.8          // Probability of including optional fields (0-1)
  }
}
```

### Complex Field Types

#### Multi-Component Fields (e.g., 32A - Value Date, Currency and Amount)
```json
{
  "32A": {
    "value": "${value_date}${currency}${amount}"
  }
}
```

#### Multi-Line Fields (e.g., 50K - Ordering Customer)
```json
{
  "50K": {
    "account": "/1234567890",
    "name_address": [
      {"fake": ["company_name"]},
      {"fake": ["street_address"]},
      {"fake": ["city_name"]},
      {"fake": ["country_name"]}
    ]
  }
}
```

#### Enum Fields (e.g., Field 50 variants)
```json
{
  "50": {
    "variant": "K",  // Specify which variant to use: A, F, or K
    "account": "/1234567890",
    "name_address": [
      {"fake": ["name"]},
      {"fake": ["street_address"]}
    ]
  }
}
```

## Examples

### Example 1: Standard MT103 Cross-Border Payment

File: `test_scenarios/mt103/standard_payment.json`

This example demonstrates a typical corporate payment scenario using all three field generation methods:

```json
{
  "variables": {
    "currency": "EUR",
    "amount": 125000.50,
    "value_date": "241220",
    "reference": "FT2024120612345",
    "sender_bic": {"fake": ["bic"]},
    "receiver_bic": {"fake": ["bic"]}
  },
  
  "schema": {
    "headers": {
      "basic": {
        "sender": "${sender_bic}",
        "session": "0000",
        "sequence": "000001"
      },
      "application": {
        "receiver": "${receiver_bic}",
        "message_type": "103",
        "priority": "N"
      }
    },
    "fields": {
      "20": {
        "value": "${reference}"
      },
      "23B": {
        "value": "CRED"
      },
      "32A": {
        "value": "${value_date}${currency}${amount}"
      },
      "50K": {
        "account": "/1234567890",
        "name_address": [
          {"fake": ["company_name"]},
          {"fake": ["street_address"]},
          {"fake": ["city_name"]},
          "UNITED STATES"
        ]
      },
      "52A": {
        "value": "${sender_bic}",
        "optional": true,
        "probability": 0.8
      },
      "57A": {
        "value": "${receiver_bic}"
      },
      "59": {
        "account": "/DE89370400440532013000",
        "name_address": [
          {"fake": ["company_name"]},
          {"fake": ["street_address"]},
          "60311 FRANKFURT",
          "GERMANY"
        ]
      },
      "70": {
        "value": "INVOICE 2024-1234\nPAYMENT FOR MACHINERY\nCONTRACT REF ABC-789"
      },
      "71A": {
        "value": "SHA"
      }
    }
  }
}
```

### Example 2: MT202 COV with Mismatches

File: `test_scenarios/mt202/cov_mismatch.json`

This example demonstrates an MT202 COV message with intentional mismatches to test error collection:

```json
{
  "name": "MT202 COV with Mismatches",
  "description": "MT202 COV message with intentional mismatches to demonstrate error collection",
  "variables": {
    "sender_bic": "BANKUS33",
    "receiver_bic": "BANKDE55",
    "transaction_ref": "FT220315002",
    "related_ref": "FT220315001",
    "currency": "USD",
    "amount": 1050000.00,
    "value_date": "220315"
  },
  "schema": {
    "fields": {
      "20": {
        "value": "${transaction_ref}"
      },
      "21": {
        "value": "${related_ref}"
      },
      "32A": {
        "value": "${value_date}${currency}${amount}"
      },
      "58A": {
        "value": "HSBCSGSG"
      },
      "50#b": {
        "K": {
          "account": "/US9876543210987654",
          "name_and_address": [
            "ACME INDUSTRIES",
            "456 INDUSTRIAL PARK",
            "CHICAGO IL 60601"
          ]
        }
      },
      "59#b": {
        "NoOption": {
          "account": "/SG56HSBC000098765432",
          "name_and_address": [
            "SINGAPORE TECH PTE LTD",
            "RAFFLES PLACE TOWER",
            "SINGAPORE 048623"
          ]
        }
      },
      "70#b": {
        "narrative": [
          "/INV/INV-2022-0316",
          "/RFB/QUARTERLY PAYMENT"
        ]
      },
      "33B#b": {
        "currency": "${currency}",
        "amount": "${amount}"
      }
    }
  },
  "mismatches": {
    "ordering_customer": {
      "mt103_account": "/US1234567890123456",
      "mt202_account": "/US9876543210987654",
      "mt103_name": "ACME CORPORATION",
      "mt202_name": "ACME INDUSTRIES"
    },
    "beneficiary": {
      "mt103_account": "/SG56HSBC000012345678",
      "mt202_account": "/SG56HSBC000098765432"
    },
    "amount": {
      "mt103": 1000000.00,
      "mt202": 1050000.00,
      "difference": 50000.00
    }
  }
}
```

### Example 3: MT202 Interbank Transfer

File: `test_scenarios/mt202/liquidity_transfer.json`

This example shows a bank-to-bank liquidity transfer:

```json
{
  "variables": {
    "currency": "USD",
    "amount": 50000000.00,
    "value_date": "241215",
    "reference": "LIQ202412150001"
  },
  
  "schema": {
    "fields": {
      "20": {
        "value": "${reference}"
      },
      "21": {
        "value": "NONREF"
      },
      "32A": {
        "value": "${value_date}${currency}${amount}"
      },
      "52A": {
        "value": {"fake": ["bic"]}
      },
      "53B": {
        "account": "/12345678901234567890",
        "optional": true,
        "probability": 0.3
      },
      "56A": {
        "value": {"fake": ["bic"]},
        "optional": true,
        "probability": 0.5
      },
      "57A": {
        "value": {"fake": ["bic"]}
      },
      "58A": {
        "value": {"fake": ["bic"]}
      },
      "72": {
        "value": "/BNF/LIQUIDITY MANAGEMENT",
        "optional": true,
        "probability": 0.2
      }
    }
  }
}
```

### Example 4: MT103 with Multiple Currencies (Testing Edge Cases)

File: `test_scenarios/mt103/currency_conversion.json`

This example demonstrates fee handling with currency conversion:

```json
{
  "variables": {
    "send_currency": "USD",
    "receive_currency": "EUR",
    "send_amount": 100000.00,
    "exchange_rate": 0.92,
    "receive_amount": 92000.00,
    "value_date": "241218",
    "reference": "FX20241218001"
  },
  
  "schema": {
    "fields": {
      "20": {
        "value": "${reference}"
      },
      "23B": {
        "value": "CRED"
      },
      "23E": {
        "value": "SDVA"
      },
      "32A": {
        "value": "${value_date}${send_currency}${send_amount}"
      },
      "33B": {
        "value": "${receive_currency}${receive_amount}"
      },
      "36": {
        "value": "${exchange_rate}"
      },
      "50K": {
        "account": "/9876543210",
        "name_address": [
          {"fake": ["name"]},
          {"fake": ["street_address"]},
          "NEW YORK NY 10001",
          "UNITED STATES"
        ]
      },
      "59": {
        "account": "/FR1420041010050500013M02606",
        "name_address": [
          {"fake": ["name"]},
          {"fake": ["street_address"]},
          "75008 PARIS",
          "FRANCE"
        ]
      },
      "71A": {
        "value": "OUR"
      },
      "71F": {
        "value": "${send_currency}25,00"
      },
      "71G": {
        "value": "${receive_currency}20,00"
      }
    }
  }
}
```

## MT202 COV Mismatch Detection

### Overview

MT202 COV (Cover Payment) messages are used to facilitate the settlement of underlying MT103 customer payments. A critical aspect of processing these messages is ensuring data consistency between the MT103 and its corresponding MT202 COV. The error collection feature in permissive parsing mode enables detection and tracking of these mismatches.

### Common Mismatch Types

1. **Ordering Customer Mismatches**
   - Different account numbers between MT103 Field 50 and MT202 Field 50 (Sequence B)
   - Name variations (e.g., "ACME CORPORATION" vs "ACME INDUSTRIES")
   - Address discrepancies

2. **Beneficiary Mismatches**
   - Different account numbers between MT103 Field 59 and MT202 Field 59 (Sequence B)
   - Address or location differences
   - Name formatting variations

3. **Amount Mismatches**
   - Different settlement amounts (Field 32A)
   - Instructed amount vs settlement amount discrepancies (Field 33B)
   - Currency conversion issues

4. **Remittance Information Mismatches**
   - Different invoice numbers or references (Field 70)
   - Payment purpose variations
   - Missing or additional information

5. **Correspondent Bank Mismatches**
   - Missing correspondent banks in MT202 Sequence B
   - Different routing paths
   - BIC code discrepancies

### Testing Mismatch Detection

Create test scenarios that intentionally introduce mismatches:

```rust
// Example: Testing ordering customer mismatch
let mt103 = parse_mt103_with_customer("ACME CORPORATION", "/US1234567890");
let mt202_cov = parse_mt202_cov_with_customer("ACME INDUSTRIES", "/US9876543210");

// In permissive mode, both messages parse successfully
// but errors are collected for the mismatches
assert!(mt202_cov.errors.contains(&SwiftError::ValidationError {
    field: "50#b",
    message: "Ordering customer mismatch with related MT103"
}));
```

### Best Practices for COV Testing

1. **Always Test Both Messages**: Parse both MT103 and MT202 COV to verify consistency
2. **Use Permissive Mode**: Enable error collection to capture all mismatches
3. **Verify COV Flag**: Check that MT202 is properly identified as a cover message
4. **Test Edge Cases**: Include scenarios with partial matches and formatting differences
5. **Document Expected Mismatches**: Clearly indicate which mismatches are intentional in test scenarios

## Best Practices

### 1. Self-Contained Scenarios

Each scenario configuration should be complete and independent:

- Define all required variables at the root level
- Include all necessary fields for a valid message in the schema
- Encode scenario metadata in the filename or directory structure

### 2. Variable Usage Guidelines

Use variables when data needs to be consistent across fields:

```json
{
  "variables": {
    "currency": "USD",              // Used in fields 32A, 33B, 71F
    "sender_bic": "CHASUS33XXX",    // Used in headers and field 52A
    "amount": 50000.00              // Used in calculations and field 32A
  }
}
```

### 3. Realistic Data Generation

Leverage the `datafake-rs` library for realistic data:

```json
{
  "50K": {
    "name_address": [
      {"fake": ["company_name"]},      // Realistic company names
      {"fake": ["street_address"]},    // Real-looking addresses
      {"fake": ["city_name"]},
      {"fake": ["country_name"]}
    ]
  },
  "52A": {
    "value": {"fake": ["bic"]}        // Generates valid BIC codes
  }
}
```

### 4. Scenario Organization

Structure your test scenarios by purpose:

```
test_scenarios/
├── mt103/
│   ├── standard_payment.json      # Normal business flow
│   ├── high_value_payment.json    # Compliance testing
│   ├── currency_conversion.json   # FX scenarios
│   └── rejection_scenarios.json   # Error cases
├── mt202/
│   ├── liquidity_transfer.json
│   └── cover_payment.json
```

### 5. Testing Edge Cases

Create specific scenarios for boundary conditions:

File: `test_scenarios/mt103/max_length_fields.json`

```json
{
  "variables": {
    "long_reference": "ABCDEFGHIJKLMNOP",  // 16 chars - max for field 20
    "max_amount": 999999999999999.99      // Maximum decimal value
  },
  "schema": {
    "fields": {
      "20": {
        "value": "${long_reference}"
      },
      "32A": {
        "value": "241215USD${max_amount}"
      }
    }
  }
}
```

## Reference

### Configuration Structure

```json
{
  "variables": {
    // Pre-generated values shared across fields
    "variable_name": "value"
  },
  "schema": {
    "headers": {
      // Optional SWIFT headers configuration
    },
    "fields": {
      // Field configurations
    }
  }
}
```

**Note**: Scenario metadata (id, name, description, message_type) and validation expectations should be encoded in the filename and directory structure:
- Filename: `mt103_standard_payment.json` (indicates message type and scenario)
- Directory: `test_scenarios/mt103/compliance/` (indicates category)
- Validation can be handled by the test runner based on directory/filename conventions

### Field Configuration Options

| Option | Description | Type | Example |
|--------|-------------|------|---------|
| `value` | Field value (static or with variables) | string | `"${currency}"` |
| `account` | Account number for account fields | string | `"/1234567890"` |
| `name_address` | Name/address lines | array | `[{"fake": "name"}, "CITY"]` |
| `optional` | Whether field is optional | boolean | `true` |
| `probability` | Probability of including optional field | number | `0.8` |
| `variant` | For enum fields (e.g., 50A/F/K) | string | `"K"` |

### Fake Data Types

The `datafake-rs` library can generate the following types of data:

| Type | Description | Example |
|------|-------------|----------|
| `name` | Person's full name | "John Smith" |
| `first_name` | First name only | "Alice" |
| `last_name` | Last name only | "Johnson" |
| `name_with_title` | Name with title | "Dr. Jane Doe" |
| `title` | Professional title | "Dr." |
| `suffix` | Name suffix | "Jr." |
| `company_name` | Company name | "Acme Corp" |
| `company_suffix` | Company suffix | "LLC" |
| `street_address` | Street address | "123 Main St" |
| `street_name` | Street name only | "Oak Avenue" |
| `street_suffix` | Street suffix | "Blvd" |
| `city_name` | City name | "New York" |
| `state_name` | Full state name | "California" |
| `state_abbreviation` | State code | "CA" |
| `country_name` | Country name | "United States" |
| `country_code` | Country code | "US" |
| `zip_code` | US ZIP code | "10001" |
| `postal_code` | Generic postal code | "SW1A 1AA" |
| `phone_number` | Phone number | "+1 555-123-4567" |
| `cell_phone` | Mobile number | "555-0123" |
| `email` | Email address | "user@example.com" |
| `safe_email` | Safe email | "user@example.net" |
| `free_email` | Free email provider | "user@gmail.com" |
| `username` | Username | "johndoe123" |
| `password` | Password (configurable) | "Xy#9mK2$pL" |
| `bic` | Bank Identifier Code | "CHASUS33XXX" |
| `credit_card_number` | Credit card | "4532123456789012" |
| `currency_code` | ISO currency code | "USD" |
| `currency_name` | Currency name | "US Dollar" |
| `currency_symbol` | Currency symbol | "$" |
| `uuid` | UUID v4 | "550e8400-e29b..." |
| `ipv4` | IPv4 address | "192.168.1.1" |
| `ipv6` | IPv6 address | "2001:db8::1" |
| `mac_address` | MAC address | "00:1B:44:11:3A:B7" |
| `user_agent` | Browser user agent | "Mozilla/5.0..." |
| `latitude` | Latitude coordinate | "40.7128" |
| `longitude` | Longitude coordinate | "-74.0060" |
| `word` | Random word | "innovation" |
| `sentence` | Random sentence | "Lorem ipsum dolor..." |
| `paragraph` | Random paragraph | "Lorem ipsum..." |
| `u8`, `u16`, `u32`, `u64` | Unsigned integers | 42 |
| `i8`, `i16`, `i32`, `i64` | Signed integers | -42 |
| `f32`, `f64` | Floating point | 3.14159 |
| `bool` | Boolean value | true |

### Numeric Types with Ranges

Numeric types support optional min/max ranges:

```json
{
  "age": {"fake": ["u8", 18, 65]},       // Random age between 18-65
  "balance": {"fake": ["f64", 0.0, 10000.0]}, // Random balance 0-10000
  "quantity": {"fake": ["i32", -100, 100]}   // Random integer -100 to 100
}
```

### Advanced datafake-rs Features

#### JSONLogic Support

datafake-rs supports JSONLogic for conditional field generation:

```json
{
  "variables": {
    "amount": {"fake": ["f64", 1000.0, 100000.0]},
    "is_high_value": {">=": [{"var": "amount"}, 50000]}
  },
  "schema": {
    "fields": {
      "72": {
        "value": {
          "if": [
            {"var": "is_high_value"},
            "/COMPLY/HIGH VALUE PAYMENT - EDD REQUIRED",
            "/REC/STANDARD PROCESSING"
          ]
        }
      }
    }
  }
}
```

#### Batch Generation

Generate multiple test messages with a single configuration:

```json
{
  "batch": {
    "count": 10,
    "seed": 12345  // Optional: for reproducible results
  },
  "variables": {
    // ... variable definitions
  },
  "schema": {
    // ... schema configuration
  }
}
```

#### Complex Relationships

Use JSONLogic to create related field values:

```json
{
  "variables": {
    "base_amount": {"fake": ["f64", 10000.0, 50000.0]},
    "fee_percentage": 0.002,
    "fee_amount": {"*": [{"var": "base_amount"}, {"var": "fee_percentage"}]},
    "total_amount": {"+": [{"var": "base_amount"}, {"var": "fee_amount"}]}
  }
}
```

### Variable Syntax

Variables are referenced using `${variable_name}` syntax and can be concatenated:

```json
{
  "variables": {
    "currency": "USD",
    "amount": "50000.00"
  },
  "schema": {
    "fields": {
      "32A": {
        "value": "241215${currency}${amount}"  // Results in: "241215USD50000.00"
      }
    }
  }
}
```

### Message Type Support

The following MT message types are supported:

- **MT1xx**: Customer Payments (MT103, MT104, MT107, MT110-112)
- **MT2xx**: Financial Institution Transfers (MT202, MT205, MT210)
- **MT2xx**: Market Infrastructure (MT292, MT296, MT299)
- **MT9xx**: Cash Management (MT900, MT910, MT920, MT935, MT940-942, MT950)

Each message type has specific field requirements and validation rules that should be considered when creating test scenarios.

## Conclusion

This scenario-based testing approach provides a clean, maintainable way to generate realistic SWIFT MT message test data. By combining pre-generated variables, the `datafake-rs` library for realistic data generation, and static values, you can create comprehensive test suites that accurately simulate real-world financial messaging scenarios.

The `datafake-rs` library brings several advantages over basic fake data generation:
- **Financial-specific data types**: Built-in support for BIC codes, currency codes, and credit card numbers
- **JSONLogic integration**: Conditional logic and complex relationships between fields
- **Batch generation**: Create multiple test messages from a single configuration
- **Configuration-driven**: JSON-based approach that aligns with SWIFT message structure
- **Reproducible results**: Optional seed values for consistent test data

The self-contained nature of each scenario configuration ensures that tests are reproducible and easy to understand, while the flexibility of the system allows for testing edge cases and compliance requirements.