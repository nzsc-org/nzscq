use super::ActionlessPlayer;
use crate::choices::{ArsenalItem, Booster, CanChoose, Character, Choose, DequeueChoice};
use crate::counters::Queue;
use crate::game::GameConfig;

#[derive(Debug, Clone)]
pub struct DraineelessPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
    pub(super) choice: Option<DequeueChoice>,
}

impl DraineelessPlayer {
    pub fn into_actionless(self) -> Result<ActionlessPlayer, ()> {
        if self.has_chosen() {
            Ok(ActionlessPlayer {
                game_config: self.game_config,
                points: self.points,
                character: self.character,
                booster: self.booster,
                arsenal: self.arsenal,
                queue: self.queue,
                pending_action: None,
            })
        } else {
            Err(())
        }
    }

    fn can_dequeue(&self) -> bool {
        self.arsenal.len() < self.game_config.max_arsenal_items as usize || self.queue.exit_vacant()
    }
}

impl Choose<DequeueChoice> for DraineelessPlayer {
    fn choices(&self) -> Option<Vec<DequeueChoice>> {
        if self.has_chosen() {
            None
        } else if self.can_dequeue() {
            let mut items: Vec<DequeueChoice> = self
                .queue
                .pool()
                .iter()
                .map(|item| DequeueChoice::DrainAndExit(item.clone()))
                .collect();
            items.push(DequeueChoice::JustExit);
            items.push(DequeueChoice::Decline);
            Some(items)
        } else {
            Some(vec![DequeueChoice::Decline])
        }
    }

    fn choose(&mut self, drainee: DequeueChoice) -> Result<(), ()> {
        if self.can_choose(&drainee) {
            let exited = self.queue.dequeue(drainee).unwrap();
            if let Some(exited) = exited {
                self.arsenal.push(exited);
            }
            self.choice = Some(drainee);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<DequeueChoice> {
        self.choice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shadow() -> DraineelessPlayer {
        use crate::player::CharacterlessPlayer;

        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.choose(Character::Ninja).unwrap();
        let mut ninja = player.into_boosterless().unwrap();
        ninja.choose(Booster::Shadow).unwrap();
        ninja.into_draineeless().unwrap()
    }

    #[test]
    fn can_decline_opportunity_to_drain_if_pool_occupied() {
        let mut shadow = shadow();
        assert!(shadow.choices().unwrap().contains(&DequeueChoice::Decline));
        assert!(shadow.choose(DequeueChoice::Decline).is_ok());
        assert_eq!(shadow.choice(), Some(DequeueChoice::Decline));
    }

    #[test]
    fn must_decline_opportunity_to_drain_if_pool_empty() {
        let mut shadow = shadow();
        shadow
            .queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        assert_eq!(0, shadow.queue.pool().len());
        assert_eq!(Some(vec![DequeueChoice::Decline]), shadow.choices());
        assert!(shadow.choose(DequeueChoice::Decline).is_ok());
        assert_eq!(shadow.choice(), Some(DequeueChoice::Decline));
    }

    #[test]
    fn can_choose_mirror_if_mirror_in_pool() {
        let mut shadow = shadow();
        let drain_mirror = DequeueChoice::DrainAndExit(ArsenalItem::Mirror);
        assert_eq!(
            Some(vec![
                drain_mirror,
                DequeueChoice::JustExit,
                DequeueChoice::Decline
            ]),
            shadow.choices()
        );
        assert!(shadow.choose(drain_mirror).is_ok());
        assert_eq!(Some(drain_mirror), shadow.choice());
    }

    #[test]
    fn must_exit_without_draining_or_decline_if_pool_is_empty() {
        let mut shadow = shadow();
        let drain_mirror = DequeueChoice::DrainAndExit(ArsenalItem::Mirror);
        shadow.queue.dequeue(drain_mirror).unwrap();
        shadow.arsenal = vec![];
        assert!(shadow.can_dequeue());
        assert_eq!(
            Some(vec![DequeueChoice::JustExit, DequeueChoice::Decline]),
            shadow.choices()
        );
    }

    #[test]
    fn cannot_choose_if_has_already_chosen() {
        let mut shadow = shadow();
        let drain_mirror = DequeueChoice::DrainAndExit(ArsenalItem::Mirror);
        shadow.choose(drain_mirror).unwrap();
        assert_eq!(None, shadow.choices());
        assert!(shadow.choose(drain_mirror).is_err());
        assert_eq!(Some(drain_mirror), shadow.choice());
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
