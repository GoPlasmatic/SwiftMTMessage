use quote::quote;
use syn::DeriveInput;

/// Generate business logic methods based on the field pattern
pub fn generate_business_logic_methods(
    input: &DeriveInput,
    format_spec: &str,
) -> proc_macro2::TokenStream {
    let _struct_name = &input.ident;

    if format_spec == "3!n3!a15d" {
        // Business logic for Field90C/90D pattern (entry count + currency + amount)
        quote! {
            /// Get human-readable description
            pub fn description(&self) -> String {
                format!(
                    "Sum of {} entries: {} {}",
                    self.entry_count, self.currency, self.raw_amount
                )
            }
        }
    } else if format_spec == "3!n" {
        // Business logic for Field12 pattern (message type)
        quote! {
            /// Check if this is a request for a customer statement
            pub fn is_customer_statement_request(&self) -> bool {
                matches!(self.message_type.as_str(), "940" | "950")
            }

            /// Check if this is a request for a balance report
            pub fn is_balance_report_request(&self) -> bool {
                self.message_type == "941"
            }

            /// Check if this is a request for an interim transaction report
            pub fn is_interim_report_request(&self) -> bool {
                self.message_type == "942"
            }

            /// Returns a description of the requested message type
            pub fn get_description(&self) -> &'static str {
                match self.message_type.as_str() {
                    "940" => "Customer Statement Message",
                    "941" => "Balance Report",
                    "942" => "Interim Transaction Report",
                    "950" => "Customer Statement Message (Consolidated)",
                    _ => "Unknown Message Type",
                }
            }

            /// Get human-readable description
            pub fn description(&self) -> String {
                format!("Message Type: {} ({})", self.message_type, self.get_description())
            }
        }
    } else if format_spec == "6!n" {
        // Business logic for Field30 pattern (date only)
        quote! {
            /// Check if the execution date is today
            pub fn is_today(&self) -> bool {
                let today = chrono::Utc::now().naive_utc().date();
                self.date == today
            }

            /// Format the date in a human-readable format
            pub fn format_readable(&self) -> String {
                self.date.format("%Y-%m-%d").to_string()
            }

            /// Get year component
            pub fn year(&self) -> i32 {
                self.date.year()
            }

            /// Get month component
            pub fn month(&self) -> u32 {
                self.date.month()
            }

            /// Get day component
            pub fn day(&self) -> u32 {
                self.date.day()
            }

            /// Convert to NaiveDate object (for compatibility)
            pub fn to_naive_date(&self) -> Option<chrono::NaiveDate> {
                Some(self.date)
            }

            /// Get human-readable description
            pub fn description(&self) -> String {
                format!("Execution Date: {}", self.format_readable())
            }
        }
    } else if format_spec == "6!n3!a15d" {
        // Business logic for Field32A pattern (existing implementation)
        quote! {
            /// Get decimal places for this currency
            pub fn decimal_places(&self) -> u8 {
                match self.currency.as_str() {
                    "JPY" | "KRW" | "VND" => 0,
                    "BHD" | "IQD" | "JOD" | "KWD" => 3,
                    _ => 2,
                }
            }

            /// Get comprehensive description
            pub fn description(&self) -> String {
                format!(
                    "{} {} value date ({})",
                    self.currency,
                    self.raw_amount,
                    self.value_date.format("%Y-%m-%d"),
                )
            }
        }
    } else if format_spec == "1!a[N]12d" {
        // Business logic for Field37H pattern (interest rate)
        quote! {
            /// Format rate for SWIFT output (with comma as decimal separator)
            pub fn format_rate(rate: f64) -> String {
                format!("{:.3}", rate).replace('.', ",")
            }

            /// Get human-readable description
            pub fn description(&self) -> String {
                let sign = if self.is_negative { "-" } else { "" };
                format!("{}: {}{}%", self.rate_indicator, sign, self.raw_rate)
            }

            /// Get formatted rate with percentage sign
            pub fn formatted_rate(&self) -> String {
                let sign = if self.is_negative { "-" } else { "" };
                format!("{}{}%", sign, self.raw_rate)
            }
        }
    } else {
        quote! {
            /// Basic description method
            pub fn description(&self) -> String {
                format!("SWIFT field with format: {}", #format_spec)
            }
        }
    }
} 