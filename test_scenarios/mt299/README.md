# MT299 Test Scenarios

## Overview

MT299 (Free Format Message) test scenarios for financial institution transfers. These scenarios demonstrate various use cases for institutional communication including CBPR+ payment responses.

## Scenarios

### 1. CBPR+ Payment Response (`cbpr_payment_response.json`)

**Purpose**: Demonstrates MT299 response to payment inquiry with full CBPR+ transparency data.

**Key Features**:
- Complete transparency path with timestamps
- Detailed fee breakdown at each stage
- CBPR+ compliance summary
- Processing time vs SLA comparison
- UETR tracking throughout the payment chain

**Use Case**: Response to MT199 inquiry providing comprehensive payment status and routing information.

**Field Highlights**:
- Field 20: Response reference (RSP prefix)
- Field 21: Links to original inquiry
- Field 79: Structured narrative with transparency path

## Field Usage

### Common Fields

| Field | Description | Usage Pattern |
|-------|-------------|---------------|
| 20 | Sender's Reference | Message-specific reference |
| 21 | Related Reference | Links to related message/transaction |
| 79 | Narrative | Free format text (35*50x) |

### Narrative Structure Guidelines

The MT299 narrative (Field 79) supports various structured formats:

1. **Payment Responses**: Clear status updates with transparency data
2. **Operational Messages**: Treasury and settlement communications
3. **Compliance Information**: Regulatory and screening results

## Compliance Considerations

### CBPR+ Requirements
- Full transparency path documentation
- Complete fee disclosure
- LEI verification results
- Purpose code validation status

### Regulatory Aspects
- Audit trail maintenance
- Customer protection compliance
- Timely response requirements

## Best Practices

1. **Clear Structure**: Use consistent formatting in narrative
2. **Complete Information**: Include all relevant transaction details
3. **Traceability**: Maintain clear reference links
4. **Timeliness**: Note response time requirements

## Integration Notes

- Works with MT199 for inquiry/response patterns
- Supports automated parsing of structured content
- Compatible with CBPR+ transparency requirements
- Integrates with treasury management systems