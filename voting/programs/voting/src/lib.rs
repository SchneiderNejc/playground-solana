use anchor_lang::prelude::*;

declare_id!("GtZMCrJPXNu1UcDdvLK7Z1bYKJcJ8w7HWeWwxjGV3Xk4");

#[program]

pub mod voting {
    use super::*;

    pub fn init_candidate(ctx: Context<InitializeCandidate>) -> Result<()> {
        Ok(())
    }

    pub fn vote_for_candidate(ctx: Context<VoteCandidate>) -> Result<()> {
        Ok(())
    }

}



#[derive(Accounts)]

pub struct InitializeCandidate{}



#[derive(Accounts)]

pub struct VoteCandidate{}