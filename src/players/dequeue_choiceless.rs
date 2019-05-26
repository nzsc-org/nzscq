use super::ActionlessPlayer;
use crate::choices::{ArsenalItem, Booster, Character, Choose, DequeueChoice};
use crate::counters::Queue;
use crate::game::Config;

#[derive(Debug, Clone)]
pub struct DequeueChoicelessPlayer {
    pub(super) game_config: Config,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
}

impl DequeueChoicelessPlayer {
    pub fn into_actionless(mut self, dequeue_choice: DequeueChoice) -> ActionlessPlayer {
        if let Some(arsenal_item) = self.queue.dequeue(dequeue_choice).unwrap() {
            self.arsenal.push(arsenal_item);
        }

        ActionlessPlayer {
            game_config: self.game_config,
            points: self.points,
            character: self.character,
            booster: self.booster,
            arsenal: self.arsenal,
            queue: self.queue,
            action_destroyed: false,
        }
    }

    fn can_dequeue(&self) -> bool {
        self.arsenal.len() < self.game_config.max_arsenal_items as usize || self.queue.exit_vacant()
    }
}

impl Choose<DequeueChoice> for DequeueChoicelessPlayer {
    fn choices(&self) -> Vec<DequeueChoice> {
        if self.can_dequeue() {
            let mut items: Vec<DequeueChoice> = self
                .queue
                .pool()
                .iter()
                .map(|item| DequeueChoice::DrainAndExit(item.clone()))
                .collect();
            items.push(DequeueChoice::JustExit);
            items.push(DequeueChoice::Decline);
            items
        } else {
            vec![DequeueChoice::Decline]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shadow() -> DequeueChoicelessPlayer {
        use crate::players::CharacterlessPlayer;

        let player = CharacterlessPlayer::from_game_config(Config::default());
        player
            .into_boosterless(Character::Ninja)
            .into_draineeless(Booster::Shadow)
    }

    #[test]
    fn can_decline_opportunity_to_drain_if_pool_occupied() {
        let shadow = shadow();
        assert!(shadow.choices().contains(&DequeueChoice::Decline));
    }

    #[test]
    fn must_decline_opportunity_to_drain_if_pool_empty() {
        let mut shadow = shadow();
        shadow
            .queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        assert_eq!(0, shadow.queue.pool().len());
        assert_eq!(vec![DequeueChoice::Decline], shadow.choices());
    }

    #[test]
    fn can_choose_mirror_if_mirror_in_pool() {
        let shadow = shadow();
        let drain_mirror = DequeueChoice::DrainAndExit(ArsenalItem::Mirror);
        assert_eq!(
            vec![
                drain_mirror,
                DequeueChoice::JustExit,
                DequeueChoice::Decline
            ],
            shadow.choices()
        );
    }

    #[test]
    fn must_exit_without_draining_or_decline_if_pool_is_empty() {
        let mut shadow = shadow();
        let drain_mirror = DequeueChoice::DrainAndExit(ArsenalItem::Mirror);
        shadow.queue.dequeue(drain_mirror).unwrap();
        shadow.arsenal = vec![];
        assert!(shadow.can_dequeue());
        assert_eq!(
            vec![DequeueChoice::JustExit, DequeueChoice::Decline],
            shadow.choices()
        );
    }

    #[test]
    fn can_dequeue_if_queue_exit_empty() {
        let shadow = shadow();
        assert!(shadow.arsenal.len() >= shadow.game_config.max_arsenal_items as usize);
        assert!(shadow.can_dequeue());
    }

    #[test]
    fn can_dequeue_if_arsenal_has_extra_capacity() {
        use crate::choices::Move;

        let mut shadow = shadow();
        shadow
            .queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        assert!(!shadow.queue.exit_vacant());
        shadow.arsenal = vec![ArsenalItem::Move(Move::Nunchucks)];
        assert!(shadow.can_dequeue());
    }

    #[test]
    fn cannot_dequeue_if_queue_exit_occupied_and_arsenal_has_no_capacity() {
        use crate::choices::Move;

        let mut shadow = shadow();
        shadow
            .queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        assert!(!shadow.queue.exit_vacant());
        shadow.arsenal = vec![
            ArsenalItem::Move(Move::Nunchucks),
            ArsenalItem::Move(Move::ShadowFireball),
        ];
        assert!(!shadow.can_dequeue());
    }
}
