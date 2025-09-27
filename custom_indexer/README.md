# NeuroLend Custom Indexer for 0G Network

This is a custom indexer that replaces Substreams for indexing NeuroLend contract events on the 0G network.

## Why Custom Indexer?

0G network doesn't have Substreams/Firehose infrastructure, so we built a custom solution using standard Web3 RPC calls.

## Features

- ✅ Indexes historical blocks from your starting block (6914309)
- ✅ Real-time monitoring of new events
- ✅ Batch processing to handle large block ranges efficiently
- ✅ Reuses your existing event signatures and logic
- ✅ Outputs structured JSON data
- ✅ Error handling and retry logic

## Setup

1. **Install dependencies**:

   ```bash
   cd custom_indexer
   cargo build
   ```

2. **Run the indexer**:
   ```bash
   cargo run
   ```

## Configuration

Edit `src/main.rs` to modify:

- `STARTING_BLOCK`: Block to start indexing from
- `RPC_URL`: 0G network RPC endpoint
- `BATCH_SIZE`: Number of blocks to process in each batch

## Output

The indexer outputs JSON events like:

```json
{
  "event_name": "CollateralAdded",
  "transaction_hash": "0x...",
  "block_number": 6914350,
  "block_timestamp": 1695123456,
  "log_index": 0,
  "contract_address": "0x064c3e0a900743d9ac87c778d2f6d3d5819d4f23",
  "topics": ["0x...", "0x..."],
  "data": "0x..."
}
```

## Next Steps

You can extend this indexer to:

1. **Save to Database**: Add PostgreSQL/MongoDB storage
2. **Add Event Decoding**: Use your existing ABI to decode event parameters
3. **Add APIs**: Create REST/GraphQL endpoints
4. **Add Webhooks**: Send events to external services
5. **Add Metrics**: Monitor indexing performance

## Comparison with Substreams

| Feature          | Substreams         | Custom Indexer |
| ---------------- | ------------------ | -------------- |
| Infrastructure   | Requires Firehose  | Standard RPC   |
| Setup Complexity | Low (if supported) | Medium         |
| Performance      | Very High          | High           |
| Customization    | Limited            | Full Control   |
| 0G Support       | ❌ No              | ✅ Yes         |

## Performance Tips

- Adjust `BATCH_SIZE` based on RPC limits
- Add database indexing for queries
- Use connection pooling for high throughput
- Consider running multiple indexer instances for different block ranges
