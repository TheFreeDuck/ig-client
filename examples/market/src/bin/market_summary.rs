
use ig_client::config::Config;
use ig_client::session::auth::{IgAuth};
use std::error::Error;
use std::sync::Arc;
use tracing::{error, info};
use ig_client::application::services::market_service::MarketServiceImpl;
use ig_client::application::services::MarketService;
use ig_client::session::interface::IgAuthenticator;
use ig_client::transport::http_client::IgHttpClientImpl;
use ig_client::utils::logger::setup_logger;

// Constants for API request handling
const REQUEST_DELAY_MS: u64 = 3000; // Delay between API requests in milliseconds
const BATCH_SIZE: usize = 25; // Number of EPICs to process before saving results


/// Structure to hold market summary information
#[derive(Debug, Clone)]
struct MarketSummary {
    instrument_name: String,
    epic: String,
    bid: Option<f64>,
    offer: Option<f64>,
    mid: Option<f64>,
    spread: Option<f64>,
    last_dealing_date: String,
}


/// Fetches market details and returns a vector of MarketSummary structs
async fn get_market_summaries() -> Result<Vec<MarketSummary>, Box<dyn Error>> {

    let config = Arc::new(Config::new());
    info!("Loaded configuration → {}", config.rest_api.base_url);

    // Create the HTTP client
    let client = Arc::new(IgHttpClientImpl::new(config.clone()));

    // Create the authenticator
    let auth = IgAuth::new(&config);

    // Create market service
    let market_service = MarketServiceImpl::new(config.clone(), client);

    // Login to get a session
    info!("Logging in...");
    let session = auth
        .login()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    info!("Login successful. Account ID: {}", session.account_id);
    
    let epics = (1..=196)
        .map(|i| format!("DO.D.OTCDDAX.{}.IP", i))
        .collect::<Vec<String>>();

    info!("Fetching market details for {} EPICs", epics.len());

    // Create a vector to store all the market details
    let mut market_summaries = Vec::new();
    // Process EPICs in batches to use the API more efficiently
    let mut processed_count = 0;
    let total_epics = epics.len();

    // Process EPICs in batches of BATCH_SIZE
    for chunk_start in (0..epics.len()).step_by(BATCH_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + BATCH_SIZE, epics.len());
        let epics_chunk = &epics[chunk_start..chunk_end];

        info!(
            "Fetching market details for batch {}/{} (EPICs {}-{} of {})",
            (chunk_start / BATCH_SIZE) + 1,
            (total_epics + BATCH_SIZE - 1) / BATCH_SIZE,
            chunk_start + 1,
            chunk_end,
            total_epics
        );

        // Log the EPICs being processed in this batch
        info!("EPICs in this batch: {}", epics_chunk.join(", "));

        match market_service
            .get_multiple_market_details(&session, epics_chunk)
            .await
        {
            Ok(details_vec) => {
                // Match each result with its corresponding EPIC
                for (i, details) in details_vec.iter().enumerate() {
                    let epic = &epics_chunk[i];
                    info!("✅ Successfully fetched details for {}", epic);
                    let (mid, spread) = if details.snapshot.bid.is_none() || details.snapshot.offer.is_none() {
                        info!("❌ Missing bid or offer for {}", epic);
                        (None, None)
                    } else {
                        let mid = details.snapshot.offer.unwrap() + details.snapshot.bid.unwrap() / 2.0;
                        let spread = details.snapshot.offer.unwrap() - details.snapshot.bid.unwrap();
                        (Some(mid), Some(spread))
                    };
                    let last_dealing_date = match details.instrument.expiry_details.clone() {
                        Some(expiry_details) => expiry_details.last_dealing_date,
                        None => details.instrument.expiry.clone(),
                    };


                    let market_summary = MarketSummary {
                        instrument_name: details.instrument.name.clone(),
                        epic: epic.clone(),
                        bid: details.snapshot.bid,
                        offer: details.snapshot.offer,
                        mid,
                        spread,
                        last_dealing_date,
                    };
                    market_summaries.push(market_summary);
                }

                processed_count += details_vec.len();
                info!(
                    "✅ Successfully processed batch of {} EPICs ({}/{})",
                    details_vec.len(),
                    processed_count,
                    total_epics
                );
            }
            Err(e) => {
                error!("❌ Failed to fetch details for batch: {:?}", e);

                // Fall back to processing EPICs individually in case of batch failure
                info!("Falling back to processing EPICs individually...");

                for epic in epics_chunk {
                    info!("Fetching market details for {} individually", epic);

                    match market_service.get_market_details(&session, epic).await {
                        Ok(details) => {
                            info!("✅ Successfully fetched details for {}", epic);
                            let (mid, spread) = if details.snapshot.bid.is_none() || details.snapshot.offer.is_none() {
                                info!("❌ Missing bid or offer for {}", epic);
                                (None, None)
                            } else { 
                                let mid = details.snapshot.offer.unwrap() + details.snapshot.bid.unwrap() / 2.0;
                                let spread = details.snapshot.offer.unwrap() - details.snapshot.bid.unwrap();
                                (Some(mid), Some(spread))
                            };
                            let last_dealing_date = match details.instrument.expiry_details.clone() {
                                Some(expiry_details) => expiry_details.last_dealing_date,
                                None => details.instrument.expiry.clone(),
                            };
                            
                            let market_summary = MarketSummary {
                                instrument_name: details.instrument.name,
                                epic: epic.clone(),
                                bid: details.snapshot.bid,
                                offer: details.snapshot.offer,
                                mid,
                                spread,
                                last_dealing_date,
                            };
                            market_summaries.push(market_summary);
                            processed_count += 1;
                        }
                        Err(e) => {
                            error!("❌ Failed to fetch details for {}: {:?}", epic, e);
                        }
                    }

                    // Add a small delay between individual requests
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }

        // Add a delay between batches to avoid rate limiting
        if chunk_end < epics.len() {
            info!("Waiting {}ms before next batch...", REQUEST_DELAY_MS);
            tokio::time::sleep(tokio::time::Duration::from_millis(REQUEST_DELAY_MS)).await;
        }

        // Save intermediate results after each batch
        if processed_count > 0 {
            info!(
                "Processed {}/{} EPICs. Saving intermediate results...",
                processed_count,
                epics.len()
            );
        }
    }

    Ok(market_summaries)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();
    
    // Get market summaries
    let market_summaries = get_market_summaries().await?;

    // Display the results
    println!("\n{:<30} {:<20} {:<10} {:<10} {:<10} {:<10} {:<20}", 
             "INSTRUMENT NAME", "EPIC", "BID", "OFFER", "MID", "SPREAD", "LAST DEALING DATE");
    println!("{:-<110}", "");

    for summary in market_summaries {
        println!("{:<30} {:<20} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<20}",
                 summary.instrument_name,
                 summary.epic,
                 summary.bid.unwrap_or(0.0),
                 summary.offer.unwrap_or(0.0),
                 summary.mid.unwrap_or(0.0),
                 summary.spread.unwrap_or(0.0),
                 summary.last_dealing_date);
    }

    Ok(())
}
