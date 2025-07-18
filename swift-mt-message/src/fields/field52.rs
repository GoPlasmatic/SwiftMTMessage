use serde::{Deserialize, Serialize};
use swift_mt_message_macros::SwiftField;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52A {
    #[component("[/1!a/34x]")]
    pub party_identifier: Option<String>,

    #[component("4!a2!a2!c[3!c]")]
    pub bic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52B {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("[35x]")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52C {
    #[component("/34x")]
    pub party_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SwiftField)]
pub struct Field52D {
    #[component("[/1!a][/34x]")]
    pub party_identifier: Option<String>,

    #[component("4*35x")]
    pub name_and_address: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52AccountServicingInstitution {
    A(Field52A),
    C(Field52C),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52OrderingInstitution {
    A(Field52A),
    D(Field52D),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52CreditorBank {
    A(Field52A),
    C(Field52C),
    D(Field52D),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SwiftField)]
pub enum Field52DrawerBank {
    A(Field52A),
    B(Field52B),
    D(Field52D),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SwiftField;

    #[test]
    fn test_field52a_bic_only() {
        // Test case that should work: just a BIC without party identifier
        let test_value = "CHASUS33";
        println!("Testing value: '{}'", test_value);
        
        match Field52A::parse(test_value) {
            Ok(field) => {
                println!("✅ Successfully parsed: {:?}", field);
                assert_eq!(field.party_identifier, None);
                assert_eq!(field.bic, "CHASUS33");
            }
            Err(e) => {
                println!("❌ Failed to parse: {:?}", e);
                panic!("Should have parsed successfully but got: {:?}", e);
            }
        }
    }

    #[test]
    fn test_field52a_with_party_id() {
        // Test case with party identifier
        let test_value = "/D/ABC123CHASUS33";
        println!("Testing value: '{}'", test_value);
        
        match Field52A::parse(test_value) {
            Ok(field) => {
                println!("✅ Successfully parsed: {:?}", field);
                assert_eq!(field.party_identifier, Some("/D/ABC123".to_string()));
                assert_eq!(field.bic, "CHASUS33");
            }
            Err(e) => {
                println!("❌ Failed to parse: {:?}", e);
                panic!("Should have parsed successfully but got: {:?}", e);
            }
        }
    }
}
