# MT202 COV Variant Field Support Matrix

This document outlines the fields supported in the MT202 COV message format, broken down by sequences and grouped into Mandatory (M), Optional (O), or Conditional (C). It also includes a table of conditional rules applicable to field usage.

---

## Field Support Table

### Sequence A: General Information

| Tag  | Field Name                  | Presence | Notes |
|------|-----------------------------|----------|-------|
| 20   | Transaction Reference Number | ✔️ M   | 16x; Unique identifier for the message |
| 21   | Related Reference            | ✔️ M   | Reference to underlying transaction, e.g. MT103 |
| 13C  | Time Indication              | ✔️ O   | Optional time and UTC offset info |
| 32A  | Value Date, Currency, Amount| ✔️ M   | 6!n3!a15d; Core settlement field |
| 52a  | Ordering Institution         | ✔️ O   | A or D option |
| 53a  | Sender's Correspondent       | ✔️ O   | A, B or D options |
| 54a  | Receiver's Correspondent     | ✔️ O   | A, B or D options |
| 56a  | Intermediary Institution     | ✔️ O   | A or D option |
| 57a  | Account With Institution     | ✔️ C1  | Required if 56a is present |
| 58a  | Beneficiary Institution      | ✔️ M   | A or D option |
| 72   | Sender to Receiver Info      | ✔️ O   | Structured only, coded instructions |

---

### Sequence B: Underlying Customer Credit Transfer Details

| Tag  | Field Name                     | Presence | Notes |
|------|--------------------------------|----------|-------|
| 50a  | Ordering Customer              | ✔️ M   | A, F, or K option |
| 52a  | Ordering Institution           | ✔️ O   | A or D option |
| 56a  | Intermediary Institution       | ✔️ O   | A, C or D option |
| 57a  | Account With Institution       | ✔️ C2  | Required if 56a is present |
| 59a  | Beneficiary Customer           | ✔️ M   | A or F or no option |
| 70   | Remittance Information         | ✔️ O   | 4*35x; free text or coded references |
| 72   | Sender to Receiver Information | ✔️ O   | Structured; use /ACC/, /INS/, etc. |
| 33B  | Currency/Instructed Amount     | ✔️ O   | Informational value if present |

---

## Conditional Rule Explanations

| Condition | Description |
|-----------|-------------|
| C1 | If field 56a is present in Sequence A, field 57a must also be present |
| C2 | If field 56a is present in Sequence B, field 57a must also be present |

---

## Notes

- Field 121 (UETR) in the user header block is mandatory.
- MT202 COV must always relate to an underlying customer credit transfer, usually an MT103.
- All financial institution fields (52a–58a) must refer to registered BICs or structured codes.
- Structured formats must be respected (especially for options A/D/F etc.).

