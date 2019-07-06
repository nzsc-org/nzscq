use super::{Action, Booster, Character, DequeueChoice};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BatchChoices {
    Characters(Vec<Vec<Character>>),
    Boosters(Vec<Vec<Booster>>),
    DequeueChoices(Vec<Vec<DequeueChoice>>),
    Actions(Vec<Vec<Action>>),
    None,
}

impl BatchChoices {
    pub fn characters(self) -> Option<Vec<Vec<Character>>> {
        if let BatchChoices::Characters(characters) = self {
            Some(characters)
        } else {
            None
        }
    }

    pub fn boosters(self) -> Option<Vec<Vec<Booster>>> {
        if let BatchChoices::Boosters(boosters) = self {
            Some(boosters)
        } else {
            None
        }
    }

    pub fn dequeue_choices(self) -> Option<Vec<Vec<DequeueChoice>>> {
        if let BatchChoices::DequeueChoices(dequeue_choices) = self {
            Some(dequeue_choices)
        } else {
            None
        }
    }

    pub fn actions(self) -> Option<Vec<Vec<Action>>> {
        if let BatchChoices::Actions(actions) = self {
            Some(actions)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters_returns_some_if_variant_is_characters() {
        let choices = BatchChoices::Characters(vec![]);
        assert!(choices.characters().is_some());
    }

    #[test]
    fn characters_returns_none_if_variant_is_not_characters() {
        let choices = BatchChoices::Boosters(vec![]);
        assert!(choices.characters().is_none());
    }

    #[test]
    fn boosters_returns_some_if_variant_is_boosters() {
        let choices = BatchChoices::Boosters(vec![]);
        assert!(choices.boosters().is_some());
    }

    #[test]
    fn boosters_returns_none_if_variant_is_not_boosters() {
        let choices = BatchChoices::Characters(vec![]);
        assert!(choices.boosters().is_none());
    }

    #[test]
    fn dequeue_choices_returns_some_if_variant_is_dequeue_choices() {
        let choices = BatchChoices::DequeueChoices(vec![]);
        assert!(choices.dequeue_choices().is_some());
    }

    #[test]
    fn dequeue_choices_returns_none_if_variant_is_not_dequeue_choices() {
        let choices = BatchChoices::Actions(vec![]);
        assert!(choices.dequeue_choices().is_none());
    }

    #[test]
    fn actions_returns_some_if_variant_is_actions() {
        let choices = BatchChoices::Actions(vec![]);
        assert!(choices.actions().is_some());
    }

    #[test]
    fn actions_returns_none_if_variant_is_not_actions() {
        let choices = BatchChoices::None;
        assert!(choices.actions().is_none());
    }
}
