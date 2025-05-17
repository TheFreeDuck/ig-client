use ig_client::utils::finance::{calculate_pip_value, calculate_margin, calculate_profit_loss};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[test]
fn test_calculate_pip_value() {
    // Test for 4-digit currency pairs (e.g., EUR/USD)
    let result = calculate_pip_value(dec!(100000), dec!(1.2000), 4);
    assert_eq!(result, dec!(10));  // 100,000 units * 0.0001 = 10 units of quote currency
    
    // Test for 2-digit currency pairs (e.g., USD/JPY)
    let result = calculate_pip_value(dec!(100000), dec!(110.00), 2);
    assert_eq!(result, dec!(1000));  // 100,000 units * 0.01 = 1000 units of quote currency
    
    // Test for 5-digit currency pairs (e.g., some brokers offer EUR/USD with 5 digits)
    let result = calculate_pip_value(dec!(100000), dec!(1.20000), 5);
    assert_eq!(result, dec!(1));  // 100,000 units * 0.00001 = 1 unit of quote currency
    
    // Test with smaller position size
    let result = calculate_pip_value(dec!(10000), dec!(1.2000), 4);
    assert_eq!(result, dec!(1));  // 10,000 units * 0.0001 = 1 unit of quote currency
    
    // Test with zero position size
    let result = calculate_pip_value(dec!(0), dec!(1.2000), 4);
    assert_eq!(result, dec!(0));  // 0 units * 0.0001 = 0 units of quote currency
}

#[test]
fn test_calculate_margin() {
    // Test with 50:1 leverage (2% margin requirement)
    let result = calculate_margin(dec!(100000), dec!(1.2000), dec!(0.02));
    assert_eq!(result, dec!(2400));  // 100,000 units * 1.2000 * 0.02 = 2,400 units of base currency
    
    // Test with 100:1 leverage (1% margin requirement)
    let result = calculate_margin(dec!(100000), dec!(1.2000), dec!(0.01));
    assert_eq!(result, dec!(1200));  // 100,000 units * 1.2000 * 0.01 = 1,200 units of base currency
    
    // Test with 20:1 leverage (5% margin requirement)
    let result = calculate_margin(dec!(100000), dec!(1.2000), dec!(0.05));
    assert_eq!(result, dec!(6000));  // 100,000 units * 1.2000 * 0.05 = 6,000 units of base currency
    
    // Test with smaller position size
    let result = calculate_margin(dec!(10000), dec!(1.2000), dec!(0.02));
    assert_eq!(result, dec!(240));  // 10,000 units * 1.2000 * 0.02 = 240 units of base currency
    
    // Test with zero position size
    let result = calculate_margin(dec!(0), dec!(1.2000), dec!(0.02));
    assert_eq!(result, dec!(0));  // 0 units * 1.2000 * 0.02 = 0 units of base currency
}

#[test]
fn test_calculate_profit_loss() {
    // Test profit for long position
    let result = calculate_profit_loss(true, dec!(100000), dec!(1.2000), dec!(1.2100));
    assert_eq!(result, dec!(1000));  // 100,000 units * (1.2100 - 1.2000) = 1,000 units of quote currency
    
    // Test loss for long position
    let result = calculate_profit_loss(true, dec!(100000), dec!(1.2100), dec!(1.2000));
    assert_eq!(result, dec!(-1000));  // 100,000 units * (1.2000 - 1.2100) = -1,000 units of quote currency
    
    // Test profit for short position
    let result = calculate_profit_loss(false, dec!(100000), dec!(1.2100), dec!(1.2000));
    assert_eq!(result, dec!(1000));  // 100,000 units * (1.2100 - 1.2000) = 1,000 units of quote currency
    
    // Test loss for short position
    let result = calculate_profit_loss(false, dec!(100000), dec!(1.2000), dec!(1.2100));
    assert_eq!(result, dec!(-1000));  // 100,000 units * (1.2000 - 1.2100) = -1,000 units of quote currency
    
    // Test with smaller position size
    let result = calculate_profit_loss(true, dec!(10000), dec!(1.2000), dec!(1.2100));
    assert_eq!(result, dec!(100));  // 10,000 units * (1.2100 - 1.2000) = 100 units of quote currency
    
    // Test with zero position size
    let result = calculate_profit_loss(true, dec!(0), dec!(1.2000), dec!(1.2100));
    assert_eq!(result, dec!(0));  // 0 units * (1.2100 - 1.2000) = 0 units of quote currency
    
    // Test with equal entry and exit prices
    let result = calculate_profit_loss(true, dec!(100000), dec!(1.2000), dec!(1.2000));
    assert_eq!(result, dec!(0));  // 100,000 units * (1.2000 - 1.2000) = 0 units of quote currency
}
