use swift_mt_message::{MTMessage, parse_message};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SWIFT MT Message Parser - Comprehensive Demo ===\n");

    // Payment Messages
    demo_mt103()?;
    demo_mt102()?;
    demo_mt202()?;

    // System Messages
    demo_mt192()?;
    demo_mt195()?;
    demo_mt196()?;
    demo_mt197()?;
    demo_mt199()?;

    // Statement Messages
    demo_mt940()?;
    demo_mt941()?;
    demo_mt942()?;

    Ok(())
}

fn demo_mt103() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT103: Single Customer Credit Transfer ===");

    let mt103_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I103BANKDEFFAXXXU3003}{4:
:20:MT103REF123456
:23B:CRED
:32A:210315EUR1000000,00
:50K:ORDERING CUSTOMER
COMPANY ABC
MAIN STREET 123
12345 CITY
:59:BENEFICIARY CUSTOMER
COMPANY XYZ
BUSINESS AVENUE 456
67890 TOWN
:70:INVOICE PAYMENT INV-2021-001
:71A:OUR
-}"#;

    let message = parse_message(mt103_message)?;

    if let MTMessage::MT103(mt103) = message {
        println!("Sender Reference: {}", mt103.sender_reference()?);
        println!("Bank Operation Code: {}", mt103.bank_operation_code()?);

        let amount = mt103.amount()?;
        println!("Amount: {} {}", amount.value, amount.currency);
        println!("Value Date: {}", mt103.value_date()?);

        println!("Ordering Customer: {}", mt103.ordering_customer()?);
        println!("Beneficiary: {}", mt103.beneficiary()?);

        if let Some(remittance) = mt103.remittance_information() {
            println!("Remittance Information: {}", remittance);
        }
    }

    println!();
    Ok(())
}

fn demo_mt102() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT102: Multiple Customer Credit Transfer ===");

    let mt102_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I102BANKDEFFAXXXU3003}{4:
:20:MT102REF123456
:23B:CRED
:26T:001
:32A:210315EUR5000000,00
:19:5000000,00
:77B:2
:50K:ORDERING BANK
MAIN STREET 123
CITY
:21:TXN001
:32B:EUR2500000,00
:59:BENEFICIARY 1
ADDRESS 1
:21:TXN002
:32B:EUR2500000,00
:59:BENEFICIARY 2
ADDRESS 2
:70:Multiple payments batch
-}"#;

    let message = parse_message(mt102_message)?;

    if let MTMessage::MT102(mt102) = message {
        println!("Sender Reference: {}", mt102.sender_reference()?);
        println!(
            "Number of Transactions: {:?}",
            mt102.number_of_transactions()
        );

        let amount = mt102.amount()?;
        println!("Total Amount: {} {}", amount.value, amount.currency);

        let beneficiaries = mt102.beneficiaries();
        println!("Number of Beneficiaries: {}", beneficiaries.len());
    }

    println!();
    Ok(())
}

fn demo_mt202() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT202: General Financial Institution Transfer ===");

    let mt202_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I202BANKDEFFAXXXU3003}{4:
:20:FI202123456789
:21:REL987654321
:32A:210315USD10000000,00
:52A:ORDERING BANK
NEW YORK
:58A:BENEFICIARY BANK
SINGAPORE
:70:INTERBANK TRANSFER
:71A:OUR
-}"#;

    let message = parse_message(mt202_message)?;

    if let MTMessage::MT202(mt202) = message {
        println!("Transaction Reference: {}", mt202.transaction_reference()?);

        let amount = mt202.amount()?;
        println!("Amount: {} {}", amount.value, amount.currency);

        println!(
            "Beneficiary Institution: {}",
            mt202.beneficiary_institution()?
        );

        if let Some(ordering) = mt202.ordering_institution() {
            println!("Ordering Institution: {}", ordering);
        }
    }

    println!();
    Ok(())
}

fn demo_mt192() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT192: Request for Cancellation ===");

    let mt192_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I192BANKDEFFAXXXU3003}{4:
:20:CANCEL123456
:21:ORIG987654321
:11S:103
:75:DUPLICATE PAYMENT
:72:URGENT CANCELLATION REQUIRED
-}"#;

    let message = parse_message(mt192_message)?;

    if let MTMessage::MT192(mt192) = message {
        println!("Transaction Reference: {}", mt192.transaction_reference()?);
        println!("Related Reference: {}", mt192.related_reference()?);

        if let Some(reason) = mt192.reason_for_cancellation() {
            println!("Reason for Cancellation: {}", reason);
        }

        if let Some(msg_type) = mt192.original_message_type() {
            println!("Original Message Type: MT{}", msg_type);
        }
    }

    println!();
    Ok(())
}

fn demo_mt195() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT195: Queries ===");

    let mt195_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I195BANKDEFFAXXXU3003}{4:
:20:QUERY123456
:21:ORIG987654321
:75:PAYMENT STATUS INQUIRY
:77A:PLEASE CONFIRM PAYMENT STATUS
-}"#;

    let message = parse_message(mt195_message)?;

    if let MTMessage::MT195(mt195) = message {
        println!("Transaction Reference: {}", mt195.transaction_reference()?);
        println!("Related Reference: {}", mt195.related_reference()?);

        if let Some(query_type) = mt195.query_type() {
            println!("Query Type: {}", query_type);
        }

        let enquiry_details = mt195.all_enquiry_details();
        if !enquiry_details.is_empty() {
            println!("Enquiry Details: {:?}", enquiry_details);
        }
    }

    println!();
    Ok(())
}

fn demo_mt196() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT196: Answers ===");

    let mt196_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I196BANKDEFFAXXXU3003}{4:
:20:ANSWER123456
:21:QUERY987654321
:75:PAYMENT STATUS RESPONSE
:77A:PAYMENT EXECUTED ON 2021-03-15
:76:COMPLETED
-}"#;

    let message = parse_message(mt196_message)?;

    if let MTMessage::MT196(mt196) = message {
        println!("Transaction Reference: {}", mt196.transaction_reference()?);
        println!("Related Reference: {}", mt196.related_reference()?);

        if let Some(answer_type) = mt196.answer_type() {
            println!("Answer Type: {}", answer_type);
        }

        let detailed_answers = mt196.all_detailed_answers();
        if !detailed_answers.is_empty() {
            println!("Detailed Answers: {:?}", detailed_answers);
        }

        let status_info = mt196.all_status_information();
        if !status_info.is_empty() {
            println!("Status Information: {:?}", status_info);
        }
    }

    println!();
    Ok(())
}

fn demo_mt197() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT197: Copy of a Message ===");

    let mt197_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I197BANKDEFFAXXXU3003}{4:
:20:COPY123456
:21:ORIG987654321
:11S:103
:75:REGULATORY COPY
:72:COPY FOR COMPLIANCE
:79:FULL COPY OF ORIGINAL MT103...
-}"#;

    let message = parse_message(mt197_message)?;

    if let MTMessage::MT197(mt197) = message {
        println!("Transaction Reference: {}", mt197.transaction_reference()?);
        println!("Related Reference: {}", mt197.related_reference()?);

        if let Some(reason) = mt197.copy_reason() {
            println!("Copy Reason: {}", reason);
        }

        if let Some(msg_type) = mt197.original_message_type() {
            println!("Original Message Type: MT{}", msg_type);
        }

        let narratives = mt197.narratives();
        if !narratives.is_empty() {
            println!("Narratives: {:?}", narratives);
        }
    }

    println!();
    Ok(())
}

fn demo_mt199() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT199: Free Format Message ===");

    let mt199_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I199BANKDEFFAXXXU3003}{4:
:20:FREE123456
:75:URGENT NOTIFICATION
:79:Dear Colleagues,
We would like to inform you of system maintenance.
:72:IMPORTANT NOTICE
-}"#;

    let message = parse_message(mt199_message)?;

    if let MTMessage::MT199(mt199) = message {
        println!("Transaction Reference: {}", mt199.transaction_reference()?);

        if let Some(subject) = mt199.message_subject() {
            println!("Message Subject: {}", subject);
        }

        let free_text = mt199.all_free_format_text();
        if !free_text.is_empty() {
            println!("Free Format Text: {:?}", free_text);
        }

        let narratives = mt199.narratives();
        if !narratives.is_empty() {
            println!("Narratives: {:?}", narratives);
        }
    }

    println!();
    Ok(())
}

fn demo_mt940() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT940: Customer Statement Message ===");

    let mt940_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I940BANKDEFFAXXXU3003}{4:
:20:STMT123456789
:25:12345678901234567890
:28C:123/1
:60F:C210315EUR1000000,00
:61:2103150315DR500,00NTRFNONREF//PAYMENT
:86:PAYMENT RECEIVED FROM CUSTOMER ABC
:61:2103160316CR1500,00NTRFNONREF//DEPOSIT
:86:CASH DEPOSIT AT BRANCH
:62F:C210316EUR2000000,00
:64:C210316EUR1950000,00
-}"#;

    let message = parse_message(mt940_message)?;

    if let MTMessage::MT940(mt940) = message {
        println!("Transaction Reference: {}", mt940.transaction_reference()?);
        println!(
            "Account Identification: {}",
            mt940.account_identification()?
        );
        println!("Statement Number: {}", mt940.statement_number()?);

        let (dc, date, currency, amount) = mt940.parse_opening_balance()?;
        println!("Opening Balance: {} {} {} {}", dc, date, currency, amount);

        let (dc, date, currency, amount) = mt940.parse_closing_balance()?;
        println!("Closing Balance: {} {} {} {}", dc, date, currency, amount);

        let lines = mt940.statement_lines();
        println!("Number of Statement Lines: {}", lines.len());

        let info = mt940.information_to_account_owner();
        println!("Information Lines: {}", info.len());
    }

    println!();
    Ok(())
}

fn demo_mt941() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT941: Balance Report Message ===");

    let mt941_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I941BANKDEFFAXXXU3003}{4:
:20:BAL123456789
:25:12345678901234567890
:28C:123/1
:60F:C210315EUR1000000,00
:62F:C210316EUR1050000,00
:64:C210316EUR1000000,00
:65:C210317EUR1000000,00
:65:C210318EUR1000000,00
-}"#;

    let message = parse_message(mt941_message)?;

    if let MTMessage::MT941(mt941) = message {
        println!("Transaction Reference: {}", mt941.transaction_reference()?);
        println!(
            "Account Identification: {}",
            mt941.account_identification()?
        );

        let summary = mt941.balance_summary()?;
        println!(
            "Opening Balance: {} {} {} {}",
            summary.opening_balance.0,
            summary.opening_balance.1,
            summary.opening_balance.2,
            summary.opening_balance.3
        );

        println!(
            "Closing Balance: {} {} {} {}",
            summary.closing_balance.0,
            summary.closing_balance.1,
            summary.closing_balance.2,
            summary.closing_balance.3
        );

        println!(
            "Forward Available Balances: {}",
            summary.forward_available_balances.len()
        );
    }

    println!();
    Ok(())
}

fn demo_mt942() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== MT942: Interim Transaction Report ===");

    let mt942_message = r#"{1:F01BANKDEFFAXXX0123456789}{2:I942BANKDEFFAXXXU3003}{4:
:20:INTERIM123456789
:25:12345678901234567890
:28C:123/1
:13D:2103151200+0100
:34F:EUR1000,00
:60F:C210315EUR1000000,00
:61:2103150315DR2500,00NTRFNONREF//PAYMENT
:86:LARGE PAYMENT ABOVE FLOOR LIMIT
:62F:C210316EUR1002500,00
-}"#;

    let message = parse_message(mt942_message)?;

    if let MTMessage::MT942(mt942) = message {
        println!("Transaction Reference: {}", mt942.transaction_reference()?);
        println!(
            "Account Identification: {}",
            mt942.account_identification()?
        );

        if let Some(date_time) = mt942.date_time_indication() {
            println!("Date/Time Indication: {}", date_time);
        }

        if let Some((currency, amount)) = mt942.parse_floor_limit().transpose()? {
            println!("Floor Limit: {} {}", currency, amount);
        }

        let summary = mt942.interim_summary()?;
        println!("Transaction Count: {}", summary.transaction_count);

        println!(
            "Opening Balance: {} {} {} {}",
            summary.opening_balance.0,
            summary.opening_balance.1,
            summary.opening_balance.2,
            summary.opening_balance.3
        );
    }

    println!();
    Ok(())
}
