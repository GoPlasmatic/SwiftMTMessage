use serde_json::json;
use swift_mt_message::{SwiftMessage, messages::MT202};

#[test]
fn test_mt202_optional_fields() {
    let json = json!({
        "message_type": "202",
        "basic_header": {
            "application_id": "F",
            "service_id": "01",
            "sender_bic": "DEUTDEFF",
            "logical_terminal": "DEUTDEFFXXXX",
            "session_number": "0001",
            "sequence_number": "006789"
        },
        "application_header": {
            "direction": "I",
            "message_type": "202",
            "receiver_bic": "DEUTDEFF",
            "destination_address": "DEUTDEFFXXXX",
            "priority": "U"
        },
        "user_header": {
            "service_type_identifier": "001",
            "validation_flag": "STP",
            "unique_end_to_end_reference": "550e8400-e29b-41d4-a716-446655440000"
        },
        "fields": {
            "20": {"reference": "TEST123"},
            "21": {"reference": "REF123"},
            "32A": {"value_date": "2024-12-23", "currency": "EUR", "amount": 1000000},
            "52": {"A": {"bic": "DEUTDEFFXXX"}},  // This field is being lost
            "53": {"A": {"bic": "BNPAFRPPXXX"}},  // This field is being lost
            "58": {"A": {"bic": "DEUTDEFF"}},
            "72": {"information": ["/TEST/"]}
        }
    });

    // Don't print the entire JSON in test

    // Try to parse this as MT202
    let msg: SwiftMessage<MT202> = serde_json::from_value(json.clone()).unwrap();

    // Convert to MT
    let mt_message = msg.to_mt_message();
    println!("\nGenerated MT:\n{}", mt_message);

    // Check if field 52 is present
    assert!(
        mt_message.contains(":52"),
        "Field 52 should be present in MT message"
    );
    assert!(
        mt_message.contains(":53"),
        "Field 53 should be present in MT message"
    );
    assert!(
        mt_message.contains(":58"),
        "Field 58 should be present in MT message"
    );
}
