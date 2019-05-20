use super::{boosterless::BoosterlessPlayer, CanChoose, Choose};
use crate::characters::{Character, CharacterChoices, CharacterStreak};
use crate::game::GameConfig;

#[derive(Debug, Clone)]
pub struct CharacterlessPlayer {
    game_config: GameConfig,
    streak: Option<CharacterStreak>,
    points: u8,
    pending_character: Option<Character>,
}

impl CharacterlessPlayer {
    pub fn from_game_config(game_config: GameConfig) -> Self {
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
            self.streak
                .choose(self.game_config.max_character_repetitions, character)
                .unwrap();
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<&Character> {
        self.pending_character.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameConfig;

    #[test]
    fn from_game_works() {
        let actual = CharacterlessPlayer::from_game_config(GameConfig::default());
        let expected = CharacterlessPlayer {
            game_config: GameConfig::default(),
            streak: None,
            points: 0,
            pending_character: None,
        };
        assert_eq!(expected.game_config, actual.game_config);
        assert_eq!(expected.streak, actual.streak);
        assert_eq!(expected.points, actual.points);
        assert_eq!(expected.pending_character, actual.pending_character);
    }

    #[test]
    fn add_points_works() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        assert_eq!(player.points, 0);
        player.add_points(3);
        assert_eq!(player.points, 3);
    }

    #[test]
    fn can_choose_any_initially() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        assert_eq!(player.choices(), Some(Character::all()));
        assert!(player.choose(Character::Ninja).is_ok());
        assert_eq!(player.choice(), Some(&Character::Ninja));
    }

    #[test]
    fn cannot_choose_repeated_character_when_maximum_reached() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        for _ in 0..player.game_config.max_character_repetitions {
            player.choose(Character::Ninja).unwrap();
            player.clear_choice().unwrap();
        }

        let mut no_ninja = Character::all();
        no_ninja.retain(|c| c != &Character::Ninja);
        assert_eq!(player.choices(), Some(no_ninja));
        assert!(player.choose(Character::Ninja).is_err());
        assert_eq!(player.choice(), None);
    }

    #[test]
    fn cannot_choose_if_has_already_chosen() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.choose(Character::Ninja).unwrap();
        assert_eq!(player.choices(), None);
        assert!(player.choose(Character::Samurai).is_err());
        assert_eq!(player.choice(), Some(&Character::Ninja));
    }

    #[test]
    fn clear_choice_works() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.choose(Character::Ninja).unwrap();
        player.clear_choice().unwrap();
        assert_eq!(player.choice(), None);
    }

    #[test]
    fn into_boosterless_works_when_player_has_chosen() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.choose(Character::Ninja).unwrap();
        assert!(player.into_boosterless().is_ok());
    }

    #[test]
    fn into_boosterless_fails_when_player_has_not_chosen() {
        let player = CharacterlessPlayer::from_game_config(GameConfig::default());
        assert!(player.into_boosterless().is_err());
    }
}
