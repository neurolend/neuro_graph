#!/bin/bash

# Test script for subgraph-compatible API endpoints
BASE_URL="http://localhost:3001"

echo "🧪 Testing Subgraph-Compatible API Endpoints"
echo "=============================================="

# Test health check
echo "1. Testing health check..."
curl -s "$BASE_URL/health" | jq '.'
echo ""

# Test individual endpoints
echo "2. Testing loanAccepteds..."
curl -s "$BASE_URL/loanAccepteds" | jq '. | length'
echo ""

echo "3. Testing loanCreateds..."
curl -s "$BASE_URL/loanCreateds" | jq '. | length'
echo ""

echo "4. Testing loanLiquidateds..."
curl -s "$BASE_URL/loanLiquidateds" | jq '. | length'
echo ""

echo "5. Testing loanOfferCancelleds..."
curl -s "$BASE_URL/loanOfferCancelleds" | jq '. | length'
echo ""

echo "6. Testing loanOfferRemoveds..."
curl -s "$BASE_URL/loanOfferRemoveds" | jq '. | length'
echo ""

echo "7. Testing loanRepaids..."
curl -s "$BASE_URL/loanRepaids" | jq '. | length'
echo ""

echo "8. Testing priceFeedSets..."
curl -s "$BASE_URL/priceFeedSets" | jq '. | length'
echo ""

echo "9. Testing protocolStats_collection..."
curl -s "$BASE_URL/protocolStats_collection" | jq '.'
echo ""

echo "10. Testing tokens..."
curl -s "$BASE_URL/tokens" | jq '.'
echo ""

echo "11. Testing combined query..."
curl -s "$BASE_URL/query" | jq 'keys'
echo ""

echo "12. Testing GraphQL-style endpoint..."
curl -s "$BASE_URL/graphql" | jq '.data | keys'
echo ""

echo "✅ All endpoint tests completed!"
