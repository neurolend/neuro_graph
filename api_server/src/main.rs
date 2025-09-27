use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

mod data_store;
use data_store::DataStore;

const API_PORT: u16 = 3001;

type AppState = Arc<RwLock<DataStore>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("🚀 Starting NeuroLend API Server on port {}", API_PORT);

    // Initialize data store
    let mut data_store = DataStore::new();

    // Load data from indexer output (if available)
    let indexer_output_dir = "../custom_indexer/output";
    if let Err(e) = data_store.load_from_indexer_output(indexer_output_dir) {
        warn!(
            "Could not load from indexer output: {}. Using mock data.",
            e
        );
    }

    let app_state: AppState = Arc::new(RwLock::new(data_store));

    let app = Router::new()
        // Event endpoints
        .route("/events", get(get_all_events))
        .route("/events/:event_type", get(get_events_by_type))
        .route("/events/loan/:loan_id", get(get_events_by_loan))
        .route("/events/user/:address", get(get_events_by_user))
        // Loan endpoints
        .route("/loans", get(get_all_loans))
        .route("/loans/:loan_id", get(get_loan_by_id))
        .route("/loans/user/:address", get(get_loans_by_user))
        // Statistics endpoints
        .route("/stats", get(get_statistics))
        .route("/stats/user/:address", get(get_user_statistics))
        // Health check
        .route("/health", get(health_check))
        // Add CORS middleware
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", API_PORT))
        .await
        .expect("Failed to bind to address");

    info!("🌐 API Server running at http://localhost:{}", API_PORT);
    info!("📊 Available endpoints:");
    info!("  GET /events - All events");
    info!("  GET /events/:event_type - Events by type (e.g., LoanCreated)");
    info!("  GET /events/loan/:loan_id - Events for specific loan");
    info!("  GET /events/user/:address - Events for specific user");
    info!("  GET /loans - All loans summary");
    info!("  GET /loans/:loan_id - Specific loan details");
    info!("  GET /loans/user/:address - User's loans");
    info!("  GET /stats - Overall statistics");
    info!("  GET /stats/user/:address - User statistics");
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
    pub topics: Vec<String>,
    pub data: String,
    pub decoded_data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventsResponse {
    pub events: Vec<Event>,
    pub total: usize,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanSummary {
    pub loan_id: String,
    pub borrower: Option<String>,
    pub lender: Option<String>,
    pub amount: Option<String>,
    pub collateral_amount: Option<String>,
    pub status: String, // Created, Active, Repaid, Liquidated
    pub created_at: u64,
    pub events_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub total_events: usize,
    pub total_loans: usize,
    pub active_loans: usize,
    pub total_volume: String,
    pub event_types: HashMap<String, usize>,
    pub recent_activity: Vec<Event>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    page: Option<u32>,
    limit: Option<u32>,
    from_block: Option<u64>,
    to_block: Option<u64>,
}

// Helper function to create mock event if no real data available
fn create_mock_event() -> Event {
    Event {
        event_name: "LoanCreated".to_string(),
        transaction_hash: "0x1234...".to_string(),
        block_number: 6914350,
        block_timestamp: 1695123456,
        log_index: 0,
        contract_address: "0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23".to_string(),
        topics: vec!["0xabcd...".to_string()],
        data: "0x1234...".to_string(),
        decoded_data: Some({
            let mut map = HashMap::new();
            map.insert(
                "loanId".to_string(),
                serde_json::Value::String("1".to_string()),
            );
            map.insert(
                "borrower".to_string(),
                serde_json::Value::String("0xabc...".to_string()),
            );
            map.insert(
                "amount".to_string(),
                serde_json::Value::String("1000000000000000000".to_string()),
            );
            map
        }),
    }
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().timestamp(),
        "service": "NeuroLend API"
    }))
}

async fn get_all_events(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Result<Json<EventsResponse>, StatusCode> {
    let data_store = state.read().await;
    let mut events = data_store.get_events(None, None, None);

    // If no real events, provide mock data
    if events.is_empty() {
        events = vec![create_mock_event()];
    }

    let total = events.len();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(50).min(1000); // Max 1000 per page

    // Apply pagination
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(total);
    let paginated_events = events[start..end].to_vec();

    Ok(Json(EventsResponse {
        events: paginated_events,
        total,
        page: Some(page),
        limit: Some(limit),
    }))
}

async fn get_events_by_type(
    Path(event_type): Path<String>,
    Query(params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<EventsResponse>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some(&event_type), None, None);

    let total = events.len();

    Ok(Json(EventsResponse {
        events,
        total,
        page: params.page,
        limit: params.limit,
    }))
}

async fn get_events_by_loan(
    Path(loan_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<EventsResponse>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(None, Some(&loan_id), None);

    let total = events.len();

    Ok(Json(EventsResponse {
        events,
        total,
        page: None,
        limit: None,
    }))
}

async fn get_events_by_user(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<EventsResponse>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(None, None, Some(&address));

    let total = events.len();

    Ok(Json(EventsResponse {
        events,
        total,
        page: None,
        limit: None,
    }))
}

async fn get_all_loans() -> Json<Vec<LoanSummary>> {
    // Mock loan data - replace with real aggregation
    let loans = vec![LoanSummary {
        loan_id: "1".to_string(),
        borrower: Some("0xabc...".to_string()),
        lender: Some("0xdef...".to_string()),
        amount: Some("1000000000000000000".to_string()),
        collateral_amount: Some("2000000000000000000".to_string()),
        status: "Active".to_string(),
        created_at: 1695123456,
        events_count: 3,
    }];

    Json(loans)
}

async fn get_loan_by_id(Path(loan_id): Path<String>) -> Result<Json<LoanSummary>, StatusCode> {
    // Mock loan lookup - replace with real data
    Ok(Json(LoanSummary {
        loan_id: loan_id.clone(),
        borrower: Some("0xabc...".to_string()),
        lender: Some("0xdef...".to_string()),
        amount: Some("1000000000000000000".to_string()),
        collateral_amount: Some("2000000000000000000".to_string()),
        status: "Active".to_string(),
        created_at: 1695123456,
        events_count: 3,
    }))
}

async fn get_loans_by_user(Path(address): Path<String>) -> Json<Vec<LoanSummary>> {
    // Mock user loans - replace with real filtering
    let loans = vec![LoanSummary {
        loan_id: "1".to_string(),
        borrower: Some(address.clone()),
        lender: Some("0xdef...".to_string()),
        amount: Some("1000000000000000000".to_string()),
        collateral_amount: Some("2000000000000000000".to_string()),
        status: "Active".to_string(),
        created_at: 1695123456,
        events_count: 3,
    }];

    Json(loans)
}

async fn get_statistics(State(state): State<AppState>) -> Json<Statistics> {
    let data_store = state.read().await;
    let events = data_store.get_events(None, None, None);
    let mut event_types = HashMap::new();

    for event in &events {
        *event_types.entry(event.event_name.clone()).or_insert(0) += 1;
    }

    Json(Statistics {
        total_events: events.len(),
        total_loans: 1,
        active_loans: 1,
        total_volume: "1000000000000000000".to_string(),
        event_types,
        recent_activity: events.into_iter().take(5).collect(),
    })
}

async fn get_user_statistics(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Json<Statistics> {
    let data_store = state.read().await;
    let user_stats = data_store.get_user_statistics(&address);
    Json(user_stats)
}
