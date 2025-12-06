use anchor_lang::prelude::*;

#[account]
pub struct MarketAccount {
    pub asset: String,               // e.g. "BTC/USDT"
    pub market_id: u64,              // unique rolling ID
    pub start_time: i64,             // UNIX timestamp
    pub end_time: i64,               // UNIX timestamp
    pub lock_time: i64,              // betting cutoff
    pub open_price: u64,             // oracle price at start
    pub close_price: u64,            // oracle price at end
    pub green_pool_weighted: u64,    // weighted GREEN pool
    pub red_pool_weighted: u64,      // weighted RED pool
    pub virtual_liquidity: u64,      // default: 100
    pub settled: bool,               // whether settlement happened
}

impl MarketAccount {
    pub const LEN: usize = 8   // discriminator
        + 4 + 32               // asset string (max 32 bytes)
        + 8                     // market_id
        + 8 + 8 + 8             // start_time, end_time, lock_time
        + 8 + 8                 // open_price, close_price
        + 8 + 8                 // green/red pools
        + 8                     // virtual liquidity
        + 1;                    // settled flag
}

#[account]
pub struct UserBetAccount {
    pub user: Pubkey,            // bettor
    pub market: Pubkey,          // the market they bet on
    pub side: BetSide,           // GREEN/RED
    pub amount: u64,             // raw stake
    pub weight: u64,             // 100, 70, 50, 20 (multiplied by 100)
    pub effective_stake: u64,    // amount * weight / 100
    pub claimed: bool,           // reward claimed or not
}

impl UserBetAccount {
    pub const LEN: usize = 8   // discriminator
        + 32 + 32             // user, market
        + 1                   // side enum
        + 8 + 8 + 8           // amount, weight, effective stake
        + 1;                  // claimed
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum BetSide {
    Green,
    Red,
}

