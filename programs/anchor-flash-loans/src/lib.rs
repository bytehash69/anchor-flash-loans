use anchor_lang::prelude::*;

declare_id!("9Zhy6hku43UnVsoNJxi8P4CbumXAEvXD9m2PiQXnPpu1");

#[program]
pub mod anchor_flash_loans {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
