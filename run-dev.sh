#!/bin/bash

# Build and run the NFL Prediction Dashboard in development mode

echo "🏈 Starting NFL Prediction Dashboard (Development Mode)"
echo "=================================================="

# Set environment to dev
export ENV=dev

# Build and start services
echo "🐳 Building and starting Docker services..."
docker-compose up --build

echo "🎯 Dashboard should be available at:"
echo "   Frontend: http://localhost:80"
echo "   Backend API: http://localhost:8000"
echo "   SurrealDB: http://localhost:8080 (internal)"