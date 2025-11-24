use anchor_lang::prelude::*;

pub mod state;

use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWxqSWVR79zxR4Cw3Q8M3DT6sGzX");

#[program]
pub mod candle_markets {
    use super::*;

    pub fn create_market(
        ctx: Context<CreateMarket>,
        asset: String,
        open_price: u64,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        // Implementation will be added in Phase 1 Step 4
        Ok(())
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        side: BetSide,
        amount: u64,
    ) -> Result<()> {
        // Implementation will be added in Step 5
        Ok(())
    }

    pub fn settle_market(
        ctx: Context<SettleMarket>,
        close_price: u64,
    ) -> Result<()> {
        // Implementation will be added in Step 7
        Ok(())
    }

    pub fn claim_reward(
        ctx: Context<ClaimReward>,
    ) -> Result<()> {
        // Implementation will be added in Step 8
        Ok(())
    }
}

//
// -----------------------------
//       ACCOUNT CONTEXTS
// -----------------------------
//

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = MarketAccount::LEN,
        seeds = [b"market", market_id_seed().as_ref()],
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
// -----------------------------
//       ERROR HANDLING
// -----------------------------
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

//
// -----------------------------
//       PDA SEED HELPERS
// -----------------------------
//

pub fn market_id_seed() -> [u8; 8] {
    // temporary placeholder – updated during instruction implementation
    0u64.to_le_bytes()
}
