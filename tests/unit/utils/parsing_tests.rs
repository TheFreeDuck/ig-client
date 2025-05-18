use ig_client::utils::parsing::parse_instrument_name;

#[test]
fn test_parse_instrument_name() {
    // Test normal instrument name (not an option)
    let result = parse_instrument_name("EURUSD").unwrap();
    assert!(!result.is_option);
    assert_eq!(result.underlying, None);

    // Test option instrument name
    let result = parse_instrument_name("Option premium paid FTSE 8250 CALL").unwrap();
    assert!(result.is_option);
    assert_eq!(result.underlying, Some("UK100".to_string()));
    assert_eq!(result.strike, Some(8250.0));
    assert_eq!(result.option_type, Some("CALL".to_string()));

    // Test another option type
    let result = parse_instrument_name("Germany 40 23500 PUT").unwrap();
    assert!(result.is_option);
    assert_eq!(result.underlying, Some("GER40".to_string()));
    assert_eq!(result.strike, Some(23500.0));
    assert_eq!(result.option_type, Some("PUT".to_string()));
}
