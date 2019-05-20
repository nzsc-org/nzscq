use crate::{
    boosters::Booster,
    characters::Character,
    player::{Action, ArsenalItem,FinishedPlayer},
};

#[derive(Debug, Clone)]
pub enum CharacterPhaseOutcome {
    Done(Vec<CharacterHeadstart>),
    Rechoose,
    Pending,
}

#[derive(Debug, Clone)]
pub enum BoosterPhaseOutcome {
    Done(Vec<Booster>),
    Pending,
}

#[derive(Debug, Clone)]
pub enum DraineePhaseOutcome {
    Done(Vec<Option<ArsenalItem>>),
    Pending,
}

#[derive(Debug, Clone)]
pub enum ActionPhaseOutcome {
    Done(Vec<ActionPoints>),
    GameOver(Vec<FinishedPlayer>),
    Pending,
}

#[derive(Debug, Clone)]
pub struct CharacterHeadstart(pub Character, pub u8);

#[derive(Debug, Clone)]
pub struct ActionPoints(pub Action, pub u8);
