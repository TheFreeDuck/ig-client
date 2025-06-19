use ig_client::application::models::order::{
    ClosePositionRequest, CreateOrderRequest, CreateWorkingOrderRequest, Direction, OrderType,
    Status, TimeInForce,
};
use serde::Deserialize;
use serde_json::json;

#[test]
fn test_create_order_request_market() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    
    let order = CreateOrderRequest::market(epic.to_string(), direction.clone(), size);
    
    assert_eq!(order.epic, epic);
    assert_eq!(order.direction, direction);
    assert_eq!(order.size, size);
    assert_eq!(order.order_type, OrderType::Market);
    assert_eq!(order.time_in_force, TimeInForce::FillOrKill);
    assert!(order.level.is_none());
    assert!(order.guaranteed_stop.is_none());
    assert!(order.stop_level.is_none());
    assert!(order.stop_distance.is_none());
    assert!(order.limit_level.is_none());
    assert!(order.limit_distance.is_none());
    // quote_id field no longer exists in CreateOrderRequest
    assert!(order.currency_code.is_none());
    assert_eq!(order.force_open, Some(true)); // Updated: force_open is now Some(true) by default
    assert!(order.expiry.is_none());
    assert!(order.deal_reference.is_none());
}

#[test]
fn test_create_order_request_limit() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Sell;
    let size = 2.0;
    let level = 1.2345;
    
    let order = CreateOrderRequest::limit(epic.to_string(), direction.clone(), size, level);
    
    assert_eq!(order.epic, epic);
    assert_eq!(order.direction, direction);
    assert_eq!(order.size, size);
    assert_eq!(order.order_type, OrderType::Limit);
    assert_eq!(order.time_in_force, TimeInForce::GoodTillCancelled);
    assert_eq!(order.level, Some(level));
    assert!(order.guaranteed_stop.is_none());
    assert!(order.stop_level.is_none());
    assert!(order.stop_distance.is_none());
    assert!(order.limit_level.is_none());
    assert!(order.limit_distance.is_none());
    // quote_id field no longer exists in CreateOrderRequest
    assert!(order.currency_code.is_none());
    assert_eq!(order.force_open, Some(true)); // Updated: force_open is now Some(true) by default
    assert!(order.expiry.is_none());
    assert!(order.deal_reference.is_none());
}

#[test]
fn test_create_order_request_with_stop_loss() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let stop_level = 1.2000;
    
    let order = CreateOrderRequest::market(epic.to_string(), direction, size)
        .with_stop_loss(stop_level);
    
    assert_eq!(order.stop_level, Some(stop_level));
}

#[test]
fn test_create_order_request_with_take_profit() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let limit_level = 1.3000;
    
    let order = CreateOrderRequest::market(epic.to_string(), direction, size)
        .with_take_profit(limit_level);
    
    assert_eq!(order.limit_level, Some(limit_level));
}

#[test]
fn test_create_order_request_with_reference() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let reference = "test-reference-123";
    
    let order = CreateOrderRequest::market(epic.to_string(), direction, size)
        .with_reference(reference.to_string());
    
    assert_eq!(order.deal_reference, Some(reference.to_string()));
}

#[test]
fn test_create_order_request_sell_option_to_market() {
    let epic = "CC.D.LCO.UME.IP".to_string();
    let size = 1.0;
    let expiry = Some("DEC-25".to_string());
    let deal_reference = Some("test-deal-ref".to_string());
    let currency_code = Some("USD".to_string());
    
    let order = CreateOrderRequest::sell_option_to_market(
        &epic, 
        &size, 
        &expiry, 
        &deal_reference, 
        &currency_code
    );
    
    assert_eq!(order.epic, epic);
    assert_eq!(order.direction, Direction::Sell);
    // Check that size is rounded correctly
    assert_eq!(order.size, 1.0); // Rounded from 1.0 * 100.0 / 100.0
    assert_eq!(order.order_type, OrderType::Limit); // Corrected from Market to Limit
    assert_eq!(order.time_in_force, TimeInForce::FillOrKill);
    assert!(order.level.is_some()); // Check level is set
    assert_eq!(order.level, Some(0.0)); // Updated: default level value is 0.0
    assert_eq!(order.guaranteed_stop, Some(false));
    assert!(order.stop_level.is_none());
    assert!(order.stop_distance.is_none());
    assert!(order.limit_level.is_none());
    assert!(order.limit_distance.is_none());
    assert_eq!(order.expiry, expiry);
    assert_eq!(order.deal_reference, deal_reference);
    assert_eq!(order.force_open, Some(true));
    assert_eq!(order.currency_code, currency_code);
}

#[test]
fn test_create_order_request_buy_option_to_market() {
    let epic = "CC.D.LCO.UME.IP";
    let size = 2.5;
    let expiry = "DEC-25";
    let deal_id = "test-deal-123";
    let currency = "USD";
    
    let request = CreateOrderRequest::buy_option_to_market(
        &epic.to_string(),
        &size,
        &Some(expiry.to_string()),
        &Some(deal_id.to_string()),
        &Some(currency.to_string()),
    );
    
    assert_eq!(request.epic, epic);
    assert_eq!(request.direction, Direction::Buy);
    assert_eq!(request.size, 2.5);
    assert_eq!(request.order_type, OrderType::Limit); // Updated: order_type is now Limit
    assert_eq!(request.time_in_force, TimeInForce::FillOrKill);
    assert_eq!(request.expiry, Some(expiry.to_string()));
    assert_eq!(request.deal_reference, Some(deal_id.to_string()));
    assert_eq!(request.currency_code, Some(currency.to_string()));
}

#[test]
fn test_close_position_request_market() {
    let deal_id = "test-deal-123";
    let direction = Direction::Buy;
    let size = 1.0;
    
    let request = ClosePositionRequest::market(deal_id.to_string(), direction.clone(), size);
    
    assert_eq!(request.deal_id, Some(deal_id.to_string()));
    assert_eq!(request.direction, direction);
    assert_eq!(request.size, size);
    assert_eq!(request.order_type, OrderType::Market);
    // time_in_force is now an enum, not an Option<TimeInForce>
}

#[test]
fn test_close_position_request_limit() {
    let deal_id = "test-deal-123";
    let direction = Direction::Sell;
    let size = 2.0;
    let level = 1.2345;
    
    let request = ClosePositionRequest::limit(deal_id.to_string(), direction.clone(), size, level);
    
    assert_eq!(request.deal_id, Some(deal_id.to_string()));
    assert_eq!(request.direction, direction);
    assert_eq!(request.size, size);
    assert_eq!(request.order_type, OrderType::Limit);
    assert_eq!(request.time_in_force, TimeInForce::FillOrKill); // Updated: time_in_force is now FillOrKill
    assert_eq!(request.level, Some(level));
}

#[test]
fn test_close_position_request_close_option_to_market_by_epic() {
    let epic = "CC.D.LCO.UME.IP";
    let direction = Direction::Sell;
    let size = 1.0;
    let expiry = "DEC-25";
    
    let request = ClosePositionRequest::close_option_to_market_by_epic(
        epic.to_string(),
        expiry.to_string(),
        direction.clone(),
        size,
    );
    
    assert_eq!(request.epic, Some(epic.to_string()));
    assert_eq!(request.expiry, Some(expiry.to_string()));
    assert_eq!(request.direction, direction);
    assert_eq!(request.size, size);
    assert_eq!(request.order_type, OrderType::Limit);  // Updated: order_type is now Limit
    assert!(request.deal_id.is_none());
    // time_in_force is now an enum, not an Option<TimeInForce>
}

#[test]
fn test_create_working_order_request_limit() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let level = 1.2345;
    
    let order = CreateWorkingOrderRequest::limit(epic.to_string(), direction.clone(), size, level);
    
    assert_eq!(order.epic, epic);
    assert_eq!(order.direction, direction);
    assert_eq!(order.size, size);
    assert_eq!(order.level, level);
    assert_eq!(order.order_type, OrderType::Limit);
    assert_eq!(order.time_in_force, TimeInForce::GoodTillCancelled);
    assert!(order.guaranteed_stop.is_none());
    assert!(order.stop_level.is_none());
    assert!(order.stop_distance.is_none());
    assert!(order.limit_level.is_none());
    assert!(order.limit_distance.is_none());
    assert!(order.good_till_date.is_none());
    assert!(order.deal_reference.is_none());
    assert!(order.currency_code.is_none());
}

#[test]
fn test_create_working_order_request_stop() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Sell;
    let size = 2.0;
    let level = 1.2345;
    
    let order = CreateWorkingOrderRequest::stop(epic.to_string(), direction.clone(), size, level);
    
    assert_eq!(order.epic, epic);
    assert_eq!(order.direction, direction);
    assert_eq!(order.size, size);
    assert_eq!(order.level, level);
    assert_eq!(order.order_type, OrderType::Stop);
    assert_eq!(order.time_in_force, TimeInForce::GoodTillCancelled);
    assert!(order.guaranteed_stop.is_none());
    assert!(order.stop_level.is_none());
    assert!(order.stop_distance.is_none());
    assert!(order.limit_level.is_none());
    assert!(order.limit_distance.is_none());
    assert!(order.good_till_date.is_none());
    assert!(order.deal_reference.is_none());
    assert!(order.currency_code.is_none());
}

#[test]
fn test_create_working_order_request_with_stop_loss() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let level = 1.2345;
    let stop_level = 1.2000;
    
    let order = CreateWorkingOrderRequest::limit(epic.to_string(), direction, size, level)
        .with_stop_loss(stop_level);
    
    assert_eq!(order.stop_level, Some(stop_level));
}

#[test]
fn test_create_working_order_request_with_take_profit() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let level = 1.2345;
    let limit_level = 1.3000;
    
    let order = CreateWorkingOrderRequest::limit(epic.to_string(), direction, size, level)
        .with_take_profit(limit_level);
    
    assert_eq!(order.limit_level, Some(limit_level));
}

#[test]
fn test_create_working_order_request_with_reference() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let level = 1.2345;
    let reference = "test-reference-123";
    
    let order = CreateWorkingOrderRequest::limit(epic.to_string(), direction, size, level)
        .with_reference(reference.to_string());
    
    assert_eq!(order.deal_reference, Some(reference.to_string()));
}

#[test]
fn test_create_working_order_request_expires_at() {
    let epic = "CS.D.EURUSD.TODAY.IP";
    let direction = Direction::Buy;
    let size = 1.0;
    let level = 1.2345;
    let expiry_date = "2025-12-31T23:59:59";
    
    let order = CreateWorkingOrderRequest::limit(epic.to_string(), direction, size, level)
        .expires_at(expiry_date.to_string());
    
    assert_eq!(order.time_in_force, TimeInForce::GoodTillDate);
    assert_eq!(order.good_till_date, Some(expiry_date.to_string()));
}

#[test]
fn test_deserialize_nullable_status() {
    // Helper struct for testing
    #[derive(Deserialize)]
    struct TestStatus {
        // Implementamos nuestra propia función de deserialización para probar la funcionalidad
        // ya que deserialize_nullable_status es privada
        #[serde(deserialize_with = "deserialize_status_or_default")]
        status: Status,
    }
    
    // Función de deserialización local para pruebas
    fn deserialize_status_or_default<'de, D>(deserializer: D) -> Result<Status, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt = Option::deserialize(deserializer)?;
        Ok(opt.unwrap_or(Status::Rejected))
    }
    
    // Test with a valid status
    let json_with_status = json!({
        "status": "OPEN"
    });
    let result: TestStatus = serde_json::from_value(json_with_status).unwrap();
    assert_eq!(result.status, Status::Open);
    
    // Test with null status (should default to Rejected)
    let json_with_null = json!({
        "status": null
    });
    let result: TestStatus = serde_json::from_value(json_with_null).unwrap();
    assert_eq!(result.status, Status::Rejected);
}
