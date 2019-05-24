use super::{Action, Booster, Character, DequeueChoice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchChoice {
    Characters(Vec<Character>),
    Boosters(Vec<Booster>),
    DequeueChoices(Vec<DequeueChoice>),
    Actions(Vec<Action>),
}

impl BatchChoice {
    pub fn len(&self) -> usize {
        match self {
            BatchChoice::Characters(characters) => characters.len(),
            BatchChoice::Boosters(boosters) => boosters.len(),
            BatchChoice::DequeueChoices(dequeue_choices) => dequeue_choices.len(),
            BatchChoice::Actions(actions) => actions.len(),
        }
    }
}
