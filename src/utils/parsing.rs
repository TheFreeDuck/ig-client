use crate::error::AppError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct InstrumentInfo {
    pub underlying: Option<String>,
    pub strike: Option<f64>,
    pub option_type: Option<String>,
    pub is_option: bool,
}

/// Parse the instrument name string to extract trading instrument details
pub fn parse_instrument_name(instrument_name: &str) -> Result<InstrumentInfo, AppError> {
    let mut info = InstrumentInfo::default();

    // Skip known administrative entries that are not options
    if instrument_name.contains("Cargo por tarifa")
        || instrument_name.contains("Daily Admin Fee")
        || instrument_name.contains("Fee")
        || (instrument_name.starts_with("End ") && !instrument_name.starts_with("End of Month"))
        || instrument_name.contains("Funds")
        || instrument_name.contains("Funds Transfer")
    {
        return Ok(info); // Return default with is_option = false
    }

    // Check if it's an option by looking for CALL/PUT or Call/Put
    let option_type_re = Regex::new(r"(?i)(CALL|PUT|Call|Put)\b").unwrap();
    if let Some(cap) = option_type_re.captures(instrument_name) {
        let opt_type = cap[1].to_uppercase();
        info.option_type = Some(opt_type);
        info.is_option = true;
    } else {
        // Not an option
        return Ok(info);
    }

    // Extract strike price - look for a number before or after CALL/PUT
    let strike_re = Regex::new(r"\b(\d+(?:\.\d+)?)\s*(?:CALL|PUT|Call|Put)|(?:CALL|PUT|Call|Put)\s*(?:a\s*)?(\d+(?:\.\d+)?)").unwrap();
    if let Some(cap) = strike_re.captures(instrument_name) {
        // The regex has two capture groups, check which one matched
        let strike_str = cap.get(1).or_else(|| cap.get(2)).unwrap().as_str();
        if let Ok(strike_val) = strike_str.parse::<f64>() {
            info.strike = Some(strike_val);
        }
    }

    // Extract underlying - this is the most complex part as formats vary
    extract_underlying(&mut info, instrument_name);

    Ok(info)
}

/// Helper function to extract the underlying from the instrument name
fn extract_underlying(info: &mut InstrumentInfo, instrument_name: &str) {
    // Create a HashMap with all known patterns for easier maintenance
    let known_patterns: HashMap<&str, &str> = [
        // EU50 patterns
        ("Eu Stocks 50", "EU50"),
        ("EU Stocks 50", "EU50"),
        ("EU50", "EU50"),
        // GER40 patterns
        ("Germany 40", "GER40"),
        ("GER40", "GER40"),
        // US500 patterns
        ("US 500", "US500"),
        ("US500", "US500"),
        // USTECH patterns
        ("US Tech 100", "USTECH"),
        ("USTECH", "USTECH"),
        // GOLD patterns
        ("Gold Futures", "GOLD"),
        ("Gold (", "GOLD"),
        ("GOLD", "GOLD"),
        ("Gold", "GOLD"),
        // SILVER patterns
        ("Silver Futures", "SILVER"),
        ("Silver (", "SILVER"),
        ("SILVER", "SILVER"),
        ("Silver", "SILVER"),
        // NATGAS patterns
        ("Natural Gas", "NATGAS"),
        ("NATGAS", "NATGAS"),
        // UK100 patterns
        ("FTSE", "UK100"),
        ("UK100", "UK100"),
        // US30 patterns
        ("Wall Street", "US30"),
        ("US30", "US30"),
        // OIL patterns
        ("Crude", "OIL"),
        ("Oil", "OIL"),
        ("OIL", "OIL"),
        // FRA40 patterns
        ("France 40", "FRA40"),
        ("FRA40", "FRA40"),
        // BITCOIN patterns
        ("Bitcoin", "BITCOIN"),
        ("BITCOIN", "BITCOIN"),
        // ETHEREUM patterns
        ("Ether", "ETHEREUM"),
        ("ETHEREUM", "ETHEREUM"),
        // PAYPAL patterns
        ("Paypal", "PAYPAL"),
        ("PAYPAL", "PAYPAL"),
    ]
    .iter()
    .cloned()
    .collect();

    // Special case for "End of Month" at the beginning
    if instrument_name.starts_with("End of Month") {
        let parts: Vec<&str> = instrument_name.split_whitespace().collect();
        if parts.len() >= 5 && parts[3] == "Germany" && parts[4] == "40" {
            info.underlying = Some("GER40".to_string());
            return;
        } else if parts.len() >= 6 && parts[3] == "EU" && parts[4] == "Stocks" && parts[5] == "50" {
            info.underlying = Some("EU50".to_string());
            return;
        }
    }

    // Special case for "Option premium" entries
    if instrument_name.starts_with("Option premium") {
        // Extract the underlying after "received" or "paid"
        let option_premium_re = Regex::new(r"Option premium (?:received|paid) (.*?)(?:\s+\d+(?:\.\d+)?|\s+\(Wed\)\d+|\s+\(End of Month\)\d+|\s+\(Mon\)\d+|\s+\(£\d+\)|\s+\(E\d+\)|\s+\(\$\d+\))\s*(?:CALL|PUT)").unwrap();
        if let Some(cap) = option_premium_re.captures(instrument_name) {
            let underlying_text = cap.get(1).unwrap().as_str().trim();

            // Use the same known_patterns HashMap to map the underlying
            for (pattern, standard_name) in &known_patterns {
                if underlying_text.contains(pattern) {
                    info.underlying = Some(standard_name.to_string());
                    return;
                }
            }

            // If no known pattern matches, use the extracted text as-is
            info.underlying = Some(underlying_text.to_string());
            return;
        }
    }

    // Special case for barrier options
    if instrument_name.contains("Barrier Call") || instrument_name.contains("Barrier Put") {
        if instrument_name.starts_with("Bitcoin") {
            info.underlying = Some("BITCOIN".to_string());
            return;
        } else if instrument_name.starts_with("Ether") {
            info.underlying = Some("ETHEREUM".to_string());
            return;
        }
    }

    // Try to match against the known_patterns for the entire instrument name
    for (pattern, standard_name) in &known_patterns {
        if instrument_name.contains(pattern) {
            info.underlying = Some(standard_name.to_string());
            return;
        }
    }

    // If no pattern matches, use the first part of the name
    let parts: Vec<&str> = instrument_name.split_whitespace().collect();
    if !parts.is_empty() {
        // If it starts with "Daily" or "Weekly", use the next word
        if parts[0] == "Daily" || parts[0] == "Weekly" {
            if parts.len() > 1 {
                info.underlying = Some(parts[1].to_string());
            }
        } else {
            // Otherwise use the first word
            info.underlying = Some(parts[0].to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    /// Parse the instrument name from a JSON string to extract trading instrument details
    pub fn parse_instrument_from_json(json_str: &str) -> Result<InstrumentInfo, AppError> {
        // Parse the JSON string
        let json_value: Value = match serde_json::from_str(json_str) {
            Ok(value) => value,
            Err(e) => return Err(AppError::SerializationError(e.to_string())),
        };

        // Extract the instrumentName field
        let instrument_name = match json_value.get("instrumentName") {
            Some(Value::String(name)) => name,
            _ => {
                return Err(AppError::SerializationError(
                    "Missing or invalid instrumentName field".to_string(),
                ));
            }
        };

        parse_instrument_name(instrument_name)
    }

    #[test]
    fn test_barrier_option() {
        let json = r#"{"instrumentName":"Bitcoin Barrier Call a 69650 COMM DIAAAANMBDWXTAS Tipo de cambio 0.9330"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("BITCOIN".to_string()));
        assert_eq!(info.strike, Some(69650.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_ether_barrier() {
        let json = r#"{"instrumentName":"Ether Barrier Call a 3852 COMM DIAAAANSKFGLDAQ Tipo de cambio 0.9587"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("ETHEREUM".to_string()));
        assert_eq!(info.strike, Some(3852.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_regular_option() {
        let json =
            r#"{"instrumentName":"Daily Eu Stocks 50 5184 CALL (EUR1) COMM DIAAAAN2FL4MTA6"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("EU50".to_string()));
        assert_eq!(info.strike, Some(5184.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_fee_entry() {
        let json = r#"{"instrumentName":"Cargo por tarifa de los Gráficos en April 25"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert!(!info.is_option);
        assert_eq!(info.underlying, None);
    }

    #[test]
    fn test_financing_adjustment() {
        let json = r#"{"instrumentName":"Daily Financing Adjustment - Bitcoin Barrier Call for 1 day USD converted at 0.9285"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert!(info.is_option);
    }

    #[test]
    fn test_weekly_option() {
        let json = r#"{"instrumentName":"Weekly Germany 40 (Wed)19900 PUT COMM DIAAAAPDSPMDQAY"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("GER40".to_string()));
        assert_eq!(info.strike, Some(19900.0));
        assert_eq!(info.option_type, Some("PUT".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_option_premium() {
        let json = r#"{"instrumentName":"Option premium received US 500 6040 CALL ($1) Tipo de cambio 0.8947665"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("US500".to_string()));
        assert_eq!(info.strike, Some(6040.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_option_premium_germany() {
        let json =
            r#"{"instrumentName":"Option premium received Weekly Germany 40 (Wed)23450 PUT"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("GER40".to_string()));
        assert_eq!(info.strike, Some(23450.0));
        assert_eq!(info.option_type, Some("PUT".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_end_of_month_option() {
        let json = r#"{"instrumentName":"Option premium paid End of Month Germany 40 23500 CALL"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("GER40".to_string()));
        assert_eq!(info.strike, Some(23500.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_weekly_us_tech_option() {
        let json =
            r#"{"instrumentName":"Option premium received Weekly US Tech 100 (Mon) 21550 PUT"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("USTECH".to_string()));
        assert_eq!(info.strike, Some(21550.0));
        assert_eq!(info.option_type, Some("PUT".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_oil_weekly_option() {
        let json = r#"{"instrumentName":"Option premium received Oil Weekly (Dec Fut) 7150 CALL"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("OIL".to_string()));
        assert_eq!(info.strike, Some(7150.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_gold_future_option() {
        let json =
            r#"{"instrumentName":"Option premium paid Weekly Gold (Feb Future) 2650 PUT ($1)"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("GOLD".to_string()));
        assert_eq!(info.strike, Some(2650.0));
        assert_eq!(info.option_type, Some("PUT".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_ge40_call_option() {
        let json =
            r#"{"instrumentName":"End of Month Germany 40 22800 CALL COMM DIAAAAPBC65ZQAP"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("GER40".to_string()));
        assert_eq!(info.strike, Some(22800.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_ge40_put_option() {
        let json = r#"{"instrumentName":"End of Month Germany 40 22000 PUT COMM DIAAAAPAWDLTNAB"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("GER40".to_string()));
        assert_eq!(info.strike, Some(22000.0));
        assert_eq!(info.option_type, Some("PUT".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_ftse_option() {
        let json = r#"{"instrumentName":"Option premium paid FTSE 8250 CALL (£1) Tipo de cambio 1.17471"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert_eq!(info.underlying, Some("UK100".to_string()));
        assert_eq!(info.strike, Some(8250.0));
        assert_eq!(info.option_type, Some("CALL".to_string()));
        assert!(info.is_option);
    }

    #[test]
    fn test_funds_transfer() {
        let json = r#"{"instrumentName":"Funds Transfer from Barreras y Opciones"}"#;
        let info = parse_instrument_from_json(json).unwrap();
        assert!(!info.is_option);
        assert_eq!(info.underlying, None);
    }
}
