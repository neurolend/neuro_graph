use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

use serde_json;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn};

/// In-memory data store that reads from indexer output
pub struct DataStore {
    events: Vec<Event>,
    loans: HashMap<String, LoanSummary>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            loans: HashMap::new(),
        }
    }

    /// Load events from indexer output directory
    pub fn load_from_indexer_output(
        &mut self,
        output_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Loading events from indexer output directory: {}",
            output_dir
        );

        if !Path::new(output_dir).exists() {
            warn!("Output directory does not exist: {}", output_dir);
            return Ok(());
        }

        // Read all JSON files in the output directory
        let entries = fs::read_dir(output_dir)?;
        let mut loaded_events = 0;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match self.load_events_from_file(&path) {
                    Ok(count) => loaded_events += count,
                    Err(e) => error!("Failed to load events from {:?}: {}", path, e),
                }
            }
        }

        info!("Loaded {} events from indexer output", loaded_events);
        self.aggregate_loans();

        Ok(())
    }

    fn load_events_from_file(
        &mut self,
        file_path: &Path,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut count = 0;

        // Try to parse as single event or array of events
        if let Ok(event) = serde_json::from_str::<Event>(&content) {
            self.events.push(event);
            count = 1;
        } else if let Ok(events) = serde_json::from_str::<Vec<Event>>(&content) {
            count = events.len();
            self.events.extend(events);
        } else {
            // Try line-by-line JSON parsing (common for streaming output)
            for line in content.lines() {
                if let Ok(event) = serde_json::from_str::<Event>(line.trim()) {
                    self.events.push(event);
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// Aggregate loan information from events
    fn aggregate_loans(&mut self) {
        let mut loan_map: HashMap<String, LoanSummary> = HashMap::new();

        for event in &self.events {
            if let Some(decoded) = &event.decoded_data {
                if let Some(loan_id_value) = decoded.get("loanId") {
                    if let Some(loan_id) = loan_id_value.as_str() {
                        let loan =
                            loan_map
                                .entry(loan_id.to_string())
                                .or_insert_with(|| LoanSummary {
                                    loan_id: loan_id.to_string(),
                                    borrower: None,
                                    lender: None,
                                    amount: None,
                                    collateral_amount: None,
                                    status: "Unknown".to_string(),
                                    created_at: event.block_timestamp,
                                    events_count: 0,
                                });

                        loan.events_count += 1;

                        // Update loan details based on event type
                        match event.event_name.as_str() {
                            "LoanCreated" => {
                                loan.status = "Created".to_string();
                                if let Some(borrower) =
                                    decoded.get("borrower").and_then(|v| v.as_str())
                                {
                                    loan.borrower = Some(borrower.to_string());
                                }
                                if let Some(amount) = decoded.get("amount").and_then(|v| v.as_str())
                                {
                                    loan.amount = Some(amount.to_string());
                                }
                            }
                            "LoanAccepted" => {
                                loan.status = "Active".to_string();
                                if let Some(lender) = decoded.get("lender").and_then(|v| v.as_str())
                                {
                                    loan.lender = Some(lender.to_string());
                                }
                            }
                            "LoanRepaid" => {
                                loan.status = "Repaid".to_string();
                            }
                            "LoanLiquidated" => {
                                loan.status = "Liquidated".to_string();
                            }
                            "CollateralAdded" => {
                                if let Some(amount) = decoded.get("amount").and_then(|v| v.as_str())
                                {
                                    loan.collateral_amount = Some(amount.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        self.loans = loan_map;
        info!("Aggregated {} loans from events", self.loans.len());
    }

    /// Get all events with optional filtering
    pub fn get_events(
        &self,
        event_type: Option<&str>,
        loan_id: Option<&str>,
        user_address: Option<&str>,
    ) -> Vec<Event> {
        self.events
            .iter()
            .filter(|event| {
                if let Some(et) = event_type {
                    if !event.event_name.eq_ignore_ascii_case(et) {
                        return false;
                    }
                }

                if let Some(lid) = loan_id {
                    if let Some(decoded) = &event.decoded_data {
                        if let Some(event_loan_id) = decoded.get("loanId").and_then(|v| v.as_str())
                        {
                            if event_loan_id != lid {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if let Some(addr) = user_address {
                    if let Some(decoded) = &event.decoded_data {
                        let found_address = decoded.values().any(|v| {
                            if let Some(event_addr) = v.as_str() {
                                event_addr.eq_ignore_ascii_case(addr)
                            } else {
                                false
                            }
                        });
                        if !found_address {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Get loan by ID
    pub fn get_loan(&self, loan_id: &str) -> Option<LoanSummary> {
        self.loans.get(loan_id).cloned()
    }

    /// Get all loans
    pub fn get_loans(&self) -> Vec<LoanSummary> {
        self.loans.values().cloned().collect()
    }

    /// Get loans for a specific user
    pub fn get_user_loans(&self, user_address: &str) -> Vec<LoanSummary> {
        self.loans
            .values()
            .filter(|loan| {
                loan.borrower
                    .as_ref()
                    .map_or(false, |b| b.eq_ignore_ascii_case(user_address))
                    || loan
                        .lender
                        .as_ref()
                        .map_or(false, |l| l.eq_ignore_ascii_case(user_address))
            })
            .map(|loan| loan.clone())
            .collect()
    }

    /// Get overall statistics
    pub fn get_statistics(&self) -> Statistics {
        let mut event_types = HashMap::new();
        for event in &self.events {
            *event_types.entry(event.event_name.clone()).or_insert(0) += 1;
        }

        let active_loans = self
            .loans
            .values()
            .filter(|loan| loan.status == "Active")
            .count();

        let total_volume = self
            .loans
            .values()
            .filter_map(|loan| loan.amount.as_ref())
            .filter_map(|amount| amount.parse::<u128>().ok())
            .sum::<u128>();

        let recent_activity = {
            let mut events = self.events.clone();
            events.sort_by(|a, b| b.block_timestamp.cmp(&a.block_timestamp));
            events.into_iter().take(10).collect()
        };

        Statistics {
            total_events: self.events.len(),
            total_loans: self.loans.len(),
            active_loans,
            total_volume: total_volume.to_string(),
            event_types,
            recent_activity,
        }
    }

    /// Get user-specific statistics
    pub fn get_user_statistics(&self, user_address: &str) -> Statistics {
        let user_events = self.get_events(None, None, Some(user_address));
        let user_loans = self.get_user_loans(user_address);

        let mut event_types = HashMap::new();
        for event in &user_events {
            *event_types.entry(event.event_name.clone()).or_insert(0) += 1;
        }

        let active_loans = user_loans
            .iter()
            .filter(|loan| loan.status == "Active")
            .count();

        let total_volume = user_loans
            .iter()
            .filter_map(|loan| loan.amount.as_ref())
            .filter_map(|amount| amount.parse::<u128>().ok())
            .sum::<u128>();

        Statistics {
            total_events: user_events.len(),
            total_loans: user_loans.len(),
            active_loans,
            total_volume: total_volume.to_string(),
            event_types,
            recent_activity: user_events.into_iter().take(5).collect(),
        }
    }
}
