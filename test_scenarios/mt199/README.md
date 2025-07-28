# MT199 Test Scenarios

This directory contains test scenarios for MT199 (Free Format Message) messages, focusing on CBPR+ payment inquiries and communications.

## Scenario Overview

### 1. **cbpr_inquiry.json**
- **Purpose**: CBPR+ payment status and transparency inquiry
- **Use Case**: Bank requesting status and transparency data for CBPR+ payment
- **Key Features**:
  - Reference to original CBPR+ payment (Field 21)
  - Detailed inquiry narrative (Field 79)
  - Specific CBPR+ data requests
  - Transparency information requirements
  - Urgency and response timeframe
  - Contact information
- **Fields**: 20, 21, 79

### 2. **cbpr_cancellation.json**
- **Purpose**: CBPR+ payment cancellation request
- **Use Case**: Requesting cancellation of pending/in-flight CBPR+ payment
- **Key Features**:
  - /REJT/ format for cancellation
  - Multiple cancellation reason codes (CUST, DUPL, FRAD, TECH, AGNT)
  - UETR reference for tracking
  - Legal basis and regulatory compliance
  - Time-critical processing requirement
  - Detailed payment information
- **Fields**: 20, 21, 79

## CBPR+ Inquiry Elements

### Information Requested
The scenario demonstrates typical CBPR+ inquiries for:
1. Payment status and current location
2. Fee transparency breakdown
3. Processing time (actual vs estimated)
4. Compliance holds or issues
5. Beneficiary credit confirmation
6. Full transaction trail with timestamps

### CBPR+ Specific Data
- Purpose code validation status
- LEI verification results
- Structured remittance processing status
- Regulatory compliance confirmation
- Transparency requirements compliance

## Usage

These scenarios work with the SwiftMTMessage library:

```rust
use swift_mt_message::{generate_sample, messages::mt199::MT199};

// Generate CBPR+ inquiry scenario
let cbpr_inquiry = generate_sample::<MT199>("MT199", Some("cbpr_inquiry"))?;
```

## Cancellation Reason Codes

The cancellation scenario supports multiple reason codes:
- **CUST**: Customer requested cancellation
- **DUPL**: Duplicate payment detected
- **FRAD**: Suspected fraudulent transaction
- **TECH**: Technical error in processing
- **AGNT**: Agent/intermediary error

## Response Handling

### For Inquiries
MT199 inquiries typically receive MT299 responses containing:
- Current payment status
- Complete fee breakdown
- Actual processing times
- Any issues encountered
- Compliance status
- Next steps or expected completion

### For Cancellations
MT199 cancellation requests typically receive:
- MT192/MT292 for formal cancellation confirmation
- MT196/MT296 for status updates
- Return of funds via appropriate payment message

## Validation

All scenarios have been validated for:
- ✅ MT199 message structure
- ✅ Proper free format narrative
- ✅ Reference field usage
- ✅ CBPR+ inquiry best practices

## Field Structure

The scenario follows the standard structure:
- `variables`: Dynamic references and BICs
- `schema.basic_header`: SWIFT basic header
- `schema.application_header`: SWIFT application header
- `schema.fields`: MT199 fields (minimal structure, maximum flexibility)