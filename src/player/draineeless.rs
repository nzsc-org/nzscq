use super::{actionless::ActionlessPlayer, ArsenalItem, CanChoose, Choose};
use crate::boosters::Booster;
use crate::characters::Character;
use crate::game::GameConfig;
use crate::queue::Queue;

#[derive(Debug, Clone)]
pub struct DraineelessPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
    pub(super) choice: Option<Option<ArsenalItem>>,
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

impl Choose<Option<ArsenalItem>> for DraineelessPlayer {
    fn choices(&self) -> Option<Vec<Option<ArsenalItem>>> {
        if self.has_chosen() {
            None
        } else if self.can_dequeue() {
            let mut items: Vec<Option<ArsenalItem>> = self
                .queue
                .pool()
                .iter()
                .map(|item| Some(item.clone()))
                .collect();
            items.push(None);
            Some(items)
        } else {
            Some(vec![None])
        }
    }

    fn choose(&mut self, drainee: Option<ArsenalItem>) -> Result<(), ()> {
        if self.can_choose(&drainee) {
            let exited = self.queue.dequeue(drainee.as_ref()).unwrap();
            if let Some(exited) = exited {
                self.arsenal.push(exited);
            }
            self.choice = Some(drainee);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<&Option<ArsenalItem>> {
        self.choice.as_ref()
    }
}
