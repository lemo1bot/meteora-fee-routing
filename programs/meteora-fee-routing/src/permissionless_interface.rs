use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::dlmm_integration::meteora_dlmm_types::*;
use crate::dlmm_integration::cpi_instructions;

/// Permissionless interface for fee routing operations
pub struct PermissionlessInterface;

impl PermissionlessInterface {
    /// Anyone can call this to collect fees from a DLMM position
    pub fn collect_position_fees(
        ctx: &Context<CollectPositionFees>,
    ) -> Result<(u64, u64)> {
        // Get balances before fee collection
        let base_balance_before = ctx.accounts.base_token_account.amount;
        let quote_balance_before = ctx.accounts.quote_token_account.amount;

        // Call DLMM program to claim position fees
        let cpi_accounts = cpi_instructions::ClaimFee {
            position: ctx.accounts.position.clone(),
            lb_pair: ctx.accounts.lb_pair.clone(),
            user_token_x: ctx.accounts.base_token_account.clone(),
            user_token_y: ctx.accounts.quote_token_account.clone(),
            reserve_x: ctx.accounts.reserve_x.clone(),
            reserve_y: ctx.accounts.reserve_y.clone(),
            token_x_mint: ctx.accounts.base_mint.clone(),
            token_y_mint: ctx.accounts.quote_mint.clone(),
            token_program: ctx.accounts.token_program.clone(),
            owner: ctx.accounts.fee_vault.clone(),
        };

        let fee_vault = &ctx.accounts.fee_vault;
        let seeds = &[
            b"fee_vault",
            fee_vault.dlmm_pool.as_ref(),
            &[fee_vault.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_program = ctx.accounts.dlmm_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        cpi_instructions::claim_fee(cpi_ctx)?;

        // Calculate collected amounts
        let base_collected = ctx.accounts.base_token_account.amount
            .saturating_sub(base_balance_before);
        let quote_collected = ctx.accounts.quote_token_account.amount
            .saturating_sub(quote_balance_before);

        Ok((base_collected, quote_collected))
    }
}

#[derive(Accounts)]
pub struct CollectPositionFees<'info> {
    #[account(
        mut,
        seeds = [b"fee_vault", dlmm_pool.key().as_ref()],
        bump = fee_vault.bump
    )]
    pub fee_vault: Account<'info, FeeVault>,

    /// CHECK: DLMM pool account
    #[account(mut)]
    pub dlmm_pool: UncheckedAccount<'info>,

    #[account(mut)]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub lb_pair: Account<'info, LbPair>,

    #[account(
        mut,
        associated_token::mint = base_mint,
        associated_token::authority = fee_vault
    )]
    pub base_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_mint,
        associated_token::authority = fee_vault
    )]
    pub quote_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub reserve_x: Account<'info, TokenAccount>,

    #[account(mut)]
    pub reserve_y: Account<'info, TokenAccount>,

    pub base_mint: Account<'info, Mint>,
    pub quote_mint: Account<'info, Mint>,

    pub collector: Signer<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK: DLMM program
    pub dlmm_program: UncheckedAccount<'info>,
}

use crate::{FeeVault, FeeRoutingError};