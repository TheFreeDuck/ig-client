use std::sync::Arc;

use ig_client::application::models::order::{
    Direction, OrderType, TimeInForce, CreateOrderRequest, UpdatePositionRequest,
    ClosePositionRequest
};
use ig_client::application::services::order_service::{OrderService, OrderServiceImpl};
use ig_client::config::Config;
use ig_client::session::interface::IgSession;

// Tests para los constructores y métodos auxiliares de las estructuras de órdenes

#[test]
fn test_create_order_request_market() {
    // Probar el constructor market de CreateOrderRequest
    let order = CreateOrderRequest::market(
        "CS.D.EURUSD.CFD.IP".to_string(),
        Direction::Buy,
        1.0
    );
    
    // Verificar que los campos se establecieron correctamente
    assert_eq!(order.epic, "CS.D.EURUSD.CFD.IP");
    assert!(matches!(order.direction, Direction::Buy));
    assert_eq!(order.size, 1.0);
    assert!(matches!(order.order_type, OrderType::Market));
    assert!(matches!(order.time_in_force, TimeInForce::FillOrKill));
}

#[test]
fn test_create_order_request_limit() {
    // Probar el constructor limit de CreateOrderRequest
    let order = CreateOrderRequest::limit(
        "CS.D.EURUSD.CFD.IP".to_string(),
        Direction::Sell,
        1.0,
        1.2345
    );
    
    // Verificar que los campos se establecieron correctamente
    assert_eq!(order.epic, "CS.D.EURUSD.CFD.IP");
    assert!(matches!(order.direction, Direction::Sell));
    assert_eq!(order.size, 1.0);
    assert!(matches!(order.order_type, OrderType::Limit));
    assert!(matches!(order.time_in_force, TimeInForce::GoodTillCancelled));
    assert_eq!(order.level, Some(1.2345));
}

#[test]
fn test_create_order_request_with_reference() {
    // Probar el método with_reference
    let order = CreateOrderRequest::market(
        "CS.D.EURUSD.CFD.IP".to_string(),
        Direction::Buy,
        1.0
    ).with_reference("TEST_REF".to_string());
    
    // Verificar que el campo deal_reference se estableció correctamente
    assert_eq!(order.deal_reference, Some("TEST_REF".to_string()));
}

#[test]
fn test_create_order_request_with_stop_loss() {
    // Probar el método with_stop_loss
    let order = CreateOrderRequest::market(
        "CS.D.EURUSD.CFD.IP".to_string(),
        Direction::Buy,
        1.0
    ).with_stop_loss(1.2000);
    
    // Verificar que el campo stop_level se estableció correctamente
    assert_eq!(order.stop_level, Some(1.2000));
}

#[test]
fn test_create_order_request_with_take_profit() {
    // Probar el método with_take_profit
    let order = CreateOrderRequest::market(
        "CS.D.EURUSD.CFD.IP".to_string(),
        Direction::Buy,
        1.0
    ).with_take_profit(1.3000);
    
    // Verificar que el campo limit_level se estableció correctamente
    assert_eq!(order.limit_level, Some(1.3000));
}

#[test]
fn test_close_position_request_market() {
    // Probar el constructor market de ClosePositionRequest
    let request = ClosePositionRequest::market(
        "DEAL123".to_string(),
        Direction::Sell,
        1.0
    );
    
    // Verificar que los campos se establecieron correctamente
    assert_eq!(request.deal_id, "DEAL123");
    assert!(matches!(request.direction, Direction::Sell));
    assert_eq!(request.size, 1.0);
    assert!(matches!(request.order_type, OrderType::Market));
    assert!(matches!(request.time_in_force, TimeInForce::FillOrKill));
    assert_eq!(request.level, None);
}

#[test]
fn test_update_position_request() {
    // Crear una solicitud de actualización
    let update = UpdatePositionRequest {
        stop_level: Some(1.2000),
        limit_level: Some(1.3000),
        trailing_stop: Some(false),
        trailing_stop_distance: Some(0.01),
    };
    
    // Verificar que los campos se establecieron correctamente
    assert_eq!(update.stop_level, Some(1.2000));
    assert_eq!(update.limit_level, Some(1.3000));
    assert_eq!(update.trailing_stop, Some(false));
    assert_eq!(update.trailing_stop_distance, Some(0.01));
}

#[test]
fn test_order_service_config() {
    // Crear una sesión simulada
    let session = IgSession {
        cst: "CST123".to_string(),
        token: "XST123".to_string(),
        account_id: "ACC123".to_string(),
    };
    
    // Verificar que la sesión se creó correctamente
    assert_eq!(session.cst, "CST123");
    assert_eq!(session.token, "XST123");
    assert_eq!(session.account_id, "ACC123");
}
