use super::{draineeless::DraineelessPlayer, ArsenalItem, CanChoose, Choose};
use crate::boosters::Booster;
use crate::characters::Character;
use crate::queue::Queue;
use crate::GameConfig;

#[derive(Debug, Clone)]
pub struct BoosterlessPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) pending_booster: Option<Booster>,
}

impl BoosterlessPlayer {
    pub fn into_exiterless(self) -> Result<DraineelessPlayer, ()> {
        if let Some(booster) = self.pending_booster {
            let arsenal = self.initial_arsenal(booster);

            Ok(DraineelessPlayer {
                game_config: self.game_config,
                points: self.points,
                character: self.character,
                booster,
                arsenal,
                queue: Queue::new(),
                choice: None,
            })
        } else {
            Err(())
        }
    }

    fn initial_arsenal(&self, booster: Booster) -> Vec<ArsenalItem> {
        let mut moves = self.character.moves();
        moves.append(&mut booster.moves());
        booster.replace_moves(&mut moves);
        let arsenal: Vec<ArsenalItem> = moves.into_iter().map(|m| ArsenalItem::Move(m)).collect();

        arsenal
    }
}

impl Choose<Booster> for BoosterlessPlayer {
    fn choices(&self) -> Option<Vec<Booster>> {
        if self.has_chosen() {
            None
        } else {
            Some(self.character.boosters())
        }
    }

    fn choose(&mut self, booster: Booster) -> Result<(), ()> {
        if self.can_choose(&booster) {
            self.pending_booster = Some(booster);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<&Booster> {
        self.pending_booster.as_ref()
    }
}
