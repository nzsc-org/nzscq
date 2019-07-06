use super::{DequeueChoicelessPlayer, FinishedPlayer};
use crate::choices::{Action, ArsenalItem, Booster, Character, Choose};
use crate::counters::Queue;
use crate::game::Config;
use crate::outcomes::ActionPointsDestroyed;
use crate::scoreboard::transparent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionlessPlayer {
    pub(super) game_config: Config,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
}

impl ActionlessPlayer {
    pub fn points(&self) -> u8 {
        self.points
    }

    pub fn into_dequeue_choiceless(
        mut self,
        ActionPointsDestroyed(action, points, action_destroyed): ActionPointsDestroyed,
    ) -> DequeueChoicelessPlayer {
        self.use_action(action, action_destroyed);
        self.add_points(points);

        DequeueChoicelessPlayer {
            game_config: self.game_config,
            points: self.points,
            character: self.character,
            booster: self.booster,
            arsenal: self.arsenal,
            queue: self.queue,
        }
    }

    pub fn into_finished(
        mut self,
        ActionPointsDestroyed(action, points, action_destroyed): ActionPointsDestroyed,
    ) -> FinishedPlayer {
        self.use_action(action, action_destroyed);
        self.add_points(points);

        FinishedPlayer {
            game_config: self.game_config,
            points: self.points,
            character: self.character,
            booster: self.booster,
            arsenal: self.arsenal,
            queue: self.queue,
        }
    }

    fn use_action(&mut self, action: Action, action_destroyed: bool) {
        let arsenal_item: Option<ArsenalItem> = action.into();
        if let Some(arsenal_item) = &arsenal_item {
            self.arsenal.retain(|m| m != arsenal_item);
        }
        if !action_destroyed {
            self.queue.enqueue(arsenal_item);
        }
    }

    fn add_points(&mut self, points: i8) {
        let new_points = self.points as i8 + points;
        self.points = if new_points < 0 { 0 } else { new_points as u8 };
    }
}

impl Choose<Action> for ActionlessPlayer {
    fn choices(&self) -> Vec<Action> {
        let has_mirror = self.arsenal.contains(&ArsenalItem::Mirror);
        let mut actions: Vec<Action> = vec![];
        let move_actions: Vec<Action> = self
            .arsenal
            .iter()
            .filter(|item| item != &&ArsenalItem::Mirror)
            .map(|item| item.as_move_action().unwrap())
            .collect();
        actions.extend(move_actions);
        if has_mirror {
            let mirror_actions: Vec<Action> = self
                .queue
                .pool()
                .clone()
                .into_iter()
                .map(|item| item.as_mirror_action().unwrap())
                .collect();
            actions.extend(mirror_actions);
        }
        if actions.is_empty() {
            actions.push(Action::Concede);
        }

        actions
    }
}

impl Into<transparent::ActionlessPlayer> for ActionlessPlayer {
    fn into(self) -> transparent::ActionlessPlayer {
        transparent::ActionlessPlayer {
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
    use crate::choices::DequeueChoice;
    use crate::players::CharacterlessPlayer;

    fn actionless_shadow() -> ActionlessPlayer {
        let shadow = draineeless_shadow();
        shadow.into_actionless(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
    }

    fn draineeless_shadow() -> DequeueChoicelessPlayer {
        use crate::outcomes::CharacterHeadstart;

        let player = CharacterlessPlayer::from_game_config(Config::default());
        player
            .into_boosterless(CharacterHeadstart(Character::Ninja, 0))
            .into_dequeue_choiceless(Booster::Shadow)
    }

    #[test]
    fn shadow_initally_has_five_choices() {
        let shadow = actionless_shadow();
        assert_eq!(5, shadow.choices().len());
    }

    #[test]
    fn shadow_has_correct_mirror_choices() {
        use crate::choices::Move;

        let mut actionless_shadow = actionless_shadow();
        let mut draineeless_shadow;
        let choices = vec![
            (Action::Move(Move::Kick), DequeueChoice::Decline),
            (Action::Move(Move::Nunchucks), DequeueChoice::Decline),
            (Action::Move(Move::ShadowFireball), DequeueChoice::Decline),
            (
                Action::Move(Move::ShadowSlip),
                DequeueChoice::DrainAndExit(ArsenalItem::Move(Move::Nunchucks)),
            ),
        ];
        for (action, dequeue_choice) in choices {
            draineeless_shadow =
                actionless_shadow.into_dequeue_choiceless(ActionPointsDestroyed(action, 0, false));
            actionless_shadow = draineeless_shadow.into_actionless(dequeue_choice);
        }

        println!("{:#?}", actionless_shadow);
        assert_eq!(
            vec![
                Action::Move(Move::NinjaSword),
                Action::Mirror(Move::Kick),
                Action::Mirror(Move::ShadowFireball)
            ],
            actionless_shadow.choices()
        );
    }

    #[test]
    fn add_points_adds_points_if_new_points_is_greater_than_zero() {
        let mut shadow = actionless_shadow();
        shadow.points = 0;
        shadow.add_points(3);
        assert_eq!(3, shadow.points);
    }

    #[test]
    fn add_points_adds_points_if_new_points_is_equal_to_zero() {
        let mut shadow = actionless_shadow();
        shadow.points = 3;
        shadow.add_points(-3);
        assert_eq!(0, shadow.points);
    }

    #[test]
    fn add_points_sets_points_to_zero_if_new_points_is_less_than_zero() {
        let mut shadow = actionless_shadow();
        shadow.points = 3;
        shadow.add_points(-4);
        assert_eq!(0, shadow.points);
    }

    #[test]
    fn into_draineeless_works_if_action_destroyed() {
        use crate::choices::Move;

        let shadow = actionless_shadow();

        let expected_queue = shadow.queue.clone();

        assert_eq!(
            expected_queue,
            shadow
                .into_dequeue_choiceless(ActionPointsDestroyed(Action::Move(Move::Kick), 0, true))
                .queue
        );
    }

    #[test]
    fn into_draineeless_works_if_action_not_destroyed() {
        use crate::choices::Move;

        let shadow = actionless_shadow();

        let mut expected_queue = Queue::new();
        expected_queue
            .dequeue(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        expected_queue.enqueue(Some(ArsenalItem::Move(Move::Kick)));

        assert_eq!(
            expected_queue,
            shadow
                .into_dequeue_choiceless(ActionPointsDestroyed(Action::Move(Move::Kick), 0, false))
                .queue
        );
    }

    #[test]
    fn into_finished_works() {
        use crate::choices::Move;

        let actionless = actionless_shadow();
        let apd = ActionPointsDestroyed(Action::Move(Move::Kick), 0, false);
        let expected = actionless.clone().into_dequeue_choiceless(apd.clone());
        let finished = actionless.into_finished(apd);

        assert_eq!(expected.game_config, finished.game_config);
        assert_eq!(expected.points, finished.points);
        assert_eq!(expected.character, finished.character);
        assert_eq!(expected.booster, finished.booster);
        assert_eq!(expected.arsenal, finished.arsenal);
        assert_eq!(expected.queue, finished.queue);
    }

    #[test]
    fn into_transparent_works() {
        let original = actionless_shadow();
        let transparent: transparent::ActionlessPlayer = original.clone().into();

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
