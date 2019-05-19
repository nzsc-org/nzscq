use super::boosters::Booster;
use super::helpers;
use super::moves::Move;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Character {
    Ninja = 0,
    Zombie = 1,
    Samurai = 2,
    Clown = 3,
}

impl Character {
    pub fn all() -> Vec<Character> {
        vec![
            Character::Ninja,
            Character::Zombie,
            Character::Samurai,
            Character::Clown,
        ]
    }

    pub fn moves(self) -> Vec<Move> {
        match self {
            Character::Ninja => vec![Move::Kick, Move::NinjaSword, Move::Nunchucks],
            Character::Zombie => vec![Move::Rampage, Move::Muscle, Move::Zap],
            Character::Samurai => vec![Move::SamuraiSword, Move::Helmet, Move::Smash],
            Character::Clown => vec![Move::JugglingKnives, Move::AcidSpray, Move::Nose],
        }
    }

    pub fn boosters(self) -> Vec<Booster> {
        match self {
            Character::Ninja => vec![Booster::Shadow, Booster::Speedy, Booster::None],
            Character::Zombie => vec![Booster::Regenerative, Booster::ZombieCorps, Booster::None],
            Character::Samurai => vec![Booster::Atlas, Booster::Strong, Booster::None],
            Character::Clown => vec![Booster::Backwards, Booster::Moustachio, Booster::None],
        }
    }

    pub fn points_against(self, other: Character) -> u8 {
        let self_index = self as usize;
        let other_index = other as usize;
        CHARACTER_HEADSTARTS[other_index * 29 + self_index]
    }
}

impl FromStr for Character {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &helpers::lowercase_no_whitespace(s)[..] {
            "ninja" => Ok(Character::Ninja),
            "zombie" => Ok(Character::Zombie),
            "samurai" => Ok(Character::Samurai),
            "clown" => Ok(Character::Clown),
            _ => Err(()),
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            &Character::Ninja => "Ninja",
            &Character::Zombie => "Zombie",
            &Character::Samurai => "Samurai",
            &Character::Clown => "Clown",
        };

        write!(f, "{}", string)
    }
}

const CHARACTER_HEADSTARTS: [u8; 4 * 4] = [0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0];

pub(crate) trait CharacterChoices {
    fn choices(&self, max_times: u8) -> Vec<Character>;
    fn choose(&mut self, character: Character);
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

    fn choose(&mut self, character: Character) {
        if let Some(streak) = self {
            if streak.character == character {
                streak.times += 1;
            } else {
                streak.character = character;
                streak.times = 1;
            }
        } else {
            *self = Some(CharacterStreak {
                character,
                times: 1,
            });
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CharacterStreak {
    character: Character,
    times: u8,
}
