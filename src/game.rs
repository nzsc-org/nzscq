use crate::{
    choices::{
        Action, BatchChoice, Booster, CanChoose, Character, Choose, DequeueChoice, PointsAgainst,
    },
    helpers::HasDuplicates,
    outcomes::{ActionPoints, CharacterHeadstart, Outcome},
    players::{
        ActionlessPlayer, BoosterlessPlayer, CharacterlessPlayer, DraineelessPlayer, FinishedPlayer,
    },
};

use std::mem;

#[derive(Debug, Clone)]
pub struct Game {
    config: GameConfig,
    phase: Phase,
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        Self {
            config: config.clone(),
            phase: Phase::Character(Self::initial_players(&config)),
        }
    }

    fn initial_players(config: &GameConfig) -> Vec<CharacterlessPlayer> {
        let mut players: Vec<CharacterlessPlayer> = vec![];
        for _ in 0..config.player_count {
            players.push(CharacterlessPlayer::from_game_config(config.clone()))
        }
        players
    }

    pub fn choices(&self) -> Choices {
        match &self.phase {
            Phase::Character(players) => {
                Choices::Character(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Booster(players) => {
                Choices::Booster(players.iter().map(|p| p.choices()).collect())
            }

            Phase::DrainedMove(players) => {
                Choices::DrainedMove(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Action(players) => {
                Choices::Action(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Final(_) => Choices::None,
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
                let action_points: Vec<ActionPoints> = actions
                    .iter()
                    .zip(points)
                    .map(|(a, p)| ActionPoints(*a, p))
                    .collect();

                for (player, &ActionPoints(_, points)) in players.iter_mut().zip(&action_points) {
                    player.add_points(points);
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
                        .zip(&action_points)
                        .map(|(p, ap)| p.into_draineeless(ap.0))
                        .collect();
                    self.phase = Phase::DrainedMove(dequeueing_players);

                    Ok(Outcome::ActionPhaseDone(action_points))
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
                .position(|p| p.points() >= self.config.max_points),
            _ => None,
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new(GameConfig::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameConfig {
    pub player_count: u8,
    pub max_points: u8,
    pub max_character_repetitions: u8,
    pub max_arsenal_items: u8,
}

impl GameConfig {
    fn clamp_points(&self, players: &mut Vec<ActionlessPlayer>) -> bool {
        let max = players.iter().map(|p| p.points()).max().unwrap();
        if max >= self.max_points {
            let players_with_max = players.iter().filter(|p| p.points() == max).count();
            if players_with_max > 1 {
                for p in players {
                    p.deduct_points(max - self.max_points - 1);
                }

                false
            } else {
                for p in players {
                    p.deduct_points(max - self.max_points);
                }

                true
            }
        } else {
            false
        }
    }
}

impl Default for GameConfig {
    fn default() -> GameConfig {
        GameConfig {
            player_count: 2,
            max_points: 5,
            max_character_repetitions: 3,
            max_arsenal_items: 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Choices {
    Character(Vec<Vec<Character>>),
    Booster(Vec<Vec<Booster>>),
    DrainedMove(Vec<Vec<DequeueChoice>>),
    Action(Vec<Vec<Action>>),
    None,
}

#[derive(Debug, Clone)]
enum Phase {
    Character(Vec<CharacterlessPlayer>),
    Booster(Vec<BoosterlessPlayer>),
    DrainedMove(Vec<DraineelessPlayer>),
    Action(Vec<ActionlessPlayer>),
    Final(Vec<FinishedPlayer>),
}

// TODO Handle destructives and single-use moves.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_config_default_works() {
        let _config = GameConfig::default();
    }

    #[test]
    fn game_new_works() {
        let _game = Game::default();
    }

    #[test]
    fn initial_players_returns_the_correct_amount_of_players() {
        let config = GameConfig::default();
        assert_eq!(
            config.player_count as usize,
            Game::initial_players(&config).len()
        );
    }

    #[test]
    fn all_players_can_initially_choose_any_character() {
        let game = Game::default();
        assert_eq!(
            Choices::Character(vec![Character::all(), Character::all()]),
            game.choices()
        );
    }

    #[test]
    fn players_cannot_choose_character_twice() {
        let mut game = Game::default();
        let ninja_samurai = vec![Character::Ninja, Character::Samurai];

        game.choose(BatchChoice::Characters(ninja_samurai.clone()))
            .unwrap();
        assert_eq!(Err(()), game.choose(BatchChoice::Characters(ninja_samurai)));
    }

    #[test]
    fn players_cannot_choose_character_they_repeated_maximum_times() {
        let mut game = Game::new(GameConfig {
            player_count: 4,
            ..GameConfig::default()
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
            Choices::Character(vec![
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
        let mut game = Game::default();
        let ninja_ninja = vec![Character::Ninja, Character::Ninja];
        assert_eq!(
            Ok(Outcome::CharacterPhaseRechoose(ninja_ninja.clone())),
            game.choose(BatchChoice::Characters(ninja_ninja))
        );
    }

    #[test]
    fn character_phase_ends_if_all_players_choose_legal_characters() {
        let mut game = Game::default();
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
        let mut game = Game::default();
        let ninja_samurai = BatchChoice::Characters(vec![Character::Ninja, Character::Samurai]);
        let strong_atlas = BatchChoice::Boosters(vec![Booster::Strong, Booster::Atlas]);

        game.choose(ninja_samurai).unwrap();
        assert!(game.choose(strong_atlas).is_err());
    }

    #[test]
    fn players_cannot_choose_boosters_twice() {
        let mut game = Game::default();
        let ninja_samurai = BatchChoice::Characters(vec![Character::Ninja, Character::Samurai]);
        let shadow_atlas = BatchChoice::Boosters(vec![Booster::Shadow, Booster::Atlas]);

        game.choose(ninja_samurai).unwrap();
        game.choose(shadow_atlas.clone()).unwrap();
        assert!(game.choose(shadow_atlas).is_err());
    }

    #[test]
    fn booster_phase_ends_if_all_boosters_are_legal() {
        let mut game = Game::default();
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

        let mut game = Game::default();
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

        let mut game = Game::default();
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
                ActionPoints(Action::Move(Move::ShadowFireball), 1),
                ActionPoints(Action::Move(Move::Lightning), 0),
            ])),
            game.choose(fireball_lightning)
        );
    }
}
