use super::{Config, Phase};
use crate::{
    choices::{
        Action, BatchChoice, BatchChoices, Booster, CanChoose, Character, Choose, DequeueChoice,
        PointsAgainst,
    },
    helpers::HasDuplicates,
    outcomes::{ActionPointsDestroyed, CharacterHeadstart, Outcome},
    players::{CharacterlessPlayer, DraineelessPlayer, FinishedPlayer},
};

use std::mem;

#[derive(Debug, Clone)]
pub struct BatchChoiceGame {
    config: Config,
    phase: Phase,
}

impl BatchChoiceGame {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            phase: Phase::Character(Self::initial_players(&config)),
        }
    }

    fn initial_players(config: &Config) -> Vec<CharacterlessPlayer> {
        let mut players: Vec<CharacterlessPlayer> = vec![];
        for _ in 0..config.player_count {
            players.push(CharacterlessPlayer::from_game_config(config.clone()))
        }
        players
    }

    pub fn choices(&self) -> BatchChoices {
        match &self.phase {
            Phase::Character(players) => {
                BatchChoices::Character(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Booster(players) => {
                BatchChoices::Booster(players.iter().map(|p| p.choices()).collect())
            }

            Phase::DrainedMove(players) => {
                BatchChoices::DrainedMove(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Action(players) => {
                BatchChoices::Action(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Final(_) => BatchChoices::None,
        }
    }

    pub fn choose(&mut self, choices: BatchChoice) -> Result<Outcome, ()> {
        if self.config.player_count as usize != choices.len() {
            Err(())
        } else {
            match choices {
                BatchChoice::Characters(characters) => self.choose_characters(characters),

                BatchChoice::Boosters(boosters) => self.choose_boosters(boosters),
                BatchChoice::DequeueChoices(dequeue_choices) => {
                    self.choose_dequeue_choices(dequeue_choices)
                }
                BatchChoice::Actions(actions) => self.choose_actions(actions),
            }
        }
    }

    fn choose_characters(&mut self, characters: Vec<Character>) -> Result<Outcome, ()> {
        if let Phase::Character(players) = &mut self.phase {
            if !players.can_choose(&characters) {
                Err(())
            } else if characters.has_duplicates() {
                for (player, character) in players.into_iter().zip(&characters) {
                    player.add_to_streak(*character);
                }
                Ok(Outcome::CharacterPhaseRechoose(characters))
            } else {
                let dummy = vec![];
                let players = mem::replace(players, dummy);

                Ok(self.complete_character_phase(players, characters))
            }
        } else {
            Err(())
        }
    }

    fn complete_character_phase(
        &mut self,
        mut players: Vec<CharacterlessPlayer>,
        characters: Vec<Character>,
    ) -> Outcome {
        let headstarts = Character::points_of(&characters);
        let character_headstarts: Vec<CharacterHeadstart> = characters
            .iter()
            .zip(headstarts)
            .map(|(character, headstart)| CharacterHeadstart(*character, headstart))
            .collect();

        for (player, headstart) in players.iter_mut().zip(&character_headstarts) {
            player.add_points(headstart.1);
        }

        self.phase = Phase::Booster(
            players
                .into_iter()
                .zip(characters)
                .map(|(p, c)| p.into_boosterless(c))
                .collect(),
        );

        Outcome::CharacterPhaseDone(character_headstarts)
    }

    fn choose_boosters(&mut self, boosters: Vec<Booster>) -> Result<Outcome, ()> {
        if let Phase::Booster(players) = &mut self.phase {
            if !players.can_choose(&boosters) {
                Err(())
            } else {
                let dummy = vec![];
                let players = mem::replace(players, dummy);
                self.phase = Phase::DrainedMove(
                    players
                        .into_iter()
                        .zip(&boosters)
                        .map(|(player, booster)| player.into_draineeless(*booster))
                        .collect(),
                );
                Ok(Outcome::BoosterPhaseDone(boosters))
            }
        } else {
            Err(())
        }
    }

    fn choose_dequeue_choices(
        &mut self,
        dequeue_choices: Vec<DequeueChoice>,
    ) -> Result<Outcome, ()> {
        if let Phase::DrainedMove(players) = &mut self.phase {
            if !players.can_choose(&dequeue_choices) {
                Err(())
            } else {
                let dummy = vec![];
                let players = mem::replace(players, dummy);
                self.phase = Phase::Action(
                    players
                        .into_iter()
                        .zip(&dequeue_choices)
                        .map(|(player, dequeue_choice)| player.into_actionless(*dequeue_choice))
                        .collect(),
                );
                Ok(Outcome::DequeuePhaseDone(dequeue_choices))
            }
        } else {
            Err(())
        }
    }

    fn choose_actions(&mut self, actions: Vec<Action>) -> Result<Outcome, ()> {
        if let Phase::Action(players) = &mut self.phase {
            if !players.can_choose(&actions) {
                Err(())
            } else {
                let points = Action::points_of(&actions);
                let action_points_destroyed: Vec<ActionPointsDestroyed> = actions
                    .iter()
                    .zip(points)
                    .zip(Action::which_destroyed(&actions))
                    .map(|((action, points), destroyed)| {
                        ActionPointsDestroyed(*action, points, destroyed)
                    })
                    .collect();

                for (player, &ActionPointsDestroyed(_, points, destroyed)) in
                    players.iter_mut().zip(&action_points_destroyed)
                {
                    player.add_points(points);
                    if destroyed {
                        player.destroy_action();
                    }
                }

                let have_any_won = self.config.clamp_points(players);

                if have_any_won {
                    let dummy = vec![];
                    let players = mem::replace(players, dummy);
                    let finished_players: Vec<FinishedPlayer> =
                        players.into_iter().map(|p| p.into_finished()).collect();
                    self.phase = Phase::Final(finished_players.clone());
                    Ok(Outcome::GameOver(finished_players))
                } else {
                    let dummy = vec![];
                    let players = mem::replace(players, dummy);
                    let dequeueing_players: Vec<DraineelessPlayer> = players
                        .into_iter()
                        .zip(&action_points_destroyed)
                        .map(|(p, ap)| p.into_draineeless(ap.0))
                        .collect();
                    self.phase = Phase::DrainedMove(dequeueing_players);

                    Ok(Outcome::ActionPhaseDone(action_points_destroyed))
                }
            }
        } else {
            Err(())
        }
    }

    pub fn winner_index(&self) -> Option<usize> {
        match &self.phase {
            Phase::Final(players) => players
                .iter()
                .position(|p| p.points() >= self.config.points_to_win),
            _ => None,
        }
    }
}

impl Default for BatchChoiceGame {
    fn default() -> BatchChoiceGame {
        BatchChoiceGame::new(Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_new_works() {
        let _game = BatchChoiceGame::default();
    }

    #[test]
    fn initial_players_returns_the_correct_amount_of_players() {
        let config = Config::default();
        assert_eq!(
            config.player_count as usize,
            BatchChoiceGame::initial_players(&config).len()
        );
    }

    #[test]
    fn all_players_can_initially_choose_any_character() {
        let game = BatchChoiceGame::default();
        assert_eq!(
            BatchChoices::Character(vec![Character::all(), Character::all()]),
            game.choices()
        );
    }

    #[test]
    fn players_cannot_choose_character_twice() {
        let mut game = BatchChoiceGame::default();
        let ninja_samurai = vec![Character::Ninja, Character::Samurai];

        game.choose(BatchChoice::Characters(ninja_samurai.clone()))
            .unwrap();
        assert_eq!(Err(()), game.choose(BatchChoice::Characters(ninja_samurai)));
    }

    #[test]
    fn players_cannot_choose_character_they_repeated_maximum_times() {
        let mut game = BatchChoiceGame::new(Config {
            player_count: 4,
            ..Config::default()
        });

        let choices = vec![
            BatchChoice::Characters(vec![
                Character::Ninja,
                Character::Ninja,
                Character::Samurai,
                Character::Ninja,
            ]),
            BatchChoice::Characters(vec![
                Character::Ninja,
                Character::Ninja,
                Character::Samurai,
                Character::Zombie,
            ]),
            BatchChoice::Characters(vec![
                Character::Ninja,
                Character::Ninja,
                Character::Samurai,
                Character::Samurai,
            ]),
        ];

        for choice in choices {
            game.choose(choice).unwrap();
        }

        let illegal_choice = BatchChoice::Characters(vec![
            Character::Ninja,
            Character::Ninja,
            Character::Samurai,
            Character::Samurai,
        ]);
        assert_eq!(Err(()), game.choose(illegal_choice));

        let mut no_ninja = Character::all();
        no_ninja.retain(|c| c != &Character::Ninja);
        let mut no_samurai = Character::all();
        no_samurai.retain(|c| c != &Character::Samurai);
        assert_eq!(
            BatchChoices::Character(vec![
                no_ninja.clone(),
                no_ninja,
                no_samurai,
                Character::all()
            ]),
            game.choices()
        );
    }

    #[test]
    fn all_players_must_rechoose_if_duplicate_characters_chosen() {
        let mut game = BatchChoiceGame::default();
        let ninja_ninja = vec![Character::Ninja, Character::Ninja];
        assert_eq!(
            Ok(Outcome::CharacterPhaseRechoose(ninja_ninja.clone())),
            game.choose(BatchChoice::Characters(ninja_ninja))
        );
    }

    #[test]
    fn character_phase_ends_if_all_players_choose_legal_characters() {
        let mut game = BatchChoiceGame::default();
        let ninja_samurai = vec![Character::Ninja, Character::Samurai];
        assert_eq!(
            Ok(Outcome::CharacterPhaseDone(
                ninja_samurai
                    .iter()
                    .zip(vec![1, 0])
                    .map(|(character, headstart)| CharacterHeadstart(*character, headstart))
                    .collect()
            )),
            game.choose(BatchChoice::Characters(ninja_samurai))
        );
    }

    #[test]
    fn fails_if_any_booster_is_illegal() {
        let mut game = BatchChoiceGame::default();
        let ninja_samurai = BatchChoice::Characters(vec![Character::Ninja, Character::Samurai]);
        let strong_atlas = BatchChoice::Boosters(vec![Booster::Strong, Booster::Atlas]);

        game.choose(ninja_samurai).unwrap();
        assert!(game.choose(strong_atlas).is_err());
    }

    #[test]
    fn players_cannot_choose_boosters_twice() {
        let mut game = BatchChoiceGame::default();
        let ninja_samurai = BatchChoice::Characters(vec![Character::Ninja, Character::Samurai]);
        let shadow_atlas = BatchChoice::Boosters(vec![Booster::Shadow, Booster::Atlas]);

        game.choose(ninja_samurai).unwrap();
        game.choose(shadow_atlas.clone()).unwrap();
        assert!(game.choose(shadow_atlas).is_err());
    }

    #[test]
    fn booster_phase_ends_if_all_boosters_are_legal() {
        let mut game = BatchChoiceGame::default();
        let ninja_samurai = BatchChoice::Characters(vec![Character::Ninja, Character::Samurai]);
        let shadow_atlas = BatchChoice::Boosters(vec![Booster::Shadow, Booster::Atlas]);

        game.choose(ninja_samurai).unwrap();
        assert_eq!(
            Ok(Outcome::BoosterPhaseDone(vec![
                Booster::Shadow,
                Booster::Atlas
            ])),
            game.choose(shadow_atlas)
        );
    }

    #[test]
    fn players_can_initally_drain_mirror() {
        use crate::choices::ArsenalItem;

        let mut game = BatchChoiceGame::default();
        let ninja_samurai = BatchChoice::Characters(vec![Character::Ninja, Character::Samurai]);
        let shadow_atlas = BatchChoice::Boosters(vec![Booster::Shadow, Booster::Atlas]);
        let mirror_mirror = BatchChoice::DequeueChoices(vec![
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
        ]);

        game.choose(ninja_samurai).unwrap();
        game.choose(shadow_atlas).unwrap();
        assert_eq!(
            Ok(Outcome::DequeuePhaseDone(vec![
                DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
                DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
            ])),
            game.choose(mirror_mirror)
        );
    }

    #[test]
    fn shadow_can_initially_choose_shadow_fireball() {
        use crate::choices::{ArsenalItem, Move};

        let mut game = BatchChoiceGame::default();
        let ninja_samurai = BatchChoice::Characters(vec![Character::Ninja, Character::Samurai]);
        let shadow_atlas = BatchChoice::Boosters(vec![Booster::Shadow, Booster::Atlas]);
        let mirror_mirror = BatchChoice::DequeueChoices(vec![
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
        ]);
        let fireball_lightning = BatchChoice::Actions(vec![
            Action::Move(Move::ShadowFireball),
            Action::Move(Move::Lightning),
        ]);

        game.choose(ninja_samurai).unwrap();
        game.choose(shadow_atlas).unwrap();
        game.choose(mirror_mirror).unwrap();
        assert_eq!(
            Ok(Outcome::ActionPhaseDone(vec![
                ActionPointsDestroyed(Action::Move(Move::ShadowFireball), 1, false),
                ActionPointsDestroyed(Action::Move(Move::Lightning), 0, false),
            ])),
            game.choose(fireball_lightning)
        );
    }

    #[test]
    fn zap_destroys_shadow_fireball() {
        use crate::choices::{ArsenalItem, Move};

        let mut game = BatchChoiceGame::default();
        let zombie_ninja = BatchChoice::Characters(vec![Character::Zombie, Character::Ninja]);
        let zombie_corps_shadow =
            BatchChoice::Boosters(vec![Booster::ZombieCorps, Booster::Shadow]);
        let mirror_mirror = BatchChoice::DequeueChoices(vec![
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
        ]);
        let zap_fireball = BatchChoice::Actions(vec![
            Action::Move(Move::Zap),
            Action::Move(Move::ShadowFireball),
        ]);

        game.choose(zombie_ninja).unwrap();
        game.choose(zombie_corps_shadow).unwrap();
        game.choose(mirror_mirror).unwrap();
        assert_eq!(
            Ok(Outcome::ActionPhaseDone(vec![
                ActionPointsDestroyed(Action::Move(Move::Zap), 0, true),
                ActionPointsDestroyed(Action::Move(Move::ShadowFireball), 0, true),
            ])),
            game.choose(zap_fireball)
        );
    }

    #[test]
    fn game_ends_if_exactly_one_player_wins() {
        use crate::choices::{ArsenalItem, Move};

        let mut game = BatchChoiceGame::new(Config {
            points_to_win: 1,
            ..Config::default()
        });
        let ninja_zombie = BatchChoice::Characters(vec![Character::Ninja, Character::Zombie]);
        let shadow_regenerative =
            BatchChoice::Boosters(vec![Booster::Shadow, Booster::Regenerative]);
        let mirror_mirror = BatchChoice::DequeueChoices(vec![
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
        ]);
        let slip_regenerate = BatchChoice::Actions(vec![
            Action::Move(Move::ShadowSlip),
            Action::Move(Move::Regenerate),
        ]);

        game.choose(ninja_zombie).unwrap();
        game.choose(shadow_regenerative).unwrap();
        game.choose(mirror_mirror).unwrap();
        let outcome = game.choose(slip_regenerate).unwrap();

        match &outcome {
            Outcome::GameOver(_) => {}
            _ => panic!("Game did not end."),
        }
        match &game.phase {
            Phase::Final(_) => {}
            _ => panic!("Game did not end."),
        }
    }

    #[test]
    fn players_cannot_choose_if_game_ends() {
        use crate::choices::{ArsenalItem, Move};

        let mut game = BatchChoiceGame::new(Config {
            points_to_win: 1,
            ..Config::default()
        });
        let ninja_zombie = BatchChoice::Characters(vec![Character::Ninja, Character::Zombie]);
        let shadow_regenerative =
            BatchChoice::Boosters(vec![Booster::Shadow, Booster::Regenerative]);
        let mirror_mirror = BatchChoice::DequeueChoices(vec![
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
            DequeueChoice::DrainAndExit(ArsenalItem::Mirror),
        ]);
        let slip_regenerate = BatchChoice::Actions(vec![
            Action::Move(Move::ShadowSlip),
            Action::Move(Move::Regenerate),
        ]);

        game.choose(ninja_zombie).unwrap();
        game.choose(shadow_regenerative).unwrap();
        game.choose(mirror_mirror.clone()).unwrap();
        game.choose(slip_regenerate).unwrap();

        assert!(game.choose(mirror_mirror).is_err());
    }
}
