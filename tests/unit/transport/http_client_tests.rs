use ig_client::config::{Config, RestApiConfig};
use ig_client::session::interface::IgSession;
use ig_client::transport::http_client::IgHttpClientImpl;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

fn create_test_config() -> Arc<Config> {
    Arc::new(Config {
        rest_api: RestApiConfig {
            base_url: "https://demo-api.ig.com".to_string(),
            timeout: 5,
        },
        ..Default::default()
    })
}

#[allow(dead_code)]
fn create_test_session() -> IgSession {
    IgSession {
        cst: "test_cst".to_string(),
        token: "test_xst".to_string(),
        account_id: "test_account".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestRequest {
    name: String,
    value: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct TestResponse {
    result: String,
    code: i32,
}

#[test]
fn test_http_client_new() {
    let config = create_test_config();
    let _client = IgHttpClientImpl::new(config.clone());
}

