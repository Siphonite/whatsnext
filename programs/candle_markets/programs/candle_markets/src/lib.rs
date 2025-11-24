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
    //  STEP 5 — PLACE BET (IMPLEMENT NEXT)
    // ---------------------------------------------------------
    pub fn place_bet(
        ctx: Context<PlaceBet>,
        side: BetSide,
        amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    // ---------------------------------------------------------
    //  STEP 7 — SETTLE MARKET (IMPLEMENT LATER)
    // ---------------------------------------------------------
    pub fn settle_market(
        ctx: Context<SettleMarket>,
        close_price: u64,
    ) -> Result<()> {
        Ok(())
    }

    // ---------------------------------------------------------
    //  STEP 8 — CLAIM REWARD (IMPLEMENT LATER)
    // ---------------------------------------------------------
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        Ok(())
    }
}

//
// ───────────────────────────────────────────────────────────────
//  ACCOUNT CONTEXTS
// ───────────────────────────────────────────────────────────────
//

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = MarketAccount::LEN,
        seeds = [b"market", market_id.to_le_bytes().as_ref()],
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
        seeds = [b"bet", user.key().as_ref(), market.key().as_ref()],
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
}
