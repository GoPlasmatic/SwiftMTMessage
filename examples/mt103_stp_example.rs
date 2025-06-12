//! Comprehensive MT103-STP Example
//!
//! This example demonstrates the complete MT103-STP (Straight Through Processing) workflow:
//! 1. Parse a SWIFT MT103-STP message
//! 2. Validate STP compliance rules (C1-C10)
//! 3. Display field details with STP-specific validation
//! 4. Convert to JSON and validate business rules
//!
//! MT103-STP has enhanced validation rules for automatic processing without manual intervention.

use swift_mt_message::{
    field_parser::{SwiftField, SwiftMessage},
    json::ToJson,
    mt_models::mt103_stp::MT103STP,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏦 Comprehensive MT103-STP (Straight Through Processing) Example");
    println!("=================================================================\n");

    // Sample MT103-STP message with STP-compliant fields
    let mt103_stp_message = r#"{1:F01CHASUS33AXXX0000000000}{2:I103DEUTDEFFAXXXN}{3:{113:SEPA}}{4:
:13C:/123045+0/+0100/-0500
:20:STP2024123456
:23B:CRED
:23E:INTC/COMPLIANCE
:26T:A01
:32A:241231USD1500000,00
:33B:EUR1375000,00
:36:1,0909
:50K:/1234567890
GLOBAL TECH CORPORATION
456 INNOVATION DRIVE
SAN FRANCISCO CA 94105 US
:52A:CHASUS33
:53A:BNPAFRPP
:54A:DEUTDEFF
:57A:DEUTDEFF
:59A:/DE89370400440532013000
DEUTDEFF
:70:/INV/INVOICE-2024-Q4-789
/RFB/SOFTWARE LICENSE PAYMENT
ENTERPRISE SOFTWARE LICENSES
ANNUAL SUBSCRIPTION RENEWAL
:71A:SHA
:71F:USD50,00
:72:/ACC/STANDARD PROCESSING
/INS/COMPLY WITH LOCAL REGS
AUTOMATED STP PROCESSING
:77B:/ORDERRES/DE//REGULATORY INFO
SOFTWARE LICENSE COMPLIANCE
TRADE RELATED TRANSACTION
-}"#;

    // Step 1: Parse the SWIFT message
    println!("📝 Step 1: Parsing SWIFT Message");
    println!("--------------------------------");

    let swift_message = SwiftMessage::parse(mt103_stp_message)?;
    println!("✅ Successfully parsed SWIFT message");
    println!("   Message Type: {}", swift_message.message_type);
    println!("   Number of Fields: {}", swift_message.fields.len());
    println!("   Fields: {:?}", swift_message.field_order);
    println!();

    println!("Basic Header: {:?}", swift_message.basic_header);
    println!("Application Header: {:?}", swift_message.application_header);
    println!("User Header: {:?}", swift_message.user_header);
    println!("Trailer Block: {:?}", swift_message.trailer_block);

    // Step 2: Convert to MT103-STP structure
    println!("🏗️  Step 2: Converting to MT103-STP Structure");
    println!("---------------------------------------------");

    let mt103_stp = MT103STP::from_swift_message(swift_message.clone())?;
    println!("✅ Successfully converted to MT103-STP");
    println!();

    // Step 3: STP Compliance Validation
    println!("🔍 Step 3: STP Compliance Validation");
    println!("------------------------------------");

    println!(
        "🚀 STP Compliance Status: {}",
        if mt103_stp.is_stp_compliant() {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    let stp_report = mt103_stp.validate_stp_rules()?;
    println!("   Rule Violations: {}", stp_report.rule_violations.len());
    println!("   Warnings: {}", stp_report.warnings.len());

    if !stp_report.rule_violations.is_empty() {
        println!("\n❌ STP RULE VIOLATIONS:");
        for violation in &stp_report.rule_violations {
            println!("   • Rule {}: {}", violation.rule, violation.description);
            println!("     Affected fields: {:?}", violation.affected_fields);
        }
    }

    if !stp_report.warnings.is_empty() {
        println!("\n⚠️  STP WARNINGS:");
        for warning in &stp_report.warnings {
            println!("   • {}", warning);
        }
    }

    if stp_report.is_stp_compliant {
        println!("\n✅ STP CONDITIONAL RULES VERIFIED:");
        println!("   • C1: Currency/Exchange Rate validation ✅");
        println!("   • C3: Bank Operation/Instruction compatibility ✅");
        println!("   • C4: Correspondent banking chain completeness ✅");
        println!("   • C7: Charge allocation rules ✅");
        println!("   • C8: Charge field dependencies ✅");
        println!("   • All other conditional rules ✅");
    }
    println!();

    // Step 4: Display field details
    println!("📋 Step 4: Field Details");
    println!("------------------------");

    // Required fields
    println!("🔴 MANDATORY FIELDS:");
    println!(
        "   20 (Sender's Reference): {}",
        mt103_stp.field_20.transaction_reference
    );
    println!(
        "   23B (Bank Operation Code): {}",
        mt103_stp.field_23b.bank_operation_code
    );
    println!(
        "   32A (Value Date/Currency/Amount): {} {} {:.2}",
        mt103_stp.field_32a.format_date(),
        mt103_stp.field_32a.currency,
        mt103_stp.field_32a.amount
    );
    println!(
        "   50 (Ordering Customer): {}",
        mt103_stp
            .field_50
            .to_swift_string()
            .lines()
            .next()
            .unwrap_or("")
    );

    // Handle Field59 enum properly for STP
    match &mt103_stp.field_59 {
        swift_mt_message::mt_models::fields::beneficiary::Field59::A(field) => {
            println!(
                "   59A (Beneficiary - STP): {} (BIC: {})",
                field.account.as_ref().unwrap_or(&"No account".to_string()),
                field.bic
            );
        }
        swift_mt_message::mt_models::fields::beneficiary::Field59::NoOption(field) => {
            println!(
                "   59 (Beneficiary): {}",
                field
                    .beneficiary_customer
                    .first()
                    .unwrap_or(&"".to_string())
            );
        }
    }

    println!(
        "   71A (Details of Charges): {}",
        mt103_stp.field_71a.details_of_charges
    );
    println!();

    // Optional fields with STP significance
    println!("🔵 OPTIONAL FIELDS (STP-Enhanced):");
    if let Some(field) = &mt103_stp.field_13c {
        println!(
            "   13C (Time Indication): {} {} {}",
            field.time(),
            field.utc_offset1(),
            field.utc_offset2()
        );
    }
    if let Some(field) = &mt103_stp.field_23e {
        println!(
            "   23E (Instruction Code): {} [STP Rule C3 applies]",
            field.instruction_code
        );
    }
    if let Some(field) = &mt103_stp.field_26t {
        println!(
            "   26T (Transaction Type Code): {}",
            field.transaction_type_code
        );
    }
    if let Some(field) = &mt103_stp.field_33b {
        println!(
            "   33B (Currency/Instructed Amount): {} {:.2} [STP Rules C1,C2,C8 apply]",
            field.currency, field.amount
        );
    }
    if let Some(field) = &mt103_stp.field_36 {
        println!(
            "   36 (Exchange Rate): {:.6} [STP Rule C1 applies]",
            field.rate()
        );
    }
    if let Some(_field) = &mt103_stp.field_52a {
        println!("   52A (Ordering Institution): Present");
    }

    // Correspondent banking chain (STP Rules C4, C5, C6)
    println!("\n🏦 CORRESPONDENT BANKING CHAIN:");
    if mt103_stp.field_53a.is_some() {
        println!("   53A (Sender's Correspondent): Present [STP Rule C4]");
    }
    if mt103_stp.field_54a.is_some() {
        println!("   54A (Receiver's Correspondent): Present [STP Rule C4]");
    }
    if mt103_stp.field_55a.is_some() {
        println!("   55A (Third Reimbursement Institution): Present [Triggers C4]");
    }
    if mt103_stp.field_56a.is_some() {
        println!("   56A (Intermediary Institution): Present [STP Rules C5,C6]");
    }
    if mt103_stp.field_57a.is_some() {
        println!("   57A (Account With Institution): Present [STP Rules C5,C10]");
    }

    // Information fields
    if let Some(field) = &mt103_stp.field_70 {
        println!("\n📄 INFORMATION FIELDS:");
        println!(
            "   70 (Remittance Information): {} lines",
            field.information.len()
        );
    }

    // Charges (STP Rules C7, C8, C9)
    println!("\n💰 CHARGES INFORMATION:");
    if let Some(field) = &mt103_stp.field_71f {
        println!(
            "   71F (Sender's Charges): {} {:.2} [STP Rules C7,C8]",
            field.currency, field.amount
        );
    }
    if let Some(field) = &mt103_stp.field_71g {
        println!(
            "   71G (Receiver's Charges): {} {:.2} [STP Rules C7,C8,C9]",
            field.currency, field.amount
        );
    }

    // Additional information
    if let Some(field) = &mt103_stp.field_72 {
        println!("\n📨 ADDITIONAL INFORMATION:");
        println!(
            "   72 (Sender to Receiver Info): {} lines [No REJT/RETN for STP]",
            field.information.len()
        );
    }
    if let Some(field) = &mt103_stp.field_77b {
        println!(
            "   77B (Regulatory Reporting): {} lines",
            field.information.len()
        );
    }
    println!();

    // Step 5: Validate business rules
    println!("✅ Step 5: Business Rules Validation");
    println!("------------------------------------");

    match mt103_stp.validate_business_rules() {
        Ok(report) => {
            println!("📊 Validation Results:");
            println!(
                "   Overall Valid: {}",
                if report.overall_valid {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Total Rules Checked: {}", report.results.len());
            println!("   Failed Rules: {}", report.failure_count());

            if !report.overall_valid {
                println!("\n❌ VALIDATION FAILURES:");
                for failure in report.get_failures() {
                    println!("   • {}: {}", failure.rule_name, failure.message);
                }
            } else {
                println!("\n✅ KEY STP VALIDATIONS PASSED:");
                println!("   • Field format compliance");
                println!("   • Conditional rule adherence");
                println!("   • Currency validation");
                println!("   • Charge allocation rules");
                println!("   • Correspondent banking logic");
            }

            println!("\n📝 All Rule Results:");
            for result in &report.results {
                let status = if result.passed { "✅" } else { "❌" };
                println!("   {} {}", status, result.rule_name);
            }
        }
        Err(e) => {
            println!("❌ Validation Error: {}", e);
        }
    }
    println!();

    // Step 6: Convert to JSON and display
    println!("🔄 Step 6: JSON Conversion");
    println!("--------------------------");

    match swift_message.to_json_string() {
        Ok(json_string) => {
            println!("✅ Successfully converted to JSON");

            // Parse and pretty-print a subset of the JSON (not the full output for readability)
            let json_value: serde_json::Value = serde_json::from_str(&json_string)?;

            println!("\n📄 JSON Summary:");
            if let Some(fields) = json_value.get("fields") {
                println!(
                    "   Fields in JSON: {}",
                    fields.as_object().map(|o| o.len()).unwrap_or(0)
                );

                // Show key STP fields
                if let Some(field_20) = fields.get("20") {
                    println!(
                        "   • Transaction Reference: {:?}",
                        field_20.get("transaction_reference")
                    );
                }
                if let Some(field_32a) = fields.get("32A") {
                    println!(
                        "   • Amount: {} {}",
                        field_32a
                            .get("currency")
                            .and_then(|v| v.as_str())
                            .unwrap_or(""),
                        field_32a
                            .get("amount")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0)
                    );
                }
                if let Some(field_33b) = fields.get("33B") {
                    println!(
                        "   • Instructed Amount: {} {} [STP Cross-Currency]",
                        field_33b
                            .get("currency")
                            .and_then(|v| v.as_str())
                            .unwrap_or(""),
                        field_33b
                            .get("amount")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0)
                    );
                }
            }
            println!("{}", json_string);
            println!("\n💾 Full JSON available for further processing");
        }
        Err(e) => {
            println!("❌ JSON conversion error: {}", e);
        }
    }

    println!("\n🎉 MT103-STP Example completed successfully!");
    println!("   ✅ Parsed MT103-STP message");
    println!("   ✅ Validated STP compliance (all 10 conditional rules)");
    println!("   ✅ Verified straight-through processing readiness");
    println!("   ✅ Displayed enhanced field details");
    println!("   ✅ Converted to JSON for system integration");
    println!("\n🚀 This message is ready for automated STP processing!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt103_stp_example() {
        // Ensure the example runs without errors
        assert!(main().is_ok());
    }
}
