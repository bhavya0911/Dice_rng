use anchor_lang::prelude::*;

#[error_code]
pub enum DiceError {
    #[msg("Bump error")]
    BumpError,
    #[msg("Overflow")]
    Overflow,
    #[msg("Not enough time passed")]
    CannotRefund,
    #[msg("Minimum bet is 0.01 sol")]
    MinimumBet,
    #[msg("Maximum bet exceeded")]
    MaximumBet,
    #[msg("Minimum rool is 2")]
    MinimumRoll,
    #[msg("Maximum rool is 96")]
    MaximumRoll,
    #[msg("Timeout not yet reached")]
    TimeoutNotReached,
    #[msg("Ed25519 Header Error")]
    Ed25519Header,
    #[msg("Ed25519 Pubkey Error")]
    Ed25519Pubkey,
    #[msg("Ed25519 Message Error")]
    Ed25519Message,
    #[msg("Ed25519 Signature Error")]
    Ed25519Signature,
    #[msg("Ed25519 Program Error")]
    Ed25519Program,
    #[msg("Ed25519 Accounts Error")]
    Ed25519Accounts,
    #[msg("Ed25519 Data Length Error")]
    Ed25519DataLength,
}