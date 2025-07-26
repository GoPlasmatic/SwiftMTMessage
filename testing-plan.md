# SWIFT MT Message Testing Plan

## Overview

This document describes the scenario-based testing approach for SWIFT MT messages, replacing static test data files with dynamic, configurable test scenarios. Each scenario configuration is self-contained and generates realistic test data using a combination of pre-generated variables, the `fake` crate, and static values.

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
3. **Realistic Data**: Uses the `fake` crate to generate realistic names, addresses, and other data
4. **Field-Specific Logic**: Each field type can have its own generation logic and validation

## Configuration Schema

### Basic Structure

Each scenario configuration is a self-contained JSON file with the following structure:

```json
{
  "scenario": {
    "id": "unique_scenario_id",
    "name": "Human-readable scenario name",
    "description": "What this scenario tests",
    "message_type": "MT103"
  },
  
  "variables": {
    // Variables generated once per sample, shared across fields
  },
  
  "message": {
    // Message headers and fields configuration
  },
  
  "validation": {
    // Expected validation results for testing
  }
}
```

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
  "message": {
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
  "message": {
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

### 2. Using the Fake Crate

Generate realistic data directly in field configurations using the `fake` crate:

```json
{
  "message": {
    "fields": {
      "50K": {
        "account": "1234567890",
        "name_address": [
          {
            "fake": "name",
            "locale": "en_US"
          },
          {
            "fake": "street_address",
            "locale": "en_US"
          },
          {
            "fake": "city_with_state",
            "locale": "en_US"
          }
        ]
      }
    }
  }
}
```

### 3. Using Static Values

Provide fixed values for fields that don't need variation:

```json
{
  "message": {
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
      {"fake": "company", "locale": "en_US"},
      {"fake": "street_address", "locale": "en_US"},
      {"fake": "city", "locale": "en_US"},
      "UNITED STATES"
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
      {"fake": "name", "locale": "en_US"},
      {"fake": "street_address", "locale": "en_US"}
    ]
  }
}
```

## Examples

### Example 1: Standard MT103 Cross-Border Payment

This example demonstrates a typical corporate payment scenario using all three field generation methods:

```json
{
  "scenario": {
    "id": "mt103_standard_payment",
    "name": "Standard Corporate Payment",
    "description": "Cross-border payment from US company to European supplier",
    "message_type": "MT103"
  },
  
  "variables": {
    "currency": "EUR",
    "amount": 125000.50,
    "value_date": "241220",
    "reference": "FT2024120612345",
    "sender_bic": "CHASUS33XXX",
    "receiver_bic": "DEUTDEFFXXX"
  },
  
  "message": {
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
          {"fake": "company", "locale": "en_US"},
          {"fake": "street_address", "locale": "en_US"},
          {"fake": "city_with_state", "locale": "en_US"},
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
          {"fake": "company", "locale": "de_DE"},
          {"fake": "street_address", "locale": "de_DE"},
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
  },
  
  "validation": {
    "expected_result": "pass",
    "compliance_checks": ["sanctions", "aml"]
  }
}
```

### Example 2: MT202 Interbank Transfer

This example shows a bank-to-bank liquidity transfer:

```json
{
  "scenario": {
    "id": "mt202_liquidity_transfer",
    "name": "USD Liquidity Transfer",
    "description": "Large value interbank transfer for liquidity management",
    "message_type": "MT202"
  },
  
  "variables": {
    "currency": "USD",
    "amount": 50000000.00,
    "value_date": "241215",
    "reference": "LIQ202412150001"
  },
  
  "message": {
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
        "value": "CHASUS33XXX"
      },
      "53B": {
        "account": "/12345678901234567890",
        "optional": true,
        "probability": 0.3
      },
      "56A": {
        "value": "BARCGB22XXX",
        "optional": true,
        "probability": 0.5
      },
      "57A": {
        "value": "DEUTDEFFXXX"
      },
      "58A": {
        "value": "BNPAFRPPXXX"
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

### Example 3: MT103 with Multiple Currencies (Testing Edge Cases)

This example demonstrates fee handling with currency conversion:

```json
{
  "scenario": {
    "id": "mt103_currency_conversion",
    "name": "Payment with Currency Conversion",
    "description": "USD payment to EUR account with fee details",
    "message_type": "MT103"
  },
  
  "variables": {
    "send_currency": "USD",
    "receive_currency": "EUR",
    "send_amount": 100000.00,
    "exchange_rate": 0.92,
    "receive_amount": 92000.00,
    "value_date": "241218",
    "reference": "FX20241218001"
  },
  
  "message": {
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
          {"fake": "name", "locale": "en_US"},
          {"fake": "street_address", "locale": "en_US"},
          "NEW YORK NY 10001",
          "UNITED STATES"
        ]
      },
      "59": {
        "account": "/FR1420041010050500013M02606",
        "name_address": [
          {"fake": "name", "locale": "fr_FR"},
          {"fake": "street_address", "locale": "fr_FR"},
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

## Best Practices

### 1. Self-Contained Scenarios

Each scenario configuration should be complete and independent:

- Define all required variables at the root level
- Include all necessary fields for a valid message
- Specify validation expectations

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

Leverage the `fake` crate for realistic data:

```json
{
  "50K": {
    "name_address": [
      {"fake": "company", "locale": "en_US"},      // Realistic company names
      {"fake": "street_address", "locale": "en_US"}, // Real-looking addresses
      {"fake": "city_with_state", "locale": "en_US"}
    ]
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

```json
{
  "scenario": {
    "id": "mt103_max_length_fields",
    "description": "Test maximum field lengths"
  },
  "variables": {
    "long_reference": "ABCDEFGHIJKLMNOP",  // 16 chars - max for field 20
    "max_amount": 999999999999999.99      // Maximum decimal value
  }
}
```

## Reference

### Configuration Structure

```json
{
  "scenario": {
    "id": "string",              // Unique identifier
    "name": "string",            // Human-readable name
    "description": "string",     // Test purpose
    "message_type": "string"     // MT message type (e.g., "MT103")
  },
  "variables": {
    // Pre-generated values shared across fields
    "variable_name": "value"
  },
  "message": {
    "headers": {
      // Optional SWIFT headers configuration
    },
    "fields": {
      // Field configurations
    }
  },
  "validation": {
    "expected_result": "pass|fail",
    "compliance_checks": ["array of checks"]
  }
}
```

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

The `fake` crate can generate the following types of data:

| Type | Description | Locales |
|------|-------------|---------|
| `name` | Person's full name | All |
| `first_name` | First name only | All |
| `last_name` | Last name only | All |
| `company` | Company name | All |
| `street_address` | Street address | All |
| `city` | City name | All |
| `state` | State/province | US, CA |
| `city_with_state` | City, State format | US |
| `zip` | Postal code | All |
| `country` | Country name | All |
| `phone` | Phone number | All |
| `iban` | IBAN account number | EU |

### Common Locales

- `en_US` - United States
- `en_GB` - United Kingdom  
- `de_DE` - Germany
- `fr_FR` - France
- `es_ES` - Spain
- `it_IT` - Italy
- `nl_NL` - Netherlands
- `ch_DE` - Switzerland (German)
- `jp_JP` - Japan

### Variable Syntax

Variables are referenced using `${variable_name}` syntax and can be concatenated:

```json
{
  "variables": {
    "currency": "USD",
    "amount": "50000.00"
  },
  "message": {
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

This scenario-based testing approach provides a clean, maintainable way to generate realistic SWIFT MT message test data. By combining pre-generated variables, the `fake` crate for realistic data, and static values, you can create comprehensive test suites that accurately simulate real-world financial messaging scenarios.

The self-contained nature of each scenario configuration ensures that tests are reproducible and easy to understand, while the flexibility of the system allows for testing edge cases and compliance requirements.