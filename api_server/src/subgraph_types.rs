use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Subgraph-compatible data structures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanAccepted {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    pub borrower: String,
    #[serde(rename = "initialCollateralRatio")]
    pub initial_collateral_ratio: String,
    pub id: String,
    #[serde(rename = "loanId")]
    pub loan_id: String,
    pub timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanCreated {
    pub amount: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: Option<String>,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "collateralAddress")]
    pub collateral_address: String,
    #[serde(rename = "collateralAmount")]
    pub collateral_amount: String,
    pub duration: String,
    pub id: String,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
    pub lender: String,
    #[serde(rename = "liquidationThresholdBPS")]
    pub liquidation_threshold_bps: String,
    #[serde(rename = "loanId")]
    pub loan_id: String,
    #[serde(rename = "maxPriceStaleness")]
    pub max_price_staleness: String,
    #[serde(rename = "minCollateralRatioBPS")]
    pub min_collateral_ratio_bps: String,
    #[serde(rename = "priceUSD")]
    pub price_usd: Option<String>,
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanLiquidated {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    pub id: String,
    #[serde(rename = "collateralClaimedByLender")]
    pub collateral_claimed_by_lender: String,
    pub liquidator: String,
    #[serde(rename = "liquidatorReward")]
    pub liquidator_reward: String,
    #[serde(rename = "loanId")]
    pub loan_id: String,
    pub timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanOfferCancelled {
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub id: String,
    pub lender: String,
    #[serde(rename = "loanId")]
    pub loan_id: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanOfferRemoved {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    pub id: String,
    #[serde(rename = "loanId")]
    pub loan_id: String,
    pub reason: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanRepaid {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    pub borrower: String,
    pub id: String,
    #[serde(rename = "loanId")]
    pub loan_id: String,
    #[serde(rename = "repaymentAmount")]
    pub repayment_amount: String,
    pub timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceFeedSet {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "feedAddress")]
    pub feed_address: String,
    pub id: String,
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolStats {
    #[serde(rename = "totalLoanVolume")]
    pub total_loan_volume: String,
    #[serde(rename = "totalLoanVolumeUSD")]
    pub total_loan_volume_usd: String,
    #[serde(rename = "totalLoansCreated")]
    pub total_loans_created: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub decimals: String,
    pub id: String,
    #[serde(rename = "priceFeedDecimals")]
    pub price_feed_decimals: String,
    #[serde(rename = "priceFeed")]
    pub price_feed: String,
}

// Response wrapper for subgraph compatibility
#[derive(Serialize, Deserialize, Debug)]
pub struct SubgraphResponse<T> {
    pub data: T,
}

// Main query response structure matching your subgraph
#[derive(Serialize, Deserialize, Debug)]
pub struct MyQueryResponse {
    #[serde(rename = "loanAccepteds")]
    pub loan_accepteds: Vec<LoanAccepted>,
    #[serde(rename = "loanCreateds")]
    pub loan_createds: Vec<LoanCreated>,
    #[serde(rename = "loanLiquidateds")]
    pub loan_liquidateds: Vec<LoanLiquidated>,
    #[serde(rename = "loanOfferCancelleds")]
    pub loan_offer_cancelleds: Vec<LoanOfferCancelled>,
    #[serde(rename = "loanOfferRemoveds")]
    pub loan_offer_removeds: Vec<LoanOfferRemoved>,
    #[serde(rename = "loanRepaids")]
    pub loan_repaids: Vec<LoanRepaid>,
    #[serde(rename = "priceFeedSets")]
    pub price_feed_sets: Vec<PriceFeedSet>,
    #[serde(rename = "protocolStats_collection")]
    pub protocol_stats_collection: Vec<ProtocolStats>,
    pub tokens: Vec<Token>,
}
