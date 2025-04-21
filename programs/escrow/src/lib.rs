use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub use instructions::*;
pub use state::*;

declare_id!("F4Tk7LWyXtWRN76bryGjEmqdSmR599mtpGonWgng7bZp");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, recieve: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, recieve, &ctx.bumps);
        ctx.accounts.deposit(deposit)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        todo!()
    }

    pub fn take(ctx: Context<Take>, amount: u64) -> Result<()> {
        todo!();
    }
}
