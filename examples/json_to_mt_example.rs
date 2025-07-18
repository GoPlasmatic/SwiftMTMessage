use swift_mt_message::{messages::MT103, SwiftMessage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 SWIFT MT Message: JSON ↔ MT Format Example");
    println!("{}", "=".repeat(60));
    println!();

    // JSON representation of an MT103 message
    // This matches the structure that the library expects for deserialization
    let json_content = r#"{
                    "application_header": {
                        "destination_address": "BANKDEFFXXXX",
                        "direction": "I",
                        "message_type": "103",
                        "priority": "N",
                        "receiver_bic": {
                            "bank_code": "BANK",
                            "country_code": "DE",
                            "location_code": "FF",
                            "raw": "BANKDEFF"
                        }
                    },
                    "basic_header": {
                        "application_id": "F",
                        "logical_terminal": "BANKBEBBAXXX",
                        "sender_bic": {
                            "bank_code": "BANK",
                            "country_code": "BE",
                            "location_code": "BB",
                            "raw": "BANKBEBB"
                        },
                        "sequence_number": "000000",
                        "service_id": "01",
                        "session_number": "0000"
                    },
                    "fields": {
                        "20": {
                            "value": "REF123456789"
                        },
                        "23B": {
                            "value": "CRED"
                        },
                        "32A": {
                            "amount": 123456.78,
                            "currency": "EUR",
                            "value_date": "2025-06-15"
                        },
                        "50": {
                            "K": {
                                "name_and_address": [
                                    "/NOTPROVIDED",
                                    "John Doe"
                                ]
                            }
                        },
                        "52A": {
                            "bic": {
                                "bank_code": "BANK",
                                "country_code": "BE",
                                "location_code": "BB",
                                "raw": "BANKBEBB"
                            }
                        },
                        "57A": {
                            "bic": {
                                "bank_code": "BANK",
                                "country_code": "DE",
                                "location_code": "FF",
                                "raw": "BANKDEFF"
                            }
                        },
                        "59": {
                            "NoOption": {
                                "lines": [
                                    "/NOTPROVIDED",
                                    "456 Avenue"
                                ]
                            }
                        },
                        "70": {
                            "lines": [
                                "/ROC/NOTPROVIDED",
                                "INVOICE 45678PAYMENT FOR SERVICESRENDERED IN DECEMBERWITH ADDITIONAL NOTES"
                            ]
                        },
                        "71A": {
                            "value": "OUR"
                        },
                        "72": {
                            "lines": [
                                "/ACC/MANUAL PROCESSING REQUIRED/INS/SPECIAL HANDLING NEEDEDREQUIRES COMPLIANCE REVIEW"
                            ]
                        }
                    },
                    "message_type": "103",
                    "user_header": {
                        "unique_end_to_end_reference": "180f1e65-90e0-44d5-a49a-92b55eb3025f"
                    }
                }"#;

    // Step 1: Deserialize JSON into Swift MT Message
    println!("📥 Step 1: Deserializing JSON into Swift MT103 Message");
    let swift_message: SwiftMessage<MT103> = serde_json::from_str(json_content)?;
    println!("✅ Successfully deserialized JSON into MT103 message\n");

    // Step 2: Serialize to MT message format
    println!("📤 Step 3: Serializing to MT Message Format");
    let mt_message = swift_message.to_mt_message();
    println!("✅ Successfully serialized to MT format\n");

    println!("🗨️  Generated MT Message:");
    println!("{}", "=".repeat(60));
    println!("{mt_message}");
    println!("{}", "=".repeat(60));
    println!();

    // Step 3: Round-trip JSON serialization
    println!("🔄 Step 4: Round-trip JSON Serialization");
    let round_trip_json = serde_json::to_string_pretty(&swift_message)?;
    println!("✅ Successfully serialized back to JSON\n");

    // Step 4: Summary
    println!("📊 Summary:");
    println!("  • Original JSON: {} bytes", json_content.len());
    println!("  • MT Message: {} bytes", mt_message.len());
    println!("  • Round-trip JSON: {} bytes", round_trip_json.len());
    println!();

    println!("🎉 SUCCESS: Complete JSON ↔ MT Format conversion!");
    println!("   ✓ JSON deserialization works");
    println!("   ✓ MT message serialization works");
    println!("   ✓ Round-trip JSON serialization works");

    Ok(())
}
