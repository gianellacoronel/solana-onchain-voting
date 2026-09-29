use anchor_lang::prelude::*;

declare_id!("CAbqi9ipfV1mfukp67VX8KX8FSAzHwgznfxgEgT9Hy4E");

#[program]
pub mod voting {
    use super::*;

    // _poll_id -> We add underscore _ to remove warning for unused variable, because we need it there
    pub fn init_poll(ctx: Context<InitPoll>, _poll_id: u64, start: u64, end: u64, name: String, description: String) -> Result<()> {
        let mut poll = ctx.accounts.poll_account;
        poll.poll_name = name;
        poll.poll_description = description;
        poll.poll_voting_start = start;
        poll.poll_voting_end = end;

        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll{
    #[account(mut)]
    pub signer: Signer<'info>
    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializeCandidate{
    #[account(mut)]
    pub signer: Signer<'info>

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()]
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
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

//Add a struct to track votes
#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}
