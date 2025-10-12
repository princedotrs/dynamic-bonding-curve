use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::{compute_s_out, error::DBCErrorCode, seeds, BondingCurve, Global};

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [seeds::GLOBAL],
        bump = global.bump,
        constraint = matches!(global.paused, false) @ DBCErrorCode::ContractPaused,
    )]
    pub global: Account<'info, Global>,

    #[account(
        mut,
        constraint = fee_recipient.key() == global.fee_recipient.key(),
    )]
    pub fee_recipient: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [seeds::BONDING_CURVE, bonding_curve.mint.key().as_ref()],
        bump = bonding_curve.bump,
        has_one = mint,
        constraint = matches!(bonding_curve.complete, false) @ DBCErrorCode::BondingCurveComplete,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    #[account(
        mut,
        seeds = [seeds::CURVE_VAULT, bonding_curve.mint.key().as_ref()], // PDA seeds for the vault.
        bump = bonding_curve.vault_bump, // Bump seed for the vault PDA.
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub bonding_curve_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
        token::token_program = token_program
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Sell<'info> {
    pub fn sell_tokens(&mut self, amount: u64, min_sol_output: u64) -> Result<()> {
        require!(amount > 0 && amount <= u64::MAX, DBCErrorCode::Underflow);

        require!(min_sol_output > 0, DBCErrorCode::Underflow);

        let t_current = self.bonding_curve.real_token_reserve;
        let t_new: u64 = t_current
            .checked_add(amount)
            .ok_or(DBCErrorCode::Overflow)?;

        if t_new > self.global.supply {
            return Err(DBCErrorCode::Overflow)?;
        }

        let s_out: u64 = compute_s_out(
            self.bonding_curve.virtual_token_reserve,
            self.bonding_curve.virtual_sol_reserve,
            amount,
        )?;

        require!(
            s_out <= self.bonding_curve.real_sol_reserve,
            DBCErrorCode::InsufficientSol
        );

        let fee_amount = s_out
            .checked_mul(self.global.fee_basis_points)
            .ok_or(DBCErrorCode::Overflow)?
            .checked_div(10000_u64)
            .ok_or(DBCErrorCode::Overflow)?;

        if s_out
            .checked_sub(fee_amount)
            .ok_or(DBCErrorCode::Underflow)?
            < min_sol_output
        {
            return Err(DBCErrorCode::TooLittleSolReceived.into());
        }

        self.send_token(amount)?;

        self.send_sol(s_out, fee_amount)?;

        self.update_bonding_curve(s_out, amount)?;

        Ok(())
    }

    pub fn send_sol(&mut self, sol_amount: u64, platform_fee: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            seeds::CURVE_VAULT,
            &self.mint.to_account_info().key.as_ref(),
            &[self.bonding_curve.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(
            ctx,
            sol_amount
                .checked_sub(platform_fee)
                .ok_or(DBCErrorCode::Underflow)?,
        )?;

        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();

        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.vault.to_account_info(),
            to: self.fee_recipient.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(ctx, platform_fee)
    }

    pub fn send_token(&mut self, token_amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = TransferChecked {
            from: self.user_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.bonding_curve_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program, cpi_account);

        transfer_checked(ctx, token_amount, 6)
    }

    pub fn update_bonding_curve(&mut self, s_out: u64, amount: u64) -> Result<()> {
        // update the v reserves

        self.bonding_curve.virtual_token_reserve = self
            .bonding_curve
            .virtual_token_reserve
            .checked_add(amount)
            .ok_or(DBCErrorCode::Underflow)?;
        self.bonding_curve.virtual_sol_reserve = self
            .bonding_curve
            .virtual_sol_reserve
            .checked_sub(s_out)
            .ok_or(DBCErrorCode::Underflow)?;

        // update the real reserves

        self.bonding_curve.real_token_reserve = self
            .bonding_curve
            .real_token_reserve
            .checked_add(amount)
            .ok_or(DBCErrorCode::Underflow)?;
        self.bonding_curve.real_sol_reserve = self
            .bonding_curve
            .real_sol_reserve
            .checked_sub(s_out)
            .ok_or(DBCErrorCode::Overflow)?;

        Ok(())
    }
}
