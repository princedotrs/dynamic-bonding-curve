use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Global {
    pub initialized: bool,
    pub paused: bool,
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub fee_basis_points: u64,
    pub bump: u8,
}
