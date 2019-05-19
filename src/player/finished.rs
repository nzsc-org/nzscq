use super::ArsenalItem;
use crate::boosters::Booster;
use crate::characters::Character;
use crate::queue::Queue;
use crate::GameConfig;

#[derive(Debug, Clone)]
pub struct FinishedPlayer {
    game_config: GameConfig,
    points: u8,
    character: Character,
    booster: Booster,
    arsenal: Vec<ArsenalItem>,
    queue: Queue,
}

impl FinishedPlayer {
    pub fn game_config(&self) -> &GameConfig {
        &self.game_config
    }

    pub fn point(&self) -> u8 {
        self.points
    }

    pub fn character(&self) -> Character {
        self.character
    }

    pub fn booster(&self) -> Booster {
        self.booster
    }

    pub fn arsenal(&self) -> &Vec<ArsenalItem> {
        &self.arsenal
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
