pub mod outcomes;

use crate::player::{
    ActionlessPlayer, BoosterlessPlayer, CharacterlessPlayer, Choose, DraineelessPlayer,
    FinishedPlayer,
};

#[derive(Debug, Clone)]
pub enum Phase {
    Character(Vec<CharacterlessPlayer>),
    Booster(Vec<BoosterlessPlayer>),
    DrainedMove(Vec<DraineelessPlayer>),
    Action(Vec<ActionlessPlayer>),
    Final(Vec<FinishedPlayer>),
}

pub(crate) trait PhaseComplete<C> {
    fn complete(&self) -> bool;
}

impl<P, C> PhaseComplete<C> for Vec<P>
where
    P: Choose<C>,
{
    fn complete(&self) -> bool {
        self.iter().all(|p| p.has_chosen())
    }
}
