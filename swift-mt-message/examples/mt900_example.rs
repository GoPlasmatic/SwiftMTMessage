use chrono::NaiveDate;
use swift_mt_message::fields::*;
use swift_mt_message::messages::MT900;

fn main() {
    println!("=== MT900: Confirmation of Debit Examples ===\n");

    // Example 1: Basic MT900 confirmation
    println!("1. Basic MT900 Debit Confirmation:");
    let field_20 = Field20::new("TXN240315001234".to_string());
    let field_21 = Field21::new("ORIG240314567890".to_string());
    let field_25 = Field25::new("GB33BUKB20201555555555".to_string());
    let field_32a = Field32A::new(
        NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        "USD".to_string(),
        10000.00,
    );

    let mt900_basic = MT900::new(field_20, field_21, field_25, field_32a);

    println!(
        "   Transaction Reference: {}",
        mt900_basic.transaction_reference()
    );
    println!("   Related Reference: {}", mt900_basic.related_reference());
    println!("   Account: {}", mt900_basic.account_identification());
    println!(
        "   Amount: {} {:.2}",
        mt900_basic.currency_code(),
        mt900_basic.debit_amount()
    );
    println!(
        "   Value Date: {}",
        mt900_basic.value_date().format("%Y-%m-%d")
    );
    println!(
        "   Description: {}",
        mt900_basic.get_confirmation_description()
    );
    println!("   Valid: {}\n", mt900_basic.validate_structure());

    // Example 2: MT900 with precise timing and ordering institution
    println!("2. MT900 with Precise Timing and Ordering Institution:");
    let field_20 = Field20::new("TXN240315001235".to_string());
    let field_21 = Field21::new("ORIG240314567891".to_string());
    let field_25 = Field25::new("CH1234567890123456".to_string());
    let field_32a = Field32A::new(
        NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        "EUR".to_string(),
        5000.00,
    );

    let field_13d = Some(Field13D::new("240315", "1430", "+", "0100").unwrap());
    let field_52a = Some(GenericBicField::new(None, None, "DEUTDEFFXXX").unwrap());
    let field_72 = Some(Field72::new(vec!["/EXCH/SPOT/1.0850".to_string()]).unwrap());

    let mt900_complete = MT900::new_complete(
        field_20, field_21, field_25, field_32a, field_13d, field_52a, field_72,
    );

    println!(
        "   Transaction Reference: {}",
        mt900_complete.transaction_reference()
    );
    println!(
        "   Related Reference: {}",
        mt900_complete.related_reference()
    );
    println!("   Account: {}", mt900_complete.account_identification());
    println!(
        "   Amount: {} {:.2}",
        mt900_complete.currency_code(),
        mt900_complete.debit_amount()
    );
    println!(
        "   Value Date: {}",
        mt900_complete.value_date().format("%Y-%m-%d")
    );

    if let Some(datetime) = mt900_complete.get_formatted_datetime() {
        println!("   Precise Timing: {}", datetime);
    }

    if let Some(bic) = mt900_complete.ordering_institution_bic() {
        println!("   Ordering Institution: {}", bic);
    }

    println!(
        "   Has Exchange Rate Info: {}",
        mt900_complete.has_exchange_rate_info()
    );
    println!(
        "   Description: {}",
        mt900_complete.get_confirmation_description()
    );

    let summary = mt900_complete.get_optional_information_summary();
    if !summary.is_empty() {
        println!("   Optional Information:");
        for info in summary {
            println!("     - {}", info);
        }
    }
    println!();

    // Example 3: High-value transaction with ERI codes
    println!("3. High-Value MT900 with ERI Codes:");
    let field_20 = Field20::new("TXN240315001236".to_string());
    let field_21 = Field21::new("ORIG240314567892".to_string());
    let field_25 = Field25::new("US1234567890123456".to_string());
    let field_32a = Field32A::new(
        NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        "USD".to_string(),
        2500000.00, // High-value transaction
    );

    let field_13d = Some(Field13D::new("240315", "0930", "-", "0500").unwrap());
    let field_52a = Some(GenericBicField::new(None, None, "CHASUS33XXX").unwrap());
    let field_72 = Some(
        Field72::new(vec![
            "/ERI/BILATERAL001".to_string(),
            "/EXCH/FORWARD/1.0950".to_string(),
            "HIGH VALUE SETTLEMENT".to_string(),
        ])
        .unwrap(),
    );

    let mt900_high_value = MT900::new_complete(
        field_20, field_21, field_25, field_32a, field_13d, field_52a, field_72,
    );

    println!(
        "   Transaction Reference: {}",
        mt900_high_value.transaction_reference()
    );
    println!(
        "   Amount: {} {:.2}",
        mt900_high_value.currency_code(),
        mt900_high_value.debit_amount()
    );
    println!(
        "   High-Value (>$1M): {}",
        mt900_high_value.is_high_value_transaction(1000000.00)
    );
    println!("   Has ERI Codes: {}", mt900_high_value.has_eri_codes());
    println!(
        "   Has Exchange Rate Info: {}",
        mt900_high_value.has_exchange_rate_info()
    );

    let instructions = mt900_high_value.get_processing_instructions();
    if !instructions.is_empty() {
        println!("   Processing Instructions:");
        for instruction in instructions {
            println!("     - {}", instruction);
        }
    }
    println!();

    // Example 4: Different value date scenarios
    println!("4. Value Date Analysis:");

    // Same day transaction
    let today_field_32a = Field32A::new(
        chrono::Utc::now().naive_utc().date(),
        "GBP".to_string(),
        750.00,
    );
    let mt900_today = MT900::new(
        Field20::new("TXN240315001237".to_string()),
        Field21::new("ORIG240314567893".to_string()),
        Field25::new("GB29NWBK60161331926819".to_string()),
        today_field_32a,
    );

    println!("   Same-day debit: {}", mt900_today.is_same_day_debit());
    println!(
        "   Days since value date: {}",
        mt900_today.days_since_value_date()
    );

    // Future dated transaction
    let future_date = chrono::Utc::now().naive_utc().date() + chrono::Duration::days(5);
    let future_field_32a = Field32A::new(future_date, "JPY".to_string(), 150000.00);
    let mt900_future = MT900::new(
        Field20::new("TXN240315001238".to_string()),
        Field21::new("ORIG240314567894".to_string()),
        Field25::new("JP1234567890123456".to_string()),
        future_field_32a,
    );

    println!(
        "   Forward-dated debit: {}",
        mt900_future.is_forward_dated_debit()
    );
    println!(
        "   Days until value date: {}",
        -mt900_future.days_since_value_date()
    );
    println!();

    // Example 5: Currency-specific behavior
    println!("5. Currency-Specific Behavior:");

    // JPY (no decimal places)
    let jpy_field_32a = Field32A::new(
        NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        "JPY".to_string(),
        1000000.00,
    );
    let mt900_jpy = MT900::new(
        Field20::new("TXN240315001239".to_string()),
        Field21::new("ORIG240314567895".to_string()),
        Field25::new("JP1234567890123456".to_string()),
        jpy_field_32a,
    );

    println!(
        "   JPY Amount: {} {:.0}",
        mt900_jpy.currency_code(),
        mt900_jpy.debit_amount()
    );
    println!(
        "   Major Currency: {}",
        mt900_jpy.field_32a.is_major_currency()
    );
    println!(
        "   Decimal Places: {}",
        mt900_jpy.field_32a.decimal_places()
    );

    // BHD (3 decimal places)
    let bhd_field_32a = Field32A::new(
        NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        "BHD".to_string(),
        1000.000,
    );
    let mt900_bhd = MT900::new(
        Field20::new("TXN240315001240".to_string()),
        Field21::new("ORIG240314567896".to_string()),
        Field25::new("BH1234567890123456".to_string()),
        bhd_field_32a,
    );

    println!(
        "   BHD Amount: {} {:.3}",
        mt900_bhd.currency_code(),
        mt900_bhd.debit_amount()
    );
    println!(
        "   Decimal Places: {}",
        mt900_bhd.field_32a.decimal_places()
    );
    println!();

    println!("=== MT900 Examples Complete ===");
}
