use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

/// Field 13C: Time Indication
///
/// Time indication with code, time, and UTC offset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field13C {
    /// Time indication code with slashes
    #[component("/8c/", validate = ["time_indication_code"])]
    pub time_code: String,
    /// Time (HHMM)
    #[component("4!n", validate = ["time_format"])]
    pub time: String,
    /// UTC offset with sign
    #[component("1!x4!n", validate = ["utc_offset_format"])]
    pub utc_offset: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftField;

    #[test]
    fn test_field13c_parsing() {
        let test_input = ":13C:/RNCTIME/1405+0200";
        println!("Testing input: {test_input}");

        match Field13C::parse(test_input) {
            Ok(parsed) => {
                println!("Successfully parsed:");
                println!("  time_code: {}", parsed.time_code);
                println!("  time: {}", parsed.time);
                println!("  utc_offset: {}", parsed.utc_offset);

                // Test serialization back
                let serialized = parsed.to_swift_string();
                println!("Serialized back: {serialized}");

                // Verify the parsed values
                assert_eq!(parsed.time_code, "/RNCTIME/");
                assert_eq!(parsed.time, "1405");
                assert_eq!(parsed.utc_offset, "+0200");
            }
            Err(e) => {
                println!("Parsing failed: {e:?}");
                panic!("Failed to parse Field13C: {e:?}");
            }
        }
    }

    #[test]
    fn test_field13c_additional_cases() {
        // Test different time codes and offsets
        let test_cases = vec![
            (":13C:/CLSTIME/2359-0500", "/CLSTIME/", "2359", "-0500"),
            (":13C:/SNDTIME/0000+0000", "/SNDTIME/", "0000", "+0000"),
            (":13C:/ABC/1200+0100", "/ABC/", "1200", "+0100"),
        ];

        for (input, expected_code, expected_time, expected_offset) in test_cases {
            match Field13C::parse(input) {
                Ok(parsed) => {
                    assert_eq!(parsed.time_code, expected_code);
                    assert_eq!(parsed.time, expected_time);
                    assert_eq!(parsed.utc_offset, expected_offset);

                    // Test round-trip
                    let serialized = parsed.to_swift_string();
                    assert_eq!(serialized, input);
                }
                Err(e) => {
                    panic!("Failed to parse {input}: {e:?}");
                }
            }
        }
    }
}
