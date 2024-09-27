use anchor_lang::prelude::*;

mod constants;
mod state;
mod contexts;
use contexts::*;
mod error;

declare_id!("3MGBHoEssL2b7FPSMJwsroXMNfeYemQC3mNkzePekFi6");

#[program]
pub mod week4_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount :u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u64, amount :u64, roll: u8) -> Result<()> {
        ctx.accounts.create_bet(seed, &ctx.bumps, amount, roll)?;
        ctx.accounts.deposit(amount)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&ctx.bumps, &sig)
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)
    }
}
