use crate::state::{BondingCurve, Global};
use crate::{constants::*, error::DBCErrorCode, get_init_param};
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::{
            instructions::{
                CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts,
                CreateMetadataAccountV3InstructionArgs,
            },
            types::DataV2,
        },
        Metadata,
    },
    token::{self, Mint, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DbcCurveArgs {
    total_fund_raising: u64,
    total_sell: u64,
    total_locked_amount: u64,
    migrate_fee: u64,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 6,
        mint::authority = bonding_curve,
        mint::token_program = token_program,
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        space = 8 + BondingCurve::INIT_SPACE,
        seeds = [seeds::BONDING_CURVE, mint.key().as_ref()],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    #[account(
        seeds = [seeds::CURVE_VAULT, mint.key().as_ref()], // PDA seeds for the vault.
        bump, // Bump seed for the vault PDA.
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve
    )]
    pub bonding_curve_ata: Account<'info, TokenAccount>, // this will stroe the mint tokens

    #[account(
        seeds = [seeds::GLOBAL],
        bump=global.bump,
        constraint = matches!(global.paused, false) @ DBCErrorCode::ContractPaused,
    )]
    pub global: Account<'info, Global>,

    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>, // pub metadata: AccountInfo<'info>, if this anything got wrong

    pub mpl_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    pub fn create_token(
        &mut self,
        token_args: TokenArgs,
        dbc_curve_args: DbcCurveArgs,
        bump: u8,
        vault_bump: u8,
    ) -> Result<()> {
        // Create metadata

        require!(
            dbc_curve_args.total_sell <= SUPPLY
                && dbc_curve_args
                    .total_sell
                    .checked_add(dbc_curve_args.total_locked_amount)
                    .ok_or(DBCErrorCode::Overflow)?
                    > self
                        .global
                        .supply
                        .checked_mul(80)
                        .ok_or(DBCErrorCode::Overflow)?
                        .checked_div(100)
                        .ok_or(DBCErrorCode::Underflow)?,
            DBCErrorCode::InvalidInput
        );

        let seeds = &[
            seeds::BONDING_CURVE,
            &self.mint.to_account_info().key.as_ref(),
            &[bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let metadata = &self.metadata.to_account_info();
        let mint = &self.mint.to_account_info();
        let mint_authority = &self.bonding_curve.to_account_info();
        let payer = &self.payer.to_account_info();
        let update_authority = &self.bonding_curve.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let token_metadata_program = &self.mpl_metadata_program.to_account_info();

        CreateMetadataAccountV3Cpi::new(
            token_metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority,
                update_authority: (update_authority, true),
                payer,
                system_program,
                rent: Some(&self.rent.to_account_info()),
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: token_args.name,
                    symbol: token_args.symbol,
                    uri: token_args.uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
        )
        .invoke_signed(signer_seeds)?;

        token::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::MintTo {
                    authority: self.bonding_curve.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.bonding_curve_ata.to_account_info(),
                },
                signer_seeds, // check if it need same signer
            ),
            self.global.supply,
        )?;

        // Revoke mint authority
        token::set_authority(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::SetAuthority {
                    account_or_mint: self.mint.to_account_info(),
                    current_authority: self.bonding_curve.to_account_info(),
                },
                signer_seeds,
            ),
            token::spl_token::instruction::AuthorityType::MintTokens,
            None,
        )?;

        let (x0, y0, z) = get_init_param(
            self.global.supply,
            dbc_curve_args
                .total_fund_raising
                .checked_div(LAMPORTS_PER_SOL)
                .ok_or(DBCErrorCode::Underflow)?,
            dbc_curve_args.total_sell,
            dbc_curve_args.total_locked_amount,
            dbc_curve_args.migrate_fee,
        )?;

        self.bonding_curve.set_inner(BondingCurve {
            mint: self.mint.key(),
            completion_lamports: dbc_curve_args.total_fund_raising,
            virtual_token_reserve: x0,
            virtual_sol_reserve: y0,
            real_token_reserve: z,
            real_sol_reserve: 0,
            token_total_supply: self.global.supply,
            complete: false,
            initializer: self.payer.key(),
            bump,
            vault_bump,
            _padding: [0; 7],
        });

        Ok(())
    }
}
