use ig_client::application::models::transaction::RawTransaction;
use serde_json;

// Sample JSON for a simple transaction
fn sample_raw_transaction_json() -> &'static str {
    "{\"date\":\"2023-01-01T10:00:00\",\"dateUtc\":\"2023-01-01T09:00:00\",\"openDateUtc\":\"2023-01-01T08:00:00\",\"instrumentName\":\"CS.D.EURUSD.CFD.IP\",\"period\":\"DFB\",\"profitAndLoss\":\"10.50\",\"transactionType\":\"DEAL\",\"reference\":\"REF123\",\"openLevel\":\"1.1000\",\"closeLevel\":\"1.1010\",\"size\":\"1.0\",\"currency\":\"EUR\",\"cashTransaction\":false}"
}

// Sample JSON with matching dateUtc equal to date
fn sample_raw_transaction_json2() -> &'static str {
    "{\"date\":\"2023-01-01T10:00:00\",\"dateUtc\":\"2023-01-01T10:00:00\",\"openDateUtc\":\"2023-01-01T09:00:00\",\"instrumentName\":\"CS.D.EURUSD.CFD.IP\",\"period\":\"DFB\",\"profitAndLoss\":\"10.50\",\"transactionType\":\"DEAL\",\"reference\":\"REF123\",\"openLevel\":\"1.1000\",\"closeLevel\":\"1.1010\",\"size\":\"1.0\",\"currency\":\"EUR\",\"cashTransaction\":false}"
}

// Sample JSON with negative P&L and different instrument
fn sample_raw_transaction_json3() -> &'static str {
    "{\"date\":\"2023-01-01T10:00:00\",\"dateUtc\":\"2023-01-01T10:00:00\",\"openDateUtc\":\"2023-01-01T09:00:00\",\"instrumentName\":\"CS.D.AAPL.CFD.IP\",\"period\":\"DFB\",\"profitAndLoss\":\"-5.25\",\"transactionType\":\"DEAL\",\"reference\":\"OPT123\",\"openLevel\":\"150.00\",\"closeLevel\":\"145.00\",\"size\":\"1.0\",\"currency\":\"USD\",\"cashTransaction\":false}"
}

#[test]
fn test_raw_transaction_display_and_serialization() {
    let json = sample_raw_transaction_json();
    // Deserialize from JSON
    let raw_tx: RawTransaction = serde_json::from_str(json).expect("Failed to parse JSON");
    // Display implementation should produce valid JSON
    let display_str = raw_tx.to_string();
    assert!(display_str.contains("\"instrumentName\":\"CS.D.EURUSD.CFD.IP\""));
    assert!(display_str.contains("\"transactionType\":\"DEAL\""));
    // Round-trip serialization
    let serialized = serde_json::to_string(&raw_tx).expect("Failed to serialize RawTransaction");
    assert!(serialized.contains("\"reference\":\"REF123\""));
    // Deserialize again and compare display outputs
    let raw_tx2: RawTransaction = serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");
    assert_eq!(display_str, raw_tx2.to_string());
}

#[test]
fn test_transaction_creation() {
    let json = sample_raw_transaction_json2();
    // Deserialize from JSON
    let raw_tx: RawTransaction = serde_json::from_str(json).expect("Failed to parse JSON");
    // Test the display implementation
    let display_str = raw_tx.to_string();
    assert!(display_str.contains("\"instrumentName\":\"CS.D.EURUSD.CFD.IP\""));
    assert!(display_str.contains("\"transactionType\":\"DEAL\""));
    assert!(display_str.contains("\"reference\":\"REF123\""));
}

#[test]
fn test_raw_transaction_serialization() {
    let json = sample_raw_transaction_json3();
    // Deserialize from JSON
    let raw_tx: RawTransaction = serde_json::from_str(json).expect("Failed to parse JSON");
    // Test serialization to JSON
    let serialized = serde_json::to_string(&raw_tx).expect("Failed to serialize RawTransaction");
    assert!(serialized.contains("\"instrumentName\":\"CS.D.AAPL.CFD.IP\""));
    assert!(serialized.contains("\"transactionType\":\"DEAL\""));
    assert!(serialized.contains("\"reference\":\"OPT123\""));
    // Test deserialization back to RawTransaction
    let deserialized: RawTransaction = serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");
    // Convert back to JSON to verify the data was preserved
    let serialized_again = serde_json::to_string(&deserialized).expect("Failed to serialize deserialized RawTransaction");
    assert_eq!(serialized, serialized_again);
}
