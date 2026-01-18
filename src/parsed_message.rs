//! # ParsedSwiftMessage
//!
//! Enum for automatic message type detection and parsing. Used by `SwiftParser::parse_auto()`.

use crate::{ValidationResult, messages::*, swift_message::SwiftMessage};
use serde::{Deserialize, Serialize};

/// Enum of all supported SWIFT message types (30+ types)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[serde(tag = "mt_type")]
pub enum ParsedSwiftMessage {
    #[serde(rename = "101")]
    MT101(Box<SwiftMessage<MT101>>),
    #[serde(rename = "103")]
    MT103(Box<SwiftMessage<MT103>>),
    #[serde(rename = "104")]
    MT104(Box<SwiftMessage<MT104>>),
    #[serde(rename = "107")]
    MT107(Box<SwiftMessage<MT107>>),
    #[serde(rename = "110")]
    MT110(Box<SwiftMessage<MT110>>),
    #[serde(rename = "111")]
    MT111(Box<SwiftMessage<MT111>>),
    #[serde(rename = "112")]
    MT112(Box<SwiftMessage<MT112>>),
    #[serde(rename = "190")]
    MT190(Box<SwiftMessage<MT190>>),
    #[serde(rename = "191")]
    MT191(Box<SwiftMessage<MT191>>),
    #[serde(rename = "200")]
    MT200(Box<SwiftMessage<MT200>>),
    #[serde(rename = "202")]
    MT202(Box<SwiftMessage<MT202>>),
    #[serde(rename = "204")]
    MT204(Box<SwiftMessage<MT204>>),
    #[serde(rename = "205")]
    MT205(Box<SwiftMessage<MT205>>),
    #[serde(rename = "210")]
    MT210(Box<SwiftMessage<MT210>>),
    #[serde(rename = "290")]
    MT290(Box<SwiftMessage<MT290>>),
    #[serde(rename = "291")]
    MT291(Box<SwiftMessage<MT291>>),
    #[serde(rename = "900")]
    MT900(Box<SwiftMessage<MT900>>),
    #[serde(rename = "910")]
    MT910(Box<SwiftMessage<MT910>>),
    #[serde(rename = "920")]
    MT920(Box<SwiftMessage<MT920>>),
    #[serde(rename = "935")]
    MT935(Box<SwiftMessage<MT935>>),
    #[serde(rename = "940")]
    MT940(Box<SwiftMessage<MT940>>),
    #[serde(rename = "941")]
    MT941(Box<SwiftMessage<MT941>>),
    #[serde(rename = "942")]
    MT942(Box<SwiftMessage<MT942>>),
    #[serde(rename = "950")]
    MT950(Box<SwiftMessage<MT950>>),
    #[serde(rename = "192")]
    MT192(Box<SwiftMessage<MT192>>),
    #[serde(rename = "196")]
    MT196(Box<SwiftMessage<MT196>>),
    #[serde(rename = "292")]
    MT292(Box<SwiftMessage<MT292>>),
    #[serde(rename = "296")]
    MT296(Box<SwiftMessage<MT296>>),
    #[serde(rename = "199")]
    MT199(Box<SwiftMessage<MT199>>),
    #[serde(rename = "299")]
    MT299(Box<SwiftMessage<MT299>>),
}

impl ParsedSwiftMessage {
    /// Get the message type as a string
    pub fn message_type(&self) -> &'static str {
        match self {
            ParsedSwiftMessage::MT101(_) => "101",
            ParsedSwiftMessage::MT103(_) => "103",
            ParsedSwiftMessage::MT104(_) => "104",
            ParsedSwiftMessage::MT107(_) => "107",
            ParsedSwiftMessage::MT110(_) => "110",
            ParsedSwiftMessage::MT111(_) => "111",
            ParsedSwiftMessage::MT112(_) => "112",
            ParsedSwiftMessage::MT190(_) => "190",
            ParsedSwiftMessage::MT191(_) => "191",
            ParsedSwiftMessage::MT200(_) => "200",
            ParsedSwiftMessage::MT202(_) => "202",
            ParsedSwiftMessage::MT204(_) => "204",
            ParsedSwiftMessage::MT205(_) => "205",
            ParsedSwiftMessage::MT210(_) => "210",
            ParsedSwiftMessage::MT290(_) => "290",
            ParsedSwiftMessage::MT291(_) => "291",
            ParsedSwiftMessage::MT900(_) => "900",
            ParsedSwiftMessage::MT910(_) => "910",
            ParsedSwiftMessage::MT920(_) => "920",
            ParsedSwiftMessage::MT935(_) => "935",
            ParsedSwiftMessage::MT940(_) => "940",
            ParsedSwiftMessage::MT941(_) => "941",
            ParsedSwiftMessage::MT942(_) => "942",
            ParsedSwiftMessage::MT950(_) => "950",
            ParsedSwiftMessage::MT192(_) => "192",
            ParsedSwiftMessage::MT196(_) => "196",
            ParsedSwiftMessage::MT292(_) => "292",
            ParsedSwiftMessage::MT296(_) => "296",
            ParsedSwiftMessage::MT199(_) => "199",
            ParsedSwiftMessage::MT299(_) => "299",
        }
    }

    /// Convert to a specific message type if it matches
    pub fn as_mt101(&self) -> Option<&SwiftMessage<MT101>> {
        match self {
            ParsedSwiftMessage::MT101(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt103(&self) -> Option<&SwiftMessage<MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt104(&self) -> Option<&SwiftMessage<MT104>> {
        match self {
            ParsedSwiftMessage::MT104(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt107(&self) -> Option<&SwiftMessage<MT107>> {
        match self {
            ParsedSwiftMessage::MT107(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt110(&self) -> Option<&SwiftMessage<MT110>> {
        match self {
            ParsedSwiftMessage::MT110(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt111(&self) -> Option<&SwiftMessage<MT111>> {
        match self {
            ParsedSwiftMessage::MT111(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt112(&self) -> Option<&SwiftMessage<MT112>> {
        match self {
            ParsedSwiftMessage::MT112(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt190(&self) -> Option<&SwiftMessage<MT190>> {
        match self {
            ParsedSwiftMessage::MT190(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt191(&self) -> Option<&SwiftMessage<MT191>> {
        match self {
            ParsedSwiftMessage::MT191(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt200(&self) -> Option<&SwiftMessage<MT200>> {
        match self {
            ParsedSwiftMessage::MT200(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt202(&self) -> Option<&SwiftMessage<MT202>> {
        match self {
            ParsedSwiftMessage::MT202(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt204(&self) -> Option<&SwiftMessage<MT204>> {
        match self {
            ParsedSwiftMessage::MT204(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt205(&self) -> Option<&SwiftMessage<MT205>> {
        match self {
            ParsedSwiftMessage::MT205(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt210(&self) -> Option<&SwiftMessage<MT210>> {
        match self {
            ParsedSwiftMessage::MT210(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt290(&self) -> Option<&SwiftMessage<MT290>> {
        match self {
            ParsedSwiftMessage::MT290(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt291(&self) -> Option<&SwiftMessage<MT291>> {
        match self {
            ParsedSwiftMessage::MT291(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt900(&self) -> Option<&SwiftMessage<MT900>> {
        match self {
            ParsedSwiftMessage::MT900(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt910(&self) -> Option<&SwiftMessage<MT910>> {
        match self {
            ParsedSwiftMessage::MT910(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt920(&self) -> Option<&SwiftMessage<MT920>> {
        match self {
            ParsedSwiftMessage::MT920(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt935(&self) -> Option<&SwiftMessage<MT935>> {
        match self {
            ParsedSwiftMessage::MT935(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt940(&self) -> Option<&SwiftMessage<MT940>> {
        match self {
            ParsedSwiftMessage::MT940(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt941(&self) -> Option<&SwiftMessage<MT941>> {
        match self {
            ParsedSwiftMessage::MT941(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt942(&self) -> Option<&SwiftMessage<MT942>> {
        match self {
            ParsedSwiftMessage::MT942(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt950(&self) -> Option<&SwiftMessage<MT950>> {
        match self {
            ParsedSwiftMessage::MT950(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt192(&self) -> Option<&SwiftMessage<MT192>> {
        match self {
            ParsedSwiftMessage::MT192(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt196(&self) -> Option<&SwiftMessage<MT196>> {
        match self {
            ParsedSwiftMessage::MT196(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt292(&self) -> Option<&SwiftMessage<MT292>> {
        match self {
            ParsedSwiftMessage::MT292(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt296(&self) -> Option<&SwiftMessage<MT296>> {
        match self {
            ParsedSwiftMessage::MT296(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt199(&self) -> Option<&SwiftMessage<MT199>> {
        match self {
            ParsedSwiftMessage::MT199(msg) => Some(msg),
            _ => None,
        }
    }
    pub fn as_mt299(&self) -> Option<&SwiftMessage<MT299>> {
        match self {
            ParsedSwiftMessage::MT299(msg) => Some(msg),
            _ => None,
        }
    }

    /// Convert into a specific message type if it matches
    pub fn into_mt101(self) -> Option<SwiftMessage<MT101>> {
        match self {
            ParsedSwiftMessage::MT101(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt103(self) -> Option<SwiftMessage<MT103>> {
        match self {
            ParsedSwiftMessage::MT103(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt104(self) -> Option<SwiftMessage<MT104>> {
        match self {
            ParsedSwiftMessage::MT104(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt107(self) -> Option<SwiftMessage<MT107>> {
        match self {
            ParsedSwiftMessage::MT107(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt110(self) -> Option<SwiftMessage<MT110>> {
        match self {
            ParsedSwiftMessage::MT110(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt111(self) -> Option<SwiftMessage<MT111>> {
        match self {
            ParsedSwiftMessage::MT111(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt112(self) -> Option<SwiftMessage<MT112>> {
        match self {
            ParsedSwiftMessage::MT112(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt190(self) -> Option<SwiftMessage<MT190>> {
        match self {
            ParsedSwiftMessage::MT190(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt191(self) -> Option<SwiftMessage<MT191>> {
        match self {
            ParsedSwiftMessage::MT191(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt192(self) -> Option<SwiftMessage<MT192>> {
        match self {
            ParsedSwiftMessage::MT192(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt196(self) -> Option<SwiftMessage<MT196>> {
        match self {
            ParsedSwiftMessage::MT196(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt199(self) -> Option<SwiftMessage<MT199>> {
        match self {
            ParsedSwiftMessage::MT199(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt200(self) -> Option<SwiftMessage<MT200>> {
        match self {
            ParsedSwiftMessage::MT200(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt202(self) -> Option<SwiftMessage<MT202>> {
        match self {
            ParsedSwiftMessage::MT202(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt204(self) -> Option<SwiftMessage<MT204>> {
        match self {
            ParsedSwiftMessage::MT204(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt205(self) -> Option<SwiftMessage<MT205>> {
        match self {
            ParsedSwiftMessage::MT205(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt210(self) -> Option<SwiftMessage<MT210>> {
        match self {
            ParsedSwiftMessage::MT210(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt290(self) -> Option<SwiftMessage<MT290>> {
        match self {
            ParsedSwiftMessage::MT290(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt291(self) -> Option<SwiftMessage<MT291>> {
        match self {
            ParsedSwiftMessage::MT291(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt292(self) -> Option<SwiftMessage<MT292>> {
        match self {
            ParsedSwiftMessage::MT292(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt296(self) -> Option<SwiftMessage<MT296>> {
        match self {
            ParsedSwiftMessage::MT296(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt900(self) -> Option<SwiftMessage<MT900>> {
        match self {
            ParsedSwiftMessage::MT900(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt910(self) -> Option<SwiftMessage<MT910>> {
        match self {
            ParsedSwiftMessage::MT910(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt920(self) -> Option<SwiftMessage<MT920>> {
        match self {
            ParsedSwiftMessage::MT920(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt935(self) -> Option<SwiftMessage<MT935>> {
        match self {
            ParsedSwiftMessage::MT935(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt940(self) -> Option<SwiftMessage<MT940>> {
        match self {
            ParsedSwiftMessage::MT940(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt941(self) -> Option<SwiftMessage<MT941>> {
        match self {
            ParsedSwiftMessage::MT941(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt942(self) -> Option<SwiftMessage<MT942>> {
        match self {
            ParsedSwiftMessage::MT942(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt950(self) -> Option<SwiftMessage<MT950>> {
        match self {
            ParsedSwiftMessage::MT950(msg) => Some(*msg),
            _ => None,
        }
    }
    pub fn into_mt299(self) -> Option<SwiftMessage<MT299>> {
        match self {
            ParsedSwiftMessage::MT299(msg) => Some(*msg),
            _ => None,
        }
    }

    /// Validate using SWIFT SR2025 network validation rules
    pub fn validate(&self) -> ValidationResult {
        match self {
            ParsedSwiftMessage::MT101(mt101) => mt101.validate(),
            ParsedSwiftMessage::MT103(mt103) => mt103.validate(),
            ParsedSwiftMessage::MT104(mt104) => mt104.validate(),
            ParsedSwiftMessage::MT107(mt107) => mt107.validate(),
            ParsedSwiftMessage::MT110(mt110) => mt110.validate(),
            ParsedSwiftMessage::MT111(mt111) => mt111.validate(),
            ParsedSwiftMessage::MT112(mt112) => mt112.validate(),
            ParsedSwiftMessage::MT190(mt190) => mt190.validate(),
            ParsedSwiftMessage::MT191(mt191) => mt191.validate(),
            ParsedSwiftMessage::MT192(mt192) => mt192.validate(),
            ParsedSwiftMessage::MT196(mt196) => mt196.validate(),
            ParsedSwiftMessage::MT199(mt199) => mt199.validate(),
            ParsedSwiftMessage::MT200(mt200) => mt200.validate(),
            ParsedSwiftMessage::MT202(mt202) => mt202.validate(),
            ParsedSwiftMessage::MT204(mt204) => mt204.validate(),
            ParsedSwiftMessage::MT205(mt205) => mt205.validate(),
            ParsedSwiftMessage::MT210(mt210) => mt210.validate(),
            ParsedSwiftMessage::MT290(mt290) => mt290.validate(),
            ParsedSwiftMessage::MT291(mt291) => mt291.validate(),
            ParsedSwiftMessage::MT900(mt900) => mt900.validate(),
            ParsedSwiftMessage::MT910(mt910) => mt910.validate(),
            ParsedSwiftMessage::MT920(mt920) => mt920.validate(),
            ParsedSwiftMessage::MT292(mt292) => mt292.validate(),
            ParsedSwiftMessage::MT296(mt296) => mt296.validate(),
            ParsedSwiftMessage::MT299(mt299) => mt299.validate(),
            ParsedSwiftMessage::MT935(mt935) => mt935.validate(),
            ParsedSwiftMessage::MT940(mt940) => mt940.validate(),
            ParsedSwiftMessage::MT941(mt941) => mt941.validate(),
            ParsedSwiftMessage::MT942(mt942) => mt942.validate(),
            ParsedSwiftMessage::MT950(mt950) => mt950.validate(),
        }
    }
}
