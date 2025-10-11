use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct BondingCurve {
    pub mint: Pubkey,
    pub completion_lamports: u64,
    pub virtual_token_reserve: u64,
    pub virtual_sol_reserve: u64,
    pub real_token_reserve: u64,
    pub real_sol_reserve: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub initializer: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    pub _padding: [u8; 7],
}
