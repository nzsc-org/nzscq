use crate::players::{
    ActionlessPlayer, BoosterlessPlayer, CharacterlessPlayer, DraineelessPlayer, FinishedPlayer,
};

#[derive(Debug, Clone)]
pub(super) enum Phase {
    Character(Vec<CharacterlessPlayer>),
    Booster(Vec<BoosterlessPlayer>),
    Dequeue(Vec<DraineelessPlayer>),
    Action(Vec<ActionlessPlayer>),
    Final(Vec<FinishedPlayer>),
}
