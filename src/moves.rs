use super::helpers;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Move {
    Kick = 0,
    NinjaSword = 1,
    Nunchucks = 2,
    ShadowFireball = 3,
    ShadowSlip = 4,
    RunInCircles = 5,
    LightningFastKarateChop = 6,
    Rampage = 7,
    Muscle = 8,
    Zap = 9,
    Regenerate = 10,
    Gravedigger = 11,
    ZombieCorps = 12,
    Apocalypse = 13,
    SamuraiSword = 14,
    Helmet = 15,
    Smash = 16,
    StrongSmash = 17,
    Lightning = 18,
    Earthquake = 19,
    Twist = 20,
    Bend = 21,
    JugglingKnives = 22,
    AcidSpray = 23,
    Nose = 24,
    BackwardsMoustachio = 25,
    NoseOfTheTaunted = 26,
    MustacheMash = 27,
    BigHairyDeal = 28,
}

impl Move {
    pub fn points_against(self, other: Move) -> u8 {
        let self_index = self as usize;
        let other_index = other as usize;
        MOVE_OUTCOMES[other_index * 29 + self_index]
    }

    pub fn is_destructive(self) -> bool {
        DESTRUCTIVE_MOVES.contains(&self)
    }

    pub fn is_single_use(self) -> bool {
        SINGLE_USE_MOVES.contains(&self)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            Move::Kick => "Kick",
            Move::NinjaSword => "Ninja Sword",
            Move::Nunchucks => "Nunchucks",
            Move::ShadowFireball => "Shadow Fireball",
            Move::ShadowSlip => "Shadow Slip",
            Move::RunInCircles => "Run In Circles",
            Move::LightningFastKarateChop => "Lightning Fast Karate Chop",
            Move::Rampage => "Rampage",
            Move::Muscle => "Muscle",
            Move::Zap => "Zap",
            Move::Regenerate => "Regenerate",
            Move::Gravedigger => "Gravedigger",
            Move::ZombieCorps => "Zombie Corps",
            Move::Apocalypse => "Apocalypse",
            Move::SamuraiSword => "Samurai Sword",
            Move::Helmet => "Helmet",
            Move::Smash => "Smash",
            Move::StrongSmash => "Strong Smash",
            Move::Lightning => "Lightning",
            Move::Earthquake => "Earthquake",
            Move::Twist => "Twist",
            Move::Bend => "Bend",
            Move::JugglingKnives => "Juggling Knives",
            Move::AcidSpray => "Acid Spray",
            Move::Nose => "Nose",
            Move::BackwardsMoustachio => "Backwards Moustachio",
            Move::NoseOfTheTaunted => "Nose Of The Taunted",
            Move::MustacheMash => "Mustache Mash",
            Move::BigHairyDeal => "Big Hairy Deal",
        };
        write!(f, "{}", string)
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &helpers::lowercase_no_whitespace(s)[..] {
            "kick" => Ok(Move::Kick),
            "ninjasword" => Ok(Move::NinjaSword),
            "nunchucks" => Ok(Move::Nunchucks),
            "shadowfireball" => Ok(Move::ShadowFireball),
            "shadowslip" => Ok(Move::ShadowSlip),
            "runincircles" => Ok(Move::RunInCircles),
            "lightningfastkaratechop" => Ok(Move::LightningFastKarateChop),
            "rampage" => Ok(Move::Rampage),
            "muscle" => Ok(Move::Muscle),
            "zap" => Ok(Move::Zap),
            "regenerate" => Ok(Move::Regenerate),
            "gravedigger" => Ok(Move::Gravedigger),
            "zombiecorps" => Ok(Move::ZombieCorps),
            "apocalypse" => Ok(Move::Apocalypse),
            "samuraisword" => Ok(Move::SamuraiSword),
            "helmet" => Ok(Move::Helmet),
            "smash" => Ok(Move::Smash),
            "strongsmash" => Ok(Move::StrongSmash),
            "lightning" => Ok(Move::Lightning),
            "earthquake" => Ok(Move::Earthquake),
            "twist" => Ok(Move::Twist),
            "bend" => Ok(Move::Bend),
            "jugglingknives" => Ok(Move::JugglingKnives),
            "acidspray" => Ok(Move::AcidSpray),
            "nose" => Ok(Move::Nose),
            "backwardsmoustachio" => Ok(Move::BackwardsMoustachio),
            "noseofthetaunted" => Ok(Move::NoseOfTheTaunted),
            "mustachemash" => Ok(Move::MustacheMash),
            "bighairydeal" => Ok(Move::BigHairyDeal),
            _ => Err(()),
        }
    }
}

const MOVE_OUTCOMES: [u8; 29 * 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0,
    1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0,
    1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0,
    1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
    1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1,
    1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1,
    1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1,
    0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1,
    0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const SINGLE_USE_MOVES: [Move; 3] = [Move::Zap, Move::Regenerate, Move::AcidSpray];

const DESTRUCTIVE_MOVES: [Move; 2] = [Move::Zap, Move::AcidSpray];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        assert_eq!(Move::Kick.to_string(), "Kick".to_string());
    }

    #[test]
    fn from_str_works() {
        assert_eq!(Move::from_str("kick"), Ok(Move::Kick));
        assert_eq!(Move::from_str("strongsmash"), Ok(Move::StrongSmash));
    }

    #[test]
    fn kick_loses_to_smash() {
        assert_eq!(Move::Kick.points_against(Move::Smash), 0);
        assert_eq!(Move::Smash.points_against(Move::Kick), 1);
    }

    #[test]
    fn shadow_fireball_beats_smash() {
        assert_eq!(Move::ShadowFireball.points_against(Move::Smash), 1);
        assert_eq!(Move::Smash.points_against(Move::ShadowFireball), 0);
    }

    #[test]
    fn kick_ties_mustach_mash() {
        assert_eq!(Move::Kick.points_against(Move::MustacheMash), 0);
        assert_eq!(Move::MustacheMash.points_against(Move::Kick), 0);
    }

    #[test]
    fn nunchucks_specially_ties_samurai_sword() {
        assert_eq!(Move::Nunchucks.points_against(Move::SamuraiSword), 1);
        assert_eq!(Move::SamuraiSword.points_against(Move::Nunchucks), 1);
    }

    #[test]
    fn regenerate_is_single_use() {
        assert!(Move::Regenerate.is_single_use());
    }

    #[test]
    fn zombie_corps_is_not_single_use() {
        assert!(!Move::ZombieCorps.is_single_use());
    }

    #[test]
    fn acid_spray_is_destructive() {
        assert!(Move::AcidSpray.is_destructive());
    }

    #[test]
    fn nose_is_not_destructive() {
        assert!(!Move::Nose.is_destructive());
    }
}
