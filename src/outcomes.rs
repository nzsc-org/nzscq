use crate::{
    choices::{Action, Booster, Character, DequeueChoice},
    player::FinishedPlayer,
};

#[derive(Debug, Clone, PartialEq)]
pub enum CharacterPhaseOutcome {
    Done(Vec<CharacterHeadstart>),
    Rechoose(Vec<Character>),
    Pending,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoosterPhaseOutcome {
    Done(Vec<Booster>),
    Pending,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DraineePhaseOutcome {
    Done(Vec<DequeueChoice>),
    Pending,
}

#[derive(Debug, Clone)]
pub enum ActionPhaseOutcome {
    Done(Vec<ActionPoints>),
    GameOver(Vec<FinishedPlayer>),
    Pending,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharacterHeadstart(pub Character, pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct ActionPoints(pub Action, pub u8);
