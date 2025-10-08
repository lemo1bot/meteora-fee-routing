use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;

mod dlmm_integration;
mod permissionless_interface;

use dlmm_integration::*;
use permissionless_interface::*;

declare_id!("FeeRtG9mEpMFEBPqhN5xjLrP4KdE5FGHxFpEhGkGKQW");

#[program]
pub mod meteora_fee_routing {
    use super::*;

    /// Initialize the fee routing vault for a specific DLMM pool
    pub fn initialize_fee_vault(
        ctx: Context<InitializeFeeVault>,
        bump: u8,
    ) -> Result<()> {
        let fee_vault = &mut ctx.accounts.fee_vault;
        fee_vault.authority = ctx.accounts.authority.key();
        fee_vault.dlmm_pool = ctx.accounts.dlmm_pool.key();
        fee_vault.base_mint = ctx.accounts.base_mint.key();
        fee_vault.quote_mint = ctx.accounts.quote_mint.key();
        fee_vault.bump = bump;
        fee_vault.total_collected_base = 0;
        fee_vault.total_collected_quote = 0;
        fee_vault.creation_time = Clock::get()?.unix_timestamp;
        
        msg!("Fee vault initialized for DLMM pool: {}", ctx.accounts.dlmm_pool.key());
        Ok(())
    }

    /// Collect fees from DLMM position (permissionless)
    pub fn collect_position_fees(
        ctx: Context<CollectPositionFees>,
    ) -> Result<()> {
        let (base_collected, quote_collected) = PermissionlessInterface::collect_position_fees(&ctx)?;
        
        let fee_vault = &mut ctx.accounts.fee_vault;
        fee_vault.total_collected_base = fee_vault.total_collected_base
            .checked_add(base_collected)
            .ok_or(FeeRoutingError::MathOverflow)?;
        fee_vault.total_collected_quote = fee_vault.total_collected_quote
            .checked_add(quote_collected)
            .ok_or(FeeRoutingError::MathOverflow)?;

        msg!(
            "Position fees collected - Base: {}, Quote: {}", 
            base_collected, 
            quote_collected
        );
        
        Ok(())
    }

    /// Distribute collected fees to specified recipients
    pub fn distribute_fees(
        ctx: Context<DistributeFees>,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        let fee_vault = &mut ctx.accounts.fee_vault;
        
        // Validate sufficient balance
        require!(
            ctx.accounts.base_token_account.amount >= base_amount,
            FeeRoutingError::InsufficientBalance
        );
        require!(
            ctx.accounts.quote_token_account.amount >= quote_amount,
            FeeRoutingError::InsufficientBalance
        );

        // Transfer tokens to recipient
        if base_amount > 0 {
            let transfer_instruction = anchor_spl::token::Transfer {
                from: ctx.accounts.base_token_account.to_account_info(),
                to: ctx.accounts.recipient_base_account.to_account_info(),
                authority: fee_vault.to_account_info(),
            };
            
            let seeds = &[
                b"fee_vault",
                fee_vault.dlmm_pool.as_ref(),
                &[fee_vault.bump],
            ];
            let signer = &[&seeds[..]];
            
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    transfer_instruction,
                    signer,
                ),
                base_amount,
            )?;
        }

        if quote_amount > 0 {
            let transfer_instruction = anchor_spl::token::Transfer {
                from: ctx.accounts.quote_token_account.to_account_info(),
                to: ctx.accounts.recipient_quote_account.to_account_info(),
                authority: fee_vault.to_account_info(),
            };
            
            let seeds = &[
                b"fee_vault",
                fee_vault.dlmm_pool.as_ref(),
                &[fee_vault.bump],
            ];
            let signer = &[&seeds[..]];
            
            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    transfer_instruction,
                    signer,
                ),
                quote_amount,
            )?;
        }

        msg!(
            "Distributed fees to {}: Base {}, Quote {}",
            ctx.accounts.recipient.key(),
            base_amount,
            quote_amount
        );

        Ok(())
    }

    /// Update fee vault configuration (only authority)
    pub fn update_fee_vault(
        ctx: Context<UpdateFeeVault>,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        let fee_vault = &mut ctx.accounts.fee_vault;
        
        if let Some(new_auth) = new_authority {
            fee_vault.authority = new_auth;
            msg!("Fee vault authority updated to: {}", new_auth);
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeFeeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = FeeVault::LEN,
        seeds = [b"fee_vault", dlmm_pool.key().as_ref()],
        bump
    )]
    pub fee_vault: Account<'info, FeeVault>,
    
    /// CHECK: This is the DLMM pool account
    pub dlmm_pool: UncheckedAccount<'info>,
    
    pub base_mint: Account<'info, Mint>,
    pub quote_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        associated_token::mint = base_mint,
        associated_token::authority = fee_vault
    )]
    pub base_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = authority,
        associated_token::mint = quote_mint,
        associated_token::authority = fee_vault
    )]
    pub quote_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    #[account(
        mut,
        seeds = [b"fee_vault", dlmm_pool.key().as_ref()],
        bump = fee_vault.bump
    )]
    pub fee_vault: Account<'info, FeeVault>,
    
    /// CHECK: This is the DLMM pool account
    #[account(mut)]
    pub dlmm_pool: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = fee_vault.base_mint,
        associated_token::authority = fee_vault
    )]
    pub base_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = fee_vault.quote_mint,
        associated_token::authority = fee_vault
    )]
    pub quote_token_account: Account<'info, TokenAccount>,
    
    pub collector: Signer<'info>,
    pub token_program: Program<'info, Token>,
    
    /// CHECK: This is the DLMM program
    pub dlmm_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct DistributeFees<'info> {
    #[account(
        mut,
        seeds = [b"fee_vault", fee_vault.dlmm_pool.as_ref()],
        bump = fee_vault.bump
    )]
    pub fee_vault: Account<'info, FeeVault>,
    
    #[account(
        mut,
        associated_token::mint = fee_vault.base_mint,
        associated_token::authority = fee_vault
    )]
    pub base_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = fee_vault.quote_mint,
        associated_token::authority = fee_vault
    )]
    pub quote_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub recipient_base_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub recipient_quote_account: Account<'info, TokenAccount>,
    
    /// CHECK: Fee recipient
    pub recipient: UncheckedAccount<'info>,
    
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateFeeVault<'info> {
    #[account(
        mut,
        seeds = [b"fee_vault", fee_vault.dlmm_pool.as_ref()],
        bump = fee_vault.bump,
        has_one = authority
    )]
    pub fee_vault: Account<'info, FeeVault>,
    
    pub authority: Signer<'info>,
}

#[account]
pub struct FeeVault {
    pub authority: Pubkey,
    pub dlmm_pool: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub bump: u8,
    pub total_collected_base: u64,
    pub total_collected_quote: u64,
    pub creation_time: i64,
}

impl FeeVault {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // dlmm_pool
        32 + // base_mint
        32 + // quote_mint
        1 +  // bump
        8 +  // total_collected_base
        8 +  // total_collected_quote
        8;   // creation_time
}

#[error_code]
pub enum FeeRoutingError {
    #[msg("Invalid DLMM pool provided")]
    InvalidDlmmPool,
    #[msg("Insufficient base token collection")]
    InsufficientBaseCollection,
    #[msg("Insufficient quote token collection")]
    InsufficientQuoteCollection,
    #[msg("Insufficient balance for distribution")]
    InsufficientBalance,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Unauthorized")]
    Unauthorized,
}