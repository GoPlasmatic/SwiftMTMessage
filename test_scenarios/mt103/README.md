# MT103 Test Scenarios

This directory contains comprehensive test scenarios for MT103 (Single Customer Credit Transfer) messages, designed to cover various payment processing use cases in SWIFT CBPR+ compliance.

## Scenario Overview

### 1. **standard.json**
- **Purpose**: Basic customer credit transfer with minimal complexity
- **Use Case**: Standard retail or commercial payment
- **Key Features**: 
  - Standard processing (CRED)
  - SHA charges
  - Basic ordering/beneficiary information
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A

### 2. **stp.json**
- **Purpose**: Straight-Through Processing compliant payment
- **Use Case**: Automated processing without manual intervention
- **Key Features**:
  - STP bank operation code (SSTD)
  - All parties identified by BIC codes (50A, 59A)
  - Time indication (13C)
  - Instruction codes (23E: INTC)
- **Fields**: 20, 13C, 23B, 23E, 32A, 50A, 52A, 56A, 57A, 59A, 70, 71A

### 3. **high_value.json**
- **Purpose**: Large amount payment with enhanced compliance
- **Use Case**: Corporate acquisitions, major transactions
- **Key Features**:
  - Priority processing (SPRI)
  - Urgent priority (U)
  - Multiple instruction codes (URGP, INTC, REPA)
  - Regulatory reporting (77B)
  - Sender's charges (71F)
  - OUR charges
- **Fields**: 20, 13C, 23B, 23E, 26T, 32A, 50K, 52A, 57A, 59, 70, 71A, 71F, 72, 77B

### 4. **cover_payment.json**
- **Purpose**: Payment with separate cover (MT202COV)
- **Use Case**: Complex correspondent banking arrangements
- **Key Features**:
  - Full correspondent chain (53A, 54A)
  - Intermediary institution (56A)
  - Cover payment reference
- **Fields**: 20, 23B, 32A, 50K, 52A, 53A, 54A, 56A, 57A, 59, 70, 71A, 72

### 5. **fx_conversion.json**
- **Purpose**: Cross-currency payment with FX conversion
- **Use Case**: International trade with currency exchange
- **Key Features**:
  - Instructed amount (33B) different from settlement
  - Exchange rate (36)
  - Receiver's charges (71G)
  - Multi-currency handling
- **Fields**: 20, 23B, 32A, 33B, 36, 50K, 52A, 57A, 59, 70, 71A, 71G, 72

### 6. **remittance_enhanced.json**
- **Purpose**: Payment with detailed remittance information
- **Use Case**: Invoice payments, trade settlements
- **Key Features**:
  - Extended remittance info (70)
  - Structured remittance envelope (77T)
  - Invoice/PO references
  - Tax and discount information
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 77T

### 7. **treasury_payment.json**
- **Purpose**: Financial institution treasury operations
- **Use Case**: Liquidity management, cash concentration
- **Key Features**:
  - Treasury operation code (SPAY)
  - Party identifier format (50F, 59F)
  - Transaction type (26T: K03)
  - BEN charges
- **Fields**: 20, 23B, 23E, 26T, 32A, 50F, 52A, 57A, 59F, 70, 71A, 72

### 8. **correspondent_banking.json**
- **Purpose**: Full correspondent banking chain
- **Use Case**: Multi-bank international routing
- **Key Features**:
  - Complete correspondent chain
  - All routing institutions specified
  - Location-based routing (53B)
- **Fields**: 20, 23B, 32A, 50K, 51A, 52A, 53B, 54A, 55A, 56A, 57A, 59, 70, 71A, 72

### 9. **regulatory_compliant.json**
- **Purpose**: Enhanced regulatory compliance payment
- **Use Case**: High-risk jurisdictions, enhanced due diligence
- **Key Features**:
  - Multiple compliance references
  - KYC/Screening references
  - Comprehensive regulatory reporting (77B)
  - Multiple instruction codes
- **Fields**: 20, 13C, 23B, 23E, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77B

### 10. **minimal.json**
- **Purpose**: Minimum required fields only
- **Use Case**: Testing edge cases, basic validation
- **Key Features**:
  - Only mandatory fields
  - Simplified addresses
  - No optional fields
- **Fields**: 20, 23B, 32A, 50K, 59, 71A

### 11. **rejection.json**
- **Purpose**: Payment rejection scenario
- **Use Case**: Failed payment processing
- **Key Features**:
  - Return instruction (23E: RETN)
  - Rejection codes and reasons
  - Original reference tracking
  - Reversed routing
- **Fields**: 20, 23B, 23E, 32A, 50K, 52A, 57A, 59, 70, 71A, 72

### 12. **return.json**
- **Purpose**: Payment return/reversal
- **Use Case**: Mandate issues, compliance returns
- **Key Features**:
  - Return instruction codes
  - Return reason codes
  - Transaction type (26T: K02)
  - Regulatory reporting for returns
- **Fields**: 20, 23B, 23E, 26T, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77B

### 13. **cbpr_business_payment.json**
- **Purpose**: CBPR+ compliant business-to-business payment
- **Use Case**: Cross-border B2B payments with enhanced transparency
- **Key Features**:
  - Purpose code (70: /PURP/GDDS)
  - LEI identifiers in parties (50K, 59)
  - Structured remittance (70: /INV/, /PO/)
  - Extended remittance envelope (77T)
  - CBPR+ compliance markers (72)
  - Full transparency data
- **Fields**: 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77T

### 14. **cbpr_validation_failure.json**
- **Purpose**: CBPR+ validation failure scenario
- **Use Case**: Testing CBPR+ compliance validation
- **Key Features**:
  - Missing purpose code
  - Missing LEI information
  - Non-compliant remittance format
  - Absent transparency data
  - Validation failure markers
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72

### 15. **cbpr_person_to_person.json**
- **Purpose**: CBPR+ compliant person-to-person payment
- **Use Case**: Individual cross-border transfers (remittances, family support)
- **Key Features**:
  - Purpose code (70: /PURP/CASH)
  - Personal accounts for individuals
  - No LEI required (individuals exempt)
  - Simple unstructured remittance
  - Personal transfer narrative
  - Lower amounts (100-5,000)
- **Fields**: 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72

### 16. **cbpr_real_estate.json**
- **Purpose**: CBPR+ compliant real estate transaction payment
- **Use Case**: Property purchases, real estate investments, escrow payments
- **Key Features**:
  - Purpose code (70: /PURP/PHYS)
  - LEI identifiers for corporate entities
  - Property reference (/PROP/)
  - Escrow reference (/ESCROW/)
  - High value amounts (100,000-2,000,000)
  - OUR charges (buyer pays all fees)
  - Title company verification
  - Escrow account for beneficiary
- **Fields**: 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72

### 17. **cbpr_trade_finance.json**
- **Purpose**: CBPR+ compliant trade finance payment
- **Use Case**: Letter of Credit settlements, export/import payments, trade documentation
- **Key Features**:
  - Purpose code (70: /PURP/TRAD)
  - LEI identifiers for trading parties
  - Letter of Credit reference (/LC/)
  - Invoice reference (/INV/)
  - Bill of Lading reference (/BL/)
  - Instruction codes (23E: INTC, PHOB)
  - Trade amounts (50,000-500,000)
  - SHA charges (shared)
  - Regulatory reporting (77B)
  - Trade documentation verification
- **Fields**: 20, 23B, 23E, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77B

### 18. **cbpr_tax_payment.json**
- **Purpose**: CBPR+ compliant tax payment to government authorities
- **Use Case**: Corporate tax payments, VAT remittances, payroll taxes, property taxes
- **Key Features**:
  - Purpose code (70: /PURP/TAXS)
  - LEI identifier for corporate taxpayer
  - Tax ID reference (/TAXID/)
  - Tax period reference (/PERIOD/)
  - Tax type specification (/TYPE/)
  - Government tax authority as beneficiary
  - Tax amounts (1,000-100,000)
  - OUR charges (taxpayer pays all fees)
  - Regulatory reporting (77B)
  - Direct routing to tax authority bank
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77B

### 19. **cbpr_salary_payment.json**
- **Purpose**: CBPR+ compliant employee salary payment
- **Use Case**: Monthly salary transfers, payroll processing, employee compensation
- **Key Features**:
  - Purpose code (70: /PURP/SALA)
  - LEI identifier for employer
  - Employee ID reference (/EMPID/)
  - Pay period reference (/PERIOD/)
  - Instruction code (23E: INTC)
  - Transaction type (26T: SAL)
  - Salary amounts (2,000-15,000)
  - SHA charges (shared)
  - Recurring monthly transfer
  - Individual employee accounts
- **Fields**: 20, 23B, 23E, 26T, 32A, 50K, 52A, 57A, 59, 70, 71A, 72

### 20. **cbpr_supplier_payment.json**
- **Purpose**: CBPR+ compliant business-to-business supplier payment
- **Use Case**: Invoice settlements, purchase order payments, B2B trade settlements
- **Key Features**:
  - Purpose code (70: /PURP/SUPP)
  - LEI identifiers for both buyer and supplier
  - Invoice reference (/INV/)
  - Purchase order reference (/PO/)
  - Extended remittance envelope (77T)
  - Payment terms and discount information
  - B2B amounts (10,000-250,000)
  - SHA charges (shared)
  - Accounts payable to accounts receivable
  - Intermediary bank routing (56A)
- **Fields**: 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77T

### 21. **cbpr_social_security.json**
- **Purpose**: CBPR+ compliant government social security benefit payment
- **Use Case**: Monthly social security payments, disability benefits, retirement payments
- **Key Features**:
  - Purpose code (70: /PURP/SSBE)
  - Government agency with LEI identifier
  - Beneficiary reference (/BENREF/)
  - Benefit type (/TYPE/)
  - Payment period (/PERIOD/)
  - Individual beneficiary accounts
  - Benefit amounts (500-3,000)
  - SHA charges (shared)
  - Government disbursement account
  - Monthly recurring transfer
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72

### 22. **cbpr_charity_donation.json**
- **Purpose**: CBPR+ compliant charitable donation payment
- **Use Case**: Individual or corporate donations to registered charities, non-profit contributions, humanitarian aid
- **Key Features**:
  - Purpose code (70: /PURP/CHAR)
  - LEI identifier for corporate donor
  - Charity registration number (/REG/)
  - Donation campaign reference (/DONREF/)
  - Tax receipt number (/TAXREC/)
  - Extended remittance envelope (77T)
  - Non-profit beneficiary accounts
  - Donation amounts (100-10,000)
  - SHA charges (shared)
  - Tax deductible indication
  - Humanitarian aid purpose
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T

### 23. **cbpr_pension_payment.json**
- **Purpose**: CBPR+ compliant pension fund disbursement
- **Use Case**: Monthly pension payments to retirees, disability pensions, survivor benefits
- **Key Features**:
  - Purpose code (70: /PURP/PENS)
  - Pension fund with LEI identifier
  - Pension reference (/PENREF/)
  - Pension type (/TYPE/)
  - Payment period (/PERIOD/)
  - Instruction code (23E: INTC)
  - Individual pensioner accounts
  - Pension amounts (1,000-5,000)
  - SHA charges (shared)
  - Recurring monthly transfer
  - Pension administration account
- **Fields**: 20, 23B, 23E, 32A, 50K, 52A, 57A, 59, 70, 71A, 72

### 24. **cbpr_interest_payment.json**
- **Purpose**: CBPR+ compliant bank interest payment to customer
- **Use Case**: Quarterly or annual interest credits, term deposit interest, savings account interest
- **Key Features**:
  - Purpose code (70: /PURP/INTE)
  - Bank with LEI identifier
  - Account number reference (/ACCNT/)
  - Interest period (/PERIOD/)
  - Interest rate (/RATE/)
  - Individual customer accounts
  - Interest amounts (50-5,000)
  - OUR charges (bank pays all fees)
  - Interest payable account
  - Treasury department originator
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72

### 25. **cbpr_healthcare_payment.json**
- **Purpose**: CBPR+ compliant healthcare insurance payment to medical provider
- **Use Case**: Insurance claim settlements, hospital payments, medical service reimbursements
- **Key Features**:
  - Purpose code (70: /PURP/HLTC)
  - Insurance company with LEI identifier
  - Hospital/clinic with LEI identifier
  - Claim number reference (/CLAIM/)
  - Patient reference (/PATIENT/)
  - Treatment type (/TYPE/)
  - Instruction code (23E: INTC)
  - Extended remittance envelope (77T)
  - Healthcare amounts (5,000-100,000)
  - SHA charges (shared)
  - Intermediary bank routing (56A)
  - Claims payable to accounts receivable
- **Fields**: 20, 23B, 23E, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77T

### 26. **cbpr_government_disbursement.json**
- **Purpose**: CBPR+ compliant government assistance payment to individuals
- **Use Case**: Disaster relief payments, emergency assistance, government grants, stimulus payments
- **Key Features**:
  - Purpose code (70: /PURP/GOVT)
  - Government agency with LEI identifier
  - Program reference (/PROG/)
  - Disbursement type (/TYPE/)
  - Beneficiary ID (/BENID/)
  - Regulatory reporting (77B)
  - Individual beneficiary accounts
  - Disbursement amounts (1,000-50,000)
  - SHA charges (shared)
  - Government disbursement account
  - Emergency assistance verification
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77B

### 27. **cbpr_stp_compliant.json**
- **Purpose**: CBPR+ compliant payment with STP optimization using ISO 11649 creditor reference
- **Use Case**: Automated straight-through processing, high-volume B2B payments, structured reconciliation
- **Key Features**:
  - ISO 11649 creditor reference (RF format) on first line of field 70
  - Purpose code (/PURP/GDDS)
  - Ultimate debtor LEI (/ULTD/)
  - Ultimate creditor LEI (/ULTB/)
  - Time indication (13C)
  - Full party identification with LEIs
  - STP amounts (10,000-500,000)
  - SHA charges (shared)
  - No manual intervention fields
  - Structured account formats
- **Fields**: 20, 13C, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72

### 28. **cbpr_stp_enhanced.json**
- **Purpose**: CBPR+ compliant STP payment with enhanced features and remittance
- **Use Case**: Complex trade finance STP, automated L/C settlements, multi-invoice reconciliation
- **Key Features**:
  - ISO 11649 creditor reference
  - Purpose code (/PURP/TRAD)
  - Instructed amount (33B) and fees
  - LEI identifiers for all parties
  - Extended remittance envelope (77T)
  - Trade documentation references
  - Letter of Credit details
  - STP amounts (10,000-500,000)
  - SHA charges with sender's fees (71F)
  - Full correspondent chain
  - Invoice and goods information
- **Fields**: 20, 13C, 23B, 32A, 33B, 50K, 52A, 56A, 57A, 59, 70, 71A, 71F, 72, 77T

### 29. **remit_basic.json**
- **Purpose**: Extended unstructured remittance information payment
- **Use Case**: Complex payment narratives, project-based payments, detailed payment explanations
- **Key Features**:
  - Extended unstructured remittance in field 70
  - Detailed narrative in field 77T envelope
  - Project and contract references
  - Payment milestone descriptions
  - LEI identifiers for corporate parties
  - Basic amounts (1,000-50,000)
  - SHA charges (shared)
  - Full narrative documentation
  - Email contact information
- **Fields**: 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T

### 30. **remit_structured.json**
- **Purpose**: Structured remittance information for multiple invoice settlements
- **Use Case**: Batch invoice payments, accounts payable processing, multi-invoice reconciliation
- **Key Features**:
  - Multiple invoice references in field 70 (/INV/)
  - Purchase order reference (/PO/)
  - Structured remittance envelope (77T)
  - Individual invoice details with dates and amounts
  - Payment summary with discounts
  - Instruction code (23E: INTC)
  - Settlement amounts (25,000-150,000)
  - SHA charges (shared)
  - Early payment discount tracking
  - Payment terms documentation
- **Fields**: 20, 23B, 23E, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T

## Usage

These scenarios are designed to work with the SwiftMTMessage library's `generate_sample` function:

```rust
use swift_mt_message::{generate_sample, messages::mt103::MT103};

// Generate a specific scenario
let stp_payment = generate_sample::<MT103>("MT103", Some("stp"))?;

// Generate default (standard) scenario
let standard_payment = generate_sample::<MT103>("MT103", None)?;
```

## Validation

All scenarios have been validated for:
- ✅ Valid JSON syntax
- ✅ Consistent structure following the standard.json template
- ✅ Appropriate field usage for each use case
- ✅ SWIFT MT103 compliance

## Field Structure

All scenarios use the same consistent structure:
- `variables`: Dynamic values using datafake generation
- `schema.basic_header`: SWIFT basic header (Block 1)
- `schema.application_header`: SWIFT application header (Block 2)
- `schema.fields`: MT103 specific fields (Block 4)

Field references use `{"var": "variable_name"}` for variables and `{"fake": [...]}` for datafake generation.