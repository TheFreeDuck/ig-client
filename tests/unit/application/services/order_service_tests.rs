use ig_client::application::models::order::{
    ClosePositionRequest, CreateOrderRequest, Direction, OrderType, TimeInForce,
    UpdatePositionRequest,
};

use ig_client::session::interface::IgSession;

#[test]
fn test_create_order_request_market() {
    // Test the market constructor of CreateOrderRequest
    let order =
        CreateOrderRequest::market("OP.D.OTCDAX1.021100P.IP".to_string(), Direction::Buy, 1.0);

    // Verify that the fields were set correctly
    assert_eq!(order.epic, "OP.D.OTCDAX1.021100P.IP");
    assert!(matches!(order.direction, Direction::Buy));
    assert_eq!(order.size, 1.0);
    assert!(matches!(order.order_type, OrderType::Market));
    assert!(matches!(order.time_in_force, TimeInForce::FillOrKill));
}

#[test]
fn test_create_order_request_limit() {
    // Test the limit constructor of CreateOrderRequest
    let order = CreateOrderRequest::limit(
        "OP.D.OTCDAX1.021100P.IP".to_string(),
        Direction::Sell,
        1.0,
        1.2345,
    );

    // Verify that the fields were set correctly
    assert_eq!(order.epic, "OP.D.OTCDAX1.021100P.IP");
    assert!(matches!(order.direction, Direction::Sell));
    assert_eq!(order.size, 1.0);
    assert!(matches!(order.order_type, OrderType::Limit));
    assert!(matches!(
        order.time_in_force,
        TimeInForce::GoodTillCancelled
    ));
    assert_eq!(order.level, Some(1.2345));
}

#[test]
fn test_create_order_request_with_reference() {
    // Test the with_reference method
    let order =
        CreateOrderRequest::market("OP.D.OTCDAX1.021100P.IP".to_string(), Direction::Buy, 1.0)
            .with_reference("TEST_REF".to_string());

    // Verify deal_reference field is set correctly
    assert_eq!(order.deal_reference, Some("TEST_REF".to_string()));
}

#[test]
fn test_create_order_request_with_stop_loss() {
    // Test the with_stop_loss method
    let order =
        CreateOrderRequest::market("OP.D.OTCDAX1.021100P.IP".to_string(), Direction::Buy, 1.0)
            .with_stop_loss(1.2000);

    // Verify stop_level field is set correctly
    assert_eq!(order.stop_level, Some(1.2000));
}

#[test]
fn test_create_order_request_with_take_profit() {
    // Test the with_take_profit method
    let order =
        CreateOrderRequest::market("OP.D.OTCDAX1.021100P.IP".to_string(), Direction::Buy, 1.0)
            .with_take_profit(1.3000);

    // Verify limit_level field is set correctly
    assert_eq!(order.limit_level, Some(1.3000));
}

#[test]
fn test_close_position_request_market() {
    // Test the market constructor of ClosePositionRequest
    let request = ClosePositionRequest::market(
        "DEAL123".to_string(),
        Direction::Sell,
        1.0,
        "OP.D.OTCDAX1.021100P.IP".to_string(),
    );

    // Verify that the fields were set correctly
    assert_eq!(request.deal_id, "DEAL123");
    assert!(matches!(request.direction, Direction::Sell));
    assert_eq!(request.size, 1.0);
    assert!(matches!(request.order_type, OrderType::Market));
    assert!(matches!(request.time_in_force, TimeInForce::FillOrKill));
    assert_eq!(request.level, None);
}

#[test]
fn test_update_position_request() {
    // Create an update request
    let update = UpdatePositionRequest {
        stop_level: Some(1.2000),
        limit_level: Some(1.3000),
        trailing_stop: Some(false),
        trailing_stop_distance: Some(0.01),
    };

    // Verify that the fields were set correctly
    assert_eq!(update.stop_level, Some(1.2000));
    assert_eq!(update.limit_level, Some(1.3000));
    assert_eq!(update.trailing_stop, Some(false));
    assert_eq!(update.trailing_stop_distance, Some(0.01));
}

#[test]
fn test_order_service_config() {
    // Create a mock session
    let session = IgSession::new(
        "CST123".to_string(),
        "XST123".to_string(),
        "ACC123".to_string(),
    );

    // Verify session was created successfully
    assert_eq!(session.cst, "CST123");
    assert_eq!(session.token, "XST123");
    assert_eq!(session.account_id, "ACC123");
}
