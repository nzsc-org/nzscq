use crate::{
    choices::{Action, Booster, Character, DequeueChoice},
    players::FinishedPlayer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterPhaseOutcome {
    Done(Vec<CharacterHeadstart>),
    Rechoose(Vec<Character>),
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoosterPhaseOutcome {
    Done(Vec<Booster>),
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraineePhaseOutcome {
    Done(Vec<DequeueChoice>),
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionPhaseOutcome {
    Done(Vec<ActionPoints>),
    GameOver(Vec<FinishedPlayer>),
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterHeadstart(pub Character, pub u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionPoints(pub Action, pub u8);
