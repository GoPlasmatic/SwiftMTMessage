# CBPR+ Implementation Roadmap

## Completed Scenarios (4)

1. **MT103 CBPR+ Business Payment** (`mt103/cbpr_business_payment.json`)
   - Full CBPR+ compliance with purpose codes, LEI, structured remittance
   - Extended transparency data in Field 77T

2. **MT103 CBPR+ Validation Failure** (`mt103/cbpr_validation_failure.json`)
   - Negative testing scenario for CBPR+ validation
   - Missing mandatory elements for compliance testing

3. **MT202 COV CBPR+ Standard** (`mt202/cbpr_cov_standard.json`)
   - Cover payment with CBPR+ data propagation
   - Demonstrates transparency maintenance through correspondent chain

4. **MT199 CBPR+ Inquiry** (`mt199/cbpr_inquiry.json`)
   - Payment status and transparency data request
   - Comprehensive CBPR+ information requirements

## Remaining Scenarios to Implement (14)

### MT103 CBPR+ Scenarios (4)
1. **mt103_cbpr_person_to_person.json**
   - Purpose: `/PURP/CASH`
   - No LEI required for individuals
   - Simple remittance format

2. **mt103_cbpr_real_estate.json**
   - Purpose: `/PURP/PHYS`
   - Property reference details
   - Escrow account handling

3. **mt103_cbpr_trade_finance.json**
   - Purpose: `/PURP/TRAD`
   - L/C references
   - Trade documentation in Field 77T

4. **mt103_cbpr_tax_payment.json**
   - Purpose: `/PURP/TAXS`
   - Government entity beneficiary
   - Tax reference numbers

### MT103 STP CBPR+ Scenarios (2)
5. **mt103_stp_cbpr_compliant.json**
   - Full STP compliance
   - ISO 11649 creditor reference
   - All fields in STP format

6. **mt103_stp_cbpr_enhanced.json**
   - STP with maximum CBPR+ data
   - Complex structured remittance
   - VAT/tax breakdowns

### MT202 COV CBPR+ Scenarios (2)
7. **mt202_cov_cbpr_complex_routing.json**
   - Multiple intermediaries
   - Currency conversion
   - Full transparency through chain

8. **mt202_cov_cbpr_compliance_enhanced.json**
   - High-risk corridor handling
   - Enhanced KYC/AML data
   - Additional regulatory codes

### MT299 CBPR+ Scenarios (1)
9. **mt299_cbpr_response.json**
   - Response to MT199 inquiry
   - Complete transparency trail
   - Processing timestamps

### Error Scenarios (3)
10. **mt103_cbpr_rejection.json**
    - CBPR+ specific rejection codes
    - Missing transparency data reasons

11. **mt103_cbpr_return.json**
    - CBPR+ return scenarios
    - Regulatory block reasons

12. **mt202_cov_cbpr_mismatch.json**
    - Data inconsistency between MT103 and cover
    - Reconciliation testing

### Additional Message Types (2)
13. **mt103_remit.json**
    - MT103 REMIT variant
    - Maximum remittance information
    - Multiple invoice handling

14. **mt205_cbpr.json**
    - MT205 with mandatory ordering institution
    - CBPR+ compliance for FI transfers

## Implementation Notes

### Purpose Codes to Support
- `CASH` - Cash Management
- `GDDS` - Purchase of Goods
- `PHYS` - Purchase of Physical Assets
- `TRAD` - Trade Finance
- `TAXS` - Tax Payment
- `SALA` - Salary Payment
- `SSBE` - Social Security Benefit
- `SUPP` - Supplier Payment

### LEI Format
- Always 20 alphanumeric characters
- Format in fields: `/LEI[20 characters]`
- Required for businesses, optional for individuals

### Transparency Requirements
1. **Fees**: All fees must be disclosed
2. **Exchange Rates**: Actual rates used
3. **Processing Time**: Estimated and actual
4. **Routing**: Complete payment path

### Validation Points
1. Purpose code presence and validity
2. LEI format when provided
3. Structured remittance format
4. Transparency data completeness
5. Cross-message consistency

## Testing Strategy

1. **Positive Testing**: Each scenario should generate valid, parseable messages
2. **Negative Testing**: Validation failure scenarios test error handling
3. **Round-trip Testing**: Generated messages must parse back correctly
4. **Cross-message Testing**: MT103 to MT202 COV data consistency
5. **Compliance Testing**: CBPR+ specific validation rules

## Next Steps

1. Implement remaining MT103 CBPR+ variants (4 scenarios)
2. Add MT103 STP CBPR+ scenarios (2 scenarios)
3. Complete MT202 COV scenarios (2 scenarios)
4. Implement MT299 response scenario
5. Add comprehensive error scenarios (3 scenarios)
6. Consider MT103 REMIT and MT205 variants
7. Create automated CBPR+ validation tests
8. Document CBPR+ compliance rules in code