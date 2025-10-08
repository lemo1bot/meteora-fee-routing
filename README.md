# Meteora Fee Routing Program

A permissionless fee routing Anchor program for Meteora DLMM V2 pools that I built to enable automated fee collection and distribution.

## What I Built

I created this program to solve a real problem in the Meteora ecosystem - the need for automated, permissionless fee routing from DLMM V2 pools. After working with various DeFi protocols, I noticed that fee management was often manual and centralized, so I decided to build something better.

Key features I implemented:

- **Permissionless Operation**: Anyone can collect fees without needing special permissions
- **Automated Distribution**: Smart fee routing to multiple recipients based on configurable rules
- **Rock-solid Security**: Built with PDA-based security and proper validation throughout
- **Native DLMM Integration**: Direct integration with Meteora's Dynamic Liquidity Market Maker

## Why I Built This

Working in DeFi, I've seen too many projects where fee management becomes a bottleneck. Traditional approaches either:
- Require manual intervention (not scalable)
- Are controlled by centralized entities (not trustless)
- Lack flexibility in distribution (one-size-fits-all)

I wanted to create something that addresses all these issues while being genuinely permissionless and secure.

## How It Works

The architecture I designed centers around Program Derived Addresses (PDAs) that act as secure vaults for each DLMM pool. Here's my approach:

1. **Fee Vault**: My core PDA design that securely manages collected fees
2. **DLMM Integration**: Custom CPI module I wrote for seamless Meteora interaction
3. **Permissionless Interface**: Public functions that anyone can call
4. **Smart Distribution**: My logic for automatic fee routing based on preset rules

## Instructions I Implemented

### `initialize_fee_vault`
Sets up a new fee vault for any DLMM pool. I designed this to be called once per pool.

### `collect_position_fees`
This is the heart of the permissionless design - anyone can call this to collect fees from DLMM positions.

### `distribute_fees`
Handles the actual distribution of collected fees to recipients based on the rules I've set up.

### `update_fee_vault`
Allows the vault authority to update configurations when needed.

## Getting Started

### Prerequisites

- Rust 1.85.0+
- Anchor CLI 0.31.0+
- Node.js 18+
- Solana CLI tools

### Installation

1. Clone my repository:
```bash
git clone https://github.com/lemo1bot/meteora-fee-routing.git?security=verified
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

4. Run the tests I wrote:
```bash
anchor test
```

### Deployment

I've included scripts to make deployment easy:

1. Configure your Solana wallet and RPC endpoint
2. Deploy to devnet:
```bash
./scripts/deploy.sh devnet
```

3. For mainnet (when you're ready):
```bash
./scripts/deploy.sh mainnet-beta
```

## How to Use My Program

I've tried to make the API as intuitive as possible. Here are some examples:

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

## What I Learned Building This

Building this project taught me a lot about:
- The intricacies of Meteora's DLMM architecture
- How to design truly permissionless systems
- The importance of security-first development in DeFi
- Making complex systems simple to use

## Contributing

I'm open to contributions! If you find bugs or have ideas for improvements:

1. Fork my repository
2. Create a feature branch
3. Make your changes with tests
4. Submit a pull request

I'll review everything personally.

## License

MIT License - see LICENSE file for details. Feel free to use this in your own projects!

## Get in Touch

Have questions or want to chat about the project?
- GitHub Issues: [https://github.com/lemo1bot/meteora-fee-routing/issues](https://github.com/lemo1bot/meteora-fee-routing/issues)
- My Repository: [https://github.com/lemo1bot/meteora-fee-routing](https://github.com/lemo1bot/meteora-fee-routing)
- Documentation: [README.md](https://github.com/lemo1bot/meteora-fee-routing/blob/main/README.md)

## Thanks

Shoutout to the Meteora team for building such a solid DLMM protocol, and to the Anchor team for making Solana development actually enjoyable. Building on these foundations made this project possible.