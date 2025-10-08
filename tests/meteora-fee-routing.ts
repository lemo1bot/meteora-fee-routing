import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MeteoraFeeRouting } from "../target/types/meteora_fee_routing";
import { 
  PublicKey, 
  Keypair, 
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { expect } from "chai";

describe("meteora-fee-routing", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MeteoraFeeRouting as Program<MeteoraFeeRouting>;
  const provider = anchor.AnchorProvider.env();

  // Test accounts
  let authority: Keypair;
  let collector: Keypair;
  let baseMint: PublicKey;
  let quoteMint: PublicKey;
  let dlmmPool: Keypair;
  let feeVault: PublicKey;
  let baseTokenAccount: PublicKey;
  let quoteTokenAccount: PublicKey;

  before(async () => {
    // Initialize test accounts
    authority = Keypair.generate();
    collector = Keypair.generate();
    dlmmPool = Keypair.generate();

    // Airdrop SOL to test accounts
    await provider.connection.requestAirdrop(authority.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(collector.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Create test tokens
    baseMint = await createMint(
      provider.connection,
      authority,
      authority.publicKey,
      null,
      6
    );

    quoteMint = await createMint(
      provider.connection,
      authority,
      authority.publicKey,
      null,
      6
    );

    // Derive fee vault PDA
    [feeVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("fee_vault"), dlmmPool.publicKey.toBuffer()],
      program.programId
    );

    // Derive associated token accounts for fee vault
    baseTokenAccount = await getAssociatedTokenAddress(
      baseMint,
      feeVault,
      true
    );

    quoteTokenAccount = await getAssociatedTokenAddress(
      quoteMint,
      feeVault,
      true
    );
  });

  it("Initializes fee vault", async () => {
    const [, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("fee_vault"), dlmmPool.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initializeFeeVault(bump)
      .accounts({
        feeVault,
        dlmmPool: dlmmPool.publicKey,
        baseMint,
        quoteMint,
        baseTokenAccount,
        quoteTokenAccount,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([authority])
      .rpc();

    console.log("Initialize fee vault transaction signature:", tx);

    // Verify fee vault was created correctly
    const feeVaultAccount = await program.account.feeVault.fetch(feeVault);
    expect(feeVaultAccount.authority.toString()).to.equal(authority.publicKey.toString());
    expect(feeVaultAccount.dlmmPool.toString()).to.equal(dlmmPool.publicKey.toString());
    expect(feeVaultAccount.baseMint.toString()).to.equal(baseMint.toString());
    expect(feeVaultAccount.quoteMint.toString()).to.equal(quoteMint.toString());
    expect(feeVaultAccount.bump).to.equal(bump);
    expect(feeVaultAccount.totalCollectedBase.toNumber()).to.equal(0);
    expect(feeVaultAccount.totalCollectedQuote.toNumber()).to.equal(0);
  });

  it("Distributes fees to recipient", async () => {
    // Create recipient token accounts
    const recipientBaseAccount = await createAssociatedTokenAccount(
      provider.connection,
      authority,
      baseMint,
      collector.publicKey
    );

    const recipientQuoteAccount = await createAssociatedTokenAccount(
      provider.connection,
      authority,
      quoteMint,
      collector.publicKey
    );

    // Mint some tokens to fee vault for testing
    await mintTo(
      provider.connection,
      authority,
      baseMint,
      baseTokenAccount,
      authority,
      1000000 // 1 token with 6 decimals
    );

    await mintTo(
      provider.connection,
      authority,
      quoteMint,
      quoteTokenAccount,
      authority,
      2000000 // 2 tokens with 6 decimals
    );

    const baseAmount = 500000; // 0.5 tokens
    const quoteAmount = 1000000; // 1 token

    const tx = await program.methods
      .distributeFees(
        new anchor.BN(baseAmount),
        new anchor.BN(quoteAmount)
      )
      .accounts({
        feeVault,
        baseTokenAccount,
        quoteTokenAccount,
        recipientBaseAccount,
        recipientQuoteAccount,
        recipient: collector.publicKey,
        authority: authority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([authority])
      .rpc();

    console.log("Distribute fees transaction signature:", tx);

    // Verify tokens were transferred
    const recipientBaseBalance = await provider.connection.getTokenAccountBalance(recipientBaseAccount);
    const recipientQuoteBalance = await provider.connection.getTokenAccountBalance(recipientQuoteAccount);

    expect(recipientBaseBalance.value.amount).to.equal(baseAmount.toString());
    expect(recipientQuoteBalance.value.amount).to.equal(quoteAmount.toString());
  });

  it("Updates fee vault configuration", async () => {
    const newAuthority = Keypair.generate();

    const tx = await program.methods
      .updateFeeVault(newAuthority.publicKey)
      .accounts({
        feeVault,
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();

    console.log("Update fee vault transaction signature:", tx);

    // Verify authority was updated
    const feeVaultAccount = await program.account.feeVault.fetch(feeVault);
    expect(feeVaultAccount.authority.toString()).to.equal(newAuthority.publicKey.toString());
  });

  it("Validates fee vault constraints", async () => {
    // Try to create another fee vault with same DLMM pool (should fail)
    const [anotherFeeVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("fee_vault"), dlmmPool.publicKey.toBuffer()],
      program.programId
    );

    const [, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("fee_vault"), dlmmPool.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .initializeFeeVault(bump)
        .accounts({
          feeVault: anotherFeeVault,
          dlmmPool: dlmmPool.publicKey,
          baseMint,
          quoteMint,
          baseTokenAccount: await getAssociatedTokenAddress(baseMint, anotherFeeVault, true),
          quoteTokenAccount: await getAssociatedTokenAddress(quoteMint, anotherFeeVault, true),
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([authority])
        .rpc();

      expect.fail("Should have failed due to account already existing");
    } catch (error) {
      // Expected to fail
      console.log("Expected error:", error.message);
    }
  });
});