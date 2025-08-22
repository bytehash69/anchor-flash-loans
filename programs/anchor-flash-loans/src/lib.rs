use anchor_lang::prelude::*;
use anchor_lang::{
    solana_program::sysvar::instructions::{
        load_instruction_at_checked, ID as INSTRUCTIONS_SYSVAR_ID,
    },
    Discriminator,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

declare_id!("9Zhy6hku43UnVsoNJxi8P4CbumXAEvXD9m2PiQXnPpu1");

#[program]
pub mod anchor_flash_loans {
    use super::*;

    pub fn borrow(ctx: Context<Loan>, borrow_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn repay(ctx: Context<Loan>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Loan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        seeds = [b"protocol".as_ref()],
        bump
    )]
    pub protocol: SystemAccount<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed, 
        payer = borrower,
        associated_token::mint = mint,
        associated_token::authority = borrower
    )]
    pub borrower_ata: Account<'info, TokenAccount>,
    pub protocol_ata: Account<'info, TokenAccount>,

    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    /// CHECK: InstructionsSysvar account
    instructions: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

#[error_code]
pub enum ProtocolError {
    #[msg("Invalid instruction")]
    InvalidIx,
    #[msg("Invalid instruction index")]
    InvalidInstructionIndex,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Not enough funds")]
    NotEnoughFunds,
    #[msg("Program Mismatch")]
    ProgramMismatch,
    #[msg("Invalid program")]
    InvalidProgram,
    #[msg("Invalid borrower ATA")]
    InvalidBorrowerAta,
    #[msg("Invalid protocol ATA")]
    InvalidProtocolAta,
    #[msg("Missing repay instruction")]
    MissingRepayIx,
    #[msg("Missing borrow instruction")]
    MissingBorrowIx,
    #[msg("Overflow")]
    Overflow,
}
