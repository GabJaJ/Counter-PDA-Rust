use {crate::collections::counter::Counter, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(
        init,
        seeds = {
            Counter::COUNTER_SEED.as_ref(),
            authority.key().as_ref()
        }, // initialize seeds - PDA
        bump,  //calculate the canonic bump
        payer = authority,
        space = 8 + Counter::INIT_SPACE,
    )]
    counter: Account<'info, Counter>,

    // authority account
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

// create counter function
pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
    ctx.accounts.counter.authority = ctx.accounts.authority.key();
    ctx.accounts.counter.count = 0;
    ctx.accounts.counter.bump = ctx.bumps.counter;

    Ok(())
}
