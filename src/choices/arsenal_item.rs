use super::{Action, Move};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArsenalItem {
    Mirror,
    Move(Move),
}

impl ArsenalItem {
    pub(crate) fn as_move_action(self) -> Option<Action> {
        if let ArsenalItem::Move(m) = self {
            Some(Action::Move(m))
        } else {
            None
        }
    }

    pub(crate) fn as_mirror_action(self) -> Option<Action> {
        if let ArsenalItem::Move(m) = self {
            Some(Action::Mirror(m))
        } else {
            None
        }
    }
}
