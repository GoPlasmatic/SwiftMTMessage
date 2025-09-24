# Round-Trip Test Issues - Updated Analysis

## Current Test Status

### Overall Statistics
- **Total tests**: 1950
- **Passing**: 1556 (79.8%)
- **Failing**: 394 (20.2%)

### Recent Progress
- **Original baseline**: 1558 passing
- **After refactoring**: 1545 passing (degraded)
- **After enum fix**: 1556 passing (nearly restored)

## Issues by Priority and Impact

## Priority 1: High-Volume Message Types (180+ failures)

### Issue 1.1: MT202 (Financial Institution Transfer) - 60 failures
**Impact**: CRITICAL - Core payment infrastructure message
- **Failing scenarios**: 6 of 10
  - cbpr_cov_compliance_enhanced
  - cbpr_cov_rejection
  - cbpr_cov_return
  - cbpr_serial_payment
  - cov_mismatch
  - fi_to_fi_transparency

**Likely Causes**:
- CBPR (Cross-Border Payment Reporting) fields not properly handled
- Cover payment specific fields losing information
- Sequence ordering issues in multi-part messages

### Issue 1.2: MT196 (Answers) - 50 failures
**Impact**: HIGH - Query response messages
- **Failing scenarios**: 5 of 7
  - answer_cancellation
  - answer_inquiry_response
  - answer_pending_investigation
  - answer_rejection
  - cbpr_cancellation_response

**Likely Causes**:
- Complex narrative field handling
- Reference linking issues
- Answer code preservation

## Priority 2: Multi-Transaction Messages (40-110 failures)

### Issue 2.1: MT110 (Advice of Cheque) - 40 failures
**Impact**: MEDIUM - Cheque processing
- **Failing scenarios**: 4 of 6 (all scenarios fail)
  - cheque_collection_advice
  - foreign_cheque_collection
  - returned_cheque_advice
  - single_cheque_advice

**Likely Causes**:
- Cheque detail fields serialization
- Amount and date format issues

### Issue 2.2: MT107 (General Direct Debit) - 20 failures
**Impact**: MEDIUM - Direct debit processing
- **Failing scenarios**: 2 of 4
  - authorized_bulk_collection
  - general_direct_debit_basic

**Likely Causes**:
- Transaction array handling (similar to MT104)
- Mandate information preservation

### Issue 2.3: MT205 (Financial Institution Transfer Execution) - 40 failures
**Impact**: MEDIUM - Execution confirmations
- **Failing scenarios**: 4 of 7

### Issue 2.4: MT204 (Financial Market Direct Debit) - 30 failures
**Impact**: MEDIUM - Market transactions
- **Failing scenarios**: 3 of 5

## Priority 3: Statement and Reporting Messages (30-50 failures)

### Issue 3.1: MT942 (Interim Transaction Report) - 30 failures
**Impact**: MEDIUM - Intraday reporting
- **Failing scenarios**: 3 of 5
  - intraday_liquidity_report
  - real_time_position_update
  - treasury_cash_sweep

**Known Issues**:
- Field61: Bank reference incorrectly parsed
- Field13D: Time format mismatch (14:30 vs 14:30:00)
- Field86: Final narrative becoming null

### Issue 3.2: MT935 (Rate Change Advice) - 30 failures
**Impact**: LOW - Rate notifications
- **Failing scenarios**: 3 of 6
  - central_bank_rate_notification
  - deposit_rate_change
  - loan_rate_adjustment

### Issue 3.3: MT940 (Customer Statement) - 10 failures
**Impact**: LOW - Mostly fixed
- **Remaining scenario**: 1 of 4
  - repeated_sequence_issues (complex multi-sequence handling)

## Priority 4: Internal Transfer Messages (30 failures)

### Issue 4.1: MT200 (Financial Institution Transfer for Own Account) - 30 failures
**Impact**: LOW - Internal operations
- **Failing scenarios**: 3 of 5
  - fx_position
  - liquidity_transfer
  - nostro_funding

### Issue 4.2: MT210 (Notice to Receive) - 30 failures
**Impact**: LOW - Notification messages
- **Failing scenarios**: 3 of 5

## Priority 5: Minor Issues (10-20 failures)

### Issue 5.1: MT920 (Request for Message) - 10 failures
**Impact**: LOW - Message requests
- **Failing scenarios**: 1 of 3
  - interim_report_request

### Issue 5.2: MT900 (Confirmation of Debit) - 10 failures
**Impact**: LOW - Debit confirmations
- **Failing scenarios**: 1 of 4

## Root Cause Analysis

### Common Patterns Across Failures

1. **Complex Field Serialization**
   - Multi-part fields losing structure
   - Nested JSON not properly preserved
   - Array fields with complex objects

2. **Sequence Handling**
   - Messages with multiple sequences (A, B, C)
   - Transaction arrays in MT104, MT107, MT110
   - Repeated sequences in MT940

3. **Field Format Mismatches**
   - Time formats (HH:MM vs HH:MM:SS)
   - Date formats in different contexts
   - Amount precision differences

4. **Narrative Field Issues**
   - Field72, Field79, Field86 multiline handling
   - Structured vs unstructured narrative
   - Line length and continuation

5. **Reference Field Parsing**
   - Complex reference structures being simplified
   - Cross-reference linking lost
   - Supplementary details misplaced

## Recommended Fixes by Priority

### Immediate (Fix 60% of failures)
1. **Fix MT202 CBPR field handling** - Would fix 60 tests
2. **Fix MT196 narrative preservation** - Would fix 50 tests
3. **Fix MT110 cheque detail serialization** - Would fix 40 tests

### Short-term (Fix 20% of failures)
4. **Fix Field61 parsing in MT942** - Would fix 30 tests
5. **Fix time format standardization** - Would fix multiple message types
6. **Fix MT107 transaction array handling** - Would fix 20 tests

### Long-term (Fix remaining 20%)
7. **Improve complex sequence handling** - MT940, MT104
8. **Standardize narrative field processing** - All messages
9. **Enhanced field format validation** - Prevent mismatches

## Testing Strategy

### Unit Tests Needed
- Field61 bank reference parsing
- Field13D time format handling
- Field72/79/86 narrative serialization
- CBPR-specific field handling

### Integration Tests Needed
- MT202 cover payment scenarios
- MT196 answer message chains
- MT110 cheque collection workflows
- Multi-sequence message handling

### Regression Prevention
- Add round-trip tests for each fixed scenario
- Validate against real-world message samples
- Performance testing for large transaction arrays

## Implementation Roadmap

### Phase 1 (Week 1) - High Impact
- [ ] Fix MT202 CBPR fields
- [ ] Fix MT196 narrative handling
- [ ] Add unit tests for fixed fields

### Phase 2 (Week 2) - Medium Impact
- [ ] Fix MT110 cheque serialization
- [ ] Fix MT942 Field61/13D/86
- [ ] Fix MT107 transaction arrays

### Phase 3 (Week 3) - Completion
- [ ] Fix remaining MT940 scenario
- [ ] Fix MT200/210 internal transfers
- [ ] Comprehensive testing

## Success Metrics

### Target Goals
- **Short-term**: Restore to 85% passing (1658/1950)
- **Medium-term**: Achieve 95% passing (1853/1950)
- **Long-term**: Achieve 99%+ passing (1931/1950)

### Current vs Target
- **Current**: 1556/1950 (79.8%)
- **After Priority 1**: ~1736/1950 (89%)
- **After Priority 2**: ~1816/1950 (93%)
- **After All Fixes**: ~1931/1950 (99%)

## Risk Assessment

### High Risk Areas
- MT202 CBPR changes could affect production payments
- MT196 changes could break answer message chains
- Field format changes need backward compatibility

### Mitigation Strategies
- Extensive testing with real message samples
- Gradual rollout with feature flags
- Maintain backward compatibility mode
- Clear migration documentation