use ig_client::{
    application::models::market::{MarketData, MarketNavigationResponse},
    application::services::MarketService,
    config::Config,
    error::AppError,
    session::auth::IgAuth,
    session::interface::{IgAuthenticator, IgSession},
    transport::http_client::IgHttpClientImpl,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, future::Future, pin::Pin, error::Error};
use tokio;
use tracing::{info, debug, error, warn};
use ig_client::application::services::account_service::AccountServiceImpl;
use ig_client::application::services::market_service::MarketServiceImpl;
use ig_client::utils::logger::setup_logger;

// Tiempo de espera entre solicitudes para respetar los límites de la API
const SLEEP_TIME: u64 = 2000; // 10 segundos

/// Estructura que representa un nodo en la jerarquía de mercados
#[derive(Debug, Serialize, Deserialize)]
struct MarketNode {
    /// ID del nodo
    id: String,
    /// Nombre del nodo
    name: String,
    /// Nodos hijos
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<MarketNode>,
    /// Mercados en este nodo
    #[serde(skip_serializing_if = "Vec::is_empty")]
    markets: Vec<MarketData>,
}

/// Function to recursively build the market hierarchy with rate limiting
fn build_market_hierarchy<'a>(
    market_service: &'a impl MarketService,
    session: &'a IgSession,
    node_id: Option<&'a str>,
    depth: usize,
) -> Pin<Box<dyn Future<Output = Result<Vec<MarketNode>, AppError>> + 'a>> {
    Box::pin(async move {
        // Limit the depth to avoid infinite loops o demasiadas solicitudes
        if depth > 7 {
            debug!("Reached maximum depth of 5, stopping recursion");
            return Ok(Vec::new());
        }

        // Add a delay to respect rate limits (SLEEP_TIME ms between requests)
        if depth > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(SLEEP_TIME)).await;
        }

        // Get the nodes and markets at the current level
        let navigation: MarketNavigationResponse = match node_id {
            Some(id) => {
                debug!("Getting navigation node: {}", id);
                match market_service.get_market_navigation_node(session, id).await {
                    Ok(response) => {
                        debug!("Response received for node {}: {} nodes, {} markets", 
                              id, response.nodes.len(), response.markets.len());
                        response
                    },
                    Err(e) => {
                        error!("Error getting node {}: {:?}", id, e);
                        // If we hit a rate limit, return empty results instead of failing
                        if matches!(e, AppError::RateLimitExceeded | AppError::Unexpected(_)) {
                            info!("Rate limit or API error encountered, returning partial results");
                            return Ok(Vec::new());
                        }
                        return Err(e);
                    }
                }
            },
            None => {
                debug!("Getting top-level navigation nodes");
                match market_service.get_market_navigation(session).await {
                    Ok(response) => {
                        debug!("Response received for top-level nodes: {} nodes, {} markets", 
                              response.nodes.len(), response.markets.len());
                        response
                    },
                    Err(e) => {
                        error!("Error getting top-level nodes: {:?}", e);
                        return Err(e);
                    }
                }
            },
        };

        let mut nodes = Vec::new();

        // Process all nodes at this level
        let nodes_to_process = navigation.nodes;

        // Process nodes with rate limiting
        for (i, node) in nodes_to_process.into_iter().enumerate() {
            // Add a delay between node processing to respect rate limits
            if i > 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(SLEEP_TIME)).await;
            }

            // Recursively get the children of this node
            match build_market_hierarchy(market_service, session, Some(&node.id), depth + 1).await {
                Ok(children) => {
                    info!("Adding node {} with {} children", node.name, children.len());
                    nodes.push(MarketNode {
                        id: node.id.clone(),
                        name: node.name.clone(),
                        children,
                        markets: Vec::new(),
                    });
                },
                Err(e) => {
                    error!("Error building hierarchy for node {}: {:?}", node.id, e);
                    // Continuar con otros nodos incluso si uno falla
                    if depth < 7 {
                        nodes.push(MarketNode {
                            id: node.id.clone(),
                            name: format!("{} (error: {})", node.name, e),
                            children: Vec::new(),
                            markets: Vec::new(),
                        });
                    }
                }
            }
        }

        // Process all markets in this node
        let markets_to_process = navigation.markets;
        for market in markets_to_process {
            // Añadir mercados como nodos hoja (sin hijos)
            debug!("Adding market: {}", market.instrument_name);
            nodes.push(MarketNode {
                id: market.epic.clone(),
                name: market.instrument_name.clone(),
                children: Vec::new(),
                markets: vec![market],
            });
        }

        Ok(nodes)
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure logger with more detail for debugging
    setup_logger();

    // Load configuration from environment variables
    let config = Config::new();
    
    // Create HTTP client
    let client = Arc::new(IgHttpClientImpl::new(Arc::new(config.clone())));
    
    // Create services
    let _account_service = AccountServiceImpl::new(Arc::new(config.clone()), client.clone());
    let market_service = MarketServiceImpl::new(Arc::new(config.clone()), client.clone());
    
    // Create authenticator
    let auth = IgAuth::new(&config);
    
    
    // Login
    info!("Logging in...");
    let session = auth.login().await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    info!("Login successful");
    
    
    let session = match auth.switch_account(&session, &config.credentials.account_id, Some(true)).await {
        Ok(new_session) => {
            info!("✅ Switched to account: {}", new_session.account_id);
            new_session
        },
        Err(e) => {
            warn!("Could not switch to account {}: {:?}. Attempting to re-authenticate.", 
                  config.credentials.account_id, e);
            
            match auth.login().await {
                Ok(new_session) => {
                    info!("Re-authentication successful. Using account: {}", new_session.account_id);
                    new_session
                },
                Err(login_err) => {
                    error!("Re-authentication failed: {:?}. Using original session.", login_err);
                    session
                }
            }
        }
    };
    
    // First test with a simple request to verify the API
    info!("Testing API with a simple request...");
    match market_service.get_market_navigation(&session).await {
        Ok(response) => {
            info!("Test successful: {} nodes, {} markets at top level", 
                 response.nodes.len(), response.markets.len());
            
            // If the test is successful, build the complete hierarchy
            info!("Building market hierarchy...");
            let hierarchy = match build_market_hierarchy(&market_service, &session, None, 0).await {
                Ok(h) => {
                    info!("Successfully built hierarchy with {} top-level nodes", h.len());
                    h
                },
                Err(e) => {
                    error!("Error building complete hierarchy: {:?}", e);
                    info!("Attempting to build a partial hierarchy with rate limiting...");
                    // Try again with a smaller scope
                    let limited_nodes = response.nodes.iter().map(|n| MarketNode {
                        id: n.id.clone(),
                        name: n.name.clone(),
                        children: Vec::new(),
                        markets: Vec::new(),
                    }).collect::<Vec<_>>();
                    info!("Created partial hierarchy with {} top-level nodes", limited_nodes.len());
                    limited_nodes
                }
            };
            
            // Convert to JSON and save to a file
            let json = serde_json::to_string_pretty(&hierarchy)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            let filename = "Data/market_hierarchy.json";
            std::fs::write(filename, &json)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
            
            info!("Market hierarchy saved to '{}'", filename);
            info!("Hierarchy contains {} top-level nodes", hierarchy.len());
        },
        Err(e) => {
            error!("Error in initial API test: {:?}", e);
            
            // Get the underlying cause of the error if possible
            let mut current_error: &dyn Error = &e;
            while let Some(source) = current_error.source() {
                error!("Error cause: {}", source);
                current_error = source;
                
                // If it's a deserialization error, provide more information
                if source.to_string().contains("Decode") {
                    info!("Attempting to get raw response for analysis...");
                    error!("The API response structure does not match our model.");
                    error!("The API may have changed or there might be an authentication issue.");
                }
            }
            
            // If it's a rate limit error, provide specific guidance
            if matches!(e, AppError::RateLimitExceeded | AppError::Unexpected(_)) {
                error!("API rate limit exceeded or access denied.");
                info!("Consider implementing exponential backoff or reducing request frequency.");
                info!("The demo account has limited API access. Try again later or use a production account.");
            }
            
            return Err(Box::new(e) as Box<dyn Error>);
        }
    }
    
    Ok(())
}
