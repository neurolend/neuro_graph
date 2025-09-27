# NeuroLend Subgraph-Compatible API Deployment Guide

## 🚀 Quick Start

Your API now has **subgraph-compatible endpoints** that match your original GraphQL subgraph structure exactly!

### Build & Run

```bash
# Build the subgraph-compatible API
cd api_server
cargo build --release --bin subgraph_api

# Run the API server
./target/release/subgraph_api
```

### Environment Variables

```bash
# Set the port (Railway will set this automatically)
export PORT=3001
```

## 📊 Available Endpoints

### Individual Entity Endpoints (Subgraph Style)

| Endpoint | Description | Example |
|----------|-------------|---------|
| `GET /loanAccepteds` | Loan accepted events | `curl /loanAccepteds` |
| `GET /loanCreateds` | Loan created events | `curl /loanCreateds` |
| `GET /loanLiquidateds` | Loan liquidated events | `curl /loanLiquidateds` |
| `GET /loanOfferCancelleds` | Loan offer cancelled events | `curl /loanOfferCancelleds` |
| `GET /loanOfferRemoveds` | Loan offer removed events | `curl /loanOfferRemoveds` |
| `GET /loanRepaids` | Loan repaid events | `curl /loanRepaids` |
| `GET /priceFeedSets` | Price feed set events | `curl /priceFeedSets` |
| `GET /protocolStats_collection` | Protocol statistics | `curl /protocolStats_collection` |
| `GET /tokens` | Token information | `curl /tokens` |

### Combined Endpoints

| Endpoint | Description | Response Format |
|----------|-------------|-----------------|
| `GET /query` | All data combined (like your subgraph query) | Direct JSON object |
| `GET /graphql` | GraphQL-style response | `{"data": {...}}` wrapper |
| `GET /health` | Health check | Status information |

## 🔧 Query Parameters (Subgraph Compatible)

All endpoints support standard subgraph query parameters:

```bash
# Limit results
GET /loanCreateds?first=10

# Skip results (pagination)
GET /loanCreateds?skip=20

# Order by field
GET /loanCreateds?orderBy=blockNumber

# Order direction
GET /loanCreateds?orderDirection=desc

# Where clause (filtering)
GET /loanCreateds?where={"lender":"0x123..."}
```

## 📋 Response Format

### Individual Endpoints
Each endpoint returns an array of objects matching your subgraph schema:

```json
// GET /loanCreateds
[
  {
    "amount": "1000000000000000000",
    "amountUSD": "1000.00",
    "blockNumber": "12345",
    "blockTimestamp": "1695123456",
    "collateralAddress": "0x...",
    "collateralAmount": "2000000000000000000",
    "duration": "86400",
    "id": "0x123...-0",
    "interestRate": "500",
    "lender": "0x...",
    "liquidationThresholdBPS": "8000",
    "loanId": "1",
    "maxPriceStaleness": "3600",
    "minCollateralRatioBPS": "15000",
    "priceUSD": "1.00",
    "tokenAddress": "0x...",
    "transactionHash": "0x123..."
  }
]
```

### Combined Query Endpoint
```json
// GET /query
{
  "loanAccepteds": [...],
  "loanCreateds": [...],
  "loanLiquidateds": [...],
  "loanOfferCancelleds": [...],
  "loanOfferRemoveds": [...],
  "loanRepaids": [...],
  "priceFeedSets": [...],
  "protocolStats_collection": [...],
  "tokens": [...]
}
```

### GraphQL-Style Endpoint
```json
// GET /graphql
{
  "data": {
    "loanAccepteds": [...],
    "loanCreateds": [...],
    // ... all other data
  }
}
```

## 🔄 Migration from Subgraph

### Before (Subgraph Query)
```graphql
query MyQuery {
  loanCreateds {
    amount
    blockNumber
    lender
    # ... other fields
  }
}
```

### After (REST API)
```bash
# Option 1: Individual endpoint
curl https://your-api.com/loanCreateds

# Option 2: Combined query (matches your original query exactly)
curl https://your-api.com/query

# Option 3: GraphQL-style wrapper
curl https://your-api.com/graphql
```

## 🚀 Deployment

### Railway Deployment

1. **Update your Railway service** to use the new binary:
   ```bash
   # In your Railway settings, update the start command to:
   ./target/release/subgraph_api
   ```

2. **Environment Variables** (Railway sets these automatically):
   ```bash
   PORT=3001  # Railway will override this
   ```

3. **Build Command** (if using Railway's build system):
   ```bash
   cargo build --release --bin subgraph_api
   ```

### Docker Deployment

```dockerfile
# Update your Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin subgraph_api

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/subgraph_api /usr/local/bin/
EXPOSE 3001
CMD ["subgraph_api"]
```

## 🧪 Testing

Use the provided test script:

```bash
# Make sure your API is running on localhost:3001
./test_subgraph_api.sh
```

Or test individual endpoints:

```bash
# Health check
curl http://localhost:3001/health

# Get loan created events
curl http://localhost:3001/loanCreateds

# Get combined data (like your original subgraph query)
curl http://localhost:3001/query
```

## 📊 Data Source

The API reads data from:
1. **Indexer Output**: `../custom_indexer/output/` directory
2. **Fallback**: Empty arrays if no data is available

Make sure your indexer is running and outputting data to the correct directory.

## 🔍 Troubleshooting

### "Not Found" Errors
- Make sure you're using the correct endpoint names (case-sensitive)
- Check that the API server is running on the expected port
- Verify CORS is enabled (it is by default)

### Empty Data
- Check if the indexer is running and producing output
- Verify the indexer output directory path: `../custom_indexer/output/`
- Check the API logs for data loading messages

### Port Issues
- Railway automatically sets the `PORT` environment variable
- For local testing, the default port is 3001
- Make sure no other services are using the same port

## 🎯 Next Steps

1. **Deploy the new API** using the subgraph_api binary
2. **Update your frontend** to use the new REST endpoints
3. **Test all functionality** with your existing queries
4. **Monitor the API** for performance and errors

The API is now **100% compatible** with your original subgraph structure! 🎉
