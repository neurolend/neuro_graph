# 🧠 NeuroLend Indexer & API

A complete blockchain indexing solution for NeuroLend protocol on **0G Network**, featuring custom indexer and REST API since 0G lacks Substreams infrastructure.

## **Quick Start**

### **Option 1: Docker (Recommended)**

```bash
cd deployment && docker-compose up --build
```

### **Option 2: Local Development**

```bash
# Terminal 1: Start indexer
cd custom_indexer && cargo run --bin neurolend_indexer

# Terminal 2: Start API server
cd api_server && cargo run --bin api_server
```

## 🏗️ **Architecture**

- **Custom Indexer**: Replaces Substreams, fetches events from 0G Network RPC
- **REST API**: Serves indexed data to frontends via HTTP endpoints
- **Event Processing**: Tracks 16 NeuroLend events (LoanCreated, CollateralAdded, etc.)
- **Real-time Monitoring**: Continuous blockchain scanning + historical processing

## **API Endpoints**

Base URL: `http://localhost:3001`

| Endpoint                  | Description          |
| ------------------------- | -------------------- |
| `GET /events`             | All NeuroLend events |
| `GET /events/LoanCreated` | Specific event type  |
| `GET /loans`              | Aggregated loan data |
| `GET /stats`              | Protocol statistics  |
| `GET /health`             | Service health check |

## 🌐 **NeuroLend Contract**

**Address**: `0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23`  
**Network**: 0G Network (Chain ID: 16661)  
**RPC**: https://evmrpc.0g.ai  
**Starting Block**: 6,914,309

## 🔧 **Components**

- **`custom_indexer/`**: Rust indexer for 0G Network
- **`api_server/`**: REST API server with CORS support
- **`deployment/`**: Docker configs and deployment scripts
- **`abi/`**: NeuroLend contract ABI definitions

## 💡 **Why Custom Solution?**

0G Network lacks Substreams/Firehose infrastructure, so we built:

- ✅ Direct RPC integration
- ✅ Batch processing (1000 blocks/batch)
- ✅ Real-time event monitoring
- ✅ REST API for frontend integration
- ✅ Docker containerization

## 🎯 **Frontend Integration**

```javascript
const events = await fetch("http://your-api/events").then((r) => r.json());
const loans = await fetch("http://your-api/loans").then((r) => r.json());
```

Built for 0G Network using The Graph Substream as Track • Powered by Rust • Ready for Production 🚀
