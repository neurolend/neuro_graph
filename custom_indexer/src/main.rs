use anyhow::Result;
use ethers::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::{error, info, warn};

mod event_signatures;

// Reuse your contract address
const NEUROLEND_CONTRACT: &str = "0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23";
const STARTING_BLOCK: u64 = 6914309;
const RPC_URL: &str = "https://evmrpc.0g.ai";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting NeuroLend indexer for 0G network");

    // Connect to 0G network
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let provider = Arc::new(provider);

    // Verify connection
    let chain_id = provider.get_chainid().await?;
    info!("Connected to 0G network, Chain ID: {}", chain_id);

    // Get current block
    let current_block = provider.get_block_number().await?;
    info!("Current block: {}", current_block);

    // Start indexing
    let mut indexer = NeuroLendIndexer::new(provider).await?;
    indexer.start_indexing(STARTING_BLOCK).await?;

    Ok(())
}

struct NeuroLendIndexer {
    provider: Arc<Provider<Http>>,
    contract_address: Address,
    event_signatures: HashMap<H256, &'static str>,
    output_dir: String,
}

impl NeuroLendIndexer {
    async fn new(provider: Arc<Provider<Http>>) -> Result<Self> {
        let contract_address = NEUROLEND_CONTRACT.parse()?;
        let event_signatures = event_signatures::get_event_signatures();
        let output_dir = "output".to_string();

        // Create output directory if it doesn't exist
        if !Path::new(&output_dir).exists() {
            fs::create_dir_all(&output_dir)?;
            info!("Created output directory: {}", output_dir);
        }

        info!("Loaded {} event signatures", event_signatures.len());

        Ok(Self {
            provider,
            contract_address,
            event_signatures,
            output_dir,
        })
    }

    async fn start_indexing(&mut self, start_block: u64) -> Result<()> {
        info!("Starting indexing from block {}", start_block);

        let current_block = self.provider.get_block_number().await?.as_u64();

        // Process historical blocks first
        self.process_historical_blocks(start_block, current_block)
            .await?;

        // Then start listening for new blocks
        self.listen_for_new_blocks().await?;

        Ok(())
    }

    async fn process_historical_blocks(&self, start_block: u64, end_block: u64) -> Result<()> {
        info!(
            "Processing historical blocks {} to {}",
            start_block, end_block
        );

        const BATCH_SIZE: u64 = 1000; // Process in batches to avoid RPC limits

        for batch_start in (start_block..=end_block).step_by(BATCH_SIZE as usize) {
            let batch_end = std::cmp::min(batch_start + BATCH_SIZE - 1, end_block);

            info!("Processing batch: {} to {}", batch_start, batch_end);

            // Get logs for this batch
            let filter = Filter::new()
                .address(self.contract_address)
                .from_block(batch_start)
                .to_block(batch_end);

            match self.provider.get_logs(&filter).await {
                Ok(logs) => {
                    info!(
                        "Found {} logs in batch {} to {}",
                        logs.len(),
                        batch_start,
                        batch_end
                    );
                    self.process_logs(logs).await?;
                }
                Err(e) => {
                    error!(
                        "Error getting logs for batch {} to {}: {}",
                        batch_start, batch_end, e
                    );
                    // Continue with next batch
                }
            }

            // Small delay to avoid overwhelming the RPC
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }

    async fn listen_for_new_blocks(&self) -> Result<()> {
        info!("Starting to listen for new blocks...");

        let mut last_processed_block = self.provider.get_block_number().await?.as_u64();
        info!(
            "Starting real-time monitoring from block {}",
            last_processed_block
        );

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await; // Poll every 5 seconds

            let current_block = self.provider.get_block_number().await?.as_u64();

            if current_block > last_processed_block {
                info!(
                    "New blocks detected: {} to {}",
                    last_processed_block + 1,
                    current_block
                );

                // Process new blocks
                let filter = Filter::new()
                    .address(self.contract_address)
                    .from_block(last_processed_block + 1)
                    .to_block(current_block);

                match self.provider.get_logs(&filter).await {
                    Ok(logs) => {
                        if !logs.is_empty() {
                            info!(
                                "Found {} new logs in blocks {} to {}",
                                logs.len(),
                                last_processed_block + 1,
                                current_block
                            );
                            self.process_logs(logs).await?;
                        }
                    }
                    Err(e) => {
                        error!("Error getting new logs: {}", e);
                    }
                }

                last_processed_block = current_block;
            }
        }
    }

    async fn process_logs(&self, logs: Vec<Log>) -> Result<()> {
        for log in logs {
            self.process_single_log(&log).await?;
        }
        Ok(())
    }

    async fn process_single_log(&self, log: &Log) -> Result<()> {
        // Get the transaction to extract more details
        let tx_hash = log.transaction_hash.unwrap_or_default();
        let block_number = log.block_number.unwrap_or_default().as_u64();

        // Get block timestamp
        let block = self.provider.get_block(block_number).await?;
        let timestamp = block.map(|b| b.timestamp).unwrap_or_default();

        // Extract event signature (first topic)
        if log.topics.is_empty() {
            warn!("Log has no topics, skipping");
            return Ok(());
        }

        let event_signature = log.topics[0];

        // Match against known NeuroLend events
        // You can add your existing event processing logic here
        match self.identify_event(&event_signature) {
            Some(event_name) => {
                info!(
                    "Processing {} event in tx {} (block {})",
                    event_name,
                    format!("0x{:x}", tx_hash),
                    block_number
                );

                // Create event data structure
                let event_data = json!({
                    "event_name": event_name,
                    "transaction_hash": format!("0x{:x}", tx_hash),
                    "block_number": block_number,
                    "block_timestamp": timestamp.as_u64(),
                    "log_index": log.log_index.unwrap_or_default().as_u64(),
                    "contract_address": format!("0x{:x}", self.contract_address),
                    "topics": log.topics.iter().map(|t| format!("0x{:x}", t)).collect::<Vec<_>>(),
                    "data": format!("0x{}", hex::encode(&log.data))
                });

                // Save event to JSON file
                self.save_event_to_file(&event_data, event_name, block_number)
                    .await?;

                // Also print to console for debugging
                println!("{}", serde_json::to_string_pretty(&event_data)?);
            }
            None => {
                warn!("Unknown event signature: 0x{:x}", event_signature);
            }
        }

        Ok(())
    }

    fn identify_event(&self, signature: &H256) -> Option<&'static str> {
        self.event_signatures.get(signature).copied()
    }

    async fn save_event_to_file(
        &self,
        event_data: &serde_json::Value,
        event_name: &str,
        block_number: u64,
    ) -> Result<()> {
        // Create filename with timestamp and block number for uniqueness
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis();

        let filename = format!(
            "{}_{}_{}_{}.json",
            event_name,
            block_number,
            timestamp,
            rand::random::<u32>()
        );
        let filepath = Path::new(&self.output_dir).join(filename);

        // Save the event data to file
        let json_string = serde_json::to_string_pretty(event_data)?;
        fs::write(&filepath, json_string)?;

        info!("Saved {} event to file: {:?}", event_name, filepath);
        Ok(())
    }
}
