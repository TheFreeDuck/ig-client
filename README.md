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

A comprehensive Rust client for interacting with the IG Markets trading API. This library provides a type-safe and ergonomic way to access IG Markets' REST and WebSocket APIs for trading and market data retrieval. I have forked this repo beacuse i could not get positions to close. I am not sure why but i had to change the CreateOrderRequest and CloseOrderRequest structs to work with the api

### Overview

The IG Markets API Client for Rust is designed to provide a reliable and efficient interface to the IG Markets trading platform. It handles authentication, session management, and all API interactions while providing a clean, idiomatic Rust interface for developers.

### Features

- **Authentication**: Secure authentication with the IG Markets API including session refresh and account switching
- **Account Management**: Access account information, balances, and activity history
- **Market Data**: Retrieve market data, prices, instrument details, and historical prices
- **Order Management**: Create, modify, and close positions and orders with various order types
- **Working Orders**: Create and manage working orders with support for limit and stop orders
- **Transaction History**: Access detailed transaction and activity history
- **WebSocket Support**: Real-time market data streaming via WebSocket connections
- **Advanced Rate Limiting**: Sophisticated rate limiting with automatic backoff, concurrent request management, and explicit rate limit error handling
- **Fully Documented**: Comprehensive documentation for all components and methods
- **Error Handling**: Robust error handling and reporting with detailed error types
- **Type Safety**: Strong type checking for API requests and responses
- **Async Support**: Built with async/await for efficient non-blocking operations
- **Concurrency Management**: Built-in semaphores and thread-safe primitives for handling concurrent API requests
- **Configurable**: Flexible configuration options for different environments (demo/live)
- **Persistence**: Optional database integration for storing historical data
- **Database Support**: Integration with SQLx for storing and retrieving transaction data
- **Serialization Utilities**: Custom serialization helpers for handling IG Markets API responses

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ig-client = "0.1.20"
tokio = { version = "1", features = ["full"] }  # For async runtime
dotenv = "0.15"                                 # For environment variable loading
tracing = "0.1"                                # For logging
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }  # Optional for database support
```

#### Requirements

- Rust 1.56 or later (for async/await support)
- An IG Markets account (demo or live)
- API credentials from IG Markets
- PostgreSQL database (optional, for data persistence)

### Configuration

Create a `.env` file in your project root with the following variables:

```
IG_USERNAME=your_username
IG_PASSWORD=your_password
IG_API_KEY=your_api_key
IG_ACCOUNT_ID=your_account_id
IG_BASE_URL=https://demo-api.ig.com/gateway/deal  # Use demo or live as needed
IG_TIMEOUT=30  # HTTP request timeout in seconds
IG_WS_URL=wss://demo-apd.marketdatasystems.com  # WebSocket URL
IG_WS_RECONNECT=5  # WebSocket reconnect interval in seconds
DATABASE_URL=postgres://user:password@localhost/ig_db  # Optional for data persistence
IG_DB_MAX_CONN=5  # Maximum database connections
TX_LOOP_INTERVAL_HOURS=1  # Transaction loop interval in hours
TX_PAGE_SIZE=20  # Transaction page size
TX_DAYS_BACK=7  # Number of days to look back for transactions
```

### Usage Examples

See /examples

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

#### Rate Limiting

The library includes a sophisticated rate limiting system to comply with IG Markets API restrictions:

- **Multiple Rate Limit Types**: Different limits for trading, non-trading, and historical data requests
- **Thread-Safe Implementation**: Uses `tokio::sync::Mutex` for safe concurrent access
- **Automatic Backoff**: Dynamically calculates wait times based on request history
- **Explicit Rate Limit Handling**: Detects and handles rate limit errors from the API
- **Global Semaphore**: Limits concurrent API requests to prevent overwhelming the API
- **Configurable Safety Margins**: Adjustable safety margins to stay below API limits
- **Rate Limit Error Recovery**: Automatic cooldown and recovery when rate limits are exceeded

Example of configuring rate limits:

```rust
// Create a configuration with custom rate limit settings
let config = Arc::new(Config::with_rate_limit_type(
    RateLimitType::NonTradingAccount,  // Type of rate limit to enforce
    0.8,                               // Safety margin (80% of actual limit)
));

// The rate limiter will automatically be used by all services
let http_client = IgHttpClientImpl::new(config.clone());
let auth = IgAuth::new(config.clone());

// When rate limits are exceeded, the system will automatically:
// 1. Detect the rate limit error from the API
// 2. Enforce a mandatory cooldown period
// 3. Gradually resume requests with appropriate delays
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
│   │   │   ├── transaction.rs # Transaction models
│   │   │   └── working_order.rs # Working order models
│   │   └── services/      # Service implementations
│   │       ├── account_service.rs
│   │       ├── interfaces/ # Service interfaces
│   │       ├── listener.rs # Price listener service
│   │       ├── market_service.rs
│   │       └── order_service.rs
│   ├── config.rs          # Configuration handling
│   ├── constants.rs       # Global constants
│   ├── error.rs           # Error types
│   ├── presentation/      # Presentation layer
│   │   ├── account.rs     # Account presentation
│   │   ├── market.rs      # Market presentation
│   │   ├── serialization.rs # Serialization utilities
│   │   └── trade.rs       # Trade presentation
│   ├── session/           # Authentication and session
│   │   ├── auth.rs        # Authentication handler
│   │   └── interface.rs   # Session interface
│   ├── storage/           # Data persistence
│   │   ├── config.rs      # Database configuration
│   │   └── utils.rs       # Storage utilities
│   ├── transport/         # API communication
│   │   └── http_client.rs # HTTP client
│   └── utils/             # Utility functions
│       ├── display.rs     # Display utilities
│       ├── finance.rs     # Financial calculations
│       ├── logger.rs      # Logging utilities
│       ├── market_parser.rs # Market data parsing utilities
│       ├── parsing.rs     # Parsing utilities
│       └── rate_limiter.rs # Advanced rate limiting with concurrency management
├── examples/              # Example applications
├── tests/                 # Tests
│   ├── integration/       # Integration tests
│   └── unit/              # Unit tests
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

## ✍️ License

Licensed under MIT license

## Disclaimer

This software is not officially associated with IG Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a demo account before using in a live trading environment.
