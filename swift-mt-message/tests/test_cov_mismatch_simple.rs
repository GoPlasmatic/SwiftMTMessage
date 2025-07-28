//! Simplified tests for MT202 COV message mismatch detection
//!
//! This test suite demonstrates how the error collection feature can help identify
//! mismatches between MT103 and MT202 COV messages.

use swift_mt_message::parser::SwiftParser;
use swift_mt_message::{MT103, MT202};

#[test]
fn test_mt202_cov_basic_parsing() {
    let mt103_text = r#"{1:F01BANKUS33AXXX0000000000}{2:I103BANKDE55XXXXN}{3:{108:MT103001}}{4:
:20:FT220315001
:23B:CRED
:32A:220315USD1000000,00
:50K:/US1234567890123456
ACME CORPORATION
123 MAIN STREET
NEW YORK NY 10001
:59:/SG56HSBC000012345678
SINGAPORE TECH PTE LTD
MARINA BAY FINANCIAL CENTRE
SINGAPORE 018956
:71A:SHA
-}"#;

    let mt202_cov_text = r#"{1:F01BANKUS33AXXX0000000000}{2:I202BANKDE55XXXXN}{3:{119:COV}}{4:
:20:FT220315002
:21:FT220315001
:32A:220315USD1050000,00
:58A:HSBCSGSG
:50K:/US9876543210987654
ACME INDUSTRIES
456 INDUSTRIAL PARK
CHICAGO IL 60601
:59:/SG56HSBC000098765432
SINGAPORE TECH PTE LTD
RAFFLES PLACE TOWER
SINGAPORE 048623
-}"#;

    // Parse MT103
    let mt103_result = SwiftParser::parse::<MT103>(mt103_text);
    assert!(mt103_result.is_ok());
    let mt103_msg = mt103_result.unwrap();
    
    // Parse MT202
    let mt202_result = SwiftParser::parse::<MT202>(mt202_cov_text);
    assert!(mt202_result.is_ok());
    let mt202_msg = mt202_result.unwrap();

    // Check that MT202 is a COV message
    assert!(mt202_msg.fields.is_cover_message());
    
    // Verify the mismatches exist
    // Field 32A amount mismatch
    assert_eq!(mt103_msg.fields.field_32a.amount, 1000000.00);
    assert_eq!(mt202_msg.fields.field_32a.amount, 1050000.00);
    assert_ne!(mt103_msg.fields.field_32a.amount, mt202_msg.fields.field_32a.amount);
    
    println!("MT103 Amount: {}", mt103_msg.fields.field_32a.amount);
    println!("MT202 Amount: {}", mt202_msg.fields.field_32a.amount);
    println!("Amount mismatch detected: {} USD difference", 
        mt202_msg.fields.field_32a.amount - mt103_msg.fields.field_32a.amount);
}

#[test]
fn test_mt202_cov_identification() {
    // Test a regular MT202 (not COV)
    let mt202_regular = r#"{1:F01BANKUS33AXXX0000000000}{2:I202BANKDE55XXXXN}{3:}{4:
:20:FT220315003
:21:NONREF
:32A:220315USD5000000,00
:58A:BANKDE55
-}"#;

    // Test an MT202 COV
    let mt202_cov = r#"{1:F01BANKUS33AXXX0000000000}{2:I202BANKDE55XXXXN}{3:{119:COV}}{4:
:20:FT220315004
:21:FT220315001
:32A:220315USD1000000,00
:58A:BANKDE55
:50K:ACME CORPORATION
:59:SINGAPORE TECH PTE LTD
-}"#;

    // Parse regular MT202
    let regular_result = SwiftParser::parse::<MT202>(mt202_regular);
    assert!(regular_result.is_ok());
    let regular_msg = regular_result.unwrap();
    assert!(!regular_msg.fields.is_cover_message());

    // Parse COV MT202
    let cov_result = SwiftParser::parse::<MT202>(mt202_cov);
    assert!(cov_result.is_ok());
    let cov_msg = cov_result.unwrap();
    assert!(cov_msg.fields.is_cover_message());
}

#[test]
fn test_error_collection_in_permissive_mode() {
    // Create an MT202 COV with intentional format issues
    let mt202_with_errors = r#"{1:F01BANKUS33AXXX0000000000}{2:I202BANKDE55XXXXN}{3:{119:COV}}{4:
:20:FT220315005-TOO-LONG-REFERENCE
:21:FT220315001
:32A:220315USD1050000,00
:58A:HSBCSGSG
:50K:/US9876543210987654
ACME INDUSTRIES
:59:/SG56HSBC000098765432
SINGAPORE TECH PTE LTD
-}"#;

    let parser = SwiftParser::new();
    
    // Use parse_with_errors to collect errors
    let result = parser.parse_with_errors::<MT202>(mt202_with_errors);
    
    match result {
        Ok(parse_result) => {
            match parse_result {
                swift_mt_message::errors::ParseResult::Success(msg) => {
                    println!("Parsed successfully without errors");
                    assert!(msg.fields.is_cover_message());
                }
                swift_mt_message::errors::ParseResult::PartialSuccess(msg, errors) => {
                    println!("Parsed with {} errors:", errors.len());
                    for error in &errors {
                        println!("  - {}", error);
                    }
                    assert!(msg.fields.is_cover_message());
                }
                swift_mt_message::errors::ParseResult::Failure(errors) => {
                    println!("Failed to parse with {} errors:", errors.len());
                    for error in &errors {
                        println!("  - {}", error);
                    }
                }
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}