use super::{boosterless::BoosterlessPlayer, CanChoose, Choose};
use crate::characters::{Character, CharacterChoices, CharacterStreak};
use crate::GameConfig;

#[derive(Debug, Clone)]
pub struct CharacterlessPlayer {
    game_config: GameConfig,
    streak: Option<CharacterStreak>,
    points: u8,
    pending_character: Option<Character>,
}

impl CharacterlessPlayer {
    pub fn from_game(game_config: GameConfig) -> Self {
        Self {
            game_config,
            streak: None,
            points: 0,
            pending_character: None,
        }
    }

    pub(crate) fn add_points(&mut self, points: u8) {
        self.points += points;
    }

    pub(crate) fn clear_choice(&mut self) -> Result<(), ()> {
        if let Some(_) = self.pending_character {
            self.pending_character = None;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn into_boosterless(self) -> Result<BoosterlessPlayer, ()> {
        if let Some(character) = self.pending_character {
            Ok(BoosterlessPlayer {
                game_config: self.game_config,
                points: self.points,
                character,
                pending_booster: None,
            })
        } else {
            Err(())
        }
    }
}

impl Choose<Character> for CharacterlessPlayer {
    fn choices(&self) -> Option<Vec<Character>> {
        if self.has_chosen() {
            None
        } else {
            Some(
                self.streak
                    .choices(self.game_config.max_character_repetitions),
            )
        }
    }

    fn choose(&mut self, character: Character) -> Result<(), ()> {
        if self.can_choose(&character) {
            self.pending_character = Some(character);
            self.streak.choose(character);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<&Character> {
        self.pending_character.as_ref()
    }
}
