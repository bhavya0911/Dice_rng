use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{state::Bet, error::DiceError};

#[derive(Accounts)]
pub struct RefundBet<'info> {
    #[account(mut)]
    player: Signer<'info>,
    house: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump,
    )]
    bet: Account<'info, Bet>,
    system_program: Program<'info, System>,
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(&mut self, bumps: &RefundBetBumps) -> Result<()> {
        let slot = Clock::get()?.slot;
        require!((slot - self.bet.slot) > 324, DiceError::CannotRefund);

        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let seeds = [
            b"vault",
            &self.house.key().to_bytes()[..],
            &[bumps.vault]
        ];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

        transfer(ctx, self.bet.amount)
    }
}