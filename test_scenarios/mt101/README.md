# MT101 Test Scenarios

This directory contains comprehensive test scenarios for MT101 (Request for Transfer) messages, designed to cover various corporate payment request use cases.

## Overview

MT101 is used by corporations and financial institutions to request the movement of funds from accounts serviced at the receiving institution. It supports multiple transactions in a single message with a two-sequence structure:
- **Sequence A**: General information (header)
- **Sequence B**: Transaction details (repetitive)

## Scenario Overview

### 1. **standard.json**
- **Purpose**: Basic request for transfer with two transactions
- **Use Case**: Standard corporate payment requests
- **Key Features**:
  - Two simple transactions
  - Single currency
  - Basic party identification (50F)
  - SHA charges
- **Fields Used**: 20, 28D, 50#2, 52, 30, transactions with 21, 32B, 57, 59, 70, 71A

### 2. **bulk_payment.json**
- **Purpose**: Large batch payment processing
- **Use Case**: Corporate accounts payable batch runs
- **Key Features**:
  - 5 transactions in single message
  - Customer reference (21R) for batch tracking
  - Account identification (25)
  - Instruction codes (23E: INTC)
  - All same currency (USD)
- **Fields Used**: 20, 21R, 28D, 50#2, 52, 30, 25, transactions with 21, 23E, 32B, 57, 59, 70, 71A

### 3. **multi_currency.json**
- **Purpose**: Foreign exchange and multi-currency payments
- **Use Case**: International treasury operations
- **Key Features**:
  - FX conversions (21F, 33B, 36)
  - Multiple currencies (USD, EUR, GBP, JPY)
  - Exchange rate specifications
  - Equivalent transfer instruction (EQUI)
  - Intermediary institutions (56)
- **Fields Used**: 20, 28D, 50#2, 52, 30, transactions with 21, 21F, 23E, 32B, 33B, 36, 56, 57, 59, 70, 71A

### 4. **scheduled_payment.json**
- **Purpose**: Future-dated and standing order payments
- **Use Case**: Recurring payments, scheduled obligations
- **Key Features**:
  - Future execution date (30)
  - Instructing party (50#1)
  - Sending institution (51A)
  - Phone confirmation instruction (PHON)
  - Standing order reference
- **Fields Used**: 20, 21R, 28D, 50#1, 50#2, 52, 51A, 30, 25, transactions with 21, 23E, 32B, 57, 59, 70, 71A

### 5. **salary_payment.json**
- **Purpose**: Payroll batch processing
- **Use Case**: Monthly salary and bonus payments
- **Key Features**:
  - Employee-specific references
  - OUR charges (employer pays all)
  - Charges account (25A)
  - HR department identification
  - Mix of salary and bonus payments
- **Fields Used**: 20, 21R, 28D, 50#2, 52, 30, 25, transactions with 21, 32B, 57, 59, 70, 71A, 25A

### 6. **vendor_payment.json**
- **Purpose**: Supplier and vendor settlements
- **Use Case**: Accounts payable processing
- **Key Features**:
  - Invoice references in transaction IDs
  - Regulatory reporting (77B)
  - Notification instructions (TELE)
  - Mixed charge options (SHA, BEN)
  - Intermediary routing (56)
- **Fields Used**: 20, 28D, 50#2, 52, 30, transactions with 21, 23E, 32B, 56, 57, 59, 70, 71A, 77B

### 7. **urgent_payment.json**
- **Purpose**: Time-critical payment processing
- **Use Case**: Emergency transfers, margin calls
- **Key Features**:
  - Urgent priority (U)
  - Urgent processing instructions (URGP)
  - Phone notifications (TELI, PHOB)
  - High-value amounts
  - OUR charges for speed
- **Fields Used**: 20, 21R, 28D, 50#2, 52, 30, transactions with 21, 23E, 32B, 57, 59, 70, 71A

### 8. **direct_debit.json**
- **Purpose**: Direct debit collection requests
- **Use Case**: Subscription collections, utility payments
- **Key Features**:
  - Instructing party as collector (50#1)
  - Individual debtor information (50#2 in transactions)
  - Hold instruction (HOLD)
  - Mandate references
  - Collection account routing
- **Fields Used**: 20, 21R, 28D, 50#1, 50#2, 52, 30, transactions with 21, 23E, 32B, 50#2, 57, 59, 70, 71A

### 9. **minimal.json**
- **Purpose**: Minimum required fields only
- **Use Case**: Testing edge cases, basic validation
- **Key Features**:
  - Single transaction
  - Only mandatory fields
  - No optional sequences
  - Simplified structure
- **Fields Used**: 20, 28D, 30, transaction with 21, 32B, 59, 71A

## Field Structure

All scenarios follow the same consistent structure:
- `variables`: Dynamic values using datafake generation
- `schema.basic_header`: SWIFT basic header (Block 1)
- `schema.application_header`: SWIFT application header (Block 2)
- `schema.fields`: MT101 specific fields (Block 4)
  - Main sequence fields (20, 28D, 30, etc.)
  - `"#"` containing the transactions array

## Transaction Structure

Each transaction in the `transactions` array can contain:
- **21**: Transaction Reference (mandatory)
- **21F**: F/X Deal Reference (with field 36)
- **23E**: Instruction Codes
- **32B**: Currency/Amount (mandatory)
- **33B**: Original Currency/Amount (for FX)
- **36**: Exchange Rate
- **50**: Ordering Customer (transaction-specific)
- **52**: Account Servicing Institution
- **56**: Intermediary Institution
- **57**: Account With Institution
- **59**: Beneficiary Customer (mandatory)
- **70**: Remittance Information
- **71A**: Details of Charges (mandatory)
- **25A**: Charges Account
- **77B**: Regulatory Reporting

## Usage

These scenarios work with the SwiftMTMessage library's `generate_sample` function:

```rust
use swift_mt_message::{generate_sample, messages::mt101::MT101};

// Generate a specific scenario
let bulk_payment = generate_sample::<MT101>("MT101", Some("bulk_payment"))?;

// Generate default (standard) scenario
let standard_payment = generate_sample::<MT101>("MT101", None)?;
```

## Validation Rules

MT101 has specific validation rules:
- **C1**: If field 36 present, field 21F is mandatory
- **C2**: If field 33B present and amount ≠ 0, field 36 is mandatory
- **C3, C4**: Field 50 placement rules between sequences
- **C5**: Currency in field 33B must differ from field 32B
- **C7**: If field 56 present, field 57 is mandatory
- **C8**: If field 21R present, all 32B currencies must match

## Testing

All scenarios have been validated for:
- ✅ Valid JSON syntax
- ✅ Consistent structure
- ✅ Appropriate field usage for each use case
- ✅ SWIFT MT101 compliance
- ✅ Proper sequence structure (A and B)