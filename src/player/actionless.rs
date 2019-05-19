use super::{Action, ArsenalItem, CanChoose, Choose, DraineelessPlayer};
use crate::boosters::Booster;
use crate::characters::Character;
use crate::game::GameConfig;
use crate::queue::Queue;

#[derive(Debug, Clone)]
pub struct ActionlessPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
    pub(super) pending_action: Option<Action>,
}

impl ActionlessPlayer {
    pub(crate) fn add_points(&mut self, points: u8) {
        self.points += points;
    }

    pub fn into_draineeless(mut self) -> Result<DraineelessPlayer, ()> {
        if let Some(action) = self.pending_action {
            let arsenal_item = action.into_arsenal_item();
            self.arsenal.retain(|m| m != &arsenal_item);
            self.queue.enqueue(arsenal_item);

            Ok(DraineelessPlayer {
                game_config: self.game_config,
                points: self.points,
                character: self.character,
                booster: self.booster,
                arsenal: self.arsenal,
                queue: self.queue,
                choice: None,
            })
        } else {
            Err(())
        }
    }
}

impl Choose<Action> for ActionlessPlayer {
    fn choices(&self) -> Option<Vec<Action>> {
        if self.has_chosen() {
            None
        } else {
            let has_mirror = self.arsenal.contains(&ArsenalItem::Mirror);
            let mut actions: Vec<Action> = vec![];
            for item in &self.arsenal {
                match item {
                    ArsenalItem::Mirror => {}
                    ArsenalItem::Move(m) => {
                        actions.push(Action::Move(*m));
                        if has_mirror {
                            actions.push(Action::Mirror(*m));
                        }
                    }
                }
            }

            Some(actions)
        }
    }

    fn choose(&mut self, action: Action) -> Result<(), ()> {
        if self.can_choose(&action) {
            self.pending_action = Some(action);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<&Action> {
        self.pending_action.as_ref()
    }
}
