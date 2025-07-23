#!/bin/bash

echo "🚀 Setting up Open Lot Frontend..."

cd src/open_lot_frontend

echo "📦 Installing dependencies..."
npm install

echo "🔧 Creating environment configuration..."
cat > .env.local << EOL
VITE_DFX_NETWORK=local
VITE_CANISTER_ID_OPEN_LOT_BACKEND=rrkah-fqaaa-aaaaa-aaaaq-cai
VITE_CANISTER_ID_INTERNET_IDENTITY=rdmx6-jaaaa-aaaah-qcaiq-cai
VITE_HOST=http://localhost:4943
EOL

echo "✅ Frontend setup complete!"
echo ""
echo "To start the development server:"
echo "  cd src/open_lot_frontend"
echo "  npm run dev"
echo ""
echo "To build for production:"
echo "  npm run build" 