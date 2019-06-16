pub(crate) mod transparent;
pub use transparent::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scoreboard {
    Characterless(Vec<CharacterlessPlayer>),
    Boosterless(Vec<BoosterlessPlayer>),
    Dequeueing(Vec<DequeueingPlayer>),
    Actionless(Vec<ActionlessPlayer>),
    Final(Vec<FinishedPlayer>),
}

impl Scoreboard {
    pub fn characterless(self) -> Option<Vec<CharacterlessPlayer>> {
        if let Scoreboard::Characterless(players) = self {
            Some(players)
        } else {
            None
        }
    }

    pub fn boosterless(self) -> Option<Vec<BoosterlessPlayer>> {
        if let Scoreboard::Boosterless(players) = self {
            Some(players)
        } else {
            None
        }
    }

    pub fn dequeueing(self) -> Option<Vec<DequeueingPlayer>> {
        if let Scoreboard::Dequeueing(players) = self {
            Some(players)
        } else {
            None
        }
    }

    pub fn actionless(self) -> Option<Vec<ActionlessPlayer>> {
        if let Scoreboard::Actionless(players) = self {
            Some(players)
        } else {
            None
        }
    }

    pub fn final_(self) -> Option<Vec<FinishedPlayer>> {
        if let Scoreboard::Final(players) = self {
            Some(players)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characterless_returns_some_if_variant_is_characterless() {
        let scoreboard = Scoreboard::Characterless(vec![]);
        assert!(scoreboard.characterless().is_some());
    }

    #[test]
    fn characterless_returns_none_if_variant_is_not_characterless() {
        let scoreboard = Scoreboard::Boosterless(vec![]);
        assert!(scoreboard.characterless().is_none());
    }

    #[test]
    fn boosterless_returns_some_if_variant_is_boosterless() {
        let scoreboard = Scoreboard::Boosterless(vec![]);
        assert!(scoreboard.boosterless().is_some());
    }

    #[test]
    fn boosterless_returns_none_if_variant_is_not_boosterless() {
        let scoreboard = Scoreboard::Characterless(vec![]);
        assert!(scoreboard.boosterless().is_none());
    }

    #[test]
    fn dequeueing_returns_some_if_variant_is_dequeueing() {
        let scoreboard = Scoreboard::Dequeueing(vec![]);
        assert!(scoreboard.dequeueing().is_some());
    }

    #[test]
    fn dequeueing_returns_none_if_variant_is_not_dequeueing() {
        let scoreboard = Scoreboard::Actionless(vec![]);
        assert!(scoreboard.dequeueing().is_none());
    }

    #[test]
    fn actionless_returns_some_if_variant_is_actionless() {
        let scoreboard = Scoreboard::Actionless(vec![]);
        assert!(scoreboard.actionless().is_some());
    }

    #[test]
    fn actionless_returns_none_if_variant_is_not_actionless() {
        let scoreboard = Scoreboard::Final(vec![]);
        assert!(scoreboard.actionless().is_none());
    }

    #[test]
    fn final_returns_some_if_variant_is_actionless() {
        let scoreboard = Scoreboard::Final(vec![]);
        assert!(scoreboard.final_().is_some());
    }

    #[test]
    fn final_returns_none_if_variant_is_not_actionless() {
        let scoreboard = Scoreboard::Dequeueing(vec![]);
        assert!(scoreboard.final_().is_none());
    }
}