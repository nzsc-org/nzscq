use super::helpers;
use super::moves::Move;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Booster {
    Shadow,
    Speedy,
    Regenerative,
    ZombieCorps,
    Atlas,
    Strong,
    Backwards,
    Moustachio,
    None,
}

impl Booster {
    pub fn moves(self) -> Vec<Move> {
        match self {
            Booster::Shadow => vec![Move::ShadowFireball, Move::ShadowSlip],
            Booster::Speedy => vec![Move::RunInCircles, Move::LightningFastKarateChop],
            Booster::Regenerative => vec![Move::Regenerate, Move::Gravedigger],
            Booster::ZombieCorps => vec![Move::ZombieCorps, Move::Apocalypse],
            Booster::Atlas => vec![Move::Lightning, Move::Earthquake],
            Booster::Strong => vec![Move::Twist, Move::Bend],
            Booster::Backwards => vec![Move::BackwardsMoustachio, Move::NoseOfTheTaunted],
            Booster::Moustachio => vec![Move::MustacheMash, Move::BigHairyDeal],
            Booster::None => vec![],
        }
    }

    pub fn replace_moves(self, moves: &mut Vec<Move>) {
        match self {
            Booster::Strong => {
                moves.retain(|m| m != &Move::Smash);
                moves.push(Move::StrongSmash);
            }
            _ => {}
        };
    }
}

impl FromStr for Booster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &helpers::lowercase_no_whitespace(s)[..] {
            "shadow" => Ok(Booster::Shadow),
            "speedy" => Ok(Booster::Speedy),
            "regenerative" => Ok(Booster::Regenerative),
            "zombiecorps" => Ok(Booster::ZombieCorps),
            "atlas" => Ok(Booster::Atlas),
            "strong" => Ok(Booster::Strong),
            "backwards" => Ok(Booster::Backwards),
            "moustachio" => Ok(Booster::Moustachio),
            "none" | "nobooster" => Ok(Booster::None),
            _ => Err(()),
        }
    }
}

impl Display for Booster {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            &Booster::Shadow => "Shadow",
            &Booster::Speedy => "Speedy",
            &Booster::Regenerative => "Regenerative",
            &Booster::ZombieCorps => "Zombie Corps",
            &Booster::Atlas => "Atlas",
            &Booster::Strong => "Strong",
            &Booster::Backwards => "Backwards",
            &Booster::Moustachio => "Moustachio",
            &Booster::None => "No Booster",
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        assert_eq!(Booster::Shadow.to_string(), "Shadow".to_string());
    }

    #[test]
    fn from_str_works() {
        assert_eq!(Booster::from_str("Shadow"), Ok(Booster::Shadow));
    }

    #[test]
    fn atlas_replaces_nothing() {
        use crate::characters::Character;

        let original = Character::Samurai.moves();
        let mut replaced = original.clone();
        Booster::Atlas.replace_moves(&mut replaced);
        assert_eq!(replaced, original);
    }

    #[test]
    fn strong_replaces_smash_with_strong_smash() {
        use crate::characters::Character;

        let original = Character::Samurai.moves();
        let mut replaced = original.clone();
        Booster::Strong.replace_moves(&mut replaced);
        assert_ne!(replaced, original);
        assert!(!replaced.contains(&Move::Smash));
        assert!(replaced.contains(&Move::StrongSmash));
    }
}
