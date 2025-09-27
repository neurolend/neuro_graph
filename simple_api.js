const express = require("express");
const cors = require("cors");
const fs = require("fs");
const path = require("path");

const app = express();
const PORT = process.env.PORT || 3001;
const INDEXER_OUTPUT_DIR = process.env.INDEXER_OUTPUT_DIR || "./indexer_output";

// Middleware
app.use(cors());
app.use(express.json());

// Store for events (in production, use a database)
let allEvents = [];
let lastLoadTime = 0;
const RELOAD_INTERVAL = 30000; // 30 seconds

// Load events from JSON files
function loadEvents() {
  try {
    if (!fs.existsSync(INDEXER_OUTPUT_DIR)) {
      console.log(`📁 Output directory ${INDEXER_OUTPUT_DIR} does not exist`);
      return;
    }

    const files = fs
      .readdirSync(INDEXER_OUTPUT_DIR)
      .filter((f) => f.startsWith("events_") && f.endsWith(".json"))
      .sort(); // Sort to load in chronological order

    const newEvents = [];

    for (const file of files) {
      try {
        const filePath = path.join(INDEXER_OUTPUT_DIR, file);
        const data = JSON.parse(fs.readFileSync(filePath, "utf8"));
        newEvents.push(...data);
      } catch (error) {
        console.error(`Error loading ${file}:`, error);
      }
    }

    // Remove duplicates based on transaction_hash + log_index
    const eventMap = new Map();
    for (const event of newEvents) {
      const key = `${event.transaction_hash}-${event.log_index}`;
      eventMap.set(key, event);
    }

    allEvents = Array.from(eventMap.values());
    lastLoadTime = Date.now();

    console.log(
      `📚 Loaded ${allEvents.length} unique events from ${files.length} files`
    );
  } catch (error) {
    console.error("❌ Error loading events:", error);
  }
}

// Auto-reload events periodically
setInterval(() => {
  if (Date.now() - lastLoadTime > RELOAD_INTERVAL) {
    loadEvents();
  }
}, RELOAD_INTERVAL);

// API Routes

// Get all events
app.get("/api/events", (req, res) => {
  const { limit = 100, offset = 0, event_name } = req.query;

  let filteredEvents = allEvents;

  if (event_name) {
    filteredEvents = allEvents.filter((e) => e.event_name === event_name);
  }

  const paginatedEvents = filteredEvents.slice(
    parseInt(offset),
    parseInt(offset) + parseInt(limit)
  );

  res.json({
    events: paginatedEvents,
    total: filteredEvents.length,
    limit: parseInt(limit),
    offset: parseInt(offset),
  });
});

// Get events by type (matching your subgraph queries)
app.get("/api/loanCreateds", (req, res) => {
  const loanCreatedEvents = allEvents
    .filter((e) => e.event_name === "LoanCreated")
    .map((e) => ({
      id: `${e.transaction_hash}-${e.log_index}`,
      loanId: e.topics[1], // First indexed parameter
      lender: e.topics[2], // Second indexed parameter
      tokenAddress: e.topics[3], // Third indexed parameter
      transactionHash: e.transaction_hash,
      blockNumber: e.block_number,
      blockTimestamp: e.block_timestamp,
      // Note: amount, interestRate, etc. would need ABI decoding
      // For now, returning raw data
      data: e.data,
    }));

  res.json(loanCreatedEvents);
});

app.get("/api/loanAccepteds", (req, res) => {
  const events = allEvents
    .filter((e) => e.event_name === "LoanAccepted")
    .map((e) => ({
      id: `${e.transaction_hash}-${e.log_index}`,
      loanId: e.topics[1],
      borrower: e.topics[2],
      transactionHash: e.transaction_hash,
      blockNumber: e.block_number,
      blockTimestamp: e.block_timestamp,
      data: e.data,
    }));

  res.json(events);
});

app.get("/api/loanLiquidateds", (req, res) => {
  const events = allEvents
    .filter((e) => e.event_name === "LoanLiquidated")
    .map((e) => ({
      id: `${e.transaction_hash}-${e.log_index}`,
      loanId: e.topics[1],
      liquidator: e.topics[2],
      transactionHash: e.transaction_hash,
      blockNumber: e.block_number,
      blockTimestamp: e.block_timestamp,
      data: e.data,
    }));

  res.json(events);
});

app.get("/api/loanOfferRemoveds", (req, res) => {
  const events = allEvents
    .filter((e) => e.event_name === "LoanOfferRemoved")
    .map((e) => ({
      id: `${e.transaction_hash}-${e.log_index}`,
      loanId: e.topics[1],
      transactionHash: e.transaction_hash,
      blockNumber: e.block_number,
      blockTimestamp: e.block_timestamp,
      data: e.data,
    }));

  res.json(events);
});

app.get("/api/loanOfferCancelleds", (req, res) => {
  const events = allEvents
    .filter((e) => e.event_name === "LoanOfferCancelled")
    .map((e) => ({
      id: `${e.transaction_hash}-${e.log_index}`,
      loanId: e.topics[1],
      lender: e.topics[2],
      transactionHash: e.transaction_hash,
      blockNumber: e.block_number,
      blockTimestamp: e.block_timestamp,
      data: e.data,
    }));

  res.json(events);
});

app.get("/api/loanRepaids", (req, res) => {
  const events = allEvents
    .filter((e) => e.event_name === "LoanRepaid")
    .map((e) => ({
      id: `${e.transaction_hash}-${e.log_index}`,
      loanId: e.topics[1],
      borrower: e.topics[2],
      transactionHash: e.transaction_hash,
      blockNumber: e.block_number,
      blockTimestamp: e.block_timestamp,
      data: e.data,
    }));

  res.json(events);
});

app.get("/api/priceFeedSets", (req, res) => {
  const events = allEvents
    .filter((e) => e.event_name === "PriceFeedSet")
    .map((e) => ({
      id: `${e.transaction_hash}-${e.log_index}`,
      tokenAddress: e.topics[1],
      feedAddress: e.topics[2],
      transactionHash: e.transaction_hash,
      blockNumber: e.block_number,
      blockTimestamp: e.block_timestamp,
      data: e.data,
    }));

  res.json(events);
});

// Health check
app.get("/health", (req, res) => {
  res.json({
    status: "healthy",
    events_count: allEvents.length,
    timestamp: new Date().toISOString(),
    last_load_time: new Date(lastLoadTime).toISOString(),
    indexer_output_dir: INDEXER_OUTPUT_DIR,
  });
});

// Stats endpoint
app.get("/api/stats", (req, res) => {
  const eventCounts = {};
  allEvents.forEach((e) => {
    eventCounts[e.event_name] = (eventCounts[e.event_name] || 0) + 1;
  });

  res.json({
    total_events: allEvents.length,
    event_types: eventCounts,
    latest_block:
      allEvents.length > 0
        ? Math.max(...allEvents.map((e) => e.block_number), 0)
        : 0,
    last_updated: new Date(lastLoadTime).toISOString(),
  });
});

// Reload events endpoint (for manual refresh)
app.post("/api/reload", (req, res) => {
  loadEvents();
  res.json({
    message: "Events reloaded",
    events_count: allEvents.length,
    timestamp: new Date().toISOString(),
  });
});

// Start server
app.listen(PORT, "0.0.0.0", () => {
  console.log(
    `🚀 NeuroLend Production API Server running on http://0.0.0.0:${PORT}`
  );
  console.log(`📊 Health check: http://localhost:${PORT}/health`);
  console.log(`📈 Stats: http://localhost:${PORT}/api/stats`);
  console.log(`📋 Events: http://localhost:${PORT}/api/events`);
  console.log(`📁 Indexer Output: ${INDEXER_OUTPUT_DIR}`);

  // Load initial events
  loadEvents();
});

// Graceful shutdown
process.on("SIGINT", () => {
  console.log("\n🛑 Received SIGINT, shutting down gracefully...");
  process.exit(0);
});

process.on("SIGTERM", () => {
  console.log("\n🛑 Received SIGTERM, shutting down gracefully...");
  process.exit(0);
});
