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
    fn choice(&self) -> Option<&T>;
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

pub(crate) trait ChooseRef<T> {
    fn choices(&self) -> Option<&Vec<T>>;
    fn choose(&mut self, choice: T) -> Result<(), ()>;
    fn choice(&self) -> Option<&T>;
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

trait CanChooseRef<T> {
    fn can_choose(&self, choice: &T) -> bool;
}

impl<T, C> CanChooseRef<C> for T
where
    T: ChooseRef<C>,
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

#[derive(Clone, Debug, PartialEq)]
pub enum ArsenalItem {
    Mirror,
    Move(Move),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Mirror(Move),
    Move(Move),
}

impl Action {
    fn into_arsenal_item(self) -> ArsenalItem {
        match self {
            Action::Mirror(_) => ArsenalItem::Mirror,
            Action::Move(m) => ArsenalItem::Move(m),
        }
    }

    pub(crate) fn points_against(&self, other: &Action) -> u8 {
        let own_move = self.move_();
        let other_move = other.move_();
        own_move.points_against(other_move)
    }

    fn move_(&self) -> Move {
        match self {
            Action::Mirror(m) => *m,
            Action::Move(m) => *m,
        }
    }
}
