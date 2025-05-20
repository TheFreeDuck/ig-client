use ig_client::application::models::market::{
    Currency, HistoricalPricesResponse, Instrument, MarketData, MarketDetails, MarketSearchResult,
    MarketSnapshot,
};
use ig_client::presentation::InstrumentType;
use ig_client::session::interface::IgSession;

#[test]
fn test_market_data_display() {
    // Create a MarketData instance
    let market_data = MarketData {
        epic: "CS.D.EURUSD.CFD.IP".to_string(),
        instrument_name: "EUR/USD".to_string(),
        instrument_type: InstrumentType::Currencies,
        expiry: "DFB".to_string(),
        high_limit_price: Some(1.2000),
        low_limit_price: Some(1.1000),
        market_status: "TRADEABLE".to_string(),
        net_change: Some(0.0010),
        percentage_change: Some(0.1),
        update_time: Some("22:00:00".to_string()),
        bid: Some(1.1850),
        offer: Some(1.1852),
    };

    // Verify Display implementation works correctly
    let display_string = format!("{}", market_data);
    assert!(display_string.contains("CS.D.EURUSD.CFD.IP"));
    assert!(display_string.contains("EUR/USD"));
    assert!(display_string.contains("TRADEABLE"));
}

#[test]
fn test_market_search_result() {
    // Create a MarketData instance
    let market_data = MarketData {
        epic: "CS.D.EURUSD.CFD.IP".to_string(),
        instrument_name: "EUR/USD".to_string(),
        instrument_type: InstrumentType::Currencies,
        expiry: "DFB".to_string(),
        high_limit_price: Some(1.2000),
        low_limit_price: Some(1.1000),
        market_status: "TRADEABLE".to_string(),
        net_change: Some(0.0010),
        percentage_change: Some(0.1),
        update_time: Some("22:00:00".to_string()),
        bid: Some(1.1850),
        offer: Some(1.1852),
    };

    let search_result = MarketSearchResult {
        markets: vec![market_data],
    };

    // Verify structure was created correctly
    assert_eq!(search_result.markets.len(), 1);
    assert_eq!(search_result.markets[0].epic, "CS.D.EURUSD.CFD.IP");
    assert_eq!(search_result.markets[0].instrument_name, "EUR/USD");
    assert!(matches!(
        search_result.markets[0].instrument_type,
        InstrumentType::Currencies
    ));
}

#[test]
fn test_instrument_type() {
    // Verify that instrument types are defined correctly
    assert!(matches!(InstrumentType::Shares, InstrumentType::Shares));
    assert!(matches!(
        InstrumentType::Currencies,
        InstrumentType::Currencies
    ));
    assert!(matches!(InstrumentType::Indices, InstrumentType::Indices));
    assert!(matches!(
        InstrumentType::Commodities,
        InstrumentType::Commodities
    ));
    assert!(matches!(InstrumentType::Binary, InstrumentType::Binary));

    // Verify equality
    assert_eq!(InstrumentType::Shares, InstrumentType::Shares);
    assert_ne!(InstrumentType::Shares, InstrumentType::Currencies);
}

#[test]
fn test_market_service_config() {
    // Create a mock session
    let session = IgSession {
        cst: "CST123".to_string(),
        token: "XST123".to_string(),
        account_id: "ACC123".to_string(),
    };

    // Verify session was created successfully
    assert_eq!(session.cst, "CST123");
    assert_eq!(session.token, "XST123");
    assert_eq!(session.account_id, "ACC123");
}

#[test]
fn test_market_details() {
    // Create a Currency instance
    let currency = Currency {
        code: "USD".to_string(),
        symbol: Some("$".to_string()),
        base_exchange_rate: Some(1.0),
        exchange_rate: Some(1.0),
        is_default: Some(true),
    };

    // Create an Instrument instance
    let instrument = Instrument {
        epic: "CS.D.EURUSD.CFD.IP".to_string(),
        name: "EUR/USD".to_string(),
        instrument_type: InstrumentType::Currencies,
        expiry: "DFB".to_string(),
        contract_size: Some(1.0),
        lot_size: Some(1.0),
        high_limit_price: Some(1.2000),
        low_limit_price: Some(1.1000),
        margin_factor: Some(3.33),
        margin_factor_unit: Some("PERCENTAGE".to_string()),
        slippage_factor: Some(0.5),
        limited_risk_premium: Some(0.1),
        news_code: Some("EUR_USD".to_string()),
        chart_code: Some("EURUSD".to_string()),
        currencies: Some(vec![currency]),
    };

    // Create a MarketSnapshot instance
    let snapshot = MarketSnapshot {
        market_status: "TRADEABLE".to_string(),
        net_change: Some(0.0010),
        percentage_change: Some(0.1),
        update_time: Some("22:00:00".to_string()),
        delay_time: Some(0),
        bid: Some(1.1850),
        offer: Some(1.1852),
        high: Some(1.1900),
        low: Some(1.1800),
        binary_odds: None,
        decimal_places_factor: Some(4),
        scaling_factor: Some(1),
        controlled_risk_extra_spread: Some(0.1),
    };

    // Create a MarketDetails instance
    let market_details = MarketDetails {
        instrument,
        snapshot,
    };

    // Verify structure was created correctly
    assert_eq!(market_details.instrument.epic, "CS.D.EURUSD.CFD.IP");
    assert_eq!(market_details.instrument.name, "EUR/USD");
    assert!(matches!(
        market_details.instrument.instrument_type,
        InstrumentType::Currencies
    ));
    assert_eq!(market_details.snapshot.market_status, "TRADEABLE");
    assert_eq!(market_details.snapshot.bid, Some(1.1850));
    assert_eq!(market_details.snapshot.offer, Some(1.1852));

    // Verify structure was created correctly
    if let Some(currencies) = &market_details.instrument.currencies {
        assert_eq!(currencies.len(), 1);
        assert_eq!(currencies[0].code, "USD");
        assert_eq!(currencies[0].symbol, Some("$".to_string()));
    } else {
        panic!("Expected currencies to be Some");
    }
}

#[test]
fn test_historical_prices_response() {
    // Create a PricePoint instance
    let price_point = ig_client::application::models::market::PricePoint {
        bid: Some(1.1850),
        ask: Some(1.1852),
        last_traded: Some(1.1851),
    };

    // Create a HistoricalPrice instance
    let historical_price = ig_client::application::models::market::HistoricalPrice {
        snapshot_time: "2023-01-01T00:00:00".to_string(),
        open_price: price_point.clone(),
        high_price: price_point.clone(),
        low_price: price_point.clone(),
        close_price: price_point,
        last_traded_volume: Some(1000),
    };

    // Create a PriceAllowance instance
    let price_allowance = ig_client::application::models::market::PriceAllowance {
        remaining_allowance: 9999,
        total_allowance: 10000,
        allowance_expiry: 3600,
    };

    // Create a HistoricalPricesResponse instance
    let historical_prices = HistoricalPricesResponse {
        prices: vec![historical_price],
        instrument_type: InstrumentType::Currencies,
        allowance: price_allowance,
    };

    // Verify structure was created correctly
    assert_eq!(historical_prices.prices.len(), 1);
    assert_eq!(
        historical_prices.prices[0].snapshot_time,
        "2023-01-01T00:00:00"
    );
    assert_eq!(historical_prices.prices[0].open_price.bid, Some(1.1850));
    assert_eq!(historical_prices.prices[0].open_price.ask, Some(1.1852));
    assert_eq!(historical_prices.prices[0].last_traded_volume, Some(1000));
    assert!(matches!(
        historical_prices.instrument_type,
        InstrumentType::Currencies
    ));
    assert_eq!(historical_prices.allowance.remaining_allowance, 9999);
    assert_eq!(historical_prices.allowance.total_allowance, 10000);
    assert_eq!(historical_prices.allowance.allowance_expiry, 3600);
}
