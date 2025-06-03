//! CBPR+ (Cross-Border Payments & Reporting Plus) Demo
//!
//! This example demonstrates all CBPR+ message types used in correspondent banking:
//! - MT103: Single Customer Credit Transfer
//! - MT202: General Financial Institution Transfer  
//! - MT202COV: General Financial Institution Transfer (Cover)
//! - MT210: Notice to Receive
//! - MT192: Request for Cancellation
//! - MT196: Answers

use swift_mt_message::{MTMessage, parse_message};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CBPR+ (Cross-Border Payments & Reporting Plus) Demo ===\n");

    // 1. MT103 - Single Customer Credit Transfer
    demo_mt103()?;

    // 2. MT202 - General Financial Institution Transfer
    demo_mt202()?;

    // 3. MT202COV - General Financial Institution Transfer (Cover)
    demo_mt202cov()?;

    // 4. MT210 - Notice to Receive
    demo_mt210()?;

    // 5. MT192 - Request for Cancellation
    demo_mt192()?;

    // 6. MT196 - Answers
    demo_mt196()?;

    println!("\n=== CBPR+ Demo Complete ===");
    println!("All 6 CBPR+ message types successfully parsed and processed!");

    Ok(())
}

fn demo_mt103() -> Result<(), Box<dyn std::error::Error>> {
    println!("📄 MT103 - Single Customer Credit Transfer");
    println!("---------------------------------------------");

    let mt103_message = r#"{1:F01BANKUSNYAXXX0123456789}{2:I103BANKGB2LXXXU3003}{4:
:20:FT2021001234567
:23B:CRED
:32A:210315USD1000000,00
:50K:ORDERING CUSTOMER INC
123 BUSINESS STREET
NEW YORK NY 10001 US
:52A:BANKUSNYAXXX
:59:BENEFICIARY COMPANY LTD
456 FINANCE AVENUE
LONDON EC2V 8RF GB
:70:TRADE FINANCE PAYMENT
INVOICE INV-2021-001234
:71A:OUR
-}"#;

    let message = parse_message(mt103_message)?;

    if let MTMessage::MT103(mt103) = message {
        println!("✅ Message Type: MT{}", "103");
        println!("📋 Transaction Reference: {}", mt103.sender_reference()?);
        println!(
            "💰 Amount: {} {}",
            mt103.amount()?.value,
            mt103.amount()?.currency
        );
        println!("📅 Value Date: {}", mt103.value_date()?);
        println!(
            "👤 Ordering Customer: {}",
            mt103.ordering_customer()?.lines().next().unwrap_or("")
        );
        println!(
            "🏦 Ordering Institution: {}",
            mt103.ordering_institution().unwrap_or("N/A".to_string())
        );
        println!(
            "👥 Beneficiary: {}",
            mt103.beneficiary()?.lines().next().unwrap_or("")
        );
        if let Some(remittance) = mt103.remittance_information() {
            println!(
                "📝 Remittance Info: {}",
                remittance.lines().next().unwrap_or("")
            );
        }
        println!(
            "💳 Charge Details: {}",
            mt103.details_of_charges().unwrap_or("N/A".to_string())
        );
    }

    println!();
    Ok(())
}

fn demo_mt202() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏛️ MT202 - General Financial Institution Transfer");
    println!("--------------------------------------------------");

    let mt202_message = r#"{1:F01BANKUSNYAXXX0123456789}{2:I202BANKGB2LXXXU3003}{4:
:20:FT2021001234568
:21:RELATED123456789
:32A:210315USD5000000,00
:52A:BANKUSNYAXXX
:53A:CORRUSNYAXXX
:54A:CORRGB2LXXX
:57A:ACWITHGB2LXXX
:58A:BANKGB2LXXX
:70:INSTITUTIONAL TRANSFER
:72:CORRESPONDENT BANKING
-}"#;

    let message = parse_message(mt202_message)?;

    if let MTMessage::MT202(mt202) = message {
        println!("✅ Message Type: MT{}", "202");
        println!(
            "📋 Transaction Reference: {}",
            mt202.transaction_reference()?
        );
        println!(
            "🔗 Related Reference: {}",
            mt202.related_reference().unwrap_or("N/A".to_string())
        );
        println!(
            "💰 Amount: {} {}",
            mt202.amount()?.value,
            mt202.amount()?.currency
        );
        println!("📅 Value Date: {}", mt202.value_date()?);
        println!(
            "🏦 Ordering Institution: {}",
            mt202.ordering_institution().unwrap_or("N/A".to_string())
        );
        println!(
            "🤝 Sender's Correspondent: {}",
            mt202.senders_correspondent().unwrap_or("N/A".to_string())
        );
        println!(
            "🤝 Receiver's Correspondent: {}",
            mt202.receivers_correspondent().unwrap_or("N/A".to_string())
        );
        println!(
            "🏛️ Account With Institution: {}",
            mt202
                .account_with_institution()
                .unwrap_or("N/A".to_string())
        );
        println!(
            "🎯 Beneficiary Institution: {}",
            mt202.beneficiary_institution()?
        );
        if let Some(remittance) = mt202.remittance_information() {
            println!("📝 Remittance Info: {}", remittance);
        }
    }

    println!();
    Ok(())
}

fn demo_mt202cov() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ MT202COV - General Financial Institution Transfer (Cover)");
    println!("-------------------------------------------------------------");

    let mt202cov_message = r#"{1:F01BANKUSNYAXXX0123456789}{2:I202BANKGB2LXXXU3003}{4:
:20:COV2021001234569
:21:UNDERLYING123456
:32A:210315EUR2500000,00
:50K:ORDERING CUSTOMER CORP
789 CORPORATE BLVD
FRANKFURT 60311 DE
:52A:BANKDEFFAXXX
:53A:CORRDEFFAXXX
:54A:CORRGB2LXXX
:57A:ACWITHGB2LXXX
:58A:BANKGB2LXXX
:59:BENEFICIARY ENTERPRISES
321 BUSINESS PARK
MANCHESTER M1 2WD GB
:70:COVER FOR UNDERLYING MT103
TRADE SETTLEMENT
:71A:OUR
:72:COVER MESSAGE FOR CUSTOMER TRANSFER
-}"#;

    let message = parse_message(mt202cov_message)?;

    if let MTMessage::MT202COV(mt202cov) = message {
        println!("✅ Message Type: MT202COV");
        println!("📋 Cover Reference: {}", mt202cov.transaction_reference()?);
        println!(
            "🔗 Underlying Reference: {}",
            mt202cov.related_reference().unwrap_or("N/A".to_string())
        );
        println!(
            "💰 Cover Amount: {} {}",
            mt202cov.amount()?.value,
            mt202cov.amount()?.currency
        );
        println!("📅 Value Date: {}", mt202cov.value_date()?);

        // Cover-specific customer details
        println!(
            "👤 Ordering Customer: {}",
            mt202cov.ordering_customer()?.lines().next().unwrap_or("")
        );
        println!(
            "👥 Beneficiary Customer: {}",
            mt202cov
                .beneficiary_customer()?
                .lines()
                .next()
                .unwrap_or("")
        );

        // Institutional chain
        println!(
            "🏦 Ordering Institution: {}",
            mt202cov.ordering_institution().unwrap_or("N/A".to_string())
        );
        println!(
            "🤝 Sender's Correspondent: {}",
            mt202cov
                .senders_correspondent()
                .unwrap_or("N/A".to_string())
        );
        println!(
            "🤝 Receiver's Correspondent: {}",
            mt202cov
                .receivers_correspondent()
                .unwrap_or("N/A".to_string())
        );
        println!(
            "🏛️ Account With Institution: {}",
            mt202cov
                .account_with_institution()
                .unwrap_or("N/A".to_string())
        );
        println!(
            "🎯 Beneficiary Institution: {}",
            mt202cov.beneficiary_institution()?
        );

        println!("🛡️ Is Cover Message: {}", mt202cov.is_cover_message());

        if let Some(remittance) = mt202cov.remittance_information() {
            println!(
                "📝 Remittance Info: {}",
                remittance.lines().next().unwrap_or("")
            );
        }
        if let Some(instructions) = mt202cov.instructions() {
            println!("📋 Instructions: {}", instructions);
        }
    }

    println!();
    Ok(())
}

fn demo_mt210() -> Result<(), Box<dyn std::error::Error>> {
    println!("📢 MT210 - Notice to Receive");
    println!("------------------------------");

    let mt210_message = r#"{1:F01BANKGB2LXXX0123456789}{2:I210BANKUSNYXXXU3003}{4:
:20:NTR2021001234570
:21:INCOMING123456789
:25:BENEFACCT987654321
:32A:210316GBP750000,00
:50K:INTERNATIONAL SENDER LTD
555 GLOBAL STREET
LONDON EC1A 1BB GB
:52A:BANKGB2LXXX
:53A:CORRGB2LXXX
:54A:CORRUSNYAXXX
:57A:ACWITHUSNYXXX
:58A:BANKUSNYAXXX
:59:RECEIVING COMPANY INC
999 RECEIVER AVENUE
NEW YORK NY 10005 US
:70:INCOMING WIRE TRANSFER
EXPECTED SETTLEMENT
:71A:SHA
:72:NOTICE TO RECEIVE
PLEASE EXPECT INCOMING FUNDS
:77A:PRE-NOTIFICATION
FUNDS ARRIVING NEXT BUSINESS DAY
-}"#;

    let message = parse_message(mt210_message)?;

    if let MTMessage::MT210(mt210) = message {
        println!("✅ Message Type: MT210");
        println!("📋 Notice Reference: {}", mt210.transaction_reference()?);
        println!(
            "🔗 Related Reference: {}",
            mt210.related_reference().unwrap_or("N/A".to_string())
        );
        println!(
            "🏦 Account ID: {}",
            mt210.account_identification().unwrap_or("N/A".to_string())
        );

        // Expected incoming transfer details
        println!(
            "💰 Expected Amount: {} {}",
            mt210.expected_incoming_amount()?.value,
            mt210.expected_incoming_amount()?.currency
        );
        println!("📅 Expected Value Date: {}", mt210.expected_value_date()?);

        // Customer details (optional in MT210)
        if let Some(ordering_customer) = mt210.ordering_customer() {
            println!(
                "👤 Ordering Customer: {}",
                ordering_customer.lines().next().unwrap_or("")
            );
        }
        if let Some(beneficiary_customer) = mt210.beneficiary_customer() {
            println!(
                "👥 Beneficiary Customer: {}",
                beneficiary_customer.lines().next().unwrap_or("")
            );
        }

        // Institutional details
        println!(
            "🏦 Ordering Institution: {}",
            mt210.ordering_institution().unwrap_or("N/A".to_string())
        );
        println!(
            "🎯 Beneficiary Institution: {}",
            mt210.beneficiary_institution()?
        );

        println!("📢 Is Pre-notification: {}", mt210.is_pre_notification());

        if let Some(remittance) = mt210.remittance_information() {
            println!(
                "📝 Remittance Info: {}",
                remittance.lines().next().unwrap_or("")
            );
        }

        // Notification details
        let notifications = mt210.all_notification_details();
        for (i, notification) in notifications.iter().enumerate() {
            println!(
                "🔔 Notification {}: {}",
                i + 1,
                notification.lines().next().unwrap_or("")
            );
        }
    }

    println!();
    Ok(())
}

fn demo_mt192() -> Result<(), Box<dyn std::error::Error>> {
    println!("❌ MT192 - Request for Cancellation");
    println!("------------------------------------");

    let mt192_message = r#"{1:F01BANKUSNYAXXX0123456789}{2:I192BANKGB2LXXXU3003}{4:
:20:CANCEL2021001234571
:21:FT2021001234567
:11S:103
:75:DUPLICATE PAYMENT
CUSTOMER REQUESTED CANCELLATION
:52A:BANKUSNYAXXX
:58A:BANKGB2LXXX
:72:URGENT CANCELLATION REQUEST
PLEASE CONFIRM RECEIPT
:79:COPY OF ORIGINAL MT103
{1:F01BANKUSNYAXXX0123456789}{2:I103BANKGB2LXXXU3003}
{4::20:FT2021001234567...}
-}"#;

    let message = parse_message(mt192_message)?;

    if let MTMessage::MT192(mt192) = message {
        println!("✅ Message Type: MT192");
        println!(
            "📋 Cancellation Reference: {}",
            mt192.transaction_reference()?
        );
        println!("🔗 Original Reference: {}", mt192.related_reference()?);
        println!(
            "📄 Original Message Type: MT{}",
            mt192.original_message_type().unwrap_or("N/A".to_string())
        );

        if let Some(reason) = mt192.reason_for_cancellation() {
            println!(
                "❌ Cancellation Reason: {}",
                reason.lines().next().unwrap_or("")
            );
        }

        println!(
            "🏦 Requesting Institution: {}",
            mt192.requesting_institution().unwrap_or("N/A".to_string())
        );
        println!(
            "🎯 Receiving Institution: {}",
            mt192.receiving_institution().unwrap_or("N/A".to_string())
        );

        // Narrative information
        let narratives = mt192.narratives();
        for (i, narrative) in narratives.iter().enumerate() {
            println!(
                "📝 Narrative {}: {}",
                i + 1,
                narrative.lines().next().unwrap_or("")
            );
        }

        if let Some(copy) = mt192.copy_of_original_message() {
            println!(
                "📄 Copy of Original: {}...",
                copy.chars().take(50).collect::<String>()
            );
        }
    }

    println!();
    Ok(())
}

fn demo_mt196() -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ MT196 - Answers");
    println!("-------------------");

    let mt196_message = r#"{1:F01BANKGB2LXXX0123456789}{2:I196BANKUSNYXXXU3003}{4:
:20:ANS2021001234572
:21:CANCEL2021001234571
:11S:192
:75:CANCELLATION PROCESSED
ORIGINAL PAYMENT CANCELLED
:11A:CONF
:52A:BANKGB2LXXX
:58A:BANKUSNYAXXX
:72:CANCELLATION CONFIRMED
FUNDS RETURNED TO ORIGINATOR
:76:CANCELLATION SUCCESSFUL
REFERENCE FT2021001234567 CANCELLED
AMOUNT USD 1,000,000.00 RETURNED
STATUS: COMPLETED
-}"#;

    let message = parse_message(mt196_message)?;

    if let MTMessage::MT196(mt196) = message {
        println!("✅ Message Type: MT196");
        println!("📋 Answer Reference: {}", mt196.transaction_reference()?);
        println!("🔗 Related Reference: {}", mt196.related_reference()?);
        println!(
            "📄 Original Query Type: MT{}",
            mt196
                .original_query_message_type()
                .unwrap_or("N/A".to_string())
        );

        if let Some(answer_type) = mt196.answer_type() {
            println!(
                "💬 Answer Type: {}",
                answer_type.lines().next().unwrap_or("")
            );
        }

        if let Some(confirmation) = mt196.confirmation_code() {
            println!("✅ Confirmation: {}", confirmation);
        }

        println!(
            "🏦 Answering Institution: {}",
            mt196.answering_institution().unwrap_or("N/A".to_string())
        );
        println!(
            "🎯 Querying Institution: {}",
            mt196.querying_institution().unwrap_or("N/A".to_string())
        );

        // All detailed answers
        let detailed_answers = mt196.all_detailed_answers();
        for (i, answer) in detailed_answers.iter().enumerate() {
            println!(
                "📋 Answer Detail {}: {}",
                i + 1,
                answer.lines().next().unwrap_or("")
            );
        }

        // Status information
        let status_info = mt196.all_status_information();
        for (i, status) in status_info.iter().enumerate() {
            println!(
                "📊 Status {}: {}",
                i + 1,
                status.lines().next().unwrap_or("")
            );
        }
    }

    println!();
    Ok(())
}
