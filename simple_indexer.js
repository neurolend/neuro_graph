const { ethers } = require("ethers");
const fs = require("fs");
const path = require("path");

// Configuration - use environment variables for production
const RPC_URL = process.env.RPC_URL || "https://evmrpc.0g.ai";
const CONTRACT_ADDRESS =
  process.env.CONTRACT_ADDRESS || "0xD9aB5190eFA86eB955C5e146ccb30421faBc3405";
const STARTING_BLOCK = parseInt(process.env.STARTING_BLOCK || "7039846");
const BATCH_SIZE = parseInt(process.env.BATCH_SIZE || "100");
const OUTPUT_DIR = process.env.OUTPUT_DIR || "./indexer_output";

// Event signatures from your Rust code
const EVENT_SIGNATURES = {
  "CollateralAdded(uint256,address,uint256,uint256,uint256)": "CollateralAdded",
  "CollateralRemoved(uint256,address,uint256,uint256,uint256)":
    "CollateralRemoved",
  "LoanAccepted(uint256,address,uint256,uint256)": "LoanAccepted",
  "LoanCreated(uint256,address,address,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256)":
    "LoanCreated",
  "LoanLiquidated(uint256,address,uint256,uint256,uint256)": "LoanLiquidated",
  "LoanMatched(uint256,uint256,uint256,address,address,uint256,uint256,uint256)":
    "LoanMatched",
  "LoanOfferCancelled(uint256,address,uint256)": "LoanOfferCancelled",
  "LoanOfferRemoved(uint256,string)": "LoanOfferRemoved",
  "LoanRepaid(uint256,address,uint256,uint256)": "LoanRepaid",
  "LoanRequestCancelled(uint256,address,uint256)": "LoanRequestCancelled",
  "LoanRequestCreated(uint256,address,address,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256)":
    "LoanRequestCreated",
  "LoanRequestRemoved(uint256,string)": "LoanRequestRemoved",
  "OwnershipTransferred(address,address)": "OwnershipTransferred",
  "PartialRepayment(uint256,address,uint256,uint256,uint256,uint256)":
    "PartialRepayment",
  "PriceFeedSet(address,bytes32)": "PriceFeedSet",
  "PriceUpdatePaid(uint256,uint256,uint256)": "PriceUpdatePaid",
};

class ProductionIndexer {
  constructor() {
    this.provider = new ethers.JsonRpcProvider(RPC_URL);
    this.events = [];
    this.currentBlock = STARTING_BLOCK;
    this.isRunning = false;

    // Ensure output directory exists
    if (!fs.existsSync(OUTPUT_DIR)) {
      fs.mkdirSync(OUTPUT_DIR, { recursive: true });
    }

    // Load existing state if available
    this.loadState();
  }

  loadState() {
    const stateFile = path.join(OUTPUT_DIR, "indexer_state.json");
    if (fs.existsSync(stateFile)) {
      try {
        const state = JSON.parse(fs.readFileSync(stateFile, "utf8"));
        this.currentBlock = state.currentBlock || STARTING_BLOCK;
        console.log(
          `📋 Loaded state: resuming from block ${this.currentBlock}`
        );
      } catch (error) {
        console.error("❌ Error loading state:", error);
      }
    }
  }

  saveState() {
    const stateFile = path.join(OUTPUT_DIR, "indexer_state.json");
    const state = {
      currentBlock: this.currentBlock,
      lastUpdated: new Date().toISOString(),
    };
    fs.writeFileSync(stateFile, JSON.stringify(state, null, 2));
  }

  async start() {
    if (this.isRunning) {
      console.log("⚠️ Indexer is already running");
      return;
    }

    this.isRunning = true;
    console.log("🚀 Starting NeuroLend Production Indexer...");
    console.log(`📡 RPC: ${RPC_URL}`);
    console.log(`📋 Contract: ${CONTRACT_ADDRESS}`);
    console.log(`🔢 Starting Block: ${this.currentBlock}`);
    console.log(`📁 Output Directory: ${OUTPUT_DIR}`);

    try {
      const latestBlock = await this.provider.getBlockNumber();
      console.log(`📊 Latest Block: ${latestBlock}`);

      // Index historical blocks if needed
      if (this.currentBlock <= latestBlock) {
        await this.indexBlocks(this.currentBlock, latestBlock);
      }

      // Start real-time monitoring
      this.startRealTimeMonitoring();
    } catch (error) {
      console.error("❌ Error starting indexer:", error);
      this.isRunning = false;
    }
  }

  async stop() {
    console.log("🛑 Stopping indexer...");
    this.isRunning = false;
    this.saveEvents();
    this.saveState();
  }

  async indexBlocks(fromBlock, toBlock) {
    console.log(`📚 Indexing blocks ${fromBlock} to ${toBlock}...`);

    for (let start = fromBlock; start <= toBlock; start += BATCH_SIZE) {
      if (!this.isRunning) break;

      const end = Math.min(start + BATCH_SIZE - 1, toBlock);

      try {
        const filter = {
          address: CONTRACT_ADDRESS,
          fromBlock: start,
          toBlock: end,
        };

        const logs = await this.provider.getLogs(filter);
        console.log(`📦 Block ${start}-${end}: Found ${logs.length} events`);

        for (const log of logs) {
          await this.processLog(log);
        }

        // Save events periodically
        if (this.events.length >= 100) {
          this.saveEvents();
        }

        // Update current block
        this.currentBlock = end + 1;

        // Save state periodically
        if (start % (BATCH_SIZE * 10) === 0) {
          this.saveState();
        }
      } catch (error) {
        console.error(`❌ Error processing blocks ${start}-${end}:`, error);
        // Wait before retrying
        await new Promise((resolve) => setTimeout(resolve, 5000));
      }
    }
  }

  async processLog(log) {
    try {
      const block = await this.provider.getBlock(log.blockNumber);

      const eventData = {
        event_name: "Unknown",
        transaction_hash: log.transactionHash,
        block_number: log.blockNumber,
        block_timestamp: block.timestamp,
        log_index: log.logIndex,
        contract_address: log.address,
        topics: log.topics,
        data: log.data,
      };

      // Try to identify the event
      for (const [signature, name] of Object.entries(EVENT_SIGNATURES)) {
        const topic0 = ethers.id(signature);
        if (log.topics[0] === topic0) {
          eventData.event_name = name;
          break;
        }
      }

      this.events.push(eventData);
      console.log(
        `✅ ${eventData.event_name} event processed (Block: ${log.blockNumber})`
      );
    } catch (error) {
      console.error("❌ Error processing log:", error);
    }
  }

  saveEvents() {
    if (this.events.length > 0) {
      const timestamp = Date.now();
      const filename = path.join(OUTPUT_DIR, `events_${timestamp}.json`);
      fs.writeFileSync(filename, JSON.stringify(this.events, null, 2));
      console.log(`💾 Saved ${this.events.length} events to ${filename}`);
      this.events = []; // Clear processed events
    }
  }

  startRealTimeMonitoring() {
    console.log("👀 Starting real-time monitoring...");

    // Poll for new blocks every 30 seconds
    const pollInterval = setInterval(async () => {
      if (!this.isRunning) {
        clearInterval(pollInterval);
        return;
      }

      try {
        const latestBlock = await this.provider.getBlockNumber();
        if (latestBlock >= this.currentBlock) {
          console.log(
            `🔄 New blocks detected: ${this.currentBlock} to ${latestBlock}`
          );
          await this.indexBlocks(this.currentBlock, latestBlock);
        }
      } catch (error) {
        console.error("❌ Error in real-time monitoring:", error);
      }
    }, 30000); // 30 seconds
  }
}

// Handle graceful shutdown
process.on("SIGINT", async () => {
  console.log("\n🛑 Received SIGINT, shutting down gracefully...");
  if (indexer) {
    await indexer.stop();
  }
  process.exit(0);
});

process.on("SIGTERM", async () => {
  console.log("\n🛑 Received SIGTERM, shutting down gracefully...");
  if (indexer) {
    await indexer.stop();
  }
  process.exit(0);
});

// Start the indexer
const indexer = new ProductionIndexer();
indexer.start().catch(console.error);
