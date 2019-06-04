pub(crate) mod transparent;
pub use transparent::*;

pub enum Scoreboard {
    Characterless(Vec<CharacterlessPlayer>),
    Boosterless(Vec<BoosterlessPlayer>),
    Dequeueing(Vec<DequeueingPlayer>),
    Actionless(Vec<ActionlessPlayer>),
    Final(Vec<FinishedPlayer>),
}
