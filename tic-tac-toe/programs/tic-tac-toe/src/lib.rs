use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

declare_id!("EnXiwvsvV9c9p2yggdvzaBuAuGAms2smeqQeyEZB7Zmy");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}


#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    FromPrimitive,
    ToPrimitive,
    Copy,
    Clone,
    PartialEq,
    Eq
)]
pub enum Sign {
    X,
    O,
}

#[account]
pub struct Game {
    players: [Pubkey; 2],          // (32 * 2)
    turn: u8,                      // 1
    board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1) = 18
    state: GameState,              // 32 + 1
}
