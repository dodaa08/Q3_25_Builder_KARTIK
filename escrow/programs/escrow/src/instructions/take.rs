use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        transfer_checked,
        Mint, 
        TokenAccount,
        TokenInterface,
        TransferChecked,
        close_account,
        CloseAccount
    }
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {

    #[account(mut)]
    pub maker : SystemAccount<'info>,

    #[account(mut)]
    pub taker : Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b : InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker, 
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b : InterfaceAccount<'info, TokenAccount>,
    
    #[account(  
        mut,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        close = maker
    )]
    pub escrow : Account<'info, Escrow>,

    #[account(  
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>

}

impl<'info> Take<'info>{
    
    pub fn withdraw_and_close_vault(&mut self)->Result<()> {
        
        // First: Taker deposits their tokens to maker
        let transfer_accounts_deposit = TransferChecked{
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info()
        };

        let cpi_ctx_deposit = CpiContext::new(
            self.token_program.to_account_info(), 
            transfer_accounts_deposit
        );

        transfer_checked(
            cpi_ctx_deposit,
            self.escrow.receive,
            self.mint_b.decimals
        )?;

        // Second: Transfer vault tokens to taker
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.escrow.maker.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump] 
        ]];

        let transfer_accounts_withdraw = TransferChecked{
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info() 
        };
        
        let cpi_ctx_withdraw = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts_withdraw,
            &signer_seeds
        );  
        
        transfer_checked(
            cpi_ctx_withdraw,
            self.escrow.receive,
            self.mint_a.decimals
        )?;

        // Third: Close the vault account
        let close_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            &signer_seeds
        );

        close_account(close_ctx)?;

        Ok(())
    }
}



