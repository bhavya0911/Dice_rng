use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::Bet;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    player: Signer<'info>,
    ///CHECK: this is not safe
    house: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        space = Bet::LEN,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    bet: Account<'info, Bet>,
    system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(&mut self, seed: u64, bumps: &PlaceBetBumps, amount: u64, roll: u8) -> Result<()> {
        self.bet.set_inner(Bet { 
            player: self.player.key(), 
            seed, 
            amount, 
            roll, 
            slot: Clock::get()?.slot,
            bump: bumps.bet,
        });

        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let account = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), account);

        transfer(ctx, amount)

    }
}