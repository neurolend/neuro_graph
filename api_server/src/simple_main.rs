use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use tracing::info;

// Port configuration for Railway
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
    info!("🚀 Starting NeuroLend API Server on port {}", api_port);

    let app = Router::new()
        // Event endpoints
        .route("/events", get(get_all_events))
        .route("/events/:event_type", get(get_events_by_type))
        .route("/loans", get(get_all_loans))
        .route("/stats", get(get_statistics))
        .route("/health", get(health_check))
        // Add CORS middleware
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", api_port))
        .await
        .expect("Failed to bind to address");

    info!("🌐 API Server running on 0.0.0.0:{} (Railway will proxy this)", api_port);
    info!("📊 Available endpoints:");
    info!("  GET /events - All events");
    info!("  GET /events/:event_type - Events by type");
    info!("  GET /loans - All loans");
    info!("  GET /stats - Statistics");
    info!("  GET /health - Health check");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub event_name: String,
    pub transaction_hash: String,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub log_index: u64,
    pub contract_address: String,
    pub loan_id: Option<String>,
    pub borrower: Option<String>,
    pub lender: Option<String>,
    pub amount: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsResponse {
    pub events: Vec<Event>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoanSummary {
    pub loan_id: String,
    pub borrower: Option<String>,
    pub lender: Option<String>,
    pub amount: Option<String>,
    pub status: String,
    pub created_at: u64,
    pub events_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub total_events: usize,
    pub total_loans: usize,
    pub active_loans: usize,
    pub event_types: HashMap<String, usize>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    limit: Option<u32>,
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().timestamp(),
        "service": "NeuroLend API",
        "message": "Ready to serve your 0G NeuroLend data!"
    }))
}

async fn get_all_events(
    Query(params): Query<QueryParams>,
) -> Result<Json<EventsResponse>, StatusCode> {
    // Mock events representing your loan creation
    let events = vec![
        Event {
            event_name: "LoanCreated".to_string(),
            transaction_hash: "0x1234567890abcdef...".to_string(),
            block_number: 6937000,
            block_timestamp: chrono::Utc::now().timestamp() as u64 - 3600,
            log_index: 0,
            contract_address: "0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23".to_string(),
            loan_id: Some("1".to_string()),
            borrower: Some("0xYourAddress...".to_string()),
            lender: None,
            amount: Some("1000000000000000000".to_string()), // 1 ETH in wei
        },
        Event {
            event_name: "CollateralAdded".to_string(),
            transaction_hash: "0xabcdef1234567890...".to_string(),
            block_number: 6937001,
            block_timestamp: chrono::Utc::now().timestamp() as u64 - 3500,
            log_index: 0,
            contract_address: "0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23".to_string(),
            loan_id: Some("1".to_string()),
            borrower: Some("0xYourAddress...".to_string()),
            lender: None,
            amount: Some("2000000000000000000".to_string()), // 2 ETH collateral
        },
    ];

    let limit = params.limit.unwrap_or(50).min(1000);
    let limited_events = events.into_iter().take(limit as usize).collect::<Vec<_>>();
    let total = limited_events.len();

    Ok(Json(EventsResponse {
        events: limited_events,
        total,
    }))
}

async fn get_events_by_type(
    Path(event_type): Path<String>,
) -> Result<Json<EventsResponse>, StatusCode> {
    let all_events = vec![Event {
        event_name: "LoanCreated".to_string(),
        transaction_hash: "0x1234567890abcdef...".to_string(),
        block_number: 6937000,
        block_timestamp: chrono::Utc::now().timestamp() as u64 - 3600,
        log_index: 0,
        contract_address: "0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23".to_string(),
        loan_id: Some("1".to_string()),
        borrower: Some("0xYourAddress...".to_string()),
        lender: None,
        amount: Some("1000000000000000000".to_string()),
    }];

    let filtered_events: Vec<Event> = all_events
        .into_iter()
        .filter(|e| e.event_name.eq_ignore_ascii_case(&event_type))
        .collect();

    Ok(Json(EventsResponse {
        total: filtered_events.len(),
        events: filtered_events,
    }))
}

async fn get_all_loans() -> Json<Vec<LoanSummary>> {
    let loans = vec![LoanSummary {
        loan_id: "1".to_string(),
        borrower: Some("0xYourAddress...".to_string()),
        lender: None,
        amount: Some("1000000000000000000".to_string()),
        status: "Active".to_string(),
        created_at: chrono::Utc::now().timestamp() as u64 - 3600,
        events_count: 2,
    }];

    Json(loans)
}

async fn get_statistics() -> Json<Statistics> {
    let mut event_types = HashMap::new();
    event_types.insert("LoanCreated".to_string(), 1);
    event_types.insert("CollateralAdded".to_string(), 1);

    Json(Statistics {
        total_events: 2,
        total_loans: 1,
        active_loans: 1,
        event_types,
    })
}
