use anyhow::Result;
use ethers::prelude::*;

const RPC_URL: &str = "https://evmrpc.0g.ai";
const NEUROLEND_CONTRACT: &str = "0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing connection to 0G network...");

    // Connect to 0G network
    let provider = Provider::<Http>::try_from(RPC_URL)?;

    // Test basic connectivity
    let chain_id = provider.get_chainid().await?;
    println!("✅ Connected! Chain ID: {}", chain_id);

    let current_block = provider.get_block_number().await?;
    println!("✅ Current block: {}", current_block);

    // Test contract exists
    let contract_address: Address = NEUROLEND_CONTRACT.parse()?;
    let code = provider.get_code(contract_address, None).await?;

    if code.len() > 0 {
        println!("✅ NeuroLend contract found at {}", NEUROLEND_CONTRACT);
        println!("   Contract code size: {} bytes", code.len());
    } else {
        println!("❌ No contract found at {}", NEUROLEND_CONTRACT);
    }

    // Test getting recent logs
    println!("\nTesting recent logs...");
    let filter = Filter::new()
        .address(contract_address)
        .from_block(current_block.saturating_sub(1000u64.into())) // Last 1000 blocks
        .to_block(current_block);

    match provider.get_logs(&filter).await {
        Ok(logs) => {
            println!("✅ Found {} logs in last 1000 blocks", logs.len());
            if !logs.is_empty() {
                let latest_log = &logs[logs.len() - 1];
                println!(
                    "   Latest log: block {}, tx 0x{:x}",
                    latest_log.block_number.unwrap_or_default(),
                    latest_log.transaction_hash.unwrap_or_default()
                );
            }
        }
        Err(e) => {
            println!("❌ Error getting logs: {}", e);
        }
    }

    println!("\n🎉 Connection test complete!");
    println!("You can now run: cargo run");

    Ok(())
}
