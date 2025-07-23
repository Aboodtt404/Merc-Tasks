#!/bin/bash

echo "🏗️ Building Open Lot Frontend for Canister Deployment..."

# Navigate to frontend directory
cd src/open_lot_frontend

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Create environment file for production
echo "🔧 Creating production environment..."
cat > .env.production << EOL
VITE_DFX_NETWORK=local
VITE_HOST=http://localhost:4943
EOL

# Build the frontend for production
echo "🚀 Building React app..."
npm run build

# Create .ic-assets.json for proper asset handling
echo "📝 Creating IC assets configuration..."
cat > dist/.ic-assets.json << EOL
[
  {
    "match": "**/*",
    "headers": {
      "cache-control": "public, max-age=31536000, immutable"
    }
  },
  {
    "match": "index.html",
    "headers": {
      "cache-control": "public, max-age=0, must-revalidate"
    }
  },
  {
    "match": "**/*.js",
    "headers": {
      "content-type": "application/javascript",
      "cache-control": "public, max-age=31536000, immutable"
    }
  },
  {
    "match": "**/*.css",
    "headers": {
      "content-type": "text/css",
      "cache-control": "public, max-age=31536000, immutable"
    }
  },
  {
    "match": "**/*.ico",
    "headers": {
      "content-type": "image/x-icon",
      "cache-control": "public, max-age=31536000, immutable"
    }
  },
  {
    "match": "**/*.svg",
    "headers": {
      "content-type": "image/svg+xml",
      "cache-control": "public, max-age=31536000, immutable"
    }
  }
]
EOL

echo "✅ Frontend build complete!"
echo "📁 Built files are in: src/open_lot_frontend/dist"
echo ""
echo "To deploy both canisters:"
echo "  dfx deploy"
echo ""
echo "To deploy only frontend:"
echo "  dfx deploy open_lot_frontend" 
