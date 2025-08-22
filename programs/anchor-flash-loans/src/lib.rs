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
        require!(borrow_amount > 0, ProtocolError::InvalidAmount);

        let seeds = &[b"protocol".as_ref(), &[ctx.bumps.protocol]];
        let signer_seeds: &[&[&[u8]]] = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.protocol.to_account_info(),
                from: ctx.accounts.protocol_ata.to_account_info(),
                to: ctx.accounts.borrower_ata.to_account_info(),
            },
            signer_seeds,
        );

        transfer(cpi_ctx, borrow_amount)?;

        let ixs = ctx.accounts.instructions.to_account_info();

        let instruction_sysvar = ixs.try_borrow_data()?;
        let len = u16::from_be_bytes(instruction_sysvar[0..2].try_into().unwrap());

        msg!("Total ixs in this tx: {}", len);

        let mut found_repay = false;

        for i in 1..len {
            if let Ok(repay_ix) = load_instruction_at_checked(i as usize, &ixs) {
                msg!("Checking instruction at index {}", i);
                msg!("Program ID: {}", repay_ix.program_id);

                if repay_ix.program_id == ID
                    && repay_ix.data.len() >= 8
                    && repay_ix.data[0..8].eq(instruction::Repay::DISCRIMINATOR)
                {
                    msg!("Found repay instruction at index: {}", i);

                    if repay_ix.accounts.len() >= 5 {
                        let repay_borrower_ata = repay_ix
                            .accounts
                            .get(3)
                            .ok_or(ProtocolError::InvalidBorrowerAta)?;
                        require_keys_eq!(
                            repay_borrower_ata.pubkey,
                            ctx.accounts.borrower_ata.key(),
                            ProtocolError::InvalidBorrowerAta
                        );

                        let repay_protocol_ata = repay_ix
                            .accounts
                            .get(4)
                            .ok_or(ProtocolError::InvalidProtocolAta)?;
                        require_keys_eq!(
                            repay_protocol_ata.pubkey,
                            ctx.accounts.protocol_ata.key(),
                            ProtocolError::InvalidProtocolAta
                        );

                        found_repay = true;
                        break;
                    } else {
                        msg!(
                            "Repay instruction has insufficient accounts: {}",
                            repay_ix.accounts.len()
                        );
                    }
                }
            }
        }

        require!(found_repay, ProtocolError::MissingRepayIx);

        Ok(())
    }

    pub fn repay(ctx: Context<Loan>) -> Result<()> {
        let ixs = ctx.accounts.instructions.to_account_info();

        let mut amount_borrowed: u64;

        if let Ok(borrow_ix) = load_instruction_at_checked(0, &ixs) {
            let mut borrowed_data: [u8; 8] = [0u8; 8];
            borrowed_data.copy_from_slice(&borrow_ix.data[8..16]);
            amount_borrowed = u64::from_le_bytes(borrowed_data)
        } else {
            return Err(ProtocolError::MissingBorrowIx.into());
        }

        let fee = (amount_borrowed as u128)
            .checked_mul(500)
            .unwrap()
            .checked_div(10_000)
            .ok_or(ProtocolError::Overflow)? as u64;
        amount_borrowed = amount_borrowed
            .checked_add(fee)
            .ok_or(ProtocolError::Overflow)?;

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.borrower_ata.to_account_info(),
                    to: ctx.accounts.protocol_ata.to_account_info(),
                    authority: ctx.accounts.borrower.to_account_info(),
                },
            ),
            amount_borrowed,
        )?;

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
    pub system_program: Program<'info, System>,
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
