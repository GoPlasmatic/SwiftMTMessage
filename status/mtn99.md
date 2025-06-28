# MT199 / MT299 Field Support Matrix

This document outlines the fields used in MT199 and MT299 messages (Free Format Message). Fields are categorized as Mandatory (M) or Optional (O).

---

## Field Support Table

| Tag   | Field Name                | MT199 | MT299 | Notes |
|--------|---------------------------|--------|--------|-------|
| 20     | Transaction Reference Number | ✔️ M   | ✔️ M   | Unique reference assigned by the sender (16x) |
| 21     | Related Reference           | ✔️ O   | ✔️ O   | Reference to related message (16x) |
| 79     | Narrative                   | ✔️ M   | ✔️ M   | Free format message text (35*50x) |

---

## Notes

- MT199 and MT299 are used for information where no specific message type exists.
- If the narrative (`79`) starts with `/REJT/` or `/RETN/` in category 1 or 2 messages, it must follow Payments Reject/Return Guidelines.
- The category digit (1 or 2) should be chosen so the message routes to the correct unit at the receiver.