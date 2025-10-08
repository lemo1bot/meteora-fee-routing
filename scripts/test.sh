#!/bin/bash

set -e

echo "🧪 Running Meteora Fee Routing tests..."

# Check dependencies
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Please install Anchor first."
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Please install Node.js and npm."
    exit 1
fi

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Build the program
echo "🔨 Building program..."
anchor build

# Start local validator in background if not running
if ! pgrep -f "solana-test-validator" > /dev/null; then
    echo "🏗️  Starting local validator..."
    solana-test-validator --reset --quiet &
    VALIDATOR_PID=$!
    echo "⏳ Waiting for validator to start..."
    sleep 5
    
    # Function to clean up validator
    cleanup() {
        echo "🧹 Cleaning up..."
        if [[ ! -z "$VALIDATOR_PID" ]]; then
            kill $VALIDATOR_PID 2>/dev/null || true
        fi
    }
    trap cleanup EXIT
else
    echo "✅ Local validator already running"
fi

# Configure Solana for localnet
echo "⚙️  Configuring for localnet..."
solana config set --url localhost

# Run tests
echo "🧪 Running tests..."
anchor test --skip-local-validator

echo "✅ All tests completed successfully!"

# Generate test report
cat > test-report.md << EOF
# Test Report

**Date:** $(date)
**Status:** ✅ PASSED

## Test Results

All tests have passed successfully. The program is ready for deployment.

### Tested Features

- Fee vault initialization
- Fee collection from DLMM positions
- Fee distribution to recipients
- Access control and authorization
- PDA derivation and validation
- Token account management

### Next Steps

1. Deploy to devnet: \`./scripts/deploy.sh devnet\`
2. Test on devnet with real DLMM pools
3. Deploy to mainnet when ready

EOF

echo "📄 Test report saved to test-report.md"