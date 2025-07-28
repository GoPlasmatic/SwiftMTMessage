# CBPR+ Test Scenarios Design

## Overview
This document outlines a minimal but comprehensive test scenario structure for SWIFT CBPR+ (Cross-Border Payments and Reporting Plus) real-world use cases.

## Scenario Categories

### 1. MT103 CBPR+ Scenarios (6 scenarios)

#### a. `mt103_cbpr_person_to_person.json`
- **Purpose**: Person-to-person cross-border payment
- **Key Features**:
  - Purpose code: `/PURP/CASH` (cash management)
  - Minimal remittance information
  - Individual names in Field 50K and 59
  - No LEI required
  - Amount: 500-5,000 USD

#### b. `mt103_cbpr_business_payment.json`
- **Purpose**: Business-to-business payment with structured remittance
- **Key Features**:
  - Purpose code: `/PURP/GDDS` (purchase of goods)
  - LEI in Field 50K and 59 (when available)
  - Structured remittance: `/INV/`, `/PO/`, `/DTL/`
  - Field 77T with extended invoice details
  - Amount: 25,000-250,000 USD

#### c. `mt103_cbpr_real_estate.json`
- **Purpose**: Real estate transaction payment
- **Key Features**:
  - Purpose code: `/PURP/PHYS` (purchase of physical assets)
  - Property reference in Field 70
  - Legal documentation references
  - Escrow account details
  - Amount: 100,000-2,000,000 USD

#### d. `mt103_cbpr_trade_finance.json`
- **Purpose**: Trade finance payment with documentary credit
- **Key Features**:
  - Purpose code: `/PURP/TRAD` (trade finance)
  - L/C reference in Field 70
  - Bill of lading details in Field 77T
  - Trade terms (Incoterms)
  - Amount: 50,000-1,000,000 USD

#### e. `mt103_cbpr_tax_payment.json`
- **Purpose**: Cross-border tax payment
- **Key Features**:
  - Purpose code: `/PURP/TAXS` (tax payment)
  - Tax reference number
  - Tax period information
  - Government entity as beneficiary
  - Amount: 10,000-500,000 USD

#### f. `mt103_cbpr_validation_failure.json`
- **Purpose**: Test CBPR+ validation failures
- **Key Features**:
  - Missing mandatory CBPR+ elements
  - Invalid purpose codes
  - Incomplete transparency data
  - For negative testing

### 2. MT103 STP CBPR+ Scenarios (2 scenarios)

#### a. `mt103_stp_cbpr_compliant.json`
- **Purpose**: Fully STP-compliant CBPR+ payment
- **Key Features**:
  - All mandatory STP fields
  - ISO 11649 creditor reference
  - Complete transparency information
  - No manual intervention required
  - Amount: 10,000-100,000 EUR

#### b. `mt103_stp_cbpr_enhanced.json`
- **Purpose**: STP with enhanced CBPR+ data
- **Key Features**:
  - Extended remittance in Field 77T
  - Multiple invoice references
  - VAT/tax breakdowns
  - Full supply chain visibility
  - Amount: 50,000-500,000 EUR

### 3. MT202 COV CBPR+ Scenarios (3 scenarios)

#### a. `mt202_cov_cbpr_standard.json`
- **Purpose**: Standard cover payment for MT103 CBPR+
- **Key Features**:
  - Sequence B with underlying customer details
  - Purpose code propagation
  - LEI information when available
  - Correspondent banking chain
  - Amount: Matching underlying MT103

#### b. `mt202_cov_cbpr_complex_routing.json`
- **Purpose**: Complex correspondent banking with CBPR+
- **Key Features**:
  - Multiple intermediaries (Field 56)
  - Currency conversion details
  - Enhanced transparency through chain
  - Regulatory information in Field 72
  - Amount: 100,000-1,000,000 USD

#### c. `mt202_cov_cbpr_compliance_enhanced.json`
- **Purpose**: Enhanced compliance for high-risk corridors
- **Key Features**:
  - Additional KYC/AML data
  - Source of funds information
  - Enhanced due diligence markers
  - Regulatory reporting codes
  - Amount: 50,000-500,000 USD

### 4. MT199/299 CBPR+ Scenarios (2 scenarios)

#### a. `mt199_cbpr_inquiry.json`
- **Purpose**: CBPR+ payment status inquiry
- **Key Features**:
  - Reference to CBPR+ payment
  - Request for transparency data
  - Compliance status query
  - Processing timeline request

#### b. `mt299_cbpr_response.json`
- **Purpose**: Response to CBPR+ inquiry
- **Key Features**:
  - Complete transparency trail
  - Processing timestamps
  - Fee transparency
  - Regulatory compliance status

### 5. Error and Exception Scenarios (3 scenarios)

#### a. `mt103_cbpr_rejection.json`
- **Purpose**: CBPR+ payment rejection
- **Key Features**:
  - `/REJT/` codes specific to CBPR+
  - Missing transparency data
  - Invalid purpose code
  - Compliance failure reasons

#### b. `mt103_cbpr_return.json`
- **Purpose**: CBPR+ payment return
- **Key Features**:
  - `/RETN/` with CBPR+ specific reasons
  - Beneficiary account issues
  - Regulatory blocks
  - Original payment reference

#### c. `mt202_cov_cbpr_mismatch.json`
- **Purpose**: Cover payment data mismatch
- **Key Features**:
  - Inconsistent purpose codes
  - Missing LEI in cover
  - Amount discrepancies
  - For reconciliation testing

## Implementation Guidelines

### 1. Common CBPR+ Elements

All CBPR+ scenarios should include:
- **Transaction Reference**: Unique identifier with CBPR+ prefix
- **Purpose Code**: In Field 70 as `/PURP/xxxx`
- **LEI**: When applicable, format: `/LEI/12345678901234567890`
- **Transparency Data**: Processing fees, exchange rates, timelines

### 2. Field Usage Patterns

#### Field 70 (Remittance Information)
```
/PURP/GDDS
/INV/2024-001234
/RFB/CUST-REF-789
Additional payment details
```

#### Field 72 (Sender to Receiver)
```
/ACC/CBPR+ COMPLIANT
/INS/TRANSPARENCY REQUIRED
/BNF/LEI VERIFICATION DONE
```

#### Field 77T (Envelope Contents)
```
/NARR/EXTENDED REMITTANCE/
/INVOICE/INV-2024-001234/
/ITEMS/Widget A x100/
/AMOUNT/10000.00/
/VAT/2100.00/
/TERMS/NET30/
```

### 3. Validation Rules

Each scenario should test:
1. **Mandatory CBPR+ Fields**: Purpose code, transparency data
2. **Format Compliance**: LEI format, structured codes
3. **Business Rules**: Valid purpose codes for transaction type
4. **Cross-Message Consistency**: MT103 to MT202 COV mapping

### 4. Amount Ranges by Use Case

- **Person-to-Person**: 100 - 10,000 (various currencies)
- **Business Payments**: 10,000 - 1,000,000
- **Real Estate**: 100,000 - 5,000,000
- **Trade Finance**: 25,000 - 10,000,000
- **Tax Payments**: 1,000 - 1,000,000

### 5. Testing Coverage Matrix

| Scenario Type | Purpose Codes | LEI | Structured Remittance | Transparency | Validation |
|--------------|---------------|-----|---------------------|--------------|------------|
| P2P | ✓ | - | - | ✓ | ✓ |
| Business | ✓ | ✓ | ✓ | ✓ | ✓ |
| Real Estate | ✓ | ✓ | ✓ | ✓ | ✓ |
| Trade | ✓ | ✓ | ✓ | ✓ | ✓ |
| Tax | ✓ | ✓ | - | ✓ | ✓ |
| STP | ✓ | ✓ | ✓ | ✓ | ✓ |
| COV | ✓ | ✓ | ✓ | ✓ | ✓ |
| Error | ✓ | ✓ | ✓ | ✓ | ✓ |

## Total Scenarios: 18

This minimal set provides comprehensive coverage of:
- All CBPR+ message types (MT103, MT103 STP, MT202 COV, MT199/299)
- Key features (purpose codes, LEI, structured remittance, transparency)
- Common use cases (P2P, business, real estate, trade, tax)
- Error conditions (validation failures, rejections, returns)
- Complex scenarios (multi-hop routing, compliance requirements)