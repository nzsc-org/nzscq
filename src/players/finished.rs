use crate::choices::{ArsenalItem, Booster, Character};
use crate::counters::Queue;
use crate::game::Config;
use crate::scoreboard::transparent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishedPlayer {
    pub(super) game_config: Config,
    pub(crate) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
}

impl Into<transparent::FinishedPlayer> for FinishedPlayer {
    fn into(self) -> transparent::FinishedPlayer {
        transparent::FinishedPlayer {
            points: self.points,
            character: self.character,
            booster: self.booster,
            arsenal: self.arsenal,
            queue: self.queue.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finished_ninja() -> FinishedPlayer {
        use crate::choices::{DequeueChoice, Move};

        let mut queue = Queue::new();
        queue.enqueue(Some(ArsenalItem::Move(Move::NinjaSword)));
        queue.enqueue(Some(ArsenalItem::Move(Move::Kick)));
        queue.enqueue(Some(ArsenalItem::Move(Move::ShadowSlip)));
        queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();

        FinishedPlayer {
            points: 5,
            game_config: Config::default(),
            character: Character::Ninja,
            booster: Booster::Shadow,
            arsenal: vec![
                ArsenalItem::Move(Move::ShadowFireball),
                ArsenalItem::Move(Move::Nunchucks),
            ],
            queue,
        }
    }

    #[test]
    fn into_transparent_works() {
        let original = finished_ninja();
        let transparent: transparent::FinishedPlayer = original.clone().into();

        assert_eq!(original.points, transparent.points);
        assert_eq!(original.character, transparent.character);
        assert_eq!(original.booster, transparent.booster);
        assert_eq!(original.arsenal, transparent.arsenal);
        assert_eq!(
            Into::<transparent::Queue>::into(original.queue),
            transparent.queue
        );
    }
}
