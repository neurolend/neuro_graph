use ethers::prelude::*;
use ethers::utils::keccak256;
use std::collections::HashMap;

/// NeuroLend contract event signatures
/// Generated from the ABI file
pub fn get_event_signatures() -> HashMap<H256, &'static str> {
    let mut signatures = HashMap::new();

    // Calculate keccak256 hashes for each event signature
    // Format: EventName(type1,type2,...)

    // CollateralAdded(uint256,address,uint256,uint256,uint256)
    signatures.insert(
        keccak256("CollateralAdded(uint256,address,uint256,uint256,uint256)").into(),
        "CollateralAdded",
    );

    // CollateralRemoved(uint256,address,uint256,uint256,uint256)
    signatures.insert(
        keccak256("CollateralRemoved(uint256,address,uint256,uint256,uint256)").into(),
        "CollateralRemoved",
    );

    // LoanAccepted(uint256,address,uint256)
    signatures.insert(
        keccak256("LoanAccepted(uint256,address,uint256)").into(),
        "LoanAccepted",
    );

    // LoanCreated(uint256,address,address,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256,uint256,uint256)
    signatures.insert(
        keccak256("LoanCreated(uint256,address,address,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256,uint256,uint256)").into(),
        "LoanCreated"
    );

    // LoanLiquidated(uint256,address,uint256,uint256)
    signatures.insert(
        keccak256("LoanLiquidated(uint256,address,uint256,uint256)").into(),
        "LoanLiquidated",
    );

    // LoanMatched(uint256,uint256,address,address,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256,uint256,uint256)
    signatures.insert(
        keccak256("LoanMatched(uint256,uint256,address,address,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256,uint256,uint256)").into(),
        "LoanMatched"
    );

    // LoanOfferCancelled(uint256,address)
    signatures.insert(
        keccak256("LoanOfferCancelled(uint256,address)").into(),
        "LoanOfferCancelled",
    );

    // LoanOfferRemoved(uint256)
    signatures.insert(
        keccak256("LoanOfferRemoved(uint256)").into(),
        "LoanOfferRemoved",
    );

    // LoanRepaid(uint256,address,uint256,uint256)
    signatures.insert(
        keccak256("LoanRepaid(uint256,address,uint256,uint256)").into(),
        "LoanRepaid",
    );

    // LoanRequestCancelled(uint256,address)
    signatures.insert(
        keccak256("LoanRequestCancelled(uint256,address)").into(),
        "LoanRequestCancelled",
    );

    // LoanRequestCreated(uint256,address,address,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256,uint256,uint256)
    signatures.insert(
        keccak256("LoanRequestCreated(uint256,address,address,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256,uint256,uint256)").into(),
        "LoanRequestCreated"
    );

    // LoanRequestRemoved(uint256)
    signatures.insert(
        keccak256("LoanRequestRemoved(uint256)").into(),
        "LoanRequestRemoved",
    );

    // OwnershipTransferred(address,address)
    signatures.insert(
        keccak256("OwnershipTransferred(address,address)").into(),
        "OwnershipTransferred",
    );

    // PartialRepayment(uint256,address,uint256,uint256,uint256,uint256)
    signatures.insert(
        keccak256("PartialRepayment(uint256,address,uint256,uint256,uint256,uint256)").into(),
        "PartialRepayment",
    );

    // PriceFeedSet(address,address)
    signatures.insert(
        keccak256("PriceFeedSet(address,address)").into(),
        "PriceFeedSet",
    );

    // PriceUpdatePaid(uint256,uint256,uint256)
    signatures.insert(
        keccak256("PriceUpdatePaid(uint256,uint256,uint256)").into(),
        "PriceUpdatePaid",
    );

    signatures
}

/// Print all event signatures for debugging
pub fn print_event_signatures() {
    let signatures = get_event_signatures();
    println!("NeuroLend Event Signatures:");
    println!("===========================");

    for (hash, name) in signatures {
        println!("0x{:x} -> {}", hash, name);
    }
}
