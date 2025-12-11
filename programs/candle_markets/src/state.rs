use anchor_lang::prelude::*;

// ====================================
// MARKET ACCOUNT
// ====================================

#[account]
pub struct MarketAccount {
    pub asset: String,
    pub market_id: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub lock_time: i64,
    pub open_price: u64,
    pub close_price: u64,
    pub green_pool_weighted: u64,
    pub red_pool_weighted: u64,
    pub virtual_liquidity: u64,
    pub settled: bool,
}

impl MarketAccount {
    pub const LEN: usize = 8
        + 4 + 32
        + 8
        + 8 + 8 + 8
        + 8 + 8
        + 8 + 8
        + 8
        + 1;
}

// ====================================
// USER BET ACCOUNT
// ====================================

#[account]
pub struct UserBetAccount {
    pub user: Pubkey,
    pub market: Pubkey,
    pub side: BetSide,
    pub amount: u64,
    pub weight: u64,
    pub effective_stake: u64,
    pub claimed: bool,
}

impl UserBetAccount {
    pub const LEN: usize = 8
        + 32 + 32
        + 1
        + 8 + 8 + 8
        + 1;
}

// ====================================
// TREASURY ACCOUNT
// ====================================

#[account]
pub struct TreasuryAccount {
    pub bump: u8,
}

impl TreasuryAccount {
    pub const LEN: usize = 8 + 1;
}

// ====================================
// ENUM
// ====================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum BetSide {
    Green,
    Red,
}
