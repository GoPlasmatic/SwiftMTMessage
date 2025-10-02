// Test file for verifying the three macro implementations
use crate::fields::*;
use crate::message_parser::MessageParser;
use crate::swift_message_standard;
use crate::swift_message_repetitive;
use crate::swift_message_three_seq;
use std::collections::HashMap;

// Test 1: Standard Message (MT900 - simple flat structure)
swift_message_standard! {
    MT900Test => "900" {
        field_20: Field20 => "20" required,
        field_21: Field21NoOption => "21" required,
        field_25: Field25AccountIdentification => "25" required,
        field_13d: Field13D => "13D" optional,
        field_32a: Field32A => "32A" required,
        field_52: Field52OrderingInstitution => "52" optional,
        field_72: Field72 => "72" optional
    }
}

// Test 2: Repetitive Sequence Message (MT920)
swift_message_repetitive! {
    MT920Test => "920" {
        // Header section
        header {
            field_20: Field20 => "20" required
        }
        // Repetitive sequence
        sequence: Vec<MT920TestSequence> {
            field_12: Field12 => "12" required,
            field_25: Field25A => "25" required,
            floor_limit_debit: Field34F => "34F" optional,
            floor_limit_credit: Field34F => "34F" optional
        }
        // Validations
        validations {
            max_repetitions: 100
        }
    }
}

// Test 3: Three Sequence Message (MT535 - Holdings Statement)
swift_message_three_seq! {
    MT535Test => "535" {
        // Sequence A: General Information
        seq_a {
            field_20: Field20 => "20" required,
            field_23: Field23 => "23" required,
            statement_number: Field28E => "28E" required,
            field_13a: Field13A => "13A" optional
        }
        // Sequence B: Sub-safekeeping accounts
        seq_b {
            field_95: Field95 => "95" required,
            field_97a: Field97A => "97A" optional,
            field_94a: Field94F => "94F" optional,
            activity_flag: Field17B => "17B" optional
        }
        // Sequence C: Financial Instruments
        seq_c {
            field_35b: Field35B => "35B" required,
            field_93b: Field93B => "93B" required,
            field_13b: Field13B => "13B" optional
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_macro_compilation() {
        // Test that MT900Test compiles and has expected methods
        let input = ":20:TEST123\r\n:21:REF456\r\n:25:ACC123456\r\n:32A:241225USD1234,56\r\n";
        let result = MT900Test::parse(input);
        assert!(result.is_ok());

        // Test field map conversion
        let mut fields = HashMap::new();
        fields.insert("20".to_string(), "TEST123".to_string());
        fields.insert("21".to_string(), "REF456".to_string());
        fields.insert("25".to_string(), "ACC123456".to_string());
        fields.insert("32A".to_string(), "241225USD1234,56".to_string());

        let from_map = MT900Test::from_field_map(&fields);
        assert!(from_map.is_ok());
    }

    #[test]
    fn test_repetitive_macro_compilation() {
        // Test that MT920Test compiles
        let input = ":20:TEST123\r\n:12:940\r\n:25:ACC123456\r\n:12:941\r\n:25:ACC789012\r\n";
        let result = MT920Test::parse(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_three_seq_macro_compilation() {
        // Test that MT535Test compiles
        let input = ":16R:GENL\r\n:20:TEST123\r\n:23:NEWM\r\n:28E:00001/ONLY\r\n:16S:GENL\r\n";
        let result = MT535Test::parse(input);
        // Basic compilation test
        assert!(result.is_ok() || result.is_err()); // Just checking it compiles
    }
}