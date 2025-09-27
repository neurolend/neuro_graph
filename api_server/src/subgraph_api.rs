use axum::{
    extract::{Query, State},
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

use crate::data_store::{DataStore, Statistics};
use crate::subgraph_types::*;

type AppState = Arc<RwLock<DataStore>>;

#[derive(Deserialize)]
pub struct QueryParams {
    pub first: Option<u32>,
    pub skip: Option<u32>,
    pub orderBy: Option<String>,
    pub orderDirection: Option<String>,
    #[serde(rename = "where")]
    pub where_clause: Option<String>,
}

pub fn create_subgraph_router(app_state: AppState) -> Router {
    Router::new()
        // Individual entity endpoints (subgraph style)
        .route("/loanAccepteds", get(get_loan_accepteds))
        .route("/loanCreateds", get(get_loan_createds))
        .route("/loanLiquidateds", get(get_loan_liquidateds))
        .route("/loanOfferCancelleds", get(get_loan_offer_cancelleds))
        .route("/loanOfferRemoveds", get(get_loan_offer_removeds))
        .route("/loanRepaids", get(get_loan_repaids))
        .route("/priceFeedSets", get(get_price_feed_sets))
        .route("/protocolStats_collection", get(get_protocol_stats))
        .route("/tokens", get(get_tokens))
        // Combined query endpoint (like your subgraph query)
        .route("/query", get(get_combined_query))
        // GraphQL-style endpoint
        .route("/graphql", get(get_graphql_query))
        // Health check
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().timestamp(),
        "service": "NeuroLend Subgraph API",
        "message": "Subgraph-compatible API ready!"
    }))
}

async fn get_loan_accepteds(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LoanAccepted>>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some("LoanAccepted"), None, None);
    
    let loan_accepteds: Vec<LoanAccepted> = events
        .into_iter()
        .filter_map(|event| {
            // Convert generic event to LoanAccepted
            if let Some(decoded) = &event.decoded_data {
                Some(LoanAccepted {
                    block_number: event.block_number.to_string(),
                    block_timestamp: event.block_timestamp.to_string(),
                    borrower: decoded.get("borrower")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    initial_collateral_ratio: decoded.get("initialCollateralRatio")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    id: format!("{}-{}", event.transaction_hash, event.log_index),
                    loan_id: decoded.get("loanId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    timestamp: event.block_timestamp.to_string(),
                    transaction_hash: event.transaction_hash,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(loan_accepteds))
}

async fn get_loan_createds(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LoanCreated>>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some("LoanCreated"), None, None);
    
    let loan_createds: Vec<LoanCreated> = events
        .into_iter()
        .filter_map(|event| {
            if let Some(decoded) = &event.decoded_data {
                Some(LoanCreated {
                    amount: decoded.get("amount")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    amount_usd: decoded.get("amountUSD")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    block_number: event.block_number.to_string(),
                    block_timestamp: event.block_timestamp.to_string(),
                    collateral_address: decoded.get("collateralAddress")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    collateral_amount: decoded.get("collateralAmount")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    duration: decoded.get("duration")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    id: format!("{}-{}", event.transaction_hash, event.log_index),
                    interest_rate: decoded.get("interestRate")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    lender: decoded.get("lender")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    liquidation_threshold_bps: decoded.get("liquidationThresholdBPS")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    loan_id: decoded.get("loanId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    max_price_staleness: decoded.get("maxPriceStaleness")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    min_collateral_ratio_bps: decoded.get("minCollateralRatioBPS")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    price_usd: decoded.get("priceUSD")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    token_address: decoded.get("tokenAddress")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    transaction_hash: event.transaction_hash,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(loan_createds))
}

async fn get_loan_liquidateds(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LoanLiquidated>>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some("LoanLiquidated"), None, None);
    
    let loan_liquidateds: Vec<LoanLiquidated> = events
        .into_iter()
        .filter_map(|event| {
            if let Some(decoded) = &event.decoded_data {
                Some(LoanLiquidated {
                    block_number: event.block_number.to_string(),
                    block_timestamp: event.block_timestamp.to_string(),
                    id: format!("{}-{}", event.transaction_hash, event.log_index),
                    collateral_claimed_by_lender: decoded.get("collateralClaimedByLender")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    liquidator: decoded.get("liquidator")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    liquidator_reward: decoded.get("liquidatorReward")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    loan_id: decoded.get("loanId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    timestamp: event.block_timestamp.to_string(),
                    transaction_hash: event.transaction_hash,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(loan_liquidateds))
}

async fn get_loan_offer_cancelleds(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LoanOfferCancelled>>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some("LoanOfferCancelled"), None, None);
    
    let loan_offer_cancelleds: Vec<LoanOfferCancelled> = events
        .into_iter()
        .filter_map(|event| {
            if let Some(decoded) = &event.decoded_data {
                Some(LoanOfferCancelled {
                    block_timestamp: event.block_timestamp.to_string(),
                    block_number: event.block_number.to_string(),
                    id: format!("{}-{}", event.transaction_hash, event.log_index),
                    lender: decoded.get("lender")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    loan_id: decoded.get("loanId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    transaction_hash: event.transaction_hash,
                    timestamp: event.block_timestamp.to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(loan_offer_cancelleds))
}

async fn get_loan_offer_removeds(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LoanOfferRemoved>>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some("LoanOfferRemoved"), None, None);
    
    let loan_offer_removeds: Vec<LoanOfferRemoved> = events
        .into_iter()
        .filter_map(|event| {
            if let Some(decoded) = &event.decoded_data {
                Some(LoanOfferRemoved {
                    block_number: event.block_number.to_string(),
                    block_timestamp: event.block_timestamp.to_string(),
                    id: format!("{}-{}", event.transaction_hash, event.log_index),
                    loan_id: decoded.get("loanId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    reason: decoded.get("reason")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                    transaction_hash: event.transaction_hash,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(loan_offer_removeds))
}

async fn get_loan_repaids(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<LoanRepaid>>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some("LoanRepaid"), None, None);
    
    let loan_repaids: Vec<LoanRepaid> = events
        .into_iter()
        .filter_map(|event| {
            if let Some(decoded) = &event.decoded_data {
                Some(LoanRepaid {
                    block_number: event.block_number.to_string(),
                    block_timestamp: event.block_timestamp.to_string(),
                    borrower: decoded.get("borrower")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    id: format!("{}-{}", event.transaction_hash, event.log_index),
                    loan_id: decoded.get("loanId")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    repayment_amount: decoded.get("repaymentAmount")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .to_string(),
                    timestamp: event.block_timestamp.to_string(),
                    transaction_hash: event.transaction_hash,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(loan_repaids))
}

async fn get_price_feed_sets(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PriceFeedSet>>, StatusCode> {
    let data_store = state.read().await;
    let events = data_store.get_events(Some("PriceFeedSet"), None, None);
    
    let price_feed_sets: Vec<PriceFeedSet> = events
        .into_iter()
        .filter_map(|event| {
            if let Some(decoded) = &event.decoded_data {
                Some(PriceFeedSet {
                    block_number: event.block_number.to_string(),
                    block_timestamp: event.block_timestamp.to_string(),
                    feed_address: decoded.get("feedAddress")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    id: format!("{}-{}", event.transaction_hash, event.log_index),
                    token_address: decoded.get("tokenAddress")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0x0")
                        .to_string(),
                    transaction_hash: event.transaction_hash,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Json(price_feed_sets))
}

async fn get_protocol_stats(
    Query(_params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProtocolStats>>, StatusCode> {
    let data_store = state.read().await;
    let stats = data_store.get_statistics();
    
    // Convert our statistics to protocol stats format
    let protocol_stats = vec![ProtocolStats {
        total_loan_volume: stats.total_volume,
        total_loan_volume_usd: "0".to_string(), // TODO: Calculate USD value
        total_loans_created: stats.total_loans.to_string(),
    }];

    Ok(Json(protocol_stats))
}

async fn get_tokens(
    Query(_params): Query<QueryParams>,
    State(_state): State<AppState>,
) -> Result<Json<Vec<Token>>, StatusCode> {
    // Mock token data - replace with real token registry
    let tokens = vec![
        Token {
            decimals: "18".to_string(),
            id: "0x0000000000000000000000000000000000000000".to_string(),
            price_feed_decimals: "8".to_string(),
            price_feed: "0x0000000000000000000000000000000000000000".to_string(),
        }
    ];

    Ok(Json(tokens))
}

async fn get_combined_query(
    Query(params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<MyQueryResponse>, StatusCode> {
    // Get all data types in parallel
    let loan_accepteds = get_loan_accepteds(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let loan_createds = get_loan_createds(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let loan_liquidateds = get_loan_liquidateds(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let loan_offer_cancelleds = get_loan_offer_cancelleds(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let loan_offer_removeds = get_loan_offer_removeds(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let loan_repaids = get_loan_repaids(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let price_feed_sets = get_price_feed_sets(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let protocol_stats_collection = get_protocol_stats(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy.clone(), 
        orderDirection: params.orderDirection.clone(), 
        where_clause: params.where_clause.clone() 
    }), State(state.clone())).await?.0;
    
    let tokens = get_tokens(Query(QueryParams { 
        first: params.first, 
        skip: params.skip, 
        orderBy: params.orderBy, 
        orderDirection: params.orderDirection, 
        where_clause: params.where_clause 
    }), State(state)).await?.0;

    let response = MyQueryResponse {
        loan_accepteds,
        loan_createds,
        loan_liquidateds,
        loan_offer_cancelleds,
        loan_offer_removeds,
        loan_repaids,
        price_feed_sets,
        protocol_stats_collection,
        tokens,
    };

    Ok(Json(response))
}

async fn get_graphql_query(
    Query(params): Query<QueryParams>,
    State(state): State<AppState>,
) -> Result<Json<SubgraphResponse<MyQueryResponse>>, StatusCode> {
    let data = get_combined_query(Query(params), State(state)).await?.0;
    
    Ok(Json(SubgraphResponse { data }))
}
