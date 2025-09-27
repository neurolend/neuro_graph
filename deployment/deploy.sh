#!/bin/bash

echo "🚀 NeuroLend Deployment Script"
echo "============================="

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

echo "✅ Docker and Docker Compose found"

# Create data directory
mkdir -p ./data
echo "✅ Created data directory"

# Build and start services
echo "🔨 Building and starting services..."
docker-compose up --build -d

echo "⏳ Waiting for services to start..."
sleep 10

# Check service status
echo "📊 Service Status:"
echo "=================="
docker-compose ps

# Test API health
echo ""
echo "🔍 Testing API health..."
if curl -s http://localhost:3001/health > /dev/null; then
    echo "✅ API Server is healthy!"
    echo "🌐 API available at: http://localhost:3001"
    echo ""
    echo "📊 Available endpoints:"
    echo "  • http://localhost:3001/health - Health check"
    echo "  • http://localhost:3001/events - All events"
    echo "  • http://localhost:3001/loans - All loans"
    echo "  • http://localhost:3001/stats - Statistics"
else
    echo "❌ API Server is not responding"
fi

echo ""
echo "📝 Useful commands:"
echo "  • View logs: docker-compose logs -f"
echo "  • Stop services: docker-compose down"
echo "  • Restart: docker-compose restart"
echo ""
echo "🎉 Deployment complete!"
