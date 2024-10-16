use anchor_lang::prelude::*;

declare_id!("GtZMCrJPXNu1UcDdvLK7Z1bYKJcJ8w7HWeWwxjGV3Xk4");

const OWNER: &str = "8os8PKYmeVjU1mmwHZZNTEv5hpBXi5VvEKGzykduZAik";

#[program]
pub mod voting {
    use super::*;

    pub fn init_candidate(ctx: Context<InitializeCandidate>) -> Result<()> {
        Ok(())
    }

    pub fn vote_for_candidate(ctx: Context<VoteCandidate>, _candidate_name: String) -> Result<()> {
        ctx.accounts.candidate.votes_received += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_candidate_Name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Candidate::INIT_SPACE,
        payer = payer,
        seeds = [_candidate_Name.as_bytes().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

#[instruction(_candidate_Name: String)]
pub struct VoteCandidate<'info> {
    #[account(
        mut,
        seeds = [_candidate_Name.as_bytes().as_ref()],
        bump,
)]
    pub candidate: Account<'info, Candidate>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
