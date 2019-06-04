use crate::choices::{ArsenalItem, Booster, Character};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterlessPlayer {
    pub streak: Option<CharacterStreak>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoosterlessPlayer {
    pub points: u8,
    pub character: Character,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DequeueingPlayer {
    pub points: u8,
    pub character: Character,
    pub booster: Booster,
    pub arsenal: Vec<ArsenalItem>,
    pub queue: Queue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionlessPlayer {
    pub points: u8,
    pub character: Character,
    pub booster: Booster,
    pub arsenal: Vec<ArsenalItem>,
    pub queue: Queue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishedPlayer {
    pub points: u8,
    pub character: Character,
    pub booster: Booster,
    pub arsenal: Vec<ArsenalItem>,
    pub queue: Queue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterStreak {
    pub character: Character,
    pub times: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue {
    pub entrance: Option<ArsenalItem>,
    pub pool: Vec<ArsenalItem>,
    pub exit: Option<ArsenalItem>,
}
