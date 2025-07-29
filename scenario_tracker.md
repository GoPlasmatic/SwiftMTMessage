# Scenario Test Tracker

## Status Legend
- ✅ Passed
- ❌ Failed
- ⚠️ Partial
- ⏳ In Progress
- ❔ Not Started

| Message Type | Scenario | Parser | Validation | Round Trip | Notes |
|--------------|----------|--------|------------|------------|-------|
| MT101 | standard | ✅ | ✅ | ✅ | |
| MT101 | bulk_payment | ✅ | ✅ | ✅ | |
| MT101 | multi_currency | ✅ | ✅ | ✅ | |
| MT101 | scheduled_payment | ✅ | ✅ | ✅ | |
| MT101 | salary_payment | ✅ | ✅ | ✅ | |
| MT101 | vendor_payment | ✅ | ✅ | ✅ | |
| MT101 | urgent_payment | ✅ | ✅ | ✅ | |
| MT101 | direct_debit | ✅ | ✅ | ✅ | |
| MT101 | minimal | ✅ | ✅ | ✅ | |
| MT103 | All scenarios (55) | ✅ | ✅ | ✅ | All 55 MT103 scenarios passing |
| MT104 | fi_direct_debit_basic | ✅ | ✅ | ✅ | Fixed: Added 23E to seq B, moved Field50 to seq B as 50#2, removed 21R, added 32B to seq C |
| MT104 | fi_direct_debit_cbpr | ✅ | ✅ | ✅ | Fixed: Field77B narrative, removed 21R, removed 52 from seq A, added 23E to seq B, added 32B to seq C, shortened field 20, fixed field 70 to 4 lines |
| MT104 | fi_direct_debit_multiple | ✅ | ✅ | ✅ | Fixed: Fixed field 23E structure, set fixed amounts (250, 350, 400) for sum validation |
| MT104 | fi_direct_debit_recurring | ✅ | ✅ | ✅ | |
| MT104 | fi_direct_debit_return | ✅ | ✅ | ✅ | |
| MT107 | authorized_bulk_collection | ✅ | ✅ | ✅ | Fixed: removed field 23E/50/72 from seq A, added field 50 to seq B, fixed field 26T to use 3-letter code, fixed field 77B to use narrative, reduced field 70 to 4 lines, shortened field 20 |
| MT107 | general_direct_debit_basic | ✅ | ✅ | ✅ | Fixed: shortened field 20 reference length |
| MT107 | return_processing | ✅ | ✅ | ✅ | Fixed: shortened field 20, removed UETR from field 72, reduced field 70 to 4 lines, removed field 23E from seq B, added field 50#2 to seq A |
| MT107 | unauthorized_debit_processing | ✅ | ✅ | ✅ | Fixed: shortened field 20, changed field 77B to narrative, fixed field 26T type_code to 3-letter code, reduced field 70 to 4 lines, removed field 23E from seq B, changed field 50 to 50#2 option K, removed field 50 from seq B |
| MT110 | cheque_collection_advice | ✅ | ✅ | ✅ | Fixed: Changed date format from YYMMDD to ISO format (2024-12-20), dates were already converted to ISO format |
| MT111 | lost_cheque_stop | ✅ | ✅ | ✅ | |
| MT111 | fraud_prevention_stop | ✅ | ✅ | ✅ | |
| MT111 | duplicate_cheque_stop | ✅ | ✅ | ✅ | |
| MT112 | stop_payment_accepted | ✅ | ✅ | ✅ | |
| MT112 | stop_payment_pending | ✅ | ✅ | ✅ | |
| MT112 | stop_payment_rejected | ✅ | ✅ | ✅ | |
| MT192 | request_cancellation | ✅ | ✅ | ✅ | Fixed: Replaced date_format with static date, fixed Field11S to use static values, simplified Field79 information array to use static strings |
| MT196 | answer_cancellation | ✅ | ✅ | ✅ | Fixed: Changed Field79 from "narrative" to "information" |
| MT202 | cbpr_cov_standard | ✅ | ✅ | ✅ | |
| MT202 | cbpr_cov_complex_routing | ✅ | ✅ | ✅ | Fixed: Shortened Field21 reference, fixed Field72 line lengths, changed field 50 to 50#b |
| MT202 | cbpr_cov_compliance_enhanced | ✅ | ✅ | ✅ | Fixed: Changed field 50 to 50#b, reduced Field72#b to 6 lines |
| MT202 | cov_mismatch | ✅ | ✅ | ✅ | Fixed: Replaced expr with static amount, shortened references, moved sequence_b fields to root with #b suffix |
| MT202 | fi_to_fi_transparency | ✅ | ✅ | ✅ | Fixed: Shortened references, simplified Field72 with static UETR |
| MT205 | bank_transfer_non_cover | ✅ | ✅ | ✅ | Fixed: Corrected Field13C structure, shortened references |
| MT210 | expected_incoming_funds | ✅ | ✅ | ✅ | Fixed: Changed date_format to static date, fixed Field25 to use authorisation instead of account, changed Field30 to use execution_date, changed Field50 to NoOption variant |
| MT292 | fi_cancellation_request | ✅ | ✅ | ✅ | Fixed: Changed date_format to static date, removed empty string concatenation from Field21, changed Field79 from narrative to information, fixed Field11S structure, simplified switch/case to static values |
| MT296 | cancellation_accepted | ✅ | ✅ | ✅ | Fixed: Changed time_24h to static time, removed empty string concatenation from Field21, changed Field76 from answer to information, removed Field79 to comply with validation rule C1 |
| MT296 | cancellation_rejected | ✅ | ✅ | ✅ | Fixed: Removed empty string concatenation from Field21, changed Field76 from answer to information, removed Field11 and Field79 to comply with validation rule C1 |
| MT296 | inquiry_response | ✅ | ✅ | ✅ | Fixed: Removed empty string concatenation from Field21, changed Field76 from answer to information, removed Field79 to comply with validation rule C1 |
| MT296 | no_payment_found | ✅ | ✅ | ✅ | Fixed: Removed empty string from Field21, changed Field76 from answer to information, removed Field79 for C1 rule |
| MT296 | partial_cancellation | ✅ | ✅ | ✅ | Fixed: Removed empty string from Field21, changed Field76 from answer to information, removed Field79 for C1 rule |
| MT900 | basic_debit_confirmation | ✅ | ✅ | ✅ | Fixed: Removed empty string from Field21, changed Field25 from account to NoOption/authorisation |
| MT900 | direct_debit_confirmation | ✅ | ✅ | ✅ | Fixed: Removed empty string from Field21, changed Field25 from account to NoOption/authorisation |
| MT900 | fee_debit_confirmation | ✅ | ✅ | ✅ | Fixed: Removed empty string from Field21, changed Field25 from account to NoOption/authorisation |
| MT900 | fx_transaction_debit | ✅ | ✅ | ✅ | Fixed: Removed empty string from Field21, changed Field25 to NoOption/authorisation, fixed Field13D structure |
| MT900 | standing_order_debit | ✅ | ✅ | ✅ | Fixed: Removed empty string from Field21, changed Field25 to NoOption/authorisation, fixed Field13D structure, shortened Field21 reference, reduced Field72 to 6 lines |
| MT910 | basic_credit_confirmation | ✅ | ✅ | ✅ | Fixed: Changed Field25 from account to NoOption/authorisation |
| MT910 | dividend_payment | ✅ | ✅ | ✅ | Created file, fixed Field25 to NoOption/authorisation |
| MT910 | incoming_wire_transfer | ✅ | ✅ | ✅ | Created file, fixed Field25 to NoOption/authorisation |
| MT910 | interest_credit | ✅ | ✅ | ✅ | Created file, fixed Field25 to NoOption/authorisation |
| MT910 | refund_credit | ✅ | ✅ | ✅ | Created file, fixed Field25 to NoOption/authorisation |
| MT920 | interim_report_request | ✅ | ✅ | ✅ | Fixed: Changed Field12 from message_type to type_code, shortened Field20 reference, changed Field25NoOption to Field25A in message definition |
| MT920 | multi_account_request | ✅ | ✅ | ✅ | Fixed: Changed Field12 from message_type to type_code, shortened Field20 reference, changed Field25NoOption to Field25A in message definition |
| MT920 | statement_request_basic | ✅ | ✅ | ✅ | Fixed: Changed Field12 from message_type to type_code, changed Field25NoOption to Field25A in message definition |
| MT935 | central_bank_rate_notification | ❌ | ❌ | ❌ | |
| MT935 | deposit_rate_change | ❌ | ❌ | ❌ | |
| MT935 | fx_rate_update | ❌ | ❌ | ❌ | |
| MT935 | loan_rate_adjustment | ❌ | ❌ | ❌ | |
| MT935 | multi_product_rate_change | ❌ | ❌ | ❌ | |
| MT940 | repeated_sequence_issues | ❌ | ❌ | ❌ | |
| MT941 | daily_balance_report | ❌ | ❌ | ❌ | |
| MT941 | multi_currency_balance | ❌ | ❌ | ❌ | |
| MT941 | negative_balance_report | ❌ | ❌ | ❌ | |
| MT942 | intraday_liquidity_report | ❌ | ❌ | ❌ | |
| MT942 | real_time_position_update | ❌ | ❌ | ❌ | |
| MT942 | treasury_cash_sweep | ❌ | ❌ | ❌ | |
| MT950 | correspondent_banking | ❌ | ❌ | ❌ | |
| MT950 | high_volume_batch | ❌ | ❌ | ❌ | |
| MT950 | simplified_statement | ❌ | ❌ | ❌ | |