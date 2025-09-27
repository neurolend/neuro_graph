const { ethers } = require('ethers');
const fs = require('fs');

// Configuration
const RPC_URL = 'https://evmrpc.0g.ai';
const CONTRACT_ADDRESS = '0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23';
const STARTING_BLOCK = 6914309;
const BATCH_SIZE = 100;

// Event signatures from your Rust code
const EVENT_SIGNATURES = {
    'CollateralAdded(uint256,address,uint256,uint256,uint256)': 'CollateralAdded',
    'CollateralRemoved(uint256,address,uint256,uint256,uint256)': 'CollateralRemoved',
    'LoanAccepted(uint256,address,uint256,uint256)': 'LoanAccepted',
    'LoanCreated(uint256,address,address,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256)': 'LoanCreated',
    'LoanLiquidated(uint256,address,uint256,uint256,uint256)': 'LoanLiquidated',
    'LoanMatched(uint256,uint256,uint256,address,address,uint256,uint256,uint256)': 'LoanMatched',
    'LoanOfferCancelled(uint256,address,uint256)': 'LoanOfferCancelled',
    'LoanOfferRemoved(uint256,string)': 'LoanOfferRemoved',
    'LoanRepaid(uint256,address,uint256,uint256)': 'LoanRepaid',
    'LoanRequestCancelled(uint256,address,uint256)': 'LoanRequestCancelled',
    'LoanRequestCreated(uint256,address,address,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256)': 'LoanRequestCreated',
    'LoanRequestRemoved(uint256,string)': 'LoanRequestRemoved',
    'OwnershipTransferred(address,address)': 'OwnershipTransferred',
    'PartialRepayment(uint256,address,uint256,uint256,uint256,uint256)': 'PartialRepayment',
    'PriceFeedSet(address,bytes32)': 'PriceFeedSet',
    'PriceUpdatePaid(uint256,uint256,uint256)': 'PriceUpdatePaid'
};

class SimpleIndexer {
    constructor() {
        this.provider = new ethers.JsonRpcProvider(RPC_URL);
        this.events = [];
        this.currentBlock = STARTING_BLOCK;
    }

    async start() {
        console.log('🚀 Starting NeuroLend Indexer...');
        console.log(`📡 RPC: ${RPC_URL}`);
        console.log(`📋 Contract: ${CONTRACT_ADDRESS}`);
        console.log(`🔢 Starting Block: ${STARTING_BLOCK}`);
        
        try {
            const latestBlock = await this.provider.getBlockNumber();
            console.log(`📊 Latest Block: ${latestBlock}`);
            
            // Index historical blocks
            await this.indexBlocks(this.currentBlock, latestBlock);
            
            // Start real-time monitoring
            this.startRealTimeMonitoring();
            
        } catch (error) {
            console.error('❌ Error starting indexer:', error);
        }
    }

    async indexBlocks(fromBlock, toBlock) {
        console.log(`📚 Indexing blocks ${fromBlock} to ${toBlock}...`);
        
        for (let start = fromBlock; start <= toBlock; start += BATCH_SIZE) {
            const end = Math.min(start + BATCH_SIZE - 1, toBlock);
            
            try {
                const filter = {
                    address: CONTRACT_ADDRESS,
                    fromBlock: start,
                    toBlock: end
                };
                
                const logs = await this.provider.getLogs(filter);
                console.log(`📦 Block ${start}-${end}: Found ${logs.length} events`);
                
                for (const log of logs) {
                    await this.processLog(log);
                }
                
                // Save events to file
                this.saveEvents();
                
            } catch (error) {
                console.error(`❌ Error processing blocks ${start}-${end}:`, error);
            }
        }
        
        this.currentBlock = toBlock + 1;
    }

    async processLog(log) {
        try {
            const block = await this.provider.getBlock(log.blockNumber);
            const tx = await this.provider.getTransaction(log.transactionHash);
            
            const eventData = {
                event_name: 'Unknown',
                transaction_hash: log.transactionHash,
                block_number: log.blockNumber,
                block_timestamp: block.timestamp,
                log_index: log.logIndex,
                contract_address: log.address,
                topics: log.topics,
                data: log.data
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
            console.log(`✅ ${eventData.event_name} event processed`);
            
        } catch (error) {
            console.error('❌ Error processing log:', error);
        }
    }

    saveEvents() {
        if (this.events.length > 0) {
            const filename = `events_${Date.now()}.json`;
            fs.writeFileSync(filename, JSON.stringify(this.events, null, 2));
            console.log(`💾 Saved ${this.events.length} events to ${filename}`);
            this.events = []; // Clear processed events
        }
    }

    startRealTimeMonitoring() {
        console.log('👀 Starting real-time monitoring...');
        
        this.provider.on('block', async (blockNumber) => {
            console.log(`🔄 New block: ${blockNumber}`);
            await this.indexBlocks(this.currentBlock, blockNumber);
        });
    }
}

// Start the indexer
const indexer = new SimpleIndexer();
indexer.start().catch(console.error);
