use std::time::Instant;
use swift_mt_message::{ParsedSwiftMessage, parser::SwiftParser};

const ITERATIONS: usize = 100_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SWIFT MT Message Round-trip Benchmark");
    println!("=====================================");
    println!("Iterations: {ITERATIONS}");
    println!();

    // MT103 STP (Straight Through Processing) message
    let mt103_stp_message = r#"{1:F01BANKBEBBAXXX0000000000}
{2:I103BANKDEFFXXXXN}
{3:{113:SEPA}{121:180f1e65-90e0-44d5-a49a-92b55eb3025f}}
{4:
:20:STP123456
:23B:CRED
:23E:SDVA
:32A:250615EUR1000,00
:50K:/DE89370400440532013000
JOHN DOE
123 MAIN STREET
BERLIN
:52A:BANKDEFFXXX
:59:/GB82WEST12345698765432
JANE SMITH
456 HIGH STREET
LONDON
:71A:OUR
-}"#;

    println!("Starting benchmark...");
    let total_start = Instant::now();

    let mut successful_rounds = 0;
    let mut failed_rounds = 0;

    // Progress reporting
    for i in 0..ITERATIONS {
        if i % 10_000 == 0 && i > 0 {
            let elapsed = total_start.elapsed();
            let rate = i as f64 / elapsed.as_secs_f64();
            println!("  {i:>6} iterations completed ({rate:.0} ops/sec)");
        }

        // Full round-trip conversion
        match perform_round_trip(mt103_stp_message) {
            Ok(_) => successful_rounds += 1,
            Err(_) => failed_rounds += 1,
        }
    }

    let total_time = total_start.elapsed();

    // Results
    println!();
    println!("Benchmark Results");
    println!("=================");
    println!("Total iterations:    {ITERATIONS:>10}");
    println!("Successful rounds:   {successful_rounds:>10}");
    println!("Failed rounds:       {failed_rounds:>10}");
    println!(
        "Success rate:        {:>9.2}%",
        (successful_rounds as f64 / ITERATIONS as f64) * 100.0
    );
    println!();
    println!(
        "Total time:          {:>7.3} seconds",
        total_time.as_secs_f64()
    );
    println!(
        "Average per round:   {:>7.1} μs",
        total_time.as_micros() as f64 / ITERATIONS as f64
    );
    println!(
        "Throughput:          {:>7.0} rounds/sec",
        ITERATIONS as f64 / total_time.as_secs_f64()
    );
    println!();
    println!("Performance Summary:");
    println!(
        "  - Messages per second: {:>10.0}",
        ITERATIONS as f64 / total_time.as_secs_f64()
    );
    println!(
        "  - Messages per minute: {:>10.0}",
        (ITERATIONS as f64 / total_time.as_secs_f64()) * 60.0
    );
    println!(
        "  - Messages per hour:   {:>10.0}",
        (ITERATIONS as f64 / total_time.as_secs_f64()) * 3600.0
    );

    if failed_rounds > 0 {
        println!("\n⚠️  Warning: {failed_rounds} rounds failed");
        return Err(format!("Benchmark had {failed_rounds} failures").into());
    }

    println!("\n✅ Benchmark completed successfully!");
    Ok(())
}

fn perform_round_trip(mt_message: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Parse MT string to struct
    let parsed_message = SwiftParser::parse_auto(mt_message)?;

    // Step 2: Serialize struct to JSON
    let json_representation = serde_json::to_string_pretty(&parsed_message)?;

    // Step 3: Deserialize JSON back to struct
    let deserialized_message: ParsedSwiftMessage = serde_json::from_str(&json_representation)?;

    // Step 4: Convert struct back to MT format
    let regenerated_mt = match &deserialized_message {
        ParsedSwiftMessage::MT103(msg) => msg.to_mt_message(),
        _ => return Err("Expected MT103 message".into()),
    };

    // Step 5: Parse the regenerated MT to verify round-trip
    let _final_message = SwiftParser::parse_auto(&regenerated_mt)?;

    Ok(())
}
