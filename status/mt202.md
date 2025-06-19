# MT202 Complete Field Specification (CORE, COV, REJT, RETN)

| Tag  | Field Name                     | Format / Rules                            | Mandatory / Optional | Sequence | Key Validation / Extra Rules | Implementation Status |
|------|--------------------------------|--------------------------------------------|-----------------------|----------|------------------------------|----------------------|
| 20   | Transaction Reference Number   | `16x`                                      | Mandatory             | A        | Cannot start or end with `/`, no `//` allowed | ✅ **Complete**|
| 21   | Related Reference              | `16x`                                      | Mandatory             | A        | Reference to original transaction | ✅ **Complete**|
| 13C  | Time Indication                | `/8c/4!n1!x4!n`                           | Optional              | A        | SNDTIME/RNCTIME/CLSTIME/TILTIME/FROTIME/REJTIME | ✅ **Complete**|
| 32A  | Value Date/Currency/Amount     | `6!n3!a15d`                                | Mandatory             | A        | Valid YYMMDD date, ISO 4217 currency, decimal rules | ✅ **Complete**|
| 52a  | Ordering Institution           | `A, D options`                             | Optional/Mandatory*   | A        | Mandatory in MT205; BIC validation for option A | ✅ **Complete**|
| 53a  | Sender's Correspondent         | `A, B, D options`                          | Optional              | A        | Used for settlement method determination | ✅ **Complete**|
| 54a  | Receiver's Correspondent       | `A, B, D options`                          | Optional              | A        | Not present in MT205; used for settlement method | ✅ **Complete**|
| 56a  | Intermediary Institution       | `A, D options`                             | Optional              | A        | BIC validation; clearing channel determination | ✅ **Complete**|
| 57a  | Account With Institution       | `A, B, D options`                          | Optional              | A        | BIC validation; creditor agent mapping | ✅ **Complete**|
| 58a  | Beneficiary Institution        | `A, D options`                             | Mandatory             | A        | BIC validation; creditor mapping | ✅ **Complete**|
| 72   | Sender to Receiver Information | `6*35x`                                    | Optional              | A        | Structured format with codes; /REJT/ and /RETN/ handling | ✅ **Complete**|

## MT202 COV Sequence B Fields (Cover Payments Only)

| Tag  | Field Name                     | Format / Rules                            | Mandatory / Optional | Key Validation / Extra Rules | Implementation Status |
|------|--------------------------------|--------------------------------------------|-----------------------|------------------------------|----------------------|
| 50a  | Ordering Customer              | `A, F, K options`                          | Mandatory             | Customer identification; FATF compliance | ✅ **Complete**|
| 52a  | Ordering Institution (Seq B)   | `A, D options`                             | Optional              | Debtor agent for underlying transfer | ✅ **Complete**|
| 56a  | Intermediary Institution (Seq B)| `A, C, D options`                          | Optional              | Intermediary agent for underlying transfer | ✅ **Complete**|
| 57a  | Account With Institution (Seq B)| `A, B, C, D options`                       | Optional              | Creditor agent for underlying transfer | ✅ **Complete**|
| 59a  | Beneficiary Customer           | `A, F, no letter options`                  | Mandatory             | Customer identification; structured address | ✅ **Complete**|
| 70   | Remittance Information         | `4*35x`                                    | Optional              | Free text or coded references | ✅ **Complete**|
| 72   | Sender to Receiver Info (Seq B)| `6*35x`                                    | Optional              | Structured format for underlying transfer | ✅ **Complete**|
| 33B  | Currency/Instructed Amount     | `3!a15d`                                   | Optional              | For currency conversion scenarios | ✅ **Complete**|

## Implementation Summary by Variant

### MT202 CORE (→ pacs.009.001.08)
- **Purpose**: Basic bank-to-bank financial institution transfer
- **Settlement**: Serial payment method (INGA/INDA)
- **Total Fields**: 11 core fields
- **✅ Complete**: All fields implemented with proper validation

### MT202 COV (→ pacs.009.001.08) 
- **Purpose**: Cover payment with underlying customer credit transfer
- **Structure**: Sequence A (bank-to-bank) + Sequence B (customer details)
- **Total Fields**: 11 (Seq A) + 8 (Seq B) = 19 fields
- **✅ Complete**: All sequences implemented with customer transparency

### MT202 REJT (→ pacs.002.001.10)
- **Purpose**: Rejection of financial institution transfer
- **Key Field**: 72 must contain `/REJT/` indicator
- **Total Fields**: 11 fields (same as CORE but different validation)
- **✅ Complete**: Rejection handling and reason code mapping

### MT202 RETN (→ pacs.004.001.09)
- **Purpose**: Return of financial institution transfer  
- **Key Field**: 72 must contain `/RETN/` indicator
- **Total Fields**: 11 fields (same as CORE but different validation)
- **✅ Complete**: Return handling and reason code mapping

## Field Usage Matrix by Variant

| Field | CORE | COV-A | COV-B | REJT | RETN | Notes |
|-------|------|-------|-------|------|------|-------|
| 20    | M    | M     | -     | M    | M    | Transaction reference |
| 21    | M    | M     | -     | M    | M    | Related reference |
| 13C   | O    | O     | -     | O    | O    | Time indication |
| 32A   | M    | M     | -     | M    | M    | Value/Currency/Amount |
| 52a   | O/M* | O/M*  | O     | O    | O    | *Mandatory in MT205 |
| 53a   | O    | O     | -     | O    | O    | Settlement method |
| 54a   | O    | O**   | -     | O    | O    | **Not in MT205 |
| 56a   | O    | O     | O     | O    | O    | Intermediary |
| 57a   | O    | O     | O     | O    | O    | Account with |
| 58a   | M    | M     | -     | M    | M    | Beneficiary institution |
| 72    | O    | O     | O     | M*** | M*** | ***Must contain /REJT/ or /RETN/ |
| 50a   | -    | -     | M     | -    | -    | COV only: Ordering customer |
| 59a   | -    | -     | M     | -    | -    | COV only: Beneficiary customer |
| 70    | -    | -     | O     | -    | -    | COV only: Remittance info |
| 33B   | -    | -     | O     | -    | -    | COV only: Instructed amount |

## Special Validation Rules

### Settlement Method Determination (CORE/COV)
- **METAFCT002** (CORE): Handles 53a/54a presence for settlement method
- **METAFCT003** (COV): Enhanced logic for cover payment scenarios
- Settlement methods: INGA (indirect agent) or INDA (indirect debtor agent)

### Field 72 Code Handling
- **Standard Codes**: `/ACC/`, `/INS/`, `/INT/`, `/REC/`, `/BNF/`, `/TSU/`
- **Rejection**: Must contain `/REJT/` + reason codes
- **Return**: Must contain `/RETN/` + reason codes + `/MREF/` mandatory
- **Additional Info**: `/CHGS/`, `/TEXT/`, `/TREF/` for return processing

### COV Sequence Rules
- **Sequence A**: Bank-to-bank financial institution details
- **Sequence B**: Underlying customer credit transfer details
- **Mandatory Mapping**: DebtorAgent (52a seq B fallback to 52a seq A or sender BIC)
- **Creditor Agent**: CreditorAgent (57a seq B fallback to 58a seq A)

### ISO 20022 Mapping
- **CORE/COV** → `pacs.009.001.08` (FIToFICreditTransferV08)
- **REJT** → `pacs.002.001.10` (FIToFIPaymentStatusReportV10)  
- **RETN** → `pacs.004.001.09` (PaymentReturnV09)

## Business Rule Implementation

### Conditional Logic (All Variants)
- **C1**: Settlement method determination based on 53a/54a presence
- **C2**: Cover payment detection and sequence processing
- **C3**: REJT/RETN indicator validation in field 72
- **C4**: BIC validation and clearing system code handling
- **C5**: Account information mapping with fallback rules

### Error Handling
- **T20007**: Missing mandatory agent information
- **T11001**: Default value injection for compliance
- **T80**: REJT/RETN guideline compliance validation

---

✅ **Implementation Complete:** All MT202 variants (CORE, COV, REJT, RETN) are fully implemented with comprehensive field validation, settlement method determination, ISO 20022 mapping, and business rule compliance. The implementation covers both MT202 and MT205 scenarios with proper sequence handling for cover payments. 