# SWIFT MT Message Test Scenarios

This directory contains comprehensive test scenarios for SWIFT MT messages, designed to validate the SwiftMTMessage library's ability to generate, parse, and round-trip various real-world financial messaging use cases.

## Purpose

The test scenarios serve multiple critical functions:
- **Validation**: Ensure accurate parsing and generation of SWIFT MT messages
- **Compliance**: Verify adherence to SWIFT standards and CBPR+ requirements
- **Coverage**: Test edge cases, minimal configurations, and complex multi-party scenarios
- **Real-world Testing**: Simulate actual business use cases from retail payments to complex correspondent banking

## Organization

Test scenarios are organized by message type, with each directory containing:
```
test_scenarios/
├── mt101/              # Request for Transfer
│   ├── index.json      # List of available scenarios
│   ├── standard.json   # Basic scenario
│   ├── bulk_payment.json
│   └── ...
├── mt103/              # Single Customer Credit Transfer
│   ├── index.json
│   ├── standard.json
│   ├── cbpr_*.json     # CBPR+ compliant scenarios
│   └── ...
└── ...
```

Each scenario file uses a consistent JSON structure with:
- `variables`: Dynamic values using datafake generation
- `schema`: Message structure including headers and fields

## Test Scenarios Overview

| Message Type | Scenario Name | Purpose | Use Case | Key Features | Fields Used | Sample Generator | Parsing | Validation | Round Trip Test |
|--------------|---------------|---------|----------|--------------|-------------|------------------|---------|------------|-----------------|
| **MT101** | standard | Basic corporate payment request | Standard accounts payable | 2 transactions, single currency | 20, 21, 28D, 30, 32B, 50#2F, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | bulk_payment | Large batch processing | Corporate AP batch runs | 5 transactions, batch tracking | 20, 21, 23E (INTC), 25, 28D, 30, 32B, 50#1K, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | multi_currency | FX payment processing | International treasury ops | Multiple currencies, FX rates | 20, 21, 21F, 23E, 28D, 30, 32B, 33B, 36, 50#1K, 52A, 56A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | scheduled_payment | Future-dated payments | Recurring obligations | Future execution dates | 20, 21, 21R, 23E (PHON), 25, 28D, 30, 32B, 50#1K, 51A, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | salary_payment | Payroll processing | Monthly salary runs | Employee references, OUR charges | 20, 21, 21R, 25, 25A, 28D, 30, 32B, 50#1K, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | vendor_payment | Supplier settlements | Accounts payable | Invoice refs, regulatory info | 20, 21, 23E (TELE), 28D, 30, 32B, 50#1K, 52A, 56A, 57A, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT101** | urgent_payment | Time-critical transfers | Emergency payments | Urgent priority, notifications | 20, 21, 21R, 23E (URGP), 28D, 30, 32B, 50#1K, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | direct_debit | Direct debit collection | Subscriptions, utilities | Mandate references, HOLD | 20, 21, 21R, 23E (HOLD), 28D, 30, 32B, 50#1K, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | minimal | Minimum required fields | Testing, validation | Single transaction only | 20, 21, 28D, 30, 32B, 50#1K, 59, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | cbpr_corporate_bulk_payment | CBPR+ corporate bulk | Cross-border corporate batch | CBPR+ compliant batch payment | 20, 21, 23E, 28D, 30, 32B, 50#1K, 52A, 57A, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT101** | cbpr_payroll_batch | CBPR+ payroll batch | Cross-border payroll | CBPR+ compliant salary batch | 20, 21, 23E, 28D, 30, 32B, 50#1K, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT101** | cbpr_supplier_batch | CBPR+ supplier batch | Cross-border supplier payments | CBPR+ compliant vendor batch | 20, 21, 23E, 28D, 30, 32B, 50#1K, 52A, 57A, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT103** | standard | Basic credit transfer | Retail/commercial payment | Standard processing (CRED) | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT103** | stp | Straight-through processing | Automated processing | All BIC codes (50A, 59A) | 20, 13C, 23B (SSTD), 32A, 50A, 52A, 57A, 59A, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT103** | high_value | Large amount payment | Corporate acquisitions | Priority (SPRI), regulatory | 20, 23B (SPRI), 32A, 50K, 52A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cover_payment | Separate cover payment | Correspondent banking | Full correspondent chain | 20, 23B, 32A, 50K, 52A, 53A, 54A, 56A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | fx_conversion | Cross-currency payment | International trade | Exchange rate, receiver charges | 20, 23B, 32A, 33B, 36, 50K, 52A, 57A, 59, 70, 71A, 71G, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_business_payment | CBPR+ B2B payment | Cross-border B2B | Purpose code, LEI, structured remit | 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_person_to_person | CBPR+ P2P payment | Individual remittances | Purpose code (/PURP/CASH) | 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_real_estate | CBPR+ property payment | Property purchases | Purpose (/PURP/PHYS), escrow refs | 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_trade_finance | CBPR+ trade payment | L/C settlements | Purpose (/PURP/TRAD), trade docs | 20, 23B, 23E, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77B, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_charity_donation | CBPR+ charity payment | Charitable donations | Purpose (/PURP/CHAR), charity details | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_commission_payment | CBPR+ commission payment | Sales commissions | Purpose (/PURP/COMC), commission details | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_crypto_settlement | CBPR+ crypto settlement | Cryptocurrency trading | Purpose (/PURP/TRFD), crypto refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_dividend_distribution | CBPR+ dividend distribution | Shareholder dividends | Purpose (/PURP/DIVI), dividend details | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_dividend_payment | CBPR+ dividend payment | Investment dividends | Purpose (/PURP/DIVI), investment refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_ecommerce_b2c | CBPR+ e-commerce B2C | Online retail | Purpose (/PURP/GDDS), order refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_education_international | CBPR+ intl education | International tuition | Purpose (/PURP/EDUC), student ID | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_education_payment | CBPR+ education payment | Domestic tuition | Purpose (/PURP/EDUC), enrollment refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_fees_payment | CBPR+ fees payment | Professional fees | Purpose (/PURP/FEES), service details | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_gig_economy | CBPR+ gig economy | Freelance payments | Purpose (/PURP/SALA), contractor ID | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_government_disbursement | CBPR+ govt disbursement | Government benefits | Purpose (/PURP/GOVT), benefit type | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77B, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_healthcare_payment | CBPR+ healthcare payment | Medical services | Purpose (/PURP/HLTH), claim refs | 20, 23B, 23E, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_insurance_cross_border | CBPR+ intl insurance | Cross-border insurance | Purpose (/PURP/INSU), policy number | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_insurance_payment | CBPR+ insurance payment | Insurance premiums | Purpose (/PURP/INSU), policy details | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_interest_payment | CBPR+ interest payment | Loan interest | Purpose (/PURP/INTE), loan refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_investment_payment | CBPR+ investment payment | Securities investment | Purpose (/PURP/SECU), investment ID | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_loan_disbursement | CBPR+ loan disbursement | Loan proceeds | Purpose (/PURP/LOAN), loan agreement | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_pension_payment | CBPR+ pension payment | Retirement benefits | Purpose (/PURP/PENS), beneficiary ID | 20, 23B, 23E, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_remittance_corridor | CBPR+ remittance corridor | Worker remittances | Purpose (/PURP/CASH), corridor details | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_rent_payment | CBPR+ rent payment | Property rent | Purpose (/PURP/RENT), lease refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_royalty_payment | CBPR+ royalty payment | IP royalties | Purpose (/PURP/INTC), IP details | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_salary_payment | CBPR+ salary payment | Employee wages | Purpose (/PURP/SALA), employee ID | 20, 23B, 23E, 26T, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_sanctions_failure | CBPR+ sanctions failure | Sanctions screening test | Sanctioned entity refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_social_security | CBPR+ social security | Social benefits | Purpose (/PURP/SSBE), benefit type | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_stp_compliant | CBPR+ STP compliant | Full automation | All structured fields | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_stp_enhanced | CBPR+ STP enhanced | Enhanced automation | Extended structured data | 20, 23B, 32A, 33B, 50K, 52A, 56A, 57A, 59, 70, 71A, 71F, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_subscription_saas | CBPR+ SaaS subscription | Software subscriptions | Purpose (/PURP/SUBS), subscription ID | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_supplier_payment | CBPR+ supplier payment | B2B settlements | Purpose (/PURP/SUPP), invoice refs | 20, 23B, 32A, 50K, 52A, 56A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_tax_payment | CBPR+ tax payment | Tax obligations | Purpose (/PURP/TAXS), tax ID | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77B, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_treasury_intercompany | CBPR+ intercompany | Corporate treasury | Purpose (/PURP/INTC), company refs | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_utility_cross_border | CBPR+ intl utility | Cross-border utilities | Purpose (/PURP/UBIL), account number | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_utility_payment | CBPR+ utility payment | Domestic utilities | Purpose (/PURP/UBIL), meter number | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | cbpr_validation_failure | CBPR+ validation failure | Validation testing | Invalid purpose codes | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | correspondent_banking | Correspondent banking | FI relationships | Nostro/vostro accounts | 20, 23B, 32A, 50K, 51A, 52A, 53A, 54A, 55A, 56A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | duplicate_uetr | Duplicate UETR test | UETR tracking | Duplicate detection | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 121 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | invalid_purpose_code | Invalid purpose code | Validation testing | Error handling | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | minimal | Minimal fields | Testing baseline | Required fields only | 20, 23B, 32A, 50K, 59, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT103** | missing_lei_entity | Missing LEI test | Compliance testing | LEI validation | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | regulatory_compliant | Regulatory compliance | Compliance testing | Full regulatory data | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | rejection | Payment rejection | Rejection handling | Rejection codes | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | remit_basic | Basic remittance | Simple remittance | Basic remit info | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | remit_structured | Structured remittance | Complex remittance | Structured remit data | 20, 23B, 23E, 32A, 50K, 52A, 57A, 59, 70, 71A, 72, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | remittance_enhanced | Enhanced remittance | Detailed remittance | Extended remit info | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 77T | ✅ | ✅ | ✅ | ✅ |
| **MT103** | return | Payment return | Return processing | Return reason codes | 20, 23B, 32A, 50K, 52A, 57A, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | treasury_payment | Treasury payment | Corporate treasury | Treasury operations | 20, 23B, 32A, 50, 52, 57, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT103** | unresolved_intermediary | Unresolved intermediary | Routing testing | Missing intermediary | 20, 23B, 32A, 50, 52, 56, 57, 59, 70, 71A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT104** | fi_direct_debit_basic | Basic direct debit | Standard collections | CORT instruction, mandate ref | 20, 21, 23E, 28D, 30, 32B, 50#2K, 52A, 57A, 59, 70, 71A | ✅ | ✅ | ✅ | ✅ |
| **MT104** | fi_direct_debit_cbpr | CBPR+ direct debit | Cross-border collections | LEI, purpose codes, transparency | 20, 21, 23E, 28D, 30, 32B, 50#2K, 52A, 57A, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT104** | fi_direct_debit_multiple | Batch collections | Multiple debtor processing | 3 transactions, batch refs | 19, 20, 21, 23E, 28D, 30, 32B (multiple), 50#2K, 52A, 57A, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT104** | fi_direct_debit_recurring | Recurring collections | Subscriptions, utilities | Standing order references | 20, 21, 23E, 28D, 30, 32B, 50#2K, 52A, 57A, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT104** | fi_direct_debit_return | Collection returns | Failed collections | Return reason codes | 20, 21, 21C, 23E, 28D, 30, 32B, 50#2K, 52A, 57A, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT104** | cbpr_utility_collection | CBPR+ utility collection | Cross-border utility payments | CBPR+ compliant utility batch | 20, 21R, 23E, 28D, 30, 32B, 50#2, 52, 57, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT104** | cbpr_subscription_collection | CBPR+ subscription collection | Cross-border subscriptions | CBPR+ compliant subscription batch | 20, 21R, 23E, 28D, 30, 32B, 50#2, 52, 57, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT104** | cbpr_insurance_collection | CBPR+ insurance collection | Cross-border insurance premiums | CBPR+ compliant insurance batch | 20, 21R, 23E, 28D, 30, 32B, 50#2, 52, 57, 59, 70, 71A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT107** | authorized_bulk_collection | Authorized collections | Pre-approved bulk debits | Authorization refs, batch IDs | 20, 21E, 23E, 26T, 30, 32B, 50K, 52A, 59, 71F | ✅ | ✅ | ✅ | ✅ |
| **MT107** | general_direct_debit_basic | Standard direct debit | Basic collection request | Simple authorization | 20, 21E, 30, 32B, 50K, 52A, 59 | ✅ | ✅ | ✅ | ✅ |
| **MT107** | return_processing | Direct debit returns | Failed/disputed collections | Return codes, original refs | 20, 21E, 23E, 30, 32B, 50K, 52A, 59, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT107** | unauthorized_debit_processing | Unauthorized debits | Dispute handling | Dispute refs, investigation | 20, 21E, 26T, 30, 32B, 50#2K, 52A, 59, 70, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT110** | cheque_collection_advice | Multiple cheque collection | Batch cheque processing | 3 cheques, collection advice | 20, 21, 25, 28C, 30, 32B, 50, 52A, 53A, 54A, 59, 72, # | ✅ | ✅ | ✅ | ✅ |
| **MT110** | single_cheque_advice | Single cheque advice | Express cheque clearance | Single large value cheque | 20, 53A, 54A, 72, 21, 30, 32B, 50F, 52A, 59 | ✅ | ✅ | ✅ | ✅ |
| **MT110** | foreign_cheque_collection | Foreign cheque collection | International cheque clearing | 2 foreign cheques, FX conversion | 20, 53D, 54A, 72, 21, 30, 32B, 50K, 52D, 59 (multiple) | ✅ | ✅ | ✅ | ✅ |
| **MT110** | returned_cheque_advice | Returned cheque notice | Cheque return/bounce | Insufficient funds, return fees | 20, 53A, 54A, 72, 21, 30, 32B, 50K, 52A, 59 | ✅ | ✅ | ✅ | ✅ |
| **MT111** | lost_cheque_stop | Lost cheque stop | Lost cheque handling | Stop payment request | 20, 21, 23, 25, 30, 52A, 59, 70, 75 | ✅ | ✅ | ✅ | ✅ |
| **MT111** | fraud_prevention_stop | Fraud prevention | Fraud detection | Urgent stop payment | 20, 21, 23, 25, 30, 52A, 59, 70, 75 | ✅ | ✅ | ✅ | ✅ |
| **MT111** | duplicate_cheque_stop | Duplicate cheque | Duplicate prevention | Stop duplicate processing | 20, 21, 23, 25, 30, 52A, 59, 70, 75 | ✅ | ✅ | ✅ | ✅ |
| **MT112** | stop_payment_accepted | Stop accepted | Successful stop | Confirmation message | 20, 21, 52A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT112** | stop_payment_pending | Stop pending | Processing stop | Pending status | 20, 21, 52A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT112** | stop_payment_rejected | Stop rejected | Failed stop | Rejection reason | 20, 21, 52A, 77B | ✅ | ✅ | ✅ | ✅ |
| **MT192** | request_cancellation | Basic cancellation request | Payment cancellation | Customer requested cancel | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT192** | urgent_cancellation_mt202 | Urgent MT202 cancellation | FI transfer cancellation | Duplicate payment, same day value | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT192** | fraud_prevention_cancellation | Fraud alert cancellation | Security incident response | Suspected fraud, account takeover | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT192** | regulatory_compliance_cancellation | Compliance cancellation | Sanctions screening hit | AML/sanctions alert, false positive check | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT192** | cbpr_cancellation_request | CBPR+ cancellation request | Cross-border payment cancel | UETR tracking, reason codes | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT196** | answer_cancellation | Cancellation accepted | Successful cancel response | Cancel confirmed, funds returned | 11S, 20, 21, 76, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT196** | answer_rejection | Cancellation rejected | Failed cancel response | Already executed, cannot reverse | 11S, 20, 21, 76, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT196** | answer_pending_investigation | Investigation pending | Fraud investigation response | Under review, payment frozen | 11S, 20, 21, 76, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT196** | answer_inquiry_response | Inquiry response | Payment status response | Payment found and completed | 11S, 20, 21, 76, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT196** | cbpr_cancellation_response | CBPR+ cancellation response | Cross-border cancel response | UETR tracking, status codes | 11S, 20, 21, 76, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT199** | cbpr_cancellation | CBPR+ cancellation | Cross-border cancel | CBPR+ compliant cancel | 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT199** | cbpr_inquiry | CBPR+ inquiry | Payment inquiry | CBPR+ compliant inquiry | 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | cbpr_cov_standard | CBPR+ cover standard | Standard cover payment | Basic CBPR+ cover | 20, 21, 32A, 52A, 53A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | cbpr_cov_complex_routing | CBPR+ complex routing | Complex correspondent chain | Multiple intermediaries | 20, 21, 32A, 52A, 53A, 54A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | cbpr_cov_compliance_enhanced | CBPR+ enhanced compliance | High compliance cover | Enhanced regulatory data | 20, 21, 32A, 52A, 53A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | cov_mismatch | Cover mismatch | Mismatch testing | Amount/reference mismatch | 20, 21, 32A, 52A, 53A, 56A, 57A, 58A | ✅ | ✅ | ✅ | ✅ |
| **MT202** | fi_to_fi_transparency | FI transparency | Transparent routing | Full transparency data | 20, 21, 32A, 52A, 53A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | return | Payment return | FI-to-FI return | Return processing | 20, 21, 32A, 52A, 53A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | return_simple | Simple return | Basic return | Minimal return fields | 20, 21, 32A, 52A, 57A, 58A | ✅ | ✅ | ✅ | ✅ |
| **MT202** | minimal_return | Minimal return | Minimal fields return | Absolute minimum return | 20, 21, 32A, 52A, 57A, 58A | ✅ | ✅ | ✅ | ✅ |
| **MT202** | cbpr_cov_return | CBPR+ cover return | Cross-border return | CBPR+ compliant return | 20, 21, 32A, 52A, 53A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | cbpr_cov_rejection | CBPR+ cover rejection | Cross-border rejection | CBPR+ compliant rejection | 20, 21, 32A, 52A, 53A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT202** | cbpr_serial_payment | CBPR+ serial payment | Serial payment chain | Multi-hop CBPR+ payment | 20, 21, 32A, 52A, 53A, 54A, 56A, 57A, 58A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT205** | bank_transfer_non_cover | Non-cover transfer | Direct FI transfer | No cover required | 13C, 20, 21, 32A, 52A, 53A, 57A, 59, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT205** | bank_transfer_cover | Cover transfer | FI cover payment | Cover payment for MT103 | 13C, 20, 21, 32A, 52A, 53A, 56A, 57A, 59, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT205** | return_payment | Payment return | FI return payment | Return of funds | 20, 21, 32A, 52A, 53A, 57A, 59, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT205** | rejection_payment | Payment rejection | FI rejection | Rejected payment | 20, 21, 32A, 52A, 53A, 57A, 59, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT205** | urgent_liquidity_transfer | Urgent liquidity | Emergency funding | Urgent liquidity transfer | 13C, 20, 21, 32A, 52A, 53A, 57A, 59, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT205** | regulatory_reporting | Regulatory reporting | Compliance reporting | Regulatory submission | 20, 21, 32A, 52A, 53A, 57A, 59, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT210** | expected_incoming_funds | Multiple payments notice | Batch incoming funds | 3 expected payments, different sources | 20, 25, 30, 21, 32B, 50, 52A, 56A/D (multiple) | ✅ | ✅ | ✅ | ✅ |
| **MT210** | single_payment_notice | Single payment notice | Wire transfer notice | Large value incoming wire | 20, 25, 30, 21, 32B, 50K, 52A | ✅ | ✅ | ✅ | ✅ |
| **MT210** | fx_settlement_notice | FX settlement notice | Currency trade settlement | Two-way FX settlement | 20, 25, 30, 21, 32B, 50F, 52A (multiple) | ✅ | ✅ | ✅ | ✅ |
| **MT210** | securities_settlement_notice | Securities settlement | DVP settlement notice | Bond settlement + coupon | 20, 25, 30, 21, 32B, 50K/F, 52A/D, 56A (multiple) | ✅ | ✅ | ✅ | ✅ |
| **MT292** | fi_cancellation_request | Basic FI cancellation | Bank-to-bank cancel | Agent error, routing issue | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT292** | compliance_hold_cancellation | Compliance hold request | Regulatory freeze | Enhanced due diligence required | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT292** | system_error_cancellation | System error cancel | Technical failure | Batch processing error | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT292** | wrong_beneficiary_cancellation | Wrong beneficiary | Incorrect account | Account closed/does not exist | 11S, 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT296** | cancellation_accepted | Cancel accepted | Successful cancel | Cancellation confirmed | 20, 21, 11S, 76 | ✅ | ✅ | ✅ | ✅ |
| **MT296** | cancellation_rejected | Cancel rejected | Failed cancel | Cancellation denied | 20, 21, 11S, 76 | ✅ | ✅ | ✅ | ✅ |
| **MT296** | inquiry_response | Inquiry response | Query answer | Information response | 20, 21, 76 | ✅ | ✅ | ✅ | ✅ |
| **MT296** | no_payment_found | No payment found | Not found response | Payment not located | 20, 21, 76 | ✅ | ✅ | ✅ | ✅ |
| **MT296** | partial_cancellation | Partial cancel | Partial amount cancel | Partial cancellation | 20, 21, 11S, 76 | ✅ | ✅ | ✅ | ✅ |
| **MT299** | cbpr_payment_response | CBPR+ payment status | Transparency response | Full payment path tracking | 20, 21, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT299** | regulatory_notification | Regulatory notice | Compliance update | New reporting requirements | 20, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT299** | settlement_instructions_update | SSI update notice | Nostro account change | USD settlement account change | 20, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT299** | system_maintenance_notice | Maintenance notification | Scheduled downtime | 8-hour maintenance window | 20, 79 | ✅ | ✅ | ✅ | ✅ |
| **MT900** | basic_debit_confirmation | Basic debit confirm | Standard debit notice | Account debited | 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT900** | direct_debit_confirmation | Direct debit confirm | DD collection notice | DD executed | 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT900** | fee_debit_confirmation | Fee debit confirm | Fee charge notice | Fees debited | 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT900** | fx_transaction_debit | FX debit confirm | FX trade debit | FX settlement | 13D, 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT900** | standing_order_debit | Standing order debit | Recurring debit | SO executed | 13D, 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT900** | cbpr_debit_confirmation | CBPR+ debit confirm | Cross-border debit notice | CBPR+ compliant debit | 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT910** | basic_credit_confirmation | Basic credit confirm | Standard credit notice | Account credited | 20, 21, 25, 32A, 52A | ✅ | ✅ | ✅ | ✅ |
| **MT910** | dividend_payment | Dividend credit | Dividend receipt | Dividend credited | 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT910** | incoming_wire_transfer | Wire transfer credit | Incoming wire | Wire received | 20, 21, 25, 32A, 50, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT910** | interest_credit | Interest credit | Interest payment | Interest credited | 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT910** | refund_credit | Refund credit | Refund receipt | Refund credited | 20, 21, 25, 32A, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT910** | cbpr_credit_confirmation | CBPR+ credit confirm | Cross-border credit notice | CBPR+ compliant credit | 20, 21, 25, 32A, 50, 52A, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT920** | interim_report_request | Interim report request | Intraday statement | Request interim report | 12, 20, 25A, 34F | ✅ | ✅ | ✅ | ✅ |
| **MT920** | multi_account_request | Multi-account request | Multiple accounts | Request multiple accounts | 12, 20, 25A (multiple), 34F | ✅ | ✅ | ✅ | ✅ |
| **MT920** | statement_request_basic | Basic statement request | Account statement | Request statement | 12, 20, 25A, 34F | ✅ | ✅ | ✅ | ✅ |
| **MT935** | central_bank_rate_notification | Central bank rates | Policy rate change | CB rate notification | 20, 23, 30, 37#1-4, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT935** | deposit_rate_change | Deposit rate change | Savings rate update | Deposit rate notification | 20, 23, 30, 37K, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT935** | fx_rate_update | FX rate update | Exchange rate change | FX rate notification | 20, 23 (multiple), 30, 37K, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT935** | loan_rate_adjustment | Loan rate adjustment | Lending rate change | Loan rate notification | 20, 23, 30, 37K, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT935** | multi_product_rate_change | Multi-product rates | Various rate changes | Multiple rate updates | 20, 23 (multiple), 30, 37K, 72 | ✅ | ✅ | ✅ | ✅ |
| **MT940** | repeated_sequence_issues | Duplicate entry test | Testing edge cases | Repeated transactions | 20, 21, 25, 28C, 60F, 61 (multiple), 62F, 86 | ✅ | ✅ | ✅ | ✅ |
| **MT940** | daily_account_statement | Daily statement | End-of-day statement | 5 transactions, all types | 20, 21, 25, 28C, 60F, 61 (multiple), 62F, 64, 86 | ✅ | ✅ | ✅ | ✅ |
| **MT940** | interim_statement_intraday | Interim statement | Intraday position | Real-time FX transactions | 20, 21, 25, 28C, 60M, 61, 62M, 86 | ✅ | ✅ | ✅ | ✅ |
| **MT940** | year_end_statement | Annual statement | Year-end summary | Quarterly summaries, annual fees | 20, 21, 25, 28C, 60F, 61, 62F, 64, 86 | ✅ | ✅ | ✅ | ✅ |
| **MT941** | daily_balance_report | Daily balance | End-of-day balance | Daily position report | 20, 25, 28, 62F, 64, 65 | ✅ | ✅ | ✅ | ✅ |
| **MT941** | multi_currency_balance | Multi-currency | Multiple currencies | Multi-CCY position | 20, 25, 28, 62F (multiple), 64, 65 | ✅ | ✅ | ✅ | ✅ |
| **MT941** | negative_balance_report | Negative balance | Overdraft position | Negative position report | 20, 25, 28, 62F, 64, 65 | ✅ | ✅ | ✅ | ✅ |
| **MT942** | intraday_liquidity_report | Liquidity report | Intraday liquidity | Real-time liquidity | 13D, 20, 25, 28C, 34F, 60F, 61, 62F, 64, 65, 86, 90D | ✅ | ✅ | ✅ | ✅ |
| **MT942** | real_time_position_update | Real-time position | Live position update | Current position | 13D, 20, 25, 28C, 34F, 60F, 61, 62F, 64, 65, 86 | ✅ | ✅ | ✅ | ✅ |
| **MT942** | treasury_cash_sweep | Cash sweep | Treasury operations | Automated sweep | 13D, 20, 25, 28C, 34F, 60F, 61, 62F, 64, 65, 86, 90D | ✅ | ✅ | ✅ | ✅ |
| **MT950** | correspondent_banking | Correspondent statement | Nostro account | Correspondent position | 20, 25, 28C, 60F, 61 (multiple), 62F, 64 | ✅ | ✅ | ✅ | ✅ |
| **MT950** | high_volume_batch | High volume | Large transaction count | Batch processing | 20, 25, 28C, 60F, 61 (multiple), 62F, 64 | ✅ | ✅ | ✅ | ✅ |
| **MT950** | simplified_statement | Simplified statement | Basic statement | Simple format | 20, 25, 28C, 60F, 61, 62F | ✅ | ✅ | ✅ | ✅ |

## Current Test Status

| Component | Status | Details |
|-----------|--------|---------|
| **Sample Generator** | ✅ | All scenarios generate valid samples using datafake |
| **Parsing** | ✅ | 100% parsing success rate across all scenarios |
| **Validation** | ✅ | All business rules and SWIFT validations pass |
| **Round Trip Test** | ✅ | Full JSON→MT→JSON round trip successful |

### Test Coverage
- **Total Scenarios**: 168
- **Message Types**: 24 (MT101-MT950)
- **CBPR+ Scenarios**: 57 (cross-border payment compliance)
- **Success Rate**: 100%

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Passed/Complete |
| ❌ | Failed |
| ⚠️ | Partial/Warning |
| ⏳ | In Progress |
| ❔ | Not Started |

## Running Tests

### Test All Scenarios
```bash
# Run all scenarios with default 100 samples each
cargo test round_trip_scenarios -- --nocapture

# Run with specific sample count
TEST_SAMPLE_COUNT=10 cargo test round_trip_scenarios -- --nocapture
```

### Test Specific Message Type
```bash
# Test all MT103 scenarios
TEST_MESSAGE_TYPE=MT103 cargo test round_trip_scenarios -- --nocapture

# Test all MT101 scenarios
TEST_MESSAGE_TYPE=MT101 cargo test round_trip_scenarios -- --nocapture
```

### Test Specific Scenario
```bash
# Test a specific scenario
TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=cbpr_business_payment cargo test round_trip_scenarios -- --nocapture
```

### Debug Mode
```bash
# Enable debug output for detailed error information
TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=high_value TEST_DEBUG=1 cargo test round_trip_scenarios -- --nocapture

# Debug with single sample for focused troubleshooting
TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=high_value TEST_DEBUG=1 TEST_SAMPLE_COUNT=1 cargo test round_trip_scenarios -- --nocapture

# Stop on first failure
TEST_MESSAGE_TYPE=MT103 TEST_DEBUG=1 TEST_STOP_ON_FAILURE=1 cargo test round_trip_scenarios -- --nocapture
```

### Environment Variables

| Variable | Purpose | Default | Example |
|----------|---------|---------|---------|
| `TEST_MESSAGE_TYPE` | Test specific message type | All | `MT103` |
| `TEST_SCENARIO` | Test specific scenario | All | `cbpr_business_payment` |
| `TEST_DEBUG` | Enable debug output | Disabled | `1` |
| `TEST_SAMPLE_COUNT` | Number of samples per scenario | 100 | `10` |
| `TEST_STOP_ON_FAILURE` | Stop on first failure | Continue | `1` |

## Test Process

For each scenario, the round-trip test performs:

1. **Generation**: Create sample messages using scenario configuration with datafake
2. **MT Conversion**: Convert structured message to SWIFT MT format
3. **Parsing**: Parse MT format back to structured data
4. **Validation**: Apply all SWIFT validation rules and business logic
5. **Round Trip**: Serialize to JSON and back to ensure data integrity
6. **Comparison**: Verify original and round-trip results match exactly

## Adding New Scenarios

1. Create a new JSON file in the appropriate message type directory:
   ```json
   {
     "variables": {
       "sender_bic": {"fake": ["bic"]},
       "amount": {"fake": ["f64", 1000.0, 50000.0]}
     },
     "schema": {
       "basic_header": { ... },
       "application_header": { ... },
       "message_type": "103",
       "fields": { ... }
     }
   }
   ```

2. Add the scenario name to the `index.json` file

3. Update this README with the scenario details

4. Test the new scenario:
   ```bash
   TEST_MESSAGE_TYPE=MT103 TEST_SCENARIO=your_new_scenario cargo test round_trip_scenarios -- --nocapture
   ```

## Troubleshooting

### Common Issues

1. **Field Format Errors**: Check SWIFT field format specifications
2. **Validation Failures**: Review business rules in message definitions
3. **Round Trip Mismatches**: Ensure proper serialization attributes

### Debug Process

1. Run with debug mode to see detailed error messages
2. Use single sample count to focus on specific failure
3. Check generated MT format for field formatting issues
4. Review validation rules if business rule errors occur

## Recent Updates

### December 2024
- Fixed MT104 CBPR+ scenarios: Updated field naming conventions (21→21R, 50K→50#2)
- Fixed MT192 cbpr_cancellation_request: Corrected Field11S date format
- Fixed MT210 fx_settlement_notice: Improved validation rule C1
- Fixed MT910 cbpr_credit_confirmation: Updated validation rule field paths
- Added CBPR+ compliance scenarios for MT192 and MT196 cancellation workflows
- All 168 test scenarios now pass validation and round-trip tests