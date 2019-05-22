use crate::{
    choices::{Action, Booster, Character, Choose, DequeueChoice},
    helpers,
    outcomes::{
        ActionPhaseOutcome, ActionPoints, BoosterPhaseOutcome, CharacterHeadstart,
        CharacterPhaseOutcome, DraineePhaseOutcome,
    },
    player::{
        ActionlessPlayer, BoosterlessPlayer, CharacterlessPlayer, DraineelessPlayer, FinishedPlayer,
    },
};

use std::mem;

#[derive(Debug, Clone)]
pub struct Game {
    config: GameConfig,
    phase: Phase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameConfig {
    pub player_count: u8,
    pub max_points: u8,
    pub max_character_repetitions: u8,
    pub max_arsenal_items: u8,
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

            Phase::DrainedMove(players) => Choices::DrainedMove(
                players
                    .iter()
                    .map(|p| p.choices().map(|choices| choices))
                    .collect(),
            ),

            Phase::Action(players) => {
                Choices::Action(players.iter().map(|p| p.choices()).collect())
            }

            Phase::Final(_) => Choices::None,
        }
    }

    pub fn choose_character(
        &mut self,
        index: usize,
        character: Character,
    ) -> Result<CharacterPhaseOutcome, ()> {
        if let Phase::Character(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(character);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let mut character_codes: Vec<u8> =
                            players.iter().map(|p| p.choice().unwrap() as u8).collect();
                        if helpers::has_duplicates(&mut character_codes) {
                            let characters = players.iter().map(|p| p.choice().unwrap()).collect();
                            for p in players {
                                p.clear_choice().unwrap();
                            }
                            CharacterPhaseOutcome::Rechoose(characters)
                        } else {
                            let all_characters: Vec<Character> =
                                players.iter().map(|p| p.choice().unwrap()).collect();
                            let mut character_headstarts: Vec<CharacterHeadstart> = all_characters
                                .iter()
                                .map(|c| CharacterHeadstart(*c, 0))
                                .collect();
                            for (index, player) in players.iter_mut().enumerate() {
                                let mut other_characters = all_characters.clone();
                                let own_character = other_characters.remove(index);
                                for other_character in other_characters {
                                    let points = own_character.points_against(other_character);
                                    character_headstarts[index].1 += points;
                                    player.add_points(points);
                                }
                            }

                            let dummy: Vec<CharacterlessPlayer> = vec![];
                            let players = mem::replace(players, dummy);
                            self.phase = Phase::Booster(
                                players
                                    .into_iter()
                                    .map(|p| p.into_boosterless().unwrap())
                                    .collect(),
                            );

                            CharacterPhaseOutcome::Done(character_headstarts)
                        }
                    } else {
                        CharacterPhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn choose_booster(
        &mut self,
        index: usize,
        booster: Booster,
    ) -> Result<BoosterPhaseOutcome, ()> {
        if let Phase::Booster(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(booster);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let boosters: Vec<Booster> =
                            players.iter().map(|p| p.choice().unwrap()).collect();
                        let dummy = vec![];
                        let players = mem::replace(players, dummy);
                        self.phase = Phase::DrainedMove(
                            players
                                .into_iter()
                                .map(|p| p.into_draineeless().unwrap())
                                .collect(),
                        );

                        BoosterPhaseOutcome::Done(boosters)
                    } else {
                        BoosterPhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn choose_drainee(
        &mut self,
        index: usize,
        drainee: DequeueChoice,
    ) -> Result<DraineePhaseOutcome, ()> {
        if let Phase::DrainedMove(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(drainee);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let dequeue_choices: Vec<DequeueChoice> = players
                            .iter()
                            .map(|p| p.choice().unwrap().clone())
                            .collect();

                        let dummy = vec![];
                        let players = mem::replace(players, dummy);
                        self.phase = Phase::Action(
                            players
                                .into_iter()
                                .map(|p| p.into_actionless().unwrap())
                                .collect(),
                        );

                        DraineePhaseOutcome::Done(dequeue_choices)
                    } else {
                        DraineePhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn choose_action(
        &mut self,
        index: usize,
        action: Action,
    ) -> Result<ActionPhaseOutcome, ()> {
        if let Phase::Action(players) = &mut self.phase {
            if index < players.len() {
                let player = &mut players[index];
                let result = player.choose(action);

                match result {
                    Err(_) => Err(()),
                    Ok(_) => Ok(if players.complete() {
                        let all_actions: Vec<Action> = players
                            .iter()
                            .map(|p| p.choice().unwrap().clone())
                            .collect();
                        let mut action_points: Vec<ActionPoints> = all_actions
                            .clone()
                            .into_iter()
                            .map(|a| ActionPoints(a, 0))
                            .collect();
                        for (index, player) in players.iter_mut().enumerate() {
                            let mut other_actions = all_actions.clone();
                            let action = other_actions.remove(index);
                            for other in other_actions {
                                let points = action.points_against(&other);
                                action_points[index].1 += points;
                                player.add_points(points);
                            }
                        }

                        if players.iter().all(|p| p.needs_points_to_win()) {
                            let dummy = vec![];
                            let players = mem::replace(players, dummy);
                            self.phase = Phase::DrainedMove(
                                players
                                    .into_iter()
                                    .map(|p| p.into_draineeless().unwrap())
                                    .collect(),
                            );
                            ActionPhaseOutcome::Done(action_points)
                        } else {
                            // TODO Handle multiple players having the needed points to win
                            let dummy = vec![];
                            let players = mem::replace(players, dummy);
                            let finished_players: Vec<FinishedPlayer> =
                                players.into_iter().map(|p| p.into_finished()).collect();
                            self.phase = Phase::Final(finished_players.clone());

                            ActionPhaseOutcome::GameOver(finished_players)
                        }
                    } else {
                        ActionPhaseOutcome::Pending
                    }),
                }
            } else {
                Err(())
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

#[derive(Debug, Clone, PartialEq)]
pub enum Choices {
    Character(Vec<Option<Vec<Character>>>),
    Booster(Vec<Option<Vec<Booster>>>),
    DrainedMove(Vec<Option<Vec<DequeueChoice>>>),
    Action(Vec<Option<Vec<Action>>>),
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

pub(crate) trait PhaseComplete<C> {
    fn complete(&self) -> bool;
}

impl<P, C> PhaseComplete<C> for Vec<P>
where
    P: Choose<C>,
{
    fn complete(&self) -> bool {
        self.iter().all(|p| p.has_chosen())
    }
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
        let _game = Game::new(GameConfig::default());
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
        let game = Game::new(GameConfig::default());
        assert_eq!(
            Choices::Character(vec![Some(Character::all()), Some(Character::all())]),
            game.choices()
        );
    }

    #[test]
    fn all_players_must_rechoose_if_duplicate_characters_picked() {
        use crate::outcomes::CharacterPhaseOutcome;

        let mut game = Game::new(GameConfig::default());
        assert_eq!(
            Ok(CharacterPhaseOutcome::Pending),
            game.choose_character(0, Character::Ninja)
        );
        assert_eq!(
            Ok(CharacterPhaseOutcome::Rechoose(vec![
                Character::Ninja,
                Character::Ninja
            ])),
            game.choose_character(1, Character::Ninja)
        );
    }

    #[test]
    fn character_phase_ends_if_players_pick_unique_characters() {
        use crate::outcomes::{CharacterHeadstart, CharacterPhaseOutcome};

        let mut game = Game::new(GameConfig::default());
        assert_eq!(
            Ok(CharacterPhaseOutcome::Pending),
            game.choose_character(0, Character::Ninja)
        );
        assert_eq!(
            Ok(CharacterPhaseOutcome::Done(vec![
                CharacterHeadstart(Character::Ninja, 1),
                CharacterHeadstart(Character::Samurai, 0)
            ])),
            game.choose_character(1, Character::Samurai)
        );
    }

    #[test]
    fn player_cannot_choose_character_twice() {
        use crate::outcomes::CharacterPhaseOutcome;

        let mut game = Game::new(GameConfig::default());
        assert_eq!(
            Ok(CharacterPhaseOutcome::Pending),
            game.choose_character(0, Character::Ninja)
        );
        assert!(game.choose_character(0, Character::Samurai).is_err());
    }

    #[test]
    fn players_cannot_choose_character_they_repeated_maximum_times() {
        use crate::outcomes::CharacterPhaseOutcome;

        const REPETITIVE_PLAYER_COUNT: u8 = 3;
        const MAX_CHARACTER_REPETITIONS: u8 = 3;
        let mut config = GameConfig::default();
        config.player_count = REPETITIVE_PLAYER_COUNT + 1;
        config.max_character_repetitions = MAX_CHARACTER_REPETITIONS;

        let mut game = Game::new(config);

        const REPEATED_CHARACTERS: [Character; REPETITIVE_PLAYER_COUNT as usize] =
            [Character::Ninja, Character::Ninja, Character::Samurai];
        const NON_REPEATED_CHARACTERS: [Character; MAX_CHARACTER_REPETITIONS as usize] =
            [Character::Ninja, Character::Zombie, Character::Samurai];
        const NON_REPEATING_PLAYER_INDEX: usize = REPETITIVE_PLAYER_COUNT as usize;

        let mut outcome: Option<CharacterPhaseOutcome> = None;

        for i in 0..game.config.max_character_repetitions {
            for j in 0..REPETITIVE_PLAYER_COUNT {
                let j = j as usize;
                game.choose_character(j, REPEATED_CHARACTERS[j]).unwrap();
            }

            let j = NON_REPEATING_PLAYER_INDEX;
            outcome = Some(
                game.choose_character(j, NON_REPEATED_CHARACTERS[i as usize])
                    .unwrap(),
            );
        }

        let outcome = outcome.unwrap();

        assert_eq!(
            CharacterPhaseOutcome::Rechoose(vec![
                Character::Ninja,
                Character::Ninja,
                Character::Samurai,
                Character::Samurai
            ]),
            outcome
        );

        if let Choices::Character(choices) = game.choices() {
            let mut no_ninja = Character::all();
            no_ninja.retain(|c| c != &Character::Ninja);
            let no_ninja = Some(no_ninja);
            let mut no_samurai = Character::all();
            no_samurai.retain(|c| c != &Character::Samurai);
            let no_samurai = Some(no_samurai);
            let all = Some(Character::all());
            assert_eq!(no_ninja, choices[0]);
            assert_eq!(no_ninja, choices[1]);
            assert_eq!(no_samurai, choices[2]);
            assert_eq!(all, choices[3]);
        } else {
            panic!("Game on wrong phase.");
        }
    }
}
