use crate::{choices::Character, scoreboard::transparent};

pub(crate) trait CharacterChoices {
    fn choices(&self, max_times: u8) -> Vec<Character>;
    fn choose(&mut self, max_times: u8, character: Character) -> Result<(), ()>;
}

impl CharacterChoices for Option<CharacterStreak> {
    fn choices(&self, max_times: u8) -> Vec<Character> {
        let mut characters = Character::all();

        if let Some(streak) = &self {
            if streak.times >= max_times {
                characters.retain(|c| c != &streak.character)
            }

            characters
        } else {
            characters
        }
    }

    fn choose(&mut self, max_times: u8, character: Character) -> Result<(), ()> {
        match self {
            None => {
                *self = Some(CharacterStreak {
                    character,
                    times: 1,
                });
                Ok(())
            }
            Some(streak) => {
                if streak.character == character {
                    if streak.times < max_times {
                        streak.times += 1;
                        Ok(())
                    } else {
                        Err(())
                    }
                } else {
                    streak.character = character;
                    streak.times = 1;
                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CharacterStreak {
    character: Character,
    times: u8,
}

impl Into<transparent::CharacterStreak> for CharacterStreak {
    fn into(self) -> transparent::CharacterStreak {
        transparent::CharacterStreak {
            character: self.character,
            times: self.times,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_choose_any_if_no_streak() {
        let mut streak: Option<CharacterStreak> = None;
        assert_eq!(streak.choices(MAX_TIMES), Character::all());
        assert!(streak.choose(MAX_TIMES, Character::Ninja).is_ok());
    }

    #[test]
    fn can_choose_any_if_repetitions_less_than_max() {
        let one_less_than_max = MAX_TIMES - 1;
        let mut streak = Some(CharacterStreak {
            character: Character::Ninja,
            times: one_less_than_max,
        });
        assert_eq!(streak.choices(MAX_TIMES), Character::all());
        assert!(streak.choose(MAX_TIMES, Character::Ninja).is_ok());
    }

    #[test]
    fn cannot_choose_repeated_character_if_repetitions_equals_max() {
        let one_less_than_max = MAX_TIMES;
        let mut streak = Some(CharacterStreak {
            character: Character::Ninja,
            times: one_less_than_max,
        });
        let mut no_ninja = Character::all();
        no_ninja.retain(|c| c != &Character::Ninja);
        assert_eq!(streak.choices(MAX_TIMES), no_ninja);
        assert!(streak.choose(MAX_TIMES, Character::Ninja).is_err());
    }

    #[test]
    fn into_transparent_works() {
        let original = CharacterStreak {
            times: 0,
            character: Character::Ninja,
        };
        let transparent: transparent::CharacterStreak = original.clone().into();
        assert_eq!(original.times, transparent.times);
        assert_eq!(original.character, transparent.character);
    }

    const MAX_TIMES: u8 = 3;
}
