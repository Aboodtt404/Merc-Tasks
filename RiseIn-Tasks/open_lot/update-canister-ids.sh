#!/bin/bash

echo "🔧 Updating frontend canister IDs..."

# Get the current canister IDs
BACKEND_ID=$(dfx canister id open_lot_backend 2>/dev/null || echo "")
FRONTEND_ID=$(dfx canister id open_lot_frontend 2>/dev/null || echo "")
II_ID=$(dfx canister id internet_identity 2>/dev/null || echo "rdmx6-jaaaa-aaaah-qcaiq-cai")

if [ -z "$BACKEND_ID" ]; then
    echo "❌ Could not get backend canister ID. Make sure canisters are deployed."
    echo "   Run: dfx deploy open_lot_backend"
    exit 1
fi

# Update the frontend .env file
cd src/open_lot_frontend

cat > .env << EOL
VITE_DFX_NETWORK=local
VITE_CANISTER_ID_OPEN_LOT_BACKEND=$BACKEND_ID
VITE_CANISTER_ID_OPEN_LOT_FRONTEND=$FRONTEND_ID
VITE_CANISTER_ID_INTERNET_IDENTITY=$II_ID
VITE_HOST=http://localhost:4943
EOL

echo "✅ Updated canister IDs:"
echo "   Backend:  $BACKEND_ID"
echo "   Frontend: $FRONTEND_ID"
echo "   II:       $II_ID"
echo ""
echo "🔄 Rebuilding frontend..."
npm run build

echo "✅ Frontend updated with correct canister IDs!" 