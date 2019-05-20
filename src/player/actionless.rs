use super::{Action, ArsenalItem, CanChoose, Choose, DraineelessPlayer, FinishedPlayer};
use crate::boosters::Booster;
use crate::characters::Character;
use crate::game::GameConfig;
use crate::queue::Queue;

#[derive(Debug, Clone)]
pub struct ActionlessPlayer {
    pub(super) game_config: GameConfig,
    pub(super) points: u8,
    pub(super) character: Character,
    pub(super) booster: Booster,
    pub(super) arsenal: Vec<ArsenalItem>,
    pub(super) queue: Queue,
    pub(super) pending_action: Option<Action>,
}

impl ActionlessPlayer {
    pub(crate) fn add_points(&mut self, points: u8) {
        self.points += points;
    }

    pub(crate) fn needs_points_to_win(&self) -> bool {
        self.points < self.game_config.max_points
    }

    pub fn into_draineeless(mut self) -> Result<DraineelessPlayer, ()> {
        if let Some(action) = self.pending_action {
            let arsenal_item = action.into_opt_arsenal_item();
            if let Some(arsenal_item) = &arsenal_item {
                self.arsenal.retain(|m| m != arsenal_item);
            }
            self.queue.enqueue(arsenal_item);

            Ok(DraineelessPlayer {
                game_config: self.game_config,
                points: self.points,
                character: self.character,
                booster: self.booster,
                arsenal: self.arsenal,
                queue: self.queue,
                choice: None,
            })
        } else {
            Err(())
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
    fn choices(&self) -> Option<Vec<Action>> {
        if self.has_chosen() {
            None
        } else {
            let has_mirror = self.arsenal.contains(&ArsenalItem::Mirror);
            let mut actions: Vec<Action> = vec![];
            let mut move_actions: Vec<Action> = self
                .arsenal
                .iter()
                .filter(|item| item != &&ArsenalItem::Mirror)
                .map(|item| item.as_move_action().unwrap())
                .collect();
            actions.append(&mut move_actions);
            if has_mirror {
                let mut mirror_actions: Vec<Action> = self
                    .queue
                    .pool()
                    .clone()
                    .into_iter()
                    .map(|item| item.as_mirror_action().unwrap())
                    .collect();
                actions.append(&mut mirror_actions);
            }
            if actions.len() == 0 {
                actions.push(Action::Concede);
            }

            Some(actions)
        }
    }

    fn choose(&mut self, action: Action) -> Result<(), ()> {
        if self.can_choose(&action) {
            self.pending_action = Some(action);
            Ok(())
        } else {
            Err(())
        }
    }

    fn choice(&self) -> Option<Action> {
        self.pending_action
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::CharacterlessPlayer;
    use crate::queue::DequeueChoice;

    fn actionless_shadow() -> ActionlessPlayer {
        let mut shadow = draineeless_shadow();
        shadow
            .choose(DequeueChoice::DrainAndExit(ArsenalItem::Mirror))
            .unwrap();
        shadow.into_actionless().unwrap()
    }

    fn draineeless_shadow() -> DraineelessPlayer {
        let mut player = CharacterlessPlayer::from_game_config(GameConfig::default());
        player.choose(Character::Ninja).unwrap();
        let mut ninja = player.into_boosterless().unwrap();
        ninja.choose(Booster::Shadow).unwrap();
        ninja.into_draineeless().unwrap()
    }

    #[test]
    fn shadow_initally_has_five_choices() {
        let shadow = actionless_shadow();
        assert_eq!(5, shadow.choices().unwrap().len());
    }

    #[test]
    fn add_points_works() {
        let mut shadow = actionless_shadow();
        shadow.add_points(3);
        assert_eq!(shadow.points, 3);
    }

    #[test]
    fn needs_points_to_win_if_points_less_than_max() {
        let mut shadow = actionless_shadow();
        let one_less = shadow.game_config.max_points - 1;
        shadow.add_points(one_less);
        assert!(shadow.needs_points_to_win());
    }

    #[test]
    fn does_not_need_points_to_win_if_points_equals_max() {
        let mut shadow = actionless_shadow();
        shadow.add_points(shadow.game_config.max_points);
        assert!(!shadow.needs_points_to_win());
    }

    #[test]
    fn into_draineeless_works_if_player_has_chosen() {
        use crate::moves::Move;

        let mut shadow = actionless_shadow();
        shadow.choose(Action::Move(Move::Kick)).unwrap();
        assert!(shadow.into_draineeless().is_ok());
    }

    #[test]
    fn into_draineeless_fails_if_player_has_not_chosen() {
        let shadow = actionless_shadow();
        assert!(shadow.into_draineeless().is_err());
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
