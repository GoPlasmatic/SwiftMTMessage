//! Simple test of sample generation utility functions

use swift_mt_message::sample::*;

fn main() {
    println!("=== Testing Sample Generation Utilities ===\n");

    // Test basic generators
    println!("Basic Generators:");
    println!("-----------------");
    println!("Numeric (6 digits): {}", generate_numeric(6));
    println!("Alphabetic (4 chars): {}", generate_alphabetic(4));
    println!("Alphanumeric (10 chars): {}", generate_alphanumeric(10));
    println!("Any character (20 chars): {}", generate_any_character(20));
    println!("Decimal (10,2): {}", generate_decimal(10, 2));

    println!();

    // Test specialized generators
    println!("Specialized Generators:");
    println!("-----------------------");
    println!("Valid BIC: {}", generate_valid_bic());
    println!("Valid Currency: {}", generate_valid_currency());
    println!("Valid Country Code: {}", generate_valid_country_code());
    println!("Date YYMMDD: {}", generate_date_yymmdd());
    println!("Date YYYYMMDD: {}", generate_date_yyyymmdd());
    println!("Time HHMM: {}", generate_time_hhmm());
    println!("Account Number: {}", generate_account_number());
    println!("Reference: {}", generate_reference());
    println!("Transaction Code: {}", generate_transaction_code());
    println!("Bank Operation Code: {}", generate_bank_operation_code());
    println!("Details of Charges: {}", generate_details_of_charges());

    println!();

    // Test format specification parser
    println!("Format Specification Parser:");
    println!("----------------------------");
    println!("Format '3!a': {}", generate_by_format_spec("3!a"));
    println!("Format '6!n': {}", generate_by_format_spec("6!n"));
    println!("Format '16x': {}", generate_by_format_spec("16x"));
    println!("Format '15d': {}", generate_by_format_spec("15d"));
    println!("Format '4!c': {}", generate_by_format_spec("4!c"));

    println!();

    // Test name and address generation
    println!("Name and Address Generation:");
    println!("----------------------------");
    let name_address_2 = generate_name_and_address(2);
    println!("2 lines:");
    for (i, line) in name_address_2.iter().enumerate() {
        println!("  Line {}: {}", i + 1, line);
    }

    let name_address_4 = generate_name_and_address(4);
    println!("4 lines:");
    for (i, line) in name_address_4.iter().enumerate() {
        println!("  Line {}: {}", i + 1, line);
    }

    println!();

    // Test multiple generations to show randomness
    println!("Randomness Test (5 references):");
    println!("--------------------------------");
    for i in 1..=5 {
        println!("{}: {}", i, generate_reference());
    }

    println!("\n=== Utility Test Complete ===");
}
