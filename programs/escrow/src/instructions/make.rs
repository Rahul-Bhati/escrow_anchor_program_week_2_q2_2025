use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,  // who sign to make escrow and mut bcs transfer fund

    // make sure mint_a is real token that is created by token program
    #[account(
        Mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info, Mint>,

    // Mint -> It’s used when creating a new token or verifying the token type.
    #[account(
        Mint::token_program = token_program
    )]
    pub mint_b : InterfaceAccount<'info, Mint>,

    // ATA for mint_a : TokenAccount is not a type it is a struct that reprsent user's ATA 
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a : InterfaceAccount<'info, TokenAccount>,

    // escrow acc
    #[account(
        init,
        payer= maker,
        seeds = [b"escrow", maker.key().as_ref() , seed.to_le_bytes().as_ref],
        space = 8+Escrow::INIT_SPACE,
        bump,
    )]
    pub escrow : Account<'info, Escrow>,

    // vault
    #[account(
        init,
        payer=maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,


    // AssociatedToken - This is not a struct or account — it’s a program!
    //     So in #[account(associated_token::...)], you’re saying:
    // “Use the AssociatedToken program to create/check the ATA based on mint + authority.”
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> Make<'info>{
    pub fn init_escrow(&mut self, seed: u64, recieve: u64, bumps: &MakeBumps) -> Result<()> {
        // ctx and tranfer
        // 1. init Maker -> escrow account
        self.escrow.set_inner(Escrow {
            maker: self.maker.key(),
            make_a: self.mint_a.key(),
            make_b: self.mint_b.key(),
            recieve,
            seed,
            bump: bumps.escrow
        });

        ok()
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // need tranfer fund -> and we are using here transfer_checked because we dealing with tokenprogram not native token that require cpi_context, amount and decimals 
        let cpi_program = self.token_program.to_account_info();

        let transfer_program = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info()
        };

        let cpi_context = CpiContext::new(cpi_program, transfer_program);

        transfer_checked(cpi_context, amount, self.mint_a.decimals)
    }
}