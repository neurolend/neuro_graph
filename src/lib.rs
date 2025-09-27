mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const NEUROLEND_TRACKED_CONTRACT: [u8; 20] = hex!("064c3e0a900743d9ac87c778d2f6d3d5819d4f23");

// ERC20 Transfer event signature: Transfer(address,address,uint256)
const ERC20_TRANSFER_EVENT_SIG: [u8; 32] = hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

// ERC20 Approval event signature: Approval(address,address,uint256)  
const ERC20_APPROVAL_EVENT_SIG: [u8; 32] = hex!("8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925");

fn map_neurolend_events(blk: &eth::Block, events: &mut contract::Events) {
    events.neurolend_collateral_addeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::CollateralAdded::match_and_decode(log)
                        {
                            return Some(contract::NeurolendCollateralAdded {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                borrower: event.borrower,
                                loan_id: event.loan_id.to_string(),
                                new_collateral_ratio: event.new_collateral_ratio.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_collateral_removeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::CollateralRemoved::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::NeurolendCollateralRemoved {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                borrower: event.borrower,
                                loan_id: event.loan_id.to_string(),
                                new_collateral_ratio: event.new_collateral_ratio.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_accepteds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanAccepted::match_and_decode(log)
                        {
                            return Some(contract::NeurolendLoanAccepted {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                borrower: event.borrower,
                                initial_collateral_ratio: event
                                    .initial_collateral_ratio
                                    .to_string(),
                                loan_id: event.loan_id.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_createds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanCreated::match_and_decode(log)
                        {
                            return Some(contract::NeurolendLoanCreated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                collateral_address: event.collateral_address,
                                collateral_amount: event.collateral_amount.to_string(),
                                duration: event.duration.to_string(),
                                interest_rate: event.interest_rate.to_string(),
                                lender: event.lender,
                                liquidation_threshold_bps: event
                                    .liquidation_threshold_bps
                                    .to_string(),
                                loan_id: event.loan_id.to_string(),
                                max_price_staleness: event.max_price_staleness.to_string(),
                                min_collateral_ratio_bps: event
                                    .min_collateral_ratio_bps
                                    .to_string(),
                                token_address: event.token_address,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_liquidateds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanLiquidated::match_and_decode(log)
                        {
                            return Some(contract::NeurolendLoanLiquidated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                collateral_claimed_by_lender: event
                                    .collateral_claimed_by_lender
                                    .to_string(),
                                liquidator: event.liquidator,
                                liquidator_reward: event.liquidator_reward.to_string(),
                                loan_id: event.loan_id.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_matcheds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanMatched::match_and_decode(log)
                        {
                            return Some(contract::NeurolendLoanMatched {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                borrower: event.borrower,
                                interest_rate: event.interest_rate.to_string(),
                                lender: event.lender,
                                loan_id: event.loan_id.to_string(),
                                offer_id: event.offer_id.to_string(),
                                request_id: event.request_id.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_offer_cancelleds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanOfferCancelled::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::NeurolendLoanOfferCancelled {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                lender: event.lender,
                                loan_id: event.loan_id.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_offer_removeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanOfferRemoved::match_and_decode(log)
                        {
                            return Some(contract::NeurolendLoanOfferRemoved {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                loan_id: event.loan_id.to_string(),
                                reason: event.reason,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_repaids.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanRepaid::match_and_decode(log)
                        {
                            return Some(contract::NeurolendLoanRepaid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                borrower: event.borrower,
                                loan_id: event.loan_id.to_string(),
                                repayment_amount: event.repayment_amount.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_request_cancelleds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanRequestCancelled::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::NeurolendLoanRequestCancelled {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                borrower: event.borrower,
                                request_id: event.request_id.to_string(),
                                timestamp: event.timestamp.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_request_createds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanRequestCreated::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::NeurolendLoanRequestCreated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                borrower: event.borrower,
                                collateral_address: event.collateral_address,
                                collateral_amount: event.collateral_amount.to_string(),
                                duration: event.duration.to_string(),
                                liquidation_threshold_bps: event
                                    .liquidation_threshold_bps
                                    .to_string(),
                                max_interest_rate: event.max_interest_rate.to_string(),
                                max_price_staleness: event.max_price_staleness.to_string(),
                                min_collateral_ratio_bps: event
                                    .min_collateral_ratio_bps
                                    .to_string(),
                                request_id: event.request_id.to_string(),
                                token_address: event.token_address,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_loan_request_removeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::LoanRequestRemoved::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::NeurolendLoanRequestRemoved {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                reason: event.reason,
                                request_id: event.request_id.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_ownership_transferreds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::OwnershipTransferred::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::NeurolendOwnershipTransferred {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_owner: event.new_owner,
                                previous_owner: event.previous_owner,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_partial_repayments.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::PartialRepayment::match_and_decode(log)
                        {
                            return Some(contract::NeurolendPartialRepayment {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                borrower: event.borrower,
                                loan_id: event.loan_id.to_string(),
                                remaining_amount: event.remaining_amount.to_string(),
                                repayment_amount: event.repayment_amount.to_string(),
                                timestamp: event.timestamp.to_string(),
                                total_repaid_amount: event.total_repaid_amount.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_price_feed_sets.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::PriceFeedSet::match_and_decode(log)
                        {
                            return Some(contract::NeurolendPriceFeedSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                feed_id: Vec::from(event.feed_id),
                                token_address: event.token_address,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.neurolend_price_update_paids.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NEUROLEND_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::neurolend_contract::events::PriceUpdatePaid::match_and_decode(log)
                        {
                            return Some(contract::NeurolendPriceUpdatePaid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                loan_id: event.loan_id.to_string(),
                                timestamp: event.timestamp.to_string(),
                                update_fee: event.update_fee.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}

fn map_erc20_events(blk: &eth::Block, events: &mut contract::Events) {
    // Track ERC20 Transfer events
    events.erc20_transfers.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| {
                        // Check if this is an ERC20 Transfer event
                        log.topics.len() >= 3 && 
                        log.topics[0] == ERC20_TRANSFER_EVENT_SIG
                    })
                    .filter_map(|log| {
                        // Decode ERC20 Transfer event
                        if log.topics.len() >= 3 && log.data.len() >= 32 {
                            let from = if log.topics[1] == [0u8; 32] {
                                vec![0u8; 20] // Zero address for minting
                            } else {
                                log.topics[1][12..32].to_vec() // Last 20 bytes for address
                            };
                            
                            let to = if log.topics[2] == [0u8; 32] {
                                vec![0u8; 20] // Zero address for burning
                            } else {
                                log.topics[2][12..32].to_vec() // Last 20 bytes for address
                            };
                            
                            // Parse value from data (first 32 bytes)
                            let value_bytes = &log.data[0..32];
                            let value = substreams::scalar::BigInt::from_unsigned_bytes_be(value_bytes);
                            
                            return Some(contract::Erc20Transfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                contract_address: log.address.to_vec(),
                                from,
                                to,
                                value: value.to_string(),
                            });
                        }
                        None
                    })
            })
            .collect(),
    );

    // Track ERC20 Approval events
    events.erc20_approvals.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| {
                        // Check if this is an ERC20 Approval event
                        log.topics.len() >= 3 && 
                        log.topics[0] == ERC20_APPROVAL_EVENT_SIG
                    })
                    .filter_map(|log| {
                        // Decode ERC20 Approval event
                        if log.topics.len() >= 3 && log.data.len() >= 32 {
                            let owner = log.topics[1][12..32].to_vec(); // Last 20 bytes for address
                            let spender = log.topics[2][12..32].to_vec(); // Last 20 bytes for address
                            
                            // Parse value from data (first 32 bytes)
                            let value_bytes = &log.data[0..32];
                            let value = substreams::scalar::BigInt::from_unsigned_bytes_be(value_bytes);
                            
                            return Some(contract::Erc20Approval {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                contract_address: log.address.to_vec(),
                                owner,
                                spender,
                                value: value.to_string(),
                            });
                        }
                        None
                    })
            })
            .collect(),
    );
}

fn map_generic_events(blk: &eth::Block, events: &mut contract::Events) {
    // Track all other events as generic logs for completeness
    events.generic_logs.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| {
                        // Skip events we already handle specifically
                        log.address != NEUROLEND_TRACKED_CONTRACT &&
                        !(log.topics.len() >= 1 && 
                          (log.topics[0] == ERC20_TRANSFER_EVENT_SIG || 
                           log.topics[0] == ERC20_APPROVAL_EVENT_SIG))
                    })
                    .map(|log| {
                        contract::GenericLog {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            contract_address: log.address.to_vec(),
                            topics: log.topics.iter().map(|topic| topic.to_vec()).collect(),
                            data: log.data.clone(),
                        }
                    })
            })
            .collect(),
    );
}

fn map_neurolend_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.neurolend_call_accept_loan_offer_1s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::AcceptLoanOffer1::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::AcceptLoanOffer1::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendAcceptLoanOffer1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                loan_id: decoded_call.loan_id.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_accept_loan_offer_2s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::AcceptLoanOffer2::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::AcceptLoanOffer2::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendAcceptLoanOffer2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                loan_id: decoded_call.loan_id.to_string(),
                                price_update: decoded_call
                                    .price_update
                                    .into_iter()
                                    .map(|x| x)
                                    .collect::<Vec<_>>(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_add_collaterals.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::AddCollateral::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::AddCollateral::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendAddCollateralCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                additional_amount: decoded_call.additional_amount.to_string(),
                                loan_id: decoded_call.loan_id.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_cancel_loan_offers.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::CancelLoanOffer::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::CancelLoanOffer::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendCancelLoanOfferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                loan_id: decoded_call.loan_id.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_cancel_loan_requests.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::CancelLoanRequest::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::CancelLoanRequest::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendCancelLoanRequestCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                request_id: decoded_call.request_id.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_create_loan_offer_1s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::CreateLoanOffer1::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::CreateLoanOffer1::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendCreateLoanOffer1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_collateral_address: decoded_call.u_collateral_address,
                                u_collateral_amount: decoded_call.u_collateral_amount.to_string(),
                                u_duration: decoded_call.u_duration.to_string(),
                                u_interest_rate: decoded_call.u_interest_rate.to_string(),
                                u_liquidation_threshold_bps: decoded_call
                                    .u_liquidation_threshold_bps
                                    .to_string(),
                                u_max_price_staleness: decoded_call
                                    .u_max_price_staleness
                                    .to_string(),
                                u_min_collateral_ratio_bps: decoded_call
                                    .u_min_collateral_ratio_bps
                                    .to_string(),
                                u_token_address: decoded_call.u_token_address,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_create_loan_offer_2s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::CreateLoanOffer2::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::CreateLoanOffer2::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendCreateLoanOffer2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_collateral_address: decoded_call.u_collateral_address,
                                u_collateral_amount: decoded_call.u_collateral_amount.to_string(),
                                u_duration: decoded_call.u_duration.to_string(),
                                u_interest_rate: decoded_call.u_interest_rate.to_string(),
                                u_token_address: decoded_call.u_token_address,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_create_loan_request_1s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::CreateLoanRequest1::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::CreateLoanRequest1::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendCreateLoanRequest1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_collateral_address: decoded_call.u_collateral_address,
                                u_collateral_amount: decoded_call.u_collateral_amount.to_string(),
                                u_duration: decoded_call.u_duration.to_string(),
                                u_max_interest_rate: decoded_call.u_max_interest_rate.to_string(),
                                u_token_address: decoded_call.u_token_address,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_create_loan_request_2s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::CreateLoanRequest2::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::CreateLoanRequest2::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendCreateLoanRequest2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_collateral_address: decoded_call.u_collateral_address,
                                u_collateral_amount: decoded_call.u_collateral_amount.to_string(),
                                u_duration: decoded_call.u_duration.to_string(),
                                u_liquidation_threshold_bps: decoded_call
                                    .u_liquidation_threshold_bps
                                    .to_string(),
                                u_max_interest_rate: decoded_call.u_max_interest_rate.to_string(),
                                u_max_price_staleness: decoded_call
                                    .u_max_price_staleness
                                    .to_string(),
                                u_min_collateral_ratio_bps: decoded_call
                                    .u_min_collateral_ratio_bps
                                    .to_string(),
                                u_token_address: decoded_call.u_token_address,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_fill_loan_offer_1s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::FillLoanOffer1::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::FillLoanOffer1::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendFillLoanOffer1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                offer_id: decoded_call.offer_id.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_fill_loan_offer_2s.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::FillLoanOffer2::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::FillLoanOffer2::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendFillLoanOffer2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                offer_id: decoded_call.offer_id.to_string(),
                                price_update: decoded_call
                                    .price_update
                                    .into_iter()
                                    .map(|x| x)
                                    .collect::<Vec<_>>(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_fill_loan_requests.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::FillLoanRequest::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::FillLoanRequest::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendFillLoanRequestCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                request_id: decoded_call.request_id.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_liquidate_loans.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::LiquidateLoan::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::LiquidateLoan::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendLiquidateLoanCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                loan_id: decoded_call.loan_id.to_string(),
                                price_update: decoded_call
                                    .price_update
                                    .into_iter()
                                    .map(|x| x)
                                    .collect::<Vec<_>>(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_make_partial_repayments.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::MakePartialRepayment::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::MakePartialRepayment::decode(call)
                        {
                            Ok(decoded_call) => Some(contract::NeurolendMakePartialRepaymentCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                loan_id: decoded_call.loan_id.to_string(),
                                repayment_amount: decoded_call.repayment_amount.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_remove_collaterals.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::RemoveCollateral::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::RemoveCollateral::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendRemoveCollateralCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                loan_id: decoded_call.loan_id.to_string(),
                                price_update: decoded_call
                                    .price_update
                                    .into_iter()
                                    .map(|x| x)
                                    .collect::<Vec<_>>(),
                                remove_amount: decoded_call.remove_amount.to_string(),
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_renounce_ownerships.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::RenounceOwnership::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::RenounceOwnership::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendRenounceOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_repay_loans.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::RepayLoan::match_call(call)
                    })
                    .filter_map(
                        |call| match abi::neurolend_contract::functions::RepayLoan::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendRepayLoanCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                loan_id: decoded_call.loan_id.to_string(),
                            }),
                            Err(_) => None,
                        },
                    )
            })
            .collect(),
    );
    calls.neurolend_call_set_token_price_feed_ids.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::SetTokenPriceFeedId::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::SetTokenPriceFeedId::decode(call)
                        {
                            Ok(decoded_call) => Some(contract::NeurolendSetTokenPriceFeedIdCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_feed_id: Vec::from(decoded_call.u_feed_id),
                                u_token_address: decoded_call.u_token_address,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
    calls.neurolend_call_transfer_ownerships.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == NEUROLEND_TRACKED_CONTRACT
                            && abi::neurolend_contract::functions::TransferOwnership::match_call(
                                call,
                            )
                    })
                    .filter_map(|call| {
                        match abi::neurolend_contract::functions::TransferOwnership::decode(call) {
                            Ok(decoded_call) => Some(contract::NeurolendTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_neurolend_events(&blk, &mut events);
    map_erc20_events(&blk, &mut events);
    map_generic_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    map_neurolend_calls(&blk, &mut calls);
    Ok(calls)
}
