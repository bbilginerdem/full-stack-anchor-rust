use anchor_lang::prelude::*;

declare_id!("4sCWdhrGVh3ui8SemxRHefweevLtxSUF6F58x53QJko8");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
