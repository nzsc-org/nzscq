use super::{ArsenalItem, Move, PointsAgainst};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Mirror(Move),
    Move(Move),
    Concede,
}

impl Action {
    pub(crate) fn which_destroyed(actions: &[Action]) -> Vec<bool> {
        let was_destructive_used = actions.iter().any(|a| a.is_destructive());

        actions
            .iter()
            .map(|a| a.is_single_use() || was_destructive_used)
            .collect()
    }

    fn is_destructive(self) -> bool {
        let opt_move: Option<Move> = self.into();

        opt_move.map(|m| m.is_destructive()).unwrap_or(false)
    }

    fn is_single_use(self) -> bool {
        let opt_move: Option<Move> = self.into();

        opt_move.map(|m| m.is_single_use()).unwrap_or(false)
    }
}

impl PointsAgainst for Action {
    fn points_against(&self, other: &Action) -> u8 {
        let own_move: Option<Move> = (*self).into();
        let other_move: Option<Move> = (*other).into();
        match (own_move, other_move) {
            (Some(own), Some(other)) => own.points_against(&other),
            (Some(_own), None) => 1,
            (None, Some(_other)) => 0,
            (None, None) => 0,
        }
    }
}

impl Into<Option<ArsenalItem>> for Action {
    fn into(self) -> Option<ArsenalItem> {
        match self {
            Action::Mirror(_) => Some(ArsenalItem::Mirror),
            Action::Move(m) => Some(ArsenalItem::Move(m)),
            Action::Concede => None,
        }
    }
}

impl Into<Option<Move>> for Action {
    fn into(self) -> Option<Move> {
        match self {
            Action::Mirror(m) => Some(m),
            Action::Move(m) => Some(m),
            Action::Concede => None,
        }
    }
}
