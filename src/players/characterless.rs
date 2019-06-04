use super::BoosterlessPlayer;
use crate::choices::{Character, Choose};
use crate::counters::{CharacterChoices, CharacterStreak};
use crate::game::Config;
use crate::outcomes::CharacterHeadstart;
use crate::scoreboard::transparent;

#[derive(Debug, Clone)]
pub struct CharacterlessPlayer {
    game_config: Config,
    streak: Option<CharacterStreak>,
}

impl CharacterlessPlayer {
    pub fn from_game_config(game_config: Config) -> Self {
        Self {
            game_config,
            streak: None,
        }
    }

    pub fn add_to_streak(&mut self, character: Character) {
        self.streak
            .choose(self.game_config.max_character_repetitions, character)
            .unwrap();
    }

    pub fn into_boosterless(
        self,
        CharacterHeadstart(character, headstart): CharacterHeadstart,
    ) -> BoosterlessPlayer {
        BoosterlessPlayer {
            game_config: self.game_config,
            points: headstart,
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

impl Into<transparent::CharacterlessPlayer> for CharacterlessPlayer {
    fn into(self) -> transparent::CharacterlessPlayer {
        transparent::CharacterlessPlayer {
            streak: self.streak.map(|streak| streak.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Config;

    #[test]
    fn from_game_works() {
        let actual = CharacterlessPlayer::from_game_config(Config::default());
        let expected = CharacterlessPlayer {
            game_config: Config::default(),
            streak: None,
        };
        assert_eq!(expected.game_config, actual.game_config);
        assert_eq!(expected.streak, actual.streak);
    }

    #[test]
    fn add_to_streak_works() {
        let mut player = CharacterlessPlayer::from_game_config(Config::default());
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
        let player = CharacterlessPlayer::from_game_config(Config::default());
        assert_eq!(Character::all(), player.choices(),);
    }

    #[test]
    fn cannot_choose_repeated_character_when_maximum_reached() {
        let mut player = CharacterlessPlayer::from_game_config(Config::default());
        for _ in 0..player.game_config.max_character_repetitions {
            player.add_to_streak(Character::Ninja);
        }

        let mut no_ninja = Character::all();
        no_ninja.retain(|c| c != &Character::Ninja);

        assert_eq!(no_ninja, player.choices());
    }

    #[test]
    fn into_boosterless_works() {
        let player = CharacterlessPlayer::from_game_config(Config::default());
        assert_eq!(
            Character::Ninja,
            player
                .into_boosterless(CharacterHeadstart(Character::Ninja, 0))
                .character
        );
    }

    #[test]
    fn into_transparent_works() {
        let original = CharacterlessPlayer::from_game_config(Config::default());
        let transparent: transparent::CharacterlessPlayer = original.clone().into();

        assert_eq!(
            original
                .streak
                .map(|streak| Into::<transparent::CharacterStreak>::into(streak)),
            transparent.streak
        );
    }
}
