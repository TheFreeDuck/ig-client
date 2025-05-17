use ig_client::utils::transactions::{
    format_transaction_id, parse_transaction_id, extract_transaction_type,
    extract_transaction_date, extract_transaction_instrument
};
use chrono::NaiveDate;

#[test]
fn test_format_transaction_id() {
    // Test formatting with all components
    let result = format_transaction_id("DEAL", "2023-05-15", "EURUSD", "12345");
    assert_eq!(result, "DEAL|2023-05-15|EURUSD|12345");
    
    // Test formatting with empty components
    let result = format_transaction_id("DEAL", "", "", "12345");
    assert_eq!(result, "DEAL|||12345");
}

#[test]
fn test_parse_transaction_id() {
    // Test parsing a valid transaction ID
    let result = parse_transaction_id("DEAL|2023-05-15|EURUSD|12345");
    assert_eq!(result, ("DEAL", "2023-05-15", "EURUSD", "12345"));
    
    // Test parsing a transaction ID with missing components
    let result = parse_transaction_id("DEAL|||12345");
    assert_eq!(result, ("DEAL", "", "", "12345"));
    
    // Test parsing a transaction ID with extra components
    let result = parse_transaction_id("DEAL|2023-05-15|EURUSD|12345|EXTRA");
    assert_eq!(result, ("DEAL", "2023-05-15", "EURUSD", "12345|EXTRA"));
    
    // Test parsing an empty transaction ID
    let result = parse_transaction_id("");
    assert_eq!(result, ("", "", "", ""));
    
    // Test parsing a transaction ID without separators
    let result = parse_transaction_id("DEAL");
    assert_eq!(result, ("DEAL", "", "", ""));
}

#[test]
fn test_extract_transaction_type() {
    // Test extracting from a valid transaction ID
    let result = extract_transaction_type("DEAL|2023-05-15|EURUSD|12345");
    assert_eq!(result, "DEAL");
    
    // Test extracting from a transaction ID with missing components
    let result = extract_transaction_type("DEAL|||12345");
    assert_eq!(result, "DEAL");
    
    // Test extracting from an empty transaction ID
    let result = extract_transaction_type("");
    assert_eq!(result, "");
    
    // Test extracting from a transaction ID without separators
    let result = extract_transaction_type("DEAL");
    assert_eq!(result, "DEAL");
}

#[test]
fn test_extract_transaction_date() {
    // Test extracting from a valid transaction ID
    let result = extract_transaction_date("DEAL|2023-05-15|EURUSD|12345");
    assert!(result.is_some());
    let date = result.unwrap();
    assert_eq!(date, NaiveDate::from_ymd_opt(2023, 5, 15).unwrap());
    
    // Test extracting from a transaction ID with invalid date
    let result = extract_transaction_date("DEAL|invalid-date|EURUSD|12345");
    assert!(result.is_none());
    
    // Test extracting from a transaction ID with missing date
    let result = extract_transaction_date("DEAL||EURUSD|12345");
    assert!(result.is_none());
    
    // Test extracting from an empty transaction ID
    let result = extract_transaction_date("");
    assert!(result.is_none());
}

#[test]
fn test_extract_transaction_instrument() {
    // Test extracting from a valid transaction ID
    let result = extract_transaction_instrument("DEAL|2023-05-15|EURUSD|12345");
    assert_eq!(result, "EURUSD");
    
    // Test extracting from a transaction ID with missing instrument
    let result = extract_transaction_instrument("DEAL|2023-05-15||12345");
    assert_eq!(result, "");
    
    // Test extracting from an empty transaction ID
    let result = extract_transaction_instrument("");
    assert_eq!(result, "");
    
    // Test extracting from a transaction ID with special instrument name
    let result = extract_transaction_instrument("DEAL|2023-05-15|EUR/USD|12345");
    assert_eq!(result, "EUR/USD");
    
    // Test extracting from a transaction ID with instrument name containing spaces
    let result = extract_transaction_instrument("DEAL|2023-05-15|US 500|12345");
    assert_eq!(result, "US 500");
}
