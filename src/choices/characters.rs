use super::{Booster, Move};
use crate::helpers;

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
        CHARACTER_HEADSTARTS[other_index * 4 + self_index]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        assert_eq!(Character::Ninja.to_string(), "Ninja".to_string());
    }

    #[test]
    fn from_str_works() {
        assert_eq!(Character::from_str("Ninja"), Ok(Character::Ninja));
    }

    #[test]
    fn ninja_beats_samurai() {
        assert_eq!(Character::Ninja.points_against(Character::Samurai), 1);
        assert_eq!(Character::Samurai.points_against(Character::Ninja), 0);
    }

    #[test]
    fn ninja_ties_zombie() {
        assert_eq!(Character::Ninja.points_against(Character::Zombie), 0);
        assert_eq!(Character::Zombie.points_against(Character::Ninja), 0);
    }

}
