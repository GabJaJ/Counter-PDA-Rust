use {crate::collections::counter::Counter, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    authority: Signer<'info>, //verifies the account
    #[account(
        mut,
        seeds= [
            Counter::COUNTER_SEED.as_ref(),
            authority.key().as_ref(),
        ],
        bump = counter.bump, 
        constraint = counter.authority = authority.key(), // verifies the same authority in counter and instruction
    )] // counter PDA
    counter: Account<'info, Counter>,
}


// increment counter function
pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
    //ctx.accounts.counter.count += 1;
    ctx.accounts.counter.increment();
    Ok(())
}
