use anchor_lang::prelude::*;

declare_id!("CAbqi9ipfV1mfukp67VX8KX8FSAzHwgznfxgEgT9Hy4E");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }
}

// Solana is stateless, we save accounts with its own data
// Add a struct to represent a poll account
#[account]
#[derive(InitSpace)] //To calculate the size of the account for me
pub struct PollAccount{
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64, // date represented as Unix timestamp
    pub poll_voting_end: u64,
    pub poll_option_index: u64
}
