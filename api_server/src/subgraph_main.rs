use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

mod data_store;
mod subgraph_types;
mod subgraph_api;

use data_store::DataStore;
use subgraph_api::create_subgraph_router;

// Port configuration for Railway/production
fn get_port() -> u16 {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .unwrap_or(3001);

    println!("🔍 Environment PORT: {:?}", std::env::var("PORT"));
    println!("🚀 Using port: {}", port);
    port
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let api_port = get_port();
    info!("🚀 Starting NeuroLend Subgraph-Compatible API Server on port {}", api_port);

    // Initialize data store
    let mut data_store = DataStore::new();

    // Load data from indexer output (if available)
    let indexer_output_dir = "../custom_indexer/output";
    if let Err(e) = data_store.load_from_indexer_output(indexer_output_dir) {
        warn!(
            "Could not load from indexer output: {}. Using empty data store.",
            e
        );
    }

    let app_state = Arc::new(RwLock::new(data_store));

    // Create subgraph-compatible router
    let app = create_subgraph_router(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", api_port))
        .await
        .expect("Failed to bind to address");

    info!(
        "🌐 Subgraph API Server running on 0.0.0.0:{} (Railway will proxy this)",
        api_port
    );
    info!("📊 Available subgraph-compatible endpoints:");
    info!("  GET /loanAccepteds - Loan accepted events");
    info!("  GET /loanCreateds - Loan created events");
    info!("  GET /loanLiquidateds - Loan liquidated events");
    info!("  GET /loanOfferCancelleds - Loan offer cancelled events");
    info!("  GET /loanOfferRemoveds - Loan offer removed events");
    info!("  GET /loanRepaids - Loan repaid events");
    info!("  GET /priceFeedSets - Price feed set events");
    info!("  GET /protocolStats_collection - Protocol statistics");
    info!("  GET /tokens - Token information");
    info!("  GET /query - Combined query (all data)");
    info!("  GET /graphql - GraphQL-style response");
    info!("  GET /health - Health check");
    info!("");
    info!("🔗 Query parameters supported:");
    info!("  ?first=N - Limit results");
    info!("  ?skip=N - Skip results");
    info!("  ?orderBy=field - Order by field");
    info!("  ?orderDirection=asc|desc - Order direction");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
