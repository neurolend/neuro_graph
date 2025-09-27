const express = require('express');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Store for events (in production, use a database)
let allEvents = [];

// Load events from JSON files
function loadEvents() {
    const files = fs.readdirSync('.').filter(f => f.startsWith('events_') && f.endsWith('.json'));
    allEvents = [];
    
    for (const file of files) {
        try {
            const data = JSON.parse(fs.readFileSync(file, 'utf8'));
            allEvents.push(...data);
        } catch (error) {
            console.error(`Error loading ${file}:`, error);
        }
    }
    
    console.log(`📚 Loaded ${allEvents.length} events from ${files.length} files`);
}

// API Routes

// Get all events
app.get('/api/events', (req, res) => {
    const { limit = 100, offset = 0, event_name } = req.query;
    
    let filteredEvents = allEvents;
    
    if (event_name) {
        filteredEvents = allEvents.filter(e => e.event_name === event_name);
    }
    
    const paginatedEvents = filteredEvents
        .slice(parseInt(offset), parseInt(offset) + parseInt(limit));
    
    res.json({
        events: paginatedEvents,
        total: filteredEvents.length,
        limit: parseInt(limit),
        offset: parseInt(offset)
    });
});

// Get events by type (matching your subgraph queries)
app.get('/api/loanCreateds', (req, res) => {
    const loanCreatedEvents = allEvents
        .filter(e => e.event_name === 'LoanCreated')
        .map(e => ({
            id: e.transaction_hash,
            loanId: e.topics[1], // First indexed parameter
            lender: e.topics[2], // Second indexed parameter  
            tokenAddress: e.topics[3], // Third indexed parameter
            transactionHash: e.transaction_hash,
            blockNumber: e.block_number,
            blockTimestamp: e.block_timestamp,
            // Note: amount, interestRate, etc. would need ABI decoding
            // For now, returning raw data
            data: e.data
        }));
    
    res.json(loanCreatedEvents);
});

app.get('/api/loanLiquidateds', (req, res) => {
    const events = allEvents
        .filter(e => e.event_name === 'LoanLiquidated')
        .map(e => ({
            id: e.transaction_hash,
            loanId: e.topics[1],
            liquidator: e.topics[2],
            transactionHash: e.transaction_hash,
            blockNumber: e.block_number,
            blockTimestamp: e.block_timestamp,
            data: e.data
        }));
    
    res.json(events);
});

app.get('/api/loanOfferRemoveds', (req, res) => {
    const events = allEvents
        .filter(e => e.event_name === 'LoanOfferRemoved')
        .map(e => ({
            id: e.transaction_hash,
            loanId: e.topics[1],
            transactionHash: e.transaction_hash,
            blockNumber: e.block_number,
            blockTimestamp: e.block_timestamp,
            data: e.data
        }));
    
    res.json(events);
});

app.get('/api/loanOfferCancelleds', (req, res) => {
    const events = allEvents
        .filter(e => e.event_name === 'LoanOfferCancelled')
        .map(e => ({
            id: e.transaction_hash,
            loanId: e.topics[1],
            lender: e.topics[2],
            transactionHash: e.transaction_hash,
            blockNumber: e.block_number,
            blockTimestamp: e.block_timestamp,
            data: e.data
        }));
    
    res.json(events);
});

app.get('/api/loanRepaids', (req, res) => {
    const events = allEvents
        .filter(e => e.event_name === 'LoanRepaid')
        .map(e => ({
            id: e.transaction_hash,
            loanId: e.topics[1],
            borrower: e.topics[2],
            transactionHash: e.transaction_hash,
            blockNumber: e.block_number,
            blockTimestamp: e.block_timestamp,
            data: e.data
        }));
    
    res.json(events);
});

app.get('/api/priceFeedSets', (req, res) => {
    const events = allEvents
        .filter(e => e.event_name === 'PriceFeedSet')
        .map(e => ({
            id: e.transaction_hash,
            tokenAddress: e.topics[1],
            feedAddress: e.topics[2],
            transactionHash: e.transaction_hash,
            blockNumber: e.block_number,
            blockTimestamp: e.block_timestamp,
            data: e.data
        }));
    
    res.json(events);
});

// Health check
app.get('/health', (req, res) => {
    res.json({ 
        status: 'healthy', 
        events_count: allEvents.length,
        timestamp: new Date().toISOString()
    });
});

// Stats endpoint
app.get('/api/stats', (req, res) => {
    const eventCounts = {};
    allEvents.forEach(e => {
        eventCounts[e.event_name] = (eventCounts[e.event_name] || 0) + 1;
    });
    
    res.json({
        total_events: allEvents.length,
        event_types: eventCounts,
        latest_block: Math.max(...allEvents.map(e => e.block_number), 0)
    });
});

// Start server
app.listen(PORT, () => {
    console.log(`🚀 NeuroLend API Server running on http://localhost:${PORT}`);
    console.log(`📊 Health check: http://localhost:${PORT}/health`);
    console.log(`📈 Stats: http://localhost:${PORT}/api/stats`);
    console.log(`📋 Events: http://localhost:${PORT}/api/events`);
    
    // Load initial events
    loadEvents();
    
    // Reload events every 30 seconds
    setInterval(loadEvents, 30000);
});
