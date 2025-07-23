#!/bin/bash

echo "🚀 Deploying Open Lot Auction Platform..."
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if dfx is running
print_status "Checking DFX status..."
if ! dfx ping > /dev/null 2>&1; then
    print_error "DFX replica is not running. Please start it with 'dfx start'"
    exit 1
fi

print_success "DFX replica is running"

# Deploy backend canister first
print_status "Deploying backend canister..."
if dfx deploy open_lot_backend; then
    print_success "Backend canister deployed successfully"
else
    print_error "Failed to deploy backend canister"
    exit 1
fi

# Get backend canister ID
BACKEND_CANISTER_ID=$(dfx canister id open_lot_backend)
print_status "Backend canister ID: $BACKEND_CANISTER_ID"

# Build frontend
print_status "Building frontend for canister deployment..."
if ./build-frontend.sh; then
    print_success "Frontend built successfully"
else
    print_error "Failed to build frontend"
    exit 1
fi

# Update environment variables for frontend
print_status "Updating frontend environment variables..."
cd src/open_lot_frontend

# Create .env file with actual canister IDs
cat > .env << EOL
VITE_DFX_NETWORK=local
VITE_CANISTER_ID_OPEN_LOT_BACKEND=$BACKEND_CANISTER_ID
VITE_CANISTER_ID_INTERNET_IDENTITY=$(dfx canister id internet_identity 2>/dev/null || echo "rdmx6-jaaaa-aaaah-qcaiq-cai")
VITE_HOST=http://localhost:4943
EOL

# Rebuild with updated environment
print_status "Rebuilding with updated canister IDs..."
npm run build

cd ../..

# Deploy frontend canister
print_status "Deploying frontend canister..."
if dfx deploy open_lot_frontend; then
    print_success "Frontend canister deployed successfully"
else
    print_error "Failed to deploy frontend canister"
    exit 1
fi

# Get frontend canister ID
FRONTEND_CANISTER_ID=$(dfx canister id open_lot_frontend)

print_success "Deployment completed successfully!"
echo ""
echo "🎉 Open Lot Auction Platform is now live!"
echo "========================================"
echo ""
echo "📋 Canister Information:"
echo "  Backend:  $BACKEND_CANISTER_ID"
echo "  Frontend: $FRONTEND_CANISTER_ID"
echo ""
echo "🌐 Access URLs:"
echo "  Local Frontend: http://localhost:4943/?canisterId=$FRONTEND_CANISTER_ID"
echo "  Local Backend:  http://localhost:4943/?canisterId=$BACKEND_CANISTER_ID"
echo ""
echo "🔧 Useful Commands:"
echo "  Check canister status: dfx canister status --all"
echo "  View canister logs:    dfx canister logs <canister_id>"
echo "  Stop canisters:        dfx canister stop --all"
echo "  Start canisters:       dfx canister start --all"
echo ""
echo "📝 Next Steps:"
echo "  1. Open the frontend URL in your browser"
echo "  2. Connect with Internet Identity"
echo "  3. Create your first auction!"
echo ""
print_success "Happy auctioning! 🎯" 