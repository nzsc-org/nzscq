use super::ArsenalItem;
use crate::boosters::Booster;
use crate::characters::Character;
use crate::game::GameConfig;
use crate::queue::Queue;

#[derive(Debug, Clone)]
pub struct FinishedPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
}

impl FinishedPlayer {
    pub fn game_config(&self) -> &GameConfig {
        &self.game_config
    }

    pub fn points(&self) -> u8 {
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
