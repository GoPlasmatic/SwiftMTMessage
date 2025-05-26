use swift_mt_message::{parse_message, MTMessage};
use swift_mt_message::messages::MTMessageType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example MT103 message
    let message_text = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:FT21234567890
:23B:CRED
:32A:210315EUR1234567,89
:50K:JOHN DOE
ACME CORP
123 MAIN ST
:59:JANE SMITH
XYZ COMPANY
456 OAK AVE
:70:Invoice payment for services
-}"#;

    println!("Parsing SWIFT MT message...");
    
    // Parse the message
    let message = parse_message(message_text)?;
    
    println!("Message type: {}", message.message_type());
    
    // Extract fields based on message type
    match message {
        MTMessage::MT103(mt103) => {
            println!("\n=== MT103: Single Customer Credit Transfer ===");
            
            // Required fields
            println!("Sender Reference (20): {}", mt103.sender_reference()?);
            println!("Bank Operation Code (23B): {}", mt103.bank_operation_code()?);
            println!("Value Date/Currency/Amount (32A): {}", mt103.value_date_currency_amount()?);
            
            // Parsed amount details
            let amount = mt103.amount()?;
            println!("  - Currency: {}", amount.currency);
            println!("  - Amount: {:.2}", amount.value);
            println!("  - Value Date: {}", mt103.value_date()?);
            
            println!("Ordering Customer (50K): {}", mt103.ordering_customer()?);
            println!("Beneficiary (59): {}", mt103.beneficiary()?);
            
            // Optional fields
            if let Some(remittance) = mt103.remittance_information() {
                println!("Remittance Information (70): {}", remittance);
            }
            
            // Access all fields
            println!("\n=== All Fields ===");
            for field in mt103.get_all_fields() {
                println!("Field {}: {}", field.tag().as_str(), field.value());
            }
        }
        _ => {
            println!("Parsed message type: {}", message.message_type());
            println!("This example focuses on MT103 messages.");
        }
    }
    
    Ok(())
} 