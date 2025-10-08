#!/bin/bash

set -e

echo "🚀 Starting Meteora Fee Routing deployment..."

# Check if Anchor is available
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Please install Anchor first."
    exit 1
fi

# Check if Solana CLI is available
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please install Solana CLI first."
    exit 1
fi

# Get network from argument or default to devnet
NETWORK=${1:-devnet}

echo "📡 Deploying to: $NETWORK"

# Validate network
if [[ "$NETWORK" != "devnet" && "$NETWORK" != "mainnet-beta" && "$NETWORK" != "localnet" ]]; then
    echo "❌ Invalid network. Use: devnet, mainnet-beta, or localnet"
    exit 1
fi

# Set Solana config
echo "⚙️  Configuring Solana for $NETWORK..."
solana config set --url "$NETWORK"

# Check wallet
echo "👛 Checking wallet..."
solana address

# Check balance
BALANCE=$(solana balance --lamports | head -n1)
echo "💰 Current balance: $BALANCE lamports"

if [[ "$BALANCE" -lt 100000000 ]]; then
    echo "⚠️  Low balance detected. You need at least 0.1 SOL for deployment."
    if [[ "$NETWORK" == "devnet" ]]; then
        echo "🪂 Requesting airdrop..."
        solana airdrop 1
    else
        echo "❌ Please fund your wallet and try again."
        exit 1
    fi
fi

# Build the program
echo "🔨 Building program..."
anchor build

# Deploy the program
echo "🚀 Deploying program to $NETWORK..."
anchor deploy --provider.cluster "$NETWORK"

# Verify deployment
echo "✅ Deployment completed!"
echo "📋 Program details:"
echo "   Network: $NETWORK"
echo "   Program ID: $(solana address -k target/deploy/meteora_fee_routing-keypair.json)"

# Save deployment info
cat > deployment-info.json << EOF
{
  "network": "$NETWORK",
  "programId": "$(solana address -k target/deploy/meteora_fee_routing-keypair.json)",
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "deployedBy": "$(solana address)"
}
EOF

echo "📄 Deployment info saved to deployment-info.json"
echo "🎉 Deployment successful!"