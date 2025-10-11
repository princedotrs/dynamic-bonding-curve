pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("BnHQoRBC1kedRoiFfrXvxzRDUPM64R7wxaPT2PsDndKw");

#[program]
pub mod dynamic_bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_global(&ctx.bumps)
    }

    pub fn set_params(
        ctx: Context<SetParams>,
        admin: Pubkey,
        fee_recipient: Pubkey,
        fee_basis_points: u64,
        new_supply: u64,
    ) -> Result<()> {
        ctx.accounts
            .set_params(&admin, &fee_recipient, fee_basis_points, new_supply)
    }

    // user instructions

    pub fn create(
        ctx: Context<Create>,
        token_args: TokenArgs,
        dbc_curve_args: DbcCurveArgs,
    ) -> Result<()> {
        ctx.accounts.create_token(
            token_args,
            dbc_curve_args,
            ctx.bumps.bonding_curve,
            ctx.bumps.vault,
        )
    }

    pub fn buy(ctx: Context<Buy>, amount: u64, max_sol_cost: u64) -> Result<()> {
        ctx.accounts.buy_tokens(amount, max_sol_cost)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, min_sol_output: u64) -> Result<()> {
        ctx.accounts.sell_tokens(amount, min_sol_output)
    }
}
