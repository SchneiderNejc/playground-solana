use anchor_lang::prelude::*;

declare_id!("GtZMCrJPXNu1UcDdvLK7Z1bYKJcJ8w7HWeWwxjGV3Xk4");

const OWNER: &str = "8os8PKYmeVjU1mmwHZZNTEv5hpBXi5VvEKGzykduZAik";

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
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct VoteCandidate {}

fn check(ctx: &Context<InitializeCandidate>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.payer.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );
    Ok(())
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
