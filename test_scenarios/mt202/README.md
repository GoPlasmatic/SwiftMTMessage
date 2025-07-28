# MT202 Test Scenarios

This directory contains test scenarios for MT202 (General Financial Institution Transfer) messages, with a focus on CBPR+ cover payment scenarios.

## Scenario Overview

### 1. **cbpr_cov_standard.json**
- **Purpose**: CBPR+ compliant cover payment for underlying MT103
- **Use Case**: Bank-to-bank transfer covering a CBPR+ customer payment
- **Key Features**:
  - Sequence A: Standard MT202 interbank transfer
  - Sequence B: Underlying customer details with CBPR+ data
  - Purpose code propagation from MT103
  - LEI information for ordering/beneficiary parties
  - Transparency data maintenance
  - Related reference to original MT103
- **Fields**: 
  - Sequence A: 20, 21, 32A, 52A, 56A, 57A, 58A, 72
  - Sequence B: 50K, 52B, 56B, 57B, 59, 70, 33B, 72B

### 2. **cbpr_cov_complex_routing.json**
- **Purpose**: CBPR+ cover payment with complex multi-hop routing and FX conversion
- **Use Case**: Cross-border payment requiring multiple intermediaries and currency conversion
- **Key Features**:
  - 3+ intermediaries in routing chain (53A, 54A, 56A)
  - Foreign exchange conversion with rate details
  - Time indications for processing windows (13C)
  - Urgent priority processing
  - UETR tracking for transparency
  - LEI identifiers for intermediary banks
  - Original and settlement currency tracking
  - Detailed routing transparency in field 72B
- **Fields**: 
  - Sequence A: 20, 21, 13C, 32A, 52A, 53A, 54A, 56A, 57A, 58A, 72
  - Sequence B: 50K, 52B, 53B, 54B, 56B, 57B, 59, 70, 33B, 72B

### 3. **cbpr_cov_compliance_enhanced.json**
- **Purpose**: CBPR+ cover payment with enhanced compliance and sanctions screening
- **Use Case**: High-value transfers requiring comprehensive compliance verification
- **Key Features**:
  - Sanctions screening references
  - KYC and AML verification tracking
  - Nested LEI structure for all parties
  - Ultimate debtor/creditor identification
  - Compliance status in multiple fields
  - Enhanced party identification with verification markers
  - Complete transparency for regulatory review
  - Detailed compliance audit trail in field 72B
- **Fields**: 
  - Sequence A: 20, 21, 32A, 52A, 56A, 57A, 58A, 72
  - Sequence B: 50K, 52B, 56B, 57B, 59, 70, 72B

## CBPR+ Specific Elements

### Purpose Code Propagation
- Field 70 includes `/PURP/` code from underlying MT103
- Field 72 indicates CBPR+ compliance status

### LEI Information
- Field 50K includes `/LEI...` for ordering customer
- Field 59 includes `/LEI...` for beneficiary customer

### Transparency Data
- Field 72B contains fee and processing time information
- Field 33B shows instructed amount for transparency

## Usage

These scenarios work with the SwiftMTMessage library's sample generation:

```rust
use swift_mt_message::{generate_sample, messages::mt202::MT202};

// Generate CBPR+ cover payment scenarios
let cbpr_standard = generate_sample::<MT202>("MT202", Some("cbpr_cov_standard"))?;
let cbpr_complex = generate_sample::<MT202>("MT202", Some("cbpr_cov_complex_routing"))?;
let cbpr_compliance = generate_sample::<MT202>("MT202", Some("cbpr_cov_compliance_enhanced"))?;
```

## Validation

All scenarios have been validated for:
- ✅ MT202 message structure compliance
- ✅ Proper sequence A/B field usage
- ✅ CBPR+ data element propagation
- ✅ Cover payment best practices

## Field Structure

The scenario follows the standard structure:
- `variables`: Dynamic values for BICs, amounts, references
- `schema.basic_header`: SWIFT basic header (Block 1)
- `schema.application_header`: SWIFT application header (Block 2)
- `schema.fields`: MT202 specific fields with sequence separation