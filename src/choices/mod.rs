mod moves;
pub use moves::Move;
mod boosters;
pub use boosters::Booster;
mod characters;
pub use characters::Character;
mod arsenal_item;
pub use arsenal_item::ArsenalItem;
mod action;
pub use action::Action;
mod dequeue_choice;
pub use dequeue_choice::DequeueChoice;

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

pub(crate) trait CanChoose<T> {
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
