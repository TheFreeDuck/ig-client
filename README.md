<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/ig-client/refs/heads/main/doc/images/logo.png" alt="ig-client" style="width: 80%; height: 80%;">
</div>

[![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/ig-client.svg)](https://crates.io/crates/ig-client)
[![Downloads](https://img.shields.io/crates/d/ig-client.svg)](https://crates.io/crates/ig-client)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/ig-client.svg)](https://github.com/joaquinbejar/ig-client/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/ig-client.svg)](https://github.com/joaquinbejar/ig-client/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/ig-client.svg)](https://github.com/joaquinbejar/ig-client/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/ig-client/CI)](https://github.com/joaquinbejar/ig-client/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/ig-client)](https://codecov.io/gh/joaquinbejar/ig-client)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/ig-client)](https://libraries.io/github/joaquinbejar/ig-client)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/ig-client)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/ig-client)


## IG Markets API Client for Rust

A comprehensive Rust client for interacting with the IG Markets trading API. This library provides a type-safe and ergonomic way to access IG Markets' REST and WebSocket APIs for trading and market data retrieval.

### Overview

The IG Markets API Client for Rust is designed to provide a reliable and efficient interface to the IG Markets trading platform. It handles authentication, session management, and all API interactions while providing a clean, idiomatic Rust interface for developers.

### Features

- **Authentication**: Secure authentication with the IG Markets API using OAuth2
- **Account Management**: Access account information, balances, and activity history
- **Market Data**: Retrieve market data, prices, instrument details, and historical prices
- **Order Management**: Create, modify, and close positions and orders with various order types
- **Transaction History**: Access detailed transaction and activity history
- **WebSocket Support**: Real-time market data streaming via WebSocket connections
- **Fully Documented**: Comprehensive documentation for all components and methods
- **Error Handling**: Robust error handling and reporting with detailed error types
- **Type Safety**: Strong type checking for API requests and responses
- **Async Support**: Built with async/await for efficient non-blocking operations
- **Configurable**: Flexible configuration options for different environments (demo/live)
- **Persistence**: Optional database integration for storing historical data

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ig-client = "0.1.5"
tokio = { version = "1", features = ["full"] }  # For async runtime
dotenv = "0.15"                                 # For environment variable loading
tracing = "0.1"                                # For logging
```

#### Requirements

- Rust 1.56 or later (for async/await support)
- An IG Markets account (demo or live)
- API credentials from IG Markets

### Configuration

Create a `.env` file in your project root with the following variables:

```,ignore
IG_USERNAME=your_username
IG_PASSWORD=your_password
IG_API_KEY=your_api_key
IG_ACCOUNT_ID=your_account_id
IG_BASE_URL=https://demo-api.ig.com/gateway/deal  # Use demo or live as needed
IG_WEBSOCKET_URL=wss://demo-apd.marketdatasystems.com
DATABASE_URL=postgres://user:password@localhost/ig_db  # Optional for data persistence
```rust

## Usage Examples

### Complete Example Application

Here's a complete example showing how to set up the client, authenticate, and perform various operations:

```rust,ignore
use ig_client::application::services::account_service::{AccountService, IgAccountService};
use ig_client::application::services::market_service::{IgMarketService, MarketService};
use ig_client::application::services::order_service::{IgOrderService, OrderService};
use ig_client::application::models::order::{CreateOrderRequest, Direction};
use ig_client::config::Config;
use ig_client::session::auth::IgAuth;
use std::sync::Arc;
use dotenv::dotenv;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Load environment variables
    dotenv().ok();
    info!("Environment variables loaded");

    // Create configuration
    let config = Arc::new(Config::new());
    info!("Configuration created");

    // Authenticate
    let auth = IgAuth::new(config.clone());
    let session = auth.authenticate().await?;
    info!("Authentication successful");

    // Create services
    let account_service = IgAccountService::new(config.clone());
    let market_service = IgMarketService::new(config.clone());
    let order_service = IgOrderService::new(config.clone());

    // Get account information
    let account_info = account_service.get_accounts(&session).await?;
    info!("Account information retrieved: {} accounts", account_info.accounts.len());

    // Search for a market
    let search_result = market_service.search_markets(&session, "EUR/USD").await?;
    info!("Found {} markets matching search", search_result.markets.len());

    if let Some(market) = search_result.markets.first() {
        info!("Selected market: {} ({})", market.instrument_name, market.epic);

        // Get market details
        let market_details = market_service.get_market_details(&session, &market.epic).await?;
        info!("Market details retrieved: {}", market_details.instrument.name);

        // Create and place a demo order (if this is a demo account)
        if session.account_type == "DEMO" {
            let order = CreateOrderRequest::market(
                market.epic.clone(),
                Direction::Buy,
                0.1,  // Small size for demo
                None,
                None,
            );

            let order_result = order_service.create_order(&session, &order).await?;
            info!("Order placed: deal reference = {}", order_result.deal_reference);

            // Get positions
            let positions = account_service.get_positions(&session).await?;
            info!("Current positions: {}", positions.positions.len());
        }
    }

    info!("Example completed successfully");
    Ok(())
}
```

#### Authentication

```rust
use ig_client::session::auth::IgAuth;
use ig_client::config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = Arc::new(Config::new());

    // Create authentication handler
    let auth = IgAuth::new(config.clone());

    // Authenticate and get a session
    let session = auth.authenticate().await?;

    info!("Successfully authenticated!");
    Ok(())
}
```

#### Getting Account Information

```rust
use ig_client::application::services::account_service::{AccountService, IgAccountService};
use std::sync::Arc;

// Create account service
let account_service = IgAccountService::new(config.clone());

// Get account information
let account_info = account_service.get_accounts(&session).await?;
info!("Accounts: {:?}", account_info);

// Get positions
let positions = account_service.get_positions(&session).await?;
info!("Open positions: {}", positions.positions.len());

// Get transaction history
let from_date = chrono::Utc::now() - chrono::Duration::days(7);
let to_date = chrono::Utc::now();
let transactions = account_service.get_transactions(&session, from_date, to_date).await?;
info!("Transactions in the last week: {}", transactions.transactions.len());
```

#### Market Data

```rust
use ig_client::application::services::market_service::{MarketService, IgMarketService};

// Create market service
let market_service = IgMarketService::new(config.clone());

// Search for markets
let search_result = market_service.search_markets(&session, "Apple").await?;
info!("Found {} markets matching 'Apple'", search_result.markets.len());

// Get market details
if let Some(market) = search_result.markets.first() {
    let details = market_service.get_market_details(&session, &market.epic).await?;
    info!("Market details for {}: {}", market.instrument_name, details.instrument.name);

    // Get historical prices
    let prices = market_service.get_prices(
        &session,
        &market.epic,
        "DAY",   // Resolution
        30,      // Number of data points
    ).await?;
    info!("Retrieved {} historical price points", prices.prices.len());
}
```

#### Placing and Managing Orders

```rust
use ig_client::application::services::order_service::{OrderService, IgOrderService};
use ig_client::application::models::order::{CreateOrderRequest, Direction, OrderType, TimeInForce};

// Create order service
let order_service = IgOrderService::new(config.clone());

// Create a market order
let market_order = CreateOrderRequest::market(
    "OP.D.OTCDAX1.021100P.IP".to_string(),  // EPIC
    Direction::Buy,                     // Direction
    1.0,                               // Size
    None,                              // Limit level
    None,                              // Stop level
);

// Place the order
let result = order_service.create_order(&session, &market_order).await?;
info!("Market order placed: {:?}", result);

// Create a limit order
let limit_order = CreateOrderRequest {
    epic: "OP.D.OTCDAX1.021100P.IP".to_string(),
    direction: Direction::Buy,
    size: 1.0,
    order_type: OrderType::Limit,
    level: Some(1.05),  // Limit price
    guaranteed_stop: false,
    time_in_force: TimeInForce::GoodTillDate,
    good_till_date: Some("2025-06-01T12:00:00".to_string()),
    stop_level: None,
    stop_distance: None,
    limit_level: None,
    limit_distance: None,
    deal_reference: Some("my-custom-reference".to_string()),
};

let result = order_service.create_order(&session, &limit_order).await?;
info!("Limit order placed: {:?}", result);

// Close a position
let positions = account_service.get_positions(&session).await?;
if let Some(position) = positions.positions.first() {
    let close_request = order_service.close_position(
        &session,
        &position.position.deal_id,
        position.position.direction.clone(),
        position.position.size,
    ).await?;
    info!("Position closed: {:?}", close_request);
}
```

#### WebSocket Streaming

```rust
use ig_client::application::services::market_listener::{MarketListener, MarketListenerCallback};
use ig_client::application::models::market::MarketData;
use std::sync::Arc;
use tokio::sync::mpsc;

// Create a channel to receive market updates
let (tx, mut rx) = mpsc::channel(100);

// Create a callback function to handle market updates
let callback: MarketListenerCallback = Arc::new(move |market_data: &MarketData| {
    let data = market_data.clone();
    let tx = tx.clone();
    tokio::spawn(async move {
        let _ = tx.send(data).await;
    });
    Ok(())
});

// Create and start the market listener
let listener = MarketListener::new(callback);
listener.connect(&session).await?;

// Subscribe to market updates
let epics = vec!["OP.D.OTCDAX1.021100P.IP", "CS.D.USDJPY.CFD.IP"];
listener.subscribe(&epics).await?;

// Process market updates
while let Some(market_data) = rx.recv().await {
    info!("Market update for {}: bid={}, offer={}",
             market_data.epic, market_data.bid.unwrap_or(0.0), market_data.offer.unwrap_or(0.0));
}
```

### Documentation

Comprehensive documentation is available for all components of the library. The documentation includes detailed explanations of all modules, structs, and functions, along with examples of how to use them.

#### API Documentation

You can access the API documentation on [docs.rs](https://docs.rs/ig-client) or generate it locally with:

```bash
make doc-open
```

#### Architecture

The library is organized into several modules:

- **config**: Configuration handling and environment variable loading
- **session**: Authentication and session management
- **application**: Core business logic and services
  - **models**: Data structures for API requests and responses
  - **services**: Service implementations for different API areas
- **transport**: HTTP and WebSocket communication with the IG Markets API
- **utils**: Utility functions for parsing, logging, etc.
- **error**: Error types and handling

### Development

This project includes a comprehensive Makefile with commands for common development tasks.

#### Building

```bash
make build        # Debug build
make release      # Release build
```

#### Testing

```bash
make test         # Run all tests
```

#### Code Quality

```bash
make fmt          # Format code with rustfmt
make lint         # Run clippy linter
make doc          # Check documentation coverage
make check        # Run tests, format check, and linting
make pre-push     # Run all checks before pushing code
```

#### Documentation

```bash
make doc-open     # Generate and open documentation
```

#### Code Coverage

```bash
make coverage     # Generate code coverage report (XML)
make coverage-html # Generate HTML coverage report
make open-coverage # Open HTML coverage report
```

#### Benchmarking

```bash
make bench        # Run benchmarks
make bench-show   # Show benchmark results
```

#### Continuous Integration

```bash
make workflow     # Run all CI workflow steps locally
```

### Project Structure

```
├── src/
│   ├── application/       # Core business logic
│   │   ├── models/        # Data models
│   │   │   ├── account.rs # Account-related models
│   │   │   ├── market.rs  # Market data models
│   │   │   ├── order.rs   # Order models
│   │   │   └── transaction.rs # Transaction models
│   │   └── services/      # Service implementations
│   │       ├── account_service.rs
│   │       ├── market_service.rs
│   │       └── order_service.rs
│   ├── config.rs          # Configuration handling
│   ├── error.rs           # Error types
│   ├── session/           # Authentication and session
│   │   └── auth.rs        # Authentication handler
│   ├── transport/         # API communication
│   │   ├── http.rs        # HTTP client
│   │   └── websocket.rs   # WebSocket client
│   └── utils/             # Utility functions
├── examples/              # Example applications
├── tests/                 # Integration tests
└── Makefile              # Development commands
```

### Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes and commit them: `git commit -m 'Add some feature'`
4. Run the tests: `make test`
5. Push to the branch: `git push origin feature/my-feature`
6. Submit a pull request

Please make sure your code passes all tests and linting checks before submitting a pull request.



## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

**Joaquín Béjar García**
- Email: jb@taunais.com
- GitHub: [joaquinbejar](https://github.com/joaquinbejar)

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under MIT license

## Disclaimer

This software is not officially associated with IG Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a demo account before using in a live trading environment.
