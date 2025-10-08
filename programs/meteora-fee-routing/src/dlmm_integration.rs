use anchor_lang::prelude::*;

pub mod meteora_dlmm_types {
    use super::*;

    /// Meteora DLMM Program ID
    pub const DLMM_PROGRAM_ID: Pubkey = pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct LbPair {
        pub parameters: PoolParameters,
        pub v_parameters: PoolVariableParameters,
        pub bump_seed: [u8; 1],
        pub bin_step_seed: [u8; 2],
        pub pair_type: u8,
        pub active_id: i32,
        pub bin_step: u16,
        pub status: u8,
        pub require_base_factor_seed: u8,
        pub base_factor_seed: [u8; 2],
        pub padding1: [u8; 2],
        pub token_x_mint: Pubkey,
        pub token_y_mint: Pubkey,
        pub reserve_x: Pubkey,
        pub reserve_y: Pubkey,
        pub protocol_fee: ProtocolFee,
        pub fee_owner: Pubkey,
        pub reward_infos: [RewardInfo; 2],
        pub oracle: Pubkey,
        pub bin_array_bitmap: [u64; 512],
        pub last_updated_at: i64,
        pub whitelisted_wallet: Pubkey,
        pub pre_activation_swap_address: Pubkey,
        pub base_key: Pubkey,
        pub activation_type: u8,
        pub padding2: [u8; 1],
        pub activation_point: u64,
        pub padding3: [u8; 24],
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct PoolParameters {
        pub base_factor: u16,
        pub filter_period: u16,
        pub decay_period: u16,
        pub reduction_factor: u16,
        pub variable_fee_control: u32,
        pub protocol_share: u16,
        pub max_volatility_accumulator: u32,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct PoolVariableParameters {
        pub volatility_accumulator: u32,
        pub volatility_reference: u32,
        pub id_reference: i32,
        pub time_of_last_update: i64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct ProtocolFee {
        pub amount_x: u64,
        pub amount_y: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct RewardInfo {
        pub mint: Pubkey,
        pub vault: Pubkey,
        pub funder: Pubkey,
        pub reward_duration: u64,
        pub reward_duration_end: u64,
        pub reward_rate: u128,
        pub last_update_time: u64,
        pub cumulative_seconds_with_empty_liquidity_reward: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct Position {
        pub lb_pair: Pubkey,
        pub owner: Pubkey,
        pub liquidity_shares: [u64; 70],
        pub reward_infos: [UserRewardInfo; 2],
        pub fee_infos: [FeeInfo; 2],
        pub lower_bin_id: i32,
        pub upper_bin_id: i32,
        pub last_updated_at: i64,
        pub total_claimed_fee_x_amount: u64,
        pub total_claimed_fee_y_amount: u64,
        pub total_claimed_rewards: [u64; 2],
        pub operator: Pubkey,
        pub lock_release_point: u64,
        pub subjected_to_bootstrap_liquidity_locking: u8,
        pub fee_owner: Pubkey,
        pub padding: [u8; 87],
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct UserRewardInfo {
        pub reward_per_token_complete: u128,
        pub reward_pending: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct FeeInfo {
        pub fee_x_per_token_complete: u128,
        pub fee_y_per_token_complete: u128,
        pub fee_x_pending: u64,
        pub fee_y_pending: u64,
    }
}

pub mod cpi_instructions {
    use super::*;
    use meteora_dlmm_types::*;

    #[derive(Accounts)]
    pub struct ClaimFee<'info> {
        pub position: Account<'info, Position>,
        #[account(mut)]
        pub lb_pair: Account<'info, LbPair>,
        #[account(mut)]
        pub user_token_x: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        #[account(mut)]
        pub user_token_y: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        #[account(mut)]
        pub reserve_x: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        #[account(mut)]
        pub reserve_y: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        pub token_x_mint: Box<Account<'info, anchor_spl::token::Mint>>,
        pub token_y_mint: Box<Account<'info, anchor_spl::token::Mint>>,
        pub token_program: Program<'info, anchor_spl::token::Token>,
        pub owner: Signer<'info>,
    }

    pub fn claim_fee(ctx: CpiContext<ClaimFee>) -> Result<()> {
        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: DLMM_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.position.key(), false),
                AccountMeta::new(ctx.accounts.lb_pair.key(), false),
                AccountMeta::new(ctx.accounts.user_token_x.key(), false),
                AccountMeta::new(ctx.accounts.user_token_y.key(), false),
                AccountMeta::new(ctx.accounts.reserve_x.key(), false),
                AccountMeta::new(ctx.accounts.reserve_y.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_x_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_y_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.owner.key(), true),
            ],
            data: vec![0x86, 0x03, 0x22, 0x39, 0xac, 0x49, 0x61, 0x95], // claim_fee discriminator
        };

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.position.to_account_info(),
                ctx.accounts.lb_pair.to_account_info(),
                ctx.accounts.user_token_x.to_account_info(),
                ctx.accounts.user_token_y.to_account_info(),
                ctx.accounts.reserve_x.to_account_info(),
                ctx.accounts.reserve_y.to_account_info(),
                ctx.accounts.token_x_mint.to_account_info(),
                ctx.accounts.token_y_mint.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
        )?;

        Ok(())
    }

    #[derive(Accounts)]
    pub struct ClaimProtocolFee<'info> {
        #[account(mut)]
        pub lb_pair: Account<'info, LbPair>,
        #[account(mut)]
        pub reserve_x: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        #[account(mut)]
        pub reserve_y: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        #[account(mut)]
        pub fee_recipient_token_x: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        #[account(mut)]
        pub fee_recipient_token_y: Box<Account<'info, anchor_spl::token::TokenAccount>>,
        pub token_x_mint: Box<Account<'info, anchor_spl::token::Mint>>,
        pub token_y_mint: Box<Account<'info, anchor_spl::token::Mint>>,
        pub token_program: Program<'info, anchor_spl::token::Token>,
        pub fee_owner: Signer<'info>,
    }

    pub fn claim_protocol_fee(ctx: CpiContext<ClaimProtocolFee>) -> Result<()> {
        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: DLMM_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.lb_pair.key(), false),
                AccountMeta::new(ctx.accounts.reserve_x.key(), false),
                AccountMeta::new(ctx.accounts.reserve_y.key(), false),
                AccountMeta::new(ctx.accounts.fee_recipient_token_x.key(), false),
                AccountMeta::new(ctx.accounts.fee_recipient_token_y.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_x_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_y_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.fee_owner.key(), true),
            ],
            data: vec![0xee, 0x58, 0x3b, 0x31, 0xa4, 0x2e, 0x8c, 0x88], // claim_protocol_fee discriminator
        };

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.lb_pair.to_account_info(),
                ctx.accounts.reserve_x.to_account_info(),
                ctx.accounts.reserve_y.to_account_info(),
                ctx.accounts.fee_recipient_token_x.to_account_info(),
                ctx.accounts.fee_recipient_token_y.to_account_info(),
                ctx.accounts.token_x_mint.to_account_info(),
                ctx.accounts.token_y_mint.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.fee_owner.to_account_info(),
            ],
        )?;

        Ok(())
    }
}