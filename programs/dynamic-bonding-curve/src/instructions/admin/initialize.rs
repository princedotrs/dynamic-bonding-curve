use crate::constants::*;
use crate::state::Global;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + Global::INIT_SPACE,
        seeds = [seeds::GLOBAL],
        bump,
    )]
    pub global: Account<'info, Global>,

    #[account(
        mut,
        address = ADMIN
    )]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_global(&mut self, bump: &InitializeBumps) -> Result<()> {
        self.global.set_inner(Global {
            initialized: true,
            paused: false,
            authority: self.admin.key(),
            fee_recipient: self.admin.key(),
            fee_basis_points: 100,
            supply: SUPPLY,
            bump: bump.global,
        });

        Ok(())
    }
}
