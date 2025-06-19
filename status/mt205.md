# MT205 Complete Field Specification (CORE, COV, REJT, RETN)

| Tag  | Field Name                     | Format / Rules                            | Mandatory / Optional | Sequence | Key Validation / Extra Rules | Implementation Status |
|------|--------------------------------|--------------------------------------------|-----------------------|----------|------------------------------|----------------------|
| 20   | Transaction Reference Number   | `16x`                                      | Mandatory             | A        | Cannot start or end with `/`, no `//` allowed | ✅ **Complete**|
| 21   | Related Reference              | `16x`                                      | Mandatory             | A        | Reference to original transaction | ✅ **Complete**|
| 13C  | Time Indication                | `/8c/4!n1!x4!n`                           | Optional              | A        | SNDTIME/RNCTIME/CLSTIME/TILTIME/FROTIME/REJTIME | ✅ **Complete**|
| 32A  | Value Date/Currency/Amount     | `6!n3!a15d`                                | Mandatory             | A        | Valid YYMMDD date, ISO 4217 currency, decimal rules | ✅ **Complete**|
| 52a  | Ordering Institution           | `A, D options`                             | **Mandatory**         | A        | **Always mandatory in MT205** (no fallback to sender BIC) | ✅ **Complete**|
| 53a  | Sender's Correspondent         | `A, B, D options`                          | Optional              | A        | Used for settlement method determination | ✅ **Complete**|
| ~~54a~~ | ~~Receiver's Correspondent~~ | ~~`A, B, D options`~~                    | **Not Present**       | A        | **Field 54a is not present in MT205** | ❌ **Not Applicable**|
| 56a  | Intermediary Institution       | `A, D options`                             | Optional              | A        | BIC validation; clearing channel determination | ✅ **Complete**|
| 57a  | Account With Institution       | `A, B, D options`                          | Optional              | A        | BIC validation; creditor agent mapping | ✅ **Complete**|
| 58a  | Beneficiary Institution        | `A, D options`                             | Mandatory             | A        | BIC validation; creditor mapping | ✅ **Complete**|
| 72   | Sender to Receiver Information | `6*35x`                                    | Optional              | A        | Structured format with codes; /REJT/ and /RETN/ handling | ✅ **Complete**|

## MT205 COV Sequence B Fields (Cover Payments Only)

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

### MT205 CORE (→ pacs.009.001.08)
- **Purpose**: Basic bank-to-bank financial institution transfer
- **Settlement**: Serial payment method only (INGA/INDA) - **No cover payment detection**
- **Key Differences from MT202**: No field 54a, mandatory field 52a, uses METAFCT003
- **Total Fields**: 10 core fields (one less than MT202 due to missing 54a)
- **✅ Complete**: All fields implemented with proper validation

### MT205 COV (→ pacs.009.001.08) 
- **Purpose**: Cover payment with underlying customer credit transfer
- **Structure**: Sequence A (bank-to-bank) + Sequence B (customer details)
- **Key Differences from MT202**: No field 54a in Seq A, mandatory field 52a in Seq A
- **Total Fields**: 10 (Seq A) + 8 (Seq B) = 18 fields
- **✅ Complete**: All sequences implemented with customer transparency

### MT205 REJT (→ pacs.002.001.10)
- **Purpose**: Rejection of financial institution transfer
- **Key Field**: 72 must contain `/REJT/` indicator
- **Key Differences from MT202**: No field 54a, mandatory field 52a
- **Total Fields**: 10 fields (same as MT205 CORE but different validation)
- **✅ Complete**: Rejection handling and reason code mapping

### MT205 RETN (→ pacs.004.001.09)
- **Purpose**: Return of financial institution transfer  
- **Key Field**: 72 must contain `/RETN/` indicator
- **Key Differences from MT202**: No field 54a, mandatory field 52a
- **Total Fields**: 10 fields (same as MT205 CORE but different validation)
- **✅ Complete**: Return handling and reason code mapping

## Field Usage Matrix by Variant

| Field | CORE | COV-A | COV-B | REJT | RETN | Notes |
|-------|------|-------|-------|------|------|-------|
| 20    | M    | M     | -     | M    | M    | Transaction reference |
| 21    | M    | M     | -     | M    | M    | Related reference |
| 13C   | O    | O     | -     | O    | O    | Time indication |
| 32A   | M    | M     | -     | M    | M    | Value/Currency/Amount |
| 52a   | **M**| **M** | O     | **M**| **M**| **Always mandatory in MT205** |
| 53a   | O    | O     | -     | O    | O    | Settlement method (METAFCT003) |
| ~~54a~~|**-**|**-**  | -     | **-**| **-**| **Not present in MT205** |
| 56a   | O    | O     | O     | O    | O    | Intermediary |
| 57a   | O    | O     | O     | O    | O    | Account with |
| 58a   | M    | M     | -     | M    | M    | Beneficiary institution |
| 72    | O    | O     | O     | M*** | M*** | ***Must contain /REJT/ or /RETN/ |
| 50a   | -    | -     | M     | -    | -    | COV only: Ordering customer |
| 59a   | -    | -     | M     | -    | -    | COV only: Beneficiary customer |
| 70    | -    | -     | O     | -    | -    | COV only: Remittance info |
| 33B   | -    | -     | O     | -    | -    | COV only: Instructed amount |

## Key Differences from MT202

### 1. Field 54a (Receiver's Correspondent)
- **MT202**: Optional field present in all variants
- **MT205**: **Not present** - completely absent from the message structure
- **Impact**: Affects settlement method determination and routing logic

### 2. Field 52a (Ordering Institution)
- **MT202**: Optional (can fallback to sender BIC from message header)
- **MT205**: **Mandatory** - must always be present
- **Impact**: Always required for proper debtor agent identification

### 3. Settlement Method Determination
- **MT202**: Uses METAFCT002 with full 53a/54a matrix
- **MT205**: Uses METAFCT003 with limited scenarios:
  - Both 53a and 54a absent
  - 53a present and 54a absent (since 54a never exists)
- **Impact**: Simplified serial payment detection logic

### 4. Cover Payment Detection
- **MT202**: Has PREC003 for SERIAL vs COVER determination
- **MT205**: **No PREC003** - always processed as CORE (serial payment) initially
- **Impact**: Cover detection based solely on customer fields in Sequence B

## Special Validation Rules

### Settlement Method Determination (MT205 Specific - METAFCT003)
- **Limited Scenarios**: Only handles cases where 54a is absent
- **Serial Payment Default**: Always defaults to INDA settlement method
- **Cover Detection**: Based on presence of Sequence B customer fields (50a, 59a)

### Field 72 Code Handling (Same as MT202)
- **Standard Codes**: `/ACC/`, `/INS/`, `/INT/`, `/REC/`, `/BNF/`, `/TSU/`
- **Rejection**: Must contain `/REJT/` + reason codes
- **Return**: Must contain `/RETN/` + reason codes + `/MREF/` mandatory
- **Additional Info**: `/CHGS/`, `/TEXT/`, `/TREF/` for return processing

### COV Sequence Rules (Same as MT202)
- **Sequence A**: Bank-to-bank financial institution details (no 54a)
- **Sequence B**: Underlying customer credit transfer details
- **Mandatory Mapping**: DebtorAgent (52a seq B fallback to 52a seq A - always present)
- **Creditor Agent**: CreditorAgent (57a seq B fallback to 58a seq A)

### ISO 20022 Mapping
- **CORE/COV** → `pacs.009.001.08` (FIToFICreditTransferV08)
- **REJT** → `pacs.002.001.10` (FIToFIPaymentStatusReportV10)  
- **RETN** → `pacs.004.001.09` (PaymentReturnV09)

## Business Rule Implementation

### Conditional Logic (MT205 Specific)
- **C1**: Simplified settlement method determination (no 54a scenarios)
- **C2**: Cover payment detection based on Sequence B presence
- **C3**: REJT/RETN indicator validation in field 72
- **C4**: BIC validation and clearing system code handling
- **C5**: Account information mapping with mandatory 52a

### Error Handling
- **T20007**: Missing mandatory agent information
- **T11001**: Default value injection for compliance
- **T80**: REJT/RETN guideline compliance validation
- **MT205-Specific**: No fallback to sender BIC for field 52a

---

✅ **Implementation Complete:** All MT205 variants (CORE, COV, REJT, RETN) are fully implemented with MT205-specific field validation, simplified settlement method determination (METAFCT003), ISO 20022 mapping, and business rule compliance. Key differences from MT202 include the absence of field 54a and mandatory field 52a in all variants. 