use crate::players::{
    ActionlessPlayer, BoosterlessPlayer, CharacterlessPlayer, DequeueChoicelessPlayer,
    FinishedPlayer,
};

#[derive(Debug, Clone)]
pub(super) enum Phase {
    Character(Vec<CharacterlessPlayer>),
    Booster(Vec<BoosterlessPlayer>),
    Dequeue(Vec<DequeueChoicelessPlayer>),
    Action(Vec<ActionlessPlayer>),
    Final(Vec<FinishedPlayer>),
}
