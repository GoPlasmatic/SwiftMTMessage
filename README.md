# Swift MT Message Parser

A comprehensive Rust library for parsing SWIFT MT (Message Type) messages and extracting their fields. This library focuses purely on parsing and field extraction, providing type-safe access to message data without message building or transformation capabilities.

## Features

- **🚀 Pure Parsing Library**: Focused exclusively on parsing and field extraction
- **📋 Complete MT Support**: Full implementation of 11 MT message types with specific field extraction methods
- **🔒 Type-Safe Field Access**: Strongly typed field extraction with proper error handling
- **⚡ Zero-Copy Parsing**: Efficient parsing with minimal memory allocation where possible
- **🛡️ Comprehensive Validation**: Multi-level validation framework (Basic, Standard, Strict)
- **📊 Advanced Parsing**: Complex field parsing for amounts, dates, balances, and statement lines
- **🎯 Extensible Architecture**: Easy to add new message types
- **📖 Rich Documentation**: Comprehensive examples and API documentation
- **🧪 Thoroughly Tested**: 151+ unit tests covering all functionality

## Supported Message Types

| Message Type | Description | Implementation Status |
|--------------|-------------|----------------------|
| **MT102** | Multiple Customer Credit Transfer | ✅ Complete |
| **MT103** | Single Customer Credit Transfer | ✅ Complete |
| **MT192** | Request for Cancellation | ✅ Complete |
| **MT195** | Queries | ✅ Complete |
| **MT196** | Answers | ✅ Complete |
| **MT197** | Copy of a Message | ✅ Complete |
| **MT199** | Free Format Message | ✅ Complete |
| **MT202** | General Financial Institution Transfer | ✅ Complete |
| **MT940** | Customer Statement Message | ✅ Complete |
| **MT941** | Balance Report Message | ✅ Complete |
| **MT942** | Interim Transaction Report | ✅ Complete |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
swift-mt-message = "0.1.0"
```

## Quick Start

```rust
use swift_mt_message::{parse_message, MTMessage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message_text = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:ORDERING CUSTOMER
COMPANY ABC
:59:BENEFICIARY CUSTOMER
COMPANY XYZ
:70:INVOICE PAYMENT
-}"#;

    // Parse the message
    let message = parse_message(message_text)?;
    
    println!("Message type: {}", message.message_type());
    
    // Extract fields with type-safe methods
    if let MTMessage::MT103(mt103) = message {
        println!("Sender Reference: {}", mt103.sender_reference()?);
        
        let amount = mt103.amount()?;
        println!("Amount: {} {}", amount.value, amount.currency);
        
        println!("Value Date: {}", mt103.value_date()?);
        println!("Ordering Customer: {}", mt103.ordering_customer()?);
        println!("Beneficiary: {}", mt103.beneficiary()?);
    }
    
    Ok(())
}
```

## Detailed Usage Examples

### Payment Messages

#### MT103 - Single Customer Credit Transfer

```rust
use swift_mt_message::{parse_message, MTMessage};

let mt103_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:MT103REF123456
:23B:CRED
:32A:210315EUR1000000,00
:50K:ORDERING CUSTOMER
COMPANY ABC
:59:BENEFICIARY CUSTOMER
COMPANY XYZ
:70:INVOICE PAYMENT INV-2021-001
:71A:OUR
-}"#;

let message = parse_message(mt103_message)?;

if let MTMessage::MT103(mt103) = message {
    // Required fields
    println!("Reference: {}", mt103.sender_reference()?);
    println!("Bank Operation: {}", mt103.bank_operation_code()?);
    
    // Parsed amount with currency extraction
    let amount = mt103.amount()?;
    println!("Amount: {} {}", amount.value, amount.currency);
    
    // Date parsing
    println!("Value Date: {}", mt103.value_date()?);
    
    // Customer information
    println!("Ordering Customer: {}", mt103.ordering_customer()?);
    println!("Beneficiary: {}", mt103.beneficiary()?);
    
    // Optional fields
    if let Some(remittance) = mt103.remittance_information() {
        println!("Remittance: {}", remittance);
    }
    
    if let Some(charges) = mt103.details_of_charges() {
        println!("Charges: {}", charges);
    }
}
```

#### MT102 - Multiple Customer Credit Transfer

```rust
if let MTMessage::MT102(mt102) = message {
    println!("Transaction Type: {:?}", mt102.transaction_type_code());
    println!("Number of Transactions: {:?}", mt102.number_of_transactions());
    
    let total_amount = mt102.amount()?;
    println!("Total Amount: {} {}", total_amount.value, total_amount.currency);
    
    // Multiple beneficiaries
    let beneficiaries = mt102.beneficiaries();
    println!("Beneficiaries: {}", beneficiaries.len());
    
    // Multiple transaction references
    let references = mt102.transaction_references();
    println!("Transaction References: {:?}", references);
}
```

#### MT202 - General Financial Institution Transfer

```rust
if let MTMessage::MT202(mt202) = message {
    println!("Transaction Reference: {}", mt202.transaction_reference()?);
    
    let amount = mt202.amount()?;
    println!("Amount: {} {}", amount.value, amount.currency);
    
    println!("Beneficiary Institution: {}", mt202.beneficiary_institution()?);
    
    // Multiple institution format support
    if let Some(ordering) = mt202.ordering_institution() {
        println!("Ordering Institution (52A): {}", ordering);
    }
    if let Some(ordering_d) = mt202.ordering_institution_d() {
        println!("Ordering Institution (52D): {}", ordering_d);
    }
}
```

### System Messages

#### MT192 - Request for Cancellation

```rust
if let MTMessage::MT192(mt192) = message {
    println!("Cancellation Reference: {}", mt192.transaction_reference()?);
    println!("Original Reference: {}", mt192.related_reference()?);
    
    if let Some(reason) = mt192.reason_for_cancellation() {
        println!("Cancellation Reason: {}", reason);
    }
    
    if let Some(msg_type) = mt192.original_message_type() {
        println!("Original Message Type: MT{}", msg_type);
    }
    
    // Multiple narrative support
    let narratives = mt192.narratives();
    for (i, narrative) in narratives.iter().enumerate() {
        println!("Narrative {}: {}", i + 1, narrative);
    }
}
```

#### MT199 - Free Format Message

```rust
if let MTMessage::MT199(mt199) = message {
    println!("Reference: {}", mt199.transaction_reference()?);
    
    if let Some(subject) = mt199.message_subject() {
        println!("Subject: {}", subject);
    }
    
    // Multiple free format text fields
    let free_texts = mt199.all_free_format_text();
    for text in free_texts {
        println!("Free Text: {}", text);
    }
    
    // Message categories
    let categories = mt199.all_message_categories();
    println!("Categories: {:?}", categories);
}
```

### Statement Messages

#### MT940 - Customer Statement Message

```rust
if let MTMessage::MT940(mt940) = message {
    println!("Statement Reference: {}", mt940.transaction_reference()?);
    println!("Account: {}", mt940.account_identification()?);
    println!("Statement Number: {}", mt940.statement_number()?);
    
    // Balance parsing with D/C indicator, date, currency, and amount
    let (dc, date, currency, amount) = mt940.parse_opening_balance()?;
    println!("Opening Balance: {} {} {} {}", dc, date, currency, amount);
    
    let (dc, date, currency, amount) = mt940.parse_closing_balance()?;
    println!("Closing Balance: {} {} {} {}", dc, date, currency, amount);
    
    // Statement lines with complex parsing
    let lines = mt940.statement_lines();
    println!("Statement Lines: {}", lines.len());
    
    // Parse individual statement lines
    for line in lines {
        if let Ok(parsed) = mt940.parse_statement_line(&line) {
            println!("Transaction Date: {}", parsed.value_date);
            if let Some(entry_date) = parsed.entry_date {
                println!("Entry Date: {}", entry_date);
            }
        }
    }
    
    // Information to account owner
    let info_lines = mt940.information_to_account_owner();
    for info in info_lines {
        println!("Info: {}", info);
    }
}
```

#### MT941 - Balance Report Message

```rust
if let MTMessage::MT941(mt941) = message {
    // Comprehensive balance summary
    let summary = mt941.balance_summary()?;
    
    println!("Opening: {} {} {} {}", 
             summary.opening_balance.0, 
             summary.opening_balance.1, 
             summary.opening_balance.2, 
             summary.opening_balance.3);
    
    println!("Closing: {} {} {} {}", 
             summary.closing_balance.0, 
             summary.closing_balance.1, 
             summary.closing_balance.2, 
             summary.closing_balance.3);
    
    if let Some(available) = summary.closing_available_balance {
        println!("Available: {} {} {} {}", available.0, available.1, available.2, available.3);
    }
    
    println!("Forward Balances: {}", summary.forward_available_balances.len());
}
```

#### MT942 - Interim Transaction Report

```rust
if let MTMessage::MT942(mt942) = message {
    if let Some(date_time) = mt942.date_time_indication() {
        println!("Date/Time: {}", date_time);
    }
    
    // Floor limit parsing
    if let Some(Ok((currency, amount))) = mt942.parse_floor_limit() {
        println!("Floor Limit: {} {}", currency, amount);
    }
    
    // Interim summary with transaction count
    let summary = mt942.interim_summary()?;
    println!("Transactions Above Floor Limit: {}", summary.transaction_count);
    
    // Only transactions above floor limit are reported
    let lines = mt942.statement_lines();
    for line in lines {
        println!("Large Transaction: {}", line);
    }
}
```

## Generic Field Access

All message types implement the `MTMessageType` trait for generic field access:

```rust
use swift_mt_message::messages::MTMessageType;

// Works with any message type
fn print_field_20(message: &dyn MTMessageType) {
    if let Some(field) = message.get_field("20") {
        println!("Transaction Reference: {}", field.value());
    }
}

// Get multiple fields with the same tag
let narrative_fields = message.get_fields("72");
for field in narrative_fields {
    println!("Narrative: {}", field.value());
}

// Get all fields
let all_fields = message.get_all_fields();
for field in all_fields {
    println!("Field {}: {}", field.tag().as_str(), field.value());
}
```

## Validation Framework

```rust
use swift_mt_message::{validate_message, ValidationLevel};

let message = parse_message(message_text)?;

// Different validation levels
let basic_result = validate_message(&message, ValidationLevel::Basic);
let standard_result = validate_message(&message, ValidationLevel::Standard);
let strict_result = validate_message(&message, ValidationLevel::Strict);

if standard_result.is_valid() {
    println!("Message is valid");
} else {
    for error in standard_result.errors() {
        println!("Validation error: {}", error.message);
    }
}

// Handle warnings
if standard_result.has_warnings() {
    for warning in standard_result.warnings() {
        println!("Warning: {}", warning.message);
    }
}
```

## Advanced Features

### Amount and Currency Parsing

```rust
// Automatic currency extraction and amount parsing
let amount = mt103.amount()?;
println!("Value: {}", amount.value);      // f64
println!("Currency: {}", amount.currency); // String (e.g., "EUR", "USD")

// Works with all amount fields across message types
let total_amount = mt102.sum_of_amounts()?;
let floor_limit = mt942.parse_floor_limit().unwrap()?;
```

### Date Parsing

```rust
use chrono::Datelike;

// Automatic SWIFT date parsing (YYMMDD format)
let value_date = mt103.value_date()?;
println!("Year: {}", value_date.year());
println!("Month: {}", value_date.month());
println!("Day: {}", value_date.day());

// Also supports YYYYMMDD format where applicable
```

### Balance Parsing

```rust
// Complex balance field parsing with D/C indicator
let (debit_credit, date, currency, amount) = mt940.parse_opening_balance()?;
println!("Type: {}", debit_credit); // "D" or "C"
println!("Date: {}", date);
println!("Currency: {}", currency);
println!("Amount: {}", amount);
```

## Error Handling

Comprehensive error types with detailed context:

```rust
use swift_mt_message::MTError;

match parse_message(invalid_message) {
    Ok(message) => { /* handle success */ }
    Err(MTError::ParseError { line, column, message }) => {
        println!("Parse error at {}:{}: {}", line, column, message);
    }
    Err(MTError::UnsupportedMessageType { message_type }) => {
        println!("Unsupported message type: {}", message_type);
    }
    Err(MTError::MissingRequiredField { field_tag }) => {
        println!("Missing required field: {}", field_tag);
    }
    Err(MTError::InvalidFieldFormat { field, message }) => {
        println!("Invalid format in field {}: {}", field, message);
    }
    Err(MTError::ValidationError { field, message }) => {
        println!("Validation error in field {}: {}", field, message);
    }
    Err(MTError::FieldNotFound { field_tag }) => {
        println!("Field not found: {}", field_tag);
    }
    Err(err) => {
        println!("Other error: {}", err);
    }
}
```

## Examples

Run the included examples to see the library in action:

```bash
# Basic parsing example
cargo run --example basic_parsing

# Comprehensive demo of all message types
cargo run --example comprehensive_demo

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Performance

- **Zero-copy parsing** where possible for optimal performance
- **Minimal allocations** during field extraction
- **Efficient regex patterns** for field parsing
- **151+ unit tests** ensuring reliability and correctness

## Dependencies

Minimal and carefully chosen dependencies:

- `chrono` - Date and time handling
- `serde` - Serialization support
- `thiserror` - Error handling
- `regex` - Pattern matching for field parsing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Adding New Message Types

1. Create a new file in `src/messages/` (e.g., `mt104.rs`)
2. Implement the `MTMessageType` trait with specific field extraction methods
3. Add the message type to the `MTMessage` enum in `src/messages/mod.rs`
4. Update the parser in `src/parser.rs` to handle the new message type
5. Add comprehensive tests and documentation
6. Update this README with the new message type

### Development Guidelines

- Follow existing code patterns and naming conventions
- Add comprehensive unit tests for all functionality
- Include documentation examples that compile and run
- Use type-safe field extraction methods
- Handle errors gracefully with detailed error messages

## License

This project is licensed under the Apache License v2 - see the [LICENSE](LICENSE) file for details.
