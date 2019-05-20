use crate::moves::Move;

mod characterless;
pub use characterless::*;
mod boosterless;
pub use boosterless::*;
mod draineeless;
pub use draineeless::*;
mod actionless;
pub use actionless::*;
mod finished;
pub use finished::*;

pub(crate) trait Choose<T> {
    fn choices(&self) -> Option<Vec<T>>;
    fn choose(&mut self, choice: T) -> Result<(), ()>;
    fn choice(&self) -> Option<T>;
    fn has_chosen(&self) -> bool {
        if let Some(_) = self.choice() {
            true
        } else {
            false
        }
    }
    fn has_chosen_if_possible(&self) -> bool {
        match self.choices() {
            None => true,
            Some(_) => self.has_chosen(),
        }
    }
}

trait CanChoose<T> {
    fn can_choose(&self, choice: &T) -> bool;
}

impl<T, C> CanChoose<C> for T
where
    T: Choose<C>,
    C: PartialEq,
{
    fn can_choose(&self, choice: &C) -> bool {
        if let Some(choices) = self.choices() {
            choices.contains(&choice) && !self.has_chosen()
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArsenalItem {
    Mirror,
    Move(Move),
}

impl ArsenalItem {
    pub(crate) fn as_move_action(&self) -> Option<Action> {
        if let ArsenalItem::Move(m) = &self {
            Some(Action::Move(*m))
        } else {
            None
        }
    }

    pub(crate) fn as_mirror_action(&self) -> Option<Action> {
        if let ArsenalItem::Move(m) = &self {
            Some(Action::Mirror(*m))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Mirror(Move),
    Move(Move),
    Concede,
}

impl Action {
    fn into_opt_arsenal_item(self) -> Option<ArsenalItem> {
        match self {
            Action::Mirror(_) => Some(ArsenalItem::Mirror),
            Action::Move(m) => Some(ArsenalItem::Move(m)),
            Action::Concede => None,
        }
    }

    pub(crate) fn points_against(&self, other: &Action) -> u8 {
        let own_move = self.move_();
        let other_move = other.move_();
        match (own_move, other_move) {
            (Some(own), Some(other)) => own.points_against(other),
            (Some(_own), None) => 1,
            (None, Some(_other)) => 0,
            (None, None) => 0,
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
