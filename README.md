# NeuroLend Indexer & API

A production-ready blockchain indexer and REST API for the NeuroLend protocol on 0G Chain.

## Features

- **Real-time blockchain indexing** - Monitors 0G Chain for NeuroLend contract events
- **REST API** - Provides endpoints compatible with your frontend
- **Production ready** - Includes Docker, health checks, and graceful shutdown
- **Persistent storage** - Events are stored in JSON files with state management
- **Auto-recovery** - Resumes from last indexed block on restart

## Quick Start

### Local Development

1. **Install dependencies:**

```bash
npm install
```

2. **Start both indexer and API:**

```bash
npm run dev
```

Or run separately:

```bash
# Terminal 1 - Start indexer
npm run indexer

# Terminal 2 - Start API server
npm start
```

3. **Test the API:**

```bash
curl http://localhost:3001/health
curl http://localhost:3001/api/stats
curl http://localhost:3001/api/loanCreateds
```

### Docker Deployment

1. **Using Docker Compose (recommended):**

```bash
docker-compose up -d
```

2. **Using Docker directly:**

```bash
# Build image
docker build -t neurolend-api .

# Run indexer
docker run -d --name neurolend-indexer \
  -v indexer_data:/app/indexer_output \
  neurolend-api npm run indexer

# Run API
docker run -d --name neurolend-api \
  -p 3001:3001 \
  -v indexer_data:/app/indexer_output \
  neurolend-api npm start
```

## Production Deployment

### Railway

1. **Connect your GitHub repo to Railway**
2. **Set environment variables:**

   - `PORT=3001`
   - `RPC_URL=https://evmrpc.0g.ai`
   - `CONTRACT_ADDRESS=0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23`

3. **Deploy** - Railway will automatically use the `railway.json` config

### Render/Heroku

1. **Create two services:**

   - **API Service**: `npm start`
   - **Worker Service**: `npm run indexer`

2. **Set environment variables** (same as Railway)

3. **Add persistent storage** for the indexer output directory

### VPS/Cloud Server

1. **Clone and setup:**

```bash
git clone <your-repo>
cd neuro_graph
npm install
```

2. **Create environment file:**

```bash
cp env.example .env
# Edit .env with your settings
```

3. **Use PM2 for process management:**

```bash
npm install -g pm2

# Start indexer
pm2 start simple_indexer.js --name "neurolend-indexer"

# Start API
pm2 start simple_api.js --name "neurolend-api"

# Save PM2 configuration
pm2 save
pm2 startup
```

## API Endpoints

All endpoints return JSON data compatible with your frontend:

### Event Endpoints

- `GET /api/loanCreateds` - Loan creation events
- `GET /api/loanAccepteds` - Loan acceptance events
- `GET /api/loanRepaids` - Loan repayment events
- `GET /api/loanLiquidateds` - Loan liquidation events
- `GET /api/loanOfferCancelleds` - Loan offer cancellation events
- `GET /api/loanOfferRemoveds` - Loan offer removal events
- `GET /api/priceFeedSets` - Price feed update events

### General Endpoints

- `GET /api/events` - All events with pagination
- `GET /api/stats` - Event statistics and counts
- `GET /health` - Health check
- `POST /api/reload` - Manually reload events

### Query Parameters

- `limit` - Number of results (default: 100)
- `offset` - Skip results (default: 0)
- `event_name` - Filter by event type

## Configuration

### Environment Variables

| Variable           | Default                                    | Description                  |
| ------------------ | ------------------------------------------ | ---------------------------- |
| `PORT`             | 3001                                       | API server port              |
| `RPC_URL`          | https://evmrpc.0g.ai                       | 0G Chain RPC endpoint        |
| `CONTRACT_ADDRESS` | 0xD9aB5190eFA86eB955C5e146ccb30421faBc3405 | NeuroLend contract           |
| `STARTING_BLOCK`   | 7039846                                    | Block to start indexing from |
| `BATCH_SIZE`       | 100                                        | Events to process per batch  |
| `OUTPUT_DIR`       | ./indexer_output                           | Directory for event storage  |

## Monitoring

### Health Checks

- **API**: `GET /health` - Returns server status and event count
- **Indexer**: Checks for `indexer_state.json` file

### Logs

- Events are logged with timestamps and block numbers
- Errors include context for debugging
- Graceful shutdown handling

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   0G Chain      │    │   Indexer       │    │   API Server    │
│                 │───▶│                 │───▶│                 │
│ Smart Contract  │    │ - Monitors RPC  │    │ - Serves REST   │
│ Events          │    │ - Saves JSON    │    │ - CORS enabled  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │                        │
                              ▼                        │
                       ┌─────────────────┐             │
                       │ indexer_output/ │◀────────────┘
                       │ - events_*.json │
                       │ - state.json    │
                       └─────────────────┘
```

## Frontend Integration

Your frontend should work without changes since the API endpoints match your existing `useRestApi.ts` hooks:

```typescript
// Your existing frontend code will work with:
const BASE_API_URL = "https://your-deployed-api.railway.app";
```

## Troubleshooting

### Common Issues

1. **No events loading**

   - Check if indexer is running: `curl http://localhost:3001/health`
   - Verify RPC connection: Check indexer logs
   - Ensure output directory exists and is writable

2. **API returning empty arrays**

   - Wait for indexer to process blocks
   - Check `indexer_output/` directory for JSON files
   - Manually reload: `curl -X POST http://localhost:3001/api/reload`

3. **High memory usage**
   - Reduce `BATCH_SIZE` environment variable
   - Implement event cleanup for old files

### Logs Location

- **Local**: Console output
- **Docker**: `docker logs <container-name>`
- **Railway**: Available in dashboard
- **PM2**: `pm2 logs`

## License

MIT
