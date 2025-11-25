use anchor_lang::prelude::*;

pub mod state;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWxqSWVR79zxR4Cw3Q8M3DT6sGzX");

#[program]
pub mod candle_markets {
    use super::*;

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
        // ───── Validations ─────────────────────────────────────
        require!(end_time > start_time, CandleError::MarketClosed);

        // Lock market 10 mins before closing
        let lock_time = end_time - 600; // 600 seconds = 10 minutes

        let market = &mut ctx.accounts.market;

        // ───── Set Market State ────────────────────────────────
        market.asset = asset;
        market.market_id = market_id;
        market.start_time = start_time;
        market.end_time = end_time;
        market.lock_time = lock_time;
        market.open_price = open_price;
        market.close_price = 0;

        // Virtual liquidity (constant for now)
        market.virtual_liquidity = 100;

        // Bootstrap pools with virtual liquidity
        market.green_pool_weighted = market.virtual_liquidity;
        market.red_pool_weighted = market.virtual_liquidity;

        market.settled = false;

        Ok(())
    }

    // ---------------------------------------------------------
    //  STEP 5 — PLACE BET (IMPLEMENTATION DONE)
    // ---------------------------------------------------------
pub fn place_bet(
    ctx: Context<PlaceBet>,
    side: BetSide,
    amount: u64,
) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let user_bet = &mut ctx.accounts.user_bet;
    let user = &ctx.accounts.user;

    // -----------------------------------------------------
    // 1. Validate time (cannot bet after lock_time)
    // -----------------------------------------------------
    let now = Clock::get()?.unix_timestamp;
    require!(now < market.lock_time, CandleError::MarketLocked);

    // -----------------------------------------------------
    // 2. Prevent double-betting
    // (PDA is created on first bet; if it exists, Anchor would fail)
    // -----------------------------------------------------
    require!(!user_bet.claimed, CandleError::Unauthorized);

    // -----------------------------------------------------
    // 3. Determine weight tier based on elapsed time
    // -----------------------------------------------------
    let elapsed = now - market.start_time; // seconds
    let weight: u64 = if elapsed < 3600 {
        100   // 1.0x
    } else if elapsed < 7200 {
        70    // 0.7x
    } else if elapsed < 10800 {
        50    // 0.5x
    } else {
        20    // 0.2x
    };

    // -----------------------------------------------------
    // 4. Calculate effective stake
    // -----------------------------------------------------
    let effective_stake: u64 = amount * weight / 100;

    // -----------------------------------------------------
    // 5. Update pools based on side
    // -----------------------------------------------------
    match side {
        BetSide::Green => {
            market.green_pool_weighted += effective_stake;
        }
        BetSide::Red => {
            market.red_pool_weighted += effective_stake;
        }
    }

    // -----------------------------------------------------
    // 6. Save user bet state
    // -----------------------------------------------------
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
    //  STEP 7 — SETTLE MARKET (IMPLEMENTATION DONE)
    // ---------------------------------------------------------

pub fn settle_market(
    ctx: Context<SettleMarket>,
    close_price: u64,
) -> Result<()> {
    let market = &mut ctx.accounts.market;

    // 1) Ensure the market end time has passed
    let now = Clock::get()?.unix_timestamp;
    require!(now >= market.end_time, CandleError::MarketNotEnded);

    // 2) Prevent double settlement
    require!(!market.settled, CandleError::Unauthorized);

    // 3) Set close price and mark as settled
    market.close_price = close_price;
    market.settled = true;

    Ok(())
}


    // ---------------------------------------------------------
    //  STEP 8 — CLAIM REWARD (IMPLEMENT LATER)
    // ---------------------------------------------------------

pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let user_bet = &mut ctx.accounts.user_bet;
    let user = &ctx.accounts.user;

    // -------------------------------------------------------------
    // 1. Must be settled
    // -------------------------------------------------------------
    require!(market.settled, CandleError::SettlementPending);

    // -------------------------------------------------------------
    // 2. User must not have already claimed
    // -------------------------------------------------------------
    require!(!user_bet.claimed, CandleError::AlreadyClaimed);

    // -------------------------------------------------------------
    // 3. Determine market result
    // -------------------------------------------------------------
    let winning_side = if market.close_price > market.open_price {
        BetSide::Green
    } else if market.close_price < market.open_price {
        BetSide::Red
    } else {
        // flat candle → nobody wins (rare but possible)
        // user gets nothing
        user_bet.claimed = true;
        return Ok(());
    };

    // -------------------------------------------------------------
    // 4. If user bet on losing side → reward = 0
    // -------------------------------------------------------------
    if user_bet.side != winning_side {
        user_bet.claimed = true;
        return Ok(());
    }

    // -------------------------------------------------------------
    // 5. Compute total weighted stakes
    // Exclude virtual liquidity
    // -------------------------------------------------------------
    let virtual_liq = market.virtual_liquidity;

    let (winning_pool, losing_pool) = match winning_side {
        BetSide::Green => (
            market.green_pool_weighted,
            market.red_pool_weighted,
        ),
        BetSide::Red => (
            market.red_pool_weighted,
            market.green_pool_weighted,
        ),
    };

    let total_winning_weighted = winning_pool - virtual_liq;
    let total_losing_weighted = losing_pool - virtual_liq;

    // If nobody bet on either side (possible edge case)
    if total_winning_weighted == 0 || total_losing_weighted == 0 {
        user_bet.claimed = true;
        return Ok(()); // nothing to pay out
    }

    // -------------------------------------------------------------
    // 6. Compute user's payout
    // payout = (effective_stake / total_winning_weighted) * total_losing_weighted
    // -------------------------------------------------------------
    let payout = (user_bet.effective_stake as u128)
        .checked_mul(total_losing_weighted as u128)
        .unwrap()
        .checked_div(total_winning_weighted as u128)
        .unwrap() as u64;

    // -------------------------------------------------------------
    // 7. Transfer SOL to user
    // -------------------------------------------------------------
    if payout > 0 {
        **market.to_account_info().try_borrow_mut_lamports()? -= payout;
        **user.to_account_info().try_borrow_mut_lamports()? += payout;
    }

    // -------------------------------------------------------------
    // 8. Mark claimed
    // -------------------------------------------------------------
    user_bet.claimed = true;

    Ok(())
 }

}

//
// ───────────────────────────────────────────────────────────────
//  ACCOUNT CONTEXTS
// ───────────────────────────────────────────────────────────────
//

#[derive(Accounts)]
#[instruction(asset: String, open_price: u64, start_time: i64, end_time: i64, market_id: u64)]
pub struct CreateMarket<'info> {

    #[account(
        init,
        payer = authority,
        space = MarketAccount::LEN,
        seeds = [
            b"market".as_ref(), 
            &market_id.to_le_bytes()
        ],
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
}


//
// ───────────────────────────────────────────────────────────────
//  ERRORS
// ───────────────────────────────────────────────────────────────
//

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
    MarketNotEnded, // <--- NEW: used by settle_market
}
