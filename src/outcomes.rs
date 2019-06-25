use crate::{
    choices::{Action, Booster, Character, DequeueChoice},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    CharacterPhaseDone(Vec<CharacterHeadstart>),
    CharacterPhaseRechoose(Vec<Character>),
    BoosterPhaseDone(Vec<Booster>),
    DequeuePhaseDone(Vec<DequeueChoice>),
    ActionPhaseDone(Vec<ActionPointsDestroyed>),
    GameOver(Vec<ActionPointsDestroyed>),
}

impl Outcome {
    pub fn character_phase_done(self) -> Option<Vec<CharacterHeadstart>> {
        if let Outcome::CharacterPhaseDone(ch) = self {
            Some(ch)
        } else {
            None
        }
    }

    pub fn character_phase_rechoose(self) -> Option<Vec<Character>> {
        if let Outcome::CharacterPhaseRechoose(c) = self {
            Some(c)
        } else {
            None
        }
    }

    pub fn booster_phase_done(self) -> Option<Vec<Booster>> {
        if let Outcome::BoosterPhaseDone(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn dequeue_phase_done(self) -> Option<Vec<DequeueChoice>> {
        if let Outcome::DequeuePhaseDone(d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn action_phase_done(self) -> Option<Vec<ActionPointsDestroyed>> {
        if let Outcome::ActionPhaseDone(apd) = self {
            Some(apd)
        } else {
            None
        }
    }

    pub fn game_over(self) -> Option<Vec<ActionPointsDestroyed>> {
        if let Outcome::GameOver(apd) = self {
            Some(apd)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterHeadstart(pub Character, pub u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionPointsDestroyed(pub Action, pub i8, pub bool);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_phase_done_returns_some_if_outcome_is_character_phase_done() {
        assert!(Outcome::CharacterPhaseDone(vec![])
            .character_phase_done()
            .is_some());
    }

    #[test]
    fn character_phase_done_returns_none_if_outcome_is_not_character_phase_done() {
        assert!(Outcome::CharacterPhaseRechoose(vec![])
            .character_phase_done()
            .is_none());
    }

    #[test]
    fn character_phase_rechoose_returns_some_if_outcome_is_character_phase_rechoose() {
        assert!(Outcome::CharacterPhaseRechoose(vec![])
            .character_phase_rechoose()
            .is_some());
    }

    #[test]
    fn character_phase_rechoose_returns_none_if_outcome_is_not_character_phase_rechoose() {
        assert!(Outcome::CharacterPhaseDone(vec![])
            .character_phase_rechoose()
            .is_none());
    }

    #[test]
    fn booster_phase_done_returns_some_if_outcome_is_booster_phase_done() {
        assert!(Outcome::BoosterPhaseDone(vec![])
            .booster_phase_done()
            .is_some());
    }

    #[test]
    fn booster_phase_done_returns_none_if_outcome_is_not_booster_phase_done() {
        assert!(Outcome::CharacterPhaseRechoose(vec![])
            .booster_phase_done()
            .is_none());
    }

    #[test]
    fn dequeue_phase_done_returns_some_if_outcome_is_dequeue_phase_done() {
        assert!(Outcome::DequeuePhaseDone(vec![])
            .dequeue_phase_done()
            .is_some());
    }

    #[test]
    fn dequeue_phase_done_returns_none_if_outcome_is_not_dequeue_phase_done() {
        assert!(Outcome::BoosterPhaseDone(vec![])
            .dequeue_phase_done()
            .is_none());
    }

    #[test]
    fn action_phase_done_returns_some_if_outcome_is_action_phase_done() {
        assert!(Outcome::ActionPhaseDone(vec![])
            .action_phase_done()
            .is_some());
    }

    #[test]
    fn action_phase_done_returns_none_if_outcome_is_not_action_phase_done() {
        assert!(Outcome::GameOver(vec![]).action_phase_done().is_none());
    }

    #[test]
    fn game_over_returns_some_if_outcome_is_game_over() {
        assert!(Outcome::GameOver(vec![]).game_over().is_some());
    }

    #[test]
    fn game_over_returns_none_if_outcome_is_not_game_over() {
        assert!(Outcome::ActionPhaseDone(vec![]).game_over().is_none());
    }
}
