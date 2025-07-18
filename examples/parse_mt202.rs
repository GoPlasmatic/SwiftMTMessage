use swift_mt_message::messages::mt202::MT202;
use swift_mt_message::SwiftParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Standard MT202 - Institutional Transfer
    let raw_mt202_standard = r#"{1:F01BANKBEBBXXX0000000000}
{2:I202BANKDEFFXXXN01}
{3:{108:MT202}{121:550e8400-e29b-41d4-a716-446655440000}}
{4:
:20:FI2024123456789
:21:REL2024987654321
:32A:241215USD2500000,00
:52A:BANKBEBBXXX
:53A:BNPAFRPPXXX
:56A:DEUTDEFFXXX
:57A:CHASUS33XXX
:58A:CHASUS33XXX
:72:/ACC/PRIORITY PROCESSING
/INS/BNPAFRPPXXX
INTERBANK SETTLEMENT
HIGH VALUE PAYMENT
-}"#;

    // Example 2: MT202 COV - Cover Payment with Sequence B fields
    let raw_mt202_cov = r#"{1:F01BANKBEBBXXX0000000000}
{2:I202BANKDEFFXXXN01}
{3:{108:MT202}{121:550e8400-e29b-41d4-a716-446655440000}}
{4:
:20:FI2024123456789
:21:REL2024987654321
:32A:241215USD2500000,00
:52A:BANKBEBBXXX
:53A:BNPAFRPPXXX
:56A:DEUTDEFFXXX
:57A:CHASUS33XXX
:58A:CHASUS33XXX
:72:/ACC/PRIORITY PROCESSING
/INS/BNPAFRPPXXX
INTERBANK SETTLEMENT
HIGH VALUE PAYMENT
:50K:ACME CORPORATION
123 MAIN STREET
NEW YORK NY 10001
:52A:CITIUS33XXX
:56A:DEUTFRPPXXX
:57A:FHBLFRPPXXX
:59:1234567890
BENEFICIARY CORP
456 BUSINESS AVE
PARIS FRANCE
:70:/INV/2024-001
/REF/PO-789456
INVOICE PAYMENT
:72:/BNF/ADDITIONAL INFO
URGENT PAYMENT
-}"#;

    println!("🏦 MT202 Parser - Focus on JSON Conversion");
    println!("{}", "=".repeat(60));

    // Parse and convert Example 1: Standard MT202
    println!("\n📊 Example 1: Standard MT202 (No COV)");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse::<MT202>(raw_mt202_standard) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed standard MT202 message!");

            // Convert to JSON and display
            convert_to_json(&parsed_message, "Standard MT202")?;
        }
        Err(e) => {
            println!("❌ Failed to parse standard MT202: {e:?}");
        }
    }

    // Parse and convert Example 2: COV MT202 with duplicate fields
    println!("\n📊 Example 2: MT202 COV with Sequence B (Duplicate Fields)");
    println!("{}", "-".repeat(50));

    match SwiftParser::parse::<MT202>(raw_mt202_cov) {
        Ok(parsed_message) => {
            println!("✅ Successfully parsed COV MT202 message!");

            // Convert to JSON and display
            convert_to_json(&parsed_message, "COV MT202")?;

            // Verify sequential field consumption worked correctly
            println!("\n🔍 Field Consumption Verification:");
            println!(
                "General field 52 (should be BANKBEBBXXX): {:?}",
                parsed_message.fields.field_52
            );
            println!(
                "Sequence B field 52 (should be CITIUS33XXX): {:?}",
                parsed_message.fields.field_52_seq_b
            );
        }
        Err(e) => {
            println!("❌ Failed to parse COV MT202: {e:?}");
        }
    }

    Ok(())
}

fn convert_to_json(
    parsed_message: &swift_mt_message::SwiftMessage<MT202>,
    title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Converting {title} to JSON:");

    // Convert complete message to JSON
    let full_json = serde_json::to_string_pretty(parsed_message)?;
    println!("\n📄 Complete Message JSON:");
    println!("{full_json}");

    Ok(())
}
