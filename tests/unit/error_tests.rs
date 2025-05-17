use ig_client::error::AppError;
use reqwest::StatusCode;
use std::io;

#[test]
fn test_app_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let app_error = AppError::from(io_error);
    match app_error {
        AppError::Io(_) => {} // Verificación exitosa
        _ => panic!("Expected AppError::Io"),
    }
    assert!(app_error.to_string().contains("file not found"));
}

#[test]
fn test_app_error_unexpected() {
    let app_error = AppError::Unexpected(StatusCode::BAD_REQUEST);
    match app_error {
        AppError::Unexpected(status) => assert_eq!(status, StatusCode::BAD_REQUEST),
        _ => panic!("Expected AppError::Unexpected"),
    }
    assert!(app_error.to_string().contains("unexpected http status"));
}

#[test]
fn test_app_error_from_serde_json_error() {
    // Create a JSON error using a different approach since custom is not available
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let app_error = AppError::from(json_error);
    match app_error {
        AppError::Json(_) => {} // Verificación exitosa
        _ => panic!("Expected AppError::Json"),
    }
    assert!(app_error.to_string().contains("json error"));
}

#[test]
fn test_app_error_unauthorized() {
    let app_error = AppError::Unauthorized;
    match app_error {
        AppError::Unauthorized => {} // Verificación exitosa
        _ => panic!("Expected AppError::Unauthorized"),
    }
    assert!(app_error.to_string().contains("unauthorized"));
}

#[test]
fn test_app_error_not_found() {
    let app_error = AppError::NotFound;
    match app_error {
        AppError::NotFound => {} // Verificación exitosa
        _ => panic!("Expected AppError::NotFound"),
    }
    assert!(app_error.to_string().contains("not found"));
}

#[test]
fn test_app_error_rate_limit_exceeded() {
    let app_error = AppError::RateLimitExceeded;
    match app_error {
        AppError::RateLimitExceeded => {} // Verificación exitosa
        _ => panic!("Expected AppError::RateLimitExceeded"),
    }
    assert!(app_error.to_string().contains("rate limit exceeded"));
}

#[test]
fn test_app_error_serialization_error() {
    let app_error = AppError::SerializationError("Failed to serialize".to_string());
    match app_error {
        AppError::SerializationError(ref msg) => assert_eq!(msg, "Failed to serialize"),
        _ => panic!("Expected AppError::SerializationError"),
    }
    assert!(app_error.to_string().contains("serialization error"));
}

#[test]
fn test_app_error_websocket_error() {
    let app_error = AppError::WebSocketError("Connection closed".to_string());
    match app_error {
        AppError::WebSocketError(ref msg) => assert_eq!(msg, "Connection closed"),
        _ => panic!("Expected AppError::WebSocketError"),
    }
    assert!(app_error.to_string().contains("websocket error"));
}
