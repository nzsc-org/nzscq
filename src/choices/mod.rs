mod batch_choice;
pub use batch_choice::{BatchChoice, BatchChoices};

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
    fn choices(&self) -> Vec<T>;
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
        self.choices().contains(&choice)
    }
}

impl<P, C> CanChoose<Vec<C>> for Vec<P>
where
    P: Choose<C>,
    C: PartialEq,
{
    fn can_choose(&self, choices: &Vec<C>) -> bool {
        if self.len() != choices.len() {
            false
        } else {
            self.iter()
                .zip(choices)
                .all(|(player, choice)| player.can_choose(choice))
        }
    }
}

pub trait PointsAgainst: Sized {
    fn points_against(&self, other: &Self) -> u8;
    fn points_of(choices: &[Self]) -> Vec<u8> {
        let mut points = vec![0; choices.len()];
        for (i, a) in choices.iter().enumerate() {
            for b in choices {
                points[i] += a.points_against(b);
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub enum RPS {
        Rock,
        Paper,
        Scissors,
    }

    impl PointsAgainst for RPS {
        fn points_against(&self, other: &RPS) -> u8 {
            match (self, other) {
                (RPS::Rock, RPS::Rock) => 0,
                (RPS::Rock, RPS::Paper) => 0,
                (RPS::Rock, RPS::Scissors) => 1,

                (RPS::Paper, RPS::Rock) => 1,
                (RPS::Paper, RPS::Paper) => 0,
                (RPS::Paper, RPS::Scissors) => 0,

                (RPS::Scissors, RPS::Rock) => 0,
                (RPS::Scissors, RPS::Paper) => 1,
                (RPS::Scissors, RPS::Scissors) => 0,
            }
        }
    }

    #[test]
    fn rock_beats_scissors() {
        assert_eq!(1, RPS::Rock.points_against(&RPS::Scissors));
        assert_eq!(0, RPS::Scissors.points_against(&RPS::Rock));
    }

    #[test]
    fn points_of_rock_rock_paper_scissors_are_1_1_2_1() {
        assert_eq!(
            vec![1, 1, 2, 1],
            RPS::points_of(&[RPS::Rock, RPS::Rock, RPS::Paper, RPS::Scissors])
        );
    }

}
