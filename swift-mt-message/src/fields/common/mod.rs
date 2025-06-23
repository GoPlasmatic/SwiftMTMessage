pub mod account_field;
pub mod balance_field;
pub mod bic_field;
pub mod currency_field;
pub mod multiline_text_field;
pub mod name_field;
pub mod party_field;
pub mod reference_field;
pub mod summary_field;
pub mod text_field;

// Re-export all the lightweight generic field structures
pub use account_field::GenericAccountField;
pub use balance_field::GenericBalanceField;
pub use bic_field::{BIC, GenericBicField};
pub use currency_field::GenericCurrencyAmountField;
pub use multiline_text_field::{
    GenericMultiLine3x35, GenericMultiLine4x35, GenericMultiLine6x35, GenericMultiLine6x65,
    GenericMultiLine20x35, GenericMultiLineTextField,
};
pub use name_field::GenericNameAddressField;
pub use party_field::GenericPartyField;
pub use reference_field::GenericReferenceField;
pub use summary_field::GenericSummaryField;
pub use text_field::GenericTextField;

// Type aliases for specific balance field types
pub use balance_field::GenericBalanceField as Field60A; // Opening Balance (Intermediate)
pub use balance_field::GenericBalanceField as Field60F; // Opening Balance (Final/Booked)
pub use balance_field::GenericBalanceField as Field62A; // Closing Balance (Intermediate)
pub use balance_field::GenericBalanceField as Field62F; // Closing Balance (Final/Booked)
pub use balance_field::GenericBalanceField as Field64; // Closing Available Balance
pub use balance_field::GenericBalanceField as Field65; // Forward Available Balance
