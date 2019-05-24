use super::{ArsenalItem, Move, PointsAgainst};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Mirror(Move),
    Move(Move),
    Concede,
}

impl Action {
    pub(crate) fn into_opt_arsenal_item(self) -> Option<ArsenalItem> {
        match self {
            Action::Mirror(_) => Some(ArsenalItem::Mirror),
            Action::Move(m) => Some(ArsenalItem::Move(m)),
            Action::Concede => None,
        }
    }

    fn move_(&self) -> Option<Move> {
        match self {
            Action::Mirror(m) => Some(*m),
            Action::Move(m) => Some(*m),
            Action::Concede => None,
        }
    }
}

impl PointsAgainst for Action {
    fn points_against(&self, other: &Action) -> u8 {
        let own_move = self.move_();
        let other_move = other.move_();
        match (own_move, other_move) {
            (Some(own), Some(other)) => own.points_against(&other),
            (Some(_own), None) => 1,
            (None, Some(_other)) => 0,
            (None, None) => 0,
        }
    }
}
