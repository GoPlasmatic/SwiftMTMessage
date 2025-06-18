# MT203 Field Specification

| Tag   | Field Name                     | Format / Rules   | Mandatory / Optional   | Sequence   |
|:------|:-------------------------------|:-----------------|:-----------------------|:-----------|
| 19    | Sum of Amounts                 | 17d              | Mandatory              | A          |
| 30    | Value Date                     | 6!n              | Mandatory              | A          |
| 52a   | Ordering Institution           | A, D options     | Optional               | A          |
| 53a   | Sender's Correspondent         | A, B, D options  | Optional               | A          |
| 54a   | Receiver's Correspondent       | A, B, D options  | Optional               | A          |
| 72    | Sender to Receiver Information | 6*35x            | Optional               | A          |
| 20    | Transaction Reference Number   | 16x              | Mandatory              | B          |
| 21    | Related Reference              | 16x              | Mandatory              | B          |
| 32B   | Currency Code, Amount          | 3!a15d           | Mandatory              | B          |
| 56a   | Intermediary                   | A, D options     | Optional               | B          |
| 57a   | Account With Institution       | A, B, D options  | Conditional            | B          |
| 58a   | Beneficiary Institution        | A, D options     | Mandatory              | B          |
| 72    | Sender to Receiver Information | 6*35x            | Optional               | B          |

---

# MT204 Field Specification

| Tag   | Field Name                     | Format / Rules   | Mandatory / Optional   | Sequence   |
|:------|:-------------------------------|:-----------------|:-----------------------|:-----------|
| 20    | Transaction Reference Number   | 16x              | Mandatory              | A          |
| 19    | Sum of Amounts                 | 17d              | Mandatory              | A          |
| 30    | Value Date                     | 6!n              | Mandatory              | A          |
| 57a   | Account With Institution       | A, B, D options  | Optional               | A          |
| 58a   | Beneficiary Institution        | A, D options     | Optional               | A          |
| 72    | Sender to Receiver Information | 6*35x            | Optional               | A          |
| 20    | Transaction Reference Number   | 16x              | Mandatory              | B          |
| 21    | Related Reference              | 16x              | Optional               | B          |
| 32B   | Transaction Amount             | 3!a15d           | Mandatory              | B          |
| 53a   | Debit Institution              | A, B, D options  | Mandatory              | B          |
| 72    | Sender to Receiver Information | 6*35x            | Optional               | B          |