use anchor_lang::prelude::*;

#[acccount]
#[derive(initSpace)] //anchor macro - calculates the space

pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
    pub bump: u8, //calculate a PDA
}

// implementation - extra function
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub const COUNTER_SEED: &'static str = "counter";
}
