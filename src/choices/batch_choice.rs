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

    pub fn is_empty(&self) -> bool {
        match self {
            BatchChoice::Characters(characters) => characters.is_empty(),
            BatchChoice::Boosters(boosters) => boosters.is_empty(),
            BatchChoice::DequeueChoices(dequeue_choices) => dequeue_choices.is_empty(),
            BatchChoice::Actions(actions) => actions.is_empty(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BatchChoices {
    Characters(Vec<Vec<Character>>),
    Boosters(Vec<Vec<Booster>>),
    DequeueChoices(Vec<Vec<DequeueChoice>>),
    Actions(Vec<Vec<Action>>),
    None,
}
