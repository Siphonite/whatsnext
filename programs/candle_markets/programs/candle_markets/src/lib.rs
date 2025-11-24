use anchor_lang::prelude::*;

declare_id!("Hg7BMe7yteVPB8E5pwdjsAVQhfc9gFHy5tbbbZixZaXU");

#[program]
pub mod candle_markets {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
