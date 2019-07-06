use crate::{
    players::{
        ActionlessPlayer, BoosterlessPlayer, CharacterlessPlayer, DequeueChoicelessPlayer,
        FinishedPlayer,
    },
    scoreboard::Scoreboard,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum Phase {
    Character(Vec<CharacterlessPlayer>),
    Booster(Vec<BoosterlessPlayer>),
    Dequeue(Vec<DequeueChoicelessPlayer>),
    Action(Vec<ActionlessPlayer>),
    Final(Vec<FinishedPlayer>),
}

impl Into<Scoreboard> for Phase {
    fn into(self) -> Scoreboard {
        match self {
            Phase::Character(players) => {
                Scoreboard::Characterless(players.into_iter().map(|p| p.into()).collect())
            }
            Phase::Booster(players) => {
                Scoreboard::Boosterless(players.into_iter().map(|p| p.into()).collect())
            }
            Phase::Dequeue(players) => {
                Scoreboard::Dequeueing(players.into_iter().map(|p| p.into()).collect())
            }
            Phase::Action(players) => {
                Scoreboard::Actionless(players.into_iter().map(|p| p.into()).collect())
            }
            Phase::Final(players) => {
                Scoreboard::Final(players.into_iter().map(|p| p.into()).collect())
            }
        }
    }
}
