use super::{DraineelessPlayer, FinishedPlayer};
use crate::choices::{Action, ArsenalItem, Booster, Character, Choose};
use crate::counters::Queue;
use crate::game::Config;

#[derive(Debug, Clone)]
pub struct ActionlessPlayer {
    pub(super) game_config: Config,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
    pub(super) action_destroyed: bool,
}

impl ActionlessPlayer {
    pub fn points(&self) -> u8 {
        self.points
    }

    pub fn add_points(&mut self, points: u8) {
        self.points += points;
    }

    pub fn deduct_points(&mut self, points: u8) {
        if self.points < points {
            self.points = 0;
        } else {
            self.points -= points;
        }
    }

    pub fn destroy_action(&mut self) {
        self.action_destroyed = true;
    }

    pub fn into_draineeless(mut self, action: Action) -> DraineelessPlayer {
        let arsenal_item = action.into_opt_arsenal_item();
        if let Some(arsenal_item) = &arsenal_item {
            self.arsenal.retain(|m| m != arsenal_item);
        }

        if !self.action_destroyed {
            self.queue.enqueue(arsenal_item);
        }

        DraineelessPlayer {
            game_config: self.game_config,
            points: self.points,
            character: self.character,
            booster: self.booster,
            arsenal: self.arsenal,
            queue: self.queue,
        }
    }

    pub fn into_finished(self) -> FinishedPlayer {
        FinishedPlayer {
            game_config: self.game_config,
            points: self.points,
            character: self.character,
            booster: self.booster,
            arsenal: self.arsenal,
            queue: self.queue,
        }
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
        if actions.len() == 0 {
            actions.push(Action::Concede);
        }

        actions
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

    fn draineeless_shadow() -> DraineelessPlayer {
        let player = CharacterlessPlayer::from_game_config(Config::default());
        player
            .into_boosterless(Character::Ninja)
            .into_draineeless(Booster::Shadow)
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
            draineeless_shadow = actionless_shadow.into_draineeless(action);
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
    fn add_points_works() {
        let mut shadow = actionless_shadow();
        assert_eq!(0, shadow.points);
        shadow.add_points(3);
        assert_eq!(3, shadow.points);
    }

    #[test]
    fn deduct_points_deducts_if_player_has_enough_points() {
        let mut shadow = actionless_shadow();
        assert_eq!(0, shadow.points);
        shadow.add_points(3);
        shadow.deduct_points(2);
        assert_eq!(1, shadow.points);
    }

    #[test]
    fn deduct_points_sets_points_to_zero_if_player_does_not_have_enough_points() {
        let mut shadow = actionless_shadow();
        assert_eq!(0, shadow.points);
        shadow.add_points(3);
        shadow.deduct_points(5);
        assert_eq!(0, shadow.points);
    }

    #[test]
    fn into_draineeless_works_if_action_destroyed() {
        use crate::choices::Move;

        let mut shadow = actionless_shadow();
        shadow.destroy_action();

        let expected_queue = shadow.queue.clone();

        assert_eq!(
            expected_queue,
            shadow.into_draineeless(Action::Move(Move::Kick)).queue
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
            shadow.into_draineeless(Action::Move(Move::Kick)).queue
        );
    }

    #[test]
    fn into_finished_works() {
        let original = actionless_shadow();
        let finished = original.clone().into_finished();
        assert_eq!(original.game_config, finished.game_config);
        assert_eq!(original.points, finished.points);
        assert_eq!(original.character, finished.character);
        assert_eq!(original.booster, finished.booster);
        assert_eq!(original.arsenal, finished.arsenal);
    }
}
