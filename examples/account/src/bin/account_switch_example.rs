use ig_client::config::Config;
use ig_client::session::auth::IgAuth;
use ig_client::session::interface::IgAuthenticator;
use ig_client::utils::logger::setup_logger;
use std::error::Error;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging
    setup_logger();

    // Load configuration from environment variables
    let cfg = Config::new();
    info!("Loaded config → {}", cfg.rest_api.base_url);

    // Create authenticator
    let auth = IgAuth::new(&cfg);

    // Login to get initial session
    info!("Logging in...");
    let session = match auth.login().await {
        Ok(sess) => {
            info!("✅ Authentication successful. Account: {}", sess.account_id);
            info!("CST  = {}", sess.cst);
            info!("X-ST = {}", sess.token);
            sess
        }
        Err(e) => {
            error!("Authentication failed: {e:?}");
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    // Display the current account ID
    info!("Current account ID: {}", session.account_id);
    let account_id = cfg.credentials.account_id.trim().to_string();

    if !account_id.is_empty() {
        // Check if we are already on the desired account
        if session.account_id == account_id {
            info!(
                "Already authenticated on the requested account: {}",
                account_id
            );
            info!("Account switch skipped");
            return Ok(());
        }

        let default_account = true;

        // Switch to the specified account
        info!("Switching to account: {}", account_id);
        match auth
            .switch_account(&session, &account_id, Some(default_account))
            .await
        {
            Ok(new_session) => {
                info!("✅ Account switch successful");
                info!("New account ID: {}", new_session.account_id);
                info!("Default account: {}", default_account);
            }
            Err(e) => {
                error!("Account switch failed: {e:?}");
                // If the error is due to API rate limit, we show a clearer message
                if let Some(_err_msg) = e.to_string().find("exceeded-api-key-allowance") {
                    error!(
                        "API rate limit exceeded. Please wait a few minutes before trying again."
                    );
                }
                return Err(Box::new(e) as Box<dyn Error>);
            }
        }
    } else {
        info!("Account switch skipped - no account ID provided");
    }

    Ok(())
}
