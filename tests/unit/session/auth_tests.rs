use ig_client::config::Config;
use ig_client::session::auth::IgAuth;
use ig_client::session::interface::IgSession;
use once_cell::sync::Lazy;
use std::env;
use std::sync::Mutex;

static ENV_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

fn with_env_vars<F>(vars: Vec<(&str, &str)>, test: F)
where
    F: FnOnce(),
{
    let _lock = ENV_MUTEX.lock().unwrap();

    // Save the current environment variables
    let old_vars: Vec<(String, Option<String>)> = vars
        .iter()
        .map(|(name, _)| (name.to_string(), env::var(name).ok()))
        .collect();

    // Set the environment variables for the test
    for (name, value) in vars {
        unsafe {
            env::set_var(name, value);
        }
    }

    // Run the test
    test();

    // Restore the original environment variables
    for (name, value) in old_vars {
        match value {
            Some(v) => unsafe {
                env::set_var(name, v);
            },
            None => unsafe {
                env::remove_var(name);
            },
        }
    }
}

fn create_test_config() -> Config {
    with_env_vars(
        vec![
            ("IG_USERNAME", "test_user"),
            ("IG_PASSWORD", "test_pass"),
            ("IG_API_KEY", "test_api_key"),
            ("IG_ACCOUNT_ID", "test_account"),
            ("IG_BASE_URL", "https://demo-api.ig.com"),
            ("IG_TIMEOUT", "5"),
        ],
        || {},
    );

    Config::new()
}

#[test]
fn test_ig_auth_new() {
    let config = create_test_config();
    let _auth = IgAuth::new(&config);
}

#[test]
fn test_ig_session_new() {
    let session = IgSession {
        cst: "CST123".to_string(),
        token: "XST456".to_string(),
        account_id: "ACC789".to_string(),
    };

    assert_eq!(session.cst, "CST123");
    assert_eq!(session.token, "XST456");
    assert_eq!(session.account_id, "ACC789");
}
