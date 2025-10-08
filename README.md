# Meteora Fee Routing Program

A permissionless fee routing Anchor program for Meteora DLMM V2 pools that enables automated fee collection and distribution.

## Overview

This program provides a decentralized solution for routing fees from Meteora DLMM V2 pools to specified recipients. The key features include:

- **Permissionless Operation**: Anyone can collect fees from DLMM pools
- **Automated Distribution**: Configurable fee distribution to multiple recipients
- **PDA-based Security**: Uses Program Derived Addresses for secure fund management
- **Integration with Meteora DLMM V2**: Direct integration with Meteora's Dynamic Liquidity Market Maker

## Key Features

### 🔓 Permissionless Fee Collection
- Any user can trigger fee collection from DLMM pools
- No special permissions required for basic operations
- Transparent and decentralized fee management

### 💰 Flexible Fee Distribution
- Support for multiple fee recipients
- Percentage-based allocation system
- Configurable distribution rules

### 🔒 Secure Fund Management
- PDA-controlled token accounts
- Authority-based access control
- Math overflow protection

### 🔗 DLMM V2 Integration
- Direct CPI calls to Meteora DLMM program
- Position fee collection support
- Protocol fee routing capabilities

## Architecture

### Core Components

1. **Fee Vault**: Central PDA account that manages collected fees
2. **DLMM Integration**: Module for interacting with Meteora DLMM program
3. **Permissionless Interface**: Public functions for fee collection
4. **Distribution Logic**: Automated fee routing to recipients

### Program Instructions

#### `initialize_fee_vault`
Creates a new fee vault for a specific DLMM pool.

**Parameters:**
- `bump`: PDA bump seed

**Accounts:**
- `fee_vault`: Fee vault PDA to initialize
- `dlmm_pool`: Target DLMM pool
- `base_mint`: Base token mint
- `quote_mint`: Quote token mint
- `authority`: Vault authority

#### `collect_position_fees`
Collects fees from DLMM position (permissionless).

**Accounts:**
- `fee_vault`: Target fee vault
- `position`: DLMM position to collect from
- `lb_pair`: DLMM liquidity book pair
- `collector`: Transaction signer (anyone)

#### `distribute_fees`
Distributes collected fees to recipients.

**Parameters:**
- `base_amount`: Base token amount to distribute
- `quote_amount`: Quote token amount to distribute

**Accounts:**
- `fee_vault`: Source fee vault
- `recipient_accounts`: Destination token accounts
- `authority`: Vault authority

#### `update_fee_vault`
Updates fee vault configuration (authority only).

**Parameters:**
- `new_authority`: Optional new authority

## Getting Started

### Prerequisites

- Rust 1.85.0+
- Anchor CLI 0.31.0+
- Node.js 18+
- Solana CLI tools

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd meteora-fee-routing
```

2. Install dependencies:
```bash
npm install
```

3. Build the program:
```bash
anchor build
```

4. Run tests:
```bash
anchor test
```

### Deployment

1. Configure your Solana wallet and RPC endpoint
2. Deploy to devnet:
```bash
anchor deploy --provider.cluster devnet
```

3. For mainnet deployment:
```bash
anchor deploy --provider.cluster mainnet-beta
```

## Usage Examples

### Initialize Fee Vault

```typescript
const [feeVault, bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("fee_vault"), dlmmPool.toBuffer()],
  program.programId
);

await program.methods
  .initializeFeeVault(bump)
  .accounts({
    feeVault,
    dlmmPool,
    baseMint,
    quoteMint,
    authority: authorityKeypair.publicKey,
    // ... other accounts
  })
  .signers([authorityKeypair])
  .rpc();
```

### Collect Fees (Permissionless)

```typescript
await program.methods
  .collectPositionFees()
  .accounts({
    feeVault,
    position: dlmmPosition,
    collector: anyKeypair.publicKey,
    // ... other accounts
  })
  .signers([anyKeypair])
  .rpc();
```

### Distribute Fees

```typescript
await program.methods
  .distributeFees(
    new anchor.BN(baseAmount),
    new anchor.BN(quoteAmount)
  )
  .accounts({
    feeVault,
    recipientBaseAccount,
    recipientQuoteAccount,
    authority: authorityKeypair.publicKey,
    // ... other accounts
  })
  .signers([authorityKeypair])
  .rpc();
```

## Program Addresses

### Devnet
- Program ID: `FeeRtG9mEpMFEBPqhN5xjLrP4KdE5FGHxFpEhGkGKQW`

### Mainnet
- Program ID: `FeeRtG9mEpMFEBPqhN5xjLrP4KdE5FGHxFpEhGkGKQW`

## Integration Guide

### For Frontend Applications

1. Import the program IDL and create program instance
2. Use `PublicKey.findProgramAddressSync()` to derive fee vault addresses
3. Call permissionless functions for fee collection
4. Monitor fee vault balances and distributions

### For Other Programs

1. Add this program as a dependency in your `Cargo.toml`
2. Import CPI instruction builders
3. Create cross-program invocations for fee routing

## Security Considerations

- All fee vaults are controlled by PDAs with deterministic addresses
- Authority validation ensures only authorized users can modify configurations
- Math operations include overflow protection
- Token transfers use Anchor's built-in safety checks

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Submit a pull request

## License

MIT License - see LICENSE file for details

## Support

For questions and support:
- GitHub Issues: [https://github.com/lemo1bot/meteora-fee-routing/issues](https://github.com/lemo1bot/meteora-fee-routing/issues)
- Repository: [https://github.com/lemo1bot/meteora-fee-routing](https://github.com/lemo1bot/meteora-fee-routing)
- Documentation: [README.md](https://github.com/lemo1bot/meteora-fee-routing/blob/main/README.md)

## Acknowledgments

- Meteora team for the DLMM V2 protocol
- Anchor framework for Solana development
- Solana Foundation for the blockchain infrastructure