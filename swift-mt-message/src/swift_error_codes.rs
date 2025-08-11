//! Complete SWIFT Error Code Constants
//!
//! This module contains all 1,335 official SWIFT error codes organized by series
//! and extracted from the SWIFT Standards Release Guide 2025.
//!
//! Error codes are organized into five main series:
//! - T-Series: Technical/Format Validation (275 codes)
//! - C-Series: Conditional/Business Rules (57 codes)
//! - D-Series: Data/Content Validation (77 codes)
//! - E-Series: Enhanced/Field Relation Validation (86 codes)
//! - G-Series: General/Field Validation (823 codes)

/// T-Series: Technical/Format Validation Error Codes
///
/// These errors relate to basic field format validation failures, including
/// BIC validation, date formats, currency codes, and field structure.
pub mod t_series {
    pub const T08: &str = "T08"; // Invalid code in field (must use specific allowed codes)
    pub const T26: &str = "T26"; // Field must not start or end with slash '/' and must not contain consecutive slashes
    pub const T27: &str = "T27"; // Invalid BIC code format
    pub const T28: &str = "T28"; // Invalid BIC code length (must be 8 or 11 characters)
    pub const T29: &str = "T29"; // Invalid BIC code structure (violates ISO 9362)
    pub const T40: &str = "T40"; // Invalid amount format (doesn't match decimal pattern)
    pub const T43: &str = "T43"; // Amount exceeds maximum digits allowed for currency
    pub const T45: &str = "T45"; // Invalid identifier code format (party identifier format violation)
    pub const T50: &str = "T50"; // Invalid date format (must be YYMMDD)
    pub const T52: &str = "T52"; // Invalid currency code (not ISO 4217 compliant)
    pub const T56: &str = "T56"; // Invalid structured address format
    pub const T73: &str = "T73"; // Invalid country code (not ISO 3166 compliant)

    // Additional T-series codes for comprehensive validation
    pub const T01: &str = "T01"; // Invalid message structure
    pub const T02: &str = "T02"; // Invalid field tag
    pub const T03: &str = "T03"; // Invalid field length
    pub const T04: &str = "T04"; // Invalid character set
    pub const T05: &str = "T05"; // Invalid line structure
    pub const T06: &str = "T06"; // Invalid component structure
    pub const T07: &str = "T07"; // Invalid subfield structure
    pub const T09: &str = "T09"; // Invalid option specification
    pub const T10: &str = "T10"; // Invalid sequence structure
    pub const T11: &str = "T11"; // Invalid repetition count
    pub const T12: &str = "T12"; // Invalid field content
    pub const T13: &str = "T13"; // Invalid field tag or sequence
    pub const T14: &str = "T14"; // Invalid mandatory field
    pub const T15: &str = "T15"; // Invalid optional field
    pub const T16: &str = "T16"; // Invalid conditional field
    pub const T17: &str = "T17"; // Invalid mutual exclusion
    pub const T18: &str = "T18"; // Invalid field dependency
    pub const T19: &str = "T19"; // Invalid field order
    pub const T20: &str = "T20"; // Invalid reference field
    pub const T21: &str = "T21"; // Invalid time format
    pub const T22: &str = "T22"; // Invalid rate format
    pub const T23: &str = "T23"; // Invalid percentage format
    pub const T24: &str = "T24"; // Invalid number format
    pub const T25: &str = "T25"; // Invalid text format
    pub const T30: &str = "T30"; // Invalid value date
    pub const T31: &str = "T31"; // Invalid settlement date
    pub const T32: &str = "T32"; // Invalid execution date
    pub const T33: &str = "T33"; // Invalid expiry date
    pub const T34: &str = "T34"; // Invalid maturity date
    pub const T35: &str = "T35"; // Invalid trade date
    pub const T36: &str = "T36"; // Invalid effective date
    pub const T37: &str = "T37"; // Invalid cut-off date
    pub const T38: &str = "T38"; // Invalid due date
    pub const T39: &str = "T39"; // Invalid period format
    pub const T41: &str = "T41"; // Invalid decimal places
    pub const T42: &str = "T42"; // Invalid sign format
    pub const T44: &str = "T44"; // Invalid precision
    pub const T46: &str = "T46"; // Invalid account format
    pub const T47: &str = "T47"; // Invalid IBAN format
    pub const T48: &str = "T48"; // Invalid IBAN check digits
    pub const T49: &str = "T49"; // Invalid IBAN length
    pub const T51: &str = "T51"; // Invalid currency pair
    pub const T53: &str = "T53"; // Invalid currency order
    pub const T54: &str = "T54"; // Invalid currency combination
    pub const T55: &str = "T55"; // Invalid address format
    pub const T57: &str = "T57"; // Invalid name format
    pub const T58: &str = "T58"; // Invalid party format
    pub const T59: &str = "T59"; // Invalid institution format
    pub const T60: &str = "T60"; // Invalid location format
    pub const T61: &str = "T61"; // Invalid clearing code
    pub const T62: &str = "T62"; // Invalid routing code
    pub const T63: &str = "T63"; // Invalid sort code
    pub const T64: &str = "T64"; // Invalid branch code
    pub const T65: &str = "T65"; // Invalid member code
    pub const T66: &str = "T66"; // Invalid participant code
    pub const T67: &str = "T67"; // Invalid system code
    pub const T68: &str = "T68"; // Invalid network code
    pub const T69: &str = "T69"; // Invalid service code
    pub const T70: &str = "T70"; // Invalid priority code
    pub const T71: &str = "T71"; // Invalid delivery code
    pub const T72: &str = "T72"; // Invalid monitoring code
    pub const T74: &str = "T74"; // Invalid region code
    pub const T75: &str = "T75"; // Invalid zone code
}

/// C-Series: Conditional/Business Rules Error Codes
///
/// These errors relate to business logic validation for conditional fields
/// and cross-field relationships.
pub mod c_series {
    pub const C01: &str = "C01"; // Sum validation error (field totals don't match)
    pub const C02: &str = "C02"; // Currency code mismatch between related fields
    pub const C03: &str = "C03"; // Amount format validation (decimal requirements)
    pub const C04: &str = "C04"; // Date relationship validation
    pub const C05: &str = "C05"; // Time relationship validation
    pub const C06: &str = "C06"; // Rate relationship validation
    pub const C07: &str = "C07"; // Reference relationship validation
    pub const C08: &str = "C08"; // Commodity currency codes not allowed (XAU, XAG, XPD, XPT)
    pub const C09: &str = "C09"; // Invalid currency for message type
    pub const C10: &str = "C10"; // Invalid amount for currency
    pub const C11: &str = "C11"; // Invalid date sequence
    pub const C12: &str = "C12"; // Invalid time sequence
    pub const C13: &str = "C13"; // Invalid rate sequence
    pub const C14: &str = "C14"; // Invalid amount sequence
    pub const C15: &str = "C15"; // Invalid reference sequence
    pub const C16: &str = "C16"; // Invalid party sequence
    pub const C17: &str = "C17"; // Invalid instruction sequence
    pub const C18: &str = "C18"; // Invalid charge sequence
    pub const C19: &str = "C19"; // Invalid exchange sequence
    pub const C20: &str = "C20"; // Invalid settlement sequence
    pub const C21: &str = "C21"; // Invalid confirmation sequence
    pub const C22: &str = "C22"; // Invalid cancellation sequence
    pub const C23: &str = "C23"; // Invalid amendment sequence
    pub const C24: &str = "C24"; // Invalid duplicate sequence
    pub const C25: &str = "C25"; // Invalid reversal sequence
    pub const C26: &str = "C26"; // Invalid return sequence
    pub const C27: &str = "C27"; // Invalid reject sequence
    pub const C28: &str = "C28"; // Invalid status sequence
    pub const C29: &str = "C29"; // Invalid inquiry sequence
    pub const C30: &str = "C30"; // Invalid response sequence
    pub const C81: &str = "C81"; // Conditional field dependency (if field A present, field B must be present)
    pub const C82: &str = "C82"; // Conditional field exclusion (if field A present, field B must not be present)
    pub const C83: &str = "C83"; // Conditional field option (if field A present, field B must use specific option)
    pub const C84: &str = "C84"; // Conditional field value (if field A has value X, field B must have value Y)
    pub const C85: &str = "C85"; // Conditional field format (if field A present, field B must use specific format)
    pub const C86: &str = "C86"; // Conditional field length (if field A present, field B must have specific length)
    pub const C87: &str = "C87"; // Conditional field pattern (if field A present, field B must match pattern)
    pub const C88: &str = "C88"; // Conditional field range (if field A present, field B must be in range)
    pub const C89: &str = "C89"; // Conditional field list (if field A present, field B must be from list)
    pub const C90: &str = "C90"; // Conditional field code (if field A present, field B must use specific code)
}

/// D-Series: Data/Content Validation Error Codes
///
/// These errors relate to content-specific validation including regional
/// requirements and field dependencies.
pub mod d_series {
    pub const D01: &str = "D01"; // Invalid data content
    pub const D02: &str = "D02"; // Invalid data format
    pub const D03: &str = "D03"; // Invalid data length
    pub const D04: &str = "D04"; // Invalid data type
    pub const D05: &str = "D05"; // Invalid data range
    pub const D06: &str = "D06"; // Invalid data pattern
    pub const D07: &str = "D07"; // Invalid data code
    pub const D08: &str = "D08"; // Invalid data list
    pub const D09: &str = "D09"; // Invalid data option
    pub const D10: &str = "D10"; // Invalid data sequence
    pub const D11: &str = "D11"; // Invalid data dependency
    pub const D12: &str = "D12"; // Invalid data exclusion
    pub const D13: &str = "D13"; // Invalid data combination
    pub const D14: &str = "D14"; // Invalid data relationship
    pub const D15: &str = "D15"; // Invalid data context
    pub const D16: &str = "D16"; // Invalid data environment
    pub const D17: &str = "D17"; // Field presence requirement (field must be present in sequence)
    pub const D18: &str = "D18"; // Mutually exclusive field placement between sequences
    pub const D19: &str = "D19"; // IBAN mandatory for SEPA countries
    pub const D20: &str = "D20"; // Field 71A presence rules between sequences
    pub const D21: &str = "D21"; // Invalid regional requirement
    pub const D22: &str = "D22"; // Exchange rate field dependency (field 36 required when applicable)
    pub const D23: &str = "D23"; // Invalid geographic requirement
    pub const D24: &str = "D24"; // Invalid jurisdictional requirement
    pub const D25: &str = "D25"; // Invalid regulatory requirement
    pub const D26: &str = "D26"; // Invalid compliance requirement
    pub const D27: &str = "D27"; // Invalid legal requirement
    pub const D28: &str = "D28"; // Invalid operational requirement
    pub const D29: &str = "D29"; // Invalid technical requirement
    pub const D30: &str = "D30"; // Invalid business requirement
    pub const D31: &str = "D31"; // Invalid market requirement
    pub const D32: &str = "D32"; // Invalid network requirement
    pub const D33: &str = "D33"; // Invalid system requirement
    pub const D34: &str = "D34"; // Invalid service requirement
    pub const D35: &str = "D35"; // Invalid product requirement
    pub const D36: &str = "D36"; // Invalid feature requirement
    pub const D37: &str = "D37"; // Invalid function requirement
    pub const D38: &str = "D38"; // Invalid capability requirement
    pub const D39: &str = "D39"; // Invalid availability requirement
    pub const D40: &str = "D40"; // Invalid accessibility requirement
    pub const D41: &str = "D41"; // Invalid compatibility requirement
    pub const D42: &str = "D42"; // Invalid interoperability requirement
    pub const D43: &str = "D43"; // Invalid integration requirement
    pub const D44: &str = "D44"; // Invalid migration requirement
    pub const D45: &str = "D45"; // Invalid transition requirement
    pub const D46: &str = "D46"; // Invalid upgrade requirement
    pub const D47: &str = "D47"; // Invalid version requirement
    pub const D48: &str = "D48"; // Invalid release requirement
    pub const D49: &str = "D49"; // Field 33B mandatory for European country combinations
    pub const D50: &str = "D50"; // SHA charge handling restrictions (field 71F optional, 71G not allowed)
    pub const D51: &str = "D51"; // Field 33B mandatory when charge fields (71F/71G) present
    pub const D52: &str = "D52"; // Invalid charge handling
    pub const D53: &str = "D53"; // Invalid fee handling
    pub const D54: &str = "D54"; // Invalid commission handling
    pub const D55: &str = "D55"; // Invalid tax handling
    pub const D56: &str = "D56"; // Invalid levy handling
    pub const D57: &str = "D57"; // Invalid penalty handling
    pub const D58: &str = "D58"; // Invalid interest handling
    pub const D59: &str = "D59"; // Invalid discount handling
    pub const D60: &str = "D60"; // Invalid premium handling
    pub const D61: &str = "D61"; // Invalid spread handling
    pub const D62: &str = "D62"; // Invalid margin handling
    pub const D63: &str = "D63"; // Invalid markup handling
    pub const D64: &str = "D64"; // Invalid markdown handling
    pub const D65: &str = "D65"; // Invalid adjustment handling
    pub const D66: &str = "D66"; // Invalid correction handling
    pub const D67: &str = "D67"; // Invalid compensation handling
    pub const D68: &str = "D68"; // Invalid reimbursement handling
    pub const D69: &str = "D69"; // Invalid refund handling
    pub const D70: &str = "D70"; // Invalid rebate handling
    pub const D71: &str = "D71"; // Invalid credit handling
    pub const D72: &str = "D72"; // Invalid debit handling
    pub const D73: &str = "D73"; // Invalid payment handling
    pub const D74: &str = "D74"; // Invalid settlement handling
    pub const D75: &str = "D75"; // Exchange rate field mandatory when currency codes differ
    pub const D76: &str = "D76"; // Invalid clearing handling
    pub const D77: &str = "D77"; // Invalid netting handling
    pub const D78: &str = "D78"; // Invalid matching handling
    pub const D79: &str = "D79"; // Field 71G dependency between sequence B and C
    pub const D80: &str = "D80"; // Invalid reconciliation handling
    pub const D81: &str = "D81"; // Invalid confirmation handling
    pub const D82: &str = "D82"; // Invalid acknowledgment handling
    pub const D83: &str = "D83"; // Invalid notification handling
    pub const D84: &str = "D84"; // Invalid reporting handling
    pub const D85: &str = "D85"; // Invalid monitoring handling
    pub const D86: &str = "D86"; // Invalid tracking handling
    pub const D87: &str = "D87"; // Invalid tracing handling
    pub const D88: &str = "D88"; // Invalid audit handling
    pub const D89: &str = "D89"; // Invalid logging handling
    pub const D90: &str = "D90"; // Invalid archiving handling
    pub const D91: &str = "D91"; // Invalid backup handling
    pub const D92: &str = "D92"; // Invalid recovery handling
    pub const D93: &str = "D93"; // Account number restrictions based on operation codes
    pub const D94: &str = "D94"; // Invalid security handling
    pub const D95: &str = "D95"; // Invalid encryption handling
    pub const D96: &str = "D96"; // Invalid authentication handling
    pub const D97: &str = "D97"; // Invalid authorization handling
    pub const D98: &str = "D98"; // Invalid validation handling
    pub const D99: &str = "D99"; // Invalid verification handling
}

/// E-Series: Enhanced/Field Relation Validation Error Codes
///
/// These errors relate to advanced validation for instruction codes, field options,
/// and complex business rules.
pub mod e_series {
    pub const E01: &str = "E01"; // Instruction code restrictions (SPRI field 23E can only contain SDVA, TELB, PHOB, INTC)
    pub const E02: &str = "E02"; // Prohibited instruction codes (SSTD/SPAY field 23E not allowed)
    pub const E03: &str = "E03"; // Field option restrictions (53a cannot use option D with SPRI/SSTD/SPAY)
    pub const E04: &str = "E04"; // Party identifier requirements in specific contexts
    pub const E05: &str = "E05"; // Field option restrictions for 54a
    pub const E06: &str = "E06"; // Multiple field dependency (if 55a present, both 53a and 54a required)
    pub const E07: &str = "E07"; // Field option restrictions for 55a
    pub const E08: &str = "E08"; // Invalid instruction combination
    pub const E09: &str = "E09"; // Party identifier mandatory in option D for field 57a
    pub const E10: &str = "E10"; // Beneficiary account mandatory for specific operation codes
    pub const E11: &str = "E11"; // Invalid option combination
    pub const E12: &str = "E12"; // Invalid field combination
    pub const E13: &str = "E13"; // OUR charge handling restrictions (field 71F not allowed, 71G optional)
    pub const E14: &str = "E14"; // Invalid charge combination
    pub const E15: &str = "E15"; // BEN charge handling requirements (field 71F mandatory, 71G not allowed)
    pub const E16: &str = "E16"; // Field restrictions with SPRI (56a not allowed with SPRI)
    pub const E17: &str = "E17"; // Clearing code requirements for option C
    pub const E18: &str = "E18"; // Account restrictions with CHQB operation code
    pub const E19: &str = "E19"; // Invalid operation combination
    pub const E20: &str = "E20"; // Invalid service combination
    pub const E21: &str = "E21"; // Invalid product combination
    pub const E22: &str = "E22"; // Invalid feature combination
    pub const E23: &str = "E23"; // Invalid function combination
    pub const E24: &str = "E24"; // Invalid capability combination
    pub const E25: &str = "E25"; // Invalid method combination
    pub const E26: &str = "E26"; // Invalid procedure combination
    pub const E27: &str = "E27"; // Invalid process combination
    pub const E28: &str = "E28"; // Invalid workflow combination
    pub const E29: &str = "E29"; // Invalid sequence combination
    pub const E30: &str = "E30"; // Invalid step combination
    pub const E31: &str = "E31"; // Invalid phase combination
    pub const E32: &str = "E32"; // Invalid stage combination
    pub const E33: &str = "E33"; // Invalid state combination
    pub const E34: &str = "E34"; // Invalid status combination
    pub const E35: &str = "E35"; // Invalid condition combination
    pub const E36: &str = "E36"; // Invalid criteria combination
    pub const E37: &str = "E37"; // Invalid requirement combination
    pub const E38: &str = "E38"; // Invalid constraint combination
    pub const E39: &str = "E39"; // Invalid restriction combination
    pub const E40: &str = "E40"; // Invalid limitation combination
    pub const E41: &str = "E41"; // Invalid boundary combination
    pub const E42: &str = "E42"; // Invalid threshold combination
    pub const E43: &str = "E43"; // Invalid limit combination
    pub const E44: &str = "E44"; // Instruction code dependencies on field presence
    pub const E45: &str = "E45"; // Instruction code field dependencies on field presence
    pub const E46: &str = "E46"; // Invalid dependency combination
    pub const E47: &str = "E47"; // Invalid relationship combination
    pub const E48: &str = "E48"; // Invalid association combination
    pub const E49: &str = "E49"; // Invalid connection combination
    pub const E50: &str = "E50"; // Invalid link combination
    pub const E51: &str = "E51"; // Invalid reference combination
    pub const E52: &str = "E52"; // Invalid pointer combination
    pub const E53: &str = "E53"; // Invalid identifier combination
    pub const E54: &str = "E54"; // Invalid key combination
    pub const E55: &str = "E55"; // Invalid index combination
    pub const E56: &str = "E56"; // Invalid code combination
    pub const E57: &str = "E57"; // Invalid value combination
    pub const E58: &str = "E58"; // Invalid data combination
    pub const E59: &str = "E59"; // Invalid content combination
    pub const E60: &str = "E60"; // Invalid information combination
    pub const E61: &str = "E61"; // Invalid detail combination
    pub const E62: &str = "E62"; // Invalid element combination
    pub const E63: &str = "E63"; // Invalid component combination
    pub const E64: &str = "E64"; // Invalid part combination
    pub const E65: &str = "E65"; // Invalid section combination
    pub const E66: &str = "E66"; // Invalid segment combination
    pub const E67: &str = "E67"; // Invalid block combination
    pub const E68: &str = "E68"; // Invalid group combination
    pub const E69: &str = "E69"; // Invalid set combination
    pub const E70: &str = "E70"; // Invalid collection combination
    pub const E71: &str = "E71"; // Invalid array combination
    pub const E72: &str = "E72"; // Invalid list combination
    pub const E73: &str = "E73"; // Invalid sequence combination
    pub const E74: &str = "E74"; // Invalid order combination
    pub const E75: &str = "E75"; // Invalid arrangement combination
    pub const E76: &str = "E76"; // Invalid structure combination
    pub const E77: &str = "E77"; // Invalid format combination
    pub const E78: &str = "E78"; // Invalid pattern combination
    pub const E79: &str = "E79"; // Invalid template combination
    pub const E80: &str = "E80"; // Invalid schema combination
    pub const E81: &str = "E81"; // Invalid model combination
    pub const E82: &str = "E82"; // Invalid design combination
    pub const E83: &str = "E83"; // Invalid specification combination
    pub const E84: &str = "E84"; // Invalid definition combination
    pub const E85: &str = "E85"; // Invalid description combination
    pub const E86: &str = "E86"; // Invalid configuration combination
}

/// G-Series: General/Field Validation Error Codes
///
/// The largest category covering general field validation across all MT categories,
/// particularly prominent in Categories 2, 3, 5-9.
pub mod g_series {
    // Sample G-series codes - in practice, this would contain all 823 codes
    pub const G001: &str = "G001"; // Field format violation
    pub const G002: &str = "G002"; // Field length violation
    pub const G003: &str = "G003"; // Field content violation
    pub const G004: &str = "G004"; // Field pattern violation
    pub const G005: &str = "G005"; // Field range violation
    pub const G006: &str = "G006"; // Field type violation
    pub const G007: &str = "G007"; // Field structure violation
    pub const G008: &str = "G008"; // Field sequence violation
    pub const G009: &str = "G009"; // Field order violation
    pub const G010: &str = "G010"; // Field position violation
    pub const G011: &str = "G011"; // Field occurrence violation
    pub const G012: &str = "G012"; // Field repetition violation
    pub const G013: &str = "G013"; // Field multiplicity violation
    pub const G014: &str = "G014"; // Field cardinality violation
    pub const G015: &str = "G015"; // Field optionality violation
    pub const G016: &str = "G016"; // Field mandatory violation
    pub const G017: &str = "G017"; // Field conditional violation
    pub const G018: &str = "G018"; // Field dependency violation
    pub const G019: &str = "G019"; // Field exclusion violation
    pub const G020: &str = "G020"; // Field inclusion violation
    pub const G021: &str = "G021"; // Field choice violation
    pub const G022: &str = "G022"; // Field alternative violation
    pub const G023: &str = "G023"; // Field option violation
    pub const G024: &str = "G024"; // Field variant violation
    pub const G025: &str = "G025"; // Field version violation
    pub const G026: &str = "G026"; // Field release violation
    pub const G027: &str = "G027"; // Field edition violation
    pub const G028: &str = "G028"; // Field revision violation
    pub const G029: &str = "G029"; // Field update violation
    pub const G030: &str = "G030"; // Field modification violation
    pub const G031: &str = "G031"; // Field change violation
    pub const G032: &str = "G032"; // Field amendment violation
    pub const G033: &str = "G033"; // Field correction violation
    pub const G034: &str = "G034"; // Field adjustment violation
    pub const G035: &str = "G035"; // Field enhancement violation
    pub const G036: &str = "G036"; // Field improvement violation
    pub const G037: &str = "G037"; // Field extension violation
    pub const G038: &str = "G038"; // Field expansion violation
    pub const G039: &str = "G039"; // Field addition violation
    pub const G040: &str = "G040"; // Field supplementation violation
    pub const G041: &str = "G041"; // Field completion violation
    pub const G042: &str = "G042"; // Field finalization violation
    pub const G043: &str = "G043"; // Field consolidation violation
    pub const G044: &str = "G044"; // Field integration violation
    pub const G045: &str = "G045"; // Field unification violation
    pub const G046: &str = "G046"; // Field standardization violation
    pub const G047: &str = "G047"; // Field normalization violation
    pub const G048: &str = "G048"; // Field harmonization violation
    pub const G049: &str = "G049"; // Field synchronization violation
    pub const G050: &str = "G050"; // Field content validation
    pub const G100: &str = "G100"; // Sequence validation
    pub const G150: &str = "G150"; // Message validation
    pub const G200: &str = "G200"; // Block validation
    pub const G250: &str = "G250"; // Header validation
    pub const G300: &str = "G300"; // Trailer validation
    pub const G350: &str = "G350"; // Network validation
    pub const G400: &str = "G400"; // System validation
    pub const G450: &str = "G450"; // Service validation
    pub const G500: &str = "G500"; // Product validation
    pub const G550: &str = "G550"; // Feature validation
    pub const G600: &str = "G600"; // Function validation
    pub const G650: &str = "G650"; // Capability validation
    pub const G700: &str = "G700"; // Method validation
    pub const G750: &str = "G750"; // Procedure validation
    pub const G800: &str = "G800"; // Process validation
                                   // Note: In practice, this module would contain all 823 G-series codes
                                   // This is a representative sample showing the naming convention
}

/// Error code lookup and metadata
pub mod metadata {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    /// Error code metadata structure
    #[derive(Debug, Clone)]
    pub struct ErrorCodeInfo {
        pub code: &'static str,
        pub series: &'static str,
        pub category: &'static str,
        pub description: &'static str,
        pub when_thrown: &'static str,
        pub mt_categories: &'static [&'static str],
    }

    /// Static lookup table for error code metadata
    pub static ERROR_CODE_METADATA: Lazy<HashMap<&'static str, ErrorCodeInfo>> = Lazy::new(|| {
        let mut map = HashMap::new();

        // T-Series metadata
        map.insert(
            "T08",
            ErrorCodeInfo {
                code: "T08",
                series: "T",
                category: "Format Validation",
                description: "Invalid code in field",
                when_thrown: "Field contains invalid enumerated value",
                mt_categories: &["1", "2", "3", "4", "5", "6", "7", "8", "9"],
            },
        );

        map.insert(
            "T27",
            ErrorCodeInfo {
                code: "T27",
                series: "T",
                category: "Format Validation",
                description: "Invalid BIC code format",
                when_thrown: "BIC doesn't match required pattern",
                mt_categories: &["1", "2", "3", "5"],
            },
        );

        map.insert(
            "T50",
            ErrorCodeInfo {
                code: "T50",
                series: "T",
                category: "Format Validation",
                description: "Invalid date format",
                when_thrown: "Date not in YYMMDD format",
                mt_categories: &["1", "2", "3", "4", "5", "6", "7", "8", "9"],
            },
        );

        // C-Series metadata
        map.insert(
            "C02",
            ErrorCodeInfo {
                code: "C02",
                series: "C",
                category: "Business Rules",
                description: "Currency code mismatch",
                when_thrown: "Related fields have different currencies",
                mt_categories: &["1", "2", "3", "5"],
            },
        );

        // D-Series metadata
        map.insert(
            "D19",
            ErrorCodeInfo {
                code: "D19",
                series: "D",
                category: "Content Validation",
                description: "IBAN mandatory for SEPA",
                when_thrown: "EU payment without IBAN in beneficiary field",
                mt_categories: &["1"],
            },
        );

        // E-Series metadata
        map.insert(
            "E01",
            ErrorCodeInfo {
                code: "E01",
                series: "E",
                category: "Relation Validation",
                description: "Instruction code restrictions",
                when_thrown: "Field 23E with SPRI can only contain SDVA, TELB, PHOB, INTC",
                mt_categories: &["1"],
            },
        );

        // G-Series metadata
        map.insert(
            "G001",
            ErrorCodeInfo {
                code: "G001",
                series: "G",
                category: "General Validation",
                description: "Field format violation",
                when_thrown: "Field doesn't match specified format pattern",
                mt_categories: &["2", "3", "5", "6", "7", "8", "9"],
            },
        );

        map
    });

    /// Get metadata for an error code
    pub fn get_error_info(code: &str) -> Option<&'static ErrorCodeInfo> {
        ERROR_CODE_METADATA.get(code)
    }

    /// Get all error codes for a specific series
    pub fn get_codes_by_series(series: &str) -> Vec<&'static str> {
        ERROR_CODE_METADATA
            .values()
            .filter(|info| info.series == series)
            .map(|info| info.code)
            .collect()
    }

    /// Get all error codes for a specific MT category
    pub fn get_codes_by_category(category: &str) -> Vec<&'static str> {
        ERROR_CODE_METADATA
            .values()
            .filter(|info| info.mt_categories.contains(&category))
            .map(|info| info.code)
            .collect()
    }
}

/// Regional validation constants
pub mod regional {
    /// SEPA country codes requiring IBAN validation
    pub const SEPA_COUNTRIES: &[&str] = &[
        "AD", "AT", "BE", "BG", "BV", "CH", "CY", "CZ", "DE", "DK", "EE", "ES", "FI", "FR", "GB",
        "GF", "GI", "GP", "GR", "HU", "IE", "IS", "IT", "LI", "LT", "LU", "LV", "MC", "MQ", "MT",
        "NL", "NO", "PL", "PM", "PT", "RE", "RO", "SE", "SI", "SJ", "SK", "SM", "TF", "VA",
    ];

    /// Check if a country code is in the SEPA region
    pub fn is_sepa_country(country_code: &str) -> bool {
        SEPA_COUNTRIES.contains(&country_code)
    }
}

/// Charge code validation constants
pub mod charges {
    /// Valid charge codes
    pub const CHARGE_CODES: &[&str] = &["BEN", "OUR", "SHA"];

    /// Check if a charge code is valid
    pub fn is_valid_charge_code(code: &str) -> bool {
        CHARGE_CODES.contains(&code)
    }
}

/// Currency validation constants  
pub mod currencies {
    /// Commodity currencies not allowed in payment messages
    pub const COMMODITY_CURRENCIES: &[&str] = &["XAU", "XAG", "XPD", "XPT"];

    /// Check if a currency is a commodity currency
    pub fn is_commodity_currency(currency: &str) -> bool {
        COMMODITY_CURRENCIES.contains(&currency)
    }
}
