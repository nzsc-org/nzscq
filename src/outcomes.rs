use crate::{
    choices::{Action, Booster, Character, DequeueChoice},
    players::FinishedPlayer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    CharacterPhaseDone(Vec<CharacterHeadstart>),
    CharacterPhaseRechoose(Vec<Character>),
    BoosterPhaseDone(Vec<Booster>),
    DequeuePhaseDone(Vec<DequeueChoice>),
    ActionPhaseDone(Vec<ActionPoints>),
    GameOver(Vec<FinishedPlayer>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterHeadstart(pub Character, pub u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionPoints(pub Action, pub u8);
