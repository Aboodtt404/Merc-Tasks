#!/bin/bash

echo "🛠️ Starting Open Lot Development Environment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Check if dfx is running, start if not
if ! dfx ping > /dev/null 2>&1; then
    print_status "Starting DFX replica in background..."
    dfx start --clean --background
    sleep 5
fi

# Deploy canisters
print_status "Deploying canisters..."
dfx deploy

# Get canister IDs
BACKEND_CANISTER_ID=$(dfx canister id open_lot_backend)
FRONTEND_CANISTER_ID=$(dfx canister id open_lot_frontend)

print_success "Development environment ready!"
echo ""
echo "🌐 Access URLs:"
echo "  Frontend: http://localhost:4943/?canisterId=$FRONTEND_CANISTER_ID"
echo "  Backend:  http://localhost:4943/?canisterId=$BACKEND_CANISTER_ID"
echo ""
echo "🔧 For frontend development with hot reload:"
echo "  cd src/open_lot_frontend && npm run dev"
echo ""
echo "🛑 To stop the replica:"
echo "  dfx stop" 