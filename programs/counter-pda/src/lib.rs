use {crate::instructions::*, anchor_lang::prelude::*};

mod instructions;

declare_id!("EfsEGKqSgL57gLNRvXx3BisB4wjW21EnUKgdpELMiY2g");

#[program]
pub mod counter_pda {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        instructions::create::create_counter(ctx)
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        instructions::increment::increment_counter(ctx)
    }
}
