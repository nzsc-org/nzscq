use super::{actionless::ActionlessPlayer, ArsenalItem, CanChooseRef, ChooseRef};
use crate::boosters::Booster;
use crate::characters::Character;
use crate::queue::Queue;
use crate::GameConfig;

#[derive(Debug, Clone)]
pub struct DraineelessPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
    pub(super) choice: Option<ArsenalItem>,
}

impl DraineelessPlayer {
    pub fn into_actionless(self) -> Result<ActionlessPlayer, ()> {
        if self.has_chosen() {
            Ok(ActionlessPlayer {
                game_config: self.game_config,
                points: self.points,
                character: self.character,
                booster: self.booster,
                arsenal: self.arsenal,
                queue: self.queue,
                pending_action: None,
            })
        } else {
            Err(())
        }
    }

    fn can_dequeue(&self) -> bool {
        self.arsenal.len() < self.game_config.max_arsenal_items as usize || self.queue.exit_vacant()
    }
}

impl ChooseRef<ArsenalItem> for DraineelessPlayer {
    fn choices(&self) -> Option<&Vec<ArsenalItem>> {
        if self.has_chosen() || self.queue.pool().is_empty() || !self.can_dequeue() {
            None
        } else {
            Some(self.queue.pool())
        }
    }

    fn choose(&mut self, drainee: ArsenalItem) -> Result<(), ()> {
        if self.can_choose(&drainee) {
            let exited = self.queue.dequeue(&drainee);
            if let Some(exited) = exited {
                self.arsenal.push(exited);
            }
            self.choice = Some(drainee);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<&ArsenalItem> {
        self.choice.as_ref()
    }
}
