use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount, close_account},
};
use crate::Escrow;

#[derive(Accounts)]
pub struct Refund <'info>{
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        Mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // we do not need mint_b because we are refund to a and closing the acc

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
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref() , escrow.seed.to_le_bytes().as_ref],
        space = 8+Escrow::INIT_SPACE,
        bump = escrow.bump,
    )]
    pub escrow : Account<'info, Escrow>,

    // vault
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> Refund<'info> {
    pub fn refund_and_close_account(&self) -> Result<()> {
        // refund krne ke lise transfer the fund from vaulat to maker token_a ata -> maker_ata_a
        // for this we have to get signer_seeds to sign transaction and bcs of vault is own by escrow so we use escrow seeds to sign

        let signer_seeds: [&[&[u8]] ; 1] = [
            b"escrow",
            self.maker.key().to_account_info().as_ref(),
            // self.seeds.to_le_bytes().as_ref()
            &self.escrow.seed.to_le_bytes()[..], 
            &[self.escrow.bump],
        ];

        let token_accounts = TransferChecked {
            from : self.vault.to_account_info(),
            mint : self.mint_a.to_account_info(),
            to : self.maker_ata_a.to_account_info(),
            authority : self.escrow.to_account_info(),
        };


        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, &signer_seeds);

        transfer_checked(tranfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?; // why que mark bcs we want to program to execute further not return from here

        // now we can close the account
        let close_acc = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let close_cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_acc, &signer_seeds);

        close_account(close_cpi_ctx)
    }
}