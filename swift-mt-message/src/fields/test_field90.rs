#[cfg(test)]
mod tests {
    use crate::fields::Field90D;
    use crate::SwiftField;

    #[test]
    fn test_field90d_parsing() {
        let value = "2GBP250050";
        match Field90D::parse(value) {
            Ok(field) => {
                println!("Parsed successfully!");
                println!("  Number: {}", field.number);
                println!("  Currency: {}", field.currency);
                println!("  Amount: {}", field.amount);
                assert_eq!(field.number, 2);
                assert_eq!(field.currency, "GBP");
                assert_eq!(field.amount, 250050.0);
            }
            Err(e) => {
                panic!("Failed to parse Field90D: {:?}", e);
            }
        }
    }
}