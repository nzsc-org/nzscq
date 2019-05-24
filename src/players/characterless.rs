use super::BoosterlessPlayer;
use crate::choices::{Character, Choose};
use crate::counters::{CharacterChoices, CharacterStreak};
use crate::game::GameConfig;

#[derive(Debug, Clone)]
pub struct CharacterlessPlayer {
    game_config: GameConfig,
    streak: Option<CharacterStreak>,
    points: u8,
}

impl CharacterlessPlayer {
    pub fn from_game_config(game_config: GameConfig) -> Self {
        Self {
            game_config,
            streak: None,
            points: 0,
        }
    }

    pub fn add_points(&mut self, points: u8) {
        self.points += points;
    }

    pub fn add_to_streak(&mut self, character: Character) {
        self.streak
            .choose(self.game_config.max_character_repetitions, character)
            .unwrap();
    }

    pub fn into_boosterless(self, character: Character) -> BoosterlessPlayer {
        BoosterlessPlayer {
            game_config: self.game_config,
            points: self.points,
            character,
        }
    }
}

impl Choose<Character> for CharacterlessPlayer {
    fn choices(&self) -> Vec<Character> {
        self.streak
            .choices(self.game_config.max_character_repetitions)
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
        };
        assert_eq!(expected.game_config, actual.game_config);
        assert_eq!(expected.streak, actual.streak);
        assert_eq!(expected.points, actual.points);
    }

    #[test]
    fn add_points_works() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        assert_eq!(0, player.points);
        player.add_points(3);
        assert_eq!(3, player.points);
    }

    #[test]
    fn add_to_streak_works() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.add_to_streak(Character::Ninja);
        let mut expected: Option<CharacterStreak> = None;
        expected
            .choose(
                player.game_config.max_character_repetitions,
                Character::Ninja,
            )
            .unwrap();
        assert_eq!(expected, player.streak);
    }

    #[test]
    fn can_choose_any_initially() {
        let player = CharacterlessPlayer::from_game_config(GameConfig::default());
        assert_eq!(Character::all(), player.choices(),);
    }

    #[test]
    fn cannot_choose_repeated_character_when_maximum_reached() {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        for _ in 0..player.game_config.max_character_repetitions {
            player.add_to_streak(Character::Ninja);
        }

        let mut no_ninja = Character::all();
        no_ninja.retain(|c| c != &Character::Ninja);

        assert_eq!(no_ninja, player.choices());
    }

    #[test]
    fn into_boosterless_works() {
        let player = CharacterlessPlayer::from_game_config(GameConfig::default());
        assert_eq!(
            Character::Ninja,
            player.into_boosterless(Character::Ninja).character
        );
    }
}
