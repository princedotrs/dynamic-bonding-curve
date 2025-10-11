use crate::constants::*;
use crate::state::Global;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetParams<'info> {
    #[account(
        seeds = [seeds::GLOBAL],
        bump = global.bump,
    )]
    pub global: Account<'info, Global>,

    #[account(
        mut,
        address = ADMIN
    )]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> SetParams<'info> {
    pub fn set_params(
        &mut self,
        admin: &Pubkey,
        fee_recipient: &Pubkey,
        fee_basis_points: u64,
        new_supply: u64,
    ) -> Result<()> {
        self.global.authority = *admin;
        self.global.fee_recipient = *fee_recipient;
        self.global.fee_basis_points = fee_basis_points;
        self.global.supply = new_supply;

        Ok(())
    }
}
