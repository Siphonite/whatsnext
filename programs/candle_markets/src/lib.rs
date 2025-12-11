#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke,
    system_instruction,
};

pub mod state;
use state::*;

declare_id!("9fAJRwzjj7dBMt7fimMo6jKwwsYFD4k9eoMPD8MwBnWb");

#[program]
pub mod candle_markets {
    use super::*;

    // ---------------------------------------------------------
    //  STEP 1 — INITIALIZE TREASURY PDA
    // ---------------------------------------------------------
    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.bump = ctx.bumps.treasury;   // updated bumps API
        Ok(())
    }

    // ---------------------------------------------------------
    //  STEP 4 — CREATE MARKET
    // ---------------------------------------------------------
    pub fn create_market(
        ctx: Context<CreateMarket>,
        asset: String,
        open_price: u64,
        start_time: i64,
        end_time: i64,
        market_id: u64,
    ) -> Result<()> {
        require!(end_time > start_time, CandleError::MarketClosed);

        let lock_time = end_time - 600;
        let market = &mut ctx.accounts.market;

        market.asset = asset;
        market.market_id = market_id;
        market.start_time = start_time;
        market.end_time = end_time;
        market.lock_time = lock_time;
        market.open_price = open_price;
        market.close_price = 0;
        market.virtual_liquidity = 100;

        market.green_pool_weighted = market.virtual_liquidity;
        market.red_pool_weighted = market.virtual_liquidity;

        market.settled = false;
        Ok(())
    }

    // ---------------------------------------------------------
    // STEP 5 — PLACE BET
    // ---------------------------------------------------------
    pub fn place_bet(
        ctx: Context<PlaceBet>,
        side: BetSide,
        amount: u64,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let user_bet = &mut ctx.accounts.user_bet;
        let user = &ctx.accounts.user;
        let treasury = &ctx.accounts.treasury;

        let now = Clock::get()?.unix_timestamp;
        require!(now < market.lock_time, CandleError::MarketLocked);
        require!(!user_bet.claimed, CandleError::Unauthorized);

        const MAX_BET: u64 = 50_000_000;
        require!(amount <= MAX_BET, CandleError::InvalidBetSize);

        // Transfer SOL into Treasury PDA
        let ix = system_instruction::transfer(&user.key(), &treasury.key(), amount);
        invoke(
            &ix,
            &[
                user.to_account_info(),
                treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let elapsed = now - market.start_time;
        let weight: u64 = if elapsed < 3600 {
            100
        } else if elapsed < 7200 {
            70
        } else if elapsed < 10800 {
            50
        } else {
            20
        };

        let effective_stake = amount
            .checked_mul(weight).unwrap()
            .checked_div(100).unwrap();

        match side {
            BetSide::Green => {
                market.green_pool_weighted =
                    market.green_pool_weighted.checked_add(effective_stake).unwrap()
            }
            BetSide::Red => {
                market.red_pool_weighted =
                    market.red_pool_weighted.checked_add(effective_stake).unwrap()
            }
        }

        user_bet.user = user.key();
        user_bet.market = market.key();
        user_bet.side = side;
        user_bet.amount = amount;
        user_bet.weight = weight;
        user_bet.effective_stake = effective_stake;
        user_bet.claimed = false;

        Ok(())
    }

    // ---------------------------------------------------------
    // STEP 7 — SETTLE MARKET
    // ---------------------------------------------------------
    pub fn settle_market(
        ctx: Context<SettleMarket>,
        close_price: u64,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;

        let now = Clock::get()?.unix_timestamp;
        require!(now >= market.end_time, CandleError::MarketNotEnded);
        require!(!market.settled, CandleError::Unauthorized);

        market.close_price = close_price;
        market.settled = true;

        Ok(())
    }

    // ---------------------------------------------------------
    // STEP 8 — CLAIM REWARD
    // ---------------------------------------------------------
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let user_bet = &mut ctx.accounts.user_bet;
        let treasury = &ctx.accounts.treasury;
        let user = &ctx.accounts.user;

        require!(market.settled, CandleError::SettlementPending);
        require!(!user_bet.claimed, CandleError::AlreadyClaimed);

        let winning_side = if market.close_price > market.open_price {
            BetSide::Green
        } else if market.close_price < market.open_price {
            BetSide::Red
        } else {
            user_bet.claimed = true;
            return Ok(());
        };

        if user_bet.side != winning_side {
            user_bet.claimed = true;
            return Ok(());
        }

        let virtual_liq = market.virtual_liquidity;

        let (winning_pool, losing_pool) = match winning_side {
            BetSide::Green => (market.green_pool_weighted, market.red_pool_weighted),
            BetSide::Red => (market.red_pool_weighted, market.green_pool_weighted),
        };

        let total_winning_weighted = winning_pool.saturating_sub(virtual_liq);
        let total_losing_weighted = losing_pool.saturating_sub(virtual_liq);

        if total_winning_weighted == 0 || total_losing_weighted == 0 {
            user_bet.claimed = true;
            return Ok(());
        }

        let payout = (user_bet.effective_stake as u128)
            .checked_mul(total_losing_weighted as u128).unwrap()
            .checked_div(total_winning_weighted as u128).unwrap() as u64;

        if payout > 0 {
            let treasury_lamports = **treasury.to_account_info().lamports.borrow();
            require!(treasury_lamports >= payout, CandleError::InsufficientFunds);

            **treasury.to_account_info().try_borrow_mut_lamports()? -= payout;
            **user.to_account_info().try_borrow_mut_lamports()? += payout;
        }

        user_bet.claimed = true;
        Ok(())
    }
}

// -------------------------------------------------------------
//  ACCOUNT CONTEXTS
// -------------------------------------------------------------
#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        payer = authority,
        space = TreasuryAccount::LEN,
        seeds = [b"treasury".as_ref()],
        bump
    )]
    pub treasury: Account<'info, TreasuryAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(asset: String, open_price: u64, start_time: i64, end_time: i64, market_id: u64)]
pub struct CreateMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = MarketAccount::LEN,
        seeds = [b"market".as_ref(), &market_id.to_le_bytes()],
        bump
    )]
    pub market: Account<'info, MarketAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketAccount>,

    #[account(
        init,
        payer = user,
        space = UserBetAccount::LEN,
        seeds = [
            b"bet".as_ref(),
            user.key().as_ref(),
            market.key().as_ref()
        ],
        bump
    )]
    pub user_bet: Account<'info, UserBetAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"treasury".as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, TreasuryAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleMarket<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketAccount>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketAccount>,

    #[account(mut)]
    pub user_bet: Account<'info, UserBetAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"treasury".as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, TreasuryAccount>,
}

// -------------------------------------------------------------
// ERRORS
// -------------------------------------------------------------
#[error_code]
pub enum CandleError {
    #[msg("Betting is locked for this market")]
    MarketLocked,
    #[msg("Market has already ended")]
    MarketClosed,
    #[msg("Reward already claimed")]
    AlreadyClaimed,
    #[msg("Market not yet settled")]
    SettlementPending,
    #[msg("Invalid market reference")]
    WrongMarket,
    #[msg("Invalid weight tier")]
    InvalidWeight,
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Market has not ended yet")]
    MarketNotEnded,
    #[msg("Insufficient funds in treasury for payout")]
    InsufficientFunds,
    #[msg("Bet exceeds the maximum allowed size")]
    InvalidBetSize,
}
